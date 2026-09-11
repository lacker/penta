//! Visions cards used by the staged Premodern deck tranche.

use super::CardRecord;
use super::PrintingRecord;
use crate::ParentBinding;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityPredicateDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AddManaEffectDef;
use crate::card::AlternativeCastKindDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::ArrivalAttachmentDef;
use crate::card::AttackDefenderScopeDef;
use crate::card::AttackRestrictionDef;
use crate::card::BasicLandType;
use crate::card::CardNameDef;
use crate::card::CardNameSetDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::ColorSet;
use crate::card::ComparisonDef;
use crate::card::CostDef;
use crate::card::CostModificationDef;
use crate::card::CostQuantityDef;
use crate::card::CounterKind;
use crate::card::DamageEventMatcherDef;
use crate::card::DamageKindDef;
use crate::card::DamageRecipientMatcherDef;
use crate::card::DamageSourceMatcherDef;
use crate::card::DiscardSelectionDef;
use crate::card::DividedTotal;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::InstalledTriggerDef;
use crate::card::KeywordAbility;
use crate::card::ManaColor;
use crate::card::MoveObjectsDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectSetFilterDef;
use crate::card::PayOrDef;
use crate::card::PlayActionMatcherDef;
use crate::card::PlayRestrictionDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SubtypeDef;
use crate::card::TargetChooserDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnPhaseDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

static INSTANT_SPEED_ENCHANTMENT_CLEANUP: AbilityDef = AbilityDef::triggered(
    "If you cast this spell any time a sorcery couldn't have been cast, the controller of the permanent it becomes sacrifices it at the beginning of the next cleanup step.",
    TriggerEventDef::zone_changed(
        ObjectPredicateDef::Source,
        None,
        Some(ZoneKind::Battlefield),
    ),
    EffectDef::IfCondition {
        condition: &TriggerConditionDef::SourceCastAtInstantSpeed,
        then: &EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
            "At the beginning of the next cleanup step, sacrifice this permanent.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Cleanup,
                player: PlayerRelation::Any,
            },
            EffectDef::sacrifice(EffectRecipientDef::Source),
        ))),
    },
);

const fn enters_bounce_or_sacrifice(text: &'static str, costs: &'static [CostDef]) -> AbilityDef {
    abilities::enters_trigger(
        text,
        EffectDef::PayOr(PayOrDef::unless(
            costs,
            &const { EffectDef::sacrifice(EffectRecipientDef::Source) },
        )),
    )
}

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "VIS",
    slug: "visions",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// VIS 1 — Archangel
pub(in crate::card::sets) static ARCHANGEL: CardRecord = CardRecord::new(
    "Archangel",
    "368144bf-d415-48ab-a957-9d7ac1ceb353",
    "Christopher Rush",
    CardRules::new_creature(mana_cost!("{5}{W}{W}"), &["Angel"], 5, 5)
        .with_abilities(&[abilities::flying(), abilities::vigilance()]),
);

// VIS 2 — Daraja Griffin
pub(in crate::card::sets) static DARAJA_GRIFFIN: CardRecord = CardRecord::new(
    "Daraja Griffin",
    "2f7afcaa-9df8-4dd6-89ad-bc2e15f1ec4b",
    "Stuart Griffin",
    // A flier that is also a sideboard card, which against the wrong deck is
    // simply a flier.
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Griffin"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_with_targets(
            "Sacrifice this creature: Destroy target black creature.",
            &[CostDef::SacrificeSource],
            &const {
                [AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Color(ManaColor::Black),
                    ]),
                )]
            },
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ]),
);

// VIS 3 — Equipoise
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EQUIPOISE: CardRecord = CardRecord::new(
    "Equipoise",
    "53783312-3551-4361-ab02-c9651ce2a926",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// VIS 4 — Eye of Singularity
pub(in crate::card::sets) static EYE_OF_SINGULARITY: CardRecord = CardRecord::new(
    "Eye of Singularity",
    "fa84e4ad-738a-4d23-a84c-06c39ff4200b",
    "Eric Peterson",
CardRules::new_enchantment(mana_cost!("{3}{W}"))
        .with_supertype(CardSupertype::World)
        .with_abilities(&[
            abilities::enters_trigger(
                "When this enchantment enters, destroy each permanent with a name other than a basic land name if another permanent has the same name as that permanent. They can't be regenerated.",
                EffectDef::WithRule {
                    rule: AppliedRuleDef::CannotRegenerate,
                    effect: &EffectDef::Destroy {
                        object: EffectRecipientDef::objects(ObjectSetDef::Matching {
                            objects: &ObjectSetDef::Query(ObjectQueryDef::new(
                                ObjectPredicateDef::Not(&ObjectPredicateDef::NameIn(&
                                    CardNameSetDef::BasicLandNames,
                                )),
                                &[ZoneKind::Battlefield],
                            )),
                            object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::NameIn(&
                                CardNameSetDef::NamesAppearingAtLeast {
                                    objects: &ObjectSetDef::Query(ObjectQueryDef::new(
                                        ObjectPredicateDef::Not(&ObjectPredicateDef::NameIn(&
                                            CardNameSetDef::BasicLandNames,
                                        )),
                                        &[ZoneKind::Battlefield],
                                    )),
                                    count: 2,
                                },
                            )),
                        }),
                        then: None,
                    },
                },
            ),
            AbilityDef::triggered(
                "Whenever a permanent with a name other than a basic land name enters, destroy all other permanents with that name. They can't be regenerated.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::NameIn(&
                        CardNameSetDef::BasicLandNames,
                    )),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::WithRule {
                    rule: AppliedRuleDef::CannotRegenerate,
                    effect: &EffectDef::Destroy {
                        object: EffectRecipientDef::objects(ObjectSetDef::ExceptObject {
                            objects: &ObjectSetDef::Matching {
                                objects: &ObjectSetDef::Query(ObjectQueryDef::new(
                                    ObjectPredicateDef::Not(&ObjectPredicateDef::NameIn(&
                                        CardNameSetDef::BasicLandNames,
                                    )),
                                    &[ZoneKind::Battlefield],
                                )),
                                object: ObjectSetFilterDef::Predicate(
                                    &ObjectPredicateDef::NameEquals(CardNameDef::NameOf(
                                        ObjectRefDef::TriggeringObject,
                                    )),
                                ),
                            },
                            object: ObjectRefDef::TriggeringObject,
                        }),
                        then: None,
                    },
                },
            ),
        ]),
);

// VIS 5 — Freewind Falcon
pub(in crate::card::sets) static FREEWIND_FALCON: CardRecord = CardRecord::new(
    "Freewind Falcon",
    "33dc0244-319c-4e15-9083-8d21ad0364d8",
    "Una Fricker",
    // A two-mana flier red cannot burn or block, which in a red format is
    // the difference between a 1/1 and a real clock.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Bird"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::protection_from_color(ManaColor::Red),
    ]),
);

// VIS 6 — Gossamer Chains
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOSSAMER_CHAINS: CardRecord = CardRecord::new(
    "Gossamer Chains",
    "e9917a29-c6b4-4e0a-a301-21868bd27e17",
    "Steve Luke",
    crate::card::CardRules::unsupported(),
);

// VIS 7 — Honorable Passage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HONORABLE_PASSAGE: CardRecord = CardRecord::new(
    "Honorable Passage",
    "6559d301-98bd-40a9-abf4-1079d7283214",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// VIS 8 — Hope Charm
pub(in crate::card::sets) static HOPE_CHARM: CardRecord = CardRecord::new(
    "Hope Charm",
    "a1a8980f-07ab-49b7-b83d-f394952ced57",
    "Greg Spalenka",
    // One mana that is never wrong to draw, because one of the three modes is
    // always the one the board needs.
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Target creature gains first strike until end of turn.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&const { abilities::first_strike() }),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell_with_targets(
                "Target player gains 2 life.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(2),
                },
            ),
            AbilityDef::spell_with_targets(
                "Destroy target Aura.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Aura")),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            ),
        ],
    )),
);

// VIS 9 — Infantry Veteran
pub(in crate::card::sets) static INFANTRY_VETERAN: CardRecord = CardRecord::new(
    "Infantry Veteran",
    "0350470b-feea-4e15-bdf0-850b71dbeea6",
    "Christopher Rush",
    // A point of each on an attacker every turn, which is a one-drop that
    // wins a combat a turn for free.
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Target attacking creature gets +1/+1 until end of turn.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Attacking,
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// VIS 10 — Jamuraan Lion
pub(in crate::card::sets) static JAMURAAN_LION: CardRecord = CardRecord::new(
    "Jamuraan Lion",
    "bfc681f5-9fff-48b6-98d9-e85c85e582a3",
    "Stuart Griffin",
    // Tapping to stop a blocker, so the 3/1 body and the ability compete
    // for the same turn.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Cat"], 3, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{W}, {T}: Target creature can't block this turn.",
            &[CostDef::Mana(mana_cost!("{W}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// VIS 11 — Knight of Valor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KNIGHT_OF_VALOR: CardRecord = CardRecord::new(
    "Knight of Valor",
    "25aa80ae-bb17-4e52-a269-efe75cf4c041",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// VIS 12 — Longbow Archer
pub(in crate::card::sets) static LONGBOW_ARCHER: CardRecord = CardRecord::new(
    "Longbow Archer",
    "e2ee185d-f5ae-4b1d-90a4-840182f87ab8",
    "Eric Peterson",
    // Two keywords that answer opposite halves of a board: it kills the
    // ground creature first and catches the flier.
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Human", "Soldier", "Archer"], 2, 2)
        .with_abilities(&[abilities::reach(), abilities::first_strike()]),
);

// VIS 13 — Miraculous Recovery
pub(in crate::card::sets) static MIRACULOUS_RECOVERY: CardRecord = CardRecord::new(
    "Miraculous Recovery",
    "76fecb31-790a-4454-918e-5aeb253021f0",
    "Brian Horton",
    // Five mana at instant speed, so it can rebuy the creature that just
    // died in combat and get a counter for the trouble.
    CardRules::new_instant(mana_cost!("{4}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Return target creature card from your graveyard to the battlefield. Put a +1/+1 \
         counter on it.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            },
        )],
        // The counter goes on the permanent that arrived, not on the
        // card that was targeted, so the move binds what it produced and
        // the follow-up names that binding.
        EffectDef::MoveObjects(MoveObjectsDef {
            input: ObjectSetDef::One(ObjectRefDef::Target(TargetIndex::PRIMARY)),
            from: Some(ZoneKind::Graveyard),
            zone: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
            moved: Some(ParentBinding),
            then: &EffectDef::AddCounters {
                object: EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        }),
    )),
);

// VIS 14 — Parapet
pub(in crate::card::sets) static PARAPET: CardRecord = CardRecord::new(
    "Parapet",
    "a7bbcaa9-edbf-48ad-bcd2-65e8fb9bb938",
    "Mark Poole",
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_abilities(&[
        abilities::flash(),
        INSTANT_SPEED_ENCHANTMENT_CLEANUP,
        AbilityDef::static_ability(
            "Creatures you control get +0/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(1),
                ),
            },
        ),
    ]),
);

// VIS 15 — Peace Talks
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PEACE_TALKS: CardRecord = CardRecord::new(
    "Peace Talks",
    "21da279d-a723-4902-bf84-dfe2c569d4c8",
    "Roger Raupp",
    crate::card::CardRules::unsupported(),
);

// VIS 16 — Relic Ward
pub(in crate::card::sets) static RELIC_WARD: CardRecord = CardRecord::new(
    "Relic Ward",
    "f0459667-b7da-43bd-b981-0e515432d147",
    "John Coulthart",
    CardRules::new_enchantment(mana_cost!("{1}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::flash(),
            INSTANT_SPEED_ENCHANTMENT_CLEANUP,
            abilities::enchant_artifact(),
            AbilityDef::static_ability(
                "Enchanted artifact has shroud.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::shroud()),
                },
            ),
        ]),
);

// VIS 17 — Remedy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REMEDY: CardRecord = CardRecord::new(
    "Remedy",
    "2a0b7162-4422-4dfb-a6ca-8d89fa74e6dc",
    "Zina Saunders",
    crate::card::CardRules::unsupported(),
);

