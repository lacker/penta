//! Assassin's Creed card records required by the cEDH corpus.

use super::{CardRecord, PrintingRecord};
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::PlayActionMatcherDef;
use crate::card::PlayRestrictionDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SubtypeDef;
use crate::card::TopOfLibraryCostDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "ACR",
    slug: "assassin-s-creed",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());























// ACR 8 — Senu, Keen-Eyed Protector
// Audit: unsupported — A resolving effect cannot put an arbitrary exiled card onto the battlefield attacking. The existing attacking-entry paths handle tokens and ninjutsu from hand, not this unblocked-attack return.
pub(in crate::card::sets) static SENU_KEEN_EYED_PROTECTOR_8: CardRecord = CardRecord::new(
    "Senu, Keen-Eyed Protector",
    "5671a03d-0858-41e7-976c-60825c29af04",
    "Michael MacRae",
    crate::card::CardRules::unsupported(),
);

// ACR 63 — Shao Jun
pub(in crate::card::sets) static SHAO_JUN_63: CardRecord = CardRecord::new(
    "Shao Jun",
    "085568e5-d622-45ff-9a31-52eaa513ff31",
    "Stephen Stark",
    CardRules::new_creature(mana_cost!("{1}{U}{R}"), &["Human", "Assassin"], 3, 3).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::static_ability("Leap Strike — During your turn, Shao Jun has flying and first strike.", EffectDef::IfCondition { condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You), then: &EffectDef::StaticApply { recipient: EffectRecipientDef::Source, effect: AppliedEffectDef::Composite(&[AppliedEffectDef::add_ability(&abilities::flying()), AppliedEffectDef::add_ability(&abilities::first_strike())]) } }),
AbilityDef::activated("Rope Dart — Tap two untapped artifacts you control: Shao Jun deals 1 damage to each opponent.", &[CostDef::TapPermanents { object: ObjectPredicateDef::HasType(CardType::Artifact), controller: PlayerRelation::You, count: 2 }], EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(1)))
]),
);

// ACR 70 — Apple of Eden, Isu Relic
// Audit: unsupported — Exile-play permissions do not carry a per-card play callback that makes the original owner draw, including land plays. A global spell-cast trigger would miss lands and count casts using unrelated permissions.
pub(in crate::card::sets) static APPLE_OF_EDEN_ISU_RELIC_70: CardRecord = CardRecord::new(
    "Apple of Eden, Isu Relic",
    "17dd0b7f-bd26-4a46-a7a1-bc65138d54ed",
    "L J Koh",
    crate::card::CardRules::unsupported(),
);

// ACR 72 — Excalibur, Sword of Eden
// Audit: unsupported — The spell-cost evaluator cannot sum the mana values of a queried set of permanents. AggregateObjectValues is available during resolution but evaluates to zero in cast-cost reductions.
pub(in crate::card::sets) static EXCALIBUR_SWORD_OF_EDEN_72: CardRecord = CardRecord::new(
    "Excalibur, Sword of Eden",
    "26dbf574-3193-413e-a982-4b9d27dafaf5",
    "Thanh Tuấn",
    crate::card::CardRules::unsupported(),
);

// ACR 79 — Abstergo Entertainment
pub(in crate::card::sets) static ABSTERGO_ENTERTAINMENT_79: CardRecord = CardRecord::new(
    "Abstergo Entertainment",
    "4d197866-7633-493c-80dd-ec3a09165934",
    "Alexander Gering",
    CardRules::new_land(&[]).with_supertype(CardSupertype::Legendary).with_abilities(&[
abilities::tap_for(ManaColor::Colorless),
AbilityDef::activated_mana("{1}, {T}: Add one mana of any color.", &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource], EffectDef::AddMana(AddManaEffectDef::choice(&[ManaColor::White, ManaColor::Blue, ManaColor::Black, ManaColor::Red, ManaColor::Green]))),
AbilityDef::activated_with_targets("{3}, {T}, Exile Abstergo Entertainment: Return up to one target historic card from your graveyard to your hand, then exile all graveyards. (Artifacts, legendaries, and Sagas are historic.)", &[CostDef::Mana(mana_cost!("{3}")), CostDef::TapSource, CostDef::ExileSource], &[AbilityTargetDef::up_to(AbilityTargetPredicate::Object { object: ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::Supertype(CardSupertype::Legendary), ObjectPredicateDef::Subtype(SubtypeDef::Literal("Saga"))]), zones: &[ZoneKind::Graveyard], controller: None, owner: Some(PlayerRelation::You) }, 1)], EffectDef::Sequence(&[EffectDef::move_to_zone(EffectRecipientDef::Target(TargetIndex::PRIMARY), ZoneKind::Hand, ZonePlacement::Top), EffectDef::move_to_zone(EffectRecipientDef::matching_objects(ObjectPredicateDef::Any, &[ZoneKind::Graveyard], PlayerRelation::Any), ZoneKind::Exile, ZonePlacement::Top)]))
]),
);

// ACR 134 — Alexios, Deimos of Kosmos
// Audit: unsupported — There is no continuous cannot-be-sacrificed rule enforced by costs and resolving sacrifice effects, or attack prohibition tied to the creature's owner.
pub(in crate::card::sets) static ALEXIOS_DEIMOS_OF_KOSMOS_134: CardRecord = CardRecord::new(
    "Alexios, Deimos of Kosmos",
    "ab907c09-56c0-40ed-aebd-63b64c7e1c2e",
    "Jessie Lam",
    crate::card::CardRules::unsupported(),
);

