//! Morningtide card records.

use crate::card::BattlefieldEntryModificationDef;
use crate::card::BindObjectsDef;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::ChooseObjectOrderDef;
use crate::card::CounterKind;
use crate::card::LookAtObjectsDef;
use crate::card::MoveObjectsDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRefDef;
use crate::card::ReplacementEffectDef;
use crate::card::ReplacementEventDef;
use crate::card::TriggerEventDef;
use super::CardRecord;
use super::PrintingRecord;
use crate::AbilityDef;
use crate::AbilityTargetDef;
use crate::AbilityTargetPredicate;
use crate::AppliedEffectDef;
use crate::CardRules;
use crate::CardType;
use crate::CardTypeSet;
use crate::CreatureTypeSetDef;
use crate::EffectDef;
use crate::EffectRecipientDef;
use crate::ManaColor;
use crate::ObjectPredicateDef;
use crate::ResolvedEffectDurationDef;
use crate::TargetIndex;
use crate::ValueDef;
use crate::ZoneKind;
use crate::ZonePlacement;
use crate::card::CostDef;
use crate::card::PlayerRelation;
use crate::card::SubtypeDef;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "MOR",
    slug: "morningtide",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// MOR 1 — Ballyrush Banneret
pub(in crate::card::sets) static BALLYRUSH_BANNERET: CardRecord = CardRecord::new(
    "Ballyrush Banneret",
    "a029814e-d84d-43e5-b483-e918871b3333",
    "Ralph Horsley",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Kithkin", "Soldier"], 2, 1).with_abilities(&[
        abilities::spell_cost_reduction(
            "Kithkin spells and Soldier spells you cast cost {1} less to cast.",
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Kithkin")),
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Soldier")),
            ]),
            PlayerRelation::You,
            ValueDef::Constant(1),
        ),
    ]),
);

// MOR 31 — Disperse
pub(in crate::card::sets) static DISPERSE: CardRecord = CardRecord::new(
    "Disperse",
    "0ae239b2-1596-4906-9711-1d180a246d35",
    "Steve Ellis",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return target nonland permanent to its owner's hand.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
        )],
        EffectDef::move_to_zone(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ZoneKind::Hand,
            ZonePlacement::Top,
        ),
    )),
);

// MOR 41 — Mind Spring
pub(in crate::card::sets) static MIND_SPRING: CardRecord = CardRecord::new(
    "Mind Spring",
    "7b7cd9b6-1ea8-423d-8aa0-8699fffbcf50",
    "Mark Zug",
    CardRules::new_sorcery(mana_cost!("{X}{U}{U}")).with_abilities(&[AbilityDef::spell(
        "Draw X cards.",
        abilities::draw_cards(ValueDef::ChosenX),
    )]),
);

// MOR 43 — Negate
pub(in crate::card::sets) static NEGATE: CardRecord = CardRecord::new(
    "Negate",
    "5a501252-e722-4ebf-bcf7-f53a42745fa7",
    "Jeremy Jarvis",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::counter_target(
        "Counter target noncreature spell.",
        &AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::Spell,
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
            ]),
            zones: &[ZoneKind::Stack],
            controller: None,
            owner: None,
        }),
    )),
);

// MOR 92 — Kindled Fury
pub(in crate::card::sets) static KINDLED_FURY: CardRecord = CardRecord::new(
    "Kindled Fury",
    "993956c9-30d8-41ee-84c2-c06d0512aea4",
    "Shelly Wan",
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +1/+0 and gains first strike until end of turn.",
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
                AppliedEffectDef::add_ability(&abilities::first_strike()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// MOR 104 — Shared Animosity
// Audit: unsupported — There is no predicate comparing two objects' creature-type sets, including changeling, to count other attacking creatures sharing a type with the triggering creature.
pub(in crate::card::sets) static SHARED_ANIMOSITY_104: CardRecord = CardRecord::new(
    "Shared Animosity",
    "fe332c46-90f0-4cc0-8bf1-35a3934ff8a0",
    "Chuck Lukacs",
    crate::card::CardRules::unsupported(),
);

// MOR 105 — Spitebellows
pub(in crate::card::sets) static SPITEBELLOWS_105: CardRecord = CardRecord::new(
    "Spitebellows",
    "43f2104d-aeff-493f-8227-cb95bf3e2eab",
    "Larry MacDougall",
    CardRules::new_creature(mana_cost!("{5}{R}"), &["Elemental"], 6, 1).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "When this creature leaves the battlefield, it deals 6 damage to target creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                None,
            ),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(6),
            ),
        ),
        abilities::evoke(&[CostDef::Mana(mana_cost!("{1}{R}{R}"))])[0],
        abilities::evoke(&[CostDef::Mana(mana_cost!("{1}{R}{R}"))])[1],
    ]),
);