// VIS 18 — Resistance Fighter
pub(in crate::card::sets) static RESISTANCE_FIGHTER: CardRecord = CardRecord::new(
    "Resistance Fighter",
    "21250bdb-9431-41b3-9fef-d66a4d3f6ecd",
    "Cecil Fernando",
CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "Sacrifice this creature: Prevent all combat damage target creature would deal this turn.",
            &[CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::PreventDamage {
                prevention: crate::card::DamagePreventionDef::unlimited(
                    crate::card::DamageEventMatcherDef::combat_from(ObjectRefDef::Target(
                        TargetIndex::PRIMARY,
                    )),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// VIS 19 — Retribution of the Meek
pub(in crate::card::sets) static RETRIBUTION_OF_THE_MEEK: CardRecord = CardRecord::new(
    "Retribution of the Meek",
    "860b8633-1bfc-426a-8666-5e6a584d4525",
    "Nathalie Hertz",
    // A wrath that spares the small creatures, which in practice means it
    // spares whichever board is not winning.
    CardRules::new_sorcery(mana_cost!("{2}{W}")).with_ability(AbilityDef::spell(
        "Destroy all creatures with power 4 or greater. They can't be regenerated.",
        EffectDef::WithRule {
            rule: AppliedRuleDef::CannotRegenerate,
            effect: &const {
                EffectDef::Destroy {
                    object: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::PowerAtLeast(4),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    then: None,
                }
            },
        },
    )),
);

// VIS 20 — Righteous Aura
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIGHTEOUS_AURA: CardRecord = CardRecord::new(
    "Righteous Aura",
    "fed82843-2853-42d3-bcf6-b831032b7a69",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// VIS 21 — Sun Clasp
pub(in crate::card::sets) static SUN_CLASP: CardRecord = CardRecord::new(
    "Sun Clasp",
    "e3f1fb74-bc08-4c3b-9fbe-da6973aaeaa2",
    "John Coulthart",
    // Four points of stats for two mana, and the creature can be picked up
    // rather than lost when the removal comes.
    CardRules::new_enchantment(mana_cost!("{1}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+3.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(3),
                    ),
                },
            ),
            AbilityDef::activated(
                "{W}: Return enchanted creature to its owner's hand.",
                &[CostDef::Mana(mana_cost!("{W}"))],
                EffectDef::move_to_zone(
                    EffectRecipientDef::AttachedPermanent,
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            ),
        ]),
);

// VIS 22 — Teferi's Honor Guard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEFERI_S_HONOR_GUARD: CardRecord = CardRecord::new(
    "Teferi's Honor Guard",
    "4177d5bf-db48-4bbf-bbd4-ee6313031920",
    "Cecil Fernando",
    crate::card::CardRules::unsupported(),
);

// VIS 23 — Tithe
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TITHE: CardRecord = CardRecord::new(
    "Tithe",
    "aae08938-e563-4322-b2eb-db81913ea730",
    "Jon J Muth",
    crate::card::CardRules::unsupported(),
);

// VIS 24 — Warrior's Honor
pub(in crate::card::sets) static WARRIOR_S_HONOR: CardRecord = CardRecord::new(
    "Warrior's Honor",
    "7babd273-3e20-4cf9-bf21-c602eb729fc5",
    "D. Alexander Gregory",
    // Three mana to win one combat step, which is what a go-wide deck is
    // actually buying.
    CardRules::new_instant(mana_cost!("{2}{W}")).with_ability(AbilityDef::spell(
        "Creatures you control get +1/+1 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(1),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// VIS 25 — Zhalfirin Crusader
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZHALFIRIN_CRUSADER: CardRecord = CardRecord::new(
    "Zhalfirin Crusader",
    "d8ed802f-6e54-4fed-a71e-6d404c2c664b",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// VIS 26 — Betrayal
pub(in crate::card::sets) static BETRAYAL: CardRecord = CardRecord::new(
    "Betrayal",
    "7f9b5c75-882e-4fe4-827f-584080e91485",
    "Gary Leach",
    CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature an opponent controls",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::Opponent),
                        owner: None,
                    },
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::triggered(
                "Whenever enchanted creature becomes tapped, you draw a card.",
                TriggerEventDef::tapped(ObjectPredicateDef::AttachedToSource),
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// VIS 27 — Breezekeeper
// Audit: unsupported — Needs phasing as a keyword. EffectDef::PhaseOut is a one-shot that phases a permanent out once; the keyword is a static that phases its permanent in and out before every one of its controller's untap steps.
pub(in crate::card::sets) static BREEZEKEEPER: CardRecord = CardRecord::new(
    "Breezekeeper",
    "beaefa77-6e4a-4724-a443-fa6b45803db5",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// VIS 28 — Chronatog
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHRONATOG: CardRecord = CardRecord::new(
    "Chronatog",
    "05ada02f-04e9-4269-b04a-97a7eaac2c46",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// VIS 29 — Cloud Elemental
pub(in crate::card::sets) static CLOUD_ELEMENTAL: CardRecord = CardRecord::new(
    "Cloud Elemental",
    "4f2a5146-cf2e-40c0-b498-06e611343196",
    "Adam Rex",
    // A 2/3 flier that only ever fights in the air, which is the discount
    // blue pays for a body that would otherwise be a fine blocker.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Elemental"], 2, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "This creature can block only creatures with flying.",
            // A restriction on the blocker rather than the attacker, so
            // it stops this creature from blocking on the ground without
            // saying anything about who may block it.
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::can_block_only(
                    ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                )),
            },
        ),
    ]),
);

// VIS 30 — Desertion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DESERTION: CardRecord = CardRecord::new(
    "Desertion",
    "9a2a1779-af08-4a9a-aba4-e6892ce2332c",
    "Richard Kane Ferguson",
    crate::card::CardRules::unsupported(),
);

// VIS 31 — Dream Tides
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DREAM_TIDES: CardRecord = CardRecord::new(
    "Dream Tides",
    "3bd292a0-ec08-4250-8d75-0802e985d6e6",
    "Jerry Tiritilli",
    crate::card::CardRules::unsupported(),
);

// VIS 32 — Flooded Shoreline
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLOODED_SHORELINE: CardRecord = CardRecord::new(
    "Flooded Shoreline",
    "49db9f58-380f-496e-9d3d-6776d30fb564",
    "Romas Kukalis",
    crate::card::CardRules::unsupported(),
);

// VIS 33 — Foreshadow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORESHADOW: CardRecord = CardRecord::new(
    "Foreshadow",
    "d54c51de-bfac-4198-a7c4-37b4db74e525",
    "George Pratt",
    crate::card::CardRules::unsupported(),
);

// VIS 34 — Impulse
pub(in crate::card::sets) static IMPULSE: CardRecord = CardRecord::new(
    "Impulse",
    "9d710a97-062f-4773-b6c6-8aeddeb3b6e8",
    "Bryan Talbot",
CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell(
        "Look at the top four cards of your library. Put one of them into your hand and the rest on the bottom of your library in any order.",
        abilities::look_at_top_cards_choose_to_hand_rest_bottom(
            ValueDef::Constant(4),
            ObjectPredicateDef::Any,
            1,
            1,
        ),
    )),
);

// VIS 35 — Inspiration
pub(in crate::card::sets) static INSPIRATION: CardRecord = CardRecord::new(
    "Inspiration",
    "5247d0b0-660e-4f27-8e76-62effbe12221",
    "Zina Saunders",
    CardRules::new_instant(mana_cost!("{3}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target player draws two cards.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(2),
        },
    )),
);

// VIS 36 — Knight of the Mists
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KNIGHT_OF_THE_MISTS: CardRecord = CardRecord::new(
    "Knight of the Mists",
    "37924cbc-fb9d-4906-9ad2-9b6d4ccfff0f",
    "Harold McNeill",
    crate::card::CardRules::unsupported(),
);

// VIS 37 — Man-o'-War
pub(in crate::card::sets) static MAN_O_WAR: CardRecord = CardRecord::new(
    "Man-o'-War",
    "4dbf9bf9-75cd-4b25-a3a1-43b7e029700b",
    "Jon J Muth",
    // Any creature, its own included, which is the out when it is the only
    // one on the board.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Jellyfish"], 2, 2).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, return target creature to its owner's hand.",
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
);

// VIS 38 — Mystic Veil
pub(in crate::card::sets) static MYSTIC_VEIL: CardRecord = CardRecord::new(
    "Mystic Veil",
    "7ddb640d-5c54-4d0a-b8c2-e22fe04f96c2",
    "D. Alexander Gregory",
    CardRules::new_enchantment(mana_cost!("{1}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::flash(),
            INSTANT_SPEED_ENCHANTMENT_CLEANUP,
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature has shroud.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::shroud()),
                },
            ),
        ]),
);

// VIS 39 — Ovinomancer
pub(in crate::card::sets) static OVINOMANCER: CardRecord = CardRecord::new(
    "Ovinomancer",
    "ae4f0988-4194-4481-a6b7-27753261174a",
    "Kev Walker",
CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Wizard"], 0, 1).with_abilities(&[
        enters_bounce_or_sacrifice(
            "When this creature enters, sacrifice it unless you return three basic lands you control to their owner's hand.",
            &[
                CostDef::MovePermanentMatching {
                    object: ObjectPredicateDef::HasAnyBasicLandType(&BasicLandType::ALL),
                    zone: ZoneKind::Hand,
                },
                CostDef::MovePermanentMatching {
                    object: ObjectPredicateDef::HasAnyBasicLandType(&BasicLandType::ALL),
                    zone: ZoneKind::Hand,
                },
                CostDef::MovePermanentMatching {
                    object: ObjectPredicateDef::HasAnyBasicLandType(&BasicLandType::ALL),
                    zone: ZoneKind::Hand,
                },
            ],
        ),
        AbilityDef::activated_with_targets(
            "{T}, Return this creature to its owner's hand: Destroy target creature. It can't be regenerated. That creature's controller creates a 0/1 green Sheep creature token.",
            &[CostDef::TapSource, CostDef::ReturnSourceToHand],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotRegenerate),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
                EffectDef::create_creature_token(&["Sheep"], &[ManaColor::Green], 0, 1)
                    .with_controller(PlayerRefDef::ControllerOf(ObjectRefDef::Target(
                        TargetIndex::PRIMARY,
                    ))),
            ]),
        ),
    ]),
);

// VIS 40 — Prosperity
pub(in crate::card::sets) static PROSPERITY: CardRecord = CardRecord::new(
    "Prosperity",
    "3fa5e806-3cf2-4241-b45d-a05d2b715efd",
    "Dan Frazier",
    // Symmetrical draw, so it is a combo piece rather than a card-advantage
    // spell: the deck that casts it has a use for their cards too.
    CardRules::new_sorcery(mana_cost!("{X}{U}")).with_ability(AbilityDef::spell(
        "Each player draws X cards.",
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::Any)),
            amount: ValueDef::ChosenX,
        },
    )),
);

// VIS 41 — Rainbow Efreet
pub(in crate::card::sets) static RAINBOW_EFREET: CardRecord = CardRecord::new(
    "Rainbow Efreet",
    "1d6f03a6-3665-40e4-ae68-640913972770",
    "Nathalie Hertz",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Efreet"], 3, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{U}{U}: This creature phases out.",
            &[CostDef::Mana(mana_cost!("{U}{U}"))],
            EffectDef::PhaseOut {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// VIS 42 — Shimmering Efreet
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIMMERING_EFREET: CardRecord = CardRecord::new(
    "Shimmering Efreet",
    "23c5704f-5856-4422-9d82-14558dbe1434",
    "Thomas Gianni",
    crate::card::CardRules::unsupported(),
);

// VIS 43 — Shrieking Drake
pub(in crate::card::sets) static SHRIEKING_DRAKE: CardRecord = CardRecord::new(
    "Shrieking Drake",
    "63971a64-c5f3-4d1f-ae0d-489d7d5b18f0",
    "Ian Miller",
    CardRules::new_creature(mana_cost!("{U}"), &["Drake"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, return a creature you control to its owner's hand.",
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Object(crate::ParentBinding),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 1,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::move_to_zone(
                    EffectRecipientDef::object(ObjectRefDef::Binding(crate::ParentBinding)),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            }),
        ),
    ]),
);

// VIS 44 — Teferi's Realm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEFERI_S_REALM: CardRecord = CardRecord::new(
    "Teferi's Realm",
    "aba3e4ea-2241-4f1e-a46b-70f512fe729e",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// VIS 45 — Three Wishes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THREE_WISHES: CardRecord = CardRecord::new(
    "Three Wishes",
    "dbb2b253-7023-44d1-963b-eae98d48f498",
    "George Pratt",
    crate::card::CardRules::unsupported(),
);

// VIS 46 — Time and Tide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIME_AND_TIDE: CardRecord = CardRecord::new(
    "Time and Tide",
    "152b348a-0301-4d45-a2c1-d78802c445ba",
    "George Pratt",
    crate::card::CardRules::unsupported(),
);

// VIS 47 — Undo
pub(in crate::card::sets) static UNDO: CardRecord = CardRecord::new(
    "Undo",
    "2bef942e-9d17-4d40-a4c9-8be715e73a08",
    "Terese Nielsen",
    // Two creatures back to hand for three mana at sorcery speed, which is
    // tempo rather than an answer.
    CardRules::new_sorcery(mana_cost!("{1}{U}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return two target creatures to their owners' hands.",
        // Exactly two, so it needs two legal targets to be cast at all.
        &[AbilityTargetDef::exactly_value(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
            ValueDef::Constant(2),
        )],
        EffectDef::move_to_zone(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ZoneKind::Hand,
            ZonePlacement::Top,
        ),
    )),
);

// VIS 48 — Vanishing
pub(in crate::card::sets) static VANISHING: CardRecord = CardRecord::new(
    "Vanishing",
    "8d1fb805-1382-458c-b98d-4491f13833b6",
    "John Matson",
    CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::activated(
                "{U}{U}: Enchanted creature phases out.",
                &[CostDef::Mana(mana_cost!("{U}{U}"))],
                EffectDef::PhaseOut {
                    object: EffectRecipientDef::AttachedPermanent,
                },
            ),
        ]),
);

// VIS 49 — Vision Charm
pub(in crate::card::sets) static VISION_CHARM: CardRecord = CardRecord::new(
    "Vision Charm",
    "78b384d3-3adf-493a-8b89-bfe68fd1c3e2",
    "Greg Spalenka",
// One blue for whichever of three the turn calls for. The deck wants the
    // land mode to strand an opponent's colours, and the phase-out to answer
    // an artifact at instant speed.
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        // The printed first choice is "a land type", which includes the nonbasic
        // ones. Nothing in this card pool carries a nonbasic land subtype, so the
        // choice offered is over the basic types alone.
        &[
            AbilityDef::spell_with_targets(
                "Target player mills four cards.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::Mill {
                    player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(4),
                },
            ),
            AbilityDef::spell(
                "Choose a land type and a basic land type. Each land of the first chosen type becomes the second chosen type until end of turn.",
                EffectDef::SubstituteBasicLandTypeUntilEndOfTurn {
                    chooser: PlayerRefDef::EffectController,
                },
            ),
            AbilityDef::spell_with_targets(
                "Target artifact phases out.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                )],
                EffectDef::PhaseOut {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
        ],
    )),
);

