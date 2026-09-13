//! Marvel Super Heroes Commander cards used for legend-rule coverage.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CharacteristicOperationDef;
use crate::card::ComparisonDef;
use crate::card::ControlDurationDef;
use crate::card::CopyExceptionsDef;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::DrawEventMatcherDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerRuleDef;
use crate::card::PlayerSetDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SetOperationDef;
use crate::card::SpellCastQueryDef;
use crate::card::SubtypeDef;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::mana_cost;

static NONCREATURE_SPELLS_YOU_CAST: SpellCastQueryDef = SpellCastQueryDef {
    player: PlayerRelation::You,
    spell: ObjectPredicateDef::NoncreatureSpell,
};

static CAST_A_NONCREATURE_SPELL_THIS_TURN: ValueComparisonDef = ValueComparisonDef {
    left: ValueDef::CountSpellsCastThisTurn(&NONCREATURE_SPELLS_YOU_CAST),
    comparison: ComparisonDef::GreaterOrEqual,
    right: ValueDef::Constant(1),
};

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "MSC",
    slug: "marvel-super-heroes-commander",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// MSC 28 — Council of Reeds
pub(in crate::card::sets) static COUNCIL_OF_REEDS: CardRecord = CardRecord::new(
    "Council of Reeds",
    "a0d824ea-75d2-4de5-923b-813bba44e80b",
    "Vlad Petruchik",
CardRules::new_creature(
        mana_cost!("{2}{U}"),
        &["Human", "Scientist", "Hero"],
        2,
        2,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        AbilityDef::static_ability(
            "The \"legend rule\" doesn't apply to creatures you control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::players(PlayerSetDef::Related(
                    PlayerRelation::You,
                )),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::PlayerRule(
                    PlayerRuleDef::LegendRuleDoesNotApplyTo(&ObjectPredicateDef::HasType(
                        CardType::Creature,
                    )),
                )),
            },
        ),
        AbilityDef::triggered_if(
            "At the beginning of combat on your turn, if you've cast a noncreature spell this turn, create a token that's a copy of Council of Reeds.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::ValueComparison(&CAST_A_NONCREATURE_SPELL_THIS_TURN),
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Copy(
                &crate::card::TokenCopyDef {
                    object: &EffectRecipientDef::Source,
                    exceptions: CopyExceptionsDef::NONE,
                },
            ))),
        ),
    ]),
);

// MSC 56 — Loki's Scepter
pub(in crate::card::sets) static LOKI_S_SCEPTER_56: CardRecord = CardRecord::new(
    "Loki's Scepter",
    "57e90938-9225-4c15-b4fb-aad2cced2e6a",
    "L J Koh",
    CardRules::new_artifact(mana_cost!("{2}{R}")).with_supertype(CardSupertype::Legendary).with_abilities(&[
abilities::enters_trigger_with_targets("When Loki's Scepter enters, gain control of target creature until end of turn. Untap that creature. Until end of turn, it becomes a Villain in addition to its other types and gains haste.", &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Creature))], EffectDef::Sequence(&[EffectDef::gain_control(EffectRecipientDef::Target(TargetIndex::PRIMARY), PlayerRefDef::EffectController, ControlDurationDef::UntilEndOfTurn), EffectDef::Untap { object: EffectRecipientDef::Target(TargetIndex::PRIMARY) }, EffectDef::Apply { recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY), effect: AppliedEffectDef::Composite(&[AppliedEffectDef::Characteristic(CharacteristicOperationDef::Subtypes(SetOperationDef::Add(&["Villain"]))), AppliedEffectDef::add_ability(&abilities::haste())]), duration: ResolvedEffectDurationDef::UntilEndOfTurn }])),
AbilityDef::activated_mana("{T}: Add one mana of any color.", &[CostDef::TapSource], EffectDef::AddMana(AddManaEffectDef::any_color()))
]),
);

