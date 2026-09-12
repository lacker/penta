//! Jumpstart card records.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BindObjectsDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CharacteristicOperationDef;
use crate::card::ComparisonDef;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::CreatureTypeSetDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::MoveObjectsDef;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectSetFilterDef;
use crate::card::PayOrDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::RandomizeObjectOrderDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::RevealAndClassifyCardsDef;
use crate::card::SetOperationDef;
use crate::card::SubtypeDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "JMP",
    slug: "jumpstart",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const TREASURE_TOKEN: TokenCharacteristics = crate::card::tokens::treasure().with_art(
    CardArt::new("4306be80-d7c9-4bcf-a3de-4bf159475546", "Alayna Danner"),
);

// JMP 3 — Emiel the Blessed
pub(in crate::card::sets) static EMIEL_THE_BLESSED_3: CardRecord = CardRecord::new(
    "Emiel the Blessed",
    "f74dfd07-d17c-4890-82c3-4b12a6029940",
    "Antonio José Manzanedo",
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Unicorn"], 4, 4).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::activated_with_targets("{3}: Exile another target creature you control, then return it to the battlefield under its owner's control.", &[CostDef::Mana(mana_cost!("{3}"))], &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Not(&ObjectPredicateDef::Source)]), zones: &[ZoneKind::Battlefield], controller: Some(PlayerRelation::You), owner: None })], EffectDef::BindObjects(BindObjectsDef { source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::LegalTargets(TargetIndex::PRIMARY)), binding: Binding!("emiel_blink"), then: &EffectDef::Sequence(&[EffectDef::move_to_zone(EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("emiel_blink"))), ZoneKind::Exile, ZonePlacement::Top), EffectDef::MoveObjects(MoveObjectsDef { input: ObjectSetDef::ZoneChangeSuccessorsOfBinding(Binding!("emiel_blink")), from: Some(ZoneKind::Exile), zone: ZoneKind::Battlefield, placement: ZonePlacement::Top, moved: None, then: &EffectDef::None })]) })),
AbilityDef::triggered("Whenever another creature you control enters, you may pay {G/W}. If you do, put a +1/+1 counter on it. If it's a Unicorn, put two +1/+1 counters on it instead. ({G/W} can be paid with either {G} or {W}.)", TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Not(&ObjectPredicateDef::Source), ObjectPredicateDef::ControlledBy(PlayerRelation::You)]), None, Some(ZoneKind::Battlefield)), EffectDef::PayOr(PayOrDef::optional(&[CostDef::Mana(mana_cost!("{G/W}"))], &EffectDef::IfElseCondition { condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef { left: ValueDef::CountObjects(&ObjectSetDef::Matching { objects: &ObjectSetDef::One(ObjectRefDef::TriggeringObject), object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::Subtype(SubtypeDef::Literal("Unicorn"))) }), comparison: ComparisonDef::Greater, right: ValueDef::Constant(0) }), then: &EffectDef::AddCounters { object: EffectRecipientDef::TriggeringObject, kind: CounterKind::PlusOnePlusOne, amount: ValueDef::Constant(2) }, otherwise: &EffectDef::AddCounters { object: EffectRecipientDef::TriggeringObject, kind: CounterKind::PlusOnePlusOne, amount: ValueDef::Constant(1) } })))
]),
);

// JMP 4 — Release the Dogs
pub(in crate::card::sets) static RELEASE_THE_DOGS: CardRecord = CardRecord::new(
    "Release the Dogs",
    "7df3cd89-02c9-4a1c-9a8a-d17a0b1030c9",
    "Jason Kang",
    CardRules::new_sorcery(mana_cost!("{3}{W}")).with_abilities(&[AbilityDef::spell(
        "Create four 1/1 white Dog creature tokens.",
        EffectDef::CreateToken(
            CreateTokenDef::new(TokenDef::Literal(TokenCharacteristics::creature(
                &["Dog"],
                &[ManaColor::White],
                1,
                1,
            )))
            .with_count(ValueDef::Constant(4)),
        ),
    )]),
);

// JMP 11 — Corsair Captain
pub(in crate::card::sets) static CORSAIR_CAPTAIN: CardRecord = CardRecord::new(
    "Corsair Captain",
    "a9b016d4-ddf6-47d6-b934-a0b979b60680",
    "Victor Adame Minguez",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Pirate"], 2, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, create a Treasure token. (It's an \
             artifact with \"{T}, Sacrifice this token: Add one mana of \
             any color.\")",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ),
        AbilityDef::static_ability(
            "Other Pirates you control get +1/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Pirate")),
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
    ]),
);