// VIS 50 — Waterspout Djinn
pub(in crate::card::sets) static WATERSPOUT_DJINN: CardRecord = CardRecord::new(
    "Waterspout Djinn",
    "6946a75e-e9d1-4a56-86d1-dd81f7b1b125",
    "Thomas Gianni",
CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Djinn"], 4, 4).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "At the beginning of your upkeep, sacrifice this creature unless you return an untapped Island you control to its owner's hand.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::PayOr(PayOrDef::unless(
                &[CostDef::MovePermanentMatching {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
                    ]),
                    zone: ZoneKind::Hand,
                }],
                &EffectDef::sacrifice(EffectRecipientDef::Source),
            )),
        ),
    ]),
);

// VIS 51 — Aku Djinn
pub(in crate::card::sets) static AKU_DJINN: CardRecord = CardRecord::new(
    "Aku Djinn",
    "369a5df5-fc36-476c-84f4-ec4bdeb4f9d2",
    "Terese Nielsen",
// Five trampling power for five, on the understanding that everything
    // opposite it grows every turn you keep it.
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Djinn"], 5, 6).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered(
            "At the beginning of your upkeep, put a +1/+1 counter on each creature each opponent controls.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Opponent,
                ),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// VIS 52 — Blanket of Night
pub(in crate::card::sets) static BLANKET_OF_NIGHT: CardRecord = CardRecord::new(
    "Blanket of Night",
    "fe012fd0-9ff0-4436-a890-3ab436e42201",
    "Cliff Nielsen",
    CardRules::new_enchantment(mana_cost!("{1}{B}{B}")).with_ability(AbilityDef::static_ability(
        "Each land is a Swamp in addition to its other land types.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Land),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::add_basic_land_types(&[BasicLandType::Swamp]),
        },
    )),
);

// VIS 53 — Brood of Cockroaches
pub(in crate::card::sets) static BROOD_OF_COCKROACHES: CardRecord = CardRecord::new(
    "Brood of Cockroaches",
    "30b6150e-7d0c-4361-b99b-79de96dfc53a",
    "Geofrey Darrow & I. Rabarot",
CardRules::new_creature(mana_cost!("{1}{B}"), &["Insect"], 1, 1).with_ability(
        abilities::dies_trigger(
            "When this creature is put into your graveyard from the battlefield, at the beginning of the next end step, you lose 1 life and return this card to your hand.",
            EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                "At the beginning of the next end step, you lose 1 life and return this card to your hand.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::Any,
                },
                EffectDef::Sequence(&[
                    EffectDef::LoseLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Source,
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                ]),
            ))),
        ),
    ),
);

// VIS 54 — Coercion
pub(in crate::card::sets) static COERCION: CardRecord = CardRecord::new(
    "Coercion",
    "f3b07d33-f5f5-45cc-b2ac-360eaf2d4146",
    "DiTerlizzi",
    // Three mana to take the best card in their hand, with no life cost and
    // no restriction on what you may take.
    CardRules::new_sorcery(mana_cost!("{2}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target opponent reveals their hand. You choose a card from it. That player discards \
         that card.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Opponent),
        )],
        // Any card, not only a nonland one: unlike the later printings of
        // this effect, a land is a legal choice.
        EffectDef::Sequence(&abilities::reveal_hand_and_discard_chosen_card(
            PlayerRefDef::Target(TargetIndex::PRIMARY),
            ObjectPredicateDef::Any,
        )),
    )),
);

// VIS 55 — Crypt Rats
// Audit: unsupported — Needs "spend only <colour> mana on X" for an activated ability. The restriction is card-level and the payment layer reads it only for ManaPaymentPurpose::Spell, so an ability's X is unrestricted; implementing this without it would let any mana pay X and make the card strictly better than printed.
pub(in crate::card::sets) static CRYPT_RATS: CardRecord = CardRecord::new(
    "Crypt Rats",
    "736455f6-c1b3-4a5a-a91f-a0cd3986ed53",
    "Paul Lee",
    crate::card::CardRules::unsupported(),
);

// VIS 56 — Dark Privilege
pub(in crate::card::sets) static DARK_PRIVILEGE: CardRecord = CardRecord::new(
    "Dark Privilege",
    "10d2cf44-cc20-4a37-81ae-930f8c6d0896",
    "Tom Kyffin",
    // A regeneration shield paid in other creatures, which is what a deck
    // making tokens has instead of mana.
    CardRules::new_enchantment(mana_cost!("{1}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
            AbilityDef::activated(
                "Sacrifice a creature: Regenerate enchanted creature.",
                &[CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    controller: PlayerRelation::You,
                }],
                EffectDef::Regenerate {
                    object: EffectRecipientDef::AttachedPermanent,
                },
            ),
        ]),
);

// VIS 57 — Death Watch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEATH_WATCH: CardRecord = CardRecord::new(
    "Death Watch",
    "0e939d8f-6989-4884-989b-9cba566c9963",
    "Brian Horton",
    crate::card::CardRules::unsupported(),
);

// VIS 58 — Desolation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DESOLATION: CardRecord = CardRecord::new(
    "Desolation",
    "3b186460-d2af-4912-ba19-95b2cb5f1639",
    "George Pratt",
    crate::card::CardRules::unsupported(),
);

// VIS 59 — Fallen Askari
pub(in crate::card::sets) static FALLEN_ASKARI: CardRecord = CardRecord::new(
    "Fallen Askari",
    "00107210-313f-49c1-84ff-92628f75b764",
    "Adrian Smith",
    // Flanking makes it a poor thing to block, and it cannot block back --
    // so it is two mana that only ever points one way.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Knight"], 2, 2).with_abilities(&[
        abilities::flanking(),
        AbilityDef::static_ability(
            "This creature can't block.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
            },
        ),
    ]),
);

// VIS 60 — Forbidden Ritual
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORBIDDEN_RITUAL: CardRecord = CardRecord::new(
    "Forbidden Ritual",
    "f5327e6d-db4e-4b44-a00e-b764e80b8946",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// VIS 61 — Funeral Charm
pub(in crate::card::sets) static FUNERAL_CHARM: CardRecord = CardRecord::new(
    "Funeral Charm",
    "e79d7240-2014-4838-bace-80666192a73e",
    "Greg Spalenka",
    // Instant-speed discard is the mode that matters; the other two are why it
    // is never a dead card against the wrong deck.
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Target player discards a card.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            ),
            AbilityDef::spell_with_targets(
                "Target creature gets +2/-1 until end of turn.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(-1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell_with_targets(
                "Target creature gains swampwalk until end of turn.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(
                        &const { abilities::landwalk(BasicLandType::Swamp) },
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    )),
);

// VIS 62 — Infernal Harvest
pub(in crate::card::sets) static INFERNAL_HARVEST: CardRecord = CardRecord::new(
    "Infernal Harvest",
    "ccf85ac9-f5d8-4a36-aa6c-3a31427a0348",
    "Nathalie Hertz",
CardRules::new_sorcery(mana_cost!("{1}{B}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, return X Swamps you control to their owner's hand.\nInfernal Harvest deals X damage divided as you choose among any number of target creatures.",
            &[AbilityTargetDef {
                predicate: AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                minimum: 0,
                maximum: AbilityTargetDef::UNLIMITED,
                exact_count: None,
                divided_total: Some(DividedTotal::ChosenX),
                another: false,
                excludes_source: false,
                chooser: TargetChooserDef::Controller,
            }],
            CostDef::return_to_hand(
                ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Swamp]),
                CostQuantityDef::ChosenX,
            ),
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::DividedAmongTargets,
            ),
        ),
    ),
);

// VIS 63 — Kaervek's Spite
// Audit: unsupported — Generic cost lists can combine the discard and sacrifice
// clauses, but no cost quantity means "all permanents you control."
pub(in crate::card::sets) static KAERVEK_S_SPITE: CardRecord = CardRecord::new(
    "Kaervek's Spite",
    "d385b9e5-e13d-4098-ba74-ea55bde164d9",
    "Bryan Talbot",
    crate::card::CardRules::unsupported(),
);

// VIS 64 — Necromancy
pub(in crate::card::sets) static NECROMANCY: CardRecord = CardRecord::new(
    "Necromancy",
    "311a6257-dd77-4bb6-81cb-c8e7862350f3",
    "Pete Venters",
// Three mana for anything in any graveyard, at instant speed if you are
    // willing to give it back at cleanup. It is typed an Aura from the
    // start rather than becoming one as it enters: the difference is only
    // visible while the spell is on the stack, and nothing there reads it.
    CardRules::new_enchantment(mana_cost!("{2}{B}"))
        .with_subtypes(&["Aura"])
        // "Enchant creature put onto the battlefield with Necromancy" is
        // narrower than this, but the card guarantees the narrowing itself:
        // it only ever attaches to the creature it just reanimated.
        .enchanting(ObjectPredicateDef::HasType(CardType::Creature))
        .with_abilities(&[
            // "As though it had flash" and having flash differ only in what
            // reads the keyword, and nothing in the pool reads an
            // enchantment's.
            abilities::flash(),
            // Any graveyard, not only your own: the card is a reanimation spell for
            // whatever died, whoever owned it.
            AbilityDef::triggered_if_with_targets("When this enchantment enters, if it's on the battlefield, it becomes an Aura with \"enchant creature put onto the battlefield with Necromancy.\" Put target creature card from a graveyard onto the battlefield under your control and attach this enchantment to it.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                // "If it's on the battlefield" is an intervening if, read again as
                // the trigger resolves: an enchantment answered in that window
                // reanimates nothing rather than pulling a creature out of a
                // graveyard from somewhere else.
                &const { TriggerConditionDef::SourceOnBattlefield },
                &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: None,
                },
            // The reanimation and the attachment are one step: what arrives is a new
            // object, so a following effect would have nothing left to name.
            )], EffectDef::Sequence(&const {
                [
                    // The reanimation and the attachment are one step: what arrives is a new
                    // object, so a following effect would have nothing left to name.
                    EffectDef::WithBattlefieldArrival {
                        effect: &const {
                            EffectDef::move_to_zone(
                                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                ZoneKind::Battlefield,
                                ZonePlacement::Top,
                            )
                        },
                        arrival: crate::card::BattlefieldArrivalDef {
                            controller: Some(PlayerRelation::You),
                            attachment: Some(ArrivalAttachmentDef::SourceToArrival),
                            ..crate::card::BattlefieldArrivalDef::DEFAULT
                        },
                    },
                    EffectDef::IfCondition {
                        condition: &const { TriggerConditionDef::SourceCastAtInstantSpeed },
                        // "The controller of the permanent it becomes sacrifices it at the
                        // beginning of the next cleanup step" -- the price of casting it at
                        // instant speed, and nothing at all when it was cast on your own turn.
                        then: &const {
                            EffectDef::InstallTrigger(InstalledTriggerDef::once(&const {
                                AbilityDef::triggered(
                                    "At the beginning of the next cleanup step, sacrifice this enchantment.",
                                    TriggerEventDef::StepBegins {
                                        step: TurnStepDef::Cleanup,
                                        player: PlayerRelation::Any,
                                    },
                                    EffectDef::sacrifice(EffectRecipientDef::Source),
                                )
                            }))
                        },
                    },
                ]
            })),
            AbilityDef::triggered(
                "When this enchantment leaves the battlefield, that creature's controller sacrifices it.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    None,
                ),
                EffectDef::sacrifice(EffectRecipientDef::AttachedPermanent),
            ),
        ]),
);

// VIS 65 — Necrosavant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NECROSAVANT: CardRecord = CardRecord::new(
    "Necrosavant",
    "e70cd5fa-ae66-4ea4-90d2-28af2aa34dd4",
    "John Coulthart",
    crate::card::CardRules::unsupported(),
);

// VIS 66 — Nekrataal
pub(in crate::card::sets) static NEKRATAAL: CardRecord = CardRecord::new(
    "Nekrataal",
    "dba3e342-88b7-4692-a3f7-a3f56c0cf6b5",
    "Adrian Smith",
CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Human", "Assassin"], 2, 1)
        .with_abilities(&[
            abilities::first_strike(),
            abilities::enters_trigger_with_targets(
                "When this creature enters, destroy target nonartifact, nonblack creature. That creature can't be regenerated.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Artifact)),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::Black)),
                    ]),
                )],
                EffectDef::WithRule {
                    rule: AppliedRuleDef::CannotRegenerate,
                    effect: &EffectDef::Destroy {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        then: None,
                    },
                },
            ),
        ]),
);

// VIS 67 — Pillar Tombs of Aku
pub(in crate::card::sets) static PILLAR_TOMBS_OF_AKU: CardRecord = CardRecord::new(
    "Pillar Tombs of Aku",
    "153f93fd-4f2c-4dce-a774-4483031ed532",
    "Terese Nielsen",
CardRules::new_enchantment(mana_cost!("{2}{B}{B}"))
        .with_supertype(CardSupertype::World)
        .with_ability(AbilityDef::triggered(
            "At the beginning of each player's upkeep, that player may sacrifice a creature of their choice. If that player doesn't, they lose 5 life and you sacrifice this enchantment.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::Any,
            },
            EffectDef::PayOr(
                PayOrDef::unless(
                    &[CostDef::sacrifice_permanent(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    )],
                    &EffectDef::Sequence(&[
                        EffectDef::LoseLife {
                            recipient: EffectRecipientDef::EventPlayer,
                            amount: ValueDef::Constant(5),
                        },
                        EffectDef::sacrifice(EffectRecipientDef::Source),
                    ]),
                )
                .with_payer(PlayerSetDef::One(PlayerRefDef::EventPlayer)),
            ),
        )),
);