// MSC 106 — H.E.R.B.I.E., Lovable Robot
pub(in crate::card::sets) static H_E_R_B_I_E_LOVABLE_ROBOT_106: CardRecord = CardRecord::new(
    "H.E.R.B.I.E., Lovable Robot",
    "5e2f9f4c-dbdd-4da6-b08f-1f4d3fcb1328",
    "Nanna Marie Steffensen",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Robot", "Scout"], 1, 1).with_supertype(CardSupertype::Legendary).with_abilities(&[
abilities::flying(),
AbilityDef::triggered_if("At the beginning of combat on your turn, if you've cast a noncreature spell this turn, surveil 1.", TriggerEventDef::StepBegins { step: TurnStepDef::BeginningOfCombat, player: PlayerRelation::You }, &TriggerConditionDef::ValueComparison(&ValueComparisonDef { left: ValueDef::CountSpellsCastThisTurn(&SpellCastQueryDef { player: PlayerRelation::You, spell: ObjectPredicateDef::NoncreatureSpell }), comparison: ComparisonDef::GreaterOrEqual, right: ValueDef::Constant(1) }), abilities::surveil(ValueDef::Constant(1))),
abilities::tap_for(ManaColor::Colorless),
AbilityDef::activated_mana("{1}, {T}: Add one mana of any color.", &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource], EffectDef::AddMana(AddManaEffectDef::any_color()))
]),
);

// MSC 395 — Black Widow, Agile Avenger
pub(in crate::card::sets) static BLACK_WIDOW_AGILE_AVENGER_395: CardRecord = CardRecord::new(
    "Black Widow, Agile Avenger",
    "0b165b56-ad86-4d8c-91e2-1464b2ac5c6d",
    "Junggeun Yoon",
    CardRules::new_creature(mana_cost!("{1}{R}{W}"), &["Human", "Spy", "Hero"], 2, 2).with_supertype(CardSupertype::Legendary).with_abilities(&[
abilities::menace(),
AbilityDef::triggered("Whenever an opponent draws their second card each turn, put a +1/+1 counter on Black Widow and you draw a card.", TriggerEventDef::DrewCard(DrawEventMatcherDef::nth_each_turn(PlayerRelation::Opponent, 2)), EffectDef::Sequence(&[EffectDef::AddCounters { object: EffectRecipientDef::Source, kind: CounterKind::PlusOnePlusOne, amount: ValueDef::Constant(1) }, abilities::draw_cards(ValueDef::Constant(1))]))
]),
);

// MSC 602 — Matt Murdock, Justice Seeker
// Audit: unsupported — Needs a reflexive trigger after the optional mana payment so its target is
// chosen after payment and receives a separate response window; PayOr only continues an effect
// within the original resolution.
pub(in crate::card::sets) static MATT_MURDOCK_JUSTICE_SEEKER_602: CardRecord = CardRecord::new(
    "Matt Murdock, Justice Seeker",
    "ab8e5c5d-e6a0-4666-930a-0a1f475d3c2e",
    "Gintas Galvanauskas",
    CardRules::unsupported(),
);

// MSC 654 — Doctor Doom, Unrivaled
pub(in crate::card::sets) static DOCTOR_DOOM_UNRIVALED_654: CardRecord = CardRecord::new(
    "Doctor Doom, Unrivaled",
    "2973e855-fe93-41a1-a62e-4699ef2c3d1d",
    "Vilhelmas Banys",
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Human", "Sorcerer", "Villain"], 4, 4).with_supertype(CardSupertype::Legendary).with_abilities(&[
abilities::lifelink(),
AbilityDef::activated("{T}: You draw a card and lose 1 life. Then if your library has no cards in it, you win the game. (You win even if you have 0 life or didn't draw a card.)", &[CostDef::TapSource], EffectDef::Sequence(&[abilities::draw_cards(ValueDef::Constant(1)), EffectDef::LoseLife { recipient: EffectRecipientDef::Controller, amount: ValueDef::Constant(1) }, EffectDef::IfCondition { condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef { left: ValueDef::LibrarySize(PlayerRelation::You), comparison: ComparisonDef::Equal, right: ValueDef::Constant(0) }), then: &EffectDef::WinTheGame { player: EffectRecipientDef::Controller } }]))
]),
);

