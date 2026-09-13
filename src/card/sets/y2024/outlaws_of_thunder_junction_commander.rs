//! Outlaws of Thunder Junction Commander card records required by the cEDH corpus.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AggregateOperationDef;
use crate::card::AppliedEffectDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::ColorSet;
use crate::card::ComparisonDef;
use crate::card::CopyStackObjectDef;
use crate::card::CostDef;
use crate::card::CreatureTypeSetDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ExilePlayDurationDef;
use crate::card::ManaColor;
use crate::card::ManaTypeSetDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectValueAggregateDef;
use crate::card::ObjectValueDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SpellCastQueryDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::mana_cost;

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "OTC",
    slug: "outlaws-of-thunder-junction-commander",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// OTC 3 — Stella Lee, Wild Card
pub(in crate::card::sets) static STELLA_LEE_WILD_CARD_3: CardRecord = CardRecord::new(
    "Stella Lee, Wild Card",
    "2a8a7696-b5d9-4378-9d5c-2c9007e4df63",
    "Fajareka Setiawan",
    CardRules::new_creature(mana_cost!("{1}{U}{R}"), &["Human", "Rogue"], 2, 4).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::triggered("Whenever you cast your second spell each turn, exile the top card of your library. Until the end of your next turn, you may play that card.", TriggerEventDef::While { event: &TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(PlayerRelation::You)), condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef { left: ValueDef::CountSpellsCastThisTurn(&SpellCastQueryDef { spell: ObjectPredicateDef::Any, player: PlayerRelation::You }), comparison: ComparisonDef::Equal, right: ValueDef::Constant(2) }) }, EffectDef::ExileTopOfLibraryToPlay { player: EffectRecipientDef::Controller, amount: ValueDef::Constant(1), free: false, face_down: false, duration: ExilePlayDurationDef::UntilEndOfYourNextTurn, spend_any_color: false, play_condition: None, cast_only: false }),
AbilityDef::activated_with_targets("{T}: Copy target instant or sorcery spell you control. You may choose new targets for the copy. Activate only if you've cast three or more spells this turn.", &[CostDef::TapSource], &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::All(&[ObjectPredicateDef::Spell, ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Instant), ObjectPredicateDef::HasType(CardType::Sorcery)])]), zones: &[ZoneKind::Stack], controller: Some(PlayerRelation::You), owner: None })], EffectDef::CopyStackObject(&CopyStackObjectDef { object: EffectRecipientDef::Target(TargetIndex::PRIMARY), controller: PlayerRefDef::EffectController, count: ValueDef::Constant(1), retarget: true, colors: None })).with_activation_condition(&TriggerConditionDef::ValueComparison(&ValueComparisonDef { left: ValueDef::CountSpellsCastThisTurn(&SpellCastQueryDef { spell: ObjectPredicateDef::Any, player: PlayerRelation::You }), comparison: ComparisonDef::GreaterOrEqual, right: ValueDef::Constant(3) }))
]),
);

// OTC 11 — Sand Scout
// Audit: unsupported — Needs a once-per-turn grouped trigger for land cards entering a graveyard from any zone.
pub(in crate::card::sets) static SAND_SCOUT: CardRecord = CardRecord::new(
    "Sand Scout",
    "e63ba7e6-87a9-49ef-bddc-60543edfd726",
    "Olena Richards",
    CardRules::unsupported(),
);

// OTC 40 — Cactus Preserve
pub(in crate::card::sets) static CACTUS_PRESERVE_40: CardRecord = CardRecord::new(
    "Cactus Preserve",
    "ad9d426f-5870-42bb-a589-9218f7e35d62",
    "Jonas De Ro",
    CardRules::new_land(&[])
        .with_subtypes(&["Desert"])
        .with_abilities(&[
            abilities::enters_tapped(CardType::Land),
            AbilityDef::activated_mana(
                "{T}: Add one mana of any type that a land you control could produce.",
                &[CostDef::TapSource],
                EffectDef::AddMana(AddManaEffectDef::choice_from(
                    ManaTypeSetDef::could_be_produced_by(&ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Land),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                )),
            ),
            AbilityDef::activated(
                "{3}: Until end of turn, this land becomes an X/X green Plant creature with \
                 reach, where X is the greatest mana value among your commanders. It's still \
                 a land.",
                &[CostDef::Mana(mana_cost!("{3}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                        AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Plant"])),
                        AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Green])),
                        AppliedEffectDef::set_base_power_toughness(
                            ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                                objects: ObjectSetDef::Query(ObjectQueryDef::owned_by(
                                    ObjectPredicateDef::Commander,
                                    &[
                                        ZoneKind::Battlefield,
                                        ZoneKind::Stack,
                                        ZoneKind::Hand,
                                        ZoneKind::Library,
                                        ZoneKind::Graveyard,
                                        ZoneKind::Exile,
                                        ZoneKind::Command,
                                    ],
                                    PlayerSetDef::One(PlayerRefDef::EffectController),
                                )),
                                select: ObjectValueDef::ManaValue,
                                operation: AggregateOperationDef::Maximum,
                            }),
                            ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                                objects: ObjectSetDef::Query(ObjectQueryDef::owned_by(
                                    ObjectPredicateDef::Commander,
                                    &[
                                        ZoneKind::Battlefield,
                                        ZoneKind::Stack,
                                        ZoneKind::Hand,
                                        ZoneKind::Library,
                                        ZoneKind::Graveyard,
                                        ZoneKind::Exile,
                                        ZoneKind::Command,
                                    ],
                                    PlayerSetDef::One(PlayerRefDef::EffectController),
                                )),
                                select: ObjectValueDef::ManaValue,
                                operation: AggregateOperationDef::Maximum,
                            }),
                        ),
                        AppliedEffectDef::add_ability(&abilities::reach()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// OTC 51 — Lock and Load
pub(in crate::card::sets) static LOCK_AND_LOAD_51: CardRecord = CardRecord::new(
    "Lock and Load",
    "3a24979d-a090-4153-9461-ac1aa1f69b74",
    "Anastasia Ovchinnikova",
    CardRules::new_sorcery(mana_cost!("{2}{U}")).with_abilities(&[
AbilityDef::spell("Draw a card, then draw a card for each other instant and sorcery spell you've cast this turn.", EffectDef::Sequence(&[abilities::draw_cards(ValueDef::Constant(1)), abilities::draw_cards(ValueDef::CountSpellsCastThisTurn(&SpellCastQueryDef { spell: ObjectPredicateDef::All(&[ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Instant), ObjectPredicateDef::HasType(CardType::Sorcery)]), ObjectPredicateDef::Not(&ObjectPredicateDef::Source)]), player: PlayerRelation::You }))])),
abilities::plot(&[CostDef::Mana(mana_cost!("{3}{U}"))])
]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &STELLA_LEE_WILD_CARD_3,
    &SAND_SCOUT,
    &CACTUS_PRESERVE_40,
    &LOCK_AND_LOAD_51,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