// VIS 68 — Python
pub(in crate::card::sets) static PYTHON: CardRecord = CardRecord::new(
    "Python",
    "e7e99969-6c21-4de6-ba57-44ef7f9c8c47",
    "Steve White",
    // A vanilla 3/2 for three, which is the rate black pays when it is not
    // buying an ability with the same card.
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Snake"], 3, 2),
);

// VIS 69 — Suq'Ata Assassin
pub(in crate::card::sets) static SUQ_ATA_ASSASSIN: CardRecord = CardRecord::new(
    "Suq'Ata Assassin",
    "1b7178c6-f989-437d-83e3-04b9817f2c54",
    "Gary Gianni",
// Fear on the poison body, so the ten attacks are much easier to get
    // through than the Cobra's.
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Human", "Assassin"], 1, 1).with_abilities(&[
        abilities::fear(),
        AbilityDef::triggered(
            "Whenever this creature attacks and isn't blocked, defending player gets a poison counter.",
            TriggerEventDef::AttacksAndIsNotBlocked {
                attacker: ObjectPredicateDef::Source,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::DefenderOfSource,
                kind: CounterKind::Poison,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// VIS 70 — Tar Pit Warrior
pub(in crate::card::sets) static TAR_PIT_WARRIOR: CardRecord = CardRecord::new(
    "Tar Pit Warrior",
    "e1283190-094e-4a9f-bf67-f9fd05778744",
    "George Pratt",
    // A 3/4 for three, which is above rate until anybody points anything
    // at it -- including your own trick.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Cyclops", "Warrior"], 3, 4).with_ability(
        AbilityDef::triggered(
            "When this creature becomes the target of a spell or ability, sacrifice it.",
            // Any spell or ability, including its controller's own: a
            // pump spell kills it just as surely as removal does.
            TriggerEventDef::becomes_targeted(ObjectPredicateDef::Any),
            EffectDef::sacrifice(EffectRecipientDef::Source),
        ),
    ),
);

// VIS 71 — Urborg Mindsucker
pub(in crate::card::sets) static URBORG_MINDSUCKER: CardRecord = CardRecord::new(
    "Urborg Mindsucker",
    "78405864-fc83-47ab-9238-8e0464a700ec",
    "DiTerlizzi",
CardRules::new_creature(mana_cost!("{2}{B}"), &["Horror"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{B}, Sacrifice this creature: Target opponent discards a card at random. Activate only as a sorcery.",
            &[
                CostDef::Mana(mana_cost!("{B}")),
                CostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                PlayerRelation::Opponent,
            ))],
            EffectDef::Discard {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::Random,
                then: None,
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ),
);

// VIS 72 — Vampiric Tutor
/// Imperial Seal prints this clause word for word, so the two share it: the
/// only difference between the cards is that one is an instant.
pub(in crate::card::sets) static VAMPIRIC_TUTOR_EFFECT: [EffectDef; 2] = [
    EffectDef::SearchZone {
        player: EffectRecipientDef::Controller,
        source: ZoneKind::Library,
        object: ObjectPredicateDef::Any,
        minimum: 0,
        maximum: ValueDef::Constant(1),
        reveal: false,
        destination: ZoneKind::Library,
        placement: ZonePlacement::Top,
        shuffle: true,
        enters_tapped: false,
        attachment: None,
        binding: None,
        then: None,
    },
    EffectDef::LoseLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(2),
    },
];

pub(in crate::card::sets) static VAMPIRIC_TUTOR: CardRecord = CardRecord::new(
    "Vampiric Tutor",
    "0a07cba3-2e8d-48ec-a6f8-4d2edfcd833d",
    "Gary Leach",
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell(
        "Search your library for a card, then shuffle and put that card on top. You lose 2 life.",
        EffectDef::Sequence(&VAMPIRIC_TUTOR_EFFECT),
    )),
);

// VIS 73 — Vampirism
static VAMPIRISM_OTHER_CREATURES: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::Not(&ObjectPredicateDef::AttachedToSource),
]);
static VAMPIRISM_BONUS: ValueDef = ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
    VAMPIRISM_OTHER_CREATURES,
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
));

pub(in crate::card::sets) static VAMPIRISM: CardRecord = CardRecord::new(
    "Vampirism",
    "2dff2817-1813-410f-aca7-96e8f9f4ce81",
    "Gary Leach",
    CardRules::new_enchantment(mana_cost!("{1}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            abilities::enters_trigger(
                "When this Aura enters, draw a card at the beginning of the next turn's upkeep.",
                EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                    "At the beginning of the next turn's upkeep, draw a card.",
                    TriggerEventDef::StepBegins {
                        step: TurnStepDef::Upkeep,
                        player: PlayerRelation::Any,
                    },
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ))),
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+1 for each other creature you control.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        VAMPIRISM_BONUS,
                        VAMPIRISM_BONUS,
                    ),
                },
            ),
            AbilityDef::static_ability(
                "Other creatures you control get -1/-1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        VAMPIRISM_OTHER_CREATURES,
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-1),
                        ValueDef::Constant(-1),
                    ),
                },
            ),
        ]),
);

// VIS 74 — Wake of Vultures
pub(in crate::card::sets) static WAKE_OF_VULTURES: CardRecord = CardRecord::new(
    "Wake of Vultures",
    "52420b80-7f34-4426-ac97-a6e15167c7a9",
    "Jeff Miracola",
    // A flier that eats the rest of the board to stay alive, which is the
    // deal a deck making tokens is happy to take.
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Bird", "Skeleton"], 3, 1).with_abilities(&[
        abilities::flying(),
        abilities::regenerate_self(
            "{1}{B}, Sacrifice a creature: Regenerate this creature.",
            &[
                CostDef::Mana(mana_cost!("{1}{B}")),
                CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    controller: PlayerRelation::You,
                },
            ],
        ),
    ]),
);

// VIS 75 — Wicked Reward
pub(in crate::card::sets) static WICKED_REWARD: CardRecord = CardRecord::new(
    "Wicked Reward",
    "ee32f8ba-3547-4913-a555-d43ee2978ba9",
    "D. Alexander Gregory",
    // A combat trick that trades a spare creature for four power, so the block
    // that looked safe kills the blocker instead.
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, sacrifice a creature.\nTarget creature \
             gets +4/+2 until end of turn.",
            &const {
                [AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )]
            },
            CostDef::sacrifice(
                ObjectPredicateDef::HasType(CardType::Creature),
                CostQuantityDef::Fixed(1),
            ),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(4),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// VIS 76 — Bogardan Phoenix
pub(in crate::card::sets) static BOGARDAN_PHOENIX: CardRecord = CardRecord::new(
    "Bogardan Phoenix",
    "253db28a-3873-4364-80d7-a8164000ea9e",
    "David O'Connor",
CardRules::new_creature(mana_cost!("{2}{R}{R}{R}"), &["Phoenix"], 3, 3).with_abilities(&[
        abilities::flying(),
        abilities::dies_trigger(
            "When this creature dies, exile it if it had a death counter on it. Otherwise, return it to the battlefield under your control and put a death counter on it.",
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("death"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
                then: &EffectDef::move_to_zone(
                    EffectRecipientDef::Source,
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
                otherwise: &EffectDef::WithZoneMoveResult {
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::Source,
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                    binding: crate::ParentBinding,
                    then: &EffectDef::AddCounters {
                        object: EffectRecipientDef::binding_zone_change_successors(
                            crate::ParentBinding,
                        ),
                        kind: CounterKind::named("death"),
                        amount: ValueDef::Constant(1),
                    },
                },
            },
        ),
    ]),
);

// VIS 77 — Dwarven Vigilantes
pub(in crate::card::sets) static DWARVEN_VIGILANTES: CardRecord = CardRecord::new(
    "Dwarven Vigilantes",
    "077d33bb-41bf-440d-939b-67ab5aacb092",
    "Pete Venters",
CardRules::new_creature(mana_cost!("{2}{R}"), &["Dwarf"], 2, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks and isn't blocked, you may have it deal damage equal to its power to target creature. If you do, this creature assigns no combat damage this turn.",
            TriggerEventDef::AttacksAndIsNotBlocked {
                attacker: ObjectPredicateDef::Source,
            },
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Sequence(&[
                    EffectDef::damage(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ValueDef::SourcePower,
                    ),
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::AssignsNoCombatDamage),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            },
        ),
    ),
);

// VIS 78 — Elkin Lair
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELKIN_LAIR: CardRecord = CardRecord::new(
    "Elkin Lair",
    "bcb625ba-3718-4988-962c-bf2e11eb4c16",
    "Jerry Tiritilli",
    crate::card::CardRules::unsupported(),
);

// VIS 79 — Fireblast
pub(in crate::card::sets) static FIREBLAST: CardRecord = CardRecord::new(
    "Fireblast",
    "b1eb5b2c-1f02-48a6-a287-88eb189d6780",
    "Michael Danza",
    CardRules::new_instant(mana_cost!("{4}{R}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Fireblast deals 4 damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(4),
            ),
        ),
        AbilityDef::alternative_cast(
            &[CostDef::sacrifice(
                ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Mountain]),
                CostQuantityDef::Fixed(2),
            )],
            AlternativeCastKindDef::AlternativeCost,
            Some("You may sacrifice two Mountains rather than pay this spell's mana cost."),
            EffectDef::None,
        ), // Two Mountains off the battlefield, which is why the card is a finisher
           // rather than a burn spell: it is cast from an empty board on the turn the
           // lands stop mattering.
    ]),
);

// VIS 80 — Goblin Recruiter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_RECRUITER: CardRecord = CardRecord::new(
    "Goblin Recruiter",
    "6ee791d5-1d48-40e8-b65f-b6aa889f3467",
    "Scott Kirschner",
    crate::card::CardRules::unsupported(),
);

// VIS 81 — Goblin Swine-Rider
pub(in crate::card::sets) static GOBLIN_SWINE_RIDER: CardRecord = CardRecord::new(
    "Goblin Swine-Rider",
    "49980982-d534-4204-bc15-3e6c4ffa1a53",
    "Geofrey Darrow & I. Rabarot",
CardRules::new_creature(mana_cost!("{R}"), &["Goblin"], 1, 1).with_ability(
        AbilityDef::triggered(
            "Whenever this creature becomes blocked, it deals 2 damage to each attacking creature and each blocking creature.",
            TriggerEventDef::BecomesBlocked(ObjectPredicateDef::Source),
            EffectDef::damage(
                EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::AttackingOrBlocking,
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                ValueDef::Constant(2),
            ),
        ),
    ),
);

// VIS 82 — Hearth Charm
pub(in crate::card::sets) static HEARTH_CHARM: CardRecord = CardRecord::new(
    "Hearth Charm",
    "caa9ac66-51b7-4aec-92dc-0f0656b0f7fe",
    "Greg Spalenka",
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::destroy_target(
                "Destroy target artifact creature.",
                &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                ])),
            ),
            AbilityDef::spell(
                "Attacking creatures get +1/+0 until end of turn.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Attacking,
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell_with_targets(
                "Target creature with power 2 or less can't be blocked this turn.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::PowerAtLeast(3)),
                    ]),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    )),
);

// VIS 83 — Heat Wave
// Audit: unsupported — BlockRestrictionDef only supports a fixed mana payment;
// this needs a life payment for each nonblue creature declared as a blocker.
pub(in crate::card::sets) static HEAT_WAVE: CardRecord = CardRecord::new(
    "Heat Wave",
    "42dd0810-4528-4a88-add8-923bb2057821",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// VIS 84 — Hulking Cyclops
pub(in crate::card::sets) static HULKING_CYCLOPS: CardRecord = CardRecord::new(
    "Hulking Cyclops",
    "a3ee5ea8-7023-4dde-ab51-d3ba234d74b9",
    "DiTerlizzi",
    // Five power for five mana, and it can only ever attack: the drawback is
    // what pays for the size.
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Cyclops"], 5, 5).with_ability(
        AbilityDef::static_ability(
            "This creature can't block.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
            },
        ),
    ),
);