// MSC 678 — Asgardian Inspiration
// Audit: unsupported — DamageKindDef distinguishes combat from any damage, but cannot match only noncombat damage for the graveyard return trigger.
pub(in crate::card::sets) static ASGARDIAN_INSPIRATION_678: CardRecord = CardRecord::new(
    "Asgardian Inspiration",
    "d1fe7eef-2636-484f-b476-4dc9c9dbfddd",
    "Nathaniel Himawan",
    crate::card::CardRules::unsupported(),
);

// MSC 707 — The Vision and Scarlet Witch
pub(in crate::card::sets) static THE_VISION_AND_SCARLET_WITCH_707: CardRecord = CardRecord::new(
    "The Vision and Scarlet Witch",
    "930afb5f-54b7-4cca-8c28-3e48938f3a43",
    "Tyler Walpole",
    CardRules::new_artifact_creature(mana_cost!("{2}{R}{R}"), &["Mutant", "Hero"], 3, 3).with_supertype(CardSupertype::Legendary).with_abilities(&[
abilities::flying(),
AbilityDef::triggered("Whenever you cast a spell, add {R} and put a +1/+1 counter on The Vision and Scarlet Witch.", TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(PlayerRelation::You)), EffectDef::Sequence(&[EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)), EffectDef::AddCounters { object: EffectRecipientDef::Source, kind: CounterKind::PlusOnePlusOne, amount: ValueDef::Constant(1) }]))
]),
);

// MSC 719 — Devil Dinosaur
pub(in crate::card::sets) static DEVIL_DINOSAUR_719: CardRecord = CardRecord::new(
    "Devil Dinosaur",
    "fcb4c268-441f-41cf-8c25-538eefc6a710",
    "Wero Gallo",
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Dinosaur"], 5, 5).with_supertype(CardSupertype::Legendary).with_abilities(&[
abilities::trample(),
AbilityDef::static_ability("Other Dinosaurs you control get +1/+1 and have hexproof. (They can't be the targets of spells or abilities your opponents control.)", EffectDef::StaticApply { recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::All(&[ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dinosaur"))]), ObjectPredicateDef::Not(&ObjectPredicateDef::Source)]), &[ZoneKind::Battlefield], PlayerRelation::You), effect: AppliedEffectDef::Composite(&[AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)), AppliedEffectDef::add_ability(&abilities::hexproof())]) })
]),
);

// MSC 754 — Fogwell's Gym
pub(in crate::card::sets) static FOGWELL_S_GYM_754: CardRecord = CardRecord::new(
    "Fogwell's Gym",
    "e63523d5-0f2c-436a-b411-662eb11fd150",
    "Pace Wilder",
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {R}. This land deals 1 damage to you.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red).with_damage_to_controller(1)),
        ),
        AbilityDef::activated(
            "{2}{R}, {T}, Discard a card: Draw a card.",
            &[
                CostDef::Mana(mana_cost!("{2}{R}")),
                CostDef::TapSource,
                CostDef::discard(ObjectPredicateDef::Any),
            ],
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &COUNCIL_OF_REEDS,
    &LOKI_S_SCEPTER_56,
    &H_E_R_B_I_E_LOVABLE_ROBOT_106,
    &BLACK_WIDOW_AGILE_AVENGER_395,
    &MATT_MURDOCK_JUSTICE_SEEKER_602,
    &DOCTOR_DOOM_UNRIVALED_654,
    &ASGARDIAN_INSPIRATION_678,
    &THE_VISION_AND_SCARLET_WITCH_707,
    &DEVIL_DINOSAUR_719,
    &FOGWELL_S_GYM_754,
];
pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