// MOR 109 — Taurean Mauler
// Audit: unsupported — Needs a creature-type characteristic-defining ability applying in every zone and supplying every creature type as copiable values; battlefield all-type modifiers do not implement changeling.
pub(in crate::card::sets) static TAUREAN_MAULER: CardRecord = CardRecord::new(
    "Taurean Mauler",
    "d50b5df1-b658-4df0-900e-79c44599b93e",
    "Dominick Domingo",
    CardRules::unsupported(),
);

// MOR 115 — Bramblewood Paragon
pub(in crate::card::sets) static BRAMBLEWOOD_PARAGON_115: CardRecord = CardRecord::new(
    "Bramblewood Paragon",
    "3910f5b2-17da-41e4-bf40-1c40b513fa12",
    "Jim Murray",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Elf", "Warrior"], 2, 2).with_abilities(&[
AbilityDef::replacement_for("Each other Warrior creature you control enters with an additional +1/+1 counter on it.", ReplacementEventDef::ObjectEntersBattlefield { object: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Subtype(SubtypeDef::Literal("Warrior")), ObjectPredicateDef::Not(&ObjectPredicateDef::Source)]), controller: PlayerRelation::You, cast: None }, ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::AddCounters { kind: CounterKind::PlusOnePlusOne, amount: 1 })),
AbilityDef::static_ability("Each creature you control with a +1/+1 counter on it has trample.", EffectDef::StaticApply { recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::HasCounter(CounterKind::PlusOnePlusOne)]), &[ZoneKind::Battlefield], PlayerRelation::You), effect: AppliedEffectDef::add_ability(&abilities::trample()) })
]),
);

// MOR 117 — Cream of the Crop
pub(in crate::card::sets) static CREAM_OF_THE_CROP_117: CardRecord = CardRecord::new(
    "Cream of the Crop",
    "030b0a9d-d0cf-4f3a-97b3-3e1d59226ee6",
    "Howard Lyon",
    CardRules::new_enchantment(mana_cost!("{1}{G}")).with_abilities(&[
AbilityDef::triggered("Whenever a creature you control enters, you may look at the top X cards of your library, where X is that creature's power. If you do, put one of those cards on top of your library and the rest on the bottom of your library in any order.", TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::ControlledBy(PlayerRelation::You)]), None, Some(ZoneKind::Battlefield)), EffectDef::May { player: EffectRecipientDef::Controller, effect: &EffectDef::BindObjects(BindObjectsDef { source: ObjectCollectionSourceDef::TopCards { player: PlayerRefDef::EffectController, count: ValueDef::TriggeringObjectPower }, binding: Binding!("crop_seen"), then: &EffectDef::Sequence(&[EffectDef::LookAtObjects(LookAtObjectsDef { actor: PlayerRefDef::EffectController, source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::Binding(Binding!("crop_seen"))), visibility: ChoiceVisibilityDef::Private, then: &EffectDef::None }), EffectDef::Choose(ChooseDef { binding: ObjectChoiceBindingDef::Objects(Binding!("crop_top")), unchosen: Some(Binding!("crop_bottom")), chooser: PlayerRefDef::EffectController, candidates: ObjectSetDef::Binding(Binding!("crop_seen")), exclude: None, minimum: 1, maximum: 1, visibility: ChoiceVisibilityDef::Private, then: &EffectDef::Sequence(&[EffectDef::MoveObjects(MoveObjectsDef { input: ObjectSetDef::Binding(Binding!("crop_top")), from: Some(ZoneKind::Library), zone: ZoneKind::Library, placement: ZonePlacement::Top, moved: None, then: &EffectDef::None }), EffectDef::ChooseObjectOrder(ChooseObjectOrderDef { actor: PlayerRefDef::EffectController, input: ObjectSetDef::Binding(Binding!("crop_bottom")), ordered: Binding!("crop_order"), placement: ZonePlacement::Bottom, visibility: ChoiceVisibilityDef::Private, then: &EffectDef::MoveObjects(MoveObjectsDef { input: ObjectSetDef::Binding(Binding!("crop_order")), from: Some(ZoneKind::Library), zone: ZoneKind::Library, placement: ZonePlacement::Bottom, moved: None, then: &EffectDef::None }) })]) })]) }) })
]),
);