// VIS 85 — Keeper of Kookus
pub(in crate::card::sets) static KEEPER_OF_KOOKUS: CardRecord = CardRecord::new(
    "Keeper of Kookus",
    "d11b6df4-449f-44ea-a4fa-f079bcd26a54",
    "Scott Hampton",
    // A one-drop that walks past every red blocker and dodges every red burn
    // spell, for one mana at a time.
    CardRules::new_creature(mana_cost!("{R}"), &["Human", "Nomad"], 1, 1).with_ability(
        AbilityDef::activated(
            "{R}: This creature gains protection from red until end of turn.",
            &[CostDef::Mana(mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(
                    &const { abilities::protection_from_color(ManaColor::Red) },
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// VIS 86 — Kookus
pub(in crate::card::sets) static KOOKUS: CardRecord = CardRecord::new(
    "Kookus",
    "8fb90922-99d2-4b36-9039-bb806fd01756",
    "Scott Hampton",
CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Djinn"], 3, 5).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered_if(
            "At the beginning of your upkeep, if you don't control a creature named Keeper of Kookus, this creature deals 3 damage to you and attacks this turn if able.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::NameEquals(CardNameDef::Literal("Keeper of Kookus")),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::Equal,
                amount: 0,
            },
            EffectDef::Sequence(&[
                EffectDef::damage(EffectRecipientDef::Controller, ValueDef::Constant(3)),
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(
                        &abilities::attacks_each_combat_if_able()
                            .override_text("This creature attacks this turn if able."),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
        AbilityDef::activated(
            "{R}: This creature gets +1/+0 until end of turn.",
            &[CostDef::Mana(mana_cost!("{R}"))],
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

// VIS 87 — Lightning Cloud
pub(in crate::card::sets) static LIGHTNING_CLOUD: CardRecord = CardRecord::new(
    "Lightning Cloud",
    "7fcfc2ad-a1a4-4f65-a239-f11383aaafe1",
    "John Matson",
CardRules::new_enchantment(mana_cost!("{3}{R}")).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever a player casts a red spell, you may pay {R}. If you do, this enchantment deals 1 damage to any target.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::Color(ManaColor::Red)),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::Mana(mana_cost!("{R}"))],
                &EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(1),
                ),
            )),
        ),
    ),
);

// VIS 88 — Mob Mentality
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOB_MENTALITY: CardRecord = CardRecord::new(
    "Mob Mentality",
    "e428d56a-9445-4e86-b281-656e2d251e0b",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// VIS 89 — Ogre Enforcer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OGRE_ENFORCER: CardRecord = CardRecord::new(
    "Ogre Enforcer",
    "b0f072d6-7489-4eb0-8c53-1fa42ad806a4",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// VIS 90 — Raging Gorilla
pub(in crate::card::sets) static RAGING_GORILLA: CardRecord = CardRecord::new(
    "Raging Gorilla",
    "07c284ce-33b8-4fb2-9dd9-4c477bedc774",
    "Tom Kyffin",
    // A 2/3 that becomes a 4/1 the moment combat pairs it off, so it wins
    // the exchange and then dies to anything.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Ape"], 2, 3).with_ability(
        AbilityDef::triggered(
            "Whenever this creature blocks or becomes blocked, it gets +2/-2 until end of turn.",
            TriggerEventDef::BlocksOrBecomesBlockedBy {
                creature: ObjectPredicateDef::Source,
                other: ObjectPredicateDef::Any,
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(-2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// VIS 91 — Relentless Assault
pub(in crate::card::sets) static RELENTLESS_ASSAULT: CardRecord = CardRecord::new(
    "Relentless Assault",
    "747161ea-cb65-4960-84dd-a05bfe5f3ba0",
    "Geofrey Darrow & I. Rabarot",
CardRules::new_sorcery(mana_cost!("{2}{R}{R}")).with_ability(AbilityDef::spell(
        "Untap all creatures that attacked this turn. After this main phase, there is an additional combat phase followed by an additional main phase.",
        EffectDef::Sequence(&[
            EffectDef::Untap {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::AttackedThisTurn,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
            },
            EffectDef::ScheduleTurnPhases(&[
                TurnPhaseDef::Combat,
                TurnPhaseDef::PostcombatMain,
            ]),
        ]),
    )),
);

// VIS 92 — Rock Slide
pub(in crate::card::sets) static ROCK_SLIDE: CardRecord = CardRecord::new(
    "Rock Slide",
    "7e01717a-d6ed-42c1-9a9a-f3f4a3d73bca",
    "Mike Kerr",
CardRules::new_instant(mana_cost!("{X}{R}")).with_ability(AbilityDef::spell_with_targets(
        "This spell deals X damage divided as you choose among any number of target attacking or blocking creatures without flying.",
        &[AbilityTargetDef {
            predicate: AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AttackingOrBlocking,
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                        KeywordAbility::Flying,
                    )),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
            minimum: 0,
            maximum: AbilityTargetDef::UNLIMITED,
            exact_count: None,
            divided_total: Some(DividedTotal::ChosenX),
            another: false,
            excludes_source: false,
            chooser: TargetChooserDef::Controller,
        }],
        EffectDef::damage(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ValueDef::DividedAmongTargets,
        ),
    )),
);

// VIS 93 — Solfatara
pub(in crate::card::sets) static SOLFATARA: CardRecord = CardRecord::new(
    "Solfatara",
    "c5d4bd6f-b019-4594-aa41-138fa58ba529",
    "Omaha Pérez",
CardRules::new_instant(mana_cost!("{2}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Target player can't play lands this turn. Draw a card at the beginning of the next turn's upkeep.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(
                        PlayRestrictionDef::new(
                            PlayActionMatcherDef::PlayLand,
                            ObjectPredicateDef::Any,
                        ),
                    )),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                    "At the beginning of the next turn's upkeep, draw a card.",
                    TriggerEventDef::StepBegins {
                        step: TurnStepDef::Upkeep,
                        player: PlayerRelation::Any,
                    },
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ))),
            ]),
        ),
    ),
);

// VIS 94 — Song of Blood
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SONG_OF_BLOOD: CardRecord = CardRecord::new(
    "Song of Blood",
    "4497a1d7-6604-4f2d-9484-1f1d77a6228f",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// VIS 95 — Spitting Drake
pub(in crate::card::sets) static SPITTING_DRAKE: CardRecord = CardRecord::new(
    "Spitting Drake",
    "c9f6ef97-587f-4f7b-98a2-e3cc8b39df8b",
    "Geofrey Darrow & I. Rabarot",
    // One activation a turn, so it is a 3/2 flier rather than a mana sink.
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Drake"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{R}: This creature gets +1/+0 until end of turn. Activate only once each turn.",
            &[CostDef::Mana(mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .activations_each_turn(1),
    ]),
);

// VIS 96 — Suq'Ata Lancer
pub(in crate::card::sets) static SUQ_ATA_LANCER: CardRecord = CardRecord::new(
    "Suq'Ata Lancer",
    "2884d8df-7fd5-4247-9da5-38c31333ff5d",
    "Jeff Miracola",
    // Haste and flanking together mean the turn it lands is the turn it wins
    // a fight it should have lost.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Knight"], 2, 2)
        .with_abilities(&[abilities::haste(), abilities::flanking()]),
);

// VIS 97 — Talruum Champion
pub(in crate::card::sets) static TALRUUM_CHAMPION: CardRecord = CardRecord::new(
    "Talruum Champion",
    "33730a07-754c-4606-bfac-d73454af9567",
    "Pete Venters",
CardRules::new_creature(mana_cost!("{4}{R}"), &["Minotaur", "Warrior"], 3, 3)
        .with_abilities(&[
            abilities::first_strike(),
            AbilityDef::triggered(
                "Whenever this creature blocks or becomes blocked by a creature, that creature loses first strike until end of turn.",
                TriggerEventDef::BlocksOrBecomesBlockedBy {
                    creature: ObjectPredicateDef::Source,
                    other: ObjectPredicateDef::HasType(CardType::Creature),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::TriggeringObject,
                    effect: AppliedEffectDef::remove_abilities(AbilityPredicateDef::Keyword(
                        KeywordAbility::FirstStrike,
                    )),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// VIS 98 — Talruum Piper
pub(in crate::card::sets) static TALRUUM_PIPER: CardRecord = CardRecord::new(
    "Talruum Piper",
    "ca2cb9a7-5063-4b31-9782-8bfd784bca0a",
    "Pete Venters",
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Minotaur"], 3, 3).with_ability(
        AbilityDef::static_ability(
            "All creatures with flying able to block this creature do so.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MustBeBlockedBy(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                    ]),
                )),
            },
        ),
    ),
);

// VIS 99 — Tremor
pub(in crate::card::sets) static TREMOR: CardRecord = CardRecord::new(
    "Tremor",
    "a9d64665-c1e0-40ab-a358-247f82966379",
    "Michael Danza",
    // One damage that misses fliers, which in the right format is a
    // one-mana answer to a whole deck of X/1s.
    CardRules::new_sorcery(mana_cost!("{R}")).with_ability(AbilityDef::spell(
        "Tremor deals 1 damage to each creature without flying.",
        EffectDef::damage(
            EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                        KeywordAbility::Flying,
                    )),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            ValueDef::Constant(1),
        ),
    )),
);

// VIS 100 — Viashino Sandstalker
pub(in crate::card::sets) static VIASHINO_SANDSTALKER: CardRecord = CardRecord::new(
    "Viashino Sandstalker",
    "01770e13-ebd4-4c83-9e72-99374239a63d",
    "Andrew Robinson",
    // Four damage for three mana with no board left behind, which is a burn
    // spell that can be blocked.
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Lizard", "Warrior"], 4, 2).with_abilities(
        &[
            abilities::haste(),
            AbilityDef::triggered(
                "At the beginning of the end step, return this creature to its owner's hand.",
                // Any end step, not only yours: cast on their turn it comes
                // back the same turn, which is what makes it a trick.
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::Any,
                },
                EffectDef::move_to_zone(
                    EffectRecipientDef::Source,
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            ),
        ],
    ),
);

// VIS 101 — Bull Elephant
pub(in crate::card::sets) static BULL_ELEPHANT: CardRecord = CardRecord::new(
    "Bull Elephant",
    "fa7f5f41-ed30-412b-b51e-37d26e9e6455",
    "Steve White",
CardRules::new_creature(mana_cost!("{3}{G}"), &["Elephant"], 4, 4).with_ability(
        enters_bounce_or_sacrifice(
            "When this creature enters, sacrifice it unless you return two Forests you control to their owner's hand.",
            &[
                CostDef::MovePermanentMatching {
                    object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
                    zone: ZoneKind::Hand,
                },
                CostDef::MovePermanentMatching {
                    object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
                    zone: ZoneKind::Hand,
                },
            ],
        ),
    ),
);

// VIS 102 — City of Solitude
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CITY_OF_SOLITUDE: CardRecord = CardRecord::new(
    "City of Solitude",
    "be499b81-bb2d-4f1d-9deb-c8bfcdca8e13",
    "Romas Kukalis",
    crate::card::CardRules::unsupported(),
);

// VIS 103 — Creeping Mold
pub(in crate::card::sets) static CREEPING_MOLD: CardRecord = CardRecord::new(
    "Creeping Mold",
    "36e7691f-c771-4451-ac54-3532ca10d48f",
    "David Seeley",
    // Four mana to answer any of three permanent types, which is green's
    // whole removal suite in one card.
    CardRules::new_sorcery(mana_cost!("{2}{G}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target artifact, enchantment, or land.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::HasType(CardType::Land),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            then: None,
        },
    )),
);

// VIS 104 — Elephant Grass
static ELEPHANT_GRASS_BLACK_CREATURES: ObjectPredicateDef =
    ObjectPredicateDef::Color(ManaColor::Black);

pub(in crate::card::sets) static ELEPHANT_GRASS: CardRecord = CardRecord::new(
    "Elephant Grass",
    "f4c1f5a7-0d28-43ab-9b66-937e963f42cd",
    "Tony Roberts",
CardRules::new_enchantment(mana_cost!("{G}")).with_abilities(&[
        abilities::cumulative_upkeep(
            &[CostDef::mana(mana_cost!("{1}"))],
        ),
        AbilityDef::static_ability(
            "Black creatures can't attack you.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::AttackRestriction(
                    AttackRestrictionDef::prohibit(
                        ELEPHANT_GRASS_BLACK_CREATURES,
                        AttackDefenderScopeDef::AffectedPlayer,
                    ),
                )),
            },
        ),
        AbilityDef::static_ability(
            "Nonblack creatures can't attack you unless their controller pays {2} for each creature they control that's attacking you.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::AttackRestriction(
                    AttackRestrictionDef::unless_paid(
                        ObjectPredicateDef::Not(&ELEPHANT_GRASS_BLACK_CREATURES),
                        AttackDefenderScopeDef::AffectedPlayer,
                        mana_cost!("{2}"),
                    ),
                )),
            },
        ),
    ]),
);

// VIS 105 — Elven Cache
pub(in crate::card::sets) static ELVEN_CACHE: CardRecord = CardRecord::new(
    "Elven Cache",
    "80fa078f-c74a-42b2-af97-7ca2c29dc316",
    "John Matson",
    // Four mana to buy back anything, which is the rate green pays for
    // recursion that does not care what it is recurring.
    CardRules::new_sorcery(mana_cost!("{2}{G}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Return target card from your graveyard to your hand.",
        // "Target card", with no type restriction, so it buys back a
        // land or an artifact just as readily as a creature.
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Any,
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            },
        )],
        EffectDef::move_to_zone(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ZoneKind::Hand,
            ZonePlacement::Top,
        ),
    )),
);

// VIS 106 — Emerald Charm
pub(in crate::card::sets) static EMERALD_CHARM: CardRecord = CardRecord::new(
    "Emerald Charm",
    "e9c9199b-61b3-4794-878b-f065058f50f3",
    "Greg Spalenka",
    CardRules::new_instant(mana_cost!("{G}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Untap target permanent.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::Any,
                )],
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::destroy_target(
                "Destroy target non-Aura enchantment.",
                &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                        "Aura",
                    ))),
                ])),
            ),
            AbilityDef::spell_with_targets(
                "Target creature loses flying until end of turn.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::remove_abilities(AbilityPredicateDef::Keyword(
                        KeywordAbility::Flying,
                    )),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    )),
);

// VIS 107 — Feral Instinct
pub(in crate::card::sets) static FERAL_INSTINCT: CardRecord = CardRecord::new(
    "Feral Instinct",
    "20dec7cf-2865-4642-9022-d3006fd7ac30",
    "Una Fricker",
CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(
        AbilityDef::spell_with_targets(
            "Target creature gets +1/+1 until end of turn. Draw a card at the beginning of the next turn's upkeep.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                    "At the beginning of the next turn's upkeep, draw a card.",
                    TriggerEventDef::StepBegins {
                        step: TurnStepDef::Upkeep,
                        player: PlayerRelation::Any,
                    },
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ))),
            ]),
        ),
    ),
);

