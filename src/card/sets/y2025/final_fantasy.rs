//! Final Fantasy card inventory.

use super::CardRecord;
use super::PrintingRecord;
use crate::AdditionalCostIndex;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityPredicateDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AddManaEffectDef;
use crate::card::AdditionalCostValueDef;
use crate::card::AggregateOperationDef;
use crate::card::AlternativeCastKindDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BattlefieldArrivalDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::BattlefieldEntryScalarChoiceDef;
use crate::card::BindObjectsDef;
use crate::card::BlockRestrictionDef;
use crate::card::CardArt;
use crate::card::CardNameDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::CharacteristicOperationDef;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseCardsFromCollectionDef;
use crate::card::ChooseDef;
use crate::card::ChooseForEachPlayerDef;
use crate::card::ChooseObjectOrderDef;
use crate::card::CollectionInspectionDef;
use crate::card::ComparisonDef;
use crate::card::ConditionDef;
use crate::card::CopyExceptionsDef;
use crate::card::CostDef;
use crate::card::CostModificationDef;
use crate::card::CostQuantityDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::CreatedTokensDef;
use crate::card::CreatureTypeSetDef;
use crate::card::DamageAssignmentDef;
use crate::card::DamageEventMatcherDef;
use crate::card::DamageKindDef;
use crate::card::DamageRecipientMatcherDef;
use crate::card::DamageSourceMatcherDef;
use crate::card::DiscardFollowUpDef;
use crate::card::DiscardSelectionDef;
use crate::card::DrawEventMatcherDef;
use crate::card::EffectChoiceDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::FreePlayDef;
use crate::card::FreePlayDurationDef;
use crate::card::GraveyardPlayPermissionDef;
use crate::card::HalvedValueDef;
use crate::card::InstalledTriggerDef;
use crate::card::KeywordAbility;
use crate::card::ManaColor;
use crate::card::ManaTypeDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetCountConditionDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectSetFilterDef;
use crate::card::ObjectSetPredicateDef;
use crate::card::ObjectValueAggregateDef;
use crate::card::ObjectValueDef;
use crate::card::PayOrDef;
use crate::card::PerPlayerSelectionDef;
use crate::card::PlayActionMatcherDef;
use crate::card::PlayRestrictionDef;
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
use crate::card::RoundingDef;
use crate::card::ScaledValueDef;
use crate::card::SetOperationDef;
use crate::card::SpellResolutionDestinationDef;
use crate::card::SubtypeDef;
use crate::card::SumValueDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenCopyDef;
use crate::card::TokenDef;
use crate::card::TopOfLibraryCostDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::card::{TriggerModificationDef, TriggerModificationKindDef};
use crate::mana_cost;

use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y2001::odyssey as catalog_ody;
use crate::card::sets::y2005::ravnica_city_of_guilds as catalog_rav;
use crate::card::sets::y2016::oath_of_the_gatewatch as catalog_ogw;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "FIN",
    slug: "final-fantasy",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

pub const TIERED: crate::card::MechanicId = crate::card::MechanicId::from_name("mtg:tiered");
/// Choose exactly one mode and pay its additional cost.
#[must_use]
pub const fn tiered(
    text: &'static str,
    modes: &'static [(&'static [CostDef], AbilityDef)],
) -> AbilityDef {
    AbilityDef::defined(
        text,
        crate::card::DeclarativeAbilityDef::Spell(crate::card::SpellAbilityDef::Modal(
            crate::card::ModalSpellDef::with_costed_modes(modes, 1, 1, false),
        )),
        EffectDef::None,
    )
    .labeled(TIERED)
}
fn adventure_land(
    record: &CardRecord,
    name: &'static str,
    alternate: &CardRules,
) -> crate::card::CardComposition {
    use crate::card::{
        AlternateSpellKind, CardComposition, CardEffectStatus, CardPart, CardStructure,
        PlayOptionDef, SpellForm,
    };
    use crate::{CardPartId, PlayOptionId};
    let primary_name = record
        .name
        .split(" // ")
        .next()
        .expect("Adventure has a main name");
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
            PlayOptionDef::play_land(
                PlayOptionId::DEFAULT,
                primary_name,
                CardPartId::PRIMARY,
                CardEffectStatus::Implemented,
            ),
            PlayOptionDef::cast(
                PlayOptionId(1),
                name,
                SpellForm::Part(CardPartId(1)),
                alternate.mana_cost().expect("Adventure cost"),
                CardEffectStatus::Implemented,
            ),
        ],
    }
    .with_derived_spell_targets()
}

const FOOD_TOKEN: TokenCharacteristics = crate::card::tokens::food().with_art(CardArt::new(
    "4853f6f5-476d-4109-a1c6-f98f4d332db3",
    "David Astruga",
));
const TREASURE_TOKEN: TokenCharacteristics = crate::card::tokens::treasure().with_art(
    CardArt::new("ab0cda01-10b5-4a1b-b2bb-ad8bb296590f", "Leonardo Santanna"),
);

const HERO_TOKEN: TokenCharacteristics = TokenCharacteristics::creature(&["Hero"], &[], 1, 1)
    .with_art(CardArt::new(
        "d0657ce1-bf75-4007-ac1b-0623eb263357",
        "Josephine Chang",
    ));
const HERO_TOKEN_2: TokenCharacteristics = TokenCharacteristics::creature(&["Hero"], &[], 1, 1)
    .with_art(CardArt::new(
        "d0657ce1-bf75-4007-ac1b-0623eb263357",
        "Josephine Chang",
    ));
const WIZARD_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Wizard"], &[ManaColor::Black], 0, 1)
        .with_abilities(&[AbilityDef::triggered(
            "Whenever you cast a noncreature spell, this token deals 1 \
                     damage to each opponent.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(1)),
        )])
        .with_art(CardArt::new(
            "187fe54c-7d0c-4225-9d46-3affbead897d",
            "Ignatius Budi",
        ));
const HORROR_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Horror"], &[ManaColor::Black], 2, 2).with_art(CardArt::new(
        "ddcf50c2-24f5-46b4-bfe9-c636bb51bae5",
        "Lordigan",
    ));
const BIRD_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Bird"], &[ManaColor::Green], 2, 2)
        .with_abilities(&[AbilityDef::triggered(
            "Whenever a land you control enters, this token gets +1/+0 \
                         until end of turn.",
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
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )])
        .with_art(CardArt::new(
            "1fbc471d-5948-47fc-b7cc-81cc13a4cd15",
            "David Frasheski",
        ));

// FIN 1 — Summon: Bahamut
// Audit: unsupported — Needs one printed Saga ability to represent several chapter numbers and to remain identifiable to the Saga final-chapter rules; the current Saga authoring and recognition path represents exactly one chapter per ability.
pub(in crate::card::sets) static SUMMON_BAHAMUT: CardRecord = CardRecord::new(
    "Summon: Bahamut",
    "95318d85-4a08-47ac-a43d-ea83c0bea81c",
    "Arif Wijaya",
    CardRules::unsupported(),
);

// FIN 2 — Ultima, Origin of Oblivion
// Audit: unsupported — Needs a resolved land-type and ability overwrite that lasts only while the affected land retains a blight counter; existing resolved effects have turn, source, or permanent durations without a counter-presence lifetime.
pub(in crate::card::sets) static ULTIMA_ORIGIN_OF_OBLIVION: CardRecord = CardRecord::new(
    "Ultima, Origin of Oblivion",
    "d55a4c02-1aa4-454c-9041-84937377a53b",
    "Russell Dongjun Lu",
    CardRules::unsupported(),
);

// FIN 3 — Adelbert Steiner
pub(in crate::card::sets) static ADELBERT_STEINER: CardRecord = CardRecord::new(
    "Adelbert Steiner",
    "1a67a991-1e52-4676-a2e3-2bc7aa943ab3",
    "Lorenzo Mastroianni",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Knight"], 2, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::lifelink(),
            AbilityDef::static_ability(
                "Adelbert Steiner gets +1/+1 for each Equipment you control.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Equipment")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Equipment")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                    ),
                },
            ),
        ]),
);

// FIN 4 — Aerith Gainsborough
pub(in crate::card::sets) static AERITH_GAINSBOROUGH: CardRecord = CardRecord::new(
    "Aerith Gainsborough",
    "e86328b6-ded2-41df-8b6e-4a770e7b171e",
    "Nakamura8",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Cleric"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::lifelink(),
            AbilityDef::triggered(
                "Whenever you gain life, put a +1/+1 counter on Aerith \
                 Gainsborough.",
                TriggerEventDef::LifeGained(PlayerRelation::You),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
            abilities::dies_trigger(
                "When Aerith Gainsborough dies, put X +1/+1 counters on each \
                 legendary creature you control, where X is the number of \
                 +1/+1 counters on Aerith Gainsborough.",
                EffectDef::AddCounters {
                    object: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::CountersOnSource(CounterKind::PlusOnePlusOne),
                },
            ),
        ]),
);

// FIN 5 — Aerith Rescue Mission
pub(in crate::card::sets) static AERITH_RESCUE_MISSION: CardRecord = CardRecord::new(
    "Aerith Rescue Mission",
    "3123d16c-e1e6-4659-a7a3-2ec6efc6bf08",
    "Hokuyuu",
    CardRules::new_sorcery(mana_cost!("{3}{W}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "Take the Elevator — Create three 1/1 colorless Hero creature \
                 tokens.",
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(HERO_TOKEN))
                        .with_count(ValueDef::Constant(3)),
                ),
            ),
            AbilityDef::spell_with_targets(
                "Take 59 Flights of Stairs — Tap up to three target creatures. \
                 Put a stun counter on one of them. (If a permanent with a \
                 stun counter would become untapped, remove one from it \
                 instead.)",
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    3,
                )],
                EffectDef::BindObjects(BindObjectsDef {
                    source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::LegalTargets(
                        TargetIndex::PRIMARY,
                    )),
                    binding: crate::Binding!("stairs"),
                    then: &EffectDef::Sequence(&[
                        EffectDef::Tap {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("stairs"),
                            )),
                        },
                        EffectDef::Choose(ChooseDef {
                            binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                            unchosen: None,
                            chooser: PlayerRefDef::EffectController,
                            candidates: ObjectSetDef::Binding(crate::Binding!("stairs")),
                            exclude: None,
                            minimum: 1,
                            maximum: 1,
                            visibility: ChoiceVisibilityDef::Public,
                            then: &EffectDef::AddCounters {
                                object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("chosen"),
                                )),
                                kind: CounterKind::Stun,
                                amount: ValueDef::Constant(1),
                            },
                        }),
                    ]),
                }),
            ),
        ],
    )]),
);

// FIN 6 — Ambrosia Whiteheart
pub(in crate::card::sets) static AMBROSIA_WHITEHEART: CardRecord = CardRecord::new(
    "Ambrosia Whiteheart",
    "f2596767-7d19-4110-86ed-3cfc93ac7483",
    "Fajareka Setiawan",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Bird"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flash(),
            abilities::enters_trigger(
                "When Ambrosia Whiteheart enters, you may return another \
                 permanent you control to its owner's hand.",
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Enchantment),
                                ObjectPredicateDef::HasType(CardType::Land),
                                ObjectPredicateDef::HasType(CardType::Planeswalker),
                            ]),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    exclude: None,
                    minimum: 0,
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
            ),
            AbilityDef::triggered(
                "Landfall — Whenever a land you control enters, Ambrosia \
                 Whiteheart gets +1/+0 until end of turn.",
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
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// FIN 7 — Ashe, Princess of Dalmasca
pub(in crate::card::sets) static ASHE_PRINCESS_OF_DALMASCA: CardRecord = CardRecord::new(
    "Ashe, Princess of Dalmasca",
    "ffe0596f-ef99-4862-9386-0fe455259995",
    "Yumi Yaoshida",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Rebel", "Noble"], 3, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::triggered(
            "Whenever Ashe attacks, look at the top five cards of your \
             library. You may reveal an artifact card from among them and \
             put it into your hand. Put the rest on the bottom of your \
             library in a random order.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(5),
                },
                actor: PlayerRefDef::EffectController,
                inspection: CollectionInspectionDef::Look,
                object: ObjectPredicateDef::HasType(CardType::Artifact),
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
                        randomized: crate::Binding!("random_bottom"),
                        then: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "random_bottom"
                            ))),
                            ZoneKind::Library,
                            ZonePlacement::Bottom,
                        ),
                    }),
                ]),
            }),
        )]),
);

// FIN 8 — Auron's Inspiration
pub(in crate::card::sets) static AURON_S_INSPIRATION: CardRecord = CardRecord::new(
    "Auron's Inspiration",
    "77d82764-563c-4bc2-b568-625ec7215e0d",
    "Fang Xinyu",
    CardRules::new_instant(mana_cost!("{2}{W}")).with_abilities(&[
        AbilityDef::spell(
            "Attacking creatures get +2/+0 until end of turn.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Attacking,
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                )),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{2}{W}{W}"))]),
    ]),
);

// FIN 9 — Battle Menu
pub(in crate::card::sets) static BATTLE_MENU: CardRecord = CardRecord::new(
    "Battle Menu",
    "240e1466-bd02-423d-b829-234dcd2bfab2",
    "Mingchen Shen",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "Attack — Create a 2/2 white Knight creature token.",
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::creature(&["Knight"], &[ManaColor::White], 2, 2),
                ))),
            ),
            AbilityDef::spell_with_targets(
                "Ability — Target creature gets +0/+4 until end of turn.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(0),
                        ValueDef::Constant(4),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell_with_targets(
                "Magic — Destroy target creature with power 4 or greater.",
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
            AbilityDef::spell(
                "Item — You gain 4 life.",
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(4),
                },
            ),
        ],
    )]),
);

// FIN 10 — Cloud, Midgar Mercenary
// Audit: unsupported — Needs triggered-ability duplication scoped to this equipped creature and Equipment attached to it; the current extra-trigger rules match entering or dying objects, not arbitrary triggered abilities on that attachment group.
pub(in crate::card::sets) static CLOUD_MIDGAR_MERCENARY: CardRecord = CardRecord::new(
    "Cloud, Midgar Mercenary",
    "2cf7e8a3-fad7-413d-b17c-7519a9cf5fb5",
    "Kazto Furuya",
    CardRules::unsupported(),
);

// FIN 11 — Cloudbound Moogle
pub(in crate::card::sets) static CLOUDBOUND_MOOGLE: CardRecord = CardRecord::new(
    "Cloudbound Moogle",
    "7387bca7-f496-45da-a0ac-6be049303a8f",
    "Andrea Radeck",
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Moogle"], 2, 3).with_abilities(&[
        abilities::flying(),
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
        abilities::typecycling!(
            "Plainscycling {2}",
            &[CostDef::Mana(mana_cost!("{2}"))],
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Plains"))
        ),
    ]),
);

// FIN 12 — Coeurl
pub(in crate::card::sets) static COEURL: CardRecord = CardRecord::new(
    "Coeurl",
    "7604b534-5480-42fa-bc36-bbae730f8582",
    "Miho Midorikawa",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Cat", "Beast"], 2, 2).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}{W}, {T}: Tap target nonenchantment creature.",
            &[CostDef::Mana(mana_cost!("{1}{W}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Enchantment)),
                ]),
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// FIN 13 — Crystal Fragments // Summon: Alexander
// Audit: unsupported — Needs one printed Saga ability to represent several chapter numbers and to remain identifiable to the Saga final-chapter rules; the current Saga authoring and recognition path represents exactly one chapter per ability.
pub(in crate::card::sets) static CRYSTAL_FRAGMENTS: CardRecord = CardRecord::new(
    "Crystal Fragments // Summon: Alexander",
    "5f51c853-949d-44e9-a3a2-02e1ce69a147",
    "Bachzim",
    CardRules::unsupported(),
);

// FIN 14 — The Crystal's Chosen
pub(in crate::card::sets) static THE_CRYSTAL_S_CHOSEN: CardRecord = CardRecord::new(
    "The Crystal's Chosen",
    "d4df4373-0ee6-44e3-81c6-0881c070014c",
    "Kotetsu Kinoshita",
    CardRules::new_sorcery(mana_cost!("{5}{W}{W}")).with_abilities(&[AbilityDef::spell(
        "Create four 1/1 colorless Hero creature tokens. Then put a \
         +1/+1 counter on each creature you control.",
        EffectDef::Sequence(&[
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(HERO_TOKEN))
                    .with_count(ValueDef::Constant(4)),
            ),
            EffectDef::AddCounters {
                object: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ))),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ]),
    )]),
);

// FIN 15 — Delivery Moogle
pub(in crate::card::sets) static DELIVERY_MOOGLE: CardRecord = CardRecord::new(
"Delivery Moogle",
"f58840dc-c641-4092-8b67-9c0d449af715",
"Joseph Weston",
CardRules::new_creature(mana_cost!("{3}{W}"), &["Moogle"], 3, 2).with_abilities(&[
abilities::flying(),
abilities::enters_trigger("When this creature enters, search your library and/or graveyard for an artifact card with mana value 2 or less, reveal it, and put it into your hand. If you search your library this way, shuffle.", EffectDef::ChooseEffect { player: EffectRecipientDef::Controller, choices: &[EffectChoiceDef { label: "Search your library.", effect: EffectDef::SearchZone { player: EffectRecipientDef::Controller, source: ZoneKind::Library, object: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::ManaValueAtMost(2)]), minimum: 0, maximum: ValueDef::Constant(1), reveal: true, destination: ZoneKind::Hand, placement: ZonePlacement::Top, shuffle: true, enters_tapped: false, attachment: None, binding: None, then: None } }, EffectChoiceDef { label: "Search your graveyard.", effect: EffectDef::SearchZone { player: EffectRecipientDef::Controller, source: ZoneKind::Graveyard, object: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::ManaValueAtMost(2)]), minimum: 1, maximum: ValueDef::Constant(1), reveal: true, destination: ZoneKind::Hand, placement: ZonePlacement::Top, shuffle: false, enters_tapped: false, attachment: None, binding: None, then: None } }, EffectChoiceDef { label: "Search both zones.", effect: EffectDef::Sequence(&[EffectDef::Choose(ChooseDef { chooser: PlayerRefDef::EffectController, candidates: ObjectSetDef::Union(&[ObjectSetDef::Query(ObjectQueryDef::matching(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::ManaValueAtMost(2)]), &[ZoneKind::Library], PlayerRelation::You)), ObjectSetDef::Query(ObjectQueryDef::matching(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::ManaValueAtMost(2)]), &[ZoneKind::Graveyard], PlayerRelation::You))]), exclude: None, minimum: 0, maximum: 1, binding: ObjectChoiceBindingDef::Objects(Binding!("moogle_found")), unchosen: None, visibility: ChoiceVisibilityDef::Private, then: &EffectDef::Sequence(&[EffectDef::RevealObjects(RevealObjectsDef { input: ObjectSetDef::Binding(Binding!("moogle_found")), then: &EffectDef::None }), EffectDef::move_to_zone(EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("moogle_found"))), ZoneKind::Hand, ZonePlacement::Top)]) }), EffectDef::ShuffleLibrary { player: EffectRecipientDef::Controller }]) }] })
]),
);

// FIN 16 — Dion, Bahamut's Dominant // Bahamut, Warden of Light
// Audit: unsupported — Needs one printed Saga ability to represent several chapter numbers and to remain identifiable to the Saga final-chapter rules; the current Saga authoring and recognition path represents exactly one chapter per ability.
pub(in crate::card::sets) static DION_BAHAMUT_S_DOMINANT: CardRecord = CardRecord::new(
    "Dion, Bahamut's Dominant // Bahamut, Warden of Light",
    "8c0f9306-2058-476d-a711-bd37a6e15e42",
    "Kevin Glint",
    CardRules::unsupported(),
);

// FIN 17 — Dragoon's Lance
pub(in crate::card::sets) static DRAGOON_S_LANCE: CardRecord = CardRecord::new(
    "Dragoon's Lance",
    "96630531-8eb7-4e3e-8d63-60c562a5571b",
    "Josephine Chang",
    CardRules::new_artifact(mana_cost!("{1}{W}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::job_select(),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+0 and is a Knight in addition to \
                 its other types.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&[
                            "Knight",
                        ])),
                    ]),
                },
            ),
            AbilityDef::static_ability(
                "During your turn, equipped creature has flying.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::add_ability(&abilities::flying()),
                    },
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{4}"))], "Gae Bolg — Equip {4}"),
        ]),
);

// FIN 18 — Dwarven Castle Guard
pub(in crate::card::sets) static DWARVEN_CASTLE_GUARD: CardRecord = CardRecord::new(
    "Dwarven Castle Guard",
    "e17c0d27-e88d-4ba9-acbb-3f916cee3d7e",
    "Crystal Fae",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Dwarf", "Soldier"], 2, 1).with_abilities(&[
        abilities::dies_trigger(
            "When this creature dies, create a 1/1 colorless Hero creature \
             token.",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(HERO_TOKEN_2))),
        ),
    ]),
);

// FIN 19 — Fate of the Sun-Cryst
// Audit: unsupported — Needs self spell-cost reduction to inspect the chosen target and whether it is a tapped creature; self-cost evaluation cannot read selected targets.
pub(in crate::card::sets) static FATE_OF_THE_SUN_CRYST: CardRecord = CardRecord::new(
    "Fate of the Sun-Cryst",
    "900cdf11-b42e-4dcc-97c3-2e4d8e406a70",
    "Erikas Perl",
    CardRules::unsupported(),
);

// FIN 20 — From Father to Son
pub(in crate::card::sets) static FROM_FATHER_TO_SON: CardRecord = CardRecord::new(
    "From Father to Son",
    "0c730a3b-334e-466b-bb9b-4b41fce2af6d",
    "Jeremy Chong",
    CardRules::new_sorcery(mana_cost!("{1}{W}")).with_abilities(&[
        AbilityDef::spell(
            "Search your library for a Vehicle card, reveal it, and put it \
             into your hand. If this spell was cast from a graveyard, put \
             that card onto the battlefield instead. Then shuffle.",
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::SourceCastFrom(ZoneKind::Graveyard),
                then: &EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: true,
                    destination: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
                otherwise: &EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
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
            },
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{4}{W}{W}{W}"))]),
    ]),
);

// FIN 21 — G'raha Tia
pub(in crate::card::sets) static G_RAHA_TIA: CardRecord = CardRecord::new(
    "G'raha Tia",
    "076a8eca-ed73-4ee9-aab4-d9d43d394ee6",
    "Narendra Bintara Adi",
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Cat", "Archer"], 3, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::reach(),
            AbilityDef::triggered(
                "The Allagan Eye — Whenever one or more other creatures and/or \
                 artifacts you control die, draw a card. This ability triggers \
                 only once each turn.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Creature),
                            ]),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                abilities::draw_cards(ValueDef::Constant(1)),
            )
            .triggering_at_most(1),
        ]),
);

// FIN 22 — Gaelicat
pub(in crate::card::sets) static GAELICAT: CardRecord = CardRecord::new(
    "Gaelicat",
    "29606c49-e1a4-49c3-883b-9122c08bbbc7",
    "Narendra Bintara Adi",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Cat"], 1, 3).with_abilities(&[
        abilities::flying(),
        abilities::vigilance(),
        AbilityDef::static_ability(
            "As long as you control two or more artifacts, this creature \
             gets +2/+0.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    comparison: ComparisonDef::GreaterOrEqual,
                    right: ValueDef::Constant(2),
                }),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                },
            },
        ),
    ]),
);

// FIN 23 — Machinist's Arsenal
pub(in crate::card::sets) static MACHINIST_S_ARSENAL: CardRecord = CardRecord::new(
    "Machinist's Arsenal",
    "ff976428-2145-4630-aab1-08870b90b2f0",
    "Thanh Tuấn",
    CardRules::new_artifact(mana_cost!("{4}{W}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::job_select(),
            AbilityDef::static_ability(
                "Equipped creature gets +2/+2 for each artifact you control \
                 and is an Artificer in addition to its other types.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Scaled(&ScaledValueDef {
                                value: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::You,
                                )),
                                factor: 2,
                            }),
                            ValueDef::Scaled(&ScaledValueDef {
                                value: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::You,
                                )),
                                factor: 2,
                            }),
                        ),
                        AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&[
                            "Artificer",
                        ])),
                    ]),
                },
            ),
            abilities::equip(
                &[CostDef::Mana(mana_cost!("{4}"))],
                "Machina — Equip {4} ({4}: Attach to target creature you \
                 control. Equip only as a sorcery.)",
            ),
        ]),
);

// FIN 24 — Magitek Armor
pub(in crate::card::sets) static MAGITEK_ARMOR: CardRecord = CardRecord::new(
    "Magitek Armor",
    "59c4a1a2-623c-43b2-8005-ecb5c6436c10",
    "Nathaniel Himawan",
    CardRules::new_vehicle(mana_cost!("{3}{W}"), 4, 4).with_abilities(&[
        abilities::enters_trigger(
            "When this Vehicle enters, create a 1/1 colorless Hero \
             creature token.",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(HERO_TOKEN_2))),
        ),
        abilities::crew(
            "Crew 1 (Tap any number of creatures you control with total \
             power 1 or more: This Vehicle becomes an artifact creature \
             until end of turn.)",
            1,
        ),
    ]),
);

// FIN 25 — Magitek Infantry
pub(in crate::card::sets) static MAGITEK_INFANTRY: CardRecord = CardRecord::new(
    "Magitek Infantry",
    "b64dc6d7-dd01-4e66-9099-4c90865448df",
    "John Tyler Christopher",
    CardRules::new_artifact_creature(mana_cost!("{W}"), &["Robot", "Soldier"], 1, 1)
        .with_abilities(&[
            AbilityDef::static_ability(
                "This creature gets +1/+0 as long as you control another artifact.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ObjectCount {
                        query: ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Artifact),
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
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(0),
                        ),
                    },
                },
            ),
            AbilityDef::activated(
                "{2}{W}: Search your library for a card named Magitek \
                 Infantry, put it onto the battlefield tapped, then shuffle.",
                &[CostDef::Mana(mana_cost!("{2}{W}"))],
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::NameEquals(CardNameDef::Literal(
                        "Magitek Infantry",
                    )),
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
        ]),
);

// FIN 26 — Minwu, White Mage
pub(in crate::card::sets) static MINWU_WHITE_MAGE: CardRecord = CardRecord::new(
    "Minwu, White Mage",
    "6822144f-f0eb-4e10-a217-52cad36d2973",
    "Josu Hernaiz",
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Human", "Cleric"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::vigilance(),
            abilities::lifelink(),
            AbilityDef::triggered(
                "Whenever you gain life, put a +1/+1 counter on each Cleric \
                 you control.",
                TriggerEventDef::LifeGained(PlayerRelation::You),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Cleric")),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// FIN 27 — Moogles' Valor
pub(in crate::card::sets) static MOOGLES_VALOR: CardRecord = CardRecord::new(
    "Moogles' Valor",
    "caa838a7-60a9-4791-af5b-194f7574c4c8",
    "Kotakan",
    CardRules::new_instant(mana_cost!("{3}{W}{W}")).with_abilities(&[AbilityDef::spell(
        "For each creature you control, create a 1/2 white Moogle \
         creature token with lifelink. Then creatures you control gain \
         indestructible until end of turn.",
        EffectDef::Sequence(&[
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::creature(&["Moogle"], &[ManaColor::White], 1, 2)
                        .with_abilities(&[abilities::lifelink()]),
                ))
                .with_count(ValueDef::CountMatchingObjects(
                    &ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
            ),
            EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )]),
);

// FIN 28 — Paladin's Arms
pub(in crate::card::sets) static PALADIN_S_ARMS: CardRecord = CardRecord::new(
    "Paladin's Arms",
    "446506c5-5e1d-4b42-aef3-ea247d7881ef",
    "Immanuela Crovius",
    CardRules::new_artifact(mana_cost!("{2}{W}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::job_select(),
            AbilityDef::static_ability(
                "Equipped creature gets +2/+1, has ward {1}, and is a Knight \
                 in addition to its other types.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::ward(
                            &[CostDef::Mana(mana_cost!("{1}"))],
                            "Ward {1}",
                        )),
                        AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&[
                            "Knight",
                        ])),
                    ]),
                },
            ),
            abilities::equip(
                &[CostDef::Mana(mana_cost!("{4}"))],
                "Lightbringer and Hero's Shield — Equip {4} ({4}: Attach to \
                 target creature you control. Equip only as a sorcery.)",
            ),
        ]),
);

// FIN 29 — Phoenix Down
pub(in crate::card::sets) static PHOENIX_DOWN: CardRecord = CardRecord::new(
    "Phoenix Down",
    "62e299b0-9ef6-49d3-aa79-384325fed89e",
    "John Severin Brassell",
    CardRules::new_artifact(mana_cost!("{W}")).with_abilities(&[AbilityDef::modal_activated(
        "{1}{W}, {T}, Exile this artifact: Choose one —",
        &[
            CostDef::Mana(mana_cost!("{1}{W}")),
            CostDef::TapSource,
            CostDef::ExileSource,
        ],
        &[
            AbilityDef::spell_with_targets(
                "Return target creature card with mana value 4 or less from \
                 your graveyard to the battlefield tapped.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::ManaValueAtMost(4),
                        ]),
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::You),
                    },
                )],
                EffectDef::WithBattlefieldArrival {
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                    arrival: BattlefieldArrivalDef {
                        modifications: &[BattlefieldEntryModificationDef::Tapped],
                        ..BattlefieldArrivalDef::DEFAULT
                    },
                },
            ),
            AbilityDef::spell_with_targets(
                "Exile target Skeleton, Spirit, or Zombie.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Skeleton")),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spirit")),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Zombie")),
                    ]),
                )],
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
            ),
        ],
        1,
        1,
        false,
    )]),
);

// FIN 30 — Restoration Magic
pub(in crate::card::sets) static RESTORATION_MAGIC: CardRecord = CardRecord::new(
    "Restoration Magic",
    "494e68e9-ecba-4482-82bc-207ad59144c1",
    "Yumi Yaoshida",
    CardRules::new_instant(mana_cost!("{W}")).with_abilities(&[tiered(
        "Tiered (Choose one additional cost.)",
        &[
            (
                &[CostDef::Mana(mana_cost!("{0}"))],
                AbilityDef::spell_with_targets(
                    "Cure — Target permanent gains hexproof and indestructible \
                     until end of turn.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                        ]),
                    )],
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::add_ability(&abilities::hexproof()),
                            AppliedEffectDef::add_ability(&abilities::indestructible()),
                        ]),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ),
            ),
            (
                &[CostDef::Mana(mana_cost!("{1}"))],
                AbilityDef::spell_with_targets(
                    "Cura — Target permanent gains hexproof and indestructible \
                     until end of turn. You gain 3 life.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                        ]),
                    )],
                    EffectDef::Sequence(&[
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            effect: AppliedEffectDef::Composite(&[
                                AppliedEffectDef::add_ability(&abilities::hexproof()),
                                AppliedEffectDef::add_ability(&abilities::indestructible()),
                            ]),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                        EffectDef::GainLife {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(3),
                        },
                    ]),
                ),
            ),
            (
                &[CostDef::Mana(mana_cost!("{3}{W}"))],
                AbilityDef::spell(
                    "Curaga — Permanents you control gain hexproof and \
                     indestructible until end of turn. You gain 6 life.",
                    EffectDef::Sequence(&[
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                                ObjectQueryDef::matching(
                                    ObjectPredicateDef::AnyOf(&[
                                        ObjectPredicateDef::HasType(CardType::Creature),
                                        ObjectPredicateDef::HasType(CardType::Artifact),
                                        ObjectPredicateDef::HasType(CardType::Enchantment),
                                        ObjectPredicateDef::HasType(CardType::Land),
                                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                                    ]),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::You,
                                ),
                            )),
                            effect: AppliedEffectDef::Composite(&[
                                AppliedEffectDef::add_ability(&abilities::hexproof()),
                                AppliedEffectDef::add_ability(&abilities::indestructible()),
                            ]),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                        EffectDef::GainLife {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(6),
                        },
                    ]),
                ),
            ),
        ],
    )]),
);

// FIN 31 — Sidequest: Catch a Fish // Cooking Campsite
pub(in crate::card::sets) static SIDEQUEST_CATCH_A_FISH: CardRecord = CardRecord::new_dfc(
    "Sidequest: Catch a Fish // Cooking Campsite",
    "bdb5452e-d97f-409b-91d0-2664f39b09b8",
    "Gal Or",
    &[
        (
            "Sidequest: Catch a Fish",
            CardRules::new_enchantment(mana_cost!("{2}{W}")).with_abilities(&[
                AbilityDef::triggered(
                    "At the beginning of your upkeep, look at the top card of your \
                     library. If it's an artifact or creature card, you may reveal \
                     it and put it into your hand. If you put a card into your \
                     hand this way, create a Food token and transform this \
                     enchantment.",
                    TriggerEventDef::StepBegins {
                        step: TurnStepDef::Upkeep,
                        player: PlayerRelation::You,
                    },
                    EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                        source: ObjectCollectionSourceDef::TopCards {
                            player: PlayerRefDef::EffectController,
                            count: ValueDef::Constant(1),
                        },
                        actor: PlayerRefDef::EffectController,
                        inspection: CollectionInspectionDef::Look,
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Creature),
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
                            EffectDef::WithZoneMoveResult {
                                effect: &EffectDef::move_to_zone(
                                    EffectRecipientDef::objects(ObjectSetDef::Binding(
                                        crate::Binding!("chosen"),
                                    )),
                                    ZoneKind::Hand,
                                    ZonePlacement::Top,
                                ),
                                binding: crate::Binding!("caught"),
                                then: &EffectDef::IfCondition {
                                    condition: &TriggerConditionDef::ValueComparison(
                                        &ValueComparisonDef {
                                            left: ValueDef::CountObjects(
                                                &ObjectSetDef::ZoneChangeSuccessorsOfBinding(
                                                    crate::Binding!("caught"),
                                                ),
                                            ),
                                            comparison: ComparisonDef::Greater,
                                            right: ValueDef::Constant(0),
                                        },
                                    ),
                                    then: &EffectDef::Sequence(&[
                                        EffectDef::CreateToken(
                                            CreateTokenDef::new(TokenDef::Literal(FOOD_TOKEN))
                                                .with_count(ValueDef::Constant(1)),
                                        ),
                                        EffectDef::Transform {
                                            object: EffectRecipientDef::Source,
                                        },
                                    ]),
                                },
                            },
                        ]),
                    }),
                ),
            ]),
        ),
        (
            "Cooking Campsite",
            CardRules::new_land(&[])
                .printed_colors(&[])
                .with_abilities(&[
                    abilities::tap_for(ManaColor::White),
                    AbilityDef::activated(
                        "{3}, {T}, Sacrifice an artifact: Put a +1/+1 counter on each \
                         creature you control. Activate only as a sorcery.",
                        &[
                            CostDef::Mana(mana_cost!("{3}")),
                            CostDef::TapSource,
                            CostDef::sacrifice_permanent(ObjectPredicateDef::HasType(
                                CardType::Artifact,
                            )),
                        ],
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::objects(ObjectSetDef::Query(
                                ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::You,
                                ),
                            )),
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(1),
                        },
                    )
                    .with_activation_timing(ActivationTimingDef::SorcerySpeed),
                ]),
        ),
    ],
);

// FIN 32 — Slash of Light
pub(in crate::card::sets) static SLASH_OF_LIGHT: CardRecord = CardRecord::new(
    "Slash of Light",
    "da6d9529-3cb0-4adc-8209-b9b02db3bf54",
    "Nathaniel Himawan",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Slash of Light deals damage equal to the number of creatures \
         you control plus the number of Equipment you control to \
         target creature.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::damage(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ValueDef::Sum(&SumValueDef::new(
                ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
                ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Equipment")),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
            )),
        ),
    )]),
);

// FIN 33 — Snow Villiers
pub(in crate::card::sets) static SNOW_VILLIERS: CardRecord = CardRecord::new(
    "Snow Villiers",
    "399bb699-e61d-4b41-b6e9-e594cbad6194",
    "Fariba Khamseh",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Rebel", "Monk"], 0, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::vigilance(),
            AbilityDef::static_ability(
                "Snow Villiers's power is equal to the number of creatures you \
                 control.",
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
        ]),
);

// FIN 34 — Stiltzkin, Moogle Merchant
// Audit: unsupported — Needs an actual gain-control result binding so drawing happens only when the opponent gains control; testing the final controller cannot distinguish a successful change from a prevented or redundant change.
pub(in crate::card::sets) static STILTZKIN_MOOGLE_MERCHANT: CardRecord = CardRecord::new(
    "Stiltzkin, Moogle Merchant",
    "06a972a4-0c1b-4f12-a5a5-fdea47c4cd35",
    "Hendry Iwanaga",
    CardRules::unsupported(),
);

// FIN 35 — Summon: Choco/Mog
// Audit: unsupported — Needs one printed Saga ability to represent several chapter numbers and to remain identifiable to the Saga final-chapter rules; the current Saga authoring and recognition path represents exactly one chapter per ability.
pub(in crate::card::sets) static SUMMON_CHOCO_MOG: CardRecord = CardRecord::new(
    "Summon: Choco/Mog",
    "00546117-018a-4286-bc20-b5446c5be56f",
    "Madeline Boni",
    CardRules::unsupported(),
);

// FIN 36 — Summon: Knights of Round
// Audit: unsupported — Needs one printed Saga ability to represent several chapter numbers and to remain identifiable to the Saga final-chapter rules; the current Saga authoring and recognition path represents exactly one chapter per ability.
pub(in crate::card::sets) static SUMMON_KNIGHTS_OF_ROUND: CardRecord = CardRecord::new(
    "Summon: Knights of Round",
    "44d23652-077e-4c1f-b640-b284685db911",
    "Takayama Toshiaki",
    CardRules::unsupported(),
);

// FIN 37 — Summon: Primal Garuda
// Audit: unsupported — Needs one printed Saga ability to represent several chapter numbers and to remain identifiable to the Saga final-chapter rules; the current Saga authoring and recognition path represents exactly one chapter per ability.
pub(in crate::card::sets) static SUMMON_PRIMAL_GARUDA: CardRecord = CardRecord::new(
    "Summon: Primal Garuda",
    "e44497a8-067e-454e-a9c0-684f03df55ff",
    "Madeline Boni",
    CardRules::unsupported(),
);

// FIN 38 — Ultima
// Audit: unsupported — Needs an end-the-turn procedure that exiles the stack, ends pending actions, and runs cleanup including hand-size discards and expiring effects; scheduling ordinary phases cannot end the turn this way.
pub(in crate::card::sets) static ULTIMA: CardRecord = CardRecord::new(
    "Ultima",
    "39504a0e-f63f-4907-afd7-c4492f6b8a3b",
    "Gintas Galvanauskas",
    CardRules::unsupported(),
);

// FIN 39 — Venat, Heart of Hydaelyn // Hydaelyn, the Mothercrystal
pub(in crate::card::sets) static VENAT_HEART_OF_HYDAELYN: CardRecord = CardRecord::new_dfc(
    "Venat, Heart of Hydaelyn // Hydaelyn, the Mothercrystal",
    "2625c00d-0a51-4481-bf36-cf13a2546242",
    "Colin Boyer",
    &[
        (
            "Venat, Heart of Hydaelyn",
            CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Elder", "Wizard"], 3, 3)
                .with_supertype(CardSupertype::Legendary)
                .with_abilities(&[
                    AbilityDef::triggered(
                        "Whenever you cast a legendary spell, draw a card. This \
                         ability triggers only once each turn.",
                        TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ])),
                        abilities::draw_cards(ValueDef::Constant(1)),
                    )
                    .triggering_at_most(1),
                    AbilityDef::activated_with_targets(
                        "Hero's Sundering — {7}, {T}: Exile target nonland permanent. \
                         Transform Venat. Activate only as a sorcery.",
                        &[CostDef::Mana(mana_cost!("{7}")), CostDef::TapSource],
                        &[AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        )],
                        EffectDef::Sequence(&[
                            EffectDef::move_to_zone(
                                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                ZoneKind::Exile,
                                ZonePlacement::Top,
                            ),
                            EffectDef::Transform {
                                object: EffectRecipientDef::Source,
                            },
                        ]),
                    )
                    .with_activation_timing(ActivationTimingDef::SorcerySpeed),
                ]),
        ),
        (
            "Hydaelyn, the Mothercrystal",
            CardRules::new_creature_without_mana_cost(&["God"], 4, 4)
                .with_supertype(CardSupertype::Legendary)
                .printed_colors(&[ManaColor::White])
                .with_abilities(&[
                    abilities::indestructible(),
                    AbilityDef::triggered_with_targets(
                        "Blessing of Light — At the beginning of combat on your turn, \
                         put a +1/+1 counter on another target creature you control. \
                         Until your next turn, it gains indestructible. If that \
                         creature is legendary, draw a card.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::BeginningOfCombat,
                            player: PlayerRelation::You,
                        },
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
                            EffectDef::AddCounters {
                                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                kind: CounterKind::PlusOnePlusOne,
                                amount: ValueDef::Constant(1),
                            },
                            EffectDef::Apply {
                                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                                duration: ResolvedEffectDurationDef::UntilYourNextTurn,
                            },
                            EffectDef::IfCondition {
                                condition: &TriggerConditionDef::TargetMatches {
                                    slot: TargetIndex::PRIMARY,
                                    object: ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                                },
                                then: &abilities::draw_cards(ValueDef::Constant(1)),
                            },
                        ]),
                    ),
                ]),
        ),
    ],
);

// FIN 40 — Weapons Vendor
// Audit: unsupported — Needs a reflexive trigger after an optional payment that survives the original source leaving, plus attachment between two independently selected permanents; OptionalEffectTaken currently discovers only battlefield listeners.
pub(in crate::card::sets) static WEAPONS_VENDOR: CardRecord = CardRecord::new(
    "Weapons Vendor",
    "c9e6b374-3e44-4df7-b0a3-4ef98dc08267",
    "Mushk Rizvi",
    CardRules::unsupported(),
);

// FIN 41 — White Auracite
// Audit: unsupported — Needs immediate return of the linked exiled object as part of the source leaving the battlefield; the current linked-exile helper uses a separate leaves trigger that can be responded to or countered.
pub(in crate::card::sets) static WHITE_AURACITE: CardRecord = CardRecord::new(
    "White Auracite",
    "2df6f515-9d21-4769-b1c4-c219611bccbb",
    "Magali Villeneuve",
    CardRules::unsupported(),
);

// FIN 42 — White Mage's Staff
pub(in crate::card::sets) static WHITE_MAGE_S_STAFF: CardRecord = CardRecord::new(
    "White Mage's Staff",
    "30db372e-0b4c-4e16-9667-bf3fda666f72",
    "Kim Dingwall",
    CardRules::new_artifact(mana_cost!("{1}{W}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::job_select(),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1, has \"Whenever this creature \
                 attacks, you gain 1 life,\" and is a Cleric in addition to \
                 its other types.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&AbilityDef::triggered(
                            "Whenever this creature attacks, you gain 1 life.",
                            TriggerEventDef::attacks(ObjectPredicateDef::Source),
                            EffectDef::GainLife {
                                recipient: EffectRecipientDef::Controller,
                                amount: ValueDef::Constant(1),
                            },
                        )),
                        AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&[
                            "Cleric",
                        ])),
                    ]),
                },
            ),
            abilities::equip(
                &[CostDef::Mana(mana_cost!("{3}"))],
                "Equip {3} ({3}: Attach to target creature you control. Equip \
                 only as a sorcery.)",
            ),
        ]),
);

// FIN 43 — The Wind Crystal
pub(in crate::card::sets) static THE_WIND_CRYSTAL: CardRecord = CardRecord::new(
    "The Wind Crystal",
    "19bd0885-baaa-40f2-9c59-b1ea53807540",
    "Pablo Mendoza",
    CardRules::new_artifact(mana_cost!("{2}{W}{W}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "White spells you cast cost {1} less to cast.",
                EffectDef::ModifyCost(CostModificationDef::reduce_spell(
                    ObjectPredicateDef::Color(ManaColor::White),
                    PlayerRelation::You,
                    ValueDef::Constant(1),
                )),
            ),
            AbilityDef::replacement_for(
                "If you would gain life, you gain twice that much life instead.",
                ReplacementEventDef::WouldGainLife(PlayerRelation::You),
                ReplacementEffectDef::MultiplyEventAmount(2),
            ),
            AbilityDef::activated(
                "{4}{W}{W}, {T}: Creatures you control gain flying and \
                 lifelink until end of turn.",
                &[CostDef::Mana(mana_cost!("{4}{W}{W}")), CostDef::TapSource],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_ability(&abilities::flying()),
                        AppliedEffectDef::add_ability(&abilities::lifelink()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// FIN 44 — You're Not Alone
pub(in crate::card::sets) static YOU_RE_NOT_ALONE: CardRecord = CardRecord::new(
    "You're Not Alone",
    "1867b5cb-2bb0-4f49-b302-036fdffa2344",
    "Ignatius Budi",
    CardRules::new_instant(mana_cost!("{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +2/+2 until end of turn. If you control \
         three or more creatures, it gets +4/+4 until end of turn \
         instead.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::IfElseCondition {
            condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
                comparison: ComparisonDef::GreaterOrEqual,
                right: ValueDef::Constant(3),
            }),
            then: &EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(4),
                    ValueDef::Constant(4),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            otherwise: &EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        },
    )]),
);

// FIN 45 — Zack Fair
// Audit: unsupported — Needs transfer of the sacrificed source's complete counter inventory and an Equipment chosen from its last-known attachments; current counter effects name a fixed counter kind and attachment queries read the current battlefield.
pub(in crate::card::sets) static ZACK_FAIR: CardRecord = CardRecord::new(
    "Zack Fair",
    "f21f9161-5945-40da-8da0-446f6a4a1c23",
    "Yoshio Sugiura",
    CardRules::unsupported(),
);

// FIN 46 — Astrologian's Planisphere (alternate printing)
const ASTROLOGIAN_S_PLANISPHERE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ASTROLOGIAN_S_PLANISPHERE,
    1,
    "bfa4e927-1d6f-4a64-9801-7d168a5ef3f6",
    "Josephine Chang",
);

// FIN 47 — Cargo Ship
// Audit: unsupported — Needs a mana restriction allowing either casting an artifact spell or activating an artifact ability; multiple current restrictions are conjunctive rather than alternative spending permissions.
pub(in crate::card::sets) static CARGO_SHIP: CardRecord = CardRecord::new(
    "Cargo Ship",
    "932b865c-bfe7-4bb7-82e9-2403cf0e0522",
    "Thanh Tuấn",
    CardRules::unsupported(),
);

// FIN 48 — Combat Tutorial
pub(in crate::card::sets) static COMBAT_TUTORIAL: CardRecord = CardRecord::new(
    "Combat Tutorial",
    "ec195607-ac30-4931-acfd-f9d8ac8b047f",
    "Fang Xinyu",
    CardRules::new_sorcery(mana_cost!("{2}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target player draws two cards. Put a +1/+1 counter on up to \
         one target creature you control.",
        &[
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Any)),
            AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
                1,
            ),
        ],
        EffectDef::Sequence(&[
            abilities::draw_cards(ValueDef::Constant(2)),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex(1)),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ]),
    )]),
);

// FIN 49 — Dragoon's Wyvern
pub(in crate::card::sets) static DRAGOON_S_WYVERN: CardRecord = CardRecord::new(
    "Dragoon's Wyvern",
    "d92bce20-308e-4841-aaf8-8e20698292e7",
    "Jason Kiantoro",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Drake"], 2, 1).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, create a 1/1 colorless Hero \
             creature token.",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(HERO_TOKEN_2))),
        ),
    ]),
);

// FIN 50 — Dreams of Laguna
pub(in crate::card::sets) static DREAMS_OF_LAGUNA: CardRecord = CardRecord::new(
    "Dreams of Laguna",
    "ba752243-2727-4b8a-8e21-e70becfd4ff3",
    "Solan",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::spell(
            "Surveil 1, then draw a card. (To surveil 1, look at the top \
             card of your library. You may put it into your graveyard.)",
            EffectDef::Sequence(&[
                abilities::surveil(ValueDef::Constant(1)),
                abilities::draw_cards(ValueDef::Constant(1)),
            ]),
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{3}{U}"))]),
    ]),
);

// FIN 51 — Edgar, King of Figaro
// Audit: unsupported — Needs a replacement for the first coin-flip batch of each turn that forces every coin to heads and every flip to be won, including flips that happened before Edgar entered.
pub(in crate::card::sets) static EDGAR_KING_OF_FIGARO: CardRecord = CardRecord::new(
    "Edgar, King of Figaro",
    "950ee302-5512-43c5-ac7c-b2b06f4177bf",
    "Jake Murray",
    CardRules::unsupported(),
);

// FIN 52 — Eject
pub(in crate::card::sets) static EJECT: CardRecord = CardRecord::new(
    "Eject",
    "aec83c9a-8ec4-4a5a-b27f-0e74a2b3d21e",
    "Ramza Psyru",
    CardRules::new_instant(mana_cost!("{3}{U}")).with_abilities(&[
        abilities::cannot_be_countered(),
        AbilityDef::spell_with_targets(
            "Return target nonland permanent to its owner's hand.\nDraw a \
             card.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
            )],
            EffectDef::Sequence(&[
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
                abilities::draw_cards(ValueDef::Constant(1)),
            ]),
        ),
    ]),
);

// FIN 53 — Ether
// Audit: unsupported — Needs a source-independent delayed trigger that fires only on the next matching spell this turn and expires if unused; installed Once and ThisTurn lifetimes cannot currently be combined.
pub(in crate::card::sets) static ETHER: CardRecord = CardRecord::new(
    "Ether",
    "896ee6e9-15a9-4974-b576-50f4759fac38",
    "Ben Wootten",
    CardRules::unsupported(),
);

// FIN 54 — Gogo, Master of Mimicry
// Audit: unsupported — Needs a rule preventing the activated ability itself from being copied; copying a targeted ability is supported but the copying prohibition is not represented.
pub(in crate::card::sets) static GOGO_MASTER_OF_MIMICRY: CardRecord = CardRecord::new(
    "Gogo, Master of Mimicry",
    "cce4eb99-d960-4ab7-911a-bb4ea74d1775",
    "Thea Dumitriu",
    CardRules::unsupported(),
);

// FIN 55 — Ice Flan
pub(in crate::card::sets) static ICE_FLAN: CardRecord = CardRecord::new(
    "Ice Flan",
    "ad304c9c-943f-442f-bb82-ff378ad7d7ba",
    "SHOSUKE",
    CardRules::new_creature(mana_cost!("{4}{U}{U}"), &["Elemental", "Ooze"], 5, 4).with_abilities(
        &[
            abilities::enters_trigger_with_targets(
                "When this creature enters, tap target artifact or creature an \
                 opponent controls. Put a stun counter on it. (If a permanent \
                 with a stun counter would become untapped, remove one from it \
                 instead.)",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Creature),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::Opponent),
                        owner: None,
                    },
                )],
                EffectDef::Sequence(&[
                    EffectDef::Tap {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::Stun,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
            abilities::typecycling!(
                "Islandcycling {2}",
                &[CostDef::Mana(mana_cost!("{2}"))],
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Island"))
            ),
        ],
    ),
);

// FIN 56 — Ice Magic
pub(in crate::card::sets) static ICE_MAGIC: CardRecord = CardRecord::new(
    "Ice Magic",
    "9dabd626-7ec3-4913-babb-d5d3fd5e32d5",
    "Masateru Ikeda",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[tiered(
        "Tiered (Choose one additional cost.)",
        &[
            (
                &[CostDef::Mana(mana_cost!("{0}"))],
                AbilityDef::spell_with_targets(
                    "Blizzard — Return target creature to its owner's hand.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    )],
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                ),
            ),
            (
                &[CostDef::Mana(mana_cost!("{2}"))],
                AbilityDef::spell_with_targets(
                    "Blizzara — Target creature's owner puts it on their choice of \
                     the top or bottom of their library.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    )],
                    EffectDef::ChooseEffect {
                        player: EffectRecipientDef::player(PlayerRefDef::OwnerOf(
                            ObjectRefDef::Target(TargetIndex::PRIMARY),
                        )),
                        choices: &[
                            EffectChoiceDef {
                                label: "Top",
                                effect: EffectDef::move_to_zone(
                                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                    ZoneKind::Library,
                                    ZonePlacement::Top,
                                ),
                            },
                            EffectChoiceDef {
                                label: "Bottom",
                                effect: EffectDef::move_to_zone(
                                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                    ZoneKind::Library,
                                    ZonePlacement::Bottom,
                                ),
                            },
                        ],
                    },
                ),
            ),
            (
                &[CostDef::Mana(mana_cost!("{5}{U}"))],
                AbilityDef::spell_with_targets(
                    "Blizzaga — Target creature's owner shuffles it into their \
                     library.",
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
            ),
        ],
    )]),
);

// FIN 57 — Il Mheg Pixie
pub(in crate::card::sets) static IL_MHEG_PIXIE: CardRecord = CardRecord::new(
    "Il Mheg Pixie",
    "ae612312-3a8e-495f-8730-deaaf7505ca1",
    "Ramza Psyru",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Faerie"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever this creature attacks, surveil 1. (Look at the top \
             card of your library. You may put it into your graveyard.)",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            abilities::surveil(ValueDef::Constant(1)),
        ),
    ]),
);

// FIN 58 — Jill, Shiva's Dominant // Shiva, Warden of Ice
// Audit: unsupported — Needs one printed Saga ability to represent several chapter numbers and to remain identifiable to the Saga final-chapter rules; the current Saga authoring and recognition path represents exactly one chapter per ability.
pub(in crate::card::sets) static JILL_SHIVA_S_DOMINANT: CardRecord = CardRecord::new(
    "Jill, Shiva's Dominant // Shiva, Warden of Ice",
    "1f163763-4802-4a96-a5bc-f3c381db7b5c",
    "Arif Wijaya",
    CardRules::unsupported(),
);

// FIN 59 — Louisoix's Sacrifice
pub(in crate::card::sets) static LOUISOIX_S_SACRIFICE: CardRecord = CardRecord::new(
    "Louisoix's Sacrifice",
    "4a6976f2-0bd5-449a-8fcf-f5a732ce22c1",
    "Mintautas Šukys",
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Counter target activated ability, triggered ability, or \
         noncreature spell.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Ability,
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Spell,
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                    ]),
                ]),
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::counter_target(TargetIndex::PRIMARY),
    )
    .with_spell_additional_cost(&CostDef::Choice(&[
        CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Supertype(CardSupertype::Legendary),
        ])),
        CostDef::Mana(mana_cost!("{2}")),
    ]))]),
);

// FIN 60 — The Lunar Whale
pub(in crate::card::sets) static THE_LUNAR_WHALE: CardRecord = CardRecord::new(
    "The Lunar Whale",
    "ae875471-346c-4a76-b26f-b7205dad5b80",
    "Fiona Hsieh",
    CardRules::new_vehicle(mana_cost!("{3}{U}"), 3, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::static_ability(
                "You may look at the top card of your library any time.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Controller,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayLookAtTopOfLibrary),
                },
            ),
            AbilityDef::static_ability(
                "As long as The Lunar Whale attacked this turn, you may play \
                 the top card of your library.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceMatches {
                        object: ObjectPredicateDef::AttackedThisTurn,
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Controller,
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromTopOfLibrary {
                            restriction: PlayRestrictionDef::new(
                                PlayActionMatcherDef::Any,
                                ObjectPredicateDef::Any,
                            ),
                            cost: TopOfLibraryCostDef::Printed,
                        }),
                    },
                },
            ),
            abilities::crew("Crew 1", 1),
        ]),
);

// FIN 61 — Magic Damper
pub(in crate::card::sets) static MAGIC_DAMPER: CardRecord = CardRecord::new(
    "Magic Damper",
    "44921b2e-5938-4f63-92b9-0b719a2f8c68",
    "YASUNARI HIRASAKA",
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature you control gets +1/+1 and gains hexproof \
         until end of turn. Untap it.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    AppliedEffectDef::add_ability(&abilities::hexproof()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ]),
    )]),
);

// FIN 62 — Matoya, Archon Elder
// Audit: unsupported — Needs completed-scry and completed-surveil events, emitted after those choices finish; neither action currently exposes a trigger event.
pub(in crate::card::sets) static MATOYA_ARCHON_ELDER: CardRecord = CardRecord::new(
    "Matoya, Archon Elder",
    "1dd61cf6-2fb5-4cff-ab00-7677ac85774c",
    "Luisa J. Preissler",
    CardRules::unsupported(),
);

// FIN 63 — Memories Returning
pub(in crate::card::sets) static MEMORIES_RETURNING: CardRecord = CardRecord::new(
    "Memories Returning",
    "a753abfc-35d3-4faf-ab35-3b51aa778174",
    "Grace Zhu",
    CardRules::new_sorcery(mana_cost!("{2}{U}{U}")).with_abilities(&[
        AbilityDef::spell(
            "Reveal the top five cards of your library. Put one of them \
             into your hand. Then choose an opponent. They put one on the \
             bottom of your library. Then you put one into your hand. Then \
             they put one on the bottom of your library. Put the other \
             into your hand.",
            EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(5),
                },
                actor: PlayerRefDef::EffectController,
                inspection: CollectionInspectionDef::Reveal,
                object: ObjectPredicateDef::Any,
                minimum: 1,
                maximum: 1,
                chosen: crate::Binding!("chosen"),
                remainder: crate::Binding!("rest"),
                then: &EffectDef::Sequence(&[
                    EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "chosen"
                        ))),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                    EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                        source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::Binding(
                            crate::Binding!("rest"),
                        )),
                        actor: PlayerRefDef::Opponent,
                        inspection: CollectionInspectionDef::Reveal,
                        object: ObjectPredicateDef::Any,
                        minimum: 1,
                        maximum: 1,
                        chosen: crate::Binding!("bottom_first"),
                        remainder: crate::Binding!("after_first_bottom"),
                        then: &EffectDef::Sequence(&[
                            EffectDef::move_to_zone(
                                EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("bottom_first"),
                                )),
                                ZoneKind::Library,
                                ZonePlacement::Bottom,
                            ),
                            EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                                source: ObjectCollectionSourceDef::ObjectSet(
                                    ObjectSetDef::Binding(crate::Binding!("after_first_bottom")),
                                ),
                                actor: PlayerRefDef::EffectController,
                                inspection: CollectionInspectionDef::Reveal,
                                object: ObjectPredicateDef::Any,
                                minimum: 1,
                                maximum: 1,
                                chosen: crate::Binding!("second_hand"),
                                remainder: crate::Binding!("after_second"),
                                then: &EffectDef::Sequence(&[
                                    EffectDef::move_to_zone(
                                        EffectRecipientDef::objects(ObjectSetDef::Binding(
                                            crate::Binding!("second_hand"),
                                        )),
                                        ZoneKind::Hand,
                                        ZonePlacement::Top,
                                    ),
                                    EffectDef::ChooseCardsFromCollection(
                                        ChooseCardsFromCollectionDef {
                                            source: ObjectCollectionSourceDef::ObjectSet(
                                                ObjectSetDef::Binding(crate::Binding!(
                                                    "after_second"
                                                )),
                                            ),
                                            actor: PlayerRefDef::Opponent,
                                            inspection: CollectionInspectionDef::Reveal,
                                            object: ObjectPredicateDef::Any,
                                            minimum: 1,
                                            maximum: 1,
                                            chosen: crate::Binding!("bottom_second"),
                                            remainder: crate::Binding!("last"),
                                            then: &EffectDef::Sequence(&[
                                                EffectDef::move_to_zone(
                                                    EffectRecipientDef::objects(
                                                        ObjectSetDef::Binding(crate::Binding!(
                                                            "bottom_second"
                                                        )),
                                                    ),
                                                    ZoneKind::Library,
                                                    ZonePlacement::Bottom,
                                                ),
                                                EffectDef::move_to_zone(
                                                    EffectRecipientDef::objects(
                                                        ObjectSetDef::Binding(crate::Binding!(
                                                            "last"
                                                        )),
                                                    ),
                                                    ZoneKind::Hand,
                                                    ZonePlacement::Top,
                                                ),
                                            ]),
                                        },
                                    ),
                                ]),
                            }),
                        ]),
                    }),
                ]),
            }),
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{7}{U}{U}"))]),
    ]),
);

// FIN 64 — The Prima Vista
// Audit: unsupported — Needs total mana spent recorded with a spell-cast event and readable by its triggered ability, including additional costs and cost reductions; the current cast-event values expose mana value and colors spent, not the amount paid.
pub(in crate::card::sets) static THE_PRIMA_VISTA: CardRecord = CardRecord::new(
    "The Prima Vista",
    "e3998132-5746-4dde-9529-97d3ad7d7361",
    "Leon Tukker",
    CardRules::unsupported(),
);

// FIN 65 — Qiqirn Merchant
pub(in crate::card::sets) static QIQIRN_MERCHANT: CardRecord = CardRecord::new(
    "Qiqirn Merchant",
    "a75a6ecc-a6a5-462c-bd92-ae57dde9b965",
    "Andrea Tentori Montalto",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Beast", "Citizen"], 1, 4).with_abilities(&[
        AbilityDef::activated(
            "{1}, {T}: Draw a card, then discard a card.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
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
        AbilityDef::activated(
            "{7}, {T}, Sacrifice this creature: Draw three cards. This \
             ability costs {1} less to activate for each Town you control.",
            &[
                CostDef::Mana(mana_cost!("{7}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            abilities::draw_cards(ValueDef::Constant(3)),
        )
        .with_activation_cost_reduction(
            ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Town")),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            )),
            0,
        ),
    ]),
);

// FIN 66 — Quistis Trepe
// Audit: unsupported — Needs an immediate targeted graveyard-cast offer at the printed cost that permits mana of any type; the granted alternative-cast offer has no per-offer unrestricted-mana payment policy.
pub(in crate::card::sets) static QUISTIS_TREPE: CardRecord = CardRecord::new(
    "Quistis Trepe",
    "61784cbd-92e9-43c7-a1a8-4004b1bf4dae",
    "Touge369",
    CardRules::unsupported(),
);

// FIN 67 — Relm's Sketching
pub(in crate::card::sets) static RELM_S_SKETCHING: CardRecord = CardRecord::new(
    "Relm's Sketching",
    "6aedac12-3714-4a81-bd4d-1d2555c66f78",
    "Smirtouille",
    CardRules::new_sorcery(mana_cost!("{2}{U}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Create a token that's a copy of target artifact, creature, or \
             land.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    ObjectPredicateDef::HasType(CardType::Land),
                ]),
            )],
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Copy(&TokenCopyDef {
                object: &EffectRecipientDef::Target(TargetIndex::PRIMARY),
                exceptions: CopyExceptionsDef::NONE,
            }))),
        ),
    ]),
);

// FIN 68 — Retrieve the Esper
pub(in crate::card::sets) static RETRIEVE_THE_ESPER: CardRecord = CardRecord::new(
    "Retrieve the Esper",
    "ebd733f0-8883-434a-b36c-ef76b091fe8e",
    "Jake Murray",
    CardRules::new_sorcery(mana_cost!("{3}{U}")).with_abilities(&[
        AbilityDef::spell(
            "Create a 3/3 blue Robot Warrior artifact creature token. Then \
             if this spell was cast from a graveyard, put two +1/+1 \
             counters on that token.",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TokenCharacteristics::artifact_creature(
                    &["Robot", "Warrior"],
                    &[ManaColor::Blue],
                    3,
                    3,
                )))
                .with_created_tokens(CreatedTokensDef {
                    binding: crate::Binding!("esper"),
                    then: &EffectDef::IfCondition {
                        condition: &TriggerConditionDef::SourceCastFrom(ZoneKind::Graveyard),
                        then: &EffectDef::AddCounters {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("esper"),
                            )),
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(2),
                        },
                    },
                }),
            ),
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{5}{U}"))]),
    ]),
);

// FIN 69 — Rook Turret
pub(in crate::card::sets) static ROOK_TURRET: CardRecord = CardRecord::new(
    "Rook Turret",
    "4572884d-0c0e-41e7-b219-f76b95fdbd01",
    "Thanh Tuấn",
    CardRules::new_artifact_creature(mana_cost!("{3}{U}"), &["Construct"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever another artifact you control enters, you may draw a \
             card. If you do, discard a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Sequence(&[
                    abilities::draw_cards(ValueDef::Constant(1)),
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: None,
                    },
                ]),
            },
        ),
    ]),
);

// FIN 70 — Sage's Nouliths
pub(in crate::card::sets) static SAGE_S_NOULITHS: CardRecord = CardRecord::new(
    "Sage's Nouliths",
    "a12ff7c3-6ae0-4098-9240-ff3fd16a5288",
    "Justyna Dura",
    CardRules::new_artifact(mana_cost!("{1}{U}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::job_select(),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+0, has \"Whenever this creature \
                 attacks, untap target attacking creature,\" and is a Cleric \
                 in addition to its other types.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::add_ability(&AbilityDef::triggered_with_targets(
                            "Whenever this creature attacks, untap target attacking creature.",
                            TriggerEventDef::attacks(ObjectPredicateDef::Source),
                            &[AbilityTargetDef::exactly_one_permanent(
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::Attacking,
                                ]),
                            )],
                            EffectDef::Untap {
                                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            },
                        )),
                        AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&[
                            "Cleric",
                        ])),
                    ]),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{3}"))], "Hagneia — Equip {3}"),
        ]),
);

// FIN 71 — Sahagin
// Audit: unsupported — Needs total mana spent recorded with a spell-cast event and readable by its triggered ability, including additional costs and cost reductions; the current cast-event values expose mana value and colors spent, not the amount paid.
pub(in crate::card::sets) static SAHAGIN: CardRecord = CardRecord::new(
    "Sahagin",
    "516940c7-c271-4f64-af75-c7ba98548382",
    "Nino Is",
    CardRules::unsupported(),
);

// FIN 72 — Scorpion Sentinel
pub(in crate::card::sets) static SCORPION_SENTINEL: CardRecord = CardRecord::new(
    "Scorpion Sentinel",
    "08ab5220-e5c1-472e-8217-97fd60e1773c",
    "HAISIRO",
    CardRules::new_artifact_creature(mana_cost!("{1}{U}"), &["Robot", "Scorpion"], 1, 4)
        .with_abilities(&[AbilityDef::static_ability(
            "As long as you control seven or more lands, this creature \
             gets +3/+0.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    comparison: ComparisonDef::GreaterOrEqual,
                    right: ValueDef::Constant(7),
                }),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(0),
                    ),
                },
            },
        )]),
);

// FIN 73 — Sidequest: Card Collection // Magicked Card
pub(in crate::card::sets) static SIDEQUEST_CARD_COLLECTION: CardRecord = CardRecord::new_dfc(
    "Sidequest: Card Collection // Magicked Card",
    "8ac3d2c9-5978-4cfb-a746-c901decff093",
    "Erikas Perl & Jurijus Chitrovas",
    &[
        (
            "Sidequest: Card Collection",
            CardRules::new_enchantment(mana_cost!("{3}{U}")).with_abilities(&[
                abilities::enters_trigger(
                    "When this enchantment enters, draw three cards, then discard \
                     two cards.",
                    EffectDef::Sequence(&[
                        abilities::draw_cards(ValueDef::Constant(3)),
                        EffectDef::Discard {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(2),
                            selection: DiscardSelectionDef::RecipientChooses,
                            then: None,
                        },
                    ]),
                ),
                AbilityDef::triggered_if(
                    "At the beginning of your end step, if eight or more cards are \
                     in your graveyard, transform this enchantment.",
                    TriggerEventDef::StepBegins {
                        step: TurnStepDef::End,
                        player: PlayerRelation::You,
                    },
                    &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Any,
                            &[ZoneKind::Graveyard],
                            PlayerRelation::You,
                        )),
                        comparison: ComparisonDef::GreaterOrEqual,
                        right: ValueDef::Constant(8),
                    }),
                    EffectDef::Transform {
                        object: EffectRecipientDef::Source,
                    },
                ),
            ]),
        ),
        (
            "Magicked Card",
            CardRules::new_vehicle_without_mana_cost(4, 4)
                .printed_colors(&[ManaColor::Blue])
                .with_abilities(&[
                    abilities::flying(),
                    abilities::crew(
                        "Crew 1 (Tap any number of creatures you control with total \
                         power 1 or more: This Vehicle becomes an artifact creature \
                         until end of turn.)",
                        1,
                    ),
                ]),
        ),
    ],
);

// FIN 74 — Sleep Magic
pub(in crate::card::sets) static SLEEP_MAGIC: CardRecord = CardRecord::new(
    "Sleep Magic",
    "c96cae63-7625-48e3-aaba-5b1632a8642d",
    "Le Vuong",
    CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            abilities::enters_trigger(
                "When this Aura enters, tap enchanted creature.",
                EffectDef::Tap {
                    object: EffectRecipientDef::AttachedPermanent,
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature doesn't untap during its controller's \
                 untap step.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
                },
            ),
            AbilityDef::triggered(
                "When enchanted creature is dealt damage, sacrifice this Aura.",
                TriggerEventDef::DamageDealt(DamageEventMatcherDef {
                    recipient: DamageRecipientMatcherDef::Recipients(
                        EffectRecipientDef::AttachedPermanent,
                    ),
                    ..DamageEventMatcherDef::ANY
                }),
                EffectDef::sacrifice(EffectRecipientDef::Source),
            ),
        ]),
);

// FIN 75 — Stolen Uniform
// Audit: unsupported — Needs attachment between independently selected objects and a delayed control-loss listener bound to that Equipment for this turn; existing attach operations use the ability source and delayed event matchers cannot retain the selected object.
pub(in crate::card::sets) static STOLEN_UNIFORM: CardRecord = CardRecord::new(
    "Stolen Uniform",
    "0d80c511-2f4d-4f77-8143-7b49b2b19fae",
    "Daniel Correia",
    CardRules::unsupported(),
);

// FIN 76 — Stuck in Summoner's Sanctum
pub(in crate::card::sets) static STUCK_IN_SUMMONER_S_SANCTUM: CardRecord = CardRecord::new(
    "Stuck in Summoner's Sanctum",
    "6678501e-6349-4e37-ab4c-a31a3d408d52",
    "Susumu Kuroi",
    CardRules::new_enchantment(mana_cost!("{2}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::flash(),
            abilities::aura_spell(
                "Enchant artifact or creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                )],
            ),
            abilities::enters_trigger(
                "When this Aura enters, tap enchanted permanent.",
                EffectDef::Tap {
                    object: EffectRecipientDef::AttachedPermanent,
                },
            ),
            AbilityDef::static_ability(
                "Enchanted permanent doesn't untap during its controller's \
                 untap step and its activated abilities can't be activated.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
                        AppliedEffectDef::cannot_activate_abilities(AbilityPredicateDef::Any),
                    ]),
                },
            ),
        ]),
);

// FIN 77 — Summon: Leviathan
// Audit: unsupported — Needs one printed Saga ability to represent several chapter numbers and to remain identifiable to the Saga final-chapter rules; the current Saga authoring and recognition path represents exactly one chapter per ability.
pub(in crate::card::sets) static SUMMON_LEVIATHAN: CardRecord = CardRecord::new(
    "Summon: Leviathan",
    "ea7f26a9-b203-4ee7-88f1-3d9c77a25bcb",
    "OTUMAMI",
    CardRules::unsupported(),
);

// FIN 78 — Summon: Shiva
// Audit: unsupported — Needs one printed Saga ability to represent several chapter numbers and to remain identifiable to the Saga final-chapter rules; the current Saga authoring and recognition path represents exactly one chapter per ability.
pub(in crate::card::sets) static SUMMON_SHIVA: CardRecord = CardRecord::new(
    "Summon: Shiva",
    "a80511f8-7cb1-4974-afde-8a5cebe13ad7",
    "Fajareka Setiawan",
    CardRules::unsupported(),
);

// FIN 79 — Swallowed by Leviathan
pub(in crate::card::sets) static SWALLOWED_BY_LEVIATHAN: CardRecord = CardRecord::new(
    "Swallowed by Leviathan",
    "2270642d-fe2a-4265-aff0-a24a43ebe0a1",
    "Sansyu",
    CardRules::new_instant(mana_cost!("{2}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Choose target spell. Surveil 2, then counter the chosen spell \
         unless its controller pays {1} for each card in your \
         graveyard. (To surveil 2, look at the top two cards of your \
         library, then put any number of them into your graveyard and \
         the rest on top of your library in any order.)",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            abilities::surveil(ValueDef::Constant(2)),
            EffectDef::PayOr(
                PayOrDef::unless(
                    &[CostDef::GenericMana(ValueDef::CountMatchingObjects(
                        &ObjectQueryDef::matching(
                            ObjectPredicateDef::Any,
                            &[ZoneKind::Graveyard],
                            PlayerRelation::You,
                        ),
                    ))],
                    &EffectDef::counter_target(TargetIndex::PRIMARY),
                )
                .with_payer(PlayerSetDef::One(PlayerRefDef::ControllerOf(
                    ObjectRefDef::Target(TargetIndex::PRIMARY),
                ))),
            ),
        ]),
    )]),
);

// FIN 80 — Syncopate (reprint)
const SYNCOPATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ody::SYNCOPATE,
    "83e1cd8d-5b97-427a-be75-2a1947f9c59b",
    "Nijihayashi",
);

// FIN 81 — Thief's Knife
pub(in crate::card::sets) static THIEF_S_KNIFE: CardRecord = CardRecord::new(
    "Thief's Knife",
    "c2dcfd0a-3f52-4616-a09a-fd2db8b6b93e",
    "Domenico Cava",
    CardRules::new_artifact(mana_cost!("{2}{U}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::job_select(),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1, has \"Whenever this creature \
                 deals combat damage to a player, draw a card,\" and is a \
                 Rogue in addition to its other types.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&AbilityDef::triggered(
                            "Whenever this creature deals combat damage to a player, draw \
                             a card.",
                            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                            abilities::draw_cards(ValueDef::Constant(1)),
                        )),
                        AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Rogue"])),
                    ]),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{4}"))], "Equip {4}"),
        ]),
);

// FIN 82 — Travel the Overworld
pub(in crate::card::sets) static TRAVEL_THE_OVERWORLD: CardRecord = CardRecord::new(
    "Travel the Overworld",
    "aa5086e0-e2f2-498f-9035-1b31e1d21e0a",
    "Ben Wootten",
    CardRules::new_sorcery(mana_cost!("{5}{U}{U}")).with_abilities(&[
        AbilityDef::static_ability(
            "Affinity for Towns (This spell costs {1} less to cast for \
             each Town you control.)",
            EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(
                &ObjectQueryDef::matching(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Town")),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
            )),
        )
        .with_source_zones(&[ZoneKind::Hand]),
        AbilityDef::spell(
            "Draw four cards.",
            abilities::draw_cards(ValueDef::Constant(4)),
        ),
    ]),
);

// FIN 83 — Ultros, Obnoxious Octopus
// Audit: unsupported — Needs total mana spent recorded with a spell-cast event and readable by its triggered ability, including additional costs and cost reductions; the current cast-event values expose mana value and colors spent, not the amount paid.
pub(in crate::card::sets) static ULTROS_OBNOXIOUS_OCTOPUS: CardRecord = CardRecord::new(
    "Ultros, Obnoxious Octopus",
    "14379198-9a0a-4853-9d51-fb074a24b1c0",
    "Domenico Cava",
    CardRules::unsupported(),
);

// FIN 84 — Valkyrie Aerial Unit
pub(in crate::card::sets) static VALKYRIE_AERIAL_UNIT: CardRecord = CardRecord::new(
    "Valkyrie Aerial Unit",
    "b40a055f-ea66-44d4-b058-a328a3d10994",
    "hippo",
    CardRules::new_artifact_creature(mana_cost!("{5}{U}{U}"), &["Construct"], 5, 4).with_abilities(
        &[
            AbilityDef::static_ability(
                "Affinity for artifacts (This spell costs {1} less to cast for \
                 each artifact you control.)",
                EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(
                    &ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
            )
            .with_source_zones(&[ZoneKind::Hand]),
            abilities::flying(),
            abilities::enters_trigger(
                "When this creature enters, surveil 2. (Look at the top two \
                 cards of your library, then put any number of them into your \
                 graveyard and the rest on top of your library in any order.)",
                abilities::surveil(ValueDef::Constant(2)),
            ),
        ],
    ),
);

// FIN 85 — The Water Crystal
// Audit: unsupported — Needs a prospective mill-batch replacement adding four cards once per mill instruction; replacing individual draws or reacting after milling gives different events and quantities.
pub(in crate::card::sets) static THE_WATER_CRYSTAL: CardRecord = CardRecord::new(
    "The Water Crystal",
    "e0af8436-797b-4e1f-b21a-d8e93701c3c9",
    "Pablo Mendoza",
    CardRules::unsupported(),
);

// FIN 86 — Y'shtola Rhul
// Audit: unsupported — Needs end-step ordinal history and insertion of an additional end step after the current step; current phase scheduling does not insert individual ending-phase steps.
pub(in crate::card::sets) static Y_SHTOLA_RHUL: CardRecord = CardRecord::new(
    "Y'shtola Rhul",
    "aef218fa-13a4-4653-95d6-6b3ef1b33a92",
    "Immanuela Crovius",
    CardRules::unsupported(),
);

// FIN 87 — Ahriman
pub(in crate::card::sets) static AHRIMAN: CardRecord = CardRecord::new(
    "Ahriman",
    "162a415c-5465-497e-8f4e-c6f09681641d",
    "Kevin Sidharta",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Eye", "Horror"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::deathtouch(),
        AbilityDef::activated(
            "{3}, Sacrifice another creature or artifact: Draw a card.",
            &[
                CostDef::Mana(mana_cost!("{3}")),
                CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ])),
            ],
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

// FIN 88 — Al Bhed Salvagers
pub(in crate::card::sets) static AL_BHED_SALVAGERS: CardRecord = CardRecord::new(
    "Al Bhed Salvagers",
    "58ccdcfc-a669-480f-bded-4273cfaf2045",
    "Masateru Ikeda",
    CardRules::new_creature(
        mana_cost!("{2}{B}"),
        &["Human", "Artificer", "Warrior"],
        2,
        3,
    )
    .with_abilities(&[AbilityDef::triggered_with_targets(
        "Whenever this creature or another creature or artifact you \
         control dies, target opponent loses 1 life and you gain 1 \
         life.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ]),
            Some(ZoneKind::Battlefield),
            Some(ZoneKind::Graveyard),
        ),
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
    )]),
);

// FIN 89 — Ardyn, the Usurper
// Audit: unsupported — Needs a token-copy exception replacing all creature types with Demon; copy exceptions currently add creature types rather than replace the copied type set.
pub(in crate::card::sets) static ARDYN_THE_USURPER: CardRecord = CardRecord::new(
    "Ardyn, the Usurper",
    "4627072e-9c72-4084-8021-690777342548",
    "Russell Dongjun Lu",
    CardRules::unsupported(),
);

// FIN 90 — Black Mage's Rod
pub(in crate::card::sets) static BLACK_MAGE_S_ROD: CardRecord = CardRecord::new(
    "Black Mage's Rod",
    "35e8f140-055f-4fc7-a765-fb030d828214",
    "Nino Is",
    CardRules::new_artifact(mana_cost!("{1}{B}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::job_select(),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+0, has \"Whenever you cast a \
                 noncreature spell, this creature deals 1 damage to each \
                 opponent,\" and is a Wizard in addition to its other types.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::add_ability(&AbilityDef::triggered(
                            "Whenever you cast a noncreature spell, this creature deals 1 \
                             damage to each opponent.",
                            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                                    CardType::Creature,
                                )),
                                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                            ])),
                            EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(1)),
                        )),
                        AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&[
                            "Wizard",
                        ])),
                    ]),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{3}"))], "Equip {3}"),
        ]),
);

// FIN 91 — Cecil, Dark Knight // Cecil, Redeemed Paladin
pub(in crate::card::sets) static CECIL_DARK_KNIGHT: CardRecord = CardRecord::new_dfc(
    "Cecil, Dark Knight // Cecil, Redeemed Paladin",
    "026e7167-d665-43d0-a51e-8df2d68cdb5e",
    "Josu Hernaiz",
    &[
        (
            "Cecil, Dark Knight",
            const {
                CardRules::new_creature(mana_cost!("{B}"), &["Human", "Knight"], 2, 3)
                .with_supertype(CardSupertype::Legendary)
                .with_abilities(&const { [
                    abilities::deathtouch(),
                    AbilityDef::triggered(
                        "Darkness — Whenever Cecil deals damage, you lose that much life. Then if your life total is less than or equal to half your starting life total, untap Cecil and transform it.",
                        TriggerEventDef::DamageDealt(DamageEventMatcherDef {
                            kind: DamageKindDef::Any,
                            source: DamageSourceMatcherDef::Object(ObjectRefDef::Source),
                            recipient: DamageRecipientMatcherDef::Any,
                        }),
                        // "You lose that much life. Then if ..." is one clause resolving in order:
                        // the life is lost first, so the very damage that cost it can be what brings
                        // the total low enough to turn the card over.
                        EffectDef::Sequence(&[
                            EffectDef::LoseLife {
                                recipient: EffectRecipientDef::Controller,
                                amount: ValueDef::TriggerEventAmount,
                            },
                            EffectDef::IfCondition {
                                condition: &TriggerConditionDef::ControllerLifeAtMostHalfStartingLife,
                                // The front half's payoff, and the reason the card is played: hitting hard
                                // enough to halve your own life is what turns Cecil over. Untapping is part
                                // of the same clause, so a Cecil that traded its attack for the transform
                                // comes back ready to block.
                                then: &EffectDef::Sequence(&[
                                    EffectDef::Untap {
                                        object: EffectRecipientDef::Source,
                                    },
                                    EffectDef::Transform {
                                        object: EffectRecipientDef::Source,
                                    },
                                ]),
                            },
                        ]),
                    ),
                ] })
            },
        ),
        // The back face has no printed mana cost and is white, where the front is
        // black: transforming changes the colour it defends in.
        (
            "Cecil, Redeemed Paladin",
            const {
                CardRules::new_creature_without_mana_cost(&["Human", "Knight"], 4, 4)
                .printed_colors(&[ManaColor::White])
                .with_supertype(CardSupertype::Legendary)
                .with_abilities(&const { [
                    abilities::lifelink(),
                    AbilityDef::triggered(
                        "Protect — Whenever Cecil attacks, other attacking creatures gain indestructible until end of turn.",
                        TriggerEventDef::attacks(ObjectPredicateDef::Source),
                        EffectDef::Apply {
                            // "Other attacking creatures" excludes Cecil and takes in the opponent's
                            // too, on the rare turn both sides are attacking at once.
                            recipient: EffectRecipientDef::matching_objects(
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::Attacking,
                                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                ]),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::Any,
                            ),
                            effect: AppliedEffectDef::add_ability(&const {
                                abilities::indestructible()
                            }),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    ),
                ] })
            },
        ),
    ],
);

// FIN 92 — Circle of Power
pub(in crate::card::sets) static CIRCLE_OF_POWER: CardRecord = CardRecord::new(
    "Circle of Power",
    "c969bd0f-f174-4e2f-b95c-c9ccd5d0b8ba",
    "Josephine Chang",
    CardRules::new_sorcery(mana_cost!("{3}{B}")).with_abilities(&[AbilityDef::spell(
        "You draw two cards and you lose 2 life. Create a 0/1 black \
         Wizard creature token with \"Whenever you cast a noncreature \
         spell, this token deals 1 damage to each opponent.\"\nWizards \
         you control get +1/+0 and gain lifelink until end of turn.",
        EffectDef::Sequence(&[
            abilities::draw_cards(ValueDef::Constant(2)),
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(WIZARD_TOKEN))),
            EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Wizard")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::add_ability(&abilities::lifelink()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )]),
);

// FIN 93 — Cornered by Black Mages
pub(in crate::card::sets) static CORNERED_BY_BLACK_MAGES: CardRecord = CardRecord::new(
    "Cornered by Black Mages",
    "688fcf8a-0a44-416a-8086-83acf9a6fe69",
    "Smirtouille",
    CardRules::new_sorcery(mana_cost!("{1}{B}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target opponent sacrifices a creature of their \
             choice.\nCreate a 0/1 black Wizard creature token with \
             \"Whenever you cast a noncreature spell, this token deals 1 \
             damage to each opponent.\"",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::Sequence(&[
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
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(WIZARD_TOKEN))),
            ]),
        ),
    ]),
);

// FIN 94 — Dark Confidant (reprint)
const DARK_CONFIDANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rav::DARK_CONFIDANT,
    "2520ab23-a068-4462-b261-2754409b4108",
    "Immanuela Crovius",
);

// FIN 95 — Dark Knight's Greatsword
pub(in crate::card::sets) static DARK_KNIGHT_S_GREATSWORD: CardRecord = CardRecord::new(
    "Dark Knight's Greatsword",
    "b50dcc7c-260f-4d8c-9a9e-9244ec23a91e",
    "Narendra Bintara Adi",
    CardRules::new_artifact(mana_cost!("{2}{B}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::job_select(),
            AbilityDef::static_ability(
                "Equipped creature gets +3/+0 and is a Knight in addition to \
                 its other types.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(3),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&[
                            "Knight",
                        ])),
                    ]),
                },
            ),
            abilities::equip(
                &[CostDef::PayLife(3)],
                "Chaosbringer — Equip—Pay 3 life. Activate only once each turn.",
            )
            .once_each_turn(),
        ]),
);

// FIN 96 — The Darkness Crystal
// Audit: unsupported — Needs a death replacement that both exiles and links each replaced object to this source and gains life in that same replacement; ordinary linked-exile effects do not expose the prospective dying object as a linkable result.
pub(in crate::card::sets) static THE_DARKNESS_CRYSTAL: CardRecord = CardRecord::new(
    "The Darkness Crystal",
    "0f93b6ac-54ce-45d0-8549-19307406e6e5",
    "Pablo Mendoza",
    CardRules::unsupported(),
);

// FIN 97 — Demon Wall
pub(in crate::card::sets) static DEMON_WALL: CardRecord = CardRecord::new(
    "Demon Wall",
    "13abd96c-d1af-43d0-b3a4-ac3db20e3b51",
    "Anton Solovianchyk",
    CardRules::new_artifact_creature(mana_cost!("{1}{B}"), &["Demon", "Wall"], 3, 3)
        .with_abilities(&[
            abilities::defender(),
            abilities::menace(),
            AbilityDef::static_ability(
                "As long as this creature has a counter on it, it can attack \
                 as though it didn't have defender.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceMatches {
                        object: ObjectPredicateDef::HasAnyCounter,
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::MayAttackDespiteDefender),
                    },
                },
            ),
            AbilityDef::activated(
                "{5}{B}: Put two +1/+1 counters on this creature.",
                &[CostDef::Mana(mana_cost!("{5}{B}"))],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(2),
                },
            ),
        ]),
);

// FIN 98 — Evil Reawakened
pub(in crate::card::sets) static EVIL_REAWAKENED: CardRecord = CardRecord::new(
    "Evil Reawakened",
    "eb98cbc3-749c-44f4-974c-00be1286d69e",
    "Nino Is",
    CardRules::new_sorcery(mana_cost!("{4}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Return target creature card from your graveyard to the \
         battlefield with two additional +1/+1 counters on it.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            },
        )],
        EffectDef::WithBattlefieldArrival {
            effect: &EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Battlefield,
                ZonePlacement::Top,
            ),
            arrival: BattlefieldArrivalDef {
                modifications: &[BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 2,
                }],
                ..BattlefieldArrivalDef::DEFAULT
            },
        },
    )]),
);

// FIN 99 — Fang, Fearless l'Cie
// Audit: unsupported — Needs meld topology combining two physical cards into one permanent, together with splitting and tracking both components when that permanent changes zones; transforming double-faced cards do not represent a melded permanent.
pub(in crate::card::sets) static FANG_FEARLESS_L_CIE: CardRecord = CardRecord::new(
    "Fang, Fearless l'Cie",
    "f73a5cbb-905f-4b99-8d23-9d1b6bd47ee8",
    "Simon Dominic",
    CardRules::unsupported(),
);

// FIN 99b — Ragnarok, Divine Deliverance
// Audit: unsupported — Needs meld topology combining two physical cards into one permanent, together with splitting and tracking both components when that permanent changes zones; transforming double-faced cards do not represent a melded permanent.
pub(in crate::card::sets) static RAGNAROK_DIVINE_DELIVERANCE: CardRecord = CardRecord::new(
    "Ragnarok, Divine Deliverance",
    "01c5bafe-c995-4cef-90fb-7ccb95858511",
    "Simon Dominic",
    CardRules::unsupported(),
);

// FIN 100 — Fight On!
pub(in crate::card::sets) static FIGHT_ON: CardRecord = CardRecord::new(
    "Fight On!",
    "e1b0739a-dcc4-4034-96d7-69e551b2f36b",
    "Hokuyuu",
    CardRules::new_instant(mana_cost!("{2}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Return up to two target creature cards from your graveyard to \
         your hand.",
        &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            },
            2,
        )],
        EffectDef::move_to_zone(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ZoneKind::Hand,
            ZonePlacement::Top,
        ),
    )]),
);

// FIN 101 — The Final Days
pub(in crate::card::sets) static THE_FINAL_DAYS: CardRecord = CardRecord::new(
    "The Final Days",
    "bbf01770-6d0f-4015-b4b4-a74a53cb767a",
    "Fang Xinyu",
    CardRules::new_sorcery(mana_cost!("{2}{B}{B}")).with_abilities(&[
        AbilityDef::spell(
            "Create two tapped 2/2 black Horror creature tokens. If this \
             spell was cast from a graveyard, instead create X of those \
             tokens, where X is the number of creature cards in your \
             graveyard.",
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::SourceCastFrom(ZoneKind::Graveyard),
                then: &EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(HORROR_TOKEN))
                        .with_count(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Graveyard],
                            PlayerRelation::You,
                        )))
                        .entering_tapped(),
                ),
                otherwise: &EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(HORROR_TOKEN))
                        .with_count(ValueDef::Constant(2))
                        .entering_tapped(),
                ),
            },
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{4}{B}{B}"))]),
    ]),
);

// FIN 102 — Gaius van Baelsar
pub(in crate::card::sets) static GAIUS_VAN_BAELSAR: CardRecord = CardRecord::new(
    "Gaius van Baelsar",
    "a4ee8ba5-6a79-4652-b2a4-a3dae804bc28",
    "Nino Is",
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Human", "Soldier"], 3, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::modal_triggered(
            "When this creature enters, choose one —",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[
                AbilityDef::spell(
                    "Each player sacrifices a creature token of their choice.",
                    EffectDef::ChooseForEachPlayer(ChooseForEachPlayerDef {
                        player: EffectRecipientDef::EachPlayer,
                        zone: ZoneKind::Battlefield,
                        candidates: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Token,
                        ]),
                        selection: PerPlayerSelectionDef::Count(ValueDef::Constant(1)),
                        chosen: crate::Binding!("sacrifice_0"),
                        unchosen: crate::Binding!("unchosen_sacrifice_0"),
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::sacrifice(EffectRecipientDef::objects(
                            ObjectSetDef::Binding(crate::Binding!("sacrifice_0")),
                        )),
                    }),
                ),
                AbilityDef::spell(
                    "Each player sacrifices a nontoken creature of their choice.",
                    EffectDef::ChooseForEachPlayer(ChooseForEachPlayerDef {
                        player: EffectRecipientDef::EachPlayer,
                        zone: ZoneKind::Battlefield,
                        candidates: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                        ]),
                        selection: PerPlayerSelectionDef::Count(ValueDef::Constant(1)),
                        chosen: crate::Binding!("sacrifice_1"),
                        unchosen: crate::Binding!("unchosen_sacrifice_1"),
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::sacrifice(EffectRecipientDef::objects(
                            ObjectSetDef::Binding(crate::Binding!("sacrifice_1")),
                        )),
                    }),
                ),
                AbilityDef::spell(
                    "Each player sacrifices an enchantment of their choice.",
                    EffectDef::ChooseForEachPlayer(ChooseForEachPlayerDef {
                        player: EffectRecipientDef::EachPlayer,
                        zone: ZoneKind::Battlefield,
                        candidates: ObjectPredicateDef::HasType(CardType::Enchantment),
                        selection: PerPlayerSelectionDef::Count(ValueDef::Constant(1)),
                        chosen: crate::Binding!("sacrifice_2"),
                        unchosen: crate::Binding!("unchosen_sacrifice_2"),
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::sacrifice(EffectRecipientDef::objects(
                            ObjectSetDef::Binding(crate::Binding!("sacrifice_2")),
                        )),
                    }),
                ),
            ],
        )]),
);

// FIN 103 — Hecteyes
pub(in crate::card::sets) static HECTEYES: CardRecord = CardRecord::new(
    "Hecteyes",
    "8680d052-c07b-4d9b-bda9-b5f69f44f424",
    "SHOSUKE",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Ooze", "Horror"], 1, 1).with_abilities(&[
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

// FIN 104 — Jecht, Reluctant Guardian // Braska's Final Aeon
// Audit: unsupported — Needs one printed Saga ability to represent several chapter numbers and to remain identifiable to the Saga final-chapter rules; the current Saga authoring and recognition path represents exactly one chapter per ability.
pub(in crate::card::sets) static JECHT_RELUCTANT_GUARDIAN: CardRecord = CardRecord::new(
    "Jecht, Reluctant Guardian // Braska's Final Aeon",
    "4ec91fe8-b3da-47fa-b45e-94b62a260aba",
    "Michael MacRae",
    CardRules::unsupported(),
);

// FIN 105 — Kain, Traitorous Dragoon
// Audit: unsupported — Needs a gain-control success result to gate its draw, Treasure, and life-loss continuation; gain control currently returns no result for the following effect to inspect.
pub(in crate::card::sets) static KAIN_TRAITOROUS_DRAGOON: CardRecord = CardRecord::new(
    "Kain, Traitorous Dragoon",
    "f8c86be0-e1b3-4a78-9254-238dd936914b",
    "Russell Dongjun Lu",
    CardRules::unsupported(),
);

// FIN 106 — Malboro
pub(in crate::card::sets) static MALBORO: CardRecord = CardRecord::new(
    "Malboro",
    "e46d8048-03ce-4e07-ba24-f41ba6140a4e",
    "Dan Watson",
    CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Plant", "Horror"], 4, 4).with_abilities(&[
        abilities::enters_trigger(
            "Bad Breath — When this creature enters, each opponent \
             discards a card, loses 2 life, and exiles the top three cards \
             of their library.",
            EffectDef::Sequence(&[
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(2),
                },
                EffectDef::BindObjects(BindObjectsDef {
                    source: ObjectCollectionSourceDef::TopCards {
                        player: PlayerRefDef::Opponent,
                        count: ValueDef::Constant(3),
                    },
                    binding: crate::Binding!("exiled"),
                    then: &EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "exiled"
                        ))),
                        ZoneKind::Exile,
                        ZonePlacement::Top,
                    ),
                }),
            ]),
        ),
        abilities::typecycling!(
            "Swampcycling {2}",
            &[CostDef::Mana(mana_cost!("{2}"))],
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Swamp"))
        ),
    ]),
);

// FIN 107 — Namazu Trader
pub(in crate::card::sets) static NAMAZU_TRADER: CardRecord = CardRecord::new(
    "Namazu Trader",
    "f9d25b34-990d-416c-aef7-1b5a73f19dd4",
    "Andrea Tentori Montalto",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Fish", "Citizen"], 3, 4).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, you lose 1 life and create a \
             Treasure token.",
            EffectDef::Sequence(&[
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            ]),
        ),
        AbilityDef::triggered(
            "Whenever this creature attacks, you may sacrifice another \
             creature or artifact. If you do, surveil 2. (Look at the top \
             two cards of your library, then put any number of them into \
             your graveyard and the rest on top of your library in any \
             order.)",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]))],
                &abilities::surveil(ValueDef::Constant(2)),
            )),
        ),
    ]),
);

// FIN 108 — Ninja's Blades
pub(in crate::card::sets) static NINJA_S_BLADES: CardRecord = CardRecord::new(
    "Ninja's Blades",
    "a6b5af82-3646-44f9-ac12-1d7fa698f037",
    "Immanuela Crovius",
    CardRules::new_artifact(mana_cost!("{2}{B}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::job_select(),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1, is a Ninja in addition to its \
                 other types, and has \"Whenever this creature deals combat \
                 damage to a player, draw a card, then discard a card. That \
                 player loses life equal to the discarded card's mana value.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Ninja"])),
                        AppliedEffectDef::add_ability(&AbilityDef::triggered(
                            "Whenever this creature deals combat damage to a player, draw \
                             a card, then discard a card. That player loses life equal to \
                             the discarded card's mana value.",
                            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                            EffectDef::Sequence(&[
                                abilities::draw_cards(ValueDef::Constant(1)),
                                EffectDef::Discard {
                                    recipient: EffectRecipientDef::Controller,
                                    amount: ValueDef::Constant(1),
                                    selection: DiscardSelectionDef::RecipientChooses,
                                    then: Some(DiscardFollowUpDef {
                                        counted: ObjectPredicateDef::Any,
                                        bound: Some(crate::Binding!("discarded")),
                                        effect: &EffectDef::LoseLife {
                                            recipient: EffectRecipientDef::EventPlayer,
                                            amount: ValueDef::AggregateObjectValues(
                                                &ObjectValueAggregateDef {
                                                    objects: ObjectSetDef::Binding(
                                                        crate::Binding!("discarded"),
                                                    ),
                                                    select: ObjectValueDef::ManaValue,
                                                    operation: AggregateOperationDef::Sum,
                                                },
                                            ),
                                        },
                                    }),
                                },
                            ]),
                        )),
                    ]),
                },
            ),
            abilities::equip(
                &[CostDef::Mana(mana_cost!("{2}"))],
                "Mutsunokami — Equip {2}",
            ),
        ]),
);

// FIN 109 — Overkill
pub(in crate::card::sets) static OVERKILL: CardRecord = CardRecord::new(
    "Overkill",
    "ae075e71-d33d-4d6c-b4a5-0b47dd6fd196",
    "Bachzim",
    CardRules::new_instant(mana_cost!("{2}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets -0/-9999 until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(0),
                ValueDef::Constant(-9999),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// FIN 110 — Phantom Train
pub(in crate::card::sets) static PHANTOM_TRAIN: CardRecord = CardRecord::new(
    "Phantom Train",
    "7a50d2ac-101d-41e1-b400-18fa7d2d7125",
    "Gal Or",
    CardRules::new_vehicle(mana_cost!("{3}{B}"), 4, 4).with_abilities(&[
        abilities::trample(),
        AbilityDef::activated(
            "Sacrifice another artifact or creature: Put a +1/+1 counter \
             on this Vehicle. It becomes a Spirit artifact creature in \
             addition to its other types until end of turn.",
            &[CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
            ]))],
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(
                            CardTypeSet::single(CardType::Artifact).with(CardType::Creature),
                        ),
                        AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&[
                            "Spirit",
                        ])),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ]),
);

// FIN 111 — Poison the Waters
pub(in crate::card::sets) static POISON_THE_WATERS: CardRecord = CardRecord::new(
    "Poison the Waters",
    "ff2bafe7-4d0f-464d-b7ba-55a54366fc68",
    "Arif Wijaya",
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "All creatures get -1/-1 until end of turn.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        ),
                    )),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-1),
                        ValueDef::Constant(-1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell_with_targets(
                "Target player reveals their hand. You choose an artifact or \
                 creature card from it. That player discards that card.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::Sequence(&abilities::reveal_hand_and_discard_chosen_card(
                    PlayerRefDef::Target(TargetIndex::PRIMARY),
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                )),
            ),
        ],
    )]),
);

// FIN 112 — Qutrub Forayer
// Audit: unsupported — Needs an optional two-card graveyard target group constrained to one shared graveyard owner; current target slots cannot compare the owners of two selected cards.
pub(in crate::card::sets) static QUTRUB_FORAYER: CardRecord = CardRecord::new(
    "Qutrub Forayer",
    "7475ecf6-23f5-45af-9ef0-ac7923bbc9cb",
    "Lordigan",
    CardRules::unsupported(),
);

// FIN 113 — Reno and Rude
// Audit: unsupported — Needs a permission granting play and unrestricted mana spending to an already-exiled bound card after the sacrifice succeeds; the current combined exile-permission effect attempts a new exile move and cannot grant this permission to that existing exile object.
pub(in crate::card::sets) static RENO_AND_RUDE: CardRecord = CardRecord::new(
    "Reno and Rude",
    "b5eb0064-c7c4-4e3e-add2-b86269de3fb9",
    "Maji",
    CardRules::unsupported(),
);

// FIN 114 — Resentful Revelation
pub(in crate::card::sets) static RESENTFUL_REVELATION: CardRecord = CardRecord::new(
    "Resentful Revelation",
    "945006ea-c6a1-4ee5-abb2-387c2b6d3123",
    "Justyna Dura",
    // The two cards it buries are the point as often as the one it keeps,
    // and the flashback is what the graveyard deck is really paying for.
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_abilities(&[
        AbilityDef::spell(
            "Look at the top three cards of your library. Put one of them into your hand and the \
             rest into your graveyard.",
            // Exactly one, not up to one: the card is mandatory, and any of
            // the three qualifies.
            abilities::look_at_top_cards_choose_to_hand_rest_graveyard(
                ValueDef::Constant(3),
                ObjectPredicateDef::Any,
                1,
                1,
            ),
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{6}{B}"))]),
    ]),
);

// FIN 115 — Sephiroth, Fabled SOLDIER // Sephiroth, One-Winged Angel
// Audit: unsupported — Needs an as-transforms replacement that creates the emblem during the transformation, without an extra counterable trigger; current entry replacements and after-transform events do not model that timing.
pub(in crate::card::sets) static SEPHIROTH_FABLED_SOLDIER: CardRecord = CardRecord::new(
    "Sephiroth, Fabled SOLDIER // Sephiroth, One-Winged Angel",
    "85eaf5e7-77dc-4842-a70c-ce4ac7f724df",
    "Wisnu Tan",
    CardRules::unsupported(),
);

// FIN 116 — Sephiroth's Intervention
pub(in crate::card::sets) static SEPHIROTH_S_INTERVENTION: CardRecord = CardRecord::new(
    "Sephiroth's Intervention",
    "cf7df82f-937a-443f-813f-2bcc6944c5c0",
    "Joshua Raphael",
    CardRules::new_instant(mana_cost!("{3}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target creature. You gain 2 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
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
    )]),
);

// FIN 117 — Shambling Cie'th
pub(in crate::card::sets) static SHAMBLING_CIE_TH: CardRecord = CardRecord::new(
    "Shambling Cie'th",
    "f02ce338-4fe2-44b0-a896-3ed7e6c874a3",
    "Nottsuo",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Mutant", "Horror"], 3, 3).with_abilities(&[
        abilities::enters_tapped(CardType::Creature),
        AbilityDef::triggered(
            "Whenever you cast a noncreature spell, you may pay {B}. If \
             you do, return this card from your graveyard to your hand.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::Mana(mana_cost!("{B}"))],
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

// FIN 118 — Shinra Reinforcements
pub(in crate::card::sets) static SHINRA_REINFORCEMENTS: CardRecord = CardRecord::new(
    "Shinra Reinforcements",
    "7367b257-1a8c-4593-a307-7116e36e0342",
    "Airi Yoshihisa",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Human", "Soldier"], 2, 3).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, mill three cards and you gain 3 \
             life. (To mill three cards, put the top three cards of your \
             library into your graveyard.)",
            EffectDef::Sequence(&[
                EffectDef::Mill {
                    player: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
            ]),
        ),
    ]),
);

// FIN 119 — Sidequest: Hunt the Mark // Yiazmat, Ultimate Mark
// Audit: unsupported — Needs a this-turn creature-death predicate retaining the dying permanent's former controller; the shared death tally and morbid condition are global.
pub(in crate::card::sets) static SIDEQUEST_HUNT_THE_MARK: CardRecord = CardRecord::new(
    "Sidequest: Hunt the Mark // Yiazmat, Ultimate Mark",
    "c3eb2ae5-10de-4c3d-91c8-8734befc80b2",
    "Nino Is & Joshua Raphael",
    CardRules::unsupported(),
);

// FIN 120 — Summon: Anima
// Audit: unsupported — Needs one printed Saga ability to represent several chapter numbers and to remain identifiable to the Saga final-chapter rules; the current Saga authoring and recognition path represents exactly one chapter per ability.
pub(in crate::card::sets) static SUMMON_ANIMA: CardRecord = CardRecord::new(
    "Summon: Anima",
    "aa4f6703-21f8-4c29-ad5a-5afb54188ade",
    "Esuthio",
    CardRules::unsupported(),
);

// FIN 121 — Summon: Primal Odin
pub(in crate::card::sets) static SUMMON_PRIMAL_ODIN: CardRecord = CardRecord::new(
    "Summon: Primal Odin",
    "8b1b5f06-e34d-44a3-976e-5157c4b7a0f4",
    "Nino Is",
    CardRules::new_enchantment_creature(mana_cost!("{4}{B}{B}"), &["Saga", "Knight"], 5, 3)
        .with_abilities(&[
            abilities::saga_chapter_with_targets(
                1,
                "I — Gungnir — Destroy target creature an opponent controls.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
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
                2,
                "II — Zantetsuken — This creature gains \"Whenever this \
                 creature deals combat damage to a player, that player loses \
                 the game.\"",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&AbilityDef::triggered(
                        "Whenever this creature deals combat damage to a player, that \
                         player loses the game.",
                        TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                        EffectDef::LoseTheGame {
                            player: EffectRecipientDef::EventPlayer,
                        },
                    )),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
            ),
            abilities::saga_chapter(
                3,
                "III — Hall of Sorrow — Draw two cards. Each player loses 2 life.",
                EffectDef::Sequence(&[
                    abilities::draw_cards(ValueDef::Constant(2)),
                    EffectDef::LoseLife {
                        recipient: EffectRecipientDef::EachPlayer,
                        amount: ValueDef::Constant(2),
                    },
                ]),
            ),
        ]),
);

// FIN 122 — Tonberry
pub(in crate::card::sets) static TONBERRY: CardRecord = CardRecord::new(
    "Tonberry",
    "1a9b8723-4383-4c14-b24d-52863af8703d",
    "Leonardo Santanna",
    CardRules::new_creature(mana_cost!("{B}"), &["Salamander", "Horror"], 2, 1).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters tapped with a stun counter on it. (If it \
             would become untapped, remove a stun counter from it \
             instead.)",
            ReplacementEffectDef::Sequence(&[
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::Tapped,
                ),
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::AddCounters {
                        kind: CounterKind::Stun,
                        amount: 1,
                    },
                ),
            ]),
        ),
        AbilityDef::static_ability(
            "Chef's Knife — During your turn, this creature has first \
             strike and deathtouch.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_ability(&abilities::first_strike()),
                        AppliedEffectDef::add_ability(&abilities::deathtouch()),
                    ]),
                },
            },
        ),
    ]),
);

// FIN 123 — Undercity Dire Rat
pub(in crate::card::sets) static UNDERCITY_DIRE_RAT: CardRecord = CardRecord::new(
    "Undercity Dire Rat",
    "274788f4-fbf3-4a15-bdc0-f513a2fde30d",
    "Leonardo Santanna",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Rat"], 2, 2).with_abilities(&[
        abilities::dies_trigger(
            "Rat Tail — When this creature dies, create a Treasure token. \
             (It's an artifact with \"{T}, Sacrifice this token: Add one \
             mana of any color.\")",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ),
    ]),
);

// FIN 124 — Vayne's Treachery
pub(in crate::card::sets) static VAYNE_S_TREACHERY: CardRecord = CardRecord::new(
    "Vayne's Treachery",
    "6de6d23b-7d42-41c1-be1c-010fe43ee586",
    "Touge369",
    CardRules::new_instant(mana_cost!("{1}{B}")).with_abilities(&[
        abilities::kicker(&[CostDef::sacrifice_permanent(ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Artifact),
            ObjectPredicateDef::HasType(CardType::Creature),
        ]))]),
        AbilityDef::spell_with_targets(
            "Target creature gets -2/-2 until end of turn. If this spell \
             was kicked, that creature gets -6/-6 until end of turn \
             instead.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::SourcePaidAdditionalCost(
                    AdditionalCostIndex::PRIMARY,
                ),
                then: &EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-6),
                        ValueDef::Constant(-6),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                otherwise: &EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-2),
                        ValueDef::Constant(-2),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            },
        ),
    ]),
);

// FIN 125 — Vincent Valentine // Galian Beast
pub(in crate::card::sets) static VINCENT_VALENTINE: CardRecord = CardRecord::new_dfc(
    "Vincent Valentine // Galian Beast",
    "028ef608-acfe-4e9d-90db-eca4411ba78a",
    "Norikatsu Miyoshi",
    &[
        (
            "Vincent Valentine",
            CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Assassin"], 2, 2)
                .with_supertype(CardSupertype::Legendary)
                .with_abilities(&[
                    AbilityDef::triggered(
                        "Whenever a creature an opponent controls dies, put a number \
                         of +1/+1 counters on Vincent Valentine equal to that \
                         creature's power.",
                        TriggerEventDef::zone_changed(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                            ]),
                            Some(ZoneKind::Battlefield),
                            Some(ZoneKind::Graveyard),
                        ),
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::Source,
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::TriggeringObjectPower,
                        },
                    ),
                    AbilityDef::triggered(
                        "Whenever Vincent Valentine attacks, you may transform it.",
                        TriggerEventDef::attacks(ObjectPredicateDef::Source),
                        EffectDef::May {
                            player: EffectRecipientDef::Controller,
                            effect: &EffectDef::Transform {
                                object: EffectRecipientDef::Source,
                            },
                        },
                    ),
                ]),
        ),
        (
            "Galian Beast",
            CardRules::new_creature_without_mana_cost(&["Werewolf", "Beast"], 3, 2)
                .with_supertype(CardSupertype::Legendary)
                .printed_colors(&[ManaColor::Black])
                .with_abilities(&[
                    abilities::trample(),
                    abilities::lifelink(),
                    abilities::dies_trigger(
                        "When Galian Beast dies, return it to the battlefield tapped \
                         (front face up).",
                        EffectDef::WithBattlefieldArrival {
                            effect: &EffectDef::move_to_zone(
                                EffectRecipientDef::TriggeringZoneChangeResult,
                                ZoneKind::Battlefield,
                                ZonePlacement::Top,
                            ),
                            arrival: BattlefieldArrivalDef {
                                modifications: &[BattlefieldEntryModificationDef::Tapped],
                                ..BattlefieldArrivalDef::DEFAULT
                            },
                        },
                    ),
                ]),
        ),
    ],
);

// FIN 126 — Vincent's Limit Break
pub(in crate::card::sets) static VINCENT_S_LIMIT_BREAK: CardRecord = CardRecord::new(
    "Vincent's Limit Break",
    "0502c426-5271-4989-8598-5bc159afe79c",
    "Ryuichi Sakuma",
    CardRules::new_instant(mana_cost!("{1}{B}")).with_abilities(&[tiered(
        "Tiered (Choose one additional cost.)\nUntil end of turn, \
         target creature you control gains \"When this creature dies, \
         return it to the battlefield tapped under its owner's \
         control\" and has the chosen base power and toughness.",
        &[
            (
                &[CostDef::Mana(mana_cost!("{0}"))],
                AbilityDef::spell_with_targets(
                    "Galian Beast — 3/2.",
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
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::add_ability(&abilities::dies_trigger(
                                "When this creature dies, return it to the battlefield tapped \
                                 under its owner's control.",
                                EffectDef::WithBattlefieldArrival {
                                    effect: &EffectDef::move_to_zone(
                                        EffectRecipientDef::TriggeringZoneChangeResult,
                                        ZoneKind::Battlefield,
                                        ZonePlacement::Top,
                                    ),
                                    arrival: BattlefieldArrivalDef {
                                        modifications: &[BattlefieldEntryModificationDef::Tapped],
                                        ..BattlefieldArrivalDef::DEFAULT
                                    },
                                },
                            )),
                            AppliedEffectDef::set_base_power_toughness(
                                ValueDef::Constant(3),
                                ValueDef::Constant(2),
                            ),
                        ]),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ),
            ),
            (
                &[CostDef::Mana(mana_cost!("{1}"))],
                AbilityDef::spell_with_targets(
                    "Death Gigas — 5/2.",
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
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::add_ability(&abilities::dies_trigger(
                                "When this creature dies, return it to the battlefield tapped \
                                 under its owner's control.",
                                EffectDef::WithBattlefieldArrival {
                                    effect: &EffectDef::move_to_zone(
                                        EffectRecipientDef::TriggeringZoneChangeResult,
                                        ZoneKind::Battlefield,
                                        ZonePlacement::Top,
                                    ),
                                    arrival: BattlefieldArrivalDef {
                                        modifications: &[BattlefieldEntryModificationDef::Tapped],
                                        ..BattlefieldArrivalDef::DEFAULT
                                    },
                                },
                            )),
                            AppliedEffectDef::set_base_power_toughness(
                                ValueDef::Constant(5),
                                ValueDef::Constant(2),
                            ),
                        ]),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ),
            ),
            (
                &[CostDef::Mana(mana_cost!("{3}"))],
                AbilityDef::spell_with_targets(
                    "Hellmasker — 7/2.",
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
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::add_ability(&abilities::dies_trigger(
                                "When this creature dies, return it to the battlefield tapped \
                                 under its owner's control.",
                                EffectDef::WithBattlefieldArrival {
                                    effect: &EffectDef::move_to_zone(
                                        EffectRecipientDef::TriggeringZoneChangeResult,
                                        ZoneKind::Battlefield,
                                        ZonePlacement::Top,
                                    ),
                                    arrival: BattlefieldArrivalDef {
                                        modifications: &[BattlefieldEntryModificationDef::Tapped],
                                        ..BattlefieldArrivalDef::DEFAULT
                                    },
                                },
                            )),
                            AppliedEffectDef::set_base_power_toughness(
                                ValueDef::Constant(7),
                                ValueDef::Constant(2),
                            ),
                        ]),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ),
            ),
        ],
    )]),
);

// FIN 127 — Zenos yae Galvus // Shinryu, Transcendent Rival
// Audit: unsupported — Needs a persistent chosen-creature leaves listener, an as-transforms opponent choice, and a player-loses-the-game event; current object bindings, replacement timing, and trigger events do not cover those clauses.
pub(in crate::card::sets) static ZENOS_YAE_GALVUS: CardRecord = CardRecord::new(
    "Zenos yae Galvus // Shinryu, Transcendent Rival",
    "b65ffce4-bb58-418a-9bad-81533a5f2ba2",
    "Alexander Mokhov",
    CardRules::unsupported(),
);

// FIN 128 — Zodiark, Umbral God
// Audit: unsupported — Needs simultaneous per-player sacrifice choices with each count computed from that player's own candidate set; current ChooseForEachPlayer evaluates its count in one shared resolution context, not once relative to each choosing player.
pub(in crate::card::sets) static ZODIARK_UMBRAL_GOD: CardRecord = CardRecord::new(
    "Zodiark, Umbral God",
    "9ba292d5-5139-42ea-950d-0a638445277f",
    "AKAGI",
    CardRules::unsupported(),
);

// FIN 129 — Barret Wallace
// Audit: unsupported — Needs a count of creatures with an Equipment attached; current attachment predicates select attached objects or Aura-enchanted hosts, not equipped hosts.
pub(in crate::card::sets) static BARRET_WALLACE: CardRecord = CardRecord::new(
    "Barret Wallace",
    "1a504dff-5857-4a61-ab99-616d5df7cf5a",
    "Patrik Hell",
    CardRules::unsupported(),
);

// FIN 130 — Blazing Bomb
// Audit: unsupported — Needs total mana spent recorded with a spell-cast event and readable by its triggered ability, including additional costs and cost reductions; the current cast-event values expose mana value and colors spent, not the amount paid.
pub(in crate::card::sets) static BLAZING_BOMB: CardRecord = CardRecord::new(
    "Blazing Bomb",
    "70f47277-ca47-428a-808f-0fb32e820a71",
    "Andrea Radeck",
    CardRules::unsupported(),
);

// FIN 131 — Call the Mountain Chocobo
pub(in crate::card::sets) static CALL_THE_MOUNTAIN_CHOCOBO: CardRecord = CardRecord::new(
    "Call the Mountain Chocobo",
    "b2e1986c-2852-4843-bdc4-eddb727ba3d4",
    "Randy Gallegos",
    CardRules::new_sorcery(mana_cost!("{3}{R}")).with_abilities(&[
        AbilityDef::spell(
            "Search your library for a Mountain card, reveal it, put it \
             into your hand, then shuffle. Create a 2/2 green Bird \
             creature token with \"Whenever a land you control enters, \
             this token gets +1/+0 until end of turn.\"",
            EffectDef::Sequence(&[
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mountain")),
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
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(BIRD_TOKEN))),
            ]),
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{5}{R}"))]),
    ]),
);

// FIN 132 — Choco-Comet
pub(in crate::card::sets) static CHOCO_COMET: CardRecord = CardRecord::new(
    "Choco-Comet",
    "8d8d3903-a7f4-4cd8-8d29-b4a173e4fbb2",
    "Fiona Hsieh",
    CardRules::new_sorcery(mana_cost!("{X}{R}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Choco-Comet deals X damage to any target.\nCreate a 2/2 green \
             Bird creature token with \"Whenever a land you control \
             enters, this token gets +1/+0 until end of turn.\"",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::Sequence(&[
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::ChosenX,
                ),
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(BIRD_TOKEN))),
            ]),
        ),
    ]),
);

// FIN 133 — Clive, Ifrit's Dominant // Ifrit, Warden of Inferno
// Audit: unsupported — Needs one printed Saga ability to represent several chapter numbers and to remain identifiable to the Saga final-chapter rules; the current Saga authoring and recognition path represents exactly one chapter per ability.
pub(in crate::card::sets) static CLIVE_IFRIT_S_DOMINANT: CardRecord = CardRecord::new(
    "Clive, Ifrit's Dominant // Ifrit, Warden of Inferno",
    "9a069e96-2786-493d-aca8-f70611435dbe",
    "Nino Is",
    CardRules::unsupported(),
);

// FIN 134 — Coral Sword
pub(in crate::card::sets) static CORAL_SWORD: CardRecord = CardRecord::new(
    "Coral Sword",
    "13e81e32-7246-46b9-872e-cde77cedd197",
    "Jason Kiantoro",
    CardRules::new_artifact(mana_cost!("{R}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::flash(),
            abilities::enters_trigger_with_targets(
                "When this Equipment enters, attach it to target creature you \
                 control. That creature gains first strike until end of turn.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::Sequence(&[
                    EffectDef::Attach {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            ),
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
            abilities::equip(&[CostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// FIN 135 — The Fire Crystal
pub(in crate::card::sets) static THE_FIRE_CRYSTAL: CardRecord = CardRecord::new(
    "The Fire Crystal",
    "ea430b17-2014-4b8e-b53f-43bcfc06f7cd",
    "Pablo Mendoza",
    CardRules::new_artifact(mana_cost!("{2}{R}{R}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Red spells you cast cost {1} less to cast.",
                EffectDef::ModifyCost(CostModificationDef::reduce_spell(
                    ObjectPredicateDef::Color(ManaColor::Red),
                    PlayerRelation::You,
                    ValueDef::Constant(1),
                )),
            ),
            AbilityDef::static_ability(
                "Creatures you control have haste.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::add_ability(&abilities::haste()),
                },
            ),
            AbilityDef::activated_with_targets(
                "{4}{R}{R}, {T}: Create a token that's a copy of target \
                 creature you control. Sacrifice it at the beginning of the \
                 next end step.",
                &[CostDef::Mana(mana_cost!("{4}{R}{R}")), CostDef::TapSource],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Copy(&TokenCopyDef {
                        object: &EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        exceptions: CopyExceptionsDef::NONE,
                    }))
                    .with_created_tokens(CreatedTokensDef {
                        binding: crate::Binding!("copy"),
                        then: &EffectDef::InstallTrigger(InstalledTriggerDef::once(
                            &AbilityDef::triggered(
                                "At the beginning of the next end step, sacrifice that token.",
                                TriggerEventDef::StepBegins {
                                    step: TurnStepDef::End,
                                    player: PlayerRelation::Any,
                                },
                                EffectDef::sacrifice(EffectRecipientDef::objects(
                                    ObjectSetDef::Binding(crate::Binding!("copy")),
                                )),
                            ),
                        )),
                    }),
                ),
            ),
        ]),
);

// FIN 136 — Fire Magic
pub(in crate::card::sets) static FIRE_MAGIC: CardRecord = CardRecord::new(
    "Fire Magic",
    "415ff6a5-61ef-4b37-ae08-e44476300d4a",
    "Toni Infante",
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[tiered(
        "Tiered (Choose one additional cost.)",
        &[
            (
                &[CostDef::Mana(mana_cost!("{0}"))],
                AbilityDef::spell(
                    "Fire — Fire Magic deals 1 damage to each creature.",
                    EffectDef::damage(
                        EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        ))),
                        ValueDef::Constant(1),
                    ),
                ),
            ),
            (
                &[CostDef::Mana(mana_cost!("{2}"))],
                AbilityDef::spell(
                    "Fira — Fire Magic deals 2 damage to each creature.",
                    EffectDef::damage(
                        EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        ))),
                        ValueDef::Constant(2),
                    ),
                ),
            ),
            (
                &[CostDef::Mana(mana_cost!("{5}"))],
                AbilityDef::spell(
                    "Firaga — Fire Magic deals 3 damage to each creature.",
                    EffectDef::damage(
                        EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        ))),
                        ValueDef::Constant(3),
                    ),
                ),
            ),
        ],
    )]),
);

// FIN 137 — Firion, Wild Rose Warrior
// Audit: unsupported — Needs an activated-cost reduction restricted to equip abilities on each copied Equipment; current permanent ability-cost reductions apply to every activated ability of the matching permanent.
pub(in crate::card::sets) static FIRION_WILD_ROSE_WARRIOR: CardRecord = CardRecord::new(
    "Firion, Wild Rose Warrior",
    "98366937-d15b-4a66-b9f6-878d50b63871",
    "Elizabeth Peiró",
    CardRules::unsupported(),
);

// FIN 138 — Freya Crescent
// Audit: unsupported — Needs one mana permission accepting either an Equipment spell or an equip ability; current restrictions combine with AND and cannot identify an equip ability as an alternative spending destination.
pub(in crate::card::sets) static FREYA_CRESCENT: CardRecord = CardRecord::new(
    "Freya Crescent",
    "9921f646-e893-44db-ac89-0633c1009788",
    "Nereida",
    CardRules::unsupported(),
);

// FIN 139 — Gilgamesh, Master-at-Arms
// Audit: unsupported — Needs a reflexive trigger after the selected Equipment enter, retained if Gilgamesh leaves, and an attachment operation between an independently chosen Equipment and Samurai.
pub(in crate::card::sets) static GILGAMESH_MASTER_AT_ARMS: CardRecord = CardRecord::new(
    "Gilgamesh, Master-at-Arms",
    "1eb81329-fb7a-4347-b96c-9960a5c48e87",
    "Lorenzo Mastroianni",
    CardRules::unsupported(),
);

// FIN 140 — Haste Magic
// Audit: unsupported — Needs exile-play permission to expire as the holder's next end step begins; the existing UntilYourNextEndStep path shares a turn-count expiry with end-of-next-turn permissions and remains usable after that end step.
pub(in crate::card::sets) static HASTE_MAGIC: CardRecord = CardRecord::new(
    "Haste Magic",
    "3af9d100-70ee-4c6c-a762-11a0c4f3ef6f",
    "David Astruga",
    CardRules::unsupported(),
);

// FIN 141 — Hill Gigas
pub(in crate::card::sets) static HILL_GIGAS: CardRecord = CardRecord::new(
    "Hill Gigas",
    "2a8b3e1e-5c10-4360-ac1c-83b2e026278c",
    "Heonhwa",
    CardRules::new_creature(mana_cost!("{4}{R}{R}"), &["Giant"], 5, 4).with_abilities(&[
        abilities::trample(),
        abilities::haste(),
        abilities::typecycling!(
            "Mountaincycling {2}",
            &[CostDef::Mana(mana_cost!("{2}"))],
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mountain"))
        ),
    ]),
);

// FIN 142 — Item Shopkeep
// Audit: unsupported — Needs a target predicate for a creature with at least one Equipment attached; the existing attachment predicates select attachments or Aura-enchanted hosts, not equipped hosts.
pub(in crate::card::sets) static ITEM_SHOPKEEP: CardRecord = CardRecord::new(
    "Item Shopkeep",
    "bd2db3f5-fd0d-4817-af90-6bea1f07e16b",
    "IWAO",
    CardRules::unsupported(),
);

// FIN 143 — Laughing Mad
pub(in crate::card::sets) static LAUGHING_MAD: CardRecord = CardRecord::new(
    "Laughing Mad",
    "cd5b9daf-6325-4eb4-a069-4a8cc7807884",
    "RARE ENGINE",
    CardRules::new_instant(mana_cost!("{2}{R}")).with_abilities(&[
        AbilityDef::spell(
            "Draw two cards.",
            abilities::draw_cards(ValueDef::Constant(2)),
        )
        .with_spell_additional_cost(&CostDef::discard(ObjectPredicateDef::Any)),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{3}{R}"))]),
    ]),
);

// FIN 144 — Light of Judgment
// Audit: unsupported — Needs selection of an Equipment attached to the referenced target creature; AttachedTo accepts an object predicate and cannot identify a previously selected target or bound host.
pub(in crate::card::sets) static LIGHT_OF_JUDGMENT: CardRecord = CardRecord::new(
    "Light of Judgment",
    "98bb716d-ca66-445f-9cb3-0fc656c8ebff",
    "Daniel Landerman",
    CardRules::unsupported(),
);

// FIN 145 — Mysidian Elder
pub(in crate::card::sets) static MYSIDIAN_ELDER: CardRecord = CardRecord::new(
    "Mysidian Elder",
    "6bc39af4-be19-4889-b930-df7ebf7b9481",
    "David Astruga",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Wizard"], 1, 3).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, create a 0/1 black Wizard creature \
             token with \"Whenever you cast a noncreature spell, this \
             token deals 1 damage to each opponent.\"",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(WIZARD_TOKEN))),
        ),
    ]),
);

// FIN 146 — Nibelheim Aflame
pub(in crate::card::sets) static NIBELHEIM_AFLAME: CardRecord = CardRecord::new(
    "Nibelheim Aflame",
    "3d3e926a-74af-4996-849f-d31e0fdedeae",
    "Arou",
    CardRules::new_sorcery(mana_cost!("{2}{R}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Choose target creature you control. It deals damage equal to \
             its power to each other creature. If this spell was cast from \
             a graveyard, discard your hand and draw four cards.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::damage_from(
                    ObjectRefDef::Target(TargetIndex::PRIMARY),
                    EffectRecipientDef::objects(ObjectSetDef::ExceptObject {
                        objects: &ObjectSetDef::Query(ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        )),
                        object: ObjectRefDef::Target(TargetIndex::PRIMARY),
                    }),
                    ValueDef::TargetPower(TargetIndex::PRIMARY),
                ),
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceCastFrom(ZoneKind::Graveyard),
                    then: &EffectDef::Sequence(&[
                        EffectDef::Discard {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                ObjectPredicateDef::Any,
                                &[ZoneKind::Hand],
                                PlayerRelation::You,
                            )),
                            selection: DiscardSelectionDef::RecipientChooses,
                            then: None,
                        },
                        abilities::draw_cards(ValueDef::Constant(4)),
                    ]),
                },
            ]),
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{5}{R}{R}"))]),
    ]),
);

// FIN 147 — Opera Love Song
// Audit: unsupported — Needs exile-play permission that ends as the holder's next end step begins; the current next-end-step permission survives beyond that step.
pub(in crate::card::sets) static OPERA_LOVE_SONG: CardRecord = CardRecord::new(
    "Opera Love Song",
    "0343916d-1b65-4e95-aef1-e72dbcebf0c4",
    "Grace Zhu",
    CardRules::unsupported(),
);

// FIN 148 — Prompto Argentum
// Audit: unsupported — Needs total mana spent recorded with a spell-cast event and readable by its triggered ability, including additional costs and cost reductions; the current cast-event values expose mana value and colors spent, not the amount paid.
pub(in crate::card::sets) static PROMPTO_ARGENTUM: CardRecord = CardRecord::new(
    "Prompto Argentum",
    "4c617bcd-05f8-40c2-bb38-489bc863ce6b",
    "Billy Christian",
    CardRules::unsupported(),
);

// FIN 149 — Queen Brahne
pub(in crate::card::sets) static QUEEN_BRAHNE: CardRecord = CardRecord::new(
    "Queen Brahne",
    "c38c98bb-74c9-460f-9997-ea5c5f922347",
    "Lorenzo Mastroianni",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Noble"], 2, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::prowess(),
            AbilityDef::triggered(
                "Whenever Queen Brahne attacks, create a 0/1 black Wizard \
                 creature token with \"Whenever you cast a noncreature spell, \
                 this token deals 1 damage to each opponent.\"",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(WIZARD_TOKEN))),
            ),
        ]),
);

// FIN 150 — Random Encounter
// Audit: unsupported — Needs a permanent haste grant to the specific creatures returned by the mill result; the production boundary rejects permanent ability grants to bound recipients that can name objects outside the battlefield.
pub(in crate::card::sets) static RANDOM_ENCOUNTER: CardRecord = CardRecord::new(
    "Random Encounter",
    "3618e283-2df9-4eb9-97b0-96b55ee31cc0",
    "Ben Wootten",
    CardRules::unsupported(),
);

// FIN 151 — Raubahn, Bull of Ala Mhigo
// Audit: unsupported — Needs attachment between an independently targeted Equipment and attacking creature; current Attach and AttachToSource operations require one side of the attachment to be the ability source.
pub(in crate::card::sets) static RAUBAHN_BULL_OF_ALA_MHIGO: CardRecord = CardRecord::new(
    "Raubahn, Bull of Ala Mhigo",
    "7035d11b-525f-4120-8dcb-610095196681",
    "Julia Vasilyeva",
    CardRules::unsupported(),
);

// FIN 152 — Red Mage's Rapier
pub(in crate::card::sets) static RED_MAGE_S_RAPIER: CardRecord = CardRecord::new(
    "Red Mage's Rapier",
    "e0579955-75f9-47a9-8b03-e287d120826a",
    "Alexandre Honoré",
    CardRules::new_artifact(mana_cost!("{1}{R}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::job_select(),
            AbilityDef::static_ability(
                "Equipped creature has \"Whenever you cast a noncreature \
                 spell, this creature gets +2/+0 until end of turn\" and is a \
                 Wizard in addition to its other types.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_ability(&AbilityDef::triggered(
                            "Whenever you cast a noncreature spell, this creature gets \
                             +2/+0 until end of turn.",
                            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                                    CardType::Creature,
                                )),
                                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                            ])),
                            EffectDef::Apply {
                                recipient: EffectRecipientDef::Source,
                                effect: AppliedEffectDef::modify_power_toughness(
                                    ValueDef::Constant(2),
                                    ValueDef::Constant(0),
                                ),
                                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                            },
                        )),
                        AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&[
                            "Wizard",
                        ])),
                    ]),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{3}"))], "Equip {3}"),
        ]),
);

// FIN 153 — Sabotender
pub(in crate::card::sets) static SABOTENDER: CardRecord = CardRecord::new(
    "Sabotender",
    "12df1295-8b08-4c8e-bac9-55b4f514c0be",
    "Toni Infante",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Plant"], 2, 1).with_abilities(&[
        abilities::reach(),
        AbilityDef::triggered(
            "Landfall — Whenever a land you control enters, this creature \
             deals 1 damage to each opponent.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(1)),
        ),
    ]),
);

// FIN 154 — Samurai's Katana
pub(in crate::card::sets) static SAMURAI_S_KATANA: CardRecord = CardRecord::new(
    "Samurai's Katana",
    "11f1d378-c78c-402a-ac46-2d32598c23e7",
    "Smirtouille",
    CardRules::new_artifact(mana_cost!("{2}{R}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::job_select(),
            AbilityDef::static_ability(
                "Equipped creature gets +2/+2, has trample and haste, and is a \
                 Samurai in addition to its other types.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&abilities::trample()),
                        AppliedEffectDef::add_ability(&abilities::haste()),
                        AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&[
                            "Samurai",
                        ])),
                    ]),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{5}"))], "Murasame — Equip {5}"),
        ]),
);

// FIN 155 — Sandworm
pub(in crate::card::sets) static SANDWORM: CardRecord = CardRecord::new(
    "Sandworm",
    "a6e021da-2397-4ad3-a07b-65c701df531a",
    "Awanqi (Angela Wang)",
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Worm"], 5, 4).with_abilities(&[
        abilities::haste(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, destroy target land. Its \
             controller may search their library for a basic land card, \
             put it onto the battlefield tapped, then shuffle.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Land),
            )],
            EffectDef::Sequence(&[
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
                EffectDef::May {
                    player: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                        ObjectRefDef::Target(TargetIndex::PRIMARY),
                    )),
                    effect: &EffectDef::SearchZone {
                        player: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                            ObjectRefDef::Target(TargetIndex::PRIMARY),
                        )),
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
                },
            ]),
        ),
    ]),
);

// FIN 156 — Seifer Almasy
pub(in crate::card::sets) static SEIFER_ALMASY: CardRecord = CardRecord::new(
    "Seifer Almasy",
    "9c776984-99ea-4181-ac95-78c41ba9d54f",
    "Kotetsu Kinoshita",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Human", "Knight"], 3, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever a creature you control attacks alone, it gains \
                 double strike until end of turn.",
                TriggerEventDef::attacks_in_declaration(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    1,
                    Some(1),
                ),
                EffectDef::Apply {
                    recipient: EffectRecipientDef::TriggeringObject,
                    effect: AppliedEffectDef::add_ability(&abilities::double_strike()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::triggered_with_targets(
                "Fire Cross — Whenever Seifer Almasy deals combat damage to a \
                 player, you may cast target instant or sorcery card with mana \
                 value 3 or less from your graveyard without paying its mana \
                 cost. If that spell would be put into your graveyard, exile \
                 it instead.",
                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Instant),
                                ObjectPredicateDef::HasType(CardType::Sorcery),
                            ]),
                            ObjectPredicateDef::ManaValueAtMost(3),
                        ]),
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::You),
                    },
                )],
                EffectDef::MayCastTargetWithoutPaying {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ability: &AbilityDef::alternative_cast(
                        crate::NO_COSTS,
                        AlternativeCastKindDef::Granted,
                        Some("Cast without paying its mana cost, then exile it."),
                        EffectDef::None,
                    )
                    .with_exile_if_put_into_graveyard(),
                },
            ),
        ]),
);

// FIN 157 — Self-Destruct
pub(in crate::card::sets) static SELF_DESTRUCT: CardRecord = CardRecord::new(
    "Self-Destruct",
    "7661003c-bf83-46bb-bcc0-8fbf5819ffa8",
    "Liiga Smilshkalne",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature you control deals X damage to any other \
         target and X damage to itself, where X is its power.",
        &[
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            }),
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget).another(),
        ],
        EffectDef::damage_simultaneously(&[
            DamageAssignmentDef::from(
                ObjectRefDef::Target(TargetIndex::PRIMARY),
                EffectRecipientDef::Target(TargetIndex(1)),
                ValueDef::TargetPower(TargetIndex::PRIMARY),
            ),
            DamageAssignmentDef::from(
                ObjectRefDef::Target(TargetIndex::PRIMARY),
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::TargetPower(TargetIndex::PRIMARY),
            ),
        ]),
    )]),
);

// FIN 158 — Sidequest: Play Blitzball // World Champion, Celestial Weapon
// Audit: unsupported — Needs per-player combat-damage totals for the current turn; the available damage history does not separately total combat damage to each player.
pub(in crate::card::sets) static SIDEQUEST_PLAY_BLITZBALL: CardRecord = CardRecord::new(
    "Sidequest: Play Blitzball // World Champion, Celestial Weapon",
    "31e2ad37-73cf-4858-8a3a-fc1165cd21a7",
    "Ittoku",
    CardRules::unsupported(),
);

// FIN 159 — Sorceress's Schemes
// Audit: unsupported — Needs an exile-card predicate for a printed or granted flashback ability; current object predicates cannot inspect that alternative-cast capability.
pub(in crate::card::sets) static SORCERESS_S_SCHEMES: CardRecord = CardRecord::new(
    "Sorceress's Schemes",
    "7efd7627-0754-4685-9d04-8f5f82f45632",
    "Jessica Fong",
    CardRules::unsupported(),
);

// FIN 160 — Summon: Brynhildr
// Audit: unsupported — Needs one printed Saga ability to represent several chapter numbers and to remain identifiable to the Saga final-chapter rules; the current Saga authoring and recognition path represents exactly one chapter per ability. Also needs exile-play permission conditioned on this Saga receiving a lore counter during the current turn and a next-creature trigger with an end-of-turn expiry.
pub(in crate::card::sets) static SUMMON_BRYNHILDR: CardRecord = CardRecord::new(
    "Summon: Brynhildr",
    "8ab5429a-1075-49aa-9608-0610080fbf7a",
    "Kevin Glint",
    CardRules::unsupported(),
);

// FIN 161 — Summon: Esper Ramuh
// Audit: unsupported — Needs one printed Saga ability to represent several chapter numbers and to remain identifiable to the Saga final-chapter rules; the current Saga authoring and recognition path represents exactly one chapter per ability.
pub(in crate::card::sets) static SUMMON_ESPER_RAMUH: CardRecord = CardRecord::new(
    "Summon: Esper Ramuh",
    "840659ee-1493-4190-a514-c2c9ae14e331",
    "Justyna Dura",
    CardRules::unsupported(),
);

// FIN 162 — Summon: G.F. Cerberus
// Audit: unsupported — Needs a source-independent delayed trigger for only the next instant or sorcery spell this turn, expiring if unused; installed Once and ThisTurn lifetimes cannot currently be combined.
pub(in crate::card::sets) static SUMMON_G_F_CERBERUS: CardRecord = CardRecord::new(
    "Summon: G.F. Cerberus",
    "d0e5cbd4-401b-4456-80bf-d90beadfd1f8",
    "Kevin Glint",
    CardRules::unsupported(),
);

// FIN 163 — Summon: G.F. Ifrit
// Audit: unsupported — Needs one printed Saga ability to represent several chapter numbers and to remain identifiable to the Saga final-chapter rules; the current Saga authoring and recognition path represents exactly one chapter per ability.
pub(in crate::card::sets) static SUMMON_G_F_IFRIT: CardRecord = CardRecord::new(
    "Summon: G.F. Ifrit",
    "c6c73092-5195-4bdc-b039-a699f6e297b2",
    "Lordigan",
    CardRules::unsupported(),
);

// FIN 164 — Suplex
pub(in crate::card::sets) static SUPLEX: CardRecord = CardRecord::new(
    "Suplex",
    "f61693a2-7042-44e0-85ba-9bf12ab94e7e",
    "Fang Xinyu",
// Three damage that answers a recursive creature for good, or the
    // artifact half when there is nothing to throw.
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Suplex deals 3 damage to target creature. If that creature would die this turn, exile it \
                 instead.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                // The second sentence is about the creature, not about the damage: it is
                // applied to the target whether or not three damage was enough, or arrived
                // at all, so the two clauses resolve in order rather than as one linked
                // effect. A creature that shrugs the three off is still exiled if
                // something else finishes it before the turn ends.
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
                "Exile target artifact.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                )],
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
            ),
        ],
    )),
);

// FIN 165 — Thunder Magic
pub(in crate::card::sets) static THUNDER_MAGIC: CardRecord = CardRecord::new(
    "Thunder Magic",
    "9f2b202c-91da-40ec-8324-6b6be7cb3bc8",
    "Josephine Chang",
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[tiered(
        "Tiered (Choose one additional cost.)",
        &[
            (
                &[CostDef::Mana(mana_cost!("{0}"))],
                AbilityDef::spell_with_targets(
                    "Thunder — Thunder Magic deals 2 damage to target creature.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    )],
                    EffectDef::damage(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ValueDef::Constant(2),
                    ),
                ),
            ),
            (
                &[CostDef::Mana(mana_cost!("{3}"))],
                AbilityDef::spell_with_targets(
                    "Thundara — Thunder Magic deals 4 damage to target creature.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    )],
                    EffectDef::damage(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ValueDef::Constant(4),
                    ),
                ),
            ),
            (
                &[CostDef::Mana(mana_cost!("{5}{R}"))],
                AbilityDef::spell_with_targets(
                    "Thundaga — Thunder Magic deals 8 damage to target creature.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    )],
                    EffectDef::damage(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ValueDef::Constant(8),
                    ),
                ),
            ),
        ],
    )]),
);

// FIN 166 — Triple Triad
pub(in crate::card::sets) static TRIPLE_TRIAD: CardRecord = CardRecord::new(
    "Triple Triad",
    "d9a1de36-7f47-4b28-bb56-38d7e5bed82f",
    "Ben Wootten",
CardRules::new_enchantment(mana_cost!("{3}{R}{R}{R}")).with_abilities(&[
AbilityDef::triggered("At the beginning of your upkeep, each player exiles the top \
 card of their library. Until end of turn, you may play the \
 card you own exiled this way and each other card exiled this \
 way with lesser mana value than it without paying their mana \
 costs.",
TriggerEventDef::StepBegins {
step:TurnStepDef::Upkeep,
player:PlayerRelation::You}
,
EffectDef::BindObjects(BindObjectsDef {
source:ObjectCollectionSourceDef::TopCards {
player:PlayerRefDef::EffectController,
count:ValueDef::Constant(1)}
,
binding:crate::Binding!("mine"),
then:&EffectDef::BindObjects(BindObjectsDef {
source:ObjectCollectionSourceDef::TopCards {
player:PlayerRefDef::Opponent,
count:ValueDef::Constant(1)}
,
binding:crate::Binding!("theirs"),
then:&EffectDef::WithZoneMoveResult {
effect:&EffectDef::move_to_zone(EffectRecipientDef::objects(ObjectSetDef::Union(&[
ObjectSetDef::Binding(crate::Binding!("mine")),
ObjectSetDef::Binding(crate::Binding!("theirs"))])),
ZoneKind::Exile,
ZonePlacement::Top),
binding:crate::Binding!("exiled"),
then:&EffectDef::MayPlayWithoutPaying(FreePlayDef {
objects:ObjectSetDef::Union(&[
ObjectSetDef::Matching {
objects:&ObjectSetDef::ZoneChangeSuccessorsOfBinding(crate::Binding!("exiled")),
object:ObjectSetFilterDef::Predicate(&ObjectPredicateDef::OwnedBy(PlayerRelation::You))}
,
ObjectSetDef::Matching {
objects:&ObjectSetDef::ZoneChangeSuccessorsOfBinding(crate::Binding!("exiled")),
object:ObjectSetFilterDef::Predicate(&ObjectPredicateDef::ManaValueAtMostValue(ValueDef::Sum(&SumValueDef::new(ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
objects:ObjectSetDef::Matching {
objects:&ObjectSetDef::ZoneChangeSuccessorsOfBinding(crate::Binding!("exiled")),
object:ObjectSetFilterDef::Predicate(&ObjectPredicateDef::OwnedBy(PlayerRelation::You))}
,
select:ObjectValueDef::ManaValue,
operation:AggregateOperationDef::Sum}
),
ValueDef::Constant(-1)))))}
]),
duration:FreePlayDurationDef::UntilEndOfTurn,
mandatory:false,
grants_haste:false}
)}
}
)}
))]),

);

// FIN 167 — Unexpected Request
// Audit: unsupported — Needs attachment between the independently selected Equipment and targeted creature; current Attach and AttachToSource require the ability source to be one of those objects.
pub(in crate::card::sets) static UNEXPECTED_REQUEST: CardRecord = CardRecord::new(
    "Unexpected Request",
    "0265fd20-a85d-49ce-b338-4c40843a5b18",
    "Ignatius Budi",
    CardRules::unsupported(),
);

// FIN 168 — Vaan, Street Thief
// Audit: unsupported — Needs one combat-damage event per player for the group of Scouts, Pirates, and Rogues dealing damage simultaneously; per-creature triggers would create too many exile and Treasure offers.
pub(in crate::card::sets) static VAAN_STREET_THIEF: CardRecord = CardRecord::new(
    "Vaan, Street Thief",
    "50e1ec29-9de3-4f1b-b818-057e030d475b",
    "Jake Murray",
    CardRules::unsupported(),
);

// FIN 169 — Warrior's Sword
pub(in crate::card::sets) static WARRIOR_S_SWORD: CardRecord = CardRecord::new(
    "Warrior's Sword",
    "cb98a7dd-542e-4448-b3bb-ff5d67a36535",
    "Andrea Tentori Montalto",
    CardRules::new_artifact(mana_cost!("{3}{R}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::job_select(),
            AbilityDef::static_ability(
                "Equipped creature gets +3/+2 and is a Warrior in addition to \
                 its other types.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(3),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&[
                            "Warrior",
                        ])),
                    ]),
                },
            ),
            abilities::equip(
                &[CostDef::Mana(mana_cost!("{5}"))],
                "Equip {5} ({5}: Attach to target creature you control. Equip \
                 only as a sorcery.)",
            ),
        ]),
);

// FIN 170 — Zell Dincht
pub(in crate::card::sets) static ZELL_DINCHT: CardRecord = CardRecord::new(
    "Zell Dincht",
    "135d6b27-9168-4513-9d7d-56edae048857",
    "Kevin Sidharta",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Monk"], 0, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "You may play an additional land on each of your turns.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Controller,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayAdditionalLands(1)),
                },
            ),
            AbilityDef::static_ability(
                "Zell Dincht gets +1/+0 for each land you control.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Land),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        ValueDef::Constant(0),
                    ),
                },
            ),
            AbilityDef::triggered(
                "At the beginning of your end step, return a land you control \
                 to its owner's hand.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::You,
                },
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
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
            ),
        ]),
);

// FIN 171 — Airship Crash
pub(in crate::card::sets) static AIRSHIP_CRASH: CardRecord = CardRecord::new(
    "Airship Crash",
    "ec91c4e4-711f-464d-bc83-e6813f4fdcdb",
    "Enora Mercier",
    CardRules::new_instant(mana_cost!("{2}{G}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Destroy target artifact, enchantment, or creature with flying.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                    ]),
                ]),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
        abilities::cycling!("Cycling {2}", &[CostDef::Mana(mana_cost!("{2}"))]),
    ]),
);

// FIN 172 — Ancient Adamantoise
// Audit: unsupported — Needs marked damage to persist through cleanup steps on this creature; the cleanup procedure currently removes damage without a per-permanent retention rule.
pub(in crate::card::sets) static ANCIENT_ADAMANTOISE: CardRecord = CardRecord::new(
    "Ancient Adamantoise",
    "4c139f30-5ecd-48fd-ae7c-ec2cc98889ff",
    "Kevin Glint",
    CardRules::unsupported(),
);

// FIN 173 — Balamb T-Rexaur
pub(in crate::card::sets) static BALAMB_T_REXAUR: CardRecord = CardRecord::new(
    "Balamb T-Rexaur",
    "e5857b1b-73bc-458e-b26b-7ed8bef785f3",
    "Fang Xinyu",
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Dinosaur"], 6, 6).with_abilities(&[
        abilities::trample(),
        abilities::enters_trigger(
            "When this creature enters, you gain 3 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ),
        abilities::typecycling!(
            "Forestcycling {2}",
            &[CostDef::Mana(mana_cost!("{2}"))],
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Forest"))
        ),
    ]),
);

// FIN 174 — Bard's Bow
pub(in crate::card::sets) static BARD_S_BOW: CardRecord = CardRecord::new(
    "Bard's Bow",
    "2ac03e90-1e16-453f-88e4-a0448db73403",
    "Josephine Chang",
    CardRules::new_artifact(mana_cost!("{2}{G}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::job_select(),
            AbilityDef::static_ability(
                "Equipped creature gets +2/+2, has reach, and is a Bard in \
                 addition to its other types.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&abilities::reach()),
                        AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Bard"])),
                    ]),
                },
            ),
            abilities::equip(
                &[CostDef::Mana(mana_cost!("{6}"))],
                "Perseus's Bow — Equip {6} ({6}: Attach to target creature you \
                 control. Equip only as a sorcery.)",
            ),
        ]),
);

// FIN 175 — Bartz and Boko
// Audit: unsupported — Needs simultaneous damage from a dynamically selected collection of Birds, each using its own power; damage assignments name individual object references rather than iterate a live source collection.
pub(in crate::card::sets) static BARTZ_AND_BOKO: CardRecord = CardRecord::new(
    "Bartz and Boko",
    "d818d574-2832-4a7a-a13b-aa6e695fdaa5",
    "Ryuichi Sakuma",
    CardRules::unsupported(),
);

// FIN 176 — Blitzball Shot
pub(in crate::card::sets) static BLITZBALL_SHOT: CardRecord = CardRecord::new(
    "Blitzball Shot",
    "ec8ab637-3d7d-4712-9f83-8920f808f715",
    "JUGEMT",
    CardRules::new_instant(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +3/+3 and gains trample until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(3),
                ),
                AppliedEffectDef::add_ability(&abilities::trample()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// FIN 177 — Cactuar
pub(in crate::card::sets) static CACTUAR: CardRecord = CardRecord::new(
    "Cactuar",
    "d5273db4-6214-41e4-825a-612fca8bbe03",
    "Kevin Sidharta",
    CardRules::new_creature(mana_cost!("{G}"), &["Plant"], 3, 3).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered_if(
            "At the beginning of your end step, if this creature didn't \
             enter the battlefield this turn, return it to its owner's \
             hand.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::SourceMatches {
                object: ObjectPredicateDef::Not(&ObjectPredicateDef::EnteredThisTurn),
            },
            EffectDef::move_to_zone(
                EffectRecipientDef::Source,
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// FIN 178 — Chocobo Kick
pub(in crate::card::sets) static CHOCOBO_KICK: CardRecord = CardRecord::new(
    "Chocobo Kick",
    "ff8c8be0-8223-499c-8704-cb68e0a42ce2",
    "Ben Wootten",
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_abilities(&[
        abilities::kicker(&[CostDef::return_to_hand(
            ObjectPredicateDef::HasType(CardType::Land),
            CostQuantityDef::Fixed(1),
        )]),
        AbilityDef::spell_with_targets(
            "Target creature you control deals damage equal to its power \
             to target creature an opponent controls. If this spell was \
             kicked, the creature you control deals twice that much damage \
             instead.",
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
                ValueDef::IfAdditionalCostPaid(&AdditionalCostValueDef::new(
                    AdditionalCostIndex::PRIMARY,
                    ValueDef::Scaled(&ScaledValueDef {
                        value: ValueDef::TargetPower(TargetIndex::PRIMARY),
                        factor: 2,
                    }),
                    ValueDef::TargetPower(TargetIndex::PRIMARY),
                )),
            ),
        ),
    ]),
);

// FIN 179 — Chocobo Racetrack
pub(in crate::card::sets) static CHOCOBO_RACETRACK: CardRecord = CardRecord::new(
    "Chocobo Racetrack",
    "1565813b-6912-42f4-bae0-22136a2d6a92",
    "Bachzim",
    CardRules::new_artifact(mana_cost!("{3}{G}{G}")).with_abilities(&[AbilityDef::triggered(
        "Landfall — Whenever a land you control enters, create a 2/2 \
         green Bird creature token with \"Whenever a land you control \
         enters, this token gets +1/+0 until end of turn.\"",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Land),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ]),
            None,
            Some(ZoneKind::Battlefield),
        ),
        EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
            TokenCharacteristics::creature(&["Bird"], &[ManaColor::Green], 2, 2).with_abilities(&[
                AbilityDef::triggered(
                    "Whenever a land you control enters, this token gets +1/+0 \
                 until end of turn.",
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
                            ValueDef::Constant(0),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ),
            ]),
        ))),
    )]),
);

// FIN 180 — Clash of the Eikons
pub(in crate::card::sets) static CLASH_OF_THE_EIKONS: CardRecord = CardRecord::new(
    "Clash of the Eikons",
    "75c18134-f517-4a68-8640-0426b3cd4f6c",
    "Gal Or",
    CardRules::new_sorcery(mana_cost!("{G}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Target creature you control fights target creature an \
                 opponent controls.",
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
                EffectDef::Fight {
                    first: ObjectRefDef::Target(TargetIndex::PRIMARY),
                    second: ObjectRefDef::Target(TargetIndex(1)),
                    excess: None,
                },
            ),
            AbilityDef::spell_with_targets(
                "Remove a lore counter from target Saga you control. (Removing \
                 lore counters doesn't cause chapter abilities to trigger.)",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Saga")),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::RemoveCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::Lore,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::spell_with_targets(
                "Put a lore counter on target Saga you control.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Saga")),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::Lore,
                    amount: ValueDef::Constant(1),
                },
            ),
        ],
    )
    .with_mode_selection(1, 3, false)]),
);

// FIN 181 — Coliseum Behemoth
pub(in crate::card::sets) static COLISEUM_BEHEMOTH: CardRecord = CardRecord::new(
    "Coliseum Behemoth",
    "900206f1-7dd6-4db9-a430-73e560fd196b",
    "Ryu Fujinaka",
    CardRules::new_creature(mana_cost!("{5}{G}{G}"), &["Beast"], 7, 7).with_abilities(&[
        abilities::trample(),
        AbilityDef::modal_triggered(
            "When this creature enters, choose one —",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[
                AbilityDef::spell_with_targets(
                    "Destroy target artifact or enchantment.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                        ]),
                    )],
                    EffectDef::Destroy {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        then: None,
                    },
                ),
                AbilityDef::spell("Draw a card.", abilities::draw_cards(ValueDef::Constant(1))),
            ],
        ),
    ]),
);

// FIN 182 — Commune with Beavers
pub(in crate::card::sets) static COMMUNE_WITH_BEAVERS: CardRecord = CardRecord::new(
    "Commune with Beavers",
    "784287c2-43c5-4210-93ae-cdd33b9acb1b",
    "hippo",
    CardRules::new_sorcery(mana_cost!("{G}")).with_abilities(&[AbilityDef::spell(
        "Look at the top three cards of your library. You may reveal \
         an artifact, creature, or land card from among them and put \
         it into your hand. Put the rest on the bottom of your library \
         in any order.",
        EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
            source: ObjectCollectionSourceDef::TopCards {
                player: PlayerRefDef::EffectController,
                count: ValueDef::Constant(3),
            },
            actor: PlayerRefDef::EffectController,
            inspection: CollectionInspectionDef::Look,
            object: ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]),
                ObjectPredicateDef::HasType(CardType::Land),
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
                    EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!("chosen"))),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
                EffectDef::ChooseObjectOrder(ChooseObjectOrderDef {
                    input: ObjectSetDef::Binding(crate::Binding!("rest")),
                    actor: PlayerRefDef::EffectController,
                    placement: ZonePlacement::Bottom,
                    visibility: ChoiceVisibilityDef::Private,
                    ordered: crate::Binding!("ordered_bottom"),
                    then: &EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "ordered_bottom"
                        ))),
                        ZoneKind::Library,
                        ZonePlacement::Bottom,
                    ),
                }),
            ]),
        }),
    )]),
);

// FIN 183 — Diamond Weapon
pub(in crate::card::sets) static DIAMOND_WEAPON: CardRecord = CardRecord::new(
    "Diamond Weapon",
    "6ce7f494-2a19-4b11-94d4-fc5e5a7068bd",
    "Esuthio",
    CardRules::new_artifact_creature(mana_cost!("{7}{G}{G}"), &["Elemental"], 8, 8)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "This spell costs {1} less to cast for each permanent card in \
                 your graveyard.",
                EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(
                    &ObjectQueryDef::matching(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                        ]),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    ),
                )),
            )
            .with_source_zones(&[ZoneKind::Hand]),
            abilities::reach(),
            AbilityDef::static_ability(
                "Immune — Prevent all combat damage that would be dealt to \
                 Diamond Weapon.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::PreventDamage(
                        DamageEventMatcherDef {
                            kind: DamageKindDef::Combat,
                            recipient: DamageRecipientMatcherDef::AffectedObject,
                            ..DamageEventMatcherDef::ANY
                        },
                    )),
                },
            ),
        ]),
);

// FIN 184 — The Earth Crystal
// Audit: unsupported — Needs a prospective +1/+1 counter-placement replacement that doubles each placement on a controlled creature; doubling counters afterward incorrectly counts existing counters and changes counter-placement events.
pub(in crate::card::sets) static THE_EARTH_CRYSTAL: CardRecord = CardRecord::new(
    "The Earth Crystal",
    "d585e218-3dc8-4fbd-8ad2-795fbc9b2155",
    "Pablo Mendoza",
    CardRules::unsupported(),
);

// FIN 185 — Esper Origins // Summon: Esper Maduin
// Audit: unsupported — Needs an instruction that exiles the currently resolving sorcery and returns that spell card transformed before resolution ends; linked exile handles battlefield and nonstack card recipients, and cannot move the resolving spell.
pub(in crate::card::sets) static ESPER_ORIGINS: CardRecord = CardRecord::new(
    "Esper Origins // Summon: Esper Maduin",
    "0f503360-216a-4629-89b2-d32072850aef",
    "Solan & Danciao",
    CardRules::unsupported(),
);

// FIN 186 — Galuf's Final Act
pub(in crate::card::sets) static GALUF_S_FINAL_ACT: CardRecord = CardRecord::new(
    "Galuf's Final Act",
    "0ce05634-6c01-4941-a135-904cb4e33ac4",
    "Nijihayashi",
    CardRules::new_instant(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Until end of turn, target creature gets +1/+0 and gains \
         \"When this creature dies, put a number of +1/+1 counters \
         equal to its power on up to one target creature.\"",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                AppliedEffectDef::add_ability(&AbilityDef::triggered_with_targets(
                    "When this creature dies, put a number of +1/+1 counters equal \
                     to its power on up to one target creature.",
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::Source,
                        Some(ZoneKind::Battlefield),
                        Some(ZoneKind::Graveyard),
                    ),
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
                        amount: ValueDef::TriggeringObjectPower,
                    },
                )),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// FIN 187 — Gigantoad
pub(in crate::card::sets) static GIGANTOAD: CardRecord = CardRecord::new(
    "Gigantoad",
    "bc10d648-4053-460f-bc52-9c20477bf6de",
    "Hristo D. Chukov",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Frog"], 4, 4).with_abilities(&[
        AbilityDef::static_ability(
            "As long as you control seven or more lands, this creature \
             gets +2/+2.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    comparison: ComparisonDef::GreaterOrEqual,
                    right: ValueDef::Constant(7),
                }),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                },
            },
        ),
    ]),
);

// FIN 188 — Goobbue Gardener
pub(in crate::card::sets) static GOOBBUE_GARDENER: CardRecord = CardRecord::new(
    "Goobbue Gardener",
    "b7c3544a-5dd5-423e-8a40-ac4803db8adc",
    "Janna Sophia",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Plant", "Beast"], 1, 3)
        .with_abilities(&[abilities::tap_for(ManaColor::Green)]),
);

// FIN 189 — Gran Pulse Ochu
pub(in crate::card::sets) static GRAN_PULSE_OCHU: CardRecord = CardRecord::new(
    "Gran Pulse Ochu",
    "4dced21f-478c-4500-9484-af5864dea5cc",
    "Domenico Cava",
    CardRules::new_creature(mana_cost!("{G}"), &["Plant", "Beast"], 1, 1).with_abilities(&[
        abilities::deathtouch(),
        AbilityDef::activated(
            "{8}: Until end of turn, this creature gets +1/+1 for each \
             permanent card in your graveyard.",
            &[CostDef::Mana(mana_cost!("{8}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                        ]),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    )),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                        ]),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    )),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// FIN 190 — Gysahl Greens
pub(in crate::card::sets) static GYSAHL_GREENS: CardRecord = CardRecord::new(
    "Gysahl Greens",
    "020dee17-d85b-44c4-9faa-b1ef977956f4",
    "Andrea Tentori Montalto",
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_abilities(&[
        AbilityDef::spell(
            "Create a 2/2 green Bird creature token with \"Whenever a land \
             you control enters, this token gets +1/+0 until end of \
             turn.\"",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(BIRD_TOKEN))),
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{6}{G}"))]),
    ]),
);

// FIN 191 — Jumbo Cactuar
pub(in crate::card::sets) static JUMBO_CACTUAR: CardRecord = CardRecord::new(
    "Jumbo Cactuar",
    "db01c222-8795-47e9-a789-e7f749a3ee7d",
    "Jason Kiantoro",
    CardRules::new_creature(mana_cost!("{5}{G}{G}"), &["Plant"], 1, 7).with_abilities(&[
        AbilityDef::triggered(
            "10,000 Needles — Whenever this creature attacks, it gets \
             +9999/+0 until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(9999),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// FIN 192 — Loporrit Scout
pub(in crate::card::sets) static LOPORRIT_SCOUT: CardRecord = CardRecord::new(
    "Loporrit Scout",
    "a182bc66-bfda-4bf5-bd12-3de5dba60945",
    "Andrea Radeck",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Rabbit", "Scout"], 3, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever another creature you control enters, this creature \
             gets +1/+1 until end of turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
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

// FIN 193 — Prishe's Wanderings
// Audit: unsupported — Needs a reflexive trigger caused by actually searching the library, with its creature target chosen only after that search; an ordinary spell target would be announced too early.
pub(in crate::card::sets) static PRISHE_S_WANDERINGS: CardRecord = CardRecord::new(
    "Prishe's Wanderings",
    "d6e1dee0-e2cd-4899-a3ea-7d0df717c9ab",
    "Daniel Correia",
    CardRules::unsupported(),
);

// FIN 194 — Quina, Qu Gourmet
// Audit: unsupported — Needs token replacement that adds one Frog to the same prospective token batch; creating a Frog in a follow-up changes replacement ordering and entry events.
pub(in crate::card::sets) static QUINA_QU_GOURMET: CardRecord = CardRecord::new(
    "Quina, Qu Gourmet",
    "4f352b5e-9731-4a8e-b872-db5d3bf32211",
    "Fajareka Setiawan",
    CardRules::unsupported(),
);

// FIN 195 — Reach the Horizon
// Audit: unsupported — Needs one qualified search selecting up to two cards with distinct names; current search selection constrains each card independently.
pub(in crate::card::sets) static REACH_THE_HORIZON: CardRecord = CardRecord::new(
    "Reach the Horizon",
    "c25960e0-5779-4e20-89f3-03950ad9d91c",
    "ikeda_cpt",
    CardRules::unsupported(),
);

// FIN 196 — A Realm Reborn
pub(in crate::card::sets) static A_REALM_REBORN: CardRecord = CardRecord::new(
    "A Realm Reborn",
    "d1af74e4-38d5-44b5-85e1-4d13f6970453",
    "Anna Podedworna",
    CardRules::new_enchantment(mana_cost!("{4}{G}{G}")).with_abilities(&[
        AbilityDef::static_ability(
            "Other permanents you control have \"{T}: Add one mana of any \
             color.\"",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Enchantment),
                                ObjectPredicateDef::HasType(CardType::Land),
                                ObjectPredicateDef::HasType(CardType::Planeswalker),
                            ]),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::add_ability(&AbilityDef::activated_mana(
                    "{T}: Add one mana of any color.",
                    &[CostDef::TapSource],
                    EffectDef::AddMana(AddManaEffectDef::any_color()),
                )),
            },
        ),
    ]),
);

// FIN 197 — Ride the Shoopuf
pub(in crate::card::sets) static RIDE_THE_SHOOPUF: CardRecord = CardRecord::new(
    "Ride the Shoopuf",
    "19ad36d6-8bf4-490c-9980-b98a470af892",
    "Leonardo Santanna",
    CardRules::new_enchantment(mana_cost!("{1}{G}")).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "Landfall — Whenever a land you control enters, put a +1/+1 \
             counter on target creature you control.",
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
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "{5}{G}{G}: This enchantment becomes a 7/7 Beast creature in \
             addition to its other types.",
            &[CostDef::Mana(mana_cost!("{5}{G}{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Beast"])),
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(7),
                        ValueDef::Constant(7),
                    ),
                ]),
                duration: ResolvedEffectDurationDef::Permanent,
            },
        ),
    ]),
);

// FIN 198 — Rydia's Return
pub(in crate::card::sets) static RYDIA_S_RETURN: CardRecord = CardRecord::new(
    "Rydia's Return",
    "40a06165-2835-4610-86a1-7f684992fcf2",
    "Kohei Hayama",
    CardRules::new_sorcery(mana_cost!("{3}{G}{G}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "Creatures you control get +3/+3 until end of turn.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(3),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell_with_targets(
                "Return up to two target permanent cards from your graveyard \
                 to your hand.",
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                        ]),
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::You),
                    },
                    2,
                )],
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            ),
        ],
    )]),
);

// FIN 199 — Sazh Katzroy
pub(in crate::card::sets) static SAZH_KATZROY: CardRecord = CardRecord::new(
    "Sazh Katzroy",
    "1e2a3566-1390-457e-8077-d776a8671319",
    "Colin Boyer",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Human", "Pilot"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When Sazh Katzroy enters, you may search your library for a \
                 Bird or basic land card, reveal it, put it into your hand, \
                 then shuffle.",
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::SearchZone {
                        player: EffectRecipientDef::Controller,
                        source: ZoneKind::Library,
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Bird")),
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Land),
                                ObjectPredicateDef::Supertype(CardSupertype::Basic),
                            ]),
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
                },
            ),
            AbilityDef::triggered_with_targets(
                "Whenever Sazh Katzroy attacks, put a +1/+1 counter on target \
                 creature, then double the number of +1/+1 counters on that \
                 creature.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Sequence(&[
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::DoubleCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::PlusOnePlusOne,
                    },
                ]),
            ),
        ]),
);

// FIN 200 — Sazh's Chocobo
pub(in crate::card::sets) static SAZH_S_CHOCOBO: CardRecord = CardRecord::new(
    "Sazh's Chocobo",
    "dda6b4d0-1b60-46b0-b321-b9ffe15afff4",
    "Domenico Cava",
    CardRules::new_creature(mana_cost!("{G}"), &["Bird"], 0, 1).with_abilities(&[
        AbilityDef::triggered(
            "Landfall — Whenever a land you control enters, put a +1/+1 \
             counter on this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// FIN 201 — Sidequest: Raise a Chocobo // Black Chocobo
pub(in crate::card::sets) static SIDEQUEST_RAISE_A_CHOCOBO: CardRecord = CardRecord::new_dfc(
    "Sidequest: Raise a Chocobo // Black Chocobo",
    "0cbf911c-a721-4b84-8645-d83a0966be18",
    "Miho Midorikawa & Sansyu",
    &[
        (
            "Sidequest: Raise a Chocobo",
            CardRules::new_enchantment(mana_cost!("{1}{G}")).with_abilities(&[
                abilities::enters_trigger(
                    "When this enchantment enters, create a 2/2 green Bird \
                     creature token with \"Whenever a land you control enters, \
                     this token gets +1/+0 until end of turn.\"",
                    EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(BIRD_TOKEN))),
                ),
                AbilityDef::triggered_if(
                    "At the beginning of your first main phase, if you control \
                     four or more Birds, transform this enchantment.",
                    TriggerEventDef::StepBegins {
                        step: TurnStepDef::PrecombatMain,
                        player: PlayerRelation::You,
                    },
                    &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Bird")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        comparison: ComparisonDef::GreaterOrEqual,
                        right: ValueDef::Constant(4),
                    }),
                    EffectDef::Transform {
                        object: EffectRecipientDef::Source,
                    },
                ),
            ]),
        ),
        (
            "Black Chocobo",
            CardRules::new_creature_without_mana_cost(&["Bird"], 2, 2)
                .printed_colors(&[ManaColor::Green])
                .with_abilities(&[
                    AbilityDef::triggered(
                        "When this permanent transforms into Black Chocobo, search \
                         your library for a land card, put it onto the battlefield \
                         tapped, then shuffle.",
                        TriggerEventDef::transforms(ObjectPredicateDef::Source),
                        EffectDef::SearchZone {
                            player: EffectRecipientDef::Controller,
                            source: ZoneKind::Library,
                            object: ObjectPredicateDef::HasType(CardType::Land),
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
                    AbilityDef::triggered(
                        "Landfall — Whenever a land you control enters, Birds you \
                         control get +1/+0 until end of turn.",
                        TriggerEventDef::zone_changed(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Land),
                                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                            ]),
                            None,
                            Some(ZoneKind::Battlefield),
                        ),
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                                ObjectQueryDef::matching(
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Bird")),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::You,
                                ),
                            )),
                            effect: AppliedEffectDef::modify_power_toughness(
                                ValueDef::Constant(1),
                                ValueDef::Constant(0),
                            ),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    ),
                ]),
        ),
    ],
);

// FIN 202 — Summon: Fat Chocobo
// Audit: unsupported — Needs one printed Saga ability to represent several chapter numbers and to remain identifiable to the Saga final-chapter rules; the current Saga authoring and recognition path represents exactly one chapter per ability.
pub(in crate::card::sets) static SUMMON_FAT_CHOCOBO: CardRecord = CardRecord::new(
    "Summon: Fat Chocobo",
    "32eb192b-de6b-4814-8077-628d343d014e",
    "Joseph Weston",
    CardRules::unsupported(),
);

// FIN 203 — Summon: Fenrir
// Audit: unsupported — Needs a source-independent next-creature-spell trigger that expires this turn if unused and modifies that spell's future battlefield arrival; installed Once and ThisTurn lifetimes cannot currently be combined.
pub(in crate::card::sets) static SUMMON_FENRIR: CardRecord = CardRecord::new(
    "Summon: Fenrir",
    "93feb9d5-d004-4598-a448-b3488c869c05",
    "Chun Lo",
    CardRules::unsupported(),
);

// FIN 204 — Summon: Titan
pub(in crate::card::sets) static SUMMON_TITAN: CardRecord = CardRecord::new(
    "Summon: Titan",
    "5ce6ea96-7293-496d-b9c8-8ed6d6109a4d",
    "Andreia Ugrai",
    CardRules::new_enchantment_creature(mana_cost!("{3}{G}{G}"), &["Saga", "Giant"], 7, 7)
        .with_abilities(&[
            abilities::saga_chapter(
                1,
                "I — Mill five cards.",
                EffectDef::Mill {
                    player: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(5),
                },
            ),
            abilities::saga_chapter(
                2,
                "II — Return all land cards from your graveyard to the \
                 battlefield tapped.",
                EffectDef::WithBattlefieldArrival {
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Land),
                            &[ZoneKind::Graveyard],
                            PlayerRelation::You,
                        ))),
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                    arrival: BattlefieldArrivalDef {
                        modifications: &[BattlefieldEntryModificationDef::Tapped],
                        ..BattlefieldArrivalDef::DEFAULT
                    },
                },
            ),
            abilities::saga_chapter_with_targets(
                3,
                "III — Until end of turn, another target creature you control \
                 gains trample and gets +X/+X, where X is the number of lands \
                 you control.",
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
                        AppliedEffectDef::add_ability(&abilities::trample()),
                        AppliedEffectDef::modify_power_toughness(
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
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            abilities::reach(),
            abilities::trample(),
        ]),
);

// FIN 205 — Summoner's Grimoire
// Audit: unsupported — Needs an arbitrary chosen hand card to enter tapped and attacking when it is an enchantment; current attacking-entry support covers tokens and the ninjutsu source, not a chosen card's battlefield arrival.
pub(in crate::card::sets) static SUMMONER_S_GRIMOIRE: CardRecord = CardRecord::new(
    "Summoner's Grimoire",
    "d9fda3fc-569d-49f8-a2ed-e0b1d6668426",
    "Daniel Correia",
    CardRules::unsupported(),
);

// FIN 206 — Tifa Lockhart
pub(in crate::card::sets) static TIFA_LOCKHART: CardRecord = CardRecord::new(
    "Tifa Lockhart",
    "fb781323-2746-405d-a9b2-e778c037a6e9",
    "Laurel Austin",
CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Monk"], 1, 2)
        .with_supertype(CardSupertype::Legendary)
        // Doubling is +X/+0 where X is her power as this resolves, so two landfalls
        // in a turn compound: the second reads the size the first left behind.
        .with_abilities(&[
            abilities::trample(),
            AbilityDef::triggered(
                "Landfall — Whenever a land you control enters, double Tifa Lockhart's power until end of turn.",
                // A land you control, not any land: the opponent's fetchland does nothing
                // for her.
                TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]), None, Some(ZoneKind::Battlefield)),
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::SourcePower,
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// FIN 207 — Tifa's Limit Break
pub(in crate::card::sets) static TIFA_S_LIMIT_BREAK: CardRecord = CardRecord::new(
    "Tifa's Limit Break",
    "24d6eab7-22bd-494f-8cbe-204446f24be9",
    "Mikio Masuda",
    CardRules::new_instant(mana_cost!("{G}")).with_abilities(&[tiered(
        "Tiered (Choose one additional cost.)",
        &[
            (
                &[CostDef::Mana(mana_cost!("{0}"))],
                AbilityDef::spell_with_targets(
                    "Somersault — Target creature gets +2/+2 until end of turn.",
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
            ),
            (
                &[CostDef::Mana(mana_cost!("{2}"))],
                AbilityDef::spell_with_targets(
                    "Meteor Strikes — Double target creature's power and toughness \
                     until end of turn.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    )],
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::TargetPower(TargetIndex::PRIMARY),
                            ValueDef::TargetToughness(TargetIndex::PRIMARY),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ),
            ),
            (
                &[CostDef::Mana(mana_cost!("{6}{G}"))],
                AbilityDef::spell_with_targets(
                    "Final Heaven — Triple target creature's power and toughness \
                     until end of turn.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    )],
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Scaled(&ScaledValueDef {
                                value: ValueDef::TargetPower(TargetIndex::PRIMARY),
                                factor: 2,
                            }),
                            ValueDef::Scaled(&ScaledValueDef {
                                value: ValueDef::TargetToughness(TargetIndex::PRIMARY),
                                factor: 2,
                            }),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ),
            ),
        ],
    )]),
);

// FIN 208 — Torgal, A Fine Hound
// Audit: unsupported — Needs a resolved cast-trigger instruction modifying that specific spell's future battlefield arrival with a computed counter amount; WithBattlefieldArrival wraps a zone move and cannot attach an entry modification to a waiting spell.
pub(in crate::card::sets) static TORGAL_A_FINE_HOUND: CardRecord = CardRecord::new(
    "Torgal, A Fine Hound",
    "0f5725aa-42bb-4dfd-9c15-135b38b33da3",
    "Narendra Bintara Adi",
    CardRules::unsupported(),
);

// FIN 209 — Town Greeter
pub(in crate::card::sets) static TOWN_GREETER: CardRecord = CardRecord::new(
    "Town Greeter",
    "49cd4efa-4df4-4257-9a42-60330f7781e2",
    "Hayaken-Sarena",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Citizen"], 1, 1).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, mill four cards. You may put a \
             land card from among them into your hand. If you put a Town \
             card into your hand this way, you gain 2 life. (To mill four \
             cards, a player puts the top four cards of their library into \
             their graveyard.)",
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
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::WithZoneMoveResult {
                        effect: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "chosen"
                            ))),
                            ZoneKind::Hand,
                            ZonePlacement::Top,
                        ),
                        binding: crate::Binding!("returned"),
                        then: &EffectDef::IfCondition {
                            condition: &TriggerConditionDef::ObjectSetCount(
                                &ObjectSetCountConditionDef {
                                    objects: &ObjectSetDef::ZoneChangeSuccessorsOfBinding(
                                        crate::Binding!("returned"),
                                    ),
                                    predicate: ObjectSetPredicateDef::contains(
                                        &ObjectPredicateDef::Subtype(SubtypeDef::Literal("Town")),
                                    ),
                                },
                            ),
                            then: &EffectDef::GainLife {
                                recipient: EffectRecipientDef::Controller,
                                amount: ValueDef::Constant(2),
                            },
                        },
                    },
                }),
            ]),
        ),
    ]),
);

// FIN 210 — Traveling Chocobo (alternate printing)
const TRAVELING_CHOCOBO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TRAVELING_CHOCOBO,
    1,
    "2462df62-fc35-47ed-9571-40452074dc6d",
    "Ashley Mackenzie",
);

// FIN 211 — Vanille, Cheerful l'Cie
// Audit: unsupported — Needs meld topology combining two physical cards into one permanent, together with splitting and tracking both components when that permanent changes zones; transforming double-faced cards do not represent a melded permanent.
pub(in crate::card::sets) static VANILLE_CHEERFUL_L_CIE: CardRecord = CardRecord::new(
    "Vanille, Cheerful l'Cie",
    "91226c1a-63a0-494e-bcf0-77c2d6f49213",
    "Simon Dominic",
    CardRules::unsupported(),
);

// FIN 212 — Absolute Virtue
// Audit: unsupported — Needs continuously derived player protection while this creature remains in play; player protection is currently read only from resolved effects with durations, not battlefield static abilities.
pub(in crate::card::sets) static ABSOLUTE_VIRTUE: CardRecord = CardRecord::new(
    "Absolute Virtue",
    "aa192912-c9ee-403f-8a46-a338c9edb4b9",
    "Toni Infante",
    CardRules::unsupported(),
);

// FIN 213 — Balthier and Fran
// Audit: unsupported — Needs history identifying which Vehicle this creature crewed this turn and whether the attack is in the turn's first combat phase.
pub(in crate::card::sets) static BALTHIER_AND_FRAN: CardRecord = CardRecord::new(
    "Balthier and Fran",
    "afcaed7d-7ea3-4f2a-a7f5-ee3315226369",
    "Arif Wijaya",
    CardRules::unsupported(),
);

// FIN 214 — Black Waltz No. 3
pub(in crate::card::sets) static BLACK_WALTZ_NO_3: CardRecord = CardRecord::new(
    "Black Waltz No. 3",
    "fe86e41b-b0f6-4aa1-8827-c095c721f304",
    "Lordigan",
    CardRules::new_creature(mana_cost!("{2}{B}{R}"), &["Wizard"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            abilities::deathtouch(),
            AbilityDef::triggered(
                "Whenever you cast a noncreature spell, Black Waltz No. 3 \
                 deals 2 damage to each opponent.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(2)),
            ),
        ]),
);

// FIN 215 — Choco, Seeker of Paradise
// Audit: unsupported — Needs a batched attack trigger to retain the number of matching Birds declared as attackers; its current event amount is the complete declaration size, including non-Birds, and a later battlefield count loses attackers that have left.
pub(in crate::card::sets) static CHOCO_SEEKER_OF_PARADISE: CardRecord = CardRecord::new(
    "Choco, Seeker of Paradise",
    "409c305a-52dc-4538-8e72-efcd568eaf49",
    "Miho Midorikawa",
    CardRules::unsupported(),
);

// FIN 216 — Cid, Timeless Artificer
// Audit: unsupported — Needs a deck-construction permission overriding the four-copy limit for this exact card name; the deck-construction vocabulary currently has commander and companion clauses but no unrestricted copy allowance.
pub(in crate::card::sets) static CID_TIMELESS_ARTIFICER: CardRecord = CardRecord::new(
    "Cid, Timeless Artificer",
    "7fb99393-d2b6-40a6-8de7-317efdc4c50b",
    "Lius Lasahido",
    CardRules::unsupported(),
);

// FIN 217 — Cloud of Darkness
pub(in crate::card::sets) static CLOUD_OF_DARKNESS: CardRecord = CardRecord::new(
    "Cloud of Darkness",
    "a292094a-674a-401f-8776-ba5ebe57c946",
    "Fariba Khamseh",
    CardRules::new_creature(mana_cost!("{2}{B}{G}{G}"), &["Avatar"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            abilities::enters_trigger_with_targets(
                "Particle Beam — When Cloud of Darkness enters, target \
                 creature an opponent controls gets -X/-X until end of turn, \
                 where X is the number of permanent cards in your graveyard.",
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
                        ValueDef::Scaled(&ScaledValueDef {
                            value: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                ObjectPredicateDef::AnyOf(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                    ObjectPredicateDef::HasType(CardType::Enchantment),
                                    ObjectPredicateDef::HasType(CardType::Land),
                                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                                ]),
                                &[ZoneKind::Graveyard],
                                PlayerRelation::You,
                            )),
                            factor: -1,
                        }),
                        ValueDef::Scaled(&ScaledValueDef {
                            value: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                ObjectPredicateDef::AnyOf(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                    ObjectPredicateDef::HasType(CardType::Enchantment),
                                    ObjectPredicateDef::HasType(CardType::Land),
                                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                                ]),
                                &[ZoneKind::Graveyard],
                                PlayerRelation::You,
                            )),
                            factor: -1,
                        }),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// FIN 218 — Emet-Selch, Unsundered // Hades, Sorcerer of Eld
pub(in crate::card::sets) static EMET_SELCH_UNSUNDERED: CardRecord = CardRecord::new_dfc(
    "Emet-Selch, Unsundered // Hades, Sorcerer of Eld",
    "75cf4eb8-33e7-4dfc-b890-a7e3b5c1b9d5",
    "Néstor Ossandón Leal",
    &[
        (
            "Emet-Selch, Unsundered",
            CardRules::new_creature(mana_cost!("{1}{U}{B}"), &["Elder", "Wizard"], 2, 4)
                .with_supertype(CardSupertype::Legendary)
                .with_abilities(&[
                    abilities::vigilance(),
                    AbilityDef::triggered(
                        "Whenever Emet-Selch enters or attacks, draw a card, then \
                         discard a card.",
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
                                then: None,
                            },
                        ]),
                    ),
                    AbilityDef::triggered_if(
                        "At the beginning of your upkeep, if there are fourteen or \
                         more cards in your graveyard, you may transform Emet-Selch.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::Upkeep,
                            player: PlayerRelation::You,
                        },
                        &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                            left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                ObjectPredicateDef::Any,
                                &[ZoneKind::Graveyard],
                                PlayerRelation::You,
                            )),
                            comparison: ComparisonDef::GreaterOrEqual,
                            right: ValueDef::Constant(14),
                        }),
                        EffectDef::May {
                            player: EffectRecipientDef::Controller,
                            effect: &EffectDef::Transform {
                                object: EffectRecipientDef::Source,
                            },
                        },
                    ),
                ]),
        ),
        (
            "Hades, Sorcerer of Eld",
            CardRules::new_creature_without_mana_cost(&["Avatar"], 6, 6)
                .with_supertype(CardSupertype::Legendary)
                .printed_colors(&[ManaColor::Black, ManaColor::Blue])
                .with_abilities(&[
                    abilities::vigilance(),
                    AbilityDef::static_ability(
                        "Echo of the Lost — During your turn, you may play cards from \
                         your graveyard.",
                        EffectDef::IfCondition {
                            condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                            then: &EffectDef::StaticApply {
                                recipient: EffectRecipientDef::Controller,
                                effect: AppliedEffectDef::Rule(
                                    AppliedRuleDef::MayPlayFromGraveyard(
                                        GraveyardPlayPermissionDef::unlimited(
                                            PlayRestrictionDef::new(
                                                PlayActionMatcherDef::Any,
                                                ObjectPredicateDef::Any,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                    ),
                    AbilityDef::replacement_for(
                        "If a card or token would be put into your graveyard from \
                         anywhere, exile it instead.",
                        ReplacementEventDef::AnyObjectWouldMove {
                            object: ObjectPredicateDef::OwnedBy(PlayerRelation::You),
                            to: ZoneKind::Graveyard,
                        },
                        ReplacementEffectDef::MoveToZone(ZoneKind::Exile),
                    ),
                ]),
        ),
    ],
);

// FIN 219 — The Emperor of Palamecia // The Lord Master of Hell
// Audit: unsupported — Needs total mana spent recorded with a spell-cast event and readable by its triggered ability, including additional costs and cost reductions; the current cast-event values expose mana value and colors spent, not the amount paid.
pub(in crate::card::sets) static THE_EMPEROR_OF_PALAMECIA: CardRecord = CardRecord::new(
    "The Emperor of Palamecia // The Lord Master of Hell",
    "3d75e8fd-6139-4b10-9ce3-195b47d72e0c",
    "Heonhwa",
    CardRules::unsupported(),
);

// FIN 220 — Exdeath, Void Warlock // Neo Exdeath, Dimension's End
pub(in crate::card::sets) static EXDEATH_VOID_WARLOCK: CardRecord = CardRecord::new_dfc(
    "Exdeath, Void Warlock // Neo Exdeath, Dimension's End",
    "1b4bab87-4000-461d-8b58-d34928fee305",
    "Jessica Fong",
    &[
        (
            "Exdeath, Void Warlock",
            CardRules::new_creature(mana_cost!("{1}{B}{G}"), &["Spirit", "Warlock"], 3, 3)
                .with_supertype(CardSupertype::Legendary)
                .with_abilities(&[
                    abilities::enters_trigger(
                        "When Exdeath enters, you gain 3 life.",
                        EffectDef::GainLife {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(3),
                        },
                    ),
                    AbilityDef::triggered_if(
                        "At the beginning of your end step, if there are six or more \
                         permanent cards in your graveyard, transform Exdeath.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::End,
                            player: PlayerRelation::You,
                        },
                        &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                            left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                ObjectPredicateDef::AnyOf(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                    ObjectPredicateDef::HasType(CardType::Enchantment),
                                    ObjectPredicateDef::HasType(CardType::Land),
                                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                                ]),
                                &[ZoneKind::Graveyard],
                                PlayerRelation::You,
                            )),
                            comparison: ComparisonDef::GreaterOrEqual,
                            right: ValueDef::Constant(6),
                        }),
                        EffectDef::Transform {
                            object: EffectRecipientDef::Source,
                        },
                    ),
                ]),
        ),
        (
            "Neo Exdeath, Dimension's End",
            CardRules::new_creature_without_mana_cost(&["Spirit", "Avatar"], 0, 3)
                .with_supertype(CardSupertype::Legendary)
                .printed_colors(&[ManaColor::Black, ManaColor::Green])
                .with_abilities(&[
                    abilities::trample(),
                    AbilityDef::static_ability(
                        "Neo Exdeath's power is equal to the number of permanent cards \
                         in your graveyard.",
                        EffectDef::StaticApply {
                            recipient: EffectRecipientDef::Source,
                            effect: AppliedEffectDef::define_power(ValueDef::CountMatchingObjects(
                                &ObjectQueryDef::matching(
                                    ObjectPredicateDef::AnyOf(&[
                                        ObjectPredicateDef::HasType(CardType::Creature),
                                        ObjectPredicateDef::HasType(CardType::Artifact),
                                        ObjectPredicateDef::HasType(CardType::Enchantment),
                                        ObjectPredicateDef::HasType(CardType::Land),
                                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                                    ]),
                                    &[ZoneKind::Graveyard],
                                    PlayerRelation::You,
                                ),
                            )),
                        },
                    ),
                ]),
        ),
    ],
);

// FIN 221 — Garland, Knight of Cornelia // Chaos, the Endless
// Audit: unsupported — Needs direct graveyard-to-battlefield entry on the transformed face; current transformed returns follow linked exiles, and transforming after entry would expose the wrong entering characteristics and triggers.
pub(in crate::card::sets) static GARLAND_KNIGHT_OF_CORNELIA: CardRecord = CardRecord::new(
    "Garland, Knight of Cornelia // Chaos, the Endless",
    "dd463dbe-5f2c-4d4f-86f8-ad8ff407af62",
    "Billy Christian",
    CardRules::unsupported(),
);

// FIN 222 — Garnet, Princess of Alexandria
// Audit: unsupported — Needs the number of lore counters actually removed from the selected Sagas as an effect result; counting selected Sagas is wrong when removal is prevented or no counter is removed.
pub(in crate::card::sets) static GARNET_PRINCESS_OF_ALEXANDRIA: CardRecord = CardRecord::new(
    "Garnet, Princess of Alexandria",
    "b883df14-8d7b-4f6a-9a6a-2f71f5b6ddda",
    "Daniel Correia",
    CardRules::unsupported(),
);

// FIN 223 — Giott, King of the Dwarves
pub(in crate::card::sets) static GIOTT_KING_OF_THE_DWARVES: CardRecord = CardRecord::new(
    "Giott, King of the Dwarves",
    "6a7784de-a10d-4ce6-98a5-aaf3e85773b6",
    "Ben Wootten",
    CardRules::new_creature(mana_cost!("{R}{W}"), &["Dwarf", "Noble"], 1, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::double_strike(),
            AbilityDef::triggered(
                "Whenever Giott or another Dwarf you control enters and \
                 whenever an Equipment you control enters, you may discard a \
                 card. If you do, draw a card.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Source,
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dwarf")),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Equipment")),
                            ]),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ]),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::PayOr(PayOrDef::optional(
                    &[CostDef::discard(ObjectPredicateDef::Any)],
                    &abilities::draw_cards(ValueDef::Constant(1)),
                )),
            ),
        ]),
);

// FIN 224 — Gladiolus Amicitia
pub(in crate::card::sets) static GLADIOLUS_AMICITIA: CardRecord = CardRecord::new(
    "Gladiolus Amicitia",
    "442957fc-045d-4db6-b82a-445f172d23e4",
    "Gal Or",
    CardRules::new_creature(mana_cost!("{4}{R}{G}"), &["Human", "Warrior"], 6, 6)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When Gladiolus Amicitia enters, search your library for a \
                 land card, put it onto the battlefield tapped, then shuffle.",
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::HasType(CardType::Land),
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
            AbilityDef::triggered_with_targets(
                "Landfall — Whenever a land you control enters, another target \
                 creature you control gets +2/+2 and gains trample until end \
                 of turn.",
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
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&abilities::trample()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// FIN 225 — Golbez, Crystal Collector
pub(in crate::card::sets) static GOLBEZ_CRYSTAL_COLLECTOR: CardRecord = CardRecord::new(
    "Golbez, Crystal Collector",
    "849f5716-7211-4e93-a220-f88d49f937f4",
    "Bachzim",
    CardRules::new_creature(mana_cost!("{U}{B}"), &["Human", "Wizard"], 1, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever an artifact you control enters, surveil 1.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                abilities::surveil(ValueDef::Constant(1)),
            ),
            AbilityDef::triggered_if_with_targets(
                "At the beginning of your end step, if you control four or \
                 more artifacts, return target creature card from your \
                 graveyard to your hand. Then if you control eight or more \
                 artifacts, each opponent loses life equal to that card's \
                 power.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::You,
                },
                &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    comparison: ComparisonDef::GreaterOrEqual,
                    right: ValueDef::Constant(4),
                }),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::You),
                    },
                )],
                EffectDef::Sequence(&[
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                    EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                            left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            )),
                            comparison: ComparisonDef::GreaterOrEqual,
                            right: ValueDef::Constant(8),
                        }),
                        then: &EffectDef::LoseLife {
                            recipient: EffectRecipientDef::Opponent,
                            amount: ValueDef::TargetPower(TargetIndex::PRIMARY),
                        },
                    },
                ]),
            ),
        ]),
);

// FIN 226 — Hope Estheim
pub(in crate::card::sets) static HOPE_ESTHEIM: CardRecord = CardRecord::new(
    "Hope Estheim",
    "fbdb68cc-5516-481a-94c5-59f6c69b8a17",
    "Fariba Khamseh",
    CardRules::new_creature(mana_cost!("{W}{U}"), &["Human", "Wizard"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::lifelink(),
            AbilityDef::triggered(
                "At the beginning of your end step, each opponent mills X \
                 cards, where X is the amount of life you gained this turn.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::You,
                },
                EffectDef::Mill {
                    player: EffectRecipientDef::Opponent,
                    amount: ValueDef::LifeGainedThisTurn(PlayerRelation::You),
                },
            ),
        ]),
);

// FIN 227 — Ignis Scientia
pub(in crate::card::sets) static IGNIS_SCIENTIA: CardRecord = CardRecord::new(
    "Ignis Scientia",
    "ab4f9721-5b2c-4371-98a5-3f6714265e57",
    "Mingchen Shen",
    CardRules::new_creature(mana_cost!("{1}{G}{U}"), &["Human", "Advisor"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When Ignis Scientia enters, look at the top six cards of your \
                 library. You may put a land card from among them onto the \
                 battlefield tapped. Put the rest on the bottom of your \
                 library in a random order.",
                EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                    source: ObjectCollectionSourceDef::TopCards {
                        player: PlayerRefDef::EffectController,
                        count: ValueDef::Constant(6),
                    },
                    actor: PlayerRefDef::EffectController,
                    inspection: CollectionInspectionDef::Look,
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    minimum: 0,
                    maximum: 1,
                    chosen: crate::Binding!("chosen"),
                    remainder: crate::Binding!("rest"),
                    then: &EffectDef::Sequence(&[
                        EffectDef::WithBattlefieldArrival {
                            effect: &EffectDef::move_to_zone(
                                EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("chosen"),
                                )),
                                ZoneKind::Battlefield,
                                ZonePlacement::Top,
                            ),
                            arrival: BattlefieldArrivalDef {
                                modifications: &[BattlefieldEntryModificationDef::Tapped],
                                ..BattlefieldArrivalDef::DEFAULT
                            },
                        },
                        EffectDef::RandomizeObjectOrder(RandomizeObjectOrderDef {
                            input: ObjectSetDef::Binding(crate::Binding!("rest")),
                            randomized: crate::Binding!("random_bottom"),
                            then: &EffectDef::move_to_zone(
                                EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("random_bottom"),
                                )),
                                ZoneKind::Library,
                                ZonePlacement::Bottom,
                            ),
                        }),
                    ]),
                }),
            ),
            AbilityDef::activated_with_targets(
                "I've Come Up with a New Recipe! — {1}{G}{U}, {T}: Exile \
                 target card from a graveyard. If a creature card was exiled \
                 this way, create a Food token.",
                &[CostDef::Mana(mana_cost!("{1}{G}{U}")), CostDef::TapSource],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Any,
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::Any),
                    },
                )],
                EffectDef::WithZoneMoveResult {
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Exile,
                        ZonePlacement::Top,
                    ),
                    binding: crate::Binding!("exiled"),
                    then: &EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ObjectSetCount(
                            &ObjectSetCountConditionDef {
                                objects: &ObjectSetDef::ZoneChangeSuccessorsOfBinding(
                                    crate::Binding!("exiled"),
                                ),
                                predicate: ObjectSetPredicateDef::contains(
                                    &ObjectPredicateDef::HasType(CardType::Creature),
                                ),
                            },
                        ),
                        then: &EffectDef::CreateToken(
                            CreateTokenDef::new(TokenDef::Literal(FOOD_TOKEN))
                                .with_count(ValueDef::Constant(1)),
                        ),
                    },
                },
            ),
        ]),
);

// FIN 228 — Jenova, Ancient Calamity
pub(in crate::card::sets) static JENOVA_ANCIENT_CALAMITY: CardRecord = CardRecord::new(
    "Jenova, Ancient Calamity",
    "534f98ee-7bc2-44d6-b49f-f57c051807d5",
    "Ignatius Budi",
    CardRules::new_creature(mana_cost!("{2}{B}{G}"), &["Alien"], 1, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered_with_targets(
                "At the beginning of combat on your turn, put a number of \
                 +1/+1 counters equal to Jenova's power on up to one other \
                 target creature. That creature becomes a Mutant in addition \
                 to its other types.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::BeginningOfCombat,
                    player: PlayerRelation::You,
                },
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    1,
                )],
                EffectDef::Sequence(&[
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::SourcePower,
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&[
                            "Mutant",
                        ])),
                        duration: ResolvedEffectDurationDef::Permanent,
                    },
                ]),
            ),
            AbilityDef::triggered_if(
                "Whenever a Mutant you control dies during your turn, you draw \
                 cards equal to its power.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mutant")),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                abilities::draw_cards(ValueDef::TriggeringObjectPower),
            ),
        ]),
);

// FIN 229 — Joshua, Phoenix's Dominant // Phoenix, Warden of Fire
// Audit: unsupported — Needs one printed Saga ability to represent several chapter numbers and to remain identifiable to the Saga final-chapter rules; the current Saga authoring and recognition path represents exactly one chapter per ability.
pub(in crate::card::sets) static JOSHUA_PHOENIX_S_DOMINANT: CardRecord = CardRecord::new(
    "Joshua, Phoenix's Dominant // Phoenix, Warden of Fire",
    "457fdbb9-5439-460f-8e37-176f8919362c",
    "Lius Lasahido",
    CardRules::unsupported(),
);

// FIN 230 — Judge Magister Gabranth
pub(in crate::card::sets) static JUDGE_MAGISTER_GABRANTH: CardRecord = CardRecord::new(
    "Judge Magister Gabranth",
    "f9e64cb6-48f7-41d3-99e7-4b0bc3b33fd7",
    "Josu Hernaiz",
    CardRules::new_creature(mana_cost!("{W}{B}"), &["Human", "Advisor", "Knight"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::menace(),
            AbilityDef::triggered(
                "Whenever another creature or artifact you control dies, put a \
                 +1/+1 counter on Judge Magister Gabranth.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Creature),
                            ]),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// FIN 231 — Kefka, Court Mage // Kefka, Ruler of Ruin
// Audit: unsupported — Needs a life-loss event carrying the amount actually lost, including life payments and life-total changes, for the transformed face; damage and life-gain events do not cover life loss.
pub(in crate::card::sets) static KEFKA_COURT_MAGE: CardRecord = CardRecord::new(
    "Kefka, Court Mage // Kefka, Ruler of Ruin",
    "8fcf3fbb-1ddd-437e-81c1-f5a3133f5ee8",
    "Xui Ton",
    CardRules::unsupported(),
);

// FIN 232 — Kuja, Genome Sorcerer // Trance Kuja, Fate Defied
// Audit: unsupported — Needs a prospective damage multiplier filtered to Wizard sources you control; the shared damage rules provide prevention, redirection, and limits, but no multiplication replacement.
pub(in crate::card::sets) static KUJA_GENOME_SORCERER: CardRecord = CardRecord::new(
    "Kuja, Genome Sorcerer // Trance Kuja, Fate Defied",
    "008782d2-72b0-4554-b1ce-2db99969a4d8",
    "Joshua Raphael",
    CardRules::unsupported(),
);

// FIN 232† — Kuja, Genome Sorcerer // Trance Kuja, Fate Defied (alternate printing)
const KUJA_GENOME_SORCERER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KUJA_GENOME_SORCERER,
    1,
    "903192c7-f4ee-4872-aa61-3f8bdf2ab5c6",
    "Joshua Raphael",
);

// FIN 233 — Lightning, Army of One
// Audit: unsupported — Needs a temporary prospective damage multiplier covering a particular damaged player and that player's permanents; current damage rules do not represent multiplication replacements.
pub(in crate::card::sets) static LIGHTNING_ARMY_OF_ONE: CardRecord = CardRecord::new(
    "Lightning, Army of One",
    "1103da9c-300c-406b-997d-9e5bb7cd02d6",
    "Shiyu",
    CardRules::unsupported(),
);

// FIN 234 — Locke Cole
pub(in crate::card::sets) static LOCKE_COLE: CardRecord = CardRecord::new(
    "Locke Cole",
    "572feb8c-6976-40a8-8a34-b4db836cca56",
    "AKAGI",
    CardRules::new_creature(mana_cost!("{1}{U}{B}"), &["Human", "Rogue"], 2, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::deathtouch(),
            abilities::lifelink(),
            AbilityDef::triggered(
                "Whenever Locke Cole deals combat damage to a player, draw a \
                 card, then discard a card.",
                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
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

// FIN 235 — Noctis, Prince of Lucis
// Audit: unsupported — Needs a graveyard casting permission with a mandatory additional life cost that composes with every chosen casting cost and gives that cast a finality-counter arrival; a granted alternative cost is not equivalent.
pub(in crate::card::sets) static NOCTIS_PRINCE_OF_LUCIS: CardRecord = CardRecord::new(
    "Noctis, Prince of Lucis",
    "1881a66b-956d-4bab-b578-5b2d3407c972",
    "Jeremy Chong",
    CardRules::unsupported(),
);

// FIN 236 — Omega, Heartless Evolution
pub(in crate::card::sets) static OMEGA_HEARTLESS_EVOLUTION: CardRecord = CardRecord::new(
    "Omega, Heartless Evolution",
    "9a8eb7e6-0c0b-42d0-aa90-2d3d29bc15aa",
    "Josu Solano",
    CardRules::new_artifact_creature(mana_cost!("{5}{G}{U}"), &["Robot"], 8, 8)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[abilities::enters_trigger_with_targets(
            "Wave Cannon — When Omega enters, for each opponent, tap up to \
             one target nonland permanent that opponent controls. Put X \
             stun counters on each of those permanents and you gain X \
             life, where X is the number of nonbasic lands you control. \
             (If a permanent with a stun counter would become untapped, \
             remove one from it instead.)",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
                1,
            )],
            EffectDef::Sequence(&[
                EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::Stun,
                    amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(
                                CardSupertype::Basic,
                            )),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(
                                CardSupertype::Basic,
                            )),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                },
            ]),
        )]),
);

// FIN 237 — Rinoa Heartilly
pub(in crate::card::sets) static RINOA_HEARTILLY: CardRecord = CardRecord::new(
    "Rinoa Heartilly",
    "ba79d293-bf42-48b6-a868-5249f4beeb76",
    "Francesca Resta",
    CardRules::new_creature(
        mana_cost!("{3}{G}{W}"),
        &["Human", "Rebel", "Warlock"],
        4,
        4,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::enters_trigger(
            "When Rinoa Heartilly enters, create Angelo, a legendary 1/1 \
             green and white Dog creature token.",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(
                    &["Dog"],
                    &[ManaColor::Green, ManaColor::White],
                    1,
                    1,
                )
                .with_name("Angelo")
                .with_supertype(CardSupertype::Legendary),
            ))),
        ),
        AbilityDef::triggered_with_targets(
            "Angelo Cannon — Whenever Rinoa Heartilly attacks, another \
             target creature you control gets +1/+1 until end of turn for \
             each creature you control.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
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
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// FIN 238 — Rufus Shinra
pub(in crate::card::sets) static RUFUS_SHINRA: CardRecord = CardRecord::new(
    "Rufus Shinra",
    "f5fff00b-c9a0-4e90-abc0-349f8716c885",
    "Ittoku",
    CardRules::new_creature(mana_cost!("{1}{W}{B}"), &["Human", "Noble"], 2, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::triggered_if(
            "Whenever Rufus Shinra attacks, if you don't control a \
             creature named Darkstar, create Darkstar, a legendary 2/2 \
             white and black Dog creature token.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::NameEquals(CardNameDef::Literal("Darkstar")),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
                comparison: ComparisonDef::Equal,
                right: ValueDef::Constant(0),
            }),
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(
                    &["Dog"],
                    &[ManaColor::White, ManaColor::Black],
                    2,
                    2,
                )
                .with_name("Darkstar")
                .with_supertype(CardSupertype::Legendary),
            ))),
        )]),
);

// FIN 239 — Rydia, Summoner of Mist
// Audit: unsupported — Needs a haste grant to the returned creature through a graveyard-targeted zone-move continuation; the production grant boundary treats these target-derived recipients as nonbattlefield and rejects the grant.
pub(in crate::card::sets) static RYDIA_SUMMONER_OF_MIST: CardRecord = CardRecord::new(
    "Rydia, Summoner of Mist",
    "99450143-6ab5-463d-9e04-e8e6703a8b92",
    "Yumi Yaoshida",
    CardRules::unsupported(),
);

// FIN 240 — Serah Farron // Crystallized Serah
// Audit: unsupported — Needs spell-cost evaluation to recognize the first legendary creature spell of the turn from persistent player cast history; a source-local use limit would be wrong after Serah enters or transforms.
pub(in crate::card::sets) static SERAH_FARRON: CardRecord = CardRecord::new(
    "Serah Farron // Crystallized Serah",
    "62fa74c0-43ae-445c-8039-ca9d00e9709a",
    "Carissa Susilo",
    CardRules::unsupported(),
);

// FIN 241 — Shantotto, Tactician Magician
// Audit: unsupported — Needs total mana spent recorded with a spell-cast event and readable by its triggered ability, including additional costs and cost reductions; the current cast-event values expose mana value and colors spent, not the amount paid.
pub(in crate::card::sets) static SHANTOTTO_TACTICIAN_MAGICIAN: CardRecord = CardRecord::new(
    "Shantotto, Tactician Magician",
    "eff984b2-6ea9-4471-91c5-99c47f87f10b",
    "Joshua Raphael",
    CardRules::unsupported(),
);

// FIN 242 — Sin, Spira's Punishment
// Audit: unsupported — Needs a loop that randomly exiles a permanent card from the graveyard, creates its tapped copy, and repeats only for a land; existing recursive mill loops do not perform random graveyard selection.
pub(in crate::card::sets) static SIN_SPIRA_S_PUNISHMENT: CardRecord = CardRecord::new(
    "Sin, Spira's Punishment",
    "659be746-bd31-4a70-8cec-7798da78b0b5",
    "John Tedrick",
    CardRules::unsupported(),
);

// FIN 243 — Squall, SeeD Mercenary
pub(in crate::card::sets) static SQUALL_SEED_MERCENARY: CardRecord = CardRecord::new(
    "Squall, SeeD Mercenary",
    "cc4e5234-fb41-48f7-91f4-039710542bc3",
    "Yuu Fujiki",
    CardRules::new_creature(
        mana_cost!("{2}{W}{B}"),
        &["Human", "Knight", "Mercenary"],
        3,
        4,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        AbilityDef::triggered(
            "Rough Divide — Whenever a creature you control attacks alone, \
             it gains double strike until end of turn.",
            TriggerEventDef::attacks_in_declaration(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                1,
                Some(1),
            ),
            EffectDef::Apply {
                recipient: EffectRecipientDef::TriggeringObject,
                effect: AppliedEffectDef::add_ability(&abilities::double_strike()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::triggered_with_targets(
            "Whenever Squall deals combat damage to a player, return \
             target permanent card with mana value 3 or less from your \
             graveyard to the battlefield.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                        ]),
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
    ]),
);

// FIN 244 — Tellah, Great Sage
// Audit: unsupported — Needs total mana spent recorded with a spell-cast event and readable by its triggered ability, including additional costs and cost reductions; the current cast-event values expose mana value and colors spent, not the amount paid.
pub(in crate::card::sets) static TELLAH_GREAT_SAGE: CardRecord = CardRecord::new(
    "Tellah, Great Sage",
    "a67793ef-ef80-4434-9c54-e3fd8a270bbe",
    "Yumi Yaoshida",
    CardRules::unsupported(),
);

// FIN 245 — Terra, Magical Adept // Esper Terra
// Audit: unsupported — Needs one printed Saga ability to represent several chapter numbers and to remain identifiable to the Saga final-chapter rules; the current Saga authoring and recognition path represents exactly one chapter per ability.
pub(in crate::card::sets) static TERRA_MAGICAL_ADEPT: CardRecord = CardRecord::new(
    "Terra, Magical Adept // Esper Terra",
    "fbd447aa-588d-4c4d-925e-a7d3bdf6a65c",
    "Clare Wong",
    CardRules::unsupported(),
);

// FIN 246 — Tidus, Blitzball Star
pub(in crate::card::sets) static TIDUS_BLITZBALL_STAR: CardRecord = CardRecord::new(
    "Tidus, Blitzball Star",
    "aa851d68-a7a4-48c0-9cd7-d3d2e079f3a1",
    "Nakamura8",
    CardRules::new_creature(mana_cost!("{1}{W}{U}"), &["Human", "Warrior"], 2, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever an artifact you control enters, put a +1/+1 counter \
                 on Tidus.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::triggered_with_targets(
                "Whenever Tidus attacks, tap target creature an opponent controls.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
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
        ]),
);

// FIN 247 — Ultimecia, Time Sorceress // Ultimecia, Omnipotent
pub(in crate::card::sets) static ULTIMECIA_TIME_SORCERESS: CardRecord = CardRecord::new_dfc(
    "Ultimecia, Time Sorceress // Ultimecia, Omnipotent",
    "2d6a2b68-5407-464e-a335-7866fd969c30",
    "Mikio Masuda",
    &[
        (
            "Ultimecia, Time Sorceress",
            CardRules::new_creature(mana_cost!("{3}{U}{B}"), &["Human", "Warlock"], 4, 5)
                .with_supertype(CardSupertype::Legendary)
                .with_abilities(&[
                    AbilityDef::triggered(
                        "Whenever Ultimecia enters or attacks, surveil 2. (Look at the \
                         top two cards of your library, then put any number of them \
                         into your graveyard and the rest on top of your library in \
                         any order.)",
                        TriggerEventDef::AnyOf(&[
                            TriggerEventDef::zone_changed(
                                ObjectPredicateDef::Source,
                                None,
                                Some(ZoneKind::Battlefield),
                            ),
                            TriggerEventDef::attacks(ObjectPredicateDef::Source),
                        ]),
                        abilities::surveil(ValueDef::Constant(2)),
                    ),
                    AbilityDef::triggered(
                        "At the beginning of your end step, you may pay \
                         {4}{U}{U}{B}{B} and exile eight cards from your graveyard. If \
                         you do, transform Ultimecia.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::End,
                            player: PlayerRelation::You,
                        },
                        EffectDef::PayOr(PayOrDef::optional(
                            &[
                                CostDef::Mana(mana_cost!("{4}{U}{U}{B}{B}")),
                                CostDef::Perform(
                                    &crate::card::actions::choose_exile_from_graveyard(8),
                                ),
                            ],
                            &EffectDef::Transform {
                                object: EffectRecipientDef::Source,
                            },
                        )),
                    ),
                ]),
        ),
        (
            "Ultimecia, Omnipotent",
            CardRules::new_creature_without_mana_cost(&["Nightmare", "Warlock"], 7, 7)
                .with_supertype(CardSupertype::Legendary)
                .printed_colors(&[ManaColor::Black, ManaColor::Blue])
                .with_abilities(&[
                    abilities::menace(),
                    AbilityDef::triggered(
                        "Time Compression — When this creature transforms into \
                         Ultimecia, Omnipotent, take an extra turn after this one.",
                        TriggerEventDef::transforms(ObjectPredicateDef::Source),
                        EffectDef::TakeExtraTurn {
                            player: EffectRecipientDef::Controller,
                        },
                    ),
                ]),
        ),
    ],
);

// FIN 248 — Vivi Ornitier
pub(in crate::card::sets) static VIVI_ORNITIER: CardRecord = CardRecord::new(
    "Vivi Ornitier",
    "ecc1027a-8c07-44a0-bdde-fa2844cff694",
    "Toni Infante",
CardRules::new_creature(mana_cost!("{1}{U}{R}"), &["Wizard"], 0, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_mana(
                "{0}: Add X mana in any combination of {U} and/or {R}, where X is this creature's power. Activate only during your turn and only once each turn.",
                &[CostDef::Mana(mana_cost!("{0}"))],
                // "Add X mana in any combination of {U} and/or {R}" divides one amount
                // across two types, so the runtime offers the ability once per division.
                // Vivi enters with no power at all, so the first activation worth making
                // comes after a noncreature spell has grown it.
                EffectDef::AddMana(AddManaEffectDef::combination(&[ManaColor::Blue, ManaColor::Red], 0).with_variable_amount(ValueDef::SourcePower)),
            )
            .with_activation_timing(ActivationTimingDef::YourTurn)
            .activations_each_turn(1),
            AbilityDef::triggered(
                "Whenever you cast a noncreature spell, put a +1/+1 counter on this creature and it deals 1 damage to each opponent.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::NoncreatureSpell,
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                // The counter and the damage are one clause, and the counter comes first --
                // so a Vivi that has just been cast at is already bigger by the time its own
                // mana ability is next offered.
                EffectDef::Sequence(&[
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(1)),
                ]),
            ),
        ]),
);

// FIN 249 — The Wandering Minstrel
// Audit: unsupported — Needs an entry replacement making lands enter untapped, including interaction and ordering with other tapped-entry replacements; a triggered untap happens after entry and is not equivalent.
pub(in crate::card::sets) static THE_WANDERING_MINSTREL: CardRecord = CardRecord::new(
    "The Wandering Minstrel",
    "77bc419d-ff69-4e7c-afe6-faca383a5ed7",
    "Thanh Tuấn",
    CardRules::unsupported(),
);

// FIN 250 — Yuna, Hope of Spira
pub(in crate::card::sets) static YUNA_HOPE_OF_SPIRA: CardRecord = CardRecord::new(
    "Yuna, Hope of Spira",
    "35b613ad-86f0-431b-af93-147d21041fde",
    "NINNIN",
    CardRules::new_creature(mana_cost!("{3}{G}{W}"), &["Human", "Cleric"], 3, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "During your turn, Yuna and enchantment creatures you control \
                 have trample, lifelink, and ward {2}.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                            ObjectQueryDef::matching(
                                ObjectPredicateDef::AnyOf(&[
                                    ObjectPredicateDef::Source,
                                    ObjectPredicateDef::All(&[
                                        ObjectPredicateDef::HasType(CardType::Creature),
                                        ObjectPredicateDef::HasType(CardType::Enchantment),
                                    ]),
                                ]),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            ),
                        )),
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::add_ability(&abilities::trample()),
                            AppliedEffectDef::add_ability(&abilities::lifelink()),
                            AppliedEffectDef::add_ability(&abilities::ward(
                                &[CostDef::Mana(mana_cost!("{2}"))],
                                "Ward {2}",
                            )),
                        ]),
                    },
                },
            ),
            AbilityDef::triggered_with_targets(
                "At the beginning of your end step, return up to one target \
                 enchantment card from your graveyard to the battlefield with \
                 a finality counter on it. (If a permanent with a finality \
                 counter on it would be put into a graveyard from the \
                 battlefield, exile it instead.)",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::You,
                },
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Enchantment),
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::You),
                    },
                    1,
                )],
                EffectDef::WithBattlefieldArrival {
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                    arrival: BattlefieldArrivalDef {
                        modifications: &[BattlefieldEntryModificationDef::AddCounters {
                            kind: CounterKind::Finality,
                            amount: 1,
                        }],
                        ..BattlefieldArrivalDef::DEFAULT
                    },
                },
            ),
        ]),
);

// FIN 251 — Zidane, Tantalus Thief
// Audit: unsupported — Needs a control-change event that records the former and new controllers of a permanent, including control effects expiring; the current trigger vocabulary has no such event.
pub(in crate::card::sets) static ZIDANE_TANTALUS_THIEF: CardRecord = CardRecord::new(
    "Zidane, Tantalus Thief",
    "e42c7d9d-8685-415b-8c5d-6ab2165863b9",
    "Eiji Kaneda",
    CardRules::unsupported(),
);

// FIN 252 — Adventurer's Airship
pub(in crate::card::sets) static ADVENTURER_S_AIRSHIP: CardRecord = CardRecord::new(
    "Adventurer's Airship",
    "0a1d6dcd-bd41-4f57-a35b-6613811fe4d4",
    "Racrufi",
    CardRules::new_vehicle(mana_cost!("{3}"), 3, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever this Vehicle attacks, draw a card, then discard a card.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
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
        abilities::crew(
            "Crew 2 (Tap any number of creatures you control with total \
             power 2 or more: This Vehicle becomes an artifact creature \
             until end of turn.)",
            2,
        ),
    ]),
);

// FIN 253 — Aettir and Priwen
// Audit: unsupported — Needs continuous power and toughness evaluation to read the controller's life total; life totals are supported by resolving effects and conditions, but not static characteristic values.
pub(in crate::card::sets) static AETTIR_AND_PRIWEN: CardRecord = CardRecord::new(
    "Aettir and Priwen",
    "038710ca-c756-4e66-a9de-278e676c9f5b",
    "Vilhelmas Banys",
    CardRules::unsupported(),
);

// FIN 254 — Blitzball
// Audit: unsupported — Needs this-turn combat-damage history filtered by both an opposing player recipient and a legendary creature source; current damage-history conditions cannot retain that source quality.
pub(in crate::card::sets) static BLITZBALL: CardRecord = CardRecord::new(
    "Blitzball",
    "92f4ad73-42bf-45c0-8bb6-0b44043c81ef",
    "Gas1",
    CardRules::unsupported(),
);

// FIN 255 — Buster Sword
// Audit: unsupported — Needs an immediate free-cast offer for a chosen card in hand; MayPlayWithoutPaying currently grants exile-only permission, and moving the chosen card to exile first would add an unprinted zone change.
pub(in crate::card::sets) static BUSTER_SWORD: CardRecord = CardRecord::new(
    "Buster Sword",
    "374d7383-a1a7-4eea-91f7-290180e14cc9",
    "Douzen",
    CardRules::unsupported(),
);

// FIN 256 — Elixir
pub(in crate::card::sets) static ELIXIR: CardRecord = CardRecord::new(
    "Elixir",
    "a4b05d37-df62-475c-8371-735ed2fa1b05",
    "Takeuchi Moto",
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        abilities::enters_tapped(CardType::Artifact),
        AbilityDef::activated(
            "{5}, {T}, Exile this artifact: Shuffle all nonland cards from \
             your graveyard into your library. You gain life equal to the \
             number of cards shuffled into your library this way.",
            &[
                CostDef::Mana(mana_cost!("{5}")),
                CostDef::TapSource,
                CostDef::ExileSource,
            ],
            EffectDef::WithZoneMoveResult {
                effect: &EffectDef::move_to_zone(
                    EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    ))),
                    ZoneKind::Library,
                    ZonePlacement::Top,
                ),
                binding: crate::Binding!("shuffled"),
                then: &EffectDef::Sequence(&[
                    EffectDef::ShuffleLibrary {
                        player: EffectRecipientDef::Controller,
                    },
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::CountObjects(
                            &ObjectSetDef::ZoneChangeSuccessorsOfBinding(crate::Binding!(
                                "shuffled"
                            )),
                        ),
                    },
                ]),
            },
        ),
    ]),
);

// FIN 257 — Excalibur II
pub(in crate::card::sets) static EXCALIBUR_II: CardRecord = CardRecord::new(
    "Excalibur II",
    "d42e5fed-67ac-46d7-a5d4-78f661f3e8b4",
    "Russell Dongjun Lu",
    CardRules::new_artifact(mana_cost!("{1}"))
        .with_subtypes(&["Equipment"])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever you gain life, put a charge counter on Excalibur II.",
                TriggerEventDef::LifeGained(PlayerRelation::You),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::named("charge"),
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1 for each charge counter on \
                 Excalibur II.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::CountersOnSource(CounterKind::named("charge")),
                        ValueDef::CountersOnSource(CounterKind::named("charge")),
                    ),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{3}"))], "Equip {3}"),
        ]),
);

// FIN 258 — Genji Glove
// Audit: unsupported — Needs a first-combat-phase predicate; scheduling an extra combat without it would trigger again on every additional attack.
pub(in crate::card::sets) static GENJI_GLOVE: CardRecord = CardRecord::new(
    "Genji Glove",
    "f724dde1-84b0-4e3b-a9b8-44cd22bb9f79",
    "Elizabeth Peiró",
    CardRules::unsupported(),
);

// FIN 259 — Instant Ramen
pub(in crate::card::sets) static INSTANT_RAMEN: CardRecord = CardRecord::new(
    "Instant Ramen",
    "ef7011f4-fc08-4b15-973d-d15357cbe744",
    "David Astruga",
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Food"])
        .with_abilities(&[
            abilities::flash(),
            abilities::enters_trigger(
                "When this artifact enters, draw a card.",
                abilities::draw_cards(ValueDef::Constant(1)),
            ),
            AbilityDef::activated(
                "{2}, {T}, Sacrifice this artifact: You gain 3 life.",
                &[
                    CostDef::Mana(mana_cost!("{2}")),
                    CostDef::TapSource,
                    CostDef::SacrificeSource,
                ],
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
            ),
        ]),
);

// FIN 260 — Iron Giant
pub(in crate::card::sets) static IRON_GIANT: CardRecord = CardRecord::new(
    "Iron Giant",
    "e48cf6d5-4d32-4b66-80be-3495ecd3e906",
    "John Tyler Christopher",
    CardRules::new_artifact_creature(mana_cost!("{7}"), &["Demon"], 6, 6).with_abilities(&[
        abilities::reach(),
        abilities::vigilance(),
        abilities::trample(),
    ]),
);

// FIN 261 — Lion Heart
pub(in crate::card::sets) static LION_HEART: CardRecord = CardRecord::new(
    "Lion Heart",
    "04d327a3-1699-4556-b681-a957671ad142",
    "Mushk Rizvi",
    CardRules::new_artifact(mana_cost!("{4}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::enters_trigger_with_targets(
                "When this Equipment enters, it deals 2 damage to any target.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::AnyTarget,
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(2),
                ),
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

// FIN 262 — Lunatic Pandora
pub(in crate::card::sets) static LUNATIC_PANDORA: CardRecord = CardRecord::new(
    "Lunatic Pandora",
    "d6e1e3e7-20d4-42cb-ad22-60356b9e8fdc",
    "Enora Mercier",
    CardRules::new_artifact(mana_cost!("{1}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated(
                "{2}, {T}: Surveil 1. (Look at the top card of your library. \
                 You may put it into your graveyard.)",
                &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
                abilities::surveil(ValueDef::Constant(1)),
            ),
            AbilityDef::activated_with_targets(
                "{6}, {T}, Sacrifice Lunatic Pandora: Destroy target nonland \
                 permanent.",
                &[
                    CostDef::Mana(mana_cost!("{6}")),
                    CostDef::TapSource,
                    CostDef::SacrificeSource,
                ],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            ),
        ]),
);

// FIN 263 — Magic Pot
pub(in crate::card::sets) static MAGIC_POT: CardRecord = CardRecord::new(
    "Magic Pot",
    "57d07ca0-5618-4a90-a605-ca14a193ce3b",
    "David Astruga",
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Goblin", "Construct"], 1, 4)
        .with_abilities(&[
            abilities::dies_trigger(
                "When this creature dies, create a Treasure token. (It's an \
                 artifact with \"{T}, Sacrifice this token: Add one mana of \
                 any color.\")",
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            ),
            AbilityDef::activated_with_targets(
                "{2}, {T}: Exile target card from a graveyard.",
                &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Any,
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::Any),
                    },
                )],
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
            ),
        ]),
);

// FIN 264 — The Masamune
// Audit: unsupported — Needs death-trigger duplication restricted to the equipped creature and its controller's emblems; the existing extra-death-trigger rule lacks that source and emblem scope.
pub(in crate::card::sets) static THE_MASAMUNE: CardRecord = CardRecord::new(
    "The Masamune",
    "fc408575-8ef7-4043-b6b7-b38cef7c97d1",
    "Masateru Ikeda",
    CardRules::unsupported(),
);

// FIN 265 — Monk's Fist
pub(in crate::card::sets) static MONK_S_FIST: CardRecord = CardRecord::new(
    "Monk's Fist",
    "995033f0-873d-4e46-b0c9-98ec8ef270ff",
    "Thanh Tuấn",
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::job_select(),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+0 and is a Monk in addition to its \
                 other types.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Monk"])),
                    ]),
                },
            ),
            abilities::equip(
                &[CostDef::Mana(mana_cost!("{2}"))],
                "Equip {2} ({2}: Attach to target creature you control. Equip \
                 only as a sorcery.)",
            ),
        ]),
);

// FIN 266 — PuPu UFO
pub(in crate::card::sets) static PUPU_UFO: CardRecord = CardRecord::new(
    "PuPu UFO",
    "989b52f7-d8a5-4488-9a5d-f14a1d48686d",
    "Racrufi",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Construct", "Alien"], 0, 4)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::activated(
                "{T}: You may put a land card from your hand onto the battlefield.",
                &[CostDef::TapSource],
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Hand],
                        PlayerRelation::You,
                    )),
                    exclude: None,
                    minimum: 0,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Private,
                    then: &EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "chosen"
                        ))),
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                }),
            ),
            AbilityDef::activated(
                "{3}: Until end of turn, this creature's base power becomes \
                 equal to the number of Towns you control.",
                &[CostDef::Mana(mana_cost!("{3}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::set_base_power(ValueDef::CountMatchingObjects(
                        &ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Town")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// FIN 267 — The Regalia
pub(in crate::card::sets) static THE_REGALIA: CardRecord = CardRecord::new(
    "The Regalia",
    "dc420e79-c483-474f-97cd-e9c6a636c306",
    "Jonas De Ro",
    CardRules::new_vehicle(mana_cost!("{4}"), 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::haste(),
            AbilityDef::triggered(
                "Whenever The Regalia attacks, reveal cards from the top of \
                 your library until you reveal a land card. Put that card onto \
                 the battlefield tapped and the rest on the bottom of your \
                 library in a random order.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                EffectDef::RevealAndClassifyCards(RevealAndClassifyCardsDef {
                    source: ObjectCollectionSourceDef::TopCardsThroughFirstMatching {
                        player: PlayerRefDef::EffectController,
                        object: ObjectPredicateDef::HasType(CardType::Land),
                    },
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    matching: crate::Binding!("land"),
                    remainder: crate::Binding!("rest"),
                    then: &EffectDef::Sequence(&[
                        EffectDef::WithBattlefieldArrival {
                            effect: &EffectDef::move_to_zone(
                                EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("land"),
                                )),
                                ZoneKind::Battlefield,
                                ZonePlacement::Top,
                            ),
                            arrival: BattlefieldArrivalDef {
                                modifications: &[BattlefieldEntryModificationDef::Tapped],
                                ..BattlefieldArrivalDef::DEFAULT
                            },
                        },
                        EffectDef::RandomizeObjectOrder(RandomizeObjectOrderDef {
                            input: ObjectSetDef::Binding(crate::Binding!("rest")),
                            randomized: crate::Binding!("random_bottom"),
                            then: &EffectDef::move_to_zone(
                                EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("random_bottom"),
                                )),
                                ZoneKind::Library,
                                ZonePlacement::Bottom,
                            ),
                        }),
                    ]),
                }),
            ),
            abilities::crew("Crew 1", 1),
        ]),
);

// FIN 268 — Relentless X-ATM092
pub(in crate::card::sets) static RELENTLESS_X_ATM092: CardRecord = CardRecord::new(
    "Relentless X-ATM092",
    "e09ee4c9-85ef-4d1e-864b-d659b8e8f51d",
    "Kevin Glint",
    CardRules::new_artifact_creature(mana_cost!("{6}"), &["Robot", "Spider"], 6, 5).with_abilities(
        &[
            AbilityDef::static_ability(
                "This creature can't be blocked except by three or more creatures.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(
                        BlockRestrictionDef::MinimumBlockers(3),
                    )),
                },
            ),
            AbilityDef::activated(
                "{8}: Return this card from your graveyard to the battlefield \
                 tapped with a finality counter on it. (If a creature with a \
                 finality counter on it would die, exile it instead.)",
                &[CostDef::Mana(mana_cost!("{8}"))],
                EffectDef::WithBattlefieldArrival {
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::Source,
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                    arrival: BattlefieldArrivalDef {
                        modifications: &[
                            BattlefieldEntryModificationDef::Tapped,
                            BattlefieldEntryModificationDef::AddCounters {
                                kind: CounterKind::Finality,
                                amount: 1,
                            },
                        ],
                        ..BattlefieldArrivalDef::DEFAULT
                    },
                },
            )
            .with_source_zones(&[ZoneKind::Graveyard]),
        ],
    ),
);

// FIN 269 — Ring of the Lucii
pub(in crate::card::sets) static RING_OF_THE_LUCII: CardRecord = CardRecord::new(
    "Ring of the Lucii",
    "75761f1e-9449-4c58-8265-8abac71dafc1",
    "Lorenzo Mastroianni",
    CardRules::new_artifact(mana_cost!("{4}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_mana(
                "{T}: Add {C}{C}.",
                &[CostDef::TapSource],
                EffectDef::AddMana(AddManaEffectDef {
                    amount: 2,
                    ..AddManaEffectDef::one(ManaColor::Colorless)
                }),
            ),
            AbilityDef::activated_with_targets(
                "{2}, {T}, Pay 1 life: Tap target nonland permanent.",
                &[
                    CostDef::Mana(mana_cost!("{2}")),
                    CostDef::TapSource,
                    CostDef::PayLife(1),
                ],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                )],
                EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
        ]),
);

// FIN 270 — World Map
pub(in crate::card::sets) static WORLD_MAP: CardRecord = CardRecord::new(
    "World Map",
    "70d9ab99-ec8a-402e-ba1d-ffa6c4c84a3f",
    "Septian Fajrianto",
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::activated(
            "{1}, {T}, Sacrifice this artifact: Search your library for a \
             basic land card, reveal it, put it into your hand, then \
             shuffle.",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
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
            "{3}, {T}, Sacrifice this artifact: Search your library for a \
             land card, reveal it, put it into your hand, then shuffle.",
            &[
                CostDef::Mana(mana_cost!("{3}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::HasType(CardType::Land),
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
    ]),
);

// FIN 271 — Adventurer's Inn
pub(in crate::card::sets) static ADVENTURER_S_INN: CardRecord = CardRecord::new(
    "Adventurer's Inn",
    "f0da2ee1-986e-4cbf-92eb-d96fdb572ca5",
    "Allen Morris",
    CardRules::new_land(&["Town"]).with_abilities(&[
        abilities::enters_trigger(
            "When this land enters, you gain 2 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
        abilities::tap_for(ManaColor::Colorless),
    ]),
);

// FIN 272 — Balamb Garden, SeeD Academy // Balamb Garden, Airborne
pub(in crate::card::sets) static BALAMB_GARDEN_SEED_ACADEMY: CardRecord = CardRecord::new_dfc(
    "Balamb Garden, SeeD Academy // Balamb Garden, Airborne",
    "001e9f20-5b15-41cb-bf82-46172decc235",
    "Jonas De Ro",
    &[
        (
            "Balamb Garden, SeeD Academy",
            CardRules::new_land(&["Town"])
                .printed_colors(&[])
                .with_abilities(&[
                    abilities::enters_tapped(CardType::Land),
                    AbilityDef::activated_mana(
                        "{T}: Add {G} or {U}.",
                        &[CostDef::TapSource],
                        EffectDef::AddMana(AddManaEffectDef::choice(&[
                            ManaColor::Green,
                            ManaColor::Blue,
                        ])),
                    ),
                    AbilityDef::activated(
                        "{5}{G}{U}, {T}: Transform this land. This ability costs {1} \
                         less to activate for each other Town you control.",
                        &[CostDef::Mana(mana_cost!("{5}{G}{U}")), CostDef::TapSource],
                        EffectDef::Transform {
                            object: EffectRecipientDef::Source,
                        },
                    )
                    .with_activation_cost_reduction(
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Town")),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        0,
                    ),
                ]),
        ),
        (
            "Balamb Garden, Airborne",
            CardRules::new_vehicle_without_mana_cost(5, 4)
                .with_supertype(CardSupertype::Legendary)
                .printed_colors(&[])
                .with_abilities(&[
                    abilities::flying(),
                    AbilityDef::triggered(
                        "Whenever Balamb Garden attacks, draw a card.",
                        TriggerEventDef::attacks(ObjectPredicateDef::Source),
                        abilities::draw_cards(ValueDef::Constant(1)),
                    ),
                    abilities::crew(
                        "Crew 1 (Tap any number of creatures you control with total \
                         power 1 or more: This Vehicle becomes an artifact creature \
                         until end of turn.)",
                        1,
                    ),
                ]),
        ),
    ],
);

// FIN 273 — Baron, Airship Kingdom
pub(in crate::card::sets) static BARON_AIRSHIP_KINGDOM: CardRecord = CardRecord::new(
    "Baron, Airship Kingdom",
    "6e4bf840-802d-47d5-bffd-8ba495e19cf6",
    "Rockey Chen",
    CardRules::new_land(&["Town"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {U} or {R}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[ManaColor::Blue, ManaColor::Red])),
        ),
    ]),
);

// FIN 274 — Capital City
pub(in crate::card::sets) static CAPITAL_CITY: CardRecord = CardRecord::new(
    "Capital City",
    "f73ce8ec-c916-48eb-ae20-c0d6d03d7145",
    "Wei Guan",
    CardRules::new_land(&["Town"]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{1}, {T}: Add one mana of any color.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
        abilities::cycling!("Cycling {2}", &[CostDef::Mana(mana_cost!("{2}"))]),
    ]),
);

// FIN 275 — Clive's Hideaway
pub(in crate::card::sets) static CLIVE_S_HIDEAWAY: CardRecord = CardRecord::new(
    "Clive's Hideaway",
    "5e43c36f-b8a2-4b2b-b2ea-57e6fa97521c",
    "Jonas De Ro",
    CardRules::new_land(&["Town"]).with_abilities(&[
        abilities::enters_trigger(
            "Hideaway 4 (When this land enters, look at the top four cards \
             of your library, exile one face down, then put the rest on \
             the bottom in a random order.)",
            abilities::hideaway(ValueDef::Constant(4)),
        ),
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{2}, {T}: You may play the exiled card without paying its \
             mana cost if you control four or more legendary creatures.",
            &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    comparison: ComparisonDef::GreaterOrEqual,
                    right: ValueDef::Constant(4),
                }),
                then: &EffectDef::MayPlayWithoutPaying(FreePlayDef {
                    objects: ObjectSetDef::LinkedExiles,
                    duration: FreePlayDurationDef::WhileResolving,
                    mandatory: false,
                    grants_haste: false,
                }),
            },
        ),
    ]),
);

// FIN 276 — Crossroads Village
pub(in crate::card::sets) static CROSSROADS_VILLAGE: CardRecord = CardRecord::new(
    "Crossroads Village",
    "64db46d4-f91f-49cc-971c-b8e19f0c4ea9",
    "Hristo D. Chukov",
    CardRules::new_land(&["Town"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::as_enters(
            "As this land enters, choose a color.",
            ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                BattlefieldEntryScalarChoiceDef::COLOR,
            )),
        ),
        AbilityDef::activated_mana(
            "{T}: Add one mana of the chosen color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one_of_type(ManaTypeDef::ChosenColor)),
        ),
    ]),
);

// FIN 277 — Eden, Seat of the Sanctum
// Audit: unsupported — Needs a reflexive trigger after sacrificing this land that exists independently of the departed source; OptionalEffectTaken currently finds only battlefield listeners.
pub(in crate::card::sets) static EDEN_SEAT_OF_THE_SANCTUM: CardRecord = CardRecord::new(
    "Eden, Seat of the Sanctum",
    "e28eac1e-adc7-4f8d-b206-bef09ba07d38",
    "Leon Tukker",
    CardRules::unsupported(),
);

// FIN 278 — Gohn, Town of Ruin
pub(in crate::card::sets) static GOHN_TOWN_OF_RUIN: CardRecord = CardRecord::new(
    "Gohn, Town of Ruin",
    "99582781-613e-4a33-aec7-7569b4a961aa",
    "Salvatorre Zee Yazzie",
    CardRules::new_land(&["Town"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {B} or {G}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Green,
            ])),
        ),
    ]),
);

// FIN 279 — The Gold Saucer
pub(in crate::card::sets) static THE_GOLD_SAUCER: CardRecord = CardRecord::new(
    "The Gold Saucer",
    "5363c881-443d-43df-afd8-f81e1a1741a2",
    "Anthony Devine",
    CardRules::new_land(&["Town"]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{2}, {T}: Flip a coin. If you win the flip, create a Treasure \
             token.",
            &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
            EffectDef::FlipCoin {
                on_win: &EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
                on_loss: &EffectDef::None,
            },
        ),
        AbilityDef::activated(
            "{3}, {T}, Sacrifice two artifacts: Draw a card.",
            &[
                CostDef::Mana(mana_cost!("{3}")),
                CostDef::TapSource,
                CostDef::sacrifice_permanents(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    PlayerRelation::You,
                    2,
                ),
            ],
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

// FIN 280 — Gongaga, Reactor Town
pub(in crate::card::sets) static GONGAGA_REACTOR_TOWN: CardRecord = CardRecord::new(
    "Gongaga, Reactor Town",
    "7beccfa6-3e4b-4460-954e-870cb39e462d",
    "Le Vuong",
    CardRules::new_land(&["Town"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {R} or {G}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Red,
                ManaColor::Green,
            ])),
        ),
    ]),
);

// FIN 281 — Guadosalam, Farplane Gateway
pub(in crate::card::sets) static GUADOSALAM_FARPLANE_GATEWAY: CardRecord = CardRecord::new(
    "Guadosalam, Farplane Gateway",
    "dfcbc131-fd50-4908-b539-c8e52bb70c58",
    "Vaigintas Pakenis",
    CardRules::new_land(&["Town"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {G} or {U}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Green,
                ManaColor::Blue,
            ])),
        ),
    ]),
);

// FIN 282 — Insomnia, Crown City
pub(in crate::card::sets) static INSOMNIA_CROWN_CITY: CardRecord = CardRecord::new(
    "Insomnia, Crown City",
    "07fca511-a65c-4779-82c0-9215b0dcd068",
    "Jonas De Ro",
    CardRules::new_land(&["Town"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {W} or {B}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Black,
            ])),
        ),
    ]),
);

// FIN 283 — Ishgard, the Holy See // Faith & Grief
pub(in crate::card::sets) static ISHGARD_THE_HOLY_SEE: CardRecord = CardRecord::new(
    "Ishgard, the Holy See // Faith & Grief",
    "068bc755-9d3d-430b-abc5-c775a5415bf9",
    "KOHEI YAMADA",
    CardRules::new_land(&["Town"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for(ManaColor::White),
    ]),
)
.with_composition(|| {
    adventure_land(
        &ISHGARD_THE_HOLY_SEE,
        "Faith & Grief",
        &const {
            CardRules::new_sorcery(mana_cost!("{3}{W}{W}"))
                .with_subtypes(&const { ["Adventure"] })
                .with_ability(
                    AbilityDef::spell_with_targets(
                        "Return up to two target artifact and/or enchantment cards \
                         from your graveyard to your hand. (Then exile this card. You \
                         may play the land later from exile.)",
                        &const {
                            [AbilityTargetDef::up_to(
                                AbilityTargetPredicate::Object {
                                    object: ObjectPredicateDef::AnyOf(
                                        &const {
                                            [
                                                ObjectPredicateDef::HasType(CardType::Artifact),
                                                ObjectPredicateDef::HasType(CardType::Enchantment),
                                            ]
                                        },
                                    ),
                                    zones: &const { [ZoneKind::Graveyard] },
                                    controller: None,
                                    owner: Some(PlayerRelation::You),
                                },
                                2,
                            )]
                        },
                        EffectDef::move_to_zone(
                            EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            ZoneKind::Hand,
                            ZonePlacement::Top,
                        ),
                    )
                    .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
                )
        },
    )
});

// FIN 284 — Jidoor, Aristocratic Capital // Overture
pub(in crate::card::sets) static JIDOOR_ARISTOCRATIC_CAPITAL: CardRecord = CardRecord::new(
    "Jidoor, Aristocratic Capital // Overture",
    "98b2d5b5-f85b-4c42-a0f5-a76f6af304ba",
    "Erikas Perl",
    CardRules::new_land(&["Town"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for(ManaColor::Blue),
    ]),
)
.with_composition(|| {
    adventure_land(
        &JIDOOR_ARISTOCRATIC_CAPITAL,
        "Overture",
        &const {
            CardRules::new_sorcery(mana_cost!("{4}{U}{U}"))
                .with_subtypes(&const { ["Adventure"] })
                .with_ability(
                    AbilityDef::spell_with_targets(
                        "Target opponent mills half their library, rounded down. (Then \
                         exile this card. You may play the land later from exile.)",
                        &const {
                            [AbilityTargetDef::exactly_one(
                                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                            )]
                        },
                        EffectDef::Mill {
                            player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            amount: ValueDef::Halved(
                                &const {
                                    HalvedValueDef {
                                        value: ValueDef::TargetLibrarySize(TargetIndex::PRIMARY),
                                        rounding: RoundingDef::Down,
                                    }
                                },
                            ),
                        },
                    )
                    .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
                )
        },
    )
});

// FIN 285 — Lindblum, Industrial Regency // Mage Siege
pub(in crate::card::sets) static LINDBLUM_INDUSTRIAL_REGENCY: CardRecord = CardRecord::new(
    "Lindblum, Industrial Regency // Mage Siege",
    "548dd152-f0b6-4e8f-9afc-a4ec1671b648",
    "Piotr Dura",
    CardRules::new_land(&["Town"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for(ManaColor::Red),
    ]),
)
.with_composition(|| {
    adventure_land(
        &LINDBLUM_INDUSTRIAL_REGENCY,
        "Mage Siege",
        &const {
            CardRules::new_instant(mana_cost!("{2}{R}"))
                .with_subtypes(&const { ["Adventure"] })
                .with_ability(
                    AbilityDef::spell(
                        "Create a 0/1 black Wizard creature token with \"Whenever you \
                         cast a noncreature spell, this token deals 1 damage to each \
                         opponent.\"",
                        EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                            TokenCharacteristics::creature(
                                &const { ["Wizard"] },
                                &const { [ManaColor::Black] },
                                0,
                                1,
                            )
                            .with_abilities(
                                &const {
                                    [AbilityDef::triggered(
                                        "Whenever you cast a noncreature spell, this token deals 1 \
                                     damage to each opponent.",
                                        TriggerEventDef::spell_cast(ObjectPredicateDef::All(
                                            &const {
                                                [
                                                    ObjectPredicateDef::Not(
                                                        &const {
                                                            ObjectPredicateDef::HasType(
                                                                CardType::Creature,
                                                            )
                                                        },
                                                    ),
                                                    ObjectPredicateDef::ControlledBy(
                                                        PlayerRelation::You,
                                                    ),
                                                ]
                                            },
                                        )),
                                        EffectDef::damage(
                                            EffectRecipientDef::Opponent,
                                            ValueDef::Constant(1),
                                        ),
                                    )]
                                },
                            ),
                        ))),
                    )
                    .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
                )
        },
    )
});

// FIN 286 — Midgar, City of Mako // Reactor Raid
pub(in crate::card::sets) static MIDGAR_CITY_OF_MAKO: CardRecord = CardRecord::new(
    "Midgar, City of Mako // Reactor Raid",
    "8a837256-6bb4-4a60-962d-d2793548d26c",
    "Anthony Devine",
    CardRules::new_land(&["Town"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for(ManaColor::Black),
    ]),
)
.with_composition(|| {
    adventure_land(
        &MIDGAR_CITY_OF_MAKO,
        "Reactor Raid",
        &const {
            CardRules::new_sorcery(mana_cost!("{2}{B}"))
                .with_subtypes(&const { ["Adventure"] })
                .with_ability(
                    AbilityDef::spell(
                        "You may sacrifice an artifact or creature. If you do, draw \
                         two cards. (Then exile this card. You may play the land later \
                         from exile.)",
                        EffectDef::PayOr(PayOrDef::optional(
                            &const {
                                [CostDef::sacrifice_permanent(ObjectPredicateDef::AnyOf(
                                    &const {
                                        [
                                            ObjectPredicateDef::HasType(CardType::Artifact),
                                            ObjectPredicateDef::HasType(CardType::Creature),
                                        ]
                                    },
                                ))]
                            },
                            &const { abilities::draw_cards(ValueDef::Constant(2)) },
                        )),
                    )
                    .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
                )
        },
    )
});

// FIN 287 — Rabanastre, Royal City
pub(in crate::card::sets) static RABANASTRE_ROYAL_CITY: CardRecord = CardRecord::new(
    "Rabanastre, Royal City",
    "c44c9bbe-f4c6-41cf-b3c3-b943f4011bc1",
    "Shahab Alizadeh",
    CardRules::new_land(&["Town"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {R} or {W}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Red,
                ManaColor::White,
            ])),
        ),
    ]),
);

// FIN 288 — Sharlayan, Nation of Scholars
pub(in crate::card::sets) static SHARLAYAN_NATION_OF_SCHOLARS: CardRecord = CardRecord::new(
    "Sharlayan, Nation of Scholars",
    "7a745b5e-cdb8-4d05-ac5c-87be69536da6",
    "David Frasheski",
    CardRules::new_land(&["Town"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {W} or {U}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Blue,
            ])),
        ),
    ]),
);

// FIN 289 — Starting Town
pub(in crate::card::sets) static STARTING_TOWN: CardRecord = CardRecord::new(
    "Starting Town",
    "fc7d1912-7e27-49ef-bd98-375d975a42b0",
    "Hristo D. Chukov",
    // A City of Brass for the turns that matter and a tapped land after
    // them, which is the trade a deck makes for fixing it only needs early.
    CardRules::new_land(&["Town"]).with_abilities(&[
        // "Your first, second, or third turn of the game" counts the turns you
        // have taken rather than the turn number: on the draw, your third turn
        // is the game's sixth, and the Town still comes in untapped.
        AbilityDef::as_enters(
            "This land enters tapped unless it's your first, second, or third turn of the game.",
            ReplacementEffectDef::Conditional {
                condition: ConditionDef::ControllerTurnsTakenAtMost(3),
                if_true: &[],
                if_false: &[ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::Tapped,
                )],
            },
        ),
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{T}, Pay 1 life: Add one mana of any color.",
            &[CostDef::TapSource, CostDef::PayLife(1)],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// FIN 290 — Treno, Dark City
pub(in crate::card::sets) static TRENO_DARK_CITY: CardRecord = CardRecord::new(
    "Treno, Dark City",
    "f6285535-bc44-4274-a886-b14d7c7aaba8",
    "Jonas De Ro",
    CardRules::new_land(&["Town"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {U} or {B}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Blue,
                ManaColor::Black,
            ])),
        ),
    ]),
);

// FIN 291 — Vector, Imperial Capital
pub(in crate::card::sets) static VECTOR_IMPERIAL_CAPITAL: CardRecord = CardRecord::new(
    "Vector, Imperial Capital",
    "10e5648e-4884-41e3-95f8-c76f6bca01e2",
    "Lordigan",
    CardRules::new_land(&["Town"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {B} or {R}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Red,
            ])),
        ),
    ]),
);

// FIN 292 — Windurst, Federation Center
pub(in crate::card::sets) static WINDURST_FEDERATION_CENTER: CardRecord = CardRecord::new(
    "Windurst, Federation Center",
    "c74024bd-b383-468d-9cf5-d112a29f6457",
    "Constantin Marin",
    CardRules::new_land(&["Town"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {G} or {W}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Green,
                ManaColor::White,
            ])),
        ),
    ]),
);

// FIN 293 — Zanarkand, Ancient Metropolis // Lasting Fayth
pub(in crate::card::sets) static ZANARKAND_ANCIENT_METROPOLIS: CardRecord = CardRecord::new(
    "Zanarkand, Ancient Metropolis // Lasting Fayth",
    "881e4c00-3b9a-47a1-bf66-1badda994c88",
    "Erikas Perl",
    CardRules::new_land(&["Town"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for(ManaColor::Green),
    ]),
)
.with_composition(|| {
    adventure_land(
        &ZANARKAND_ANCIENT_METROPOLIS,
        "Lasting Fayth",
        &const {
            CardRules::new_sorcery(mana_cost!("{4}{G}{G}"))
                .with_subtypes(&const { ["Adventure"] })
                .with_ability(
                    AbilityDef::spell(
                        "Create a 1/1 colorless Hero creature token. Put a +1/+1 \
                         counter on it for each land you control. (Then exile this \
                         card. You may play the land later from exile.)",
                        EffectDef::CreateToken(
                            CreateTokenDef::new(TokenDef::Literal(TokenCharacteristics::creature(
                                &const { ["Hero"] },
                                &const { [] },
                                1,
                                1,
                            )))
                            .with_created_tokens(CreatedTokensDef {
                                binding: crate::Binding!("hero"),
                                then: &const {
                                    EffectDef::AddCounters {
                                        object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                            crate::Binding!("hero"),
                                        )),
                                        kind: CounterKind::PlusOnePlusOne,
                                        amount: ValueDef::CountMatchingObjects(
                                            &const {
                                                ObjectQueryDef::matching(
                                                    ObjectPredicateDef::HasType(CardType::Land),
                                                    &const { [ZoneKind::Battlefield] },
                                                    PlayerRelation::You,
                                                )
                                            },
                                        ),
                                    }
                                },
                            }),
                        ),
                    )
                    .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
                )
        },
    )
});

// FIN 294 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "9dd2d666-7c6b-48ce-93dc-c004ebdd1fe9",
    "Shahab Alizadeh",
);

// FIN 295 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "e623b058-76d2-476b-9ee6-82807f7992c3",
    "Jonas De Ro",
);

// FIN 296 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "ee86ac2f-f398-4422-8721-8ac859fbf5bc",
    "Eddie Mendoza",
);

// FIN 297 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "b92ec9f6-a56d-40c6-aee2-7d5e1524c985",
    "Fariba Khamseh",
);

// FIN 298 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "eeecf096-87dd-4dcb-953f-700445bf3f3a",
    "Eddie Mendoza",
);

// FIN 299 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "347e00db-abcf-4053-aa27-4a5b42e1da55",
    "Jeremy Paillotin",
);

// FIN 300 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "f66094ef-059b-4511-aa6e-835906736de4",
    "Domenico Cava",
);

// FIN 301 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "a7f56d20-16dc-4b7f-98bf-124817a83bae",
    "Fang Xinyu",
);

// FIN 302 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "08ec1e4f-e284-4216-a022-bd94c4dae02b",
    "Sean Vo",
);

// FIN 303 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "a18ef64b-a9de-4548-b4d5-168758442db7",
    "Domenico Cava",
);

// FIN 304 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "d061b9a8-e95d-48ec-a1c9-337433b62dfc",
    "Randy Gallegos",
);

// FIN 305 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "ea1735b9-fd10-4c9c-b9d3-046d8c22b852",
    "Sean Vo",
);

// FIN 306 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "be72862d-d71e-4b18-98a6-59019399f631",
    "Alayna Danner",
);

// FIN 307 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "3d08f6a6-316d-45d8-aabe-1760a20903ac",
    "Leon Tukker",
);

// FIN 308 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "c52038c8-5bba-4d8e-845f-30af44300acc",
    "Sean Vo",
);

// FIN 309 — Wastes (reprint)
const WASTES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ogw::WASTES,
    "c61feafd-ef09-437c-a12c-fd7d6cb8c15a",
    "Eddie Mendoza",
);

// FIN 310 — Ishgard, the Holy See // Faith & Grief (alternate printing)
const ISHGARD_THE_HOLY_SEE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ISHGARD_THE_HOLY_SEE,
    1,
    "b5327e09-2ea0-4365-bb20-087a08da8ce6",
    "David Frasheski",
);

// FIN 311 — Jidoor, Aristocratic Capital // Overture (alternate printing)
const JIDOOR_ARISTOCRATIC_CAPITAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &JIDOOR_ARISTOCRATIC_CAPITAL,
    1,
    "72678d58-888e-4d3c-bc2c-78f5b7862e03",
    "Craig Elliott",
);

// FIN 312 — Lindblum, Industrial Regency // Mage Siege (alternate printing)
const LINDBLUM_INDUSTRIAL_REGENCY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LINDBLUM_INDUSTRIAL_REGENCY,
    1,
    "b2ae92d9-7047-424e-a482-2311888a8c56",
    "Enora Mercier",
);

// FIN 313 — Midgar, City of Mako // Reactor Raid (alternate printing)
const MIDGAR_CITY_OF_MAKO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MIDGAR_CITY_OF_MAKO,
    1,
    "fa605930-a7f0-410d-ac58-f5f0bd8caadf",
    "Josu Solano",
);

// FIN 314 — Zanarkand, Ancient Metropolis // Lasting Fayth (alternate printing)
const ZANARKAND_ANCIENT_METROPOLIS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ZANARKAND_ANCIENT_METROPOLIS,
    1,
    "c64bb94b-c1a7-4dd3-b2dc-b87d306d62ad",
    "Jonas De Ro",
);

// FIN 315 — Ardyn, the Usurper (alternate printing)
const ARDYN_THE_USURPER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ARDYN_THE_USURPER,
    1,
    "41ae833d-77ee-4876-8990-591ae0d800ab",
    "Roberto Ferrari",
);

// FIN 316 — Kain, Traitorous Dragoon (alternate printing)
const KAIN_TRAITOROUS_DRAGOON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KAIN_TRAITOROUS_DRAGOON,
    1,
    "39f9de73-84a7-483f-927e-91ade20a2d06",
    "Toshitaka Matsuda",
);

// FIN 317 — Sephiroth, Fabled SOLDIER // Sephiroth, One-Winged Angel (alternate printing)
const SEPHIROTH_FABLED_SOLDIER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SEPHIROTH_FABLED_SOLDIER,
    1,
    "83c82209-47d5-4c4c-b8ec-96ab907e15d3",
    "Tetsuya Nomura",
);

// FIN 318 — Clive, Ifrit's Dominant // Ifrit, Warden of Inferno (alternate printing)
const CLIVE_IFRIT_S_DOMINANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CLIVE_IFRIT_S_DOMINANT,
    1,
    "425d5bea-9c9a-4e8a-b4bc-02284fb9b5b4",
    "Kazuya Takahashi",
);

// FIN 319 — Balthier and Fran (alternate printing)
const BALTHIER_AND_FRAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BALTHIER_AND_FRAN,
    1,
    "6aaba8b1-1fcb-4a73-894f-f32400352c8c",
    "Akihiko Yoshida",
);

// FIN 320 — Lightning, Army of One (alternate printing)
const LIGHTNING_ARMY_OF_ONE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LIGHTNING_ARMY_OF_ONE,
    1,
    "be2af52c-9f38-40e4-a643-06c5f2a9f416",
    "ISAMU KAMIKOKURYO",
);

// FIN 321 — Vivi Ornitier (alternate printing)
const VIVI_ORNITIER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VIVI_ORNITIER,
    1,
    "ada6aeb6-8c9f-4498-9326-b4da0fcb19a3",
    "Toshiyuki Itahana",
);

// FIN 322 — Kefka, Court Mage // Kefka, Ruler of Ruin (alternate printing)
const KEFKA_COURT_MAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KEFKA_COURT_MAGE,
    1,
    "0cadfe0e-9cdf-4e64-b143-f996c76bf93b",
    "Yoshitaka Amano",
);

// FIN 323 — Terra, Magical Adept // Esper Terra (alternate printing)
const TERRA_MAGICAL_ADEPT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TERRA_MAGICAL_ADEPT,
    1,
    "0cf789e7-045c-4ad1-abc3-23eabda54f02",
    "Yoshitaka Amano",
);

// FIN 324 — Ultima, Origin of Oblivion (alternate printing)
const ULTIMA_ORIGIN_OF_OBLIVION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ULTIMA_ORIGIN_OF_OBLIVION,
    1,
    "2ac1b165-27ff-47f3-b598-43f7f021093a",
    "Ono Tako",
);

// FIN 325 — Ambrosia Whiteheart (alternate printing)
const AMBROSIA_WHITEHEART_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AMBROSIA_WHITEHEART,
    1,
    "2ecefb14-23a2-44f4-a3aa-63c246f744e1",
    "Shie Nanahara",
);

// FIN 326 — Moogles' Valor (alternate printing)
const MOOGLES_VALOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MOOGLES_VALOR,
    1,
    "522920b3-e7a4-4097-8321-2dbb326719c6",
    "Yoshiya",
);

// FIN 327 — Stiltzkin, Moogle Merchant (alternate printing)
const STILTZKIN_MOOGLE_MERCHANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STILTZKIN_MOOGLE_MERCHANT,
    1,
    "8347cb57-7c20-4085-83f3-bee35b55b3f1",
    "Yumeko",
);

// FIN 328 — Ultima (alternate printing)
const ULTIMA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ULTIMA,
    1,
    "e673fb51-d36c-42fb-91bf-805812522064",
    "Hisashi Momose",
);

// FIN 329 — Venat, Heart of Hydaelyn // Hydaelyn, the Mothercrystal (alternate printing)
const VENAT_HEART_OF_HYDAELYN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VENAT_HEART_OF_HYDAELYN,
    1,
    "a9a432dc-6e53-48d2-8fbe-b01bd2cab6dd",
    "Minoru & Kei Satsuki",
);

// FIN 330 — The Wind Crystal (alternate printing)
const THE_WIND_CRYSTAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_WIND_CRYSTAL,
    1,
    "423134fa-62d9-439c-b123-eeea52f42414",
    "Minoru",
);

// FIN 331 — Memories Returning (alternate printing)
const MEMORIES_RETURNING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MEMORIES_RETURNING,
    1,
    "fb3c836d-8012-4591-ba4f-e6db6d59b974",
    "Hagiya Kaoru",
);

// FIN 332 — Stolen Uniform (alternate printing)
const STOLEN_UNIFORM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STOLEN_UNIFORM,
    1,
    "0cf03aad-0562-4d29-8c70-dc529c3a3961",
    "Karuta Shiki",
);

// FIN 333 — The Water Crystal (alternate printing)
const THE_WATER_CRYSTAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_WATER_CRYSTAL,
    1,
    "7572888a-c394-4d9f-b66f-30d91364d265",
    "Makura Tami",
);

// FIN 334 — Dark Confidant (alternate printing)
const DARK_CONFIDANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_rav::DARK_CONFIDANT,
    1,
    "378df0f4-5043-461f-8705-9a2c0c3acd43",
    "Ezoi",
);

// FIN 335 — The Darkness Crystal (alternate printing)
const THE_DARKNESS_CRYSTAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_DARKNESS_CRYSTAL,
    1,
    "749f2041-0eb5-42c3-b9ac-05a7575cb693",
    "Kei Satsuki",
);

// FIN 336 — Zodiark, Umbral God (alternate printing)
const ZODIARK_UMBRAL_GOD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ZODIARK_UMBRAL_GOD,
    1,
    "2eb50ecd-d32f-482b-8e6f-57f68179241b",
    "Hisashi Momose",
);

// FIN 337 — The Fire Crystal (alternate printing)
const THE_FIRE_CRYSTAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_FIRE_CRYSTAL,
    1,
    "58306c68-5de8-48f4-9ce5-ea69792af58c",
    "TSUKKU",
);

// FIN 338 — Gilgamesh, Master-at-Arms (alternate printing)
const GILGAMESH_MASTER_AT_ARMS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GILGAMESH_MASTER_AT_ARMS,
    1,
    "816f618c-ed6c-4320-9c33-9ae903e32abf",
    "Ezoi",
);

// FIN 339 — Nibelheim Aflame (alternate printing)
const NIBELHEIM_AFLAME_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NIBELHEIM_AFLAME,
    1,
    "f6c21de9-7278-4406-8037-afe2c29b271f",
    "Rindo Karasuba",
);

// FIN 340 — Triple Triad (alternate printing)
const TRIPLE_TRIAD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TRIPLE_TRIAD,
    1,
    "5bd84d2c-6f09-4aef-a98b-7b256c7de821",
    "Hagiya Kaoru",
);

// FIN 341 — Clash of the Eikons (alternate printing)
const CLASH_OF_THE_EIKONS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CLASH_OF_THE_EIKONS,
    1,
    "fa15ebf3-7ada-4e86-abc9-85dea83edb8c",
    "Hagiya Kaoru",
);

// FIN 342 — The Earth Crystal (alternate printing)
const THE_EARTH_CRYSTAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_EARTH_CRYSTAL,
    1,
    "9240303a-73a3-4daf-b5fc-090ce7cbd92a",
    "Kei Satsuki",
);

// FIN 343 — Jumbo Cactuar (alternate printing)
const JUMBO_CACTUAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &JUMBO_CACTUAR,
    1,
    "fe80b498-a25d-4f68-ac26-9554c3ee1677",
    "Hisashi Momose",
);

// FIN 344 — A Realm Reborn (alternate printing)
const A_REALM_REBORN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &A_REALM_REBORN,
    1,
    "aa78ab04-6ddf-4a67-8bcf-0da4646a1e48",
    "Ono Tako",
);

// FIN 345 — Torgal, A Fine Hound (alternate printing)
const TORGAL_A_FINE_HOUND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TORGAL_A_FINE_HOUND,
    1,
    "66188ecc-ac97-4dcb-ada3-57d86915a911",
    "Makura Tami",
);

// FIN 346 — Jenova, Ancient Calamity (alternate printing)
const JENOVA_ANCIENT_CALAMITY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &JENOVA_ANCIENT_CALAMITY,
    1,
    "9f80efa6-769f-4205-b69e-9cf0e78b7bcd",
    "Karo ARAI",
);

// FIN 347 — Omega, Heartless Evolution (alternate printing)
const OMEGA_HEARTLESS_EVOLUTION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OMEGA_HEARTLESS_EVOLUTION,
    1,
    "615f1a04-2ad4-443b-a915-2181a6bf13af",
    "Tomohito",
);

// FIN 348 — Sin, Spira's Punishment (alternate printing)
const SIN_SPIRA_S_PUNISHMENT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SIN_SPIRA_S_PUNISHMENT,
    1,
    "ce15bf1d-debf-4e0f-a69d-a57619d4b0ee",
    "Ayami Nakashima",
);

// FIN 349 — Tellah, Great Sage (alternate printing)
const TELLAH_GREAT_SAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TELLAH_GREAT_SAGE,
    1,
    "0dcb6a53-ad1b-465b-8d4c-f2c8b2b29e25",
    "Ono Tako",
);

// FIN 350 — Aettir and Priwen (alternate printing)
const AETTIR_AND_PRIWEN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AETTIR_AND_PRIWEN,
    1,
    "2ceea116-6197-45ec-a5a9-73588744b8a0",
    "Shie Nanahara",
);

// FIN 351 — Buster Sword (alternate printing)
const BUSTER_SWORD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BUSTER_SWORD,
    1,
    "f57d0446-1eae-4b88-b65c-92a14a3f9cef",
    "Karuta Shiki",
);

// FIN 352 — Excalibur II (alternate printing)
const EXCALIBUR_II_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EXCALIBUR_II,
    1,
    "becf759b-e3bc-4281-a1e1-1da9f36247ef",
    "Karo ARAI",
);

// FIN 353 — The Masamune (alternate printing)
const THE_MASAMUNE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_MASAMUNE,
    1,
    "a288efba-7c70-428d-a2b7-571891976f78",
    "Ezoi",
);

// FIN 354 — Balamb Garden, SeeD Academy // Balamb Garden, Airborne (alternate printing)
const BALAMB_GARDEN_SEED_ACADEMY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BALAMB_GARDEN_SEED_ACADEMY,
    1,
    "1a6c87dc-188c-4c75-9026-568263940b90",
    "Maiko Aoji",
);

// FIN 355 — Eden, Seat of the Sanctum (alternate printing)
const EDEN_SEAT_OF_THE_SANCTUM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EDEN_SEAT_OF_THE_SANCTUM,
    1,
    "4786e79c-1e64-41bd-87f6-9a33110afa71",
    "Minoru",
);

// FIN 356 — Summon: Bahamut (alternate printing)
const SUMMON_BAHAMUT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUMMON_BAHAMUT,
    1,
    "fd5cb1fc-71b2-4b05-abd3-c90e95c53ad1",
    "Kota Nakatsubo",
);

// FIN 357 — Crystal Fragments // Summon: Alexander (alternate printing)
const CRYSTAL_FRAGMENTS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CRYSTAL_FRAGMENTS,
    1,
    "c2416ecc-bb6b-44f6-8fa6-61d168777f2e",
    "So-Taro",
);

// FIN 358 — Summon: Choco/Mog (alternate printing)
const SUMMON_CHOCO_MOG_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUMMON_CHOCO_MOG,
    1,
    "b95481b1-1780-4cf9-b910-dd0a4461f818",
    "Yoshiya",
);

// FIN 359 — Summon: Knights of Round (alternate printing)
const SUMMON_KNIGHTS_OF_ROUND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUMMON_KNIGHTS_OF_ROUND,
    1,
    "b8a48466-6dc1-4874-9328-c7358e642f39",
    "Hisashi Momose",
);

// FIN 360 — Summon: Primal Garuda (alternate printing)
const SUMMON_PRIMAL_GARUDA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUMMON_PRIMAL_GARUDA,
    1,
    "f6ee6624-79be-4a76-8c12-80bf31f81ae2",
    "Tokima",
);

// FIN 361 — Summon: Leviathan (alternate printing)
const SUMMON_LEVIATHAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUMMON_LEVIATHAN,
    1,
    "4c6738f5-55cb-48f0-8e70-296d1c549cd3",
    "Kota Nakatsubo",
);

// FIN 362 — Summon: Shiva (alternate printing)
const SUMMON_SHIVA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUMMON_SHIVA,
    1,
    "340b2b03-1b20-411c-ba30-2be77655290a",
    "Taku Haruno",
);

// FIN 363 — Jecht, Reluctant Guardian // Braska's Final Aeon (alternate printing)
const JECHT_RELUCTANT_GUARDIAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &JECHT_RELUCTANT_GUARDIAN,
    1,
    "20c75373-7db4-4964-8e09-2badce2f0f84",
    "Tomohito",
);

// FIN 364 — Summon: Anima (alternate printing)
const SUMMON_ANIMA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUMMON_ANIMA,
    1,
    "d4aa9f4e-cba9-41fa-8c65-3e263988f90d",
    "Kota Nakatsubo",
);

// FIN 365 — Summon: Primal Odin (alternate printing)
const SUMMON_PRIMAL_ODIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUMMON_PRIMAL_ODIN,
    1,
    "d8117e64-6e43-4aa8-90df-c5b21fbb9182",
    "Tomohito",
);

// FIN 366 — Summon: Brynhildr (alternate printing)
const SUMMON_BRYNHILDR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUMMON_BRYNHILDR,
    1,
    "fac98745-a30e-4f64-be21-302c0d73d4f7",
    "Tomohito",
);

// FIN 367 — Summon: Esper Ramuh (alternate printing)
const SUMMON_ESPER_RAMUH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUMMON_ESPER_RAMUH,
    1,
    "a6edb808-9753-4029-81ed-60a91ebc1d77",
    "Hagiya Kaoru",
);

// FIN 368 — Summon: G.F. Cerberus (alternate printing)
const SUMMON_G_F_CERBERUS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUMMON_G_F_CERBERUS,
    1,
    "a5ad1e50-2bf1-4f43-9da4-62a89b521206",
    "Yoshiya",
);

// FIN 369 — Summon: G.F. Ifrit (alternate printing)
const SUMMON_G_F_IFRIT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUMMON_G_F_IFRIT,
    1,
    "25c3ce9a-39e5-4273-ba67-224a837b75e7",
    "Ono Tako",
);

// FIN 370 — Esper Origins // Summon: Esper Maduin (alternate printing)
const ESPER_ORIGINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ESPER_ORIGINS,
    1,
    "efbe3aa0-ddf1-4db2-ae4c-cfea7b5d09ed",
    "Tokima",
);

// FIN 371 — Summon: Fat Chocobo (alternate printing)
const SUMMON_FAT_CHOCOBO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUMMON_FAT_CHOCOBO,
    1,
    "a44ae92c-4894-4172-82b3-11e2e94653ef",
    "Buchi",
);

// FIN 372 — Summon: Fenrir (alternate printing)
const SUMMON_FENRIR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUMMON_FENRIR,
    1,
    "412093fb-15c1-4b36-aa53-9484f955ee2c",
    "Maiko Aoji",
);

// FIN 373 — Summon: Titan (alternate printing)
const SUMMON_TITAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUMMON_TITAN,
    1,
    "e5881485-7935-427e-911e-843f198d94dc",
    "Ezoi",
);

// FIN 374 — Aerith Gainsborough (alternate printing)
const AERITH_GAINSBOROUGH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AERITH_GAINSBOROUGH,
    1,
    "fa316d6e-9ea7-4496-a3f3-5023a6952473",
    "Syutsuri",
);

// FIN 375 — Cloud, Midgar Mercenary (alternate printing)
const CLOUD_MIDGAR_MERCENARY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CLOUD_MIDGAR_MERCENARY,
    1,
    "5d8690ec-fded-4801-8eaf-5e5fe3d444ec",
    "Maji",
);

// FIN 376 — Dion, Bahamut's Dominant // Bahamut, Warden of Light (alternate printing)
const DION_BAHAMUT_S_DOMINANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DION_BAHAMUT_S_DOMINANT,
    1,
    "f2309a7e-4ba0-4820-8313-ecd38d32a55f",
    "Maji",
);

// FIN 377 — Gogo, Master of Mimicry (alternate printing)
const GOGO_MASTER_OF_MIMICRY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GOGO_MASTER_OF_MIMICRY,
    1,
    "054e180d-4e66-426d-adb5-a4fee94b6a67",
    "Ryuichi Sakuma",
);

// FIN 378 — Jill, Shiva's Dominant // Shiva, Warden of Ice (alternate printing)
const JILL_SHIVA_S_DOMINANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &JILL_SHIVA_S_DOMINANT,
    1,
    "8b67055b-4a0a-4a16-9c83-d493c36fce01",
    "Rika Suzuki",
);

// FIN 379 — Ardyn, the Usurper (alternate printing)
const ARDYN_THE_USURPER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ARDYN_THE_USURPER,
    2,
    "35090ce1-6fb4-438c-88e9-dd116d550a92",
    "Rorubei",
);

// FIN 380 — Cecil, Dark Knight // Cecil, Redeemed Paladin (alternate printing)
const CECIL_DARK_KNIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CECIL_DARK_KNIGHT,
    1,
    "d8c5c414-c7a0-489b-9cb8-1b910aa89b8e",
    "Misei Ito",
);

// FIN 381 — Fang, Fearless l'Cie (alternate printing)
const FANG_FEARLESS_L_CIE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FANG_FEARLESS_L_CIE,
    1,
    "dca1a706-9eee-4d75-80ee-fa12257908e3",
    "ikeda_cpt",
);

// FIN 381b — Ragnarok, Divine Deliverance (alternate printing)
const RAGNAROK_DIVINE_DELIVERANCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAGNAROK_DIVINE_DELIVERANCE,
    1,
    "0f40e47b-32ed-4846-92b3-cc54e89368fe",
    "ikeda_cpt",
);

// FIN 382 — Sephiroth, Fabled SOLDIER // Sephiroth, One-Winged Angel (alternate printing)
const SEPHIROTH_FABLED_SOLDIER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SEPHIROTH_FABLED_SOLDIER,
    2,
    "f150d6e9-3da6-4655-9c63-dd34525d08a1",
    "Maji",
);

// FIN 383 — Vincent Valentine // Galian Beast (alternate printing)
const VINCENT_VALENTINE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VINCENT_VALENTINE,
    1,
    "221efaa7-ba6e-4f6e-99d1-e258b91091f7",
    "Murakami Hisashi",
);

// FIN 384 — Zenos yae Galvus // Shinryu, Transcendent Rival (alternate printing)
const ZENOS_YAE_GALVUS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ZENOS_YAE_GALVUS,
    1,
    "59fc74c0-9a97-44d5-a62e-a2b68880cfac",
    "Susumu Kuroi",
);

// FIN 385 — Clive, Ifrit's Dominant // Ifrit, Warden of Inferno (alternate printing)
const CLIVE_IFRIT_S_DOMINANT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CLIVE_IFRIT_S_DOMINANT,
    2,
    "faf87bad-253a-4c7a-9dd5-c575cb27db9f",
    "Murakami Hisashi",
);

// FIN 386 — Firion, Wild Rose Warrior (alternate printing)
const FIRION_WILD_ROSE_WARRIOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FIRION_WILD_ROSE_WARRIOR,
    1,
    "8f1cb5a5-2f52-4a9d-a4fe-9603116b7ad5",
    "Kato Ayaka",
);

// FIN 387 — Prompto Argentum (alternate printing)
const PROMPTO_ARGENTUM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PROMPTO_ARGENTUM,
    1,
    "d8f9b257-6491-46d2-be41-846ab4c39d5e",
    "Kato Ayaka",
);

// FIN 388 — Raubahn, Bull of Ala Mhigo (alternate printing)
const RAUBAHN_BULL_OF_ALA_MHIGO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAUBAHN_BULL_OF_ALA_MHIGO,
    1,
    "90ee46cc-12fc-499f-8c13-a289d5fdbaba",
    "Penekor",
);

// FIN 389 — Seifer Almasy (alternate printing)
const SEIFER_ALMASY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SEIFER_ALMASY,
    1,
    "e0863980-7f20-4af0-8a5a-5504b2419f64",
    "Rika Suzuki",
);

// FIN 390 — Vaan, Street Thief (alternate printing)
const VAAN_STREET_THIEF_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VAAN_STREET_THIEF,
    1,
    "6db5c3de-eb63-40f1-a721-9f2de47ea59e",
    "Kato Ayaka",
);

// FIN 391 — Tifa Lockhart (alternate printing)
const TIFA_LOCKHART_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TIFA_LOCKHART,
    1,
    "d402d825-4c2c-46db-b16b-5df583bc5f83",
    "Yoshiro Ambe",
);

// FIN 392 — Vanille, Cheerful l'Cie (alternate printing)
const VANILLE_CHEERFUL_L_CIE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VANILLE_CHEERFUL_L_CIE,
    1,
    "28cdd11b-7337-4696-b2a0-77236c7b7210",
    "ikeda_cpt",
);

// FIN 393 — Balthier and Fran (alternate printing)
const BALTHIER_AND_FRAN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BALTHIER_AND_FRAN,
    2,
    "c8198264-150f-4f95-9de1-cbc7fa456535",
    "S. Makimura",
);

// FIN 394 — Emet-Selch, Unsundered // Hades, Sorcerer of Eld (alternate printing)
const EMET_SELCH_UNSUNDERED_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EMET_SELCH_UNSUNDERED,
    1,
    "fd537047-d997-4ee1-b259-3ed54b7c5576",
    "Rorubei",
);

// FIN 395 — Golbez, Crystal Collector (alternate printing)
const GOLBEZ_CRYSTAL_COLLECTOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GOLBEZ_CRYSTAL_COLLECTOR,
    1,
    "95822b03-a49b-4a77-a257-fe0527088177",
    "Tetsu Kurosawa",
);

// FIN 396 — Hope Estheim (alternate printing)
const HOPE_ESTHEIM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HOPE_ESTHEIM,
    1,
    "5b1fd4c8-7de6-47cb-916a-513d04774f01",
    "Shiyu",
);

// FIN 397 — Joshua, Phoenix's Dominant // Phoenix, Warden of Fire (alternate printing)
const JOSHUA_PHOENIX_S_DOMINANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &JOSHUA_PHOENIX_S_DOMINANT,
    1,
    "04dd9ad9-eab2-447c-b065-078a0b184519",
    "Susumu Kuroi",
);

// FIN 398 — Kefka, Court Mage // Kefka, Ruler of Ruin (alternate printing)
const KEFKA_COURT_MAGE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &KEFKA_COURT_MAGE,
    2,
    "53756218-bcdb-498a-8c61-654183153487",
    "Rorubei",
);

// FIN 399 — Kuja, Genome Sorcerer // Trance Kuja, Fate Defied (alternate printing)
const KUJA_GENOME_SORCERER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &KUJA_GENOME_SORCERER,
    2,
    "374877d1-167a-42ea-adf3-a707ea0300e8",
    "Masateru Ikeda & Robert Cornelius",
);

// FIN 399† — Kuja, Genome Sorcerer // Trance Kuja, Fate Defied (alternate printing)
const KUJA_GENOME_SORCERER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &KUJA_GENOME_SORCERER,
    3,
    "b058d305-d032-47c9-8567-11a4b4319695",
    "Masateru Ikeda & Robert Cornelius",
);

// FIN 400 — Lightning, Army of One (alternate printing)
const LIGHTNING_ARMY_OF_ONE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &LIGHTNING_ARMY_OF_ONE,
    2,
    "45cb2a76-7d12-41e6-93c7-09922934699c",
    "Koji Nishino",
);

// FIN 401 — Noctis, Prince of Lucis (alternate printing)
const NOCTIS_PRINCE_OF_LUCIS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NOCTIS_PRINCE_OF_LUCIS,
    1,
    "8d61fe60-0318-4362-8cea-7c6499b8ca70",
    "Kato Ayaka",
);

// FIN 402 — Squall, SeeD Mercenary (alternate printing)
const SQUALL_SEED_MERCENARY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SQUALL_SEED_MERCENARY,
    1,
    "883c6111-c921-4cd6-930d-4fa335ef2871",
    "Kato Ayaka",
);

// FIN 403 — The Wandering Minstrel (alternate printing)
const THE_WANDERING_MINSTREL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_WANDERING_MINSTREL,
    1,
    "f39b9892-9b82-4dea-bae0-e1735618341e",
    "Penekor",
);

// FIN 404 — Yuna, Hope of Spira (alternate printing)
const YUNA_HOPE_OF_SPIRA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &YUNA_HOPE_OF_SPIRA,
    1,
    "8f861d59-f2ce-457f-99f4-48b12ae34fa2",
    "osamu",
);

// FIN 405 — Zidane, Tantalus Thief (alternate printing)
const ZIDANE_TANTALUS_THIEF_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ZIDANE_TANTALUS_THIEF,
    1,
    "8dca28e1-21f1-4ed7-b294-df2e0c5c9503",
    "Canata Katana",
);

// FIN 406 — Traveling Chocobo (alternate printing)
const TRAVELING_CHOCOBO_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &TRAVELING_CHOCOBO,
    2,
    "28ce1dbc-508c-4d09-8e44-37bf5ebfdc8f",
    "Toni Infante",
);

// FIN 407 — Cid, Timeless Artificer (alternate printing)
const CID_TIMELESS_ARTIFICER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CID_TIMELESS_ARTIFICER,
    1,
    "a3e8d94e-050b-4571-ae12-1e80e6b63611",
    "Gal Or",
);

// FIN 408 — Cid, Timeless Artificer (alternate printing)
const CID_TIMELESS_ARTIFICER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CID_TIMELESS_ARTIFICER,
    2,
    "a9801368-263b-4515-9027-4f1827e7214c",
    "Lee Woo-chul",
);

// FIN 409 — Cid, Timeless Artificer (alternate printing)
const CID_TIMELESS_ARTIFICER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &CID_TIMELESS_ARTIFICER,
    3,
    "2bbab097-acb3-480d-a433-e4f81dde62e4",
    "Narendra Bintara Adi",
);

// FIN 410 — Cid, Timeless Artificer (alternate printing)
const CID_TIMELESS_ARTIFICER_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &CID_TIMELESS_ARTIFICER,
    4,
    "fc4d240f-8920-4f8d-82a2-4cd4da93d6fb",
    "Alexander Mokhov",
);

// FIN 411 — Cid, Timeless Artificer (alternate printing)
const CID_TIMELESS_ARTIFICER_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &CID_TIMELESS_ARTIFICER,
    5,
    "8200a5e1-0c8b-45a4-b468-03e5067e9f81",
    "Nijihayashi",
);

// FIN 412 — Cid, Timeless Artificer (alternate printing)
const CID_TIMELESS_ARTIFICER_ALTERNATE_6: PrintingRecord = PrintingRecord::alternate(
    &CID_TIMELESS_ARTIFICER,
    6,
    "5a9b1404-3061-4979-81eb-e056bc212205",
    "David Astruga",
);

// FIN 413 — Cid, Timeless Artificer (alternate printing)
const CID_TIMELESS_ARTIFICER_ALTERNATE_7: PrintingRecord = PrintingRecord::alternate(
    &CID_TIMELESS_ARTIFICER,
    7,
    "cc027995-3d7c-4283-9e0e-eaef5727f2b8",
    "David Astruga",
);

// FIN 414 — Cid, Timeless Artificer (alternate printing)
const CID_TIMELESS_ARTIFICER_ALTERNATE_8: PrintingRecord = PrintingRecord::alternate(
    &CID_TIMELESS_ARTIFICER,
    8,
    "4bd892ea-b5bc-4b57-b317-741113381b50",
    "Kevin Glint",
);

// FIN 415 — Cid, Timeless Artificer (alternate printing)
const CID_TIMELESS_ARTIFICER_ALTERNATE_9: PrintingRecord = PrintingRecord::alternate(
    &CID_TIMELESS_ARTIFICER,
    9,
    "b6776f23-5f5b-4f0a-a00c-1ca8f03ffc5f",
    "Nurikabe",
);

// FIN 416 — Cid, Timeless Artificer (alternate printing)
const CID_TIMELESS_ARTIFICER_ALTERNATE_10: PrintingRecord = PrintingRecord::alternate(
    &CID_TIMELESS_ARTIFICER,
    10,
    "e564eda2-36dd-4f44-b84b-d0c18a9bdb1e",
    "David Astruga",
);

// FIN 417 — Cid, Timeless Artificer (alternate printing)
const CID_TIMELESS_ARTIFICER_ALTERNATE_11: PrintingRecord = PrintingRecord::alternate(
    &CID_TIMELESS_ARTIFICER,
    11,
    "3952bbbb-6fe4-4d13-8825-e93df1d048e9",
    "Arif Wijaya",
);

// FIN 418 — Cid, Timeless Artificer (alternate printing)
const CID_TIMELESS_ARTIFICER_ALTERNATE_12: PrintingRecord = PrintingRecord::alternate(
    &CID_TIMELESS_ARTIFICER,
    12,
    "0148fa7c-b6d6-458c-8cdd-a6c4e6dd4e59",
    "Russell Dongjun Lu",
);

// FIN 419 — Cid, Timeless Artificer (alternate printing)
const CID_TIMELESS_ARTIFICER_ALTERNATE_13: PrintingRecord = PrintingRecord::alternate(
    &CID_TIMELESS_ARTIFICER,
    13,
    "7d8b2dfc-ecaf-403b-932f-a5efc8b713e6",
    "Jason Kiantoro",
);

// FIN 420 — Cid, Timeless Artificer (alternate printing)
const CID_TIMELESS_ARTIFICER_ALTERNATE_14: PrintingRecord = PrintingRecord::alternate(
    &CID_TIMELESS_ARTIFICER,
    14,
    "2c1b48ba-b158-4b9d-802c-5cbfdf96dca5",
    "Magali Villeneuve",
);

// FIN 421 — Ultima, Origin of Oblivion (alternate printing)
const ULTIMA_ORIGIN_OF_OBLIVION_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ULTIMA_ORIGIN_OF_OBLIVION,
    2,
    "e6e27054-03e1-424e-92d8-6e77d7683d79",
    "Russell Dongjun Lu",
);

// FIN 422 — Adelbert Steiner (alternate printing)
const ADELBERT_STEINER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ADELBERT_STEINER,
    1,
    "817ffbb6-5e7c-4498-af65-9e3d0cc5bcd6",
    "Lorenzo Mastroianni",
);

// FIN 423 — Aerith Gainsborough (alternate printing)
const AERITH_GAINSBOROUGH_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &AERITH_GAINSBOROUGH,
    2,
    "6a1796e1-30c5-4e55-bec8-7013512606b0",
    "Nakamura8",
);

// FIN 424 — Ambrosia Whiteheart (alternate printing)
const AMBROSIA_WHITEHEART_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &AMBROSIA_WHITEHEART,
    2,
    "fcd2f288-9d05-465a-bb3e-2a444a027096",
    "Fajareka Setiawan",
);

// FIN 425 — Ashe, Princess of Dalmasca (alternate printing)
const ASHE_PRINCESS_OF_DALMASCA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ASHE_PRINCESS_OF_DALMASCA,
    1,
    "9b103e50-8bac-41d4-966e-a5d42132bc12",
    "Yumi Yaoshida",
);

// FIN 426 — Beatrix, Loyal General
// Audit: unsupported — Needs an attachment operation between a selected collection of Equipment and an independently targeted creature; current attachment effects require the ability source as the Equipment or host.
pub(in crate::card::sets) static BEATRIX_LOYAL_GENERAL: CardRecord = CardRecord::new(
    "Beatrix, Loyal General",
    "ab1b24dd-ab23-41b0-a074-e77e8d9c5564",
    "Bachzim",
    CardRules::unsupported(),
);

// FIN 427 — Cloud, Midgar Mercenary (alternate printing)
const CLOUD_MIDGAR_MERCENARY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CLOUD_MIDGAR_MERCENARY,
    2,
    "86fd126f-7672-4d83-93df-c61184d2fcf4",
    "Kazto Furuya",
);

// FIN 428 — Dion, Bahamut's Dominant // Bahamut, Warden of Light (alternate printing)
const DION_BAHAMUT_S_DOMINANT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DION_BAHAMUT_S_DOMINANT,
    2,
    "d869c60b-3aa2-4a90-aa93-5d4c5afd1672",
    "Kevin Glint",
);

// FIN 429 — G'raha Tia (alternate printing)
const G_RAHA_TIA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &G_RAHA_TIA,
    1,
    "d285d6fa-b618-4558-b1e6-d148e0927cb5",
    "Narendra Bintara Adi",
);

// FIN 430 — Minwu, White Mage (alternate printing)
const MINWU_WHITE_MAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MINWU_WHITE_MAGE,
    1,
    "99ad3e31-815c-409f-8fe5-d13a3f5d2b65",
    "Josu Hernaiz",
);

// FIN 431 — Rosa, Resolute White Mage
pub(in crate::card::sets) static ROSA_RESOLUTE_WHITE_MAGE: CardRecord = CardRecord::new(
    "Rosa, Resolute White Mage",
    "d7964800-1392-4bb8-84e2-c9f3938ceac3",
    "Christian Angel",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Noble", "Cleric"], 2, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::reach(),
            AbilityDef::triggered_with_targets(
                "At the beginning of combat on your turn, put a +1/+1 counter \
                 on target creature you control. It gains lifelink until end \
                 of turn. (Damage dealt by the creature also causes you to \
                 gain that much life.)",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::BeginningOfCombat,
                    player: PlayerRelation::You,
                },
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
                        effect: AppliedEffectDef::add_ability(&abilities::lifelink()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            ),
        ]),
);

// FIN 432 — Snow Villiers (alternate printing)
const SNOW_VILLIERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SNOW_VILLIERS,
    1,
    "8ff58500-479d-48d2-b84e-ff3cdb9d0154",
    "Fariba Khamseh",
);

// FIN 433 — Stiltzkin, Moogle Merchant (alternate printing)
const STILTZKIN_MOOGLE_MERCHANT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &STILTZKIN_MOOGLE_MERCHANT,
    2,
    "ea207686-faf2-4e60-a613-a6752a3d1ddf",
    "Hendry Iwanaga",
);

// FIN 434 — Venat, Heart of Hydaelyn // Hydaelyn, the Mothercrystal (alternate printing)
const VENAT_HEART_OF_HYDAELYN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &VENAT_HEART_OF_HYDAELYN,
    2,
    "790d1050-025a-4792-8e89-24b14278eec5",
    "Colin Boyer",
);

// FIN 435 — Zack Fair (alternate printing)
const ZACK_FAIR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ZACK_FAIR,
    1,
    "f99f3a14-3a7c-4c1c-b747-c6b1144ef1f1",
    "Yoshio Sugiura",
);

// FIN 436 — Edgar, King of Figaro (alternate printing)
const EDGAR_KING_OF_FIGARO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EDGAR_KING_OF_FIGARO,
    1,
    "f28bd4d2-b2cf-48f1-ba11-956c12284550",
    "Jake Murray",
);

// FIN 437 — Gogo, Master of Mimicry (alternate printing)
const GOGO_MASTER_OF_MIMICRY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GOGO_MASTER_OF_MIMICRY,
    2,
    "59156e99-5514-4eb3-b976-bb6cfe354d12",
    "Thea Dumitriu",
);

// FIN 438 — Jill, Shiva's Dominant // Shiva, Warden of Ice (alternate printing)
const JILL_SHIVA_S_DOMINANT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &JILL_SHIVA_S_DOMINANT,
    2,
    "863dc1bd-8554-42ba-8d49-d99ea969103d",
    "Arif Wijaya",
);

// FIN 439 — Matoya, Archon Elder (alternate printing)
const MATOYA_ARCHON_ELDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MATOYA_ARCHON_ELDER,
    1,
    "0429bba6-d496-49ac-b849-fb8274f86880",
    "Luisa J. Preissler",
);

// FIN 440 — Quistis Trepe (alternate printing)
const QUISTIS_TREPE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &QUISTIS_TREPE,
    1,
    "3cc379aa-6810-421d-a317-026867c4d245",
    "Touge369",
);

// FIN 441 — Ultimecia, Temporal Threat
pub(in crate::card::sets) static ULTIMECIA_TEMPORAL_THREAT: CardRecord = CardRecord::new(
    "Ultimecia, Temporal Threat",
    "f988af75-acf4-41bb-b4c5-58492fd1f6a0",
    "Bachzim",
    CardRules::new_creature(mana_cost!("{4}{U}{U}"), &["Human", "Warlock"], 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When Ultimecia enters, tap all creatures your opponents control.",
                EffectDef::Tap {
                    object: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Opponent,
                        ),
                    )),
                },
            ),
            AbilityDef::triggered(
                "Whenever a creature you control deals combat damage to a \
                 player, draw a card.",
                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                abilities::draw_cards(ValueDef::Constant(1)),
            ),
        ]),
);

// FIN 442 — Ultros, Obnoxious Octopus (alternate printing)
const ULTROS_OBNOXIOUS_OCTOPUS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ULTROS_OBNOXIOUS_OCTOPUS,
    1,
    "9e3f57b6-0d03-4176-8c2a-0937437fb62e",
    "Domenico Cava",
);

// FIN 443 — Y'shtola Rhul (alternate printing)
const Y_SHTOLA_RHUL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &Y_SHTOLA_RHUL,
    1,
    "a1d3d33d-4cd9-4009-82c9-adb2e1ec45cb",
    "Immanuela Crovius",
);

// FIN 444 — Ardyn, the Usurper (alternate printing)
const ARDYN_THE_USURPER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &ARDYN_THE_USURPER,
    3,
    "c4c34b33-4b03-4225-ac22-a34efe173b42",
    "Russell Dongjun Lu",
);

// FIN 445 — Cecil, Dark Knight // Cecil, Redeemed Paladin (alternate printing)
const CECIL_DARK_KNIGHT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CECIL_DARK_KNIGHT,
    2,
    "c976f89d-768a-4cee-b0ac-90f854bfe94f",
    "Josu Hernaiz",
);

// FIN 446 — Fang, Fearless l'Cie (alternate printing)
const FANG_FEARLESS_L_CIE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &FANG_FEARLESS_L_CIE,
    2,
    "b82f08c2-3944-437d-9af6-f1e9088e02bb",
    "Simon Dominic",
);

// FIN 446b — Ragnarok, Divine Deliverance (alternate printing)
const RAGNAROK_DIVINE_DELIVERANCE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &RAGNAROK_DIVINE_DELIVERANCE,
    2,
    "cbcde2c1-e973-4ec0-b333-d71fa7452ef1",
    "Simon Dominic",
);

// FIN 447 — Gaius van Baelsar (alternate printing)
const GAIUS_VAN_BAELSAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GAIUS_VAN_BAELSAR,
    1,
    "42605473-e37d-46b3-8979-31cc6b21bc3c",
    "Nino Is",
);

// FIN 448 — Jecht, Reluctant Guardian // Braska's Final Aeon (alternate printing)
const JECHT_RELUCTANT_GUARDIAN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &JECHT_RELUCTANT_GUARDIAN,
    2,
    "3ae96898-3a72-428a-93e8-93ded3662669",
    "Michael MacRae",
);

// FIN 449 — Kain, Traitorous Dragoon (alternate printing)
const KAIN_TRAITOROUS_DRAGOON_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &KAIN_TRAITOROUS_DRAGOON,
    2,
    "28eb294b-4cab-4c34-ae29-20aac9799471",
    "Russell Dongjun Lu",
);

// FIN 450 — Reno and Rude (alternate printing)
const RENO_AND_RUDE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RENO_AND_RUDE,
    1,
    "ef6a2ef4-4aa1-4037-89d5-50c4a6579371",
    "Maji",
);

// FIN 451 — Sephiroth, Fabled SOLDIER // Sephiroth, One-Winged Angel (alternate printing)
const SEPHIROTH_FABLED_SOLDIER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SEPHIROTH_FABLED_SOLDIER,
    3,
    "7cb5acb6-14af-4e17-8e6c-b44122bd81e4",
    "Wisnu Tan",
);

// FIN 452 — Seymour Flux
pub(in crate::card::sets) static SEYMOUR_FLUX: CardRecord = CardRecord::new(
    "Seymour Flux",
    "8fc42c17-a12b-42db-bb01-3e444ac2a376",
    "K-SUWABE",
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Spirit", "Avatar"], 5, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::triggered(
            "At the beginning of your upkeep, you may pay 1 life. If you \
             do, draw a card and put a +1/+1 counter on Seymour Flux.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::PayLife(1)],
                &EffectDef::Sequence(&[
                    abilities::draw_cards(ValueDef::Constant(1)),
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            )),
        )]),
);

// FIN 453 — Sidequest: Hunt the Mark // Yiazmat, Ultimate Mark (alternate printing)
const SIDEQUEST_HUNT_THE_MARK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SIDEQUEST_HUNT_THE_MARK,
    1,
    "de8067cc-3de3-43b2-8fcf-6dce6f4f7db1",
    "Nino Is & Joshua Raphael",
);

// FIN 454 — Vincent Valentine // Galian Beast (alternate printing)
const VINCENT_VALENTINE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &VINCENT_VALENTINE,
    2,
    "2b2a676e-901f-4e0e-87c6-69820f319d7f",
    "Norikatsu Miyoshi",
);

// FIN 455 — Zenos yae Galvus // Shinryu, Transcendent Rival (alternate printing)
const ZENOS_YAE_GALVUS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ZENOS_YAE_GALVUS,
    2,
    "40658fd0-78a6-44e5-9351-512e6bdcf637",
    "Alexander Mokhov",
);

// FIN 456 — Zodiark, Umbral God (alternate printing)
const ZODIARK_UMBRAL_GOD_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ZODIARK_UMBRAL_GOD,
    2,
    "f7f9977f-b2dc-46bb-b961-097675cdcb45",
    "AKAGI",
);

// FIN 457 — Barret Wallace (alternate printing)
const BARRET_WALLACE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BARRET_WALLACE,
    1,
    "3c8d6eaa-e6d1-4a52-98fb-9cb4d876cc0b",
    "Patrik Hell",
);

// FIN 458 — Clive, Ifrit's Dominant // Ifrit, Warden of Inferno (alternate printing)
const CLIVE_IFRIT_S_DOMINANT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &CLIVE_IFRIT_S_DOMINANT,
    3,
    "cdf7fb4a-acb5-4fae-a44c-c437e5b82ec4",
    "Nino Is",
);

// FIN 459 — Firion, Wild Rose Warrior (alternate printing)
const FIRION_WILD_ROSE_WARRIOR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &FIRION_WILD_ROSE_WARRIOR,
    2,
    "70def31c-e328-4f1f-a165-a43f94401c65",
    "Elizabeth Peiró",
);

// FIN 460 — Freya Crescent (alternate printing)
const FREYA_CRESCENT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FREYA_CRESCENT,
    1,
    "e5aa0b07-9ebb-46d7-a80f-2e644dbd9d36",
    "Nereida",
);

// FIN 461 — Gilgamesh, Master-at-Arms (alternate printing)
const GILGAMESH_MASTER_AT_ARMS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GILGAMESH_MASTER_AT_ARMS,
    2,
    "79070b71-3737-4d9a-8193-b2d7bf8d876b",
    "Lorenzo Mastroianni",
);

// FIN 462 — Lightning, Security Sergeant
// Audit: unsupported — Needs exile-play permission tied to continuous control of this source, expiring when control is lost and remaining expired if it is regained; current exile permissions have no source-control lifetime.
pub(in crate::card::sets) static LIGHTNING_SECURITY_SERGEANT: CardRecord = CardRecord::new(
    "Lightning, Security Sergeant",
    "40cce7b4-1d5d-4d91-90c5-71b79b5be1ac",
    "Ramza Psyru",
    CardRules::unsupported(),
);

// FIN 463 — Prompto Argentum (alternate printing)
const PROMPTO_ARGENTUM_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &PROMPTO_ARGENTUM,
    2,
    "5ee02af5-cb6c-43a3-8a1d-7e8630a90739",
    "Billy Christian",
);

// FIN 464 — Queen Brahne (alternate printing)
const QUEEN_BRAHNE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &QUEEN_BRAHNE,
    1,
    "5957ab2d-55ed-4160-8c78-8c70009c39d8",
    "Lorenzo Mastroianni",
);

// FIN 465 — Raubahn, Bull of Ala Mhigo (alternate printing)
const RAUBAHN_BULL_OF_ALA_MHIGO_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &RAUBAHN_BULL_OF_ALA_MHIGO,
    2,
    "3732c276-3635-46e0-a46e-1b433501d3fd",
    "Julia Vasilyeva",
);

// FIN 466 — Seifer Almasy (alternate printing)
const SEIFER_ALMASY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SEIFER_ALMASY,
    2,
    "50467632-dddf-45ef-900e-9718f364cf9b",
    "Kotetsu Kinoshita",
);

// FIN 467 — Vaan, Street Thief (alternate printing)
const VAAN_STREET_THIEF_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &VAAN_STREET_THIEF,
    2,
    "dbd01ef3-0b06-46a3-97f9-eaff2a539a4a",
    "Jake Murray",
);

// FIN 468 — Zell Dincht (alternate printing)
const ZELL_DINCHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ZELL_DINCHT,
    1,
    "25d28011-f0a6-4d7b-ab87-aae0fe354d9e",
    "Kevin Sidharta",
);

// FIN 469 — Bartz and Boko (alternate printing)
const BARTZ_AND_BOKO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BARTZ_AND_BOKO,
    1,
    "1b9a92f8-2044-4554-aec9-00b410dada35",
    "Ryuichi Sakuma",
);

// FIN 470 — Diamond Weapon (alternate printing)
const DIAMOND_WEAPON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DIAMOND_WEAPON,
    1,
    "bb17f0c0-4738-42a0-94e1-27344f13735f",
    "Esuthio",
);

// FIN 471 — Quina, Qu Gourmet (alternate printing)
const QUINA_QU_GOURMET_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &QUINA_QU_GOURMET,
    1,
    "0208a5a9-3a6f-44c5-9e46-24bdf924753c",
    "Fajareka Setiawan",
);

// FIN 472 — Sazh Katzroy (alternate printing)
const SAZH_KATZROY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SAZH_KATZROY,
    1,
    "366fbdb4-56b4-4dd1-8cc7-565786c71c24",
    "Colin Boyer",
);

// FIN 473 — Tifa Lockhart (alternate printing)
const TIFA_LOCKHART_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &TIFA_LOCKHART,
    2,
    "165a50ba-5ead-45aa-be8a-9bdd9766da66",
    "Laurel Austin",
);

// FIN 474 — Torgal, A Fine Hound (alternate printing)
const TORGAL_A_FINE_HOUND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &TORGAL_A_FINE_HOUND,
    2,
    "b4b78fba-cde2-4ab9-bc96-51d0fc5b29cf",
    "Narendra Bintara Adi",
);

// FIN 475 — Vanille, Cheerful l'Cie (alternate printing)
const VANILLE_CHEERFUL_L_CIE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &VANILLE_CHEERFUL_L_CIE,
    2,
    "6c752ee2-81b0-468a-a54b-740c22f2334e",
    "Simon Dominic",
);

// FIN 476 — Absolute Virtue (alternate printing)
const ABSOLUTE_VIRTUE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ABSOLUTE_VIRTUE,
    1,
    "50716867-4370-4c2f-aae5-9d791a5d5a2e",
    "Toni Infante",
);

// FIN 477 — Balthier and Fran (alternate printing)
const BALTHIER_AND_FRAN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &BALTHIER_AND_FRAN,
    3,
    "8c9eaf03-0af1-496f-b3d9-37938a8dc6dd",
    "Arif Wijaya",
);

// FIN 478 — Black Waltz No. 3 (alternate printing)
const BLACK_WALTZ_NO_3_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BLACK_WALTZ_NO_3,
    1,
    "7d143a13-a627-4b64-9356-253a2f9ddf98",
    "Lordigan",
);

// FIN 479 — Choco, Seeker of Paradise (alternate printing)
const CHOCO_SEEKER_OF_PARADISE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHOCO_SEEKER_OF_PARADISE,
    1,
    "6bf0cce3-ba72-4f7f-8b65-01e5dcd3299f",
    "Miho Midorikawa",
);

// FIN 480 — Cid, Timeless Artificer (alternate printing)
const CID_TIMELESS_ARTIFICER_ALTERNATE_15: PrintingRecord = PrintingRecord::alternate(
    &CID_TIMELESS_ARTIFICER,
    15,
    "db9830c7-1f40-4ff2-8e99-7c6393d4a968",
    "Lius Lasahido",
);

// FIN 481 — Cloud of Darkness (alternate printing)
const CLOUD_OF_DARKNESS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CLOUD_OF_DARKNESS,
    1,
    "c53b7098-02c6-4977-8871-334f3fd4ee1f",
    "Fariba Khamseh",
);

// FIN 482 — Cloud, Planet's Champion
// Audit: unsupported — Needs equip-cost reduction to inspect that activation's selected creature target; current ability-cost reductions filter source permanents rather than targets.
pub(in crate::card::sets) static CLOUD_PLANET_S_CHAMPION: CardRecord = CardRecord::new(
    "Cloud, Planet's Champion",
    "75180e47-064e-4dd6-adcf-d4fb497a445b",
    "Magali Villeneuve",
    CardRules::unsupported(),
);

// FIN 483 — Emet-Selch, Unsundered // Hades, Sorcerer of Eld (alternate printing)
const EMET_SELCH_UNSUNDERED_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &EMET_SELCH_UNSUNDERED,
    2,
    "0f462d81-bb1b-4d44-8952-6dff52970792",
    "Néstor Ossandón Leal",
);

// FIN 484 — The Emperor of Palamecia // The Lord Master of Hell (alternate printing)
const THE_EMPEROR_OF_PALAMECIA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_EMPEROR_OF_PALAMECIA,
    1,
    "1b4d8a51-2307-4ed6-af25-c48946219096",
    "Heonhwa",
);

// FIN 485 — Exdeath, Void Warlock // Neo Exdeath, Dimension's End (alternate printing)
const EXDEATH_VOID_WARLOCK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EXDEATH_VOID_WARLOCK,
    1,
    "df51bf51-839a-4b4d-a6c2-431c9ebff875",
    "Jessica Fong",
);

// FIN 486 — Garland, Knight of Cornelia // Chaos, the Endless (alternate printing)
const GARLAND_KNIGHT_OF_CORNELIA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GARLAND_KNIGHT_OF_CORNELIA,
    1,
    "8e6e21d2-e282-4f56-8e6b-929176b264b3",
    "Billy Christian",
);

// FIN 487 — Garnet, Princess of Alexandria (alternate printing)
const GARNET_PRINCESS_OF_ALEXANDRIA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GARNET_PRINCESS_OF_ALEXANDRIA,
    1,
    "6136d21e-e6fa-4aaf-9eca-3847fa2c65fd",
    "Daniel Correia",
);

// FIN 488 — Giott, King of the Dwarves (alternate printing)
const GIOTT_KING_OF_THE_DWARVES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GIOTT_KING_OF_THE_DWARVES,
    1,
    "e5220843-4964-4abe-b397-a909be8ed8e0",
    "Ben Wootten",
);

// FIN 489 — Gladiolus Amicitia (alternate printing)
const GLADIOLUS_AMICITIA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GLADIOLUS_AMICITIA,
    1,
    "8e2d8e83-1fd4-4c99-ae84-b450e3650cb7",
    "Gal Or",
);

// FIN 490 — Golbez, Crystal Collector (alternate printing)
const GOLBEZ_CRYSTAL_COLLECTOR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GOLBEZ_CRYSTAL_COLLECTOR,
    2,
    "db013d39-3681-4ed0-8671-d1d2452df9de",
    "Bachzim",
);

// FIN 491 — Hope Estheim (alternate printing)
const HOPE_ESTHEIM_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &HOPE_ESTHEIM,
    2,
    "a50f113e-be5d-4251-8605-67f54fbe9b3f",
    "Fariba Khamseh",
);

// FIN 492 — Ignis Scientia (alternate printing)
const IGNIS_SCIENTIA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &IGNIS_SCIENTIA,
    1,
    "2790a23d-00ff-47b7-badd-9dd68042bb99",
    "Mingchen Shen",
);

// FIN 493 — Jenova, Ancient Calamity (alternate printing)
const JENOVA_ANCIENT_CALAMITY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &JENOVA_ANCIENT_CALAMITY,
    2,
    "22dd493f-e481-4563-8f87-c5bdbfd54fdd",
    "Ignatius Budi",
);

// FIN 494 — Joshua, Phoenix's Dominant // Phoenix, Warden of Fire (alternate printing)
const JOSHUA_PHOENIX_S_DOMINANT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &JOSHUA_PHOENIX_S_DOMINANT,
    2,
    "8d5aaba7-1d36-4a0f-ad3e-70dca5944c53",
    "Lius Lasahido",
);

// FIN 495 — Judge Magister Gabranth (alternate printing)
const JUDGE_MAGISTER_GABRANTH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &JUDGE_MAGISTER_GABRANTH,
    1,
    "ab39a24e-c73b-4d62-b7a0-c4ea968768c9",
    "Josu Hernaiz",
);

// FIN 496 — Kefka, Court Mage // Kefka, Ruler of Ruin (alternate printing)
const KEFKA_COURT_MAGE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &KEFKA_COURT_MAGE,
    3,
    "06b7ca77-7194-4a0b-a650-4afd7afb50eb",
    "Xui Ton",
);

// FIN 497 — Kuja, Genome Sorcerer // Trance Kuja, Fate Defied (alternate printing)
const KUJA_GENOME_SORCERER_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &KUJA_GENOME_SORCERER,
    4,
    "98067c9a-d81e-4f5c-b55a-28872331fce8",
    "Joshua Raphael",
);

// FIN 497† — Kuja, Genome Sorcerer // Trance Kuja, Fate Defied (alternate printing)
const KUJA_GENOME_SORCERER_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &KUJA_GENOME_SORCERER,
    5,
    "cdb5acf7-2878-466d-8224-24fa64a602f5",
    "Joshua Raphael",
);

// FIN 498 — Lightning, Army of One (alternate printing)
const LIGHTNING_ARMY_OF_ONE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &LIGHTNING_ARMY_OF_ONE,
    3,
    "0c665905-183b-401f-b83c-a312d032e061",
    "Shiyu",
);

// FIN 499 — Locke Cole (alternate printing)
const LOCKE_COLE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LOCKE_COLE,
    1,
    "81d0f2e4-8b22-4c9b-ad01-fd0818fe7372",
    "AKAGI",
);

// FIN 500 — Noctis, Prince of Lucis (alternate printing)
const NOCTIS_PRINCE_OF_LUCIS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &NOCTIS_PRINCE_OF_LUCIS,
    2,
    "8717cde9-eaee-4926-ab6e-02bb58421d05",
    "Jeremy Chong",
);

// FIN 501 — Omega, Heartless Evolution (alternate printing)
const OMEGA_HEARTLESS_EVOLUTION_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &OMEGA_HEARTLESS_EVOLUTION,
    2,
    "926953df-e1f8-4d3e-a05d-93657143da04",
    "Josu Solano",
);

// FIN 502 — Rinoa Heartilly (alternate printing)
const RINOA_HEARTILLY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RINOA_HEARTILLY,
    1,
    "3f77c0f4-7a1b-496e-b030-55627446dfe3",
    "Francesca Resta",
);

// FIN 503 — Rufus Shinra (alternate printing)
const RUFUS_SHINRA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RUFUS_SHINRA,
    1,
    "5d49c95e-2398-4646-b9f1-44d0b73fc157",
    "Ittoku",
);

// FIN 504 — Rydia, Summoner of Mist (alternate printing)
const RYDIA_SUMMONER_OF_MIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RYDIA_SUMMONER_OF_MIST,
    1,
    "f67652be-9b37-41d0-93db-ba26f8a23186",
    "Yumi Yaoshida",
);

// FIN 505 — Sephiroth, Planet's Heir
pub(in crate::card::sets) static SEPHIROTH_PLANET_S_HEIR: CardRecord = CardRecord::new(
    "Sephiroth, Planet's Heir",
    "b03e29e2-cfda-4284-8d30-0686c01e861e",
    "Magali Villeneuve",
    CardRules::new_creature(
        mana_cost!("{4}{U}{B}"),
        &["Human", "Avatar", "Soldier"],
        4,
        4,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::vigilance(),
        abilities::enters_trigger(
            "When Sephiroth enters, creatures your opponents control get \
             -2/-2 until end of turn.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Opponent,
                    ),
                )),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-2),
                    ValueDef::Constant(-2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::triggered(
            "Whenever a creature an opponent controls dies, put a +1/+1 \
             counter on Sephiroth.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// FIN 506 — Serah Farron // Crystallized Serah (alternate printing)
const SERAH_FARRON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SERAH_FARRON,
    1,
    "7224bb1b-167c-47e5-8314-84aebca054b6",
    "Carissa Susilo",
);

// FIN 507 — Shantotto, Tactician Magician (alternate printing)
const SHANTOTTO_TACTICIAN_MAGICIAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SHANTOTTO_TACTICIAN_MAGICIAN,
    1,
    "d1e01457-1696-4313-b21e-36e088735b1f",
    "Joshua Raphael",
);

// FIN 508 — Sin, Spira's Punishment (alternate printing)
const SIN_SPIRA_S_PUNISHMENT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SIN_SPIRA_S_PUNISHMENT,
    2,
    "075dd582-61b7-45e9-9dcf-0b544b733e61",
    "John Tedrick",
);

// FIN 509 — Squall, SeeD Mercenary (alternate printing)
const SQUALL_SEED_MERCENARY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SQUALL_SEED_MERCENARY,
    2,
    "fef02132-34e6-4f3a-ac60-af3221a782b3",
    "Yuu Fujiki",
);

// FIN 510 — Tellah, Great Sage (alternate printing)
const TELLAH_GREAT_SAGE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &TELLAH_GREAT_SAGE,
    2,
    "e0c5f3d5-ca17-4d4c-8de4-3559d6c6fe94",
    "Yumi Yaoshida",
);

// FIN 511 — Terra, Magical Adept // Esper Terra (alternate printing)
const TERRA_MAGICAL_ADEPT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &TERRA_MAGICAL_ADEPT,
    2,
    "fe0aa7d7-73e7-4c8c-92b5-b923817ce461",
    "Clare Wong",
);

// FIN 512 — Tidus, Blitzball Star (alternate printing)
const TIDUS_BLITZBALL_STAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TIDUS_BLITZBALL_STAR,
    1,
    "cfe3a1fa-d885-48c0-aa47-bfd8d340bf8e",
    "Nakamura8",
);

// FIN 513 — Ultimecia, Time Sorceress // Ultimecia, Omnipotent (alternate printing)
const ULTIMECIA_TIME_SORCERESS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ULTIMECIA_TIME_SORCERESS,
    1,
    "e7922b5a-6e75-4910-93ad-37e322073d35",
    "Mikio Masuda",
);

// FIN 514 — Vivi Ornitier (alternate printing)
const VIVI_ORNITIER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &VIVI_ORNITIER,
    2,
    "0cfc4614-f6c1-4247-ab96-5bd41006ad85",
    "Toni Infante",
);

// FIN 515 — The Wandering Minstrel (alternate printing)
const THE_WANDERING_MINSTREL_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_WANDERING_MINSTREL,
    2,
    "fd5de0c8-6af9-4b0a-be9d-19af1e7bd7c3",
    "Thanh Tuấn",
);

// FIN 516 — Xande, Dark Mage
pub(in crate::card::sets) static XANDE_DARK_MAGE: CardRecord = CardRecord::new(
    "Xande, Dark Mage",
    "e11df7e8-e154-4fe2-b8d6-a452d0e0d7d5",
    "Joseph Weston",
    CardRules::new_creature(mana_cost!("{2}{U}{B}"), &["Human", "Wizard"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::menace(),
            AbilityDef::static_ability(
                "Xande gets +1/+1 for each noncreature, nonland card in your \
                 graveyard.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                                    CardType::Creature,
                                )),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                                    CardType::Land,
                                )),
                            ]),
                            &[ZoneKind::Graveyard],
                            PlayerRelation::You,
                        )),
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                                    CardType::Creature,
                                )),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                                    CardType::Land,
                                )),
                            ]),
                            &[ZoneKind::Graveyard],
                            PlayerRelation::You,
                        )),
                    ),
                },
            ),
        ]),
);

// FIN 517 — Yuna, Hope of Spira (alternate printing)
const YUNA_HOPE_OF_SPIRA_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &YUNA_HOPE_OF_SPIRA,
    2,
    "cf41b2a8-0f8f-48fd-a430-056a2f669615",
    "NINNIN",
);

// FIN 518 — Zidane, Tantalus Thief (alternate printing)
const ZIDANE_TANTALUS_THIEF_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ZIDANE_TANTALUS_THIEF,
    2,
    "2eaffe7a-0e04-4c19-942b-695a0c350ef0",
    "Eiji Kaneda",
);

// FIN 519 — Aerith Gainsborough (alternate printing)
const AERITH_GAINSBOROUGH_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &AERITH_GAINSBOROUGH,
    3,
    "024a8033-fb41-40a4-b71d-0aa533e0f378",
    "Syutsuri",
);

// FIN 520 — Cloud, Midgar Mercenary (alternate printing)
const CLOUD_MIDGAR_MERCENARY_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &CLOUD_MIDGAR_MERCENARY,
    3,
    "f0b73ab0-b8eb-4a6f-a8d9-25d56fdf740a",
    "Maji",
);

// FIN 521 — Dion, Bahamut's Dominant // Bahamut, Warden of Light (alternate printing)
const DION_BAHAMUT_S_DOMINANT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &DION_BAHAMUT_S_DOMINANT,
    3,
    "9f347fa7-301f-4bc5-954c-b910a840a23b",
    "Maji",
);

// FIN 522 — Gogo, Master of Mimicry (alternate printing)
const GOGO_MASTER_OF_MIMICRY_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &GOGO_MASTER_OF_MIMICRY,
    3,
    "8cbeb01f-965f-4aa2-9e75-31718cc574e3",
    "Ryuichi Sakuma",
);

// FIN 523 — Jill, Shiva's Dominant // Shiva, Warden of Ice (alternate printing)
const JILL_SHIVA_S_DOMINANT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &JILL_SHIVA_S_DOMINANT,
    3,
    "bbd46c0d-cd9d-4e48-b6bf-f619e141100c",
    "Rika Suzuki",
);

// FIN 524 — Ardyn, the Usurper (alternate printing)
const ARDYN_THE_USURPER_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &ARDYN_THE_USURPER,
    4,
    "34bd7a66-afb1-4a6b-80b3-fce1bb361d88",
    "Rorubei",
);

// FIN 525 — Cecil, Dark Knight // Cecil, Redeemed Paladin (alternate printing)
const CECIL_DARK_KNIGHT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &CECIL_DARK_KNIGHT,
    3,
    "7d95f911-91c3-41ee-a709-c579c723eede",
    "Misei Ito",
);

// FIN 526 — Fang, Fearless l'Cie (alternate printing)
const FANG_FEARLESS_L_CIE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &FANG_FEARLESS_L_CIE,
    3,
    "a8bfc03f-227c-4a5d-b172-067ea5860bf9",
    "ikeda_cpt",
);

// FIN 526b — Ragnarok, Divine Deliverance (alternate printing)
const RAGNAROK_DIVINE_DELIVERANCE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &RAGNAROK_DIVINE_DELIVERANCE,
    3,
    "a951cc35-bd29-467f-b28c-5d57340e7450",
    "ikeda_cpt",
);

// FIN 527 — Sephiroth, Fabled SOLDIER // Sephiroth, One-Winged Angel (alternate printing)
const SEPHIROTH_FABLED_SOLDIER_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &SEPHIROTH_FABLED_SOLDIER,
    4,
    "db95465a-2a58-44d7-9439-edd18c9505f0",
    "Maji",
);

// FIN 528 — Vincent Valentine // Galian Beast (alternate printing)
const VINCENT_VALENTINE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &VINCENT_VALENTINE,
    3,
    "15ea1113-7360-462d-91b8-22d5110cbf5a",
    "Murakami Hisashi",
);

// FIN 529 — Zenos yae Galvus // Shinryu, Transcendent Rival (alternate printing)
const ZENOS_YAE_GALVUS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &ZENOS_YAE_GALVUS,
    3,
    "8e3b09a9-631d-4174-8390-a82db0a3d68d",
    "Susumu Kuroi",
);

// FIN 530 — Clive, Ifrit's Dominant // Ifrit, Warden of Inferno (alternate printing)
const CLIVE_IFRIT_S_DOMINANT_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &CLIVE_IFRIT_S_DOMINANT,
    4,
    "57f83eb3-d9d3-4565-b898-4ba1da768172",
    "Murakami Hisashi",
);

// FIN 531 — Firion, Wild Rose Warrior (alternate printing)
const FIRION_WILD_ROSE_WARRIOR_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &FIRION_WILD_ROSE_WARRIOR,
    3,
    "6687a6e8-f674-4db8-81eb-5ad28b7890f7",
    "Kato Ayaka",
);

// FIN 532 — Prompto Argentum (alternate printing)
const PROMPTO_ARGENTUM_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &PROMPTO_ARGENTUM,
    3,
    "516105c2-462e-44e8-b412-5e9eb27ba040",
    "Kato Ayaka",
);

// FIN 533 — Raubahn, Bull of Ala Mhigo (alternate printing)
const RAUBAHN_BULL_OF_ALA_MHIGO_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &RAUBAHN_BULL_OF_ALA_MHIGO,
    3,
    "3afa1696-e242-469f-b3ae-99109c4b36e0",
    "Penekor",
);

// FIN 534 — Seifer Almasy (alternate printing)
const SEIFER_ALMASY_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SEIFER_ALMASY,
    3,
    "0a38194e-44bb-4f79-9797-03cb733808d0",
    "Rika Suzuki",
);

// FIN 535 — Vaan, Street Thief (alternate printing)
const VAAN_STREET_THIEF_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &VAAN_STREET_THIEF,
    3,
    "9cbdcfad-dd4f-4405-9379-eb11c6e4d4f5",
    "Kato Ayaka",
);

// FIN 536 — Tifa Lockhart (alternate printing)
const TIFA_LOCKHART_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &TIFA_LOCKHART,
    3,
    "17e88e4e-d8f0-4598-bfd4-006469c6f26e",
    "Yoshiro Ambe",
);

// FIN 537 — Vanille, Cheerful l'Cie (alternate printing)
const VANILLE_CHEERFUL_L_CIE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &VANILLE_CHEERFUL_L_CIE,
    3,
    "f1083637-7c43-44e0-8029-ad56cca8cc19",
    "ikeda_cpt",
);

// FIN 538 — Balthier and Fran (alternate printing)
const BALTHIER_AND_FRAN_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &BALTHIER_AND_FRAN,
    4,
    "ab6954bc-4fbc-4cb9-9c4d-1f46b3fe0085",
    "S. Makimura",
);

// FIN 539 — Emet-Selch, Unsundered // Hades, Sorcerer of Eld (alternate printing)
const EMET_SELCH_UNSUNDERED_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &EMET_SELCH_UNSUNDERED,
    3,
    "7083fa8a-a841-4c69-9443-af35676a7817",
    "Rorubei",
);

// FIN 540 — Golbez, Crystal Collector (alternate printing)
const GOLBEZ_CRYSTAL_COLLECTOR_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &GOLBEZ_CRYSTAL_COLLECTOR,
    3,
    "de4c9b6b-27be-45b6-9642-49471aaddcb9",
    "Tetsu Kurosawa",
);

// FIN 541 — Hope Estheim (alternate printing)
const HOPE_ESTHEIM_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &HOPE_ESTHEIM,
    3,
    "0b3ba36b-4a15-425c-98ae-37a10b3772bc",
    "Shiyu",
);

// FIN 542 — Joshua, Phoenix's Dominant // Phoenix, Warden of Fire (alternate printing)
const JOSHUA_PHOENIX_S_DOMINANT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &JOSHUA_PHOENIX_S_DOMINANT,
    3,
    "b818aec4-f5a1-411f-ae19-85787cad9032",
    "Susumu Kuroi",
);

// FIN 543 — Kefka, Court Mage // Kefka, Ruler of Ruin (alternate printing)
const KEFKA_COURT_MAGE_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &KEFKA_COURT_MAGE,
    4,
    "21977a0a-7e76-4e1f-9c42-dcc4d63961a1",
    "Rorubei",
);

// FIN 544 — Kuja, Genome Sorcerer // Trance Kuja, Fate Defied (alternate printing)
const KUJA_GENOME_SORCERER_ALTERNATE_6: PrintingRecord = PrintingRecord::alternate(
    &KUJA_GENOME_SORCERER,
    6,
    "d673228a-eec8-4f6e-82fd-10db909ccee2",
    "Masateru Ikeda & Robert Cornelius",
);

// FIN 544† — Kuja, Genome Sorcerer // Trance Kuja, Fate Defied (alternate printing)
const KUJA_GENOME_SORCERER_ALTERNATE_7: PrintingRecord = PrintingRecord::alternate(
    &KUJA_GENOME_SORCERER,
    7,
    "65530632-5b09-4359-921b-a6d094937aff",
    "Masateru Ikeda & Robert Cornelius",
);

// FIN 545 — Lightning, Army of One (alternate printing)
const LIGHTNING_ARMY_OF_ONE_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &LIGHTNING_ARMY_OF_ONE,
    4,
    "2b760674-b6cc-4730-8c88-4e627f60589c",
    "Koji Nishino",
);

// FIN 546 — Noctis, Prince of Lucis (alternate printing)
const NOCTIS_PRINCE_OF_LUCIS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &NOCTIS_PRINCE_OF_LUCIS,
    3,
    "49543cd4-d976-4d3b-a7d2-6acfbb28b1b5",
    "Kato Ayaka",
);

// FIN 547 — Squall, SeeD Mercenary (alternate printing)
const SQUALL_SEED_MERCENARY_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SQUALL_SEED_MERCENARY,
    3,
    "51df7137-8bfc-4140-b436-88834bd78b73",
    "Kato Ayaka",
);

// FIN 548 — The Wandering Minstrel (alternate printing)
const THE_WANDERING_MINSTREL_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &THE_WANDERING_MINSTREL,
    3,
    "fe9cd860-1b4e-48ec-be30-e0ec4ca75e26",
    "Penekor",
);

// FIN 549 — Yuna, Hope of Spira (alternate printing)
const YUNA_HOPE_OF_SPIRA_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &YUNA_HOPE_OF_SPIRA,
    3,
    "ed59c09c-b972-4470-bcae-c79866825941",
    "osamu",
);

// FIN 550 — Zidane, Tantalus Thief (alternate printing)
const ZIDANE_TANTALUS_THIEF_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &ZIDANE_TANTALUS_THIEF,
    3,
    "439b31a8-d6e2-4db4-bac3-c02e2eb11719",
    "Canata Katana",
);

// FIN 551 — Traveling Chocobo (alternate printing)
const TRAVELING_CHOCOBO_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &TRAVELING_CHOCOBO,
    3,
    "71b97e69-f198-41ec-9385-015ec2f0160f",
    "Toni Infante",
);

// FIN 551a — Traveling Chocobo (alternate printing)
const TRAVELING_CHOCOBO_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &TRAVELING_CHOCOBO,
    4,
    "81690ce7-0651-40e4-9769-baebe10e2cbf",
    "Toni Infante",
);

// FIN 551b — Traveling Chocobo (alternate printing)
const TRAVELING_CHOCOBO_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &TRAVELING_CHOCOBO,
    5,
    "ac3db85f-8cc3-4869-a555-882b62a1726b",
    "Toni Infante",
);

// FIN 551c — Traveling Chocobo
pub(in crate::card::sets) static TRAVELING_CHOCOBO: CardRecord = CardRecord::new(
    "Traveling Chocobo",
    "156cfd45-1556-4804-becf-039cfff7de3d",
    "Toni Infante",
// Three mana for a body, a land engine, and a Panharmonicon that only
    // reads lands and its own kind -- which in a deck built for it is most
    // of what enters.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Bird"], 3, 2)
        .with_abilities(&[
            AbilityDef::static_ability(
                "You may look at the top card of your library any time.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::You)),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayLookAtTopOfLibrary),
                },
            ),
            AbilityDef::static_ability(
                "You may play lands and cast Bird spells from the top of your library.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::You)),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromTopOfLibrary {
                            // Two permissions rather than one: the printed sentence names two kinds of
                            // play, and the restriction each carries is a single action and a single
                            // predicate. Lands cost nothing beyond the land drop; a Bird pays its own
                            // mana cost, since nothing here says otherwise.
                            restriction: PlayRestrictionDef::new(
                                PlayActionMatcherDef::PlayLand,
                                ObjectPredicateDef::HasType(CardType::Land),
                            ),
                            cost: TopOfLibraryCostDef::Printed,
                        }),
                        AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromTopOfLibrary {
                            restriction: PlayRestrictionDef::new(
                                PlayActionMatcherDef::CastSpell,
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Bird")),
                            ),
                            cost: TopOfLibraryCostDef::Printed,
                        }),
                    ]),
                },
            ),
            AbilityDef::static_ability(
                "If a land or Bird you control entering the battlefield causes a triggered ability of a \
                 permanent you control to trigger, that ability triggers an additional time.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Controller,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::ModifyTriggers(
                        &TriggerModificationDef {
                            cause: TriggerEventDef::zone_changed(
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::AnyOf(&[
                                        ObjectPredicateDef::HasType(CardType::Land),
                                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Bird")),
                                    ]),
                                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                                ]),
                                None,
                                Some(ZoneKind::Battlefield),
                            ),
                            permanent: Some(ObjectPredicateDef::Any),
                            kind: TriggerModificationKindDef::Additional,
                        },
                    )),
                },
            ),
        ]),
);

// FIN 551d — Traveling Chocobo (alternate printing)
const TRAVELING_CHOCOBO_ALTERNATE_6: PrintingRecord = PrintingRecord::alternate(
    &TRAVELING_CHOCOBO,
    6,
    "e40d85a5-ecfa-4f95-920b-404eb448324f",
    "Toni Infante",
);

// FIN 551f — Traveling Chocobo (alternate printing)
const TRAVELING_CHOCOBO_ALTERNATE_7: PrintingRecord = PrintingRecord::alternate(
    &TRAVELING_CHOCOBO,
    7,
    "6ce8744a-ede3-4662-968a-360eb6639f08",
    "Toni Infante",
);

// FIN 552 — Cloud, Planet's Champion (alternate printing)
const CLOUD_PLANET_S_CHAMPION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CLOUD_PLANET_S_CHAMPION,
    1,
    "a6d58067-337d-43dc-b4a3-c6acc701d450",
    "Magali Villeneuve",
);

// FIN 553 — Sephiroth, Planet's Heir (alternate printing)
const SEPHIROTH_PLANET_S_HEIR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SEPHIROTH_PLANET_S_HEIR,
    1,
    "abd73e52-62f0-4e89-9dc6-90ff0bc2a9b7",
    "Magali Villeneuve",
);

// FIN 554 — Beatrix, Loyal General (alternate printing)
const BEATRIX_LOYAL_GENERAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BEATRIX_LOYAL_GENERAL,
    1,
    "9da83b07-4978-4af7-be51-8aa8f35ec0bb",
    "Bachzim",
);

// FIN 555 — Rosa, Resolute White Mage (alternate printing)
const ROSA_RESOLUTE_WHITE_MAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ROSA_RESOLUTE_WHITE_MAGE,
    1,
    "c44e0ad6-9f44-4e4c-8c3b-3ee99409a740",
    "Christian Angel",
);

// FIN 556 — Ultimecia, Temporal Threat (alternate printing)
const ULTIMECIA_TEMPORAL_THREAT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ULTIMECIA_TEMPORAL_THREAT,
    1,
    "b64e4475-f1c6-4d85-b42f-6d84d06855bb",
    "Bachzim",
);

// FIN 557 — Deadly Embrace
pub(in crate::card::sets) static DEADLY_EMBRACE: CardRecord = CardRecord::new(
    "Deadly Embrace",
    "a11cb85c-85dd-435c-8303-4d0d18bdb1e9",
    "Lius Lasahido",
    CardRules::new_sorcery(mana_cost!("{3}{B}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Destroy target creature an opponent controls. Then draw a \
             card for each creature that died this turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
                abilities::draw_cards(ValueDef::CreaturesDiedThisTurn),
            ]),
        ),
    ]),
);

// FIN 558 — Seymour Flux (alternate printing)
const SEYMOUR_FLUX_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SEYMOUR_FLUX,
    1,
    "c5fdc78e-0815-443c-8c26-35387b6f4f37",
    "K-SUWABE",
);

// FIN 559 — Judgment Bolt
pub(in crate::card::sets) static JUDGMENT_BOLT: CardRecord = CardRecord::new(
    "Judgment Bolt",
    "05b04b06-9271-4a28-a60e-287df0d1a4d1",
    "Billy Christian",
    CardRules::new_instant(mana_cost!("{3}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Judgment Bolt deals 5 damage to target creature and X damage \
         to that creature's controller, where X is the number of \
         Equipment you control.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::damage_simultaneously(&[
            DamageAssignmentDef::from(
                ObjectRefDef::Source,
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(5),
            ),
            DamageAssignmentDef::from(
                ObjectRefDef::Source,
                EffectRecipientDef::player(PlayerRefDef::ControllerOf(ObjectRefDef::Target(
                    TargetIndex::PRIMARY,
                ))),
                ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Equipment")),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
            ),
        ]),
    )]),
);

// FIN 560 — Lightning, Security Sergeant (alternate printing)
const LIGHTNING_SECURITY_SERGEANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LIGHTNING_SECURITY_SERGEANT,
    1,
    "4ad4dce3-6e43-4528-b570-85547d03164e",
    "Ramza Psyru",
);

// FIN 561 — Xande, Dark Mage (alternate printing)
const XANDE_DARK_MAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &XANDE_DARK_MAGE,
    1,
    "1caed9a8-b73b-470e-b9d8-8c3b7cac3eee",
    "Joseph Weston",
);

// FIN 562 — Magitek Scythe
// Audit: unsupported — Needs an attack requirement that the equipped creature be blocked by at least one creature if able; MustBeBlockedBy requires every matching creature to block, and minimum-blocker restrictions alone do not require a block.
pub(in crate::card::sets) static MAGITEK_SCYTHE: CardRecord = CardRecord::new(
    "Magitek Scythe",
    "8b691d42-3498-4d47-9a46-f7c376df8886",
    "Thanh Tuấn",
    CardRules::unsupported(),
);

// FIN 563 — Ultima Weapon
pub(in crate::card::sets) static ULTIMA_WEAPON: CardRecord = CardRecord::new(
    "Ultima Weapon",
    "b9162d08-a6ba-4e6e-b82c-9b092bd781dd",
    "Fariba Khamseh",
    CardRules::new_artifact(mana_cost!("{7}"))
        .with_subtypes(&["Equipment"])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered_with_targets(
                "Whenever equipped creature attacks, destroy target creature \
                 an opponent controls.",
                TriggerEventDef::attacks(ObjectPredicateDef::AttachedToSource),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
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
            AbilityDef::static_ability(
                "Equipped creature gets +7/+7.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(7),
                        ValueDef::Constant(7),
                    ),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{7}"))], "Equip {7}"),
        ]),
);

// FIN 564 — Cloud, Midgar Mercenary (alternate printing)
const CLOUD_MIDGAR_MERCENARY_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &CLOUD_MIDGAR_MERCENARY,
    4,
    "b358dadc-9cc9-4baf-afec-3b50d0822609",
    "Inuchiyo Meimaru",
);

// FIN 565 — Stiltzkin, Moogle Merchant (alternate printing)
const STILTZKIN_MOOGLE_MERCHANT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &STILTZKIN_MOOGLE_MERCHANT,
    3,
    "c7d9c54d-edec-40a8-82d8-b12cee7daa21",
    "Yukihiro Maruo",
);

// FIN 566 — A Realm Reborn (alternate printing)
const A_REALM_REBORN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &A_REALM_REBORN,
    2,
    "13f4b6f3-736f-4f33-9d05-9bfd0b05b214",
    "Susumu Kuroi",
);

// FIN 567 — Tifa Lockhart (alternate printing)
const TIFA_LOCKHART_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &TIFA_LOCKHART,
    4,
    "39a038ed-9ed9-4a32-9613-989e4809ec94",
    "Yoshiro Ambe",
);

// FIN 568 — Traveling Chocobo (alternate printing)
const TRAVELING_CHOCOBO_ALTERNATE_8: PrintingRecord = PrintingRecord::alternate(
    &TRAVELING_CHOCOBO,
    8,
    "f58e882a-c59f-4f72-91b5-0291f8360a2c",
    "Ishikawa Kenta",
);

// FIN 569 — Choco, Seeker of Paradise (alternate printing)
const CHOCO_SEEKER_OF_PARADISE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CHOCO_SEEKER_OF_PARADISE,
    2,
    "1ce688fa-1fdb-4acd-b9b0-bf7325a6b8e0",
    "Kemonomichi",
);

// FIN 570 — Vivi Ornitier (alternate printing)
const VIVI_ORNITIER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &VIVI_ORNITIER,
    3,
    "25ef2d44-c261-4511-b96f-85e7a291e318",
    "Ryanroro",
);

// FIN 571 — Yuna, Hope of Spira (alternate printing)
const YUNA_HOPE_OF_SPIRA_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &YUNA_HOPE_OF_SPIRA,
    4,
    "d9228c15-06d1-4701-8619-1ad08d63275a",
    "Yuichi Murakami",
);

// FIN 572 — Plains (alternate printing)
const PLAINS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    3,
    "dec72b07-8f41-4d42-a455-794bf106302c",
    "Jonas De Ro",
);

// FIN 573 — Island (alternate printing)
const ISLAND_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    3,
    "46255a65-b735-4888-85db-3ab2ab3d903c",
    "Eddie Mendoza",
);

// FIN 574 — Swamp (alternate printing)
const SWAMP_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    3,
    "1176ebbf-4130-4e4e-ad49-65101a7357b4",
    "Domenico Cava",
);

// FIN 575 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    3,
    "0cef98b7-d54a-456e-8240-1c5af3a72a04",
    "Domenico Cava",
);

// FIN 576 — Forest (alternate printing)
const FOREST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    3,
    "2036f825-ef57-4a40-b45f-0668d9c8ec6a",
    "Leon Tukker",
);

// FIN 577 — Y'shtola Rhul (alternate printing)
const Y_SHTOLA_RHUL_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &Y_SHTOLA_RHUL,
    2,
    "6d24956c-b123-4682-9f89-6821ae45fc00",
    "Yusuke Mogi",
);

// FIN 578 — Phoenix Down (alternate printing)
const PHOENIX_DOWN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PHOENIX_DOWN,
    1,
    "de3b9b08-f430-4f01-8a42-3a9f1415eee0",
    "John Severin Brassell",
);

// FIN 579 — White Auracite (alternate printing)
const WHITE_AURACITE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WHITE_AURACITE,
    1,
    "b8921d64-fdd3-4cdd-9687-983c8d6e0f72",
    "Magali Villeneuve",
);

// FIN 580 — Zack Fair (alternate printing)
const ZACK_FAIR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ZACK_FAIR,
    2,
    "90ab9143-23e0-437e-bc51-a46f3497c195",
    "Yoshio Sugiura",
);

// FIN 581 — Astrologian's Planisphere
pub(in crate::card::sets) static ASTROLOGIAN_S_PLANISPHERE: CardRecord = CardRecord::new(
    "Astrologian's Planisphere",
    "a0f6e2d7-58b5-4a7d-8c42-e25185cd173f",
    "Josephine Chang",
// Two mana for a 1/1 that grows on the turns a blue deck was having
    // anyway, and an Equipment left over when it dies.
    CardRules::new_artifact(mana_cost!("{1}{U}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::job_select(),
            AbilityDef::static_ability(
                "Equipped creature is a Wizard in addition to its other types and has \"Whenever you \
                 cast a noncreature spell and whenever you draw your third card each turn, put a +1/+1 \
                 counter on this creature.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::Characteristic(CharacteristicOperationDef::CreatureTypes(
                            SetOperationDef::Add(CreatureTypeSetDef::named(&["Wizard"])),
                        )),
                        // Granted to the equipped creature, so "this creature" is the creature
                        // rather than the Equipment: the counter goes where the ability lives.
                        AppliedEffectDef::add_ability(&AbilityDef::triggered(
                            "Whenever you cast a noncreature spell and whenever you draw your third card each turn, put \
                             a +1/+1 counter on this creature.",
                            // Two events, one clause, one counter each: a noncreature spell, and the
                            // third card of the turn however it was drawn. The Hero's own draw step
                            // counts toward the third, which is why the card wants a turn with two
                            // cantrips in it rather than a big draw spell.
                            TriggerEventDef::AnyOf(&[
                                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::NoncreatureSpell,
                                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                                ])),
                                TriggerEventDef::DrewCard(DrawEventMatcherDef::nth_each_turn(PlayerRelation::You, 3)),
                            ]),
                            EffectDef::AddCounters {
                                object: EffectRecipientDef::Source,
                                kind: CounterKind::PlusOnePlusOne,
                                amount: ValueDef::Constant(1),
                            },
                        )),
                    ]),
                },
            ),
            // The flavour name in front of the cost is the whole of what "Diana —"
            // adds: it is an ordinary equip ability underneath.
            abilities::equip(&[CostDef::Mana(mana_cost!("{2}"))], "Diana — Equip {2}"),
        ]),
);

// FIN 582 — Sage's Nouliths (alternate printing)
const SAGE_S_NOULITHS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SAGE_S_NOULITHS,
    1,
    "ace9baac-4aa6-49ef-b85d-514de6e77043",
    "Justyna Dura",
);

// FIN 583 — Circle of Power (alternate printing)
const CIRCLE_OF_POWER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CIRCLE_OF_POWER,
    1,
    "b6dc1f5a-a6cc-4ab4-8bb9-e216e24ca735",
    "Josephine Chang",
);

// FIN 584 — Barret Wallace (alternate printing)
const BARRET_WALLACE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BARRET_WALLACE,
    2,
    "ea5b9fe8-2561-4176-adfd-3fb790250bbc",
    "Patrik Hell",
);

// FIN 585 — Laughing Mad (alternate printing)
const LAUGHING_MAD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LAUGHING_MAD,
    1,
    "9268ccdb-0b5b-4624-9443-f7b5330ec71f",
    "RARE ENGINE",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &SUMMON_BAHAMUT,
    &ULTIMA_ORIGIN_OF_OBLIVION,
    &ADELBERT_STEINER,
    &AERITH_GAINSBOROUGH,
    &AERITH_RESCUE_MISSION,
    &AMBROSIA_WHITEHEART,
    &ASHE_PRINCESS_OF_DALMASCA,
    &AURON_S_INSPIRATION,
    &BATTLE_MENU,
    &CLOUD_MIDGAR_MERCENARY,
    &CLOUDBOUND_MOOGLE,
    &COEURL,
    &CRYSTAL_FRAGMENTS,
    &THE_CRYSTAL_S_CHOSEN,
    &DELIVERY_MOOGLE,
    &DION_BAHAMUT_S_DOMINANT,
    &DRAGOON_S_LANCE,
    &DWARVEN_CASTLE_GUARD,
    &FATE_OF_THE_SUN_CRYST,
    &FROM_FATHER_TO_SON,
    &G_RAHA_TIA,
    &GAELICAT,
    &MACHINIST_S_ARSENAL,
    &MAGITEK_ARMOR,
    &MAGITEK_INFANTRY,
    &MINWU_WHITE_MAGE,
    &MOOGLES_VALOR,
    &PALADIN_S_ARMS,
    &PHOENIX_DOWN,
    &RESTORATION_MAGIC,
    &SIDEQUEST_CATCH_A_FISH,
    &SLASH_OF_LIGHT,
    &SNOW_VILLIERS,
    &STILTZKIN_MOOGLE_MERCHANT,
    &SUMMON_CHOCO_MOG,
    &SUMMON_KNIGHTS_OF_ROUND,
    &SUMMON_PRIMAL_GARUDA,
    &ULTIMA,
    &VENAT_HEART_OF_HYDAELYN,
    &WEAPONS_VENDOR,
    &WHITE_AURACITE,
    &WHITE_MAGE_S_STAFF,
    &THE_WIND_CRYSTAL,
    &YOU_RE_NOT_ALONE,
    &ZACK_FAIR,
    &CARGO_SHIP,
    &COMBAT_TUTORIAL,
    &DRAGOON_S_WYVERN,
    &DREAMS_OF_LAGUNA,
    &EDGAR_KING_OF_FIGARO,
    &EJECT,
    &ETHER,
    &GOGO_MASTER_OF_MIMICRY,
    &ICE_FLAN,
    &ICE_MAGIC,
    &IL_MHEG_PIXIE,
    &JILL_SHIVA_S_DOMINANT,
    &LOUISOIX_S_SACRIFICE,
    &THE_LUNAR_WHALE,
    &MAGIC_DAMPER,
    &MATOYA_ARCHON_ELDER,
    &MEMORIES_RETURNING,
    &THE_PRIMA_VISTA,
    &QIQIRN_MERCHANT,
    &QUISTIS_TREPE,
    &RELM_S_SKETCHING,
    &RETRIEVE_THE_ESPER,
    &ROOK_TURRET,
    &SAGE_S_NOULITHS,
    &SAHAGIN,
    &SCORPION_SENTINEL,
    &SIDEQUEST_CARD_COLLECTION,
    &SLEEP_MAGIC,
    &STOLEN_UNIFORM,
    &STUCK_IN_SUMMONER_S_SANCTUM,
    &SUMMON_LEVIATHAN,
    &SUMMON_SHIVA,
    &SWALLOWED_BY_LEVIATHAN,
    &THIEF_S_KNIFE,
    &TRAVEL_THE_OVERWORLD,
    &ULTROS_OBNOXIOUS_OCTOPUS,
    &VALKYRIE_AERIAL_UNIT,
    &THE_WATER_CRYSTAL,
    &Y_SHTOLA_RHUL,
    &AHRIMAN,
    &AL_BHED_SALVAGERS,
    &ARDYN_THE_USURPER,
    &BLACK_MAGE_S_ROD,
    &CECIL_DARK_KNIGHT,
    &CIRCLE_OF_POWER,
    &CORNERED_BY_BLACK_MAGES,
    &DARK_KNIGHT_S_GREATSWORD,
    &THE_DARKNESS_CRYSTAL,
    &DEMON_WALL,
    &EVIL_REAWAKENED,
    &FANG_FEARLESS_L_CIE,
    &RAGNAROK_DIVINE_DELIVERANCE,
    &FIGHT_ON,
    &THE_FINAL_DAYS,
    &GAIUS_VAN_BAELSAR,
    &HECTEYES,
    &JECHT_RELUCTANT_GUARDIAN,
    &KAIN_TRAITOROUS_DRAGOON,
    &MALBORO,
    &NAMAZU_TRADER,
    &NINJA_S_BLADES,
    &OVERKILL,
    &PHANTOM_TRAIN,
    &POISON_THE_WATERS,
    &QUTRUB_FORAYER,
    &RENO_AND_RUDE,
    &RESENTFUL_REVELATION,
    &SEPHIROTH_FABLED_SOLDIER,
    &SEPHIROTH_S_INTERVENTION,
    &SHAMBLING_CIE_TH,
    &SHINRA_REINFORCEMENTS,
    &SIDEQUEST_HUNT_THE_MARK,
    &SUMMON_ANIMA,
    &SUMMON_PRIMAL_ODIN,
    &TONBERRY,
    &UNDERCITY_DIRE_RAT,
    &VAYNE_S_TREACHERY,
    &VINCENT_VALENTINE,
    &VINCENT_S_LIMIT_BREAK,
    &ZENOS_YAE_GALVUS,
    &ZODIARK_UMBRAL_GOD,
    &BARRET_WALLACE,
    &BLAZING_BOMB,
    &CALL_THE_MOUNTAIN_CHOCOBO,
    &CHOCO_COMET,
    &CLIVE_IFRIT_S_DOMINANT,
    &CORAL_SWORD,
    &THE_FIRE_CRYSTAL,
    &FIRE_MAGIC,
    &FIRION_WILD_ROSE_WARRIOR,
    &FREYA_CRESCENT,
    &GILGAMESH_MASTER_AT_ARMS,
    &HASTE_MAGIC,
    &HILL_GIGAS,
    &ITEM_SHOPKEEP,
    &LAUGHING_MAD,
    &LIGHT_OF_JUDGMENT,
    &MYSIDIAN_ELDER,
    &NIBELHEIM_AFLAME,
    &OPERA_LOVE_SONG,
    &PROMPTO_ARGENTUM,
    &QUEEN_BRAHNE,
    &RANDOM_ENCOUNTER,
    &RAUBAHN_BULL_OF_ALA_MHIGO,
    &RED_MAGE_S_RAPIER,
    &SABOTENDER,
    &SAMURAI_S_KATANA,
    &SANDWORM,
    &SEIFER_ALMASY,
    &SELF_DESTRUCT,
    &SIDEQUEST_PLAY_BLITZBALL,
    &SORCERESS_S_SCHEMES,
    &SUMMON_BRYNHILDR,
    &SUMMON_ESPER_RAMUH,
    &SUMMON_G_F_CERBERUS,
    &SUMMON_G_F_IFRIT,
    &SUPLEX,
    &THUNDER_MAGIC,
    &TRIPLE_TRIAD,
    &UNEXPECTED_REQUEST,
    &VAAN_STREET_THIEF,
    &WARRIOR_S_SWORD,
    &ZELL_DINCHT,
    &AIRSHIP_CRASH,
    &ANCIENT_ADAMANTOISE,
    &BALAMB_T_REXAUR,
    &BARD_S_BOW,
    &BARTZ_AND_BOKO,
    &BLITZBALL_SHOT,
    &CACTUAR,
    &CHOCOBO_KICK,
    &CHOCOBO_RACETRACK,
    &CLASH_OF_THE_EIKONS,
    &COLISEUM_BEHEMOTH,
    &COMMUNE_WITH_BEAVERS,
    &DIAMOND_WEAPON,
    &THE_EARTH_CRYSTAL,
    &ESPER_ORIGINS,
    &GALUF_S_FINAL_ACT,
    &GIGANTOAD,
    &GOOBBUE_GARDENER,
    &GRAN_PULSE_OCHU,
    &GYSAHL_GREENS,
    &JUMBO_CACTUAR,
    &LOPORRIT_SCOUT,
    &PRISHE_S_WANDERINGS,
    &QUINA_QU_GOURMET,
    &REACH_THE_HORIZON,
    &A_REALM_REBORN,
    &RIDE_THE_SHOOPUF,
    &RYDIA_S_RETURN,
    &SAZH_KATZROY,
    &SAZH_S_CHOCOBO,
    &SIDEQUEST_RAISE_A_CHOCOBO,
    &SUMMON_FAT_CHOCOBO,
    &SUMMON_FENRIR,
    &SUMMON_TITAN,
    &SUMMONER_S_GRIMOIRE,
    &TIFA_LOCKHART,
    &TIFA_S_LIMIT_BREAK,
    &TORGAL_A_FINE_HOUND,
    &TOWN_GREETER,
    &VANILLE_CHEERFUL_L_CIE,
    &ABSOLUTE_VIRTUE,
    &BALTHIER_AND_FRAN,
    &BLACK_WALTZ_NO_3,
    &CHOCO_SEEKER_OF_PARADISE,
    &CID_TIMELESS_ARTIFICER,
    &CLOUD_OF_DARKNESS,
    &EMET_SELCH_UNSUNDERED,
    &THE_EMPEROR_OF_PALAMECIA,
    &EXDEATH_VOID_WARLOCK,
    &GARLAND_KNIGHT_OF_CORNELIA,
    &GARNET_PRINCESS_OF_ALEXANDRIA,
    &GIOTT_KING_OF_THE_DWARVES,
    &GLADIOLUS_AMICITIA,
    &GOLBEZ_CRYSTAL_COLLECTOR,
    &HOPE_ESTHEIM,
    &IGNIS_SCIENTIA,
    &JENOVA_ANCIENT_CALAMITY,
    &JOSHUA_PHOENIX_S_DOMINANT,
    &JUDGE_MAGISTER_GABRANTH,
    &KEFKA_COURT_MAGE,
    &KUJA_GENOME_SORCERER,
    &LIGHTNING_ARMY_OF_ONE,
    &LOCKE_COLE,
    &NOCTIS_PRINCE_OF_LUCIS,
    &OMEGA_HEARTLESS_EVOLUTION,
    &RINOA_HEARTILLY,
    &RUFUS_SHINRA,
    &RYDIA_SUMMONER_OF_MIST,
    &SERAH_FARRON,
    &SHANTOTTO_TACTICIAN_MAGICIAN,
    &SIN_SPIRA_S_PUNISHMENT,
    &SQUALL_SEED_MERCENARY,
    &TELLAH_GREAT_SAGE,
    &TERRA_MAGICAL_ADEPT,
    &TIDUS_BLITZBALL_STAR,
    &ULTIMECIA_TIME_SORCERESS,
    &VIVI_ORNITIER,
    &THE_WANDERING_MINSTREL,
    &YUNA_HOPE_OF_SPIRA,
    &ZIDANE_TANTALUS_THIEF,
    &ADVENTURER_S_AIRSHIP,
    &AETTIR_AND_PRIWEN,
    &BLITZBALL,
    &BUSTER_SWORD,
    &ELIXIR,
    &EXCALIBUR_II,
    &GENJI_GLOVE,
    &INSTANT_RAMEN,
    &IRON_GIANT,
    &LION_HEART,
    &LUNATIC_PANDORA,
    &MAGIC_POT,
    &THE_MASAMUNE,
    &MONK_S_FIST,
    &PUPU_UFO,
    &THE_REGALIA,
    &RELENTLESS_X_ATM092,
    &RING_OF_THE_LUCII,
    &WORLD_MAP,
    &ADVENTURER_S_INN,
    &BALAMB_GARDEN_SEED_ACADEMY,
    &BARON_AIRSHIP_KINGDOM,
    &CAPITAL_CITY,
    &CLIVE_S_HIDEAWAY,
    &CROSSROADS_VILLAGE,
    &EDEN_SEAT_OF_THE_SANCTUM,
    &GOHN_TOWN_OF_RUIN,
    &THE_GOLD_SAUCER,
    &GONGAGA_REACTOR_TOWN,
    &GUADOSALAM_FARPLANE_GATEWAY,
    &INSOMNIA_CROWN_CITY,
    &ISHGARD_THE_HOLY_SEE,
    &JIDOOR_ARISTOCRATIC_CAPITAL,
    &LINDBLUM_INDUSTRIAL_REGENCY,
    &MIDGAR_CITY_OF_MAKO,
    &RABANASTRE_ROYAL_CITY,
    &SHARLAYAN_NATION_OF_SCHOLARS,
    &STARTING_TOWN,
    &TRENO_DARK_CITY,
    &VECTOR_IMPERIAL_CAPITAL,
    &WINDURST_FEDERATION_CENTER,
    &ZANARKAND_ANCIENT_METROPOLIS,
    &BEATRIX_LOYAL_GENERAL,
    &ROSA_RESOLUTE_WHITE_MAGE,
    &ULTIMECIA_TEMPORAL_THREAT,
    &SEYMOUR_FLUX,
    &LIGHTNING_SECURITY_SERGEANT,
    &CLOUD_PLANET_S_CHAMPION,
    &SEPHIROTH_PLANET_S_HEIR,
    &XANDE_DARK_MAGE,
    &TRAVELING_CHOCOBO,
    &DEADLY_EMBRACE,
    &JUDGMENT_BOLT,
    &MAGITEK_SCYTHE,
    &ULTIMA_WEAPON,
    &ASTROLOGIAN_S_PLANISPHERE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    ASTROLOGIAN_S_PLANISPHERE_ALTERNATE_1,
    SYNCOPATE_REPRINT,
    DARK_CONFIDANT_REPRINT,
    TRAVELING_CHOCOBO_ALTERNATE_1,
    KUJA_GENOME_SORCERER_ALTERNATE_1,
    PLAINS_REPRINT,
    PLAINS_ALTERNATE_1,
    PLAINS_ALTERNATE_2,
    ISLAND_REPRINT,
    ISLAND_ALTERNATE_1,
    ISLAND_ALTERNATE_2,
    SWAMP_REPRINT,
    SWAMP_ALTERNATE_1,
    SWAMP_ALTERNATE_2,
    MOUNTAIN_REPRINT,
    MOUNTAIN_ALTERNATE_1,
    MOUNTAIN_ALTERNATE_2,
    FOREST_REPRINT,
    FOREST_ALTERNATE_1,
    FOREST_ALTERNATE_2,
    WASTES_REPRINT,
    ISHGARD_THE_HOLY_SEE_ALTERNATE_1,
    JIDOOR_ARISTOCRATIC_CAPITAL_ALTERNATE_1,
    LINDBLUM_INDUSTRIAL_REGENCY_ALTERNATE_1,
    MIDGAR_CITY_OF_MAKO_ALTERNATE_1,
    ZANARKAND_ANCIENT_METROPOLIS_ALTERNATE_1,
    ARDYN_THE_USURPER_ALTERNATE_1,
    KAIN_TRAITOROUS_DRAGOON_ALTERNATE_1,
    SEPHIROTH_FABLED_SOLDIER_ALTERNATE_1,
    CLIVE_IFRIT_S_DOMINANT_ALTERNATE_1,
    BALTHIER_AND_FRAN_ALTERNATE_1,
    LIGHTNING_ARMY_OF_ONE_ALTERNATE_1,
    VIVI_ORNITIER_ALTERNATE_1,
    KEFKA_COURT_MAGE_ALTERNATE_1,
    TERRA_MAGICAL_ADEPT_ALTERNATE_1,
    ULTIMA_ORIGIN_OF_OBLIVION_ALTERNATE_1,
    AMBROSIA_WHITEHEART_ALTERNATE_1,
    MOOGLES_VALOR_ALTERNATE_1,
    STILTZKIN_MOOGLE_MERCHANT_ALTERNATE_1,
    ULTIMA_ALTERNATE_1,
    VENAT_HEART_OF_HYDAELYN_ALTERNATE_1,
    THE_WIND_CRYSTAL_ALTERNATE_1,
    MEMORIES_RETURNING_ALTERNATE_1,
    STOLEN_UNIFORM_ALTERNATE_1,
    THE_WATER_CRYSTAL_ALTERNATE_1,
    DARK_CONFIDANT_ALTERNATE_1,
    THE_DARKNESS_CRYSTAL_ALTERNATE_1,
    ZODIARK_UMBRAL_GOD_ALTERNATE_1,
    THE_FIRE_CRYSTAL_ALTERNATE_1,
    GILGAMESH_MASTER_AT_ARMS_ALTERNATE_1,
    NIBELHEIM_AFLAME_ALTERNATE_1,
    TRIPLE_TRIAD_ALTERNATE_1,
    CLASH_OF_THE_EIKONS_ALTERNATE_1,
    THE_EARTH_CRYSTAL_ALTERNATE_1,
    JUMBO_CACTUAR_ALTERNATE_1,
    A_REALM_REBORN_ALTERNATE_1,
    TORGAL_A_FINE_HOUND_ALTERNATE_1,
    JENOVA_ANCIENT_CALAMITY_ALTERNATE_1,
    OMEGA_HEARTLESS_EVOLUTION_ALTERNATE_1,
    SIN_SPIRA_S_PUNISHMENT_ALTERNATE_1,
    TELLAH_GREAT_SAGE_ALTERNATE_1,
    AETTIR_AND_PRIWEN_ALTERNATE_1,
    BUSTER_SWORD_ALTERNATE_1,
    EXCALIBUR_II_ALTERNATE_1,
    THE_MASAMUNE_ALTERNATE_1,
    BALAMB_GARDEN_SEED_ACADEMY_ALTERNATE_1,
    EDEN_SEAT_OF_THE_SANCTUM_ALTERNATE_1,
    SUMMON_BAHAMUT_ALTERNATE_1,
    CRYSTAL_FRAGMENTS_ALTERNATE_1,
    SUMMON_CHOCO_MOG_ALTERNATE_1,
    SUMMON_KNIGHTS_OF_ROUND_ALTERNATE_1,
    SUMMON_PRIMAL_GARUDA_ALTERNATE_1,
    SUMMON_LEVIATHAN_ALTERNATE_1,
    SUMMON_SHIVA_ALTERNATE_1,
    JECHT_RELUCTANT_GUARDIAN_ALTERNATE_1,
    SUMMON_ANIMA_ALTERNATE_1,
    SUMMON_PRIMAL_ODIN_ALTERNATE_1,
    SUMMON_BRYNHILDR_ALTERNATE_1,
    SUMMON_ESPER_RAMUH_ALTERNATE_1,
    SUMMON_G_F_CERBERUS_ALTERNATE_1,
    SUMMON_G_F_IFRIT_ALTERNATE_1,
    ESPER_ORIGINS_ALTERNATE_1,
    SUMMON_FAT_CHOCOBO_ALTERNATE_1,
    SUMMON_FENRIR_ALTERNATE_1,
    SUMMON_TITAN_ALTERNATE_1,
    AERITH_GAINSBOROUGH_ALTERNATE_1,
    CLOUD_MIDGAR_MERCENARY_ALTERNATE_1,
    DION_BAHAMUT_S_DOMINANT_ALTERNATE_1,
    GOGO_MASTER_OF_MIMICRY_ALTERNATE_1,
    JILL_SHIVA_S_DOMINANT_ALTERNATE_1,
    ARDYN_THE_USURPER_ALTERNATE_2,
    CECIL_DARK_KNIGHT_ALTERNATE_1,
    FANG_FEARLESS_L_CIE_ALTERNATE_1,
    RAGNAROK_DIVINE_DELIVERANCE_ALTERNATE_1,
    SEPHIROTH_FABLED_SOLDIER_ALTERNATE_2,
    VINCENT_VALENTINE_ALTERNATE_1,
    ZENOS_YAE_GALVUS_ALTERNATE_1,
    CLIVE_IFRIT_S_DOMINANT_ALTERNATE_2,
    FIRION_WILD_ROSE_WARRIOR_ALTERNATE_1,
    PROMPTO_ARGENTUM_ALTERNATE_1,
    RAUBAHN_BULL_OF_ALA_MHIGO_ALTERNATE_1,
    SEIFER_ALMASY_ALTERNATE_1,
    VAAN_STREET_THIEF_ALTERNATE_1,
    TIFA_LOCKHART_ALTERNATE_1,
    VANILLE_CHEERFUL_L_CIE_ALTERNATE_1,
    BALTHIER_AND_FRAN_ALTERNATE_2,
    EMET_SELCH_UNSUNDERED_ALTERNATE_1,
    GOLBEZ_CRYSTAL_COLLECTOR_ALTERNATE_1,
    HOPE_ESTHEIM_ALTERNATE_1,
    JOSHUA_PHOENIX_S_DOMINANT_ALTERNATE_1,
    KEFKA_COURT_MAGE_ALTERNATE_2,
    KUJA_GENOME_SORCERER_ALTERNATE_2,
    KUJA_GENOME_SORCERER_ALTERNATE_3,
    LIGHTNING_ARMY_OF_ONE_ALTERNATE_2,
    NOCTIS_PRINCE_OF_LUCIS_ALTERNATE_1,
    SQUALL_SEED_MERCENARY_ALTERNATE_1,
    THE_WANDERING_MINSTREL_ALTERNATE_1,
    YUNA_HOPE_OF_SPIRA_ALTERNATE_1,
    ZIDANE_TANTALUS_THIEF_ALTERNATE_1,
    TRAVELING_CHOCOBO_ALTERNATE_2,
    CID_TIMELESS_ARTIFICER_ALTERNATE_1,
    CID_TIMELESS_ARTIFICER_ALTERNATE_2,
    CID_TIMELESS_ARTIFICER_ALTERNATE_3,
    CID_TIMELESS_ARTIFICER_ALTERNATE_4,
    CID_TIMELESS_ARTIFICER_ALTERNATE_5,
    CID_TIMELESS_ARTIFICER_ALTERNATE_6,
    CID_TIMELESS_ARTIFICER_ALTERNATE_7,
    CID_TIMELESS_ARTIFICER_ALTERNATE_8,
    CID_TIMELESS_ARTIFICER_ALTERNATE_9,
    CID_TIMELESS_ARTIFICER_ALTERNATE_10,
    CID_TIMELESS_ARTIFICER_ALTERNATE_11,
    CID_TIMELESS_ARTIFICER_ALTERNATE_12,
    CID_TIMELESS_ARTIFICER_ALTERNATE_13,
    CID_TIMELESS_ARTIFICER_ALTERNATE_14,
    ULTIMA_ORIGIN_OF_OBLIVION_ALTERNATE_2,
    ADELBERT_STEINER_ALTERNATE_1,
    AERITH_GAINSBOROUGH_ALTERNATE_2,
    AMBROSIA_WHITEHEART_ALTERNATE_2,
    ASHE_PRINCESS_OF_DALMASCA_ALTERNATE_1,
    CLOUD_MIDGAR_MERCENARY_ALTERNATE_2,
    DION_BAHAMUT_S_DOMINANT_ALTERNATE_2,
    G_RAHA_TIA_ALTERNATE_1,
    MINWU_WHITE_MAGE_ALTERNATE_1,
    SNOW_VILLIERS_ALTERNATE_1,
    STILTZKIN_MOOGLE_MERCHANT_ALTERNATE_2,
    VENAT_HEART_OF_HYDAELYN_ALTERNATE_2,
    ZACK_FAIR_ALTERNATE_1,
    EDGAR_KING_OF_FIGARO_ALTERNATE_1,
    GOGO_MASTER_OF_MIMICRY_ALTERNATE_2,
    JILL_SHIVA_S_DOMINANT_ALTERNATE_2,
    MATOYA_ARCHON_ELDER_ALTERNATE_1,
    QUISTIS_TREPE_ALTERNATE_1,
    ULTROS_OBNOXIOUS_OCTOPUS_ALTERNATE_1,
    Y_SHTOLA_RHUL_ALTERNATE_1,
    ARDYN_THE_USURPER_ALTERNATE_3,
    CECIL_DARK_KNIGHT_ALTERNATE_2,
    FANG_FEARLESS_L_CIE_ALTERNATE_2,
    RAGNAROK_DIVINE_DELIVERANCE_ALTERNATE_2,
    GAIUS_VAN_BAELSAR_ALTERNATE_1,
    JECHT_RELUCTANT_GUARDIAN_ALTERNATE_2,
    KAIN_TRAITOROUS_DRAGOON_ALTERNATE_2,
    RENO_AND_RUDE_ALTERNATE_1,
    SEPHIROTH_FABLED_SOLDIER_ALTERNATE_3,
    SIDEQUEST_HUNT_THE_MARK_ALTERNATE_1,
    VINCENT_VALENTINE_ALTERNATE_2,
    ZENOS_YAE_GALVUS_ALTERNATE_2,
    ZODIARK_UMBRAL_GOD_ALTERNATE_2,
    BARRET_WALLACE_ALTERNATE_1,
    CLIVE_IFRIT_S_DOMINANT_ALTERNATE_3,
    FIRION_WILD_ROSE_WARRIOR_ALTERNATE_2,
    FREYA_CRESCENT_ALTERNATE_1,
    GILGAMESH_MASTER_AT_ARMS_ALTERNATE_2,
    PROMPTO_ARGENTUM_ALTERNATE_2,
    QUEEN_BRAHNE_ALTERNATE_1,
    RAUBAHN_BULL_OF_ALA_MHIGO_ALTERNATE_2,
    SEIFER_ALMASY_ALTERNATE_2,
    VAAN_STREET_THIEF_ALTERNATE_2,
    ZELL_DINCHT_ALTERNATE_1,
    BARTZ_AND_BOKO_ALTERNATE_1,
    DIAMOND_WEAPON_ALTERNATE_1,
    QUINA_QU_GOURMET_ALTERNATE_1,
    SAZH_KATZROY_ALTERNATE_1,
    TIFA_LOCKHART_ALTERNATE_2,
    TORGAL_A_FINE_HOUND_ALTERNATE_2,
    VANILLE_CHEERFUL_L_CIE_ALTERNATE_2,
    ABSOLUTE_VIRTUE_ALTERNATE_1,
    BALTHIER_AND_FRAN_ALTERNATE_3,
    BLACK_WALTZ_NO_3_ALTERNATE_1,
    CHOCO_SEEKER_OF_PARADISE_ALTERNATE_1,
    CID_TIMELESS_ARTIFICER_ALTERNATE_15,
    CLOUD_OF_DARKNESS_ALTERNATE_1,
    EMET_SELCH_UNSUNDERED_ALTERNATE_2,
    THE_EMPEROR_OF_PALAMECIA_ALTERNATE_1,
    EXDEATH_VOID_WARLOCK_ALTERNATE_1,
    GARLAND_KNIGHT_OF_CORNELIA_ALTERNATE_1,
    GARNET_PRINCESS_OF_ALEXANDRIA_ALTERNATE_1,
    GIOTT_KING_OF_THE_DWARVES_ALTERNATE_1,
    GLADIOLUS_AMICITIA_ALTERNATE_1,
    GOLBEZ_CRYSTAL_COLLECTOR_ALTERNATE_2,
    HOPE_ESTHEIM_ALTERNATE_2,
    IGNIS_SCIENTIA_ALTERNATE_1,
    JENOVA_ANCIENT_CALAMITY_ALTERNATE_2,
    JOSHUA_PHOENIX_S_DOMINANT_ALTERNATE_2,
    JUDGE_MAGISTER_GABRANTH_ALTERNATE_1,
    KEFKA_COURT_MAGE_ALTERNATE_3,
    KUJA_GENOME_SORCERER_ALTERNATE_4,
    KUJA_GENOME_SORCERER_ALTERNATE_5,
    LIGHTNING_ARMY_OF_ONE_ALTERNATE_3,
    LOCKE_COLE_ALTERNATE_1,
    NOCTIS_PRINCE_OF_LUCIS_ALTERNATE_2,
    OMEGA_HEARTLESS_EVOLUTION_ALTERNATE_2,
    RINOA_HEARTILLY_ALTERNATE_1,
    RUFUS_SHINRA_ALTERNATE_1,
    RYDIA_SUMMONER_OF_MIST_ALTERNATE_1,
    SERAH_FARRON_ALTERNATE_1,
    SHANTOTTO_TACTICIAN_MAGICIAN_ALTERNATE_1,
    SIN_SPIRA_S_PUNISHMENT_ALTERNATE_2,
    SQUALL_SEED_MERCENARY_ALTERNATE_2,
    TELLAH_GREAT_SAGE_ALTERNATE_2,
    TERRA_MAGICAL_ADEPT_ALTERNATE_2,
    TIDUS_BLITZBALL_STAR_ALTERNATE_1,
    ULTIMECIA_TIME_SORCERESS_ALTERNATE_1,
    VIVI_ORNITIER_ALTERNATE_2,
    THE_WANDERING_MINSTREL_ALTERNATE_2,
    YUNA_HOPE_OF_SPIRA_ALTERNATE_2,
    ZIDANE_TANTALUS_THIEF_ALTERNATE_2,
    AERITH_GAINSBOROUGH_ALTERNATE_3,
    CLOUD_MIDGAR_MERCENARY_ALTERNATE_3,
    DION_BAHAMUT_S_DOMINANT_ALTERNATE_3,
    GOGO_MASTER_OF_MIMICRY_ALTERNATE_3,
    JILL_SHIVA_S_DOMINANT_ALTERNATE_3,
    ARDYN_THE_USURPER_ALTERNATE_4,
    CECIL_DARK_KNIGHT_ALTERNATE_3,
    FANG_FEARLESS_L_CIE_ALTERNATE_3,
    RAGNAROK_DIVINE_DELIVERANCE_ALTERNATE_3,
    SEPHIROTH_FABLED_SOLDIER_ALTERNATE_4,
    VINCENT_VALENTINE_ALTERNATE_3,
    ZENOS_YAE_GALVUS_ALTERNATE_3,
    CLIVE_IFRIT_S_DOMINANT_ALTERNATE_4,
    FIRION_WILD_ROSE_WARRIOR_ALTERNATE_3,
    PROMPTO_ARGENTUM_ALTERNATE_3,
    RAUBAHN_BULL_OF_ALA_MHIGO_ALTERNATE_3,
    SEIFER_ALMASY_ALTERNATE_3,
    VAAN_STREET_THIEF_ALTERNATE_3,
    TIFA_LOCKHART_ALTERNATE_3,
    VANILLE_CHEERFUL_L_CIE_ALTERNATE_3,
    BALTHIER_AND_FRAN_ALTERNATE_4,
    EMET_SELCH_UNSUNDERED_ALTERNATE_3,
    GOLBEZ_CRYSTAL_COLLECTOR_ALTERNATE_3,
    HOPE_ESTHEIM_ALTERNATE_3,
    JOSHUA_PHOENIX_S_DOMINANT_ALTERNATE_3,
    KEFKA_COURT_MAGE_ALTERNATE_4,
    KUJA_GENOME_SORCERER_ALTERNATE_6,
    KUJA_GENOME_SORCERER_ALTERNATE_7,
    LIGHTNING_ARMY_OF_ONE_ALTERNATE_4,
    NOCTIS_PRINCE_OF_LUCIS_ALTERNATE_3,
    SQUALL_SEED_MERCENARY_ALTERNATE_3,
    THE_WANDERING_MINSTREL_ALTERNATE_3,
    YUNA_HOPE_OF_SPIRA_ALTERNATE_3,
    ZIDANE_TANTALUS_THIEF_ALTERNATE_3,
    TRAVELING_CHOCOBO_ALTERNATE_3,
    TRAVELING_CHOCOBO_ALTERNATE_4,
    TRAVELING_CHOCOBO_ALTERNATE_5,
    TRAVELING_CHOCOBO_ALTERNATE_6,
    TRAVELING_CHOCOBO_ALTERNATE_7,
    CLOUD_PLANET_S_CHAMPION_ALTERNATE_1,
    SEPHIROTH_PLANET_S_HEIR_ALTERNATE_1,
    BEATRIX_LOYAL_GENERAL_ALTERNATE_1,
    ROSA_RESOLUTE_WHITE_MAGE_ALTERNATE_1,
    ULTIMECIA_TEMPORAL_THREAT_ALTERNATE_1,
    SEYMOUR_FLUX_ALTERNATE_1,
    LIGHTNING_SECURITY_SERGEANT_ALTERNATE_1,
    XANDE_DARK_MAGE_ALTERNATE_1,
    CLOUD_MIDGAR_MERCENARY_ALTERNATE_4,
    STILTZKIN_MOOGLE_MERCHANT_ALTERNATE_3,
    A_REALM_REBORN_ALTERNATE_2,
    TIFA_LOCKHART_ALTERNATE_4,
    TRAVELING_CHOCOBO_ALTERNATE_8,
    CHOCO_SEEKER_OF_PARADISE_ALTERNATE_2,
    VIVI_ORNITIER_ALTERNATE_3,
    YUNA_HOPE_OF_SPIRA_ALTERNATE_4,
    PLAINS_ALTERNATE_3,
    ISLAND_ALTERNATE_3,
    SWAMP_ALTERNATE_3,
    MOUNTAIN_ALTERNATE_3,
    FOREST_ALTERNATE_3,
    Y_SHTOLA_RHUL_ALTERNATE_2,
    PHOENIX_DOWN_ALTERNATE_1,
    WHITE_AURACITE_ALTERNATE_1,
    ZACK_FAIR_ALTERNATE_2,
    SAGE_S_NOULITHS_ALTERNATE_1,
    CIRCLE_OF_POWER_ALTERNATE_1,
    BARRET_WALLACE_ALTERNATE_2,
    LAUGHING_MAD_ALTERNATE_1,
];