// MOR 143 — Door of Destinies
// Audit: unsupported — Predicates cannot consume a stored creature-type choice for both spell triggers and a counter-scaled continuous bonus.
pub(in crate::card::sets) static DOOR_OF_DESTINIES: CardRecord = CardRecord::new(
    "Door of Destinies",
    "ac4800be-5f77-42f5-914c-2a8e647e3af5",
    "Larry MacDougall",
    crate::card::CardRules::unsupported(),
);

// MOR 145 — Thornbite Staff
pub(in crate::card::sets) static THORNBITE_STAFF_145: CardRecord = CardRecord::new(
    "Thornbite Staff",
    "c1ab3225-64a9-411e-b22b-1869e958b8e5",
    "Jesper Ejsing",
    CardRules::new_artifact(mana_cost!("{2}")).with_subtypes(&["Shaman", "Equipment"]).with_abilities(&[
AbilityDef::static_ability("Equipped creature has \"{2}, {T}: This creature deals 1 damage to any target\" and \"Whenever a creature dies, untap this creature.\"", EffectDef::StaticApply { recipient: EffectRecipientDef::AttachedPermanent, effect: AppliedEffectDef::Composite(&[AppliedEffectDef::add_ability(&AbilityDef::activated_with_targets("{2}, {T}: This creature deals 1 damage to any target.", &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource], &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)], EffectDef::damage(EffectRecipientDef::Target(TargetIndex::PRIMARY), ValueDef::Constant(1)))), AppliedEffectDef::add_ability(&AbilityDef::triggered("Whenever a creature dies, untap this creature.", TriggerEventDef::zone_changed(ObjectPredicateDef::HasType(CardType::Creature), Some(ZoneKind::Battlefield), Some(ZoneKind::Graveyard)), EffectDef::Untap { object: EffectRecipientDef::Source }))]) }),
AbilityDef::triggered("Whenever a Shaman creature enters, you may attach this Equipment to it.", TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature),ObjectPredicateDef::Subtype(SubtypeDef::Literal("Shaman"))]), None, Some(ZoneKind::Battlefield)), EffectDef::May { player: EffectRecipientDef::Controller, effect: &EffectDef::Attach { object: EffectRecipientDef::TriggeringObject } }),
abilities::equip(&[CostDef::Mana(mana_cost!("{4}"))], "Equip {4}")
])
.with_type(crate::card::CardType::Kindred),
);

// MOR 148 — Mutavault
pub(in crate::card::sets) static MUTAVAULT: CardRecord = CardRecord::new(
    "Mutavault",
    "8ca3c48b-f104-4292-9a4e-2ce87a65893c",
    "Fred Fields",
CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{1}: This land becomes a 2/2 creature with all creature types until end of turn. It's still a land.",
            &[CostDef::Mana(mana_cost!("{1}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                // The animation keeps the land types Mutavault is printed with, so the
                // creature types are added rather than replacing anything.
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::ALL),
                    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &BALLYRUSH_BANNERET,
    &DISPERSE,
    &MIND_SPRING,
    &NEGATE,
    &KINDLED_FURY,
    &SHARED_ANIMOSITY_104,
    &SPITEBELLOWS_105,
    &TAUREAN_MAULER,
    &BRAMBLEWOOD_PARAGON_115,
    &CREAM_OF_THE_CROP_117,
    &DOOR_OF_DESTINIES,
    &THORNBITE_STAFF_145,
    &MUTAVAULT,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