// VIS 108 — Giant Caterpillar
pub(in crate::card::sets) static GIANT_CATERPILLAR: CardRecord = CardRecord::new(
    "Giant Caterpillar",
    "b7f602a6-3d35-49a3-b5cb-d754e03a9573",
    "Zina Saunders",
CardRules::new_creature(mana_cost!("{3}{G}"), &["Insect"], 3, 3).with_ability(
        AbilityDef::activated(
            "{G}, Sacrifice this creature: Create a 1/1 green Insect creature token with flying named Butterfly at the beginning of the next end step.",
            &[
                CostDef::Mana(mana_cost!("{G}")),
                CostDef::SacrificeSource,
            ],
            EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                "At the beginning of the next end step, create a 1/1 green Insect creature token with flying named Butterfly.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::Any,
                },
                EffectDef::create_creature_token(&["Insect"], &[ManaColor::Green], 1, 1)
                    .with_name("Butterfly")
                    .with_abilities(&[abilities::flying()]),
            ))),
        ),
    ),
);

// VIS 109 — Katabatic Winds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KATABATIC_WINDS: CardRecord = CardRecord::new(
    "Katabatic Winds",
    "97b34ce8-1eb2-44eb-813a-09d0308e27a0",
    "Gary Gianni",
    crate::card::CardRules::unsupported(),
);

// VIS 110 — King Cheetah
pub(in crate::card::sets) static KING_CHEETAH: CardRecord = CardRecord::new(
    "King Cheetah",
    "38149d49-8661-427c-9338-93c11a2a8093",
    "Terese Nielsen",
    // A 3/2 that arrives at the end of a turn, which is worth a mana over
    // a bear that has to be cast on the caster's own.
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Cat"], 3, 2).with_ability(abilities::flash()),
);

// VIS 111 — Kyscu Drake
pub(in crate::card::sets) static KYSCU_DRAKE: CardRecord = CardRecord::new(
    "Kyscu Drake",
    "b6f14bbe-2436-4a5a-8e2a-8066b740b715",
    "Geofrey Darrow & I. Rabarot",
CardRules::new_creature(mana_cost!("{3}{G}"), &["Drake"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{G}: This creature gets +0/+1 until end of turn. Activate only once each turn.",
            &[CostDef::Mana(mana_cost!("{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .once_each_turn(),
        AbilityDef::activated(
            "Sacrifice this creature and a creature named Spitting Drake: Search your library for a card named Viashivan Dragon, put that card onto the battlefield, then shuffle.",
            &[
                CostDef::SacrificeSource,
                CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::NameEquals(CardNameDef::Literal("Spitting Drake")),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::NameEquals(CardNameDef::Literal(
                    "Viashivan Dragon",
                )),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: false,
                destination: ZoneKind::Battlefield,
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

// VIS 112 — Lichenthrope
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LICHENTHROPE: CardRecord = CardRecord::new(
    "Lichenthrope",
    "76f0c356-a81d-41d4-a8b7-8c159146a8b8",
    "Bob Eggleton",
    crate::card::CardRules::unsupported(),
);

// VIS 113 — Mortal Wound
pub(in crate::card::sets) static MORTAL_WOUND: CardRecord = CardRecord::new(
    "Mortal Wound",
    "808830ff-496a-41dc-8b64-334ddaca9435",
    "Kev Walker",
    // One mana that turns any ping into removal, which is why a deck with
    // one damage to spare plays it over a real answer.
    CardRules::new_enchantment(mana_cost!("{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::triggered(
                "When enchanted creature is dealt damage, destroy it.",
                TriggerEventDef::DamageDealt(DamageEventMatcherDef {
                    kind: DamageKindDef::Any,
                    source: DamageSourceMatcherDef::Any,
                    recipient: DamageRecipientMatcherDef::Recipients(
                        EffectRecipientDef::AttachedPermanent,
                    ),
                }),
                EffectDef::Destroy {
                    object: EffectRecipientDef::AttachedPermanent,
                    then: None,
                },
            ),
        ]),
);

// VIS 114 — Natural Order
pub(in crate::card::sets) static NATURAL_ORDER: CardRecord = CardRecord::new(
    "Natural Order",
    "0845f0b0-9413-4ddd-861d-9607636bebc6",
    "Terese Nielsen",
    // Four mana and a Llanowar Elves for whatever the deck is built around.
    // The search is mandatory and the sacrifice is a cost, so the card is a
    // dead draw exactly when the board is empty.
    CardRules::new_sorcery(mana_cost!("{2}{G}{G}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, sacrifice a green creature.\nSearch your \
             library for a green creature card, put it onto the battlefield, then shuffle.",
            &[],
            // Paid as the spell is cast, so a board with nothing green on it cannot
            // cast this at all.
            CostDef::sacrifice(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::Green),
                ]),
                CostQuantityDef::Fixed(1),
            ),
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::Green),
                ]),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: false,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: false,
                attachment: None,
                binding: None,
                then: None,
            },
        ),
    ),
);

// VIS 115 — Panther Warriors
pub(in crate::card::sets) static PANTHER_WARRIORS: CardRecord = CardRecord::new(
    "Panther Warriors",
    "76c9bc99-28e3-4d64-8383-2b92011104ed",
    "Cecil Fernando",
    // Five mana for six power and no defence at all: it wins every race and
    // loses every fight it does not start.
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Cat", "Warrior"], 6, 3),
);

// VIS 116 — Quirion Druid
pub(in crate::card::sets) static QUIRION_DRUID: CardRecord = CardRecord::new(
    "Quirion Druid",
    "8ca5319a-5c26-487f-ba87-d317633122ba",
    "John Matson",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf", "Druid"], 1, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{G}, {T}: Target land becomes a 2/2 green creature that's still a land.",
            &[CostDef::Mana(mana_cost!("{G}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Land),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                    AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Green])),
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                ]),
                duration: ResolvedEffectDurationDef::Permanent,
            },
        ),
    ),
);

// VIS 117 — Quirion Ranger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QUIRION_RANGER: CardRecord = CardRecord::new(
    "Quirion Ranger",
    "56efe72c-6d7f-44f6-ac74-01af9305c4b6",
    "Tom Kyffin",
    crate::card::CardRules::unsupported(),
);

// VIS 118 — River Boa
pub(in crate::card::sets) static RIVER_BOA: CardRecord = CardRecord::new(
    "River Boa",
    "2e9d5aaf-b7e8-4676-aec8-7d29a0169a2c",
    "Steve White",
    // Unblockable against blue and hard to kill in combat, which is why a
    // two-drop bear has been a sideboard card for thirty years.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Snake"], 2, 1).with_abilities(&[
        abilities::landwalk(BasicLandType::Island),
        abilities::regenerate_self(
            "{G}: Regenerate this creature.",
            &[CostDef::Mana(mana_cost!("{G}"))],
        ),
    ]),
);

// VIS 119 — Rowen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROWEN: CardRecord = CardRecord::new(
    "Rowen",
    "07144d84-f7f3-4101-805d-07cce8342a64",
    "Jon J Muth",
    crate::card::CardRules::unsupported(),
);

// VIS 120 — Spider Climb
pub(in crate::card::sets) static SPIDER_CLIMB: CardRecord = CardRecord::new(
    "Spider Climb",
    "a1818812-4cb8-4fe1-98c0-b40086b4991c",
    "Ron Spencer",
    CardRules::new_enchantment(mana_cost!("{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::flash(),
            INSTANT_SPEED_ENCHANTMENT_CLEANUP,
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +0/+3 and has reach.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(0),
                            ValueDef::Constant(3),
                        ),
                        AppliedEffectDef::add_ability(&abilities::reach()),
                    ]),
                },
            ),
        ]),
);

// VIS 121 — Stampeding Wildebeests
pub(in crate::card::sets) static STAMPEDING_WILDEBEESTS: CardRecord = CardRecord::new(
    "Stampeding Wildebeests",
    "ddb5f524-fad6-4a63-b20f-3348a844fefa",
    "Randy Gallegos",
CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Antelope", "Beast"], 5, 4)
        .with_abilities(&[
            abilities::trample(),
            AbilityDef::triggered(
                "At the beginning of your upkeep, return a green creature you control to its owner's hand.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Object(crate::ParentBinding),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Color(ManaColor::Green),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::move_to_zone(
                        EffectRecipientDef::object(ObjectRefDef::Binding(
                            crate::ParentBinding,
                        )),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                }),
            ),
        ]),
);

// VIS 122 — Summer Bloom
// Audit: unsupported — Needs the additional-land permission as a resolving effect. AppliedRuleDef::MayPlayAdditionalLands exists but only as a StaticApply from a permanent; applying it for a turn is rejected as UnsupportedResolvingAppliedEffect.
pub(in crate::card::sets) static SUMMER_BLOOM: CardRecord = CardRecord::new(
    "Summer Bloom",
    "35d78f4e-d95d-49bc-9971-06a68a4e35fd",
    "Nicola Leonard",
    crate::card::CardRules::unsupported(),
);

// VIS 123 — Uktabi Orangutan
pub(in crate::card::sets) static UKTABI_ORANGUTAN: CardRecord = CardRecord::new(
    "Uktabi Orangutan",
    "101c7d58-43cc-4ebd-87f1-2016fbff56dd",
    "Una Fricker",
    // Three mana for a body and an artifact, which made it a maindeck card
    // in a format full of them.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Ape"], 2, 2).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, destroy target artifact.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Artifact),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ),
);

// VIS 124 — Warthog
pub(in crate::card::sets) static WARTHOG: CardRecord = CardRecord::new(
    "Warthog",
    "dd2510b8-52d6-4d2e-89a5-31b27b732dd8",
    "Steve White",
    // A green creature with swampwalk, which is Visions asking green to
    // attack the colour it is worst against.
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Boar"], 3, 2)
        .with_ability(abilities::landwalk(BasicLandType::Swamp)),
);

// VIS 125 — Wind Shear
pub(in crate::card::sets) static WIND_SHEAR: CardRecord = CardRecord::new(
    "Wind Shear",
    "b8324f44-c7f5-41ee-bc8d-16822bd8942f",
    "John Matson",
    CardRules::new_instant(mana_cost!("{2}{G}")).with_ability(AbilityDef::spell(
        "Attacking creatures with flying get -2/-2 and lose flying until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Attacking,
                    ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-2),
                    ValueDef::Constant(-2),
                ),
                AppliedEffectDef::remove_abilities(AbilityPredicateDef::Keyword(
                    KeywordAbility::Flying,
                )),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// VIS 126 — Army Ants
pub(in crate::card::sets) static ARMY_ANTS: CardRecord = CardRecord::new(
    "Army Ants",
    "7e129be5-e2c5-4f69-b8e8-539ac2085c7a",
    "Geofrey Darrow & I. Rabarot",
    CardRules::new_creature(mana_cost!("{1}{B}{R}"), &["Insect"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice a land: Destroy target land.",
            &[
                CostDef::TapSource,
                CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    controller: PlayerRelation::You,
                },
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Land),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ),
);

// VIS 127 — Breathstealer's Crypt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BREATHSTEALER_S_CRYPT: CardRecord = CardRecord::new(
    "Breathstealer's Crypt",
    "f87ace53-d77c-4df5-b200-4be2ac2b7fdb",
    "Blackie del Rio",
    crate::card::CardRules::unsupported(),
);

// VIS 128 — Corrosion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CORROSION: CardRecord = CardRecord::new(
    "Corrosion",
    "176122b2-f60f-4150-8c0c-757c8f8914d2",
    "Michael Danza",
    crate::card::CardRules::unsupported(),
);

// VIS 129 — Femeref Enchantress
pub(in crate::card::sets) static FEMEREF_ENCHANTRESS: CardRecord = CardRecord::new(
    "Femeref Enchantress",
    "20ba72c7-7957-4d02-b41e-c0132fe1f2e6",
    "D. Alexander Gregory",
    CardRules::new_creature(mana_cost!("{G}{W}"), &["Human", "Druid"], 1, 2).with_ability(
        AbilityDef::triggered(
            "Whenever an enchantment is put into a graveyard from the battlefield, draw a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::HasType(CardType::Enchantment),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// VIS 130 — Firestorm Hellkite
pub(in crate::card::sets) static FIRESTORM_HELLKITE: CardRecord = CardRecord::new(
    "Firestorm Hellkite",
    "def23574-4a41-4323-84d9-49f58b2ca322",
    "Pete Venters",
    CardRules::new_creature(mana_cost!("{4}{U}{R}"), &["Dragon"], 6, 6).with_abilities(&[
        abilities::flying(),
        abilities::trample(),
        abilities::cumulative_upkeep(&[CostDef::mana(mana_cost!("{U}{R}"))])
            .override_text("Cumulative upkeep {U}{R}"),
    ]),
);

// VIS 131 — Guiding Spirit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GUIDING_SPIRIT: CardRecord = CardRecord::new(
    "Guiding Spirit",
    "5f96d184-0ef8-40f7-98bc-bd4c53c57072",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// VIS 132 — Mundungu
pub(in crate::card::sets) static MUNDUNGU: CardRecord = CardRecord::new(
    "Mundungu",
    "d6e320ca-848b-4743-93f1-ec04ef1ce402",
    "Terese Nielsen",
    CardRules::new_creature(mana_cost!("{1}{U}{B}"), &["Human", "Wizard"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Counter target spell unless its controller pays {1} and 1 life.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Spell,
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            abilities::counter_target_unless_paid(&[
                CostDef::GenericMana(ValueDef::Constant(1)),
                CostDef::PayLife(1),
            ]),
        ),
    ),
);

// VIS 133 — Pygmy Hippo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PYGMY_HIPPO: CardRecord = CardRecord::new(
    "Pygmy Hippo",
    "2e3f6220-6ead-46b4-8663-57609ef5a12e",
    "Steve White",
    crate::card::CardRules::unsupported(),
);

// VIS 134 — Righteous War
pub(in crate::card::sets) static RIGHTEOUS_WAR: CardRecord = CardRecord::new(
    "Righteous War",
    "bbcacb8e-1aff-4807-b70c-a17d6703d279",
    "Ian Miller",
    CardRules::new_enchantment(mana_cost!("{1}{W}{B}")).with_abilities(&[
        AbilityDef::static_ability(
            "White creatures you control have protection from black.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Color(ManaColor::White),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::protection_from_color(
                    ManaColor::Black,
                )),
            },
        ),
        AbilityDef::static_ability(
            "Black creatures you control have protection from white.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Color(ManaColor::Black),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::protection_from_color(
                    ManaColor::White,
                )),
            },
        ),
    ]),
);