// JMP 24 — Muxus, Goblin Grandee
pub(in crate::card::sets) static MUXUS_GOBLIN_GRANDEE_24: CardRecord = CardRecord::new(
    "Muxus, Goblin Grandee",
    "2c716d10-2130-43b7-a939-349d437e1091",
    "Dmitry Burmak",
    CardRules::new_creature(mana_cost!("{4}{R}{R}"), &["Goblin", "Noble"], 4, 4).with_supertype(CardSupertype::Legendary).with_abilities(&[
abilities::enters_trigger("When Muxus enters, reveal the top six cards of your library. Put all Goblin creature cards with mana value 5 or less from among them onto the battlefield and the rest on the bottom of your library in a random order.", EffectDef::RevealAndClassifyCards(RevealAndClassifyCardsDef { source: ObjectCollectionSourceDef::TopCards { player: PlayerRefDef::EffectController, count: ValueDef::Constant(6) }, object: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Subtype(SubtypeDef::Literal("Goblin")), ObjectPredicateDef::ManaValueAtMost(5)]), matching: Binding!("recruited"), remainder: Binding!("remainder"), then: &EffectDef::Sequence(&[EffectDef::move_to_zone(EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("recruited"))), ZoneKind::Battlefield, ZonePlacement::Top), EffectDef::RandomizeObjectOrder(RandomizeObjectOrderDef { input: ObjectSetDef::Binding(Binding!("remainder")), randomized: Binding!("random_bottom"), then: &EffectDef::MoveObjects(MoveObjectsDef { input: ObjectSetDef::Binding(Binding!("random_bottom")), from: Some(ZoneKind::Library), zone: ZoneKind::Library, placement: ZonePlacement::Bottom, moved: None, then: &EffectDef::None }) })]) })),
AbilityDef::triggered("Whenever Muxus attacks, it gets +1/+1 until end of turn for each other Goblin you control.", TriggerEventDef::attacks(ObjectPredicateDef::Source), EffectDef::Apply { recipient: EffectRecipientDef::Source, effect: AppliedEffectDef::modify_power_toughness(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(ObjectPredicateDef::All(&[ObjectPredicateDef::Subtype(SubtypeDef::Literal("Goblin")), ObjectPredicateDef::Not(&ObjectPredicateDef::Source)]), &[ZoneKind::Battlefield], PlayerRelation::You)), ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(ObjectPredicateDef::All(&[ObjectPredicateDef::Subtype(SubtypeDef::Literal("Goblin")), ObjectPredicateDef::Not(&ObjectPredicateDef::Source)]), &[ZoneKind::Battlefield], PlayerRelation::You))), duration: ResolvedEffectDurationDef::UntilEndOfTurn })
]),
);

// JMP 28 — Allosaurus Shepherd
pub(in crate::card::sets) static ALLOSAURUS_SHEPHERD_28: CardRecord = CardRecord::new(
    "Allosaurus Shepherd",
    "0ee4a931-5d61-49ba-affc-f022263938ca",
    "Randy Vargas",
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Shaman"], 1, 1).with_abilities(&[
abilities::cannot_be_countered(),
AbilityDef::static_ability("Green spells you control can't be countered.", EffectDef::StaticApply { recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::Color(ManaColor::Green), &[ZoneKind::Stack], PlayerRelation::You), effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered) }),
AbilityDef::activated("{4}{G}{G}: Until end of turn, each Elf creature you control has base power and toughness 5/5 and becomes a Dinosaur in addition to its other creature types.", &[CostDef::Mana(mana_cost!("{4}{G}{G}"))], EffectDef::Apply { recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elf"))]), &[ZoneKind::Battlefield], PlayerRelation::You), effect: AppliedEffectDef::Composite(&[AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(5), ValueDef::Constant(5)), AppliedEffectDef::Characteristic(CharacteristicOperationDef::CreatureTypes(SetOperationDef::Add(CreatureTypeSetDef::named(&["Dinosaur"]))))]), duration: ResolvedEffectDurationDef::UntilEndOfTurn })
]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &EMIEL_THE_BLESSED_3,
    &RELEASE_THE_DOGS,
    &CORSAIR_CAPTAIN,
    &MUXUS_GOBLIN_GRANDEE_24,
    &ALLOSAURUS_SHEPHERD_28,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