// ACR 141 — Basim Ibn Ishaq
pub(in crate::card::sets) static BASIM_IBN_ISHAQ_141: CardRecord = CardRecord::new(
    "Basim Ibn Ishaq",
    "0e74cc38-108d-46d4-9d4b-43ed5653982a",
    "Astri Lohne",
    CardRules::new_creature(mana_cost!("{U}{B}"), &["Human", "Assassin"], 2, 2).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::triggered("Whenever you cast a historic spell, draw a card. Basim Ibn Ishaq can't be blocked this turn. This ability triggers only once each turn. (Artifacts, legendaries, and Sagas are historic.)", TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::Supertype(CardSupertype::Legendary), ObjectPredicateDef::Subtype(SubtypeDef::Literal("Saga"))]), ObjectPredicateDef::ControlledBy(PlayerRelation::You)])), EffectDef::Sequence(&[abilities::draw_cards(ValueDef::Constant(1)), EffectDef::Apply { recipient: EffectRecipientDef::Source, effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED), duration: ResolvedEffectDurationDef::UntilEndOfTurn }])).triggering_at_most(1),
AbilityDef::triggered("Whenever Basim Ibn Ishaq deals combat damage to a player, put a +1/+1 counter on it.", TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source), EffectDef::AddCounters { object: EffectRecipientDef::Source, kind: CounterKind::PlusOnePlusOne, amount: ValueDef::Constant(1) })
]),
);

// ACR 158 — Crystal Skull, Isu Spyglass
pub(in crate::card::sets) static CRYSTAL_SKULL_ISU_SPYGLASS_158: CardRecord = CardRecord::new(
    "Crystal Skull, Isu Spyglass",
    "1068ed8a-a062-4249-802d-6f4070992de0",
    "Thanh Tuấn",
    CardRules::new_artifact(mana_cost!("{2}{U}{U}")).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::static_ability("You may look at the top card of your library any time.", EffectDef::StaticApply { recipient: EffectRecipientDef::Controller, effect: AppliedEffectDef::Rule(AppliedRuleDef::MayLookAtTopOfLibrary) }),
AbilityDef::static_ability("You may play historic lands and cast historic spells from the top of your library. (Artifacts, legendaries, and Sagas are historic.)", EffectDef::StaticApply { recipient: EffectRecipientDef::Controller, effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromTopOfLibrary { restriction: PlayRestrictionDef::new(PlayActionMatcherDef::Any, ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::Supertype(CardSupertype::Legendary), ObjectPredicateDef::Subtype(SubtypeDef::Literal("Saga"))])), cost: TopOfLibraryCostDef::Printed }) }),
abilities::tap_for(ManaColor::Blue)
]),
);

// ACR 182 — Tax Collector
// Audit: unsupported — Cost modifications are read from static abilities on permanents or spells. No resolving operation installs a temporary spell-cost increase lasting until your next turn after this trigger resolves.
pub(in crate::card::sets) static TAX_COLLECTOR_182: CardRecord = CardRecord::new(
    "Tax Collector",
    "825615d2-2bb9-486e-8a05-4244b7ba66c3",
    "Miklós Ligeti",
    crate::card::CardRules::unsupported(),
);

// ACR 218 — Overpowering Attack
// Audit: unsupported — Additional combat and main phases can be scheduled, but there is no freerunning alternative-cast kind or filtered combat-damage history combining Assassin and commander sources.
pub(in crate::card::sets) static OVERPOWERING_ATTACK_218: CardRecord = CardRecord::new(
    "Overpowering Attack",
    "b62be00b-b6cb-47df-aa60-7c77c2f15fd3",
    "Kim Sokol",
    crate::card::CardRules::unsupported(),
);

// ACR 231 — Bayek of Siwa
pub(in crate::card::sets) static BAYEK_OF_SIWA_231: CardRecord = CardRecord::new(
    "Bayek of Siwa",
    "381497ac-653e-4c98-be03-2192d42885f5",
    "JB Casacop",
    CardRules::new_creature(mana_cost!("{3}{R}{W}"), &["Human", "Assassin"], 3, 4).with_supertype(CardSupertype::Legendary).with_morph(&[CostDef::Mana(mana_cost!("{1}{R}{W}"))]).with_abilities(&[
abilities::double_strike(),
AbilityDef::static_ability("During your turn, other historic creatures you control have double strike.", EffectDef::IfCondition { condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You), then: &EffectDef::StaticApply { recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Not(&ObjectPredicateDef::Source), ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::Supertype(CardSupertype::Legendary), ObjectPredicateDef::Subtype(SubtypeDef::Literal("Saga"))])]), &[ZoneKind::Battlefield], PlayerRelation::You), effect: AppliedEffectDef::add_ability(&abilities::double_strike()) } }),
AbilityDef::alternative_cast(&[CostDef::Mana(mana_cost!("{3}"))], crate::card::face_down::disguise_cast(), Some("Disguise {1}{R}{W} (You may cast this card face down for {3} as a 2/2 creature with ward {2}. Turn it face up any time for its disguise cost.)"), EffectDef::None)
]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &SENU_KEEN_EYED_PROTECTOR_8,
    &SHAO_JUN_63,
    &APPLE_OF_EDEN_ISU_RELIC_70,
    &EXCALIBUR_SWORD_OF_EDEN_72,
    &ABSTERGO_ENTERTAINMENT_79,
    &ALEXIOS_DEIMOS_OF_KOSMOS_134,
    &BASIM_IBN_ISHAQ_141,
    &CRYSTAL_SKULL_ISU_SPYGLASS_158,
    &TAX_COLLECTOR_182,
    &OVERPOWERING_ATTACK_218,
    &BAYEK_OF_SIWA_231,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