// VIS 135 — Scalebane's Elite
pub(in crate::card::sets) static SCALEBANE_S_ELITE: CardRecord = CardRecord::new(
    "Scalebane's Elite",
    "b3bff610-783a-46b7-bd15-061da41027bb",
    "Steve Luke",
    // A 4/4 that black cannot block or remove, printed for a format where
    // black did both.
    CardRules::new_creature(mana_cost!("{3}{G}{W}"), &["Human", "Soldier"], 4, 4)
        .with_ability(abilities::protection_from_color(ManaColor::Black)),
);

// VIS 136 — Simoon
pub(in crate::card::sets) static SIMOON: CardRecord = CardRecord::new(
    "Simoon",
    "642d9239-82e0-4696-ad99-10796042d1f8",
    "Randy Gallegos",
    // The same sweep pointed at one player, which is what makes it worth
    // two colours instead of one.
    CardRules::new_instant(mana_cost!("{R}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Simoon deals 1 damage to each creature target opponent controls.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Opponent),
        )],
        EffectDef::damage(
            EffectRecipientDef::objects_controlled_by_target(
                ObjectPredicateDef::HasType(CardType::Creature),
                TargetIndex::PRIMARY,
            ),
            ValueDef::Constant(1),
        ),
    )),
);

// VIS 137 — Squandered Resources
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SQUANDERED_RESOURCES: CardRecord = CardRecord::new(
    "Squandered Resources",
    "fcddbea7-3025-47b1-a597-2d2b2711fb81",
    "Romas Kukalis",
    crate::card::CardRules::unsupported(),
);

// VIS 138 — Suleiman's Legacy
pub(in crate::card::sets) static SULEIMAN_S_LEGACY: CardRecord = CardRecord::new(
    "Suleiman's Legacy",
    "3a15e970-e605-425a-b4ec-391d9cacde38",
    "Kaja Foglio",
CardRules::new_enchantment(mana_cost!("{R}{W}")).with_abilities(&[
        abilities::enters_trigger(
            "When this enchantment enters, destroy all Djinns and Efreets. They can't be regenerated.",
            EffectDef::WithRule {
                rule: AppliedRuleDef::CannotRegenerate,
                effect: &EffectDef::Destroy {
                    object: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Djinn")),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Efreet")),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    then: None,
                },
            },
        ),
        AbilityDef::triggered(
            "Whenever a Djinn or Efreet enters, destroy it. It can't be regenerated.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Djinn")),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Efreet")),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::WithRule {
                rule: AppliedRuleDef::CannotRegenerate,
                effect: &EffectDef::Destroy {
                    object: EffectRecipientDef::TriggeringZoneChangeResult,
                    then: None,
                },
            },
        ),
    ]),
);

// VIS 139 — Tempest Drake
pub(in crate::card::sets) static TEMPEST_DRAKE: CardRecord = CardRecord::new(
    "Tempest Drake",
    "54aa5262-d0d9-4b4a-8027-00393568b3df",
    "Gerry Grace",
    // Flying and vigilance on a 2/2: it attacks and still holds the air,
    // which is more than either colour bought alone.
    CardRules::new_creature(mana_cost!("{1}{W}{U}"), &["Drake"], 2, 2)
        .with_abilities(&[abilities::flying(), abilities::vigilance()]),
);

// VIS 140 — Viashivan Dragon
pub(in crate::card::sets) static VIASHIVAN_DRAGON: CardRecord = CardRecord::new(
    "Viashivan Dragon",
    "7172ef0b-ca9e-47cf-8ec6-2d8cb18f2283",
    "Ian Miller",
    CardRules::new_creature(mana_cost!("{2}{R}{R}{G}{G}"), &["Dragon"], 4, 4).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{R}: This creature gets +1/+0 until end of turn.",
            &[CostDef::Mana(mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated(
            "{G}: This creature gets +0/+1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// VIS 141 — Anvil of Bogardan
pub(in crate::card::sets) static ANVIL_OF_BOGARDAN: CardRecord = CardRecord::new(
    "Anvil of Bogardan",
    "7ff965dd-54b4-4f21-a52f-81c0dd1e691e",
    "Roger Raupp",
CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::static_ability(
            "Players have no maximum hand size.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::EachPlayer,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::PlayerRule(
                    crate::card::PlayerRuleDef::NoMaximumHandSize,
                )),
            },
        ),
        AbilityDef::triggered(
            "At the beginning of each player's draw step, that player draws an additional card, then discards a card.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Draw,
                player: PlayerRelation::Any,
            },
            EffectDef::Sequence(&[
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::EventPlayer,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::Discard {
                    recipient: EffectRecipientDef::EventPlayer,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            ]),
        ),
    ]),
);

// VIS 142 — Brass-Talon Chimera
pub(in crate::card::sets) static BRASS_TALON_CHIMERA: CardRecord = CardRecord::new(
    "Brass-Talon Chimera",
    "200c9655-e51c-4b63-96cf-7f3fba3ec75c",
    "Mike Dringenberg",
CardRules::new_artifact_creature(mana_cost!("{4}"), &["Chimera"], 2, 2).with_abilities(&[
        abilities::first_strike(),
        AbilityDef::activated_with_targets(
            "Sacrifice this creature: Put a +2/+2 counter on target Chimera creature. It gains first strike.",
            &[CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Chimera")),
                ]),
            )],
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(2),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
            ]),
        ),
    ]),
);

// VIS 143 — Diamond Kaleidoscope
pub(in crate::card::sets) static DIAMOND_KALEIDOSCOPE: CardRecord = CardRecord::new(
    "Diamond Kaleidoscope",
    "548ff852-274d-4068-818d-58a883e74a5f",
    "Ron Spencer",
    CardRules::new_artifact(mana_cost!("{4}")).with_abilities(&[
        AbilityDef::activated(
            "{3}, {T}: Create a 0/1 colorless Prism artifact creature token.",
            &[CostDef::Mana(mana_cost!("{3}")), CostDef::TapSource],
            EffectDef::create_artifact_creature_token(&["Prism"], &[], 0, 1),
        ),
        AbilityDef::activated_mana(
            "Sacrifice a Prism token: Add one mana of any color.",
            &[CostDef::SacrificePermanent {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Token,
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Prism")),
                ]),
                controller: PlayerRelation::You,
            }],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// VIS 144 — Dragon Mask
pub(in crate::card::sets) static DRAGON_MASK: CardRecord = CardRecord::new(
    "Dragon Mask",
    "f098e329-adc8-42dd-b779-d00d9ccc3dbd",
    "Craig Hooper",
CardRules::new_artifact(mana_cost!("{3}")).with_ability(
        AbilityDef::activated_with_targets(
            "{3}, {T}: Target creature you control gets +2/+2 until end of turn. Return it to its owner's hand at the beginning of the next end step.",
            &[
                CostDef::Mana(mana_cost!("{3}")),
                CostDef::TapSource,
            ],
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
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                    "At the beginning of the next end step, return that creature to its owner's hand.",
                    TriggerEventDef::StepBegins {
                        step: TurnStepDef::End,
                        player: PlayerRelation::Any,
                    },
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                ))),
            ]),
        ),
    ),
);

// VIS 145 — Helm of Awakening
pub(in crate::card::sets) static HELM_OF_AWAKENING: CardRecord = CardRecord::new(
    "Helm of Awakening",
    "41bba882-39b8-42db-9a01-54c6712b8019",
    "Adam Rex",
    // It helps whoever casts the most spells, and that is not always the
    // player who paid two mana for it.
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::static_ability(
        "Spells cost {1} less to cast.",
        EffectDef::ModifyCost(CostModificationDef::reduce_spell(
            ObjectPredicateDef::Any,
            PlayerRelation::Any,
            ValueDef::Constant(1),
        )),
    )),
);

// VIS 146 — Iron-Heart Chimera
pub(in crate::card::sets) static IRON_HEART_CHIMERA: CardRecord = CardRecord::new(
    "Iron-Heart Chimera",
    "5899a575-a97d-4850-b55c-22ad6900ba20",
    "Mike Dringenberg",
CardRules::new_artifact_creature(mana_cost!("{4}"), &["Chimera"], 2, 2).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::activated_with_targets(
            "Sacrifice this creature: Put a +2/+2 counter on target Chimera creature. It gains vigilance.",
            &[CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Chimera")),
                ]),
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
                    duration: ResolvedEffectDurationDef::Permanent,
                },
            ]),
        ),
    ]),
);

// VIS 147 — Juju Bubble
pub(in crate::card::sets) static JUJU_BUBBLE: CardRecord = CardRecord::new(
    "Juju Bubble",
    "a5fa8208-7d65-4f8f-b07e-f5c3a66e1143",
    "Donato Giancola",
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        abilities::cumulative_upkeep(&[CostDef::mana(mana_cost!("{1}"))]),
        AbilityDef::triggered(
            "When you play a card, sacrifice this artifact.",
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(PlayerRelation::You)),
                TriggerEventDef::LandPlayed {
                    land: ObjectPredicateDef::Any,
                    player: PlayerRelation::You,
                },
            ]),
            EffectDef::sacrifice(EffectRecipientDef::Source),
        ),
        AbilityDef::activated(
            "{2}: You gain 1 life.",
            &[CostDef::Mana(mana_cost!("{2}"))],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// VIS 148 — Lead-Belly Chimera
pub(in crate::card::sets) static LEAD_BELLY_CHIMERA: CardRecord = CardRecord::new(
    "Lead-Belly Chimera",
    "5d89b377-80d2-42a0-b84e-a455a72ed9fe",
    "Mike Dringenberg",
CardRules::new_artifact_creature(mana_cost!("{4}"), &["Chimera"], 2, 2).with_abilities(&[
        abilities::trample(),
        AbilityDef::activated_with_targets(
            "Sacrifice this creature: Put a +2/+2 counter on target Chimera creature. It gains trample.",
            &[CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Chimera")),
                ]),
            )],
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(2),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::trample()),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
            ]),
        ),
    ]),
);

// VIS 149 — Magma Mine
pub(in crate::card::sets) static MAGMA_MINE: CardRecord = CardRecord::new(
    "Magma Mine",
    "1aecc3df-7ce6-419c-b3d6-60fc28bfe941",
    "Ron Spencer",
CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::activated(
            "{4}: Put a pressure counter on this artifact.",
            &[CostDef::Mana(mana_cost!("{4}"))],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("pressure"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice this artifact: It deals damage equal to the number of pressure counters on it to any target.",
            &[CostDef::TapSource, CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::CountersOnSource(CounterKind::named("pressure")),
            ),
        ),
    ]),
);

// VIS 150 — Matopi Golem
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MATOPI_GOLEM: CardRecord = CardRecord::new(
    "Matopi Golem",
    "92378d6f-89ee-49dc-8964-0e9c55daeffc",
    "Tom Kyffin",
    crate::card::CardRules::unsupported(),
);

// VIS 151 — Phyrexian Marauder
// Audit: unsupported — AttackRestrictionDef only supports a fixed mana payment;
// this needs a payment derived from the source's +1/+1 counter count.
pub(in crate::card::sets) static PHYREXIAN_MARAUDER: CardRecord = CardRecord::new(
    "Phyrexian Marauder",
    "29a75dc8-1c24-4063-8944-d7e71b4a5755",
    "David Seeley",
    crate::card::CardRules::unsupported(),
);

// VIS 152 — Phyrexian Walker
pub(in crate::card::sets) static PHYREXIAN_WALKER: CardRecord = CardRecord::new(
    "Phyrexian Walker",
    "9f8a3979-2947-4692-8b2f-d4c07c534777",
    "Bryan Talbot",
    // A free 0/3. It blocks, it is an artifact, and it is a body -- which is
    // three things that some deck somewhere wants for nothing.
    CardRules::new_artifact_creature(mana_cost!("{0}"), &["Phyrexian", "Construct"], 0, 3),
);

// VIS 153 — Sands of Time
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SANDS_OF_TIME: CardRecord = CardRecord::new(
    "Sands of Time",
    "a782ee95-bde4-41f4-a947-b073cc4c1e7c",
    "Paul Lee",
    crate::card::CardRules::unsupported(),
);

// VIS 154 — Sisay's Ring
pub(in crate::card::sets) static SISAY_S_RING: CardRecord = CardRecord::new(
    "Sisay's Ring",
    "a08becd3-ca5e-4150-8d28-52436a3eaffd",
    "Donato Giancola",
    // Four mana for two, which only pays off in a deck whose top end is
    // expensive enough to want the turn back.
    CardRules::new_artifact(mana_cost!("{4}")).with_ability(abilities::tap_for_mana(
        "{T}: Add {C}{C}.",
        AddManaEffectDef::one(ManaColor::Colorless).with_amount(2),
    )),
);

// VIS 155 — Snake Basket
pub(in crate::card::sets) static SNAKE_BASKET: CardRecord = CardRecord::new(
    "Snake Basket",
    "bfda9a16-9cdb-494a-b662-ac24e3b89d0c",
    "Roger Raupp",
CardRules::new_artifact(mana_cost!("{4}")).with_ability(
        AbilityDef::activated(
            "{X}, Sacrifice this artifact: Create X 1/1 green Snake creature tokens. Activate only as a sorcery.",
            &[
                CostDef::Mana(mana_cost!("{X}")),
                CostDef::SacrificeSource,
            ],
            EffectDef::create_creature_token(&["Snake"], &[ManaColor::Green], 1, 1)
                .with_count(ValueDef::ChosenX),
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ),
);

// VIS 156 — Teferi's Puzzle Box
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEFERI_S_PUZZLE_BOX: CardRecord = CardRecord::new(
    "Teferi's Puzzle Box",
    "1377dab4-b814-46cc-a097-24a3cf8d0f8f",
    "Kaja Foglio",
    crate::card::CardRules::unsupported(),
);

// VIS 157 — Tin-Wing Chimera
pub(in crate::card::sets) static TIN_WING_CHIMERA: CardRecord = CardRecord::new(
    "Tin-Wing Chimera",
    "3375dcc6-9399-48eb-9aa4-7b40c3686cc5",
    "Mike Dringenberg",
CardRules::new_artifact_creature(mana_cost!("{4}"), &["Chimera"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_with_targets(
            "Sacrifice this creature: Put a +2/+2 counter on target Chimera creature. It gains flying.",
            &[CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Chimera")),
                ]),
            )],
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(2),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::flying()),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
            ]),
        ),
    ]),
);

// VIS 158 — Triangle of War
pub(in crate::card::sets) static TRIANGLE_OF_WAR: CardRecord = CardRecord::new(
    "Triangle of War",
    "4c1d7d4d-bed7-4d28-a304-ad33f42e9831",
    "Ian Miller",
// Removal that costs a creature's toughness rather than a card, which
    // is the cheapest way a green deck ever got to kill something.
    CardRules::new_artifact(mana_cost!("{1}")).with_ability(AbilityDef::activated_with_targets(
        "{2}, Sacrifice this artifact: Target creature you control fights target creature an opponent controls.",
        &[CostDef::Mana(mana_cost!("{2}")), CostDef::SacrificeSource],
        &[
            AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
            ])),
        ],
        EffectDef::Fight {
            first: ObjectRefDef::Target(TargetIndex::PRIMARY),
            second: ObjectRefDef::Target(TargetIndex(1)),
            excess: None,
        },
    )),
);

// VIS 159 — Wand of Denial
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAND_OF_DENIAL: CardRecord = CardRecord::new(
    "Wand of Denial",
    "0b1c856f-6d29-4bfc-976e-7875d60abd52",
    "Steve Luke",
    crate::card::CardRules::unsupported(),
);

// VIS 160 — Coral Atoll
pub(in crate::card::sets) static CORAL_ATOLL: CardRecord = CardRecord::new(
    "Coral Atoll",
    "5d7c4619-e5af-4aa0-bd3f-6bf0e1fdc1fc",
    "John Avon",
// Two mana out of one land, paid for with the land drop you already made
    // -- which is a turn behind and a mana ahead.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        enters_bounce_or_sacrifice(
            "When this land enters, sacrifice it unless you return an untapped Island you control to its owner's hand.",
            &[CostDef::MovePermanentMatching {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                zone: ZoneKind::Hand,
            }],
        ),
        abilities::tap_for_mana(
            "{T}: Add {C}{U}.",
            AddManaEffectDef::one_of_each(
                ManaColor::Colorless,
                ManaColor::Blue,
            ),
        ),
    ]),
);

// VIS 161 — Dormant Volcano
pub(in crate::card::sets) static DORMANT_VOLCANO: CardRecord = CardRecord::new(
    "Dormant Volcano",
    "6aa92be7-883f-42bd-8623-00eb2df28a98",
    "John Avon",
// The red member of the cycle, and the one a burn deck could least
    // afford the tempo for.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        enters_bounce_or_sacrifice(
            "When this land enters, sacrifice it unless you return an untapped Mountain you control to its owner's hand.",
            &[CostDef::MovePermanentMatching {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Mountain]),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                zone: ZoneKind::Hand,
            }],
        ),
        abilities::tap_for_mana(
            "{T}: Add {C}{R}.",
            AddManaEffectDef::one_of_each(
                ManaColor::Colorless,
                ManaColor::Red,
            ),
        ),
    ]),
);

// VIS 162 — Everglades
pub(in crate::card::sets) static EVERGLADES: CardRecord = CardRecord::new(
    "Everglades",
    "c1f2eaf7-7f08-446b-892f-5a844f74808f",
    "Bob Eggleton",
// Black's copy of the same bargain: a turn of tempo for a permanent
    // extra mana.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        enters_bounce_or_sacrifice(
            "When this land enters, sacrifice it unless you return an untapped Swamp you control to its owner's hand.",
            &[CostDef::MovePermanentMatching {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Swamp]),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                zone: ZoneKind::Hand,
            }],
        ),
        abilities::tap_for_mana(
            "{T}: Add {C}{B}.",
            AddManaEffectDef::one_of_each(
                ManaColor::Colorless,
                ManaColor::Black,
            ),
        ),
    ]),
);

// VIS 163 — Griffin Canyon
pub(in crate::card::sets) static GRIFFIN_CANYON: CardRecord = CardRecord::new(
    "Griffin Canyon",
    "705d8194-3ad0-41b7-ae32-9c0cd8cd46b9",
    "Stuart Griffin",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{T}: Untap target Griffin. If it's a creature, it gets +1/+1 until end of turn.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Griffin")),
            )],
            EffectDef::Sequence(&[
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::TargetMatches {
                        slot: TargetIndex::PRIMARY,
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                    },
                    then: &EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                },
            ]),
        ),
    ]),
);

// VIS 164 — Jungle Basin
pub(in crate::card::sets) static JUNGLE_BASIN: CardRecord = CardRecord::new(
    "Jungle Basin",
    "cc3146db-2f86-4728-9af1-ff651f871652",
    "John Avon",
// Green's copy, in the colour most likely to have a spare Forest to
    // hand back.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        enters_bounce_or_sacrifice(
            "When this land enters, sacrifice it unless you return an untapped Forest you control to its owner's hand.",
            &[CostDef::MovePermanentMatching {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                zone: ZoneKind::Hand,
            }],
        ),
        abilities::tap_for_mana(
            "{T}: Add {C}{G}.",
            AddManaEffectDef::one_of_each(
                ManaColor::Colorless,
                ManaColor::Green,
            ),
        ),
    ]),
);

// VIS 165 — Karoo
pub(in crate::card::sets) static KAROO: CardRecord = CardRecord::new(
    "Karoo",
    "d786815c-53ec-483e-ad56-382778a57b1a",
    "Zina Saunders",
// The cycle's namesake, and the reason a two-mana land was worth an
    // entire turn of setup.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        enters_bounce_or_sacrifice(
            "When this land enters, sacrifice it unless you return an untapped Plains you control to its owner's hand.",
            &[CostDef::MovePermanentMatching {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Plains]),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                zone: ZoneKind::Hand,
            }],
        ),
        abilities::tap_for_mana(
            "{T}: Add {C}{W}.",
            AddManaEffectDef::one_of_each(
                ManaColor::Colorless,
                ManaColor::White,
            ),
        ),
    ]),
);

// VIS 166 — Quicksand
pub(in crate::card::sets) static QUICKSAND: CardRecord = CardRecord::new(
    "Quicksand",
    "11370658-8d80-4d2f-afa5-ec6df6dee369",
    "Roger Raupp",
CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {C}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice this land: Target attacking creature without flying gets -1/-2 until end of turn.",
            &[CostDef::TapSource, CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Attacking,
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                        crate::card::KeywordAbility::Flying,
                    )),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(-2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// VIS 167 — Undiscovered Paradise
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNDISCOVERED_PARADISE: CardRecord = CardRecord::new(
    "Undiscovered Paradise",
    "5f6e8830-5e62-4945-8b73-60f0628d38e7",
    "David O'Connor",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ARCHANGEL,
    &DARAJA_GRIFFIN,
    &EQUIPOISE,
    &EYE_OF_SINGULARITY,
    &FREEWIND_FALCON,
    &GOSSAMER_CHAINS,
    &HONORABLE_PASSAGE,
    &HOPE_CHARM,
    &INFANTRY_VETERAN,
    &JAMURAAN_LION,
    &KNIGHT_OF_VALOR,
    &LONGBOW_ARCHER,
    &MIRACULOUS_RECOVERY,
    &PARAPET,
    &PEACE_TALKS,
    &RELIC_WARD,
    &REMEDY,
    &RESISTANCE_FIGHTER,
    &RETRIBUTION_OF_THE_MEEK,
    &RIGHTEOUS_AURA,
    &SUN_CLASP,
    &TEFERI_S_HONOR_GUARD,
    &TITHE,
    &WARRIOR_S_HONOR,
    &ZHALFIRIN_CRUSADER,
    &BETRAYAL,
    &BREEZEKEEPER,
    &CHRONATOG,
    &CLOUD_ELEMENTAL,
    &DESERTION,
    &DREAM_TIDES,
    &FLOODED_SHORELINE,
    &FORESHADOW,
    &IMPULSE,
    &INSPIRATION,
    &KNIGHT_OF_THE_MISTS,
    &MAN_O_WAR,
    &MYSTIC_VEIL,
    &OVINOMANCER,
    &PROSPERITY,
    &RAINBOW_EFREET,
    &SHIMMERING_EFREET,
    &SHRIEKING_DRAKE,
    &TEFERI_S_REALM,
    &THREE_WISHES,
    &TIME_AND_TIDE,
    &UNDO,
    &VANISHING,
    &VISION_CHARM,
    &WATERSPOUT_DJINN,
    &AKU_DJINN,
    &BLANKET_OF_NIGHT,
    &BROOD_OF_COCKROACHES,
    &COERCION,
    &CRYPT_RATS,
    &DARK_PRIVILEGE,
    &DEATH_WATCH,
    &DESOLATION,
    &FALLEN_ASKARI,
    &FORBIDDEN_RITUAL,
    &FUNERAL_CHARM,
    &INFERNAL_HARVEST,
    &KAERVEK_S_SPITE,
    &NECROMANCY,
    &NECROSAVANT,
    &NEKRATAAL,
    &PILLAR_TOMBS_OF_AKU,
    &PYTHON,
    &SUQ_ATA_ASSASSIN,
    &TAR_PIT_WARRIOR,
    &URBORG_MINDSUCKER,
    &VAMPIRIC_TUTOR,
    &VAMPIRISM,
    &WAKE_OF_VULTURES,
    &WICKED_REWARD,
    &BOGARDAN_PHOENIX,
    &DWARVEN_VIGILANTES,
    &ELKIN_LAIR,
    &FIREBLAST,
    &GOBLIN_RECRUITER,
    &GOBLIN_SWINE_RIDER,
    &HEARTH_CHARM,
    &HEAT_WAVE,
    &HULKING_CYCLOPS,
    &KEEPER_OF_KOOKUS,
    &KOOKUS,
    &LIGHTNING_CLOUD,
    &MOB_MENTALITY,
    &OGRE_ENFORCER,
    &RAGING_GORILLA,
    &RELENTLESS_ASSAULT,
    &ROCK_SLIDE,
    &SOLFATARA,
    &SONG_OF_BLOOD,
    &SPITTING_DRAKE,
    &SUQ_ATA_LANCER,
    &TALRUUM_CHAMPION,
    &TALRUUM_PIPER,
    &TREMOR,
    &VIASHINO_SANDSTALKER,
    &BULL_ELEPHANT,
    &CITY_OF_SOLITUDE,
    &CREEPING_MOLD,
    &ELEPHANT_GRASS,
    &ELVEN_CACHE,
    &EMERALD_CHARM,
    &FERAL_INSTINCT,
    &GIANT_CATERPILLAR,
    &KATABATIC_WINDS,
    &KING_CHEETAH,
    &KYSCU_DRAKE,
    &LICHENTHROPE,
    &MORTAL_WOUND,
    &NATURAL_ORDER,
    &PANTHER_WARRIORS,
    &QUIRION_DRUID,
    &QUIRION_RANGER,
    &RIVER_BOA,
    &ROWEN,
    &SPIDER_CLIMB,
    &STAMPEDING_WILDEBEESTS,
    &SUMMER_BLOOM,
    &UKTABI_ORANGUTAN,
    &WARTHOG,
    &WIND_SHEAR,
    &ARMY_ANTS,
    &BREATHSTEALER_S_CRYPT,
    &CORROSION,
    &FEMEREF_ENCHANTRESS,
    &FIRESTORM_HELLKITE,
    &GUIDING_SPIRIT,
    &MUNDUNGU,
    &PYGMY_HIPPO,
    &RIGHTEOUS_WAR,
    &SCALEBANE_S_ELITE,
    &SIMOON,
    &SQUANDERED_RESOURCES,
    &SULEIMAN_S_LEGACY,
    &TEMPEST_DRAKE,
    &VIASHIVAN_DRAGON,
    &ANVIL_OF_BOGARDAN,
    &BRASS_TALON_CHIMERA,
    &DIAMOND_KALEIDOSCOPE,
    &DRAGON_MASK,
    &HELM_OF_AWAKENING,
    &IRON_HEART_CHIMERA,
    &JUJU_BUBBLE,
    &LEAD_BELLY_CHIMERA,
    &MAGMA_MINE,
    &MATOPI_GOLEM,
    &PHYREXIAN_MARAUDER,
    &PHYREXIAN_WALKER,
    &SANDS_OF_TIME,
    &SISAY_S_RING,
    &SNAKE_BASKET,
    &TEFERI_S_PUZZLE_BOX,
    &TIN_WING_CHIMERA,
    &TRIANGLE_OF_WAR,
    &WAND_OF_DENIAL,
    &CORAL_ATOLL,
    &DORMANT_VOLCANO,
    &EVERGLADES,
    &GRIFFIN_CANYON,
    &JUNGLE_BASIN,
    &KAROO,
    &QUICKSAND,
    &UNDISCOVERED_PARADISE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
