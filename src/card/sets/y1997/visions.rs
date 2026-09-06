//! Visions cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::CostQuantityDef;
use crate::card::sets::y2010::worldwake as catalog_wwk;
use crate::card::sets::y2012::avacyn_restored as catalog_avr;
use crate::card::sets::y2012::return_to_ravnica as catalog_rtr;
use crate::card::sets::y2019::modern_horizons as catalog_mh1;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AlternativeCastKindDef,
    AppliedEffectDef, AppliedRuleDef, ArrivalAttachmentDef, AttackDefenderScopeDef,
    AttackRestrictionDef, BasicLandType, CardArt, CardNameDef, CardNameSetDef, CardRules, CardSet,
    CardSupertype, CardType, CounterKind, EffectDef, EffectPaymentDef, EffectRecipientDef,
    InstalledTriggerDef, KeywordAbility, ManaColor, MoveObjectsDef, ObjectPredicateDef,
    ObjectQueryDef, ObjectRefDef, ObjectSetDef, ObjectSetFilterDef, PayOrDef, PlayerRefDef,
    PlayerRelation, PlayerSetDef, ResolvedEffectDurationDef, SpellAdditionalCostDef,
    TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement,
    abilities,
};
use crate::{ParentBinding, TargetIndex, mana_cost};

// VIS 1 — Archangel (reprint)

// VIS 2 — Daraja Griffin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARAJA_GRIFFIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2f7afcaa-9df8-4dd6-89ad-bc2e15f1ec4b"),
    "Daraja Griffin",
    crate::card::CardArt::new("2f7afcaa-9df8-4dd6-89ad-bc2e15f1ec4b", "Stuart Griffin"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 3 — Equipoise
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EQUIPOISE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("53783312-3551-4361-ab02-c9651ce2a926"),
    "Equipoise",
    crate::card::CardArt::new("53783312-3551-4361-ab02-c9651ce2a926", "Adam Rex"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 4 — Eye of Singularity
pub(in crate::card::sets) static EYE_OF_SINGULARITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fa84e4ad-738a-4d23-a84c-06c39ff4200b"),
    "Eye of Singularity",
    crate::card::CardArt::new("fa84e4ad-738a-4d23-a84c-06c39ff4200b", "Eric Peterson"),
    crate::card::CardSet::Visions,
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
    PrintingAnchor::scryfall("33dc0244-319c-4e15-9083-8d21ad0364d8"),
    "Freewind Falcon",
    CardArt::new("33dc0244-319c-4e15-9083-8d21ad0364d8", "Una Fricker"),
    CardSet::Visions,
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
    PrintingAnchor::scryfall("e9917a29-c6b4-4e0a-a301-21868bd27e17"),
    "Gossamer Chains",
    crate::card::CardArt::new("e9917a29-c6b4-4e0a-a301-21868bd27e17", "Steve Luke"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 7 — Honorable Passage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HONORABLE_PASSAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6559d301-98bd-40a9-abf4-1079d7283214"),
    "Honorable Passage",
    crate::card::CardArt::new("6559d301-98bd-40a9-abf4-1079d7283214", "Jeff Miracola"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 8 — Hope Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOPE_CHARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a1a8980f-07ab-49b7-b83d-f394952ced57"),
    "Hope Charm",
    crate::card::CardArt::new("a1a8980f-07ab-49b7-b83d-f394952ced57", "Greg Spalenka"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 9 — Infantry Veteran
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INFANTRY_VETERAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0350470b-feea-4e15-bdf0-850b71dbeea6"),
    "Infantry Veteran",
    crate::card::CardArt::new("0350470b-feea-4e15-bdf0-850b71dbeea6", "Christopher Rush"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 10 — Jamuraan Lion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JAMURAAN_LION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a30df3d3-17b3-4bd0-b566-ed4d32a921f2"),
    "Jamuraan Lion",
    crate::card::CardArt::new("bfc681f5-9fff-48b6-98d9-e85c85e582a3", "Stuart Griffin"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 11 — Knight of Valor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KNIGHT_OF_VALOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("25aa80ae-bb17-4e52-a269-efe75cf4c041"),
    "Knight of Valor",
    crate::card::CardArt::new("25aa80ae-bb17-4e52-a269-efe75cf4c041", "Jeff Miracola"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 12 — Longbow Archer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LONGBOW_ARCHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e2ee185d-f5ae-4b1d-90a4-840182f87ab8"),
    "Longbow Archer",
    crate::card::CardArt::new("e2ee185d-f5ae-4b1d-90a4-840182f87ab8", "Eric Peterson"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 13 — Miraculous Recovery
pub(in crate::card::sets) static MIRACULOUS_RECOVERY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("76fecb31-790a-4454-918e-5aeb253021f0"),
    "Miraculous Recovery",
    CardArt::new("76fecb31-790a-4454-918e-5aeb253021f0", "Brian Horton"),
    CardSet::Visions,
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PARAPET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a7bbcaa9-edbf-48ad-bcd2-65e8fb9bb938"),
    "Parapet",
    crate::card::CardArt::new("a7bbcaa9-edbf-48ad-bcd2-65e8fb9bb938", "Mark Poole"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 15 — Peace Talks
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PEACE_TALKS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d4200017-5b26-40fe-9a0d-2872aa3b017e"),
    "Peace Talks",
    crate::card::CardArt::new("21da279d-a723-4902-bf84-dfe2c569d4c8", "Roger Raupp"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 16 — Relic Ward
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RELIC_WARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f0459667-b7da-43bd-b981-0e515432d147"),
    "Relic Ward",
    crate::card::CardArt::new("f0459667-b7da-43bd-b981-0e515432d147", "John Coulthart"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 17 — Remedy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REMEDY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2a0b7162-4422-4dfb-a6ca-8d89fa74e6dc"),
    "Remedy",
    crate::card::CardArt::new("2a0b7162-4422-4dfb-a6ca-8d89fa74e6dc", "Zina Saunders"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 18 — Resistance Fighter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RESISTANCE_FIGHTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("21250bdb-9431-41b3-9fef-d66a4d3f6ecd"),
    "Resistance Fighter",
    crate::card::CardArt::new("21250bdb-9431-41b3-9fef-d66a4d3f6ecd", "Cecil Fernando"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 19 — Retribution of the Meek
pub(in crate::card::sets) static RETRIBUTION_OF_THE_MEEK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("860b8633-1bfc-426a-8666-5e6a584d4525"),
    "Retribution of the Meek",
    CardArt::new("860b8633-1bfc-426a-8666-5e6a584d4525", "Nathalie Hertz"),
    CardSet::Visions,
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
    PrintingAnchor::scryfall("fed82843-2853-42d3-bcf6-b831032b7a69"),
    "Righteous Aura",
    crate::card::CardArt::new("fed82843-2853-42d3-bcf6-b831032b7a69", "Jeff Miracola"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 21 — Sun Clasp
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUN_CLASP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e3f1fb74-bc08-4c3b-9fbe-da6973aaeaa2"),
    "Sun Clasp",
    crate::card::CardArt::new("e3f1fb74-bc08-4c3b-9fbe-da6973aaeaa2", "John Coulthart"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 22 — Teferi's Honor Guard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEFERI_S_HONOR_GUARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4177d5bf-db48-4bbf-bbd4-ee6313031920"),
    "Teferi's Honor Guard",
    crate::card::CardArt::new("4177d5bf-db48-4bbf-bbd4-ee6313031920", "Cecil Fernando"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 23 — Tithe
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TITHE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aae08938-e563-4322-b2eb-db81913ea730"),
    "Tithe",
    crate::card::CardArt::new("aae08938-e563-4322-b2eb-db81913ea730", "Jon J Muth"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 24 — Warrior's Honor
pub(in crate::card::sets) static WARRIOR_S_HONOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7babd273-3e20-4cf9-bf21-c602eb729fc5"),
    "Warrior's Honor",
    CardArt::new(
        "7babd273-3e20-4cf9-bf21-c602eb729fc5",
        "D. Alexander Gregory",
    ),
    CardSet::Visions,
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
    PrintingAnchor::scryfall("d8ed802f-6e54-4fed-a71e-6d404c2c664b"),
    "Zhalfirin Crusader",
    crate::card::CardArt::new("d8ed802f-6e54-4fed-a71e-6d404c2c664b", "Alan Rabinowitz"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 26 — Betrayal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BETRAYAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f9b5c75-882e-4fe4-827f-584080e91485"),
    "Betrayal",
    crate::card::CardArt::new("7f9b5c75-882e-4fe4-827f-584080e91485", "Gary Leach"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 27 — Breezekeeper
// Audit: unsupported — Needs phasing as a keyword. EffectDef::PhaseOut is a one-shot that phases a permanent out once; the keyword is a static that phases its permanent in and out before every one of its controller's untap steps.
pub(in crate::card::sets) static BREEZEKEEPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("beaefa77-6e4a-4724-a443-fa6b45803db5"),
    "Breezekeeper",
    crate::card::CardArt::new("beaefa77-6e4a-4724-a443-fa6b45803db5", "Adam Rex"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 28 — Chronatog
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHRONATOG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("05ada02f-04e9-4269-b04a-97a7eaac2c46"),
    "Chronatog",
    crate::card::CardArt::new("05ada02f-04e9-4269-b04a-97a7eaac2c46", "Christopher Rush"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 29 — Cloud Elemental
pub(in crate::card::sets) static CLOUD_ELEMENTAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4f2a5146-cf2e-40c0-b498-06e611343196"),
    "Cloud Elemental",
    CardArt::new("4f2a5146-cf2e-40c0-b498-06e611343196", "Adam Rex"),
    CardSet::Visions,
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
    PrintingAnchor::scryfall("9a2a1779-af08-4a9a-aba4-e6892ce2332c"),
    "Desertion",
    crate::card::CardArt::new(
        "9a2a1779-af08-4a9a-aba4-e6892ce2332c",
        "Richard Kane Ferguson",
    ),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 31 — Dream Tides
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DREAM_TIDES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3bd292a0-ec08-4250-8d75-0802e985d6e6"),
    "Dream Tides",
    crate::card::CardArt::new("3bd292a0-ec08-4250-8d75-0802e985d6e6", "Jerry Tiritilli"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 32 — Flooded Shoreline
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLOODED_SHORELINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("49db9f58-380f-496e-9d3d-6776d30fb564"),
    "Flooded Shoreline",
    crate::card::CardArt::new("49db9f58-380f-496e-9d3d-6776d30fb564", "Romas Kukalis"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 33 — Foreshadow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORESHADOW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d54c51de-bfac-4198-a7c4-37b4db74e525"),
    "Foreshadow",
    crate::card::CardArt::new("d54c51de-bfac-4198-a7c4-37b4db74e525", "George Pratt"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 34 — Impulse
pub(in crate::card::sets) static IMPULSE: CardRecord = CardRecord::new_with_legacy_id(
    310,
    "Impulse",
    CardArt::new("9d710a97-062f-4773-b6c6-8aeddeb3b6e8", "Bryan Talbot"),
    CardSet::Visions,
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

// VIS 35 — Inspiration (reprint)

// VIS 36 — Knight of the Mists
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KNIGHT_OF_THE_MISTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("37924cbc-fb9d-4906-9ad2-9b6d4ccfff0f"),
    "Knight of the Mists",
    crate::card::CardArt::new("37924cbc-fb9d-4906-9ad2-9b6d4ccfff0f", "Harold McNeill"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 37 — Man-o'-War (reprint)

// VIS 38 — Mystic Veil
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MYSTIC_VEIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7ddb640d-5c54-4d0a-b8c2-e22fe04f96c2"),
    "Mystic Veil",
    crate::card::CardArt::new(
        "7ddb640d-5c54-4d0a-b8c2-e22fe04f96c2",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 39 — Ovinomancer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OVINOMANCER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("978eb187-50dc-4774-993b-7e95be360d25"),
    "Ovinomancer",
    crate::card::CardArt::new("ae4f0988-4194-4481-a6b7-27753261174a", "Kev Walker"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 40 — Prosperity
pub(in crate::card::sets) static PROSPERITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3fa5e806-3cf2-4241-b45d-a05d2b715efd"),
    "Prosperity",
    CardArt::new("3fa5e806-3cf2-4241-b45d-a05d2b715efd", "Dan Frazier"),
    CardSet::Visions,
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAINBOW_EFREET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1d6f03a6-3665-40e4-ae68-640913972770"),
    "Rainbow Efreet",
    crate::card::CardArt::new("1d6f03a6-3665-40e4-ae68-640913972770", "Nathalie Hertz"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 42 — Shimmering Efreet
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIMMERING_EFREET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("23c5704f-5856-4422-9d82-14558dbe1434"),
    "Shimmering Efreet",
    crate::card::CardArt::new("23c5704f-5856-4422-9d82-14558dbe1434", "Thomas Gianni"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 43 — Shrieking Drake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHRIEKING_DRAKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dae31023-2649-4f37-b1ec-d9f650d53f09"),
    "Shrieking Drake",
    crate::card::CardArt::new("63971a64-c5f3-4d1f-ae0d-489d7d5b18f0", "Ian Miller"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 44 — Teferi's Realm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEFERI_S_REALM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aba3e4ea-2241-4f1e-a46b-70f512fe729e"),
    "Teferi's Realm",
    crate::card::CardArt::new("aba3e4ea-2241-4f1e-a46b-70f512fe729e", "Alan Rabinowitz"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 45 — Three Wishes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THREE_WISHES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dbb2b253-7023-44d1-963b-eae98d48f498"),
    "Three Wishes",
    crate::card::CardArt::new("dbb2b253-7023-44d1-963b-eae98d48f498", "George Pratt"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 46 — Time and Tide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIME_AND_TIDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("152b348a-0301-4d45-a2c1-d78802c445ba"),
    "Time and Tide",
    crate::card::CardArt::new("152b348a-0301-4d45-a2c1-d78802c445ba", "George Pratt"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 47 — Undo
pub(in crate::card::sets) static UNDO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2bef942e-9d17-4d40-a4c9-8be715e73a08"),
    "Undo",
    CardArt::new("2bef942e-9d17-4d40-a4c9-8be715e73a08", "Terese Nielsen"),
    CardSet::Visions,
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
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
        },
    )),
);

// VIS 48 — Vanishing
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VANISHING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8d1fb805-1382-458c-b98d-4491f13833b6"),
    "Vanishing",
    crate::card::CardArt::new("8d1fb805-1382-458c-b98d-4491f13833b6", "John Matson"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 49 — Vision Charm
pub(in crate::card::sets) static VISION_CHARM: CardRecord = CardRecord::new_with_legacy_id(
    2090,
    "Vision Charm",
    CardArt::new("0efaa72c-8f65-4488-ad66-80dc877166cc", "Greg Spalenka"),
    CardSet::Visions,
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WATERSPOUT_DJINN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6946a75e-e9d1-4a56-86d1-dd81f7b1b125"),
    "Waterspout Djinn",
    crate::card::CardArt::new("6946a75e-e9d1-4a56-86d1-dd81f7b1b125", "Thomas Gianni"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 51 — Aku Djinn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AKU_DJINN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("369a5df5-fc36-476c-84f4-ec4bdeb4f9d2"),
    "Aku Djinn",
    crate::card::CardArt::new("369a5df5-fc36-476c-84f4-ec4bdeb4f9d2", "Terese Nielsen"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 52 — Blanket of Night
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLANKET_OF_NIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fe012fd0-9ff0-4436-a890-3ab436e42201"),
    "Blanket of Night",
    crate::card::CardArt::new("fe012fd0-9ff0-4436-a890-3ab436e42201", "Cliff Nielsen"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 53 — Brood of Cockroaches
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BROOD_OF_COCKROACHES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("30b6150e-7d0c-4361-b99b-79de96dfc53a"),
    "Brood of Cockroaches",
    crate::card::CardArt::new(
        "30b6150e-7d0c-4361-b99b-79de96dfc53a",
        "Geofrey Darrow & I. Rabarot",
    ),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 54 — Coercion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COERCION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f3b07d33-f5f5-45cc-b2ac-360eaf2d4146"),
    "Coercion",
    crate::card::CardArt::new("f3b07d33-f5f5-45cc-b2ac-360eaf2d4146", "DiTerlizzi"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 55 — Crypt Rats
// Audit: unsupported — Needs "spend only <colour> mana on X" for an activated ability. The restriction is card-level and the payment layer reads it only for ManaPaymentPurpose::Spell, so an ability's X is unrestricted; implementing this without it would let any mana pay X and make the card strictly better than printed.
pub(in crate::card::sets) static CRYPT_RATS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("736455f6-c1b3-4a5a-a91f-a0cd3986ed53"),
    "Crypt Rats",
    crate::card::CardArt::new("736455f6-c1b3-4a5a-a91f-a0cd3986ed53", "Paul Lee"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 56 — Dark Privilege
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARK_PRIVILEGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c63ecd7b-a5e6-4e19-9ca2-dda14754305a"),
    "Dark Privilege",
    crate::card::CardArt::new("10d2cf44-cc20-4a37-81ae-930f8c6d0896", "Tom Kyffin"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 57 — Death Watch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEATH_WATCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0e939d8f-6989-4884-989b-9cba566c9963"),
    "Death Watch",
    crate::card::CardArt::new("0e939d8f-6989-4884-989b-9cba566c9963", "Brian Horton"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 58 — Desolation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DESOLATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3b186460-d2af-4912-ba19-95b2cb5f1639"),
    "Desolation",
    crate::card::CardArt::new("3b186460-d2af-4912-ba19-95b2cb5f1639", "George Pratt"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 59 — Fallen Askari
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FALLEN_ASKARI: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("00107210-313f-49c1-84ff-92628f75b764"),
    "Fallen Askari",
    crate::card::CardArt::new("00107210-313f-49c1-84ff-92628f75b764", "Adrian Smith"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 60 — Forbidden Ritual
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORBIDDEN_RITUAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f5327e6d-db4e-4b44-a00e-b764e80b8946"),
    "Forbidden Ritual",
    crate::card::CardArt::new("f5327e6d-db4e-4b44-a00e-b764e80b8946", "Christopher Rush"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 61 — Funeral Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FUNERAL_CHARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e79d7240-2014-4838-bace-80666192a73e"),
    "Funeral Charm",
    crate::card::CardArt::new("e79d7240-2014-4838-bace-80666192a73e", "Greg Spalenka"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 62 — Infernal Harvest
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INFERNAL_HARVEST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ccf85ac9-f5d8-4a36-aa6c-3a31427a0348"),
    "Infernal Harvest",
    crate::card::CardArt::new("ccf85ac9-f5d8-4a36-aa6c-3a31427a0348", "Nathalie Hertz"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 63 — Kaervek's Spite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAERVEK_S_SPITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d385b9e5-e13d-4098-ba74-ea55bde164d9"),
    "Kaervek's Spite",
    crate::card::CardArt::new("d385b9e5-e13d-4098-ba74-ea55bde164d9", "Bryan Talbot"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 64 — Necromancy
pub(in crate::card::sets) static NECROMANCY: CardRecord = CardRecord::new_with_legacy_id(
    2202,
    "Necromancy",
    CardArt::new("311a6257-dd77-4bb6-81cb-c8e7862350f3", "Pete Venters"),
    CardSet::Visions,
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
                            EffectDef::MoveToZone {
                                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                zone: ZoneKind::Battlefield,
                                placement: ZonePlacement::Top,
                            }
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
                                    EffectDef::Sacrifice {
                                        object: EffectRecipientDef::Source,
                                    },
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
                EffectDef::Sacrifice {
                    object: EffectRecipientDef::AttachedPermanent,
                },
            ),
        ]),
);

// VIS 65 — Necrosavant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NECROSAVANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("50668c85-49ad-456c-9022-0abe560bc50c"),
    "Necrosavant",
    crate::card::CardArt::new("e70cd5fa-ae66-4ea4-90d2-28af2aa34dd4", "John Coulthart"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 66 — Nekrataal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NEKRATAAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dba3e342-88b7-4692-a3f7-a3f56c0cf6b5"),
    "Nekrataal",
    crate::card::CardArt::new("dba3e342-88b7-4692-a3f7-a3f56c0cf6b5", "Adrian Smith"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 67 — Pillar Tombs of Aku
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PILLAR_TOMBS_OF_AKU: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("153f93fd-4f2c-4dce-a774-4483031ed532"),
    "Pillar Tombs of Aku",
    crate::card::CardArt::new("153f93fd-4f2c-4dce-a774-4483031ed532", "Terese Nielsen"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 68 — Python
pub(in crate::card::sets) static PYTHON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e7e99969-6c21-4de6-ba57-44ef7f9c8c47"),
    "Python",
    CardArt::new("e7e99969-6c21-4de6-ba57-44ef7f9c8c47", "Steve White"),
    CardSet::Visions,
    // A vanilla 3/2 for three, which is the rate black pays when it is not
    // buying an ability with the same card.
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Snake"], 3, 2),
);

// VIS 69 — Suq'Ata Assassin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUQ_ATA_ASSASSIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1b7178c6-f989-437d-83e3-04b9817f2c54"),
    "Suq'Ata Assassin",
    crate::card::CardArt::new("1b7178c6-f989-437d-83e3-04b9817f2c54", "Gary Gianni"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 70 — Tar Pit Warrior
pub(in crate::card::sets) static TAR_PIT_WARRIOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e1283190-094e-4a9f-bf67-f9fd05778744"),
    "Tar Pit Warrior",
    CardArt::new("e1283190-094e-4a9f-bf67-f9fd05778744", "George Pratt"),
    CardSet::Visions,
    // A 3/4 for three, which is above rate until anybody points anything
    // at it -- including your own trick.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Cyclops", "Warrior"], 3, 4).with_ability(
        AbilityDef::triggered(
            "When this creature becomes the target of a spell or ability, sacrifice it.",
            // Any spell or ability, including its controller's own: a
            // pump spell kills it just as surely as removal does.
            TriggerEventDef::becomes_targeted(ObjectPredicateDef::Any),
            EffectDef::Sacrifice {
                object: EffectRecipientDef::Source,
            },
        ),
    ),
);

// VIS 71 — Urborg Mindsucker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static URBORG_MINDSUCKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8acbd8c6-da34-45d6-921a-11f370662833"),
    "Urborg Mindsucker",
    crate::card::CardArt::new("78405864-fc83-47ab-9238-8e0464a700ec", "DiTerlizzi"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
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

pub(in crate::card::sets) static VAMPIRIC_TUTOR: CardRecord = CardRecord::new_with_legacy_id(
    2108,
    "Vampiric Tutor",
    CardArt::new("0a07cba3-2e8d-48ec-a6f8-4d2edfcd833d", "Gary Leach"),
    CardSet::Visions,
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell(
        "Search your library for a card, then shuffle and put that card on top. You lose 2 life.",
        EffectDef::Sequence(&VAMPIRIC_TUTOR_EFFECT),
    )),
);

// VIS 73 — Vampirism
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VAMPIRISM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c65b5cfd-d45b-4144-8608-541d455fb004"),
    "Vampirism",
    crate::card::CardArt::new("2dff2817-1813-410f-aca7-96e8f9f4ce81", "Gary Leach"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 74 — Wake of Vultures
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAKE_OF_VULTURES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("52420b80-7f34-4426-ac97-a6e15167c7a9"),
    "Wake of Vultures",
    crate::card::CardArt::new("52420b80-7f34-4426-ac97-a6e15167c7a9", "Jeff Miracola"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 75 — Wicked Reward
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WICKED_REWARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e02aeae5-4918-42c6-872d-ffe1517de2ad"),
    "Wicked Reward",
    crate::card::CardArt::new(
        "ee32f8ba-3547-4913-a555-d43ee2978ba9",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 76 — Bogardan Phoenix
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOGARDAN_PHOENIX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("253db28a-3873-4364-80d7-a8164000ea9e"),
    "Bogardan Phoenix",
    crate::card::CardArt::new("253db28a-3873-4364-80d7-a8164000ea9e", "David O'Connor"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 77 — Dwarven Vigilantes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DWARVEN_VIGILANTES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("077d33bb-41bf-440d-939b-67ab5aacb092"),
    "Dwarven Vigilantes",
    crate::card::CardArt::new("077d33bb-41bf-440d-939b-67ab5aacb092", "Pete Venters"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 78 — Elkin Lair
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELKIN_LAIR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bcb625ba-3718-4988-962c-bf2e11eb4c16"),
    "Elkin Lair",
    crate::card::CardArt::new("bcb625ba-3718-4988-962c-bf2e11eb4c16", "Jerry Tiritilli"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 79 — Fireblast
pub(in crate::card::sets) static FIREBLAST: CardRecord = CardRecord::new_with_legacy_id(
    2035,
    "Fireblast",
    CardArt::new("b1eb5b2c-1f02-48a6-a287-88eb189d6780", "Michael Danza"),
    CardSet::Visions,
    CardRules::new_instant(mana_cost!("{4}{R}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Fireblast deals 4 damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(4),
            },
        ),
        AbilityDef::alternative_cast(
            crate::mana_cost!("{0}"),
            AlternativeCastKindDef::AlternativeCost,
            Some("You may sacrifice two Mountains rather than pay this spell's mana cost."),
            EffectDef::None,
        )
        // Two Mountains off the battlefield, which is why the card is a finisher
        // rather than a burn spell: it is cast from an empty board on the turn the
        // lands stop mattering.
        .with_alternative_additional_cost(&SpellAdditionalCostDef::sacrifice(
            ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Mountain]),
            CostQuantityDef::Fixed(2),
        )),
    ]),
);

// VIS 80 — Goblin Recruiter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_RECRUITER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6ee791d5-1d48-40e8-b65f-b6aa889f3467"),
    "Goblin Recruiter",
    crate::card::CardArt::new("6ee791d5-1d48-40e8-b65f-b6aa889f3467", "Scott Kirschner"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 81 — Goblin Swine-Rider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_SWINE_RIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("49980982-d534-4204-bc15-3e6c4ffa1a53"),
    "Goblin Swine-Rider",
    crate::card::CardArt::new(
        "49980982-d534-4204-bc15-3e6c4ffa1a53",
        "Geofrey Darrow & I. Rabarot",
    ),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 82 — Hearth Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEARTH_CHARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("caa9ac66-51b7-4aec-92dc-0f0656b0f7fe"),
    "Hearth Charm",
    crate::card::CardArt::new("caa9ac66-51b7-4aec-92dc-0f0656b0f7fe", "Greg Spalenka"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 83 — Heat Wave
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEAT_WAVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("42dd0810-4528-4a88-add8-923bb2057821"),
    "Heat Wave",
    crate::card::CardArt::new("42dd0810-4528-4a88-add8-923bb2057821", "Alan Rabinowitz"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 84 — Hulking Cyclops
pub(in crate::card::sets) static HULKING_CYCLOPS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a3ee5ea8-7023-4dde-ab51-d3ba234d74b9"),
    "Hulking Cyclops",
    CardArt::new("a3ee5ea8-7023-4dde-ab51-d3ba234d74b9", "DiTerlizzi"),
    CardSet::Visions,
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KEEPER_OF_KOOKUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d11b6df4-449f-44ea-a4fa-f079bcd26a54"),
    "Keeper of Kookus",
    crate::card::CardArt::new("d11b6df4-449f-44ea-a4fa-f079bcd26a54", "Scott Hampton"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 86 — Kookus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KOOKUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8fb90922-99d2-4b36-9039-bb806fd01756"),
    "Kookus",
    crate::card::CardArt::new("8fb90922-99d2-4b36-9039-bb806fd01756", "Scott Hampton"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 87 — Lightning Cloud
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIGHTNING_CLOUD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7fcfc2ad-a1a4-4f65-a239-f11383aaafe1"),
    "Lightning Cloud",
    crate::card::CardArt::new("7fcfc2ad-a1a4-4f65-a239-f11383aaafe1", "John Matson"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 88 — Mob Mentality
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOB_MENTALITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e428d56a-9445-4e86-b281-656e2d251e0b"),
    "Mob Mentality",
    crate::card::CardArt::new("e428d56a-9445-4e86-b281-656e2d251e0b", "Douglas Shuler"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 89 — Ogre Enforcer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OGRE_ENFORCER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b0f072d6-7489-4eb0-8c53-1fa42ad806a4"),
    "Ogre Enforcer",
    crate::card::CardArt::new("b0f072d6-7489-4eb0-8c53-1fa42ad806a4", "Pete Venters"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 90 — Raging Gorilla
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAGING_GORILLA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("07c284ce-33b8-4fb2-9dd9-4c477bedc774"),
    "Raging Gorilla",
    crate::card::CardArt::new("07c284ce-33b8-4fb2-9dd9-4c477bedc774", "Tom Kyffin"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 91 — Relentless Assault
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RELENTLESS_ASSAULT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("747161ea-cb65-4960-84dd-a05bfe5f3ba0"),
    "Relentless Assault",
    crate::card::CardArt::new(
        "747161ea-cb65-4960-84dd-a05bfe5f3ba0",
        "Geofrey Darrow & I. Rabarot",
    ),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 92 — Rock Slide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROCK_SLIDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7e01717a-d6ed-42c1-9a9a-f3f4a3d73bca"),
    "Rock Slide",
    crate::card::CardArt::new("7e01717a-d6ed-42c1-9a9a-f3f4a3d73bca", "Mike Kerr"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 93 — Solfatara
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLFATARA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c5d4bd6f-b019-4594-aa41-138fa58ba529"),
    "Solfatara",
    crate::card::CardArt::new("c5d4bd6f-b019-4594-aa41-138fa58ba529", "Omaha Pérez"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 94 — Song of Blood
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SONG_OF_BLOOD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4497a1d7-6604-4f2d-9484-1f1d77a6228f"),
    "Song of Blood",
    crate::card::CardArt::new("4497a1d7-6604-4f2d-9484-1f1d77a6228f", "Eric Peterson"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 95 — Spitting Drake
pub(in crate::card::sets) static SPITTING_DRAKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c9f6ef97-587f-4f7b-98a2-e3cc8b39df8b"),
    "Spitting Drake",
    CardArt::new(
        "c9f6ef97-587f-4f7b-98a2-e3cc8b39df8b",
        "Geofrey Darrow & I. Rabarot",
    ),
    CardSet::Visions,
    // One activation a turn, so it is a 3/2 flier rather than a mana sink.
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Drake"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{R}: This creature gets +1/+0 until end of turn. Activate only once each turn.",
            &[AbilityCostDef::Mana(mana_cost!("{R}"))],
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUQ_ATA_LANCER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2884d8df-7fd5-4247-9da5-38c31333ff5d"),
    "Suq'Ata Lancer",
    crate::card::CardArt::new("2884d8df-7fd5-4247-9da5-38c31333ff5d", "Jeff Miracola"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 97 — Talruum Champion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TALRUUM_CHAMPION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1526a1e7-b874-4409-8c84-81996fbc8d12"),
    "Talruum Champion",
    crate::card::CardArt::new("33730a07-754c-4606-bfac-d73454af9567", "Pete Venters"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 98 — Talruum Piper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TALRUUM_PIPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ca2cb9a7-5063-4b31-9782-8bfd784bca0a"),
    "Talruum Piper",
    crate::card::CardArt::new("ca2cb9a7-5063-4b31-9782-8bfd784bca0a", "Pete Venters"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 99 — Tremor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TREMOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a9d64665-c1e0-40ab-a358-247f82966379"),
    "Tremor",
    crate::card::CardArt::new("a9d64665-c1e0-40ab-a358-247f82966379", "Michael Danza"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 100 — Viashino Sandstalker
pub(in crate::card::sets) static VIASHINO_SANDSTALKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0eb0579a-a0b0-43b0-884c-13035158ae64"),
    "Viashino Sandstalker",
    CardArt::new("01770e13-ebd4-4c83-9e72-99374239a63d", "Andrew Robinson"),
    CardSet::Visions,
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
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Source,
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
            ),
        ],
    ),
);

// VIS 101 — Bull Elephant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BULL_ELEPHANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6b161f83-a0d9-4b65-af36-6d71aa76c912"),
    "Bull Elephant",
    crate::card::CardArt::new("fa7f5f41-ed30-412b-b51e-37d26e9e6455", "Steve White"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 102 — City of Solitude
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CITY_OF_SOLITUDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("be499b81-bb2d-4f1d-9deb-c8bfcdca8e13"),
    "City of Solitude",
    crate::card::CardArt::new("be499b81-bb2d-4f1d-9deb-c8bfcdca8e13", "Romas Kukalis"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 103 — Creeping Mold
pub(in crate::card::sets) static CREEPING_MOLD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("36e7691f-c771-4451-ac54-3532ca10d48f"),
    "Creeping Mold",
    CardArt::new("36e7691f-c771-4451-ac54-3532ca10d48f", "David Seeley"),
    CardSet::Visions,
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
    PrintingAnchor::scryfall("f4c1f5a7-0d28-43ab-9b66-937e963f42cd"),
    "Elephant Grass",
    CardArt::new("f4c1f5a7-0d28-43ab-9b66-937e963f42cd", "Tony Roberts"),
    CardSet::Visions,
    CardRules::new_enchantment(mana_cost!("{G}")).with_abilities(&[
        AbilityDef::triggered(
            "Cumulative upkeep {1} (At the beginning of your upkeep, put an age counter on this permanent, then sacrifice it unless you pay its upkeep cost for each age counter on it.)",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceOnBattlefield,
                then: &EffectDef::Sequence(&[
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::named("age"),
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::PayOr(PayOrDef::unless(
                        EffectPaymentDef::generic_mana(
                            PlayerSetDef::One(PlayerRefDef::EffectController),
                            ValueDef::CountersOnSource(CounterKind::named("age")),
                        ),
                        &EffectDef::Sacrifice {
                            object: EffectRecipientDef::Source,
                        },
                    )),
                ]),
            },
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
    PrintingAnchor::scryfall("80fa078f-c74a-42b2-af97-7ca2c29dc316"),
    "Elven Cache",
    CardArt::new("80fa078f-c74a-42b2-af97-7ca2c29dc316", "John Matson"),
    CardSet::Visions,
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
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
        },
    )),
);

// VIS 106 — Emerald Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EMERALD_CHARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e9c9199b-61b3-4794-878b-f065058f50f3"),
    "Emerald Charm",
    crate::card::CardArt::new("e9c9199b-61b3-4794-878b-f065058f50f3", "Greg Spalenka"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 107 — Feral Instinct
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FERAL_INSTINCT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("20dec7cf-2865-4642-9022-d3006fd7ac30"),
    "Feral Instinct",
    crate::card::CardArt::new("20dec7cf-2865-4642-9022-d3006fd7ac30", "Una Fricker"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 108 — Giant Caterpillar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GIANT_CATERPILLAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b7f602a6-3d35-49a3-b5cb-d754e03a9573"),
    "Giant Caterpillar",
    crate::card::CardArt::new("b7f602a6-3d35-49a3-b5cb-d754e03a9573", "Zina Saunders"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 109 — Katabatic Winds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KATABATIC_WINDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("97b34ce8-1eb2-44eb-813a-09d0308e27a0"),
    "Katabatic Winds",
    crate::card::CardArt::new("97b34ce8-1eb2-44eb-813a-09d0308e27a0", "Gary Gianni"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 110 — King Cheetah
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KING_CHEETAH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0c607ba1-e133-47f3-a79a-5f0dc8c4b9ac"),
    "King Cheetah",
    crate::card::CardArt::new("38149d49-8661-427c-9338-93c11a2a8093", "Terese Nielsen"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 111 — Kyscu Drake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KYSCU_DRAKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b6f14bbe-2436-4a5a-8e2a-8066b740b715"),
    "Kyscu Drake",
    crate::card::CardArt::new(
        "b6f14bbe-2436-4a5a-8e2a-8066b740b715",
        "Geofrey Darrow & I. Rabarot",
    ),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 112 — Lichenthrope
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LICHENTHROPE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("76f0c356-a81d-41d4-a8b7-8c159146a8b8"),
    "Lichenthrope",
    crate::card::CardArt::new("76f0c356-a81d-41d4-a8b7-8c159146a8b8", "Bob Eggleton"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 113 — Mortal Wound
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MORTAL_WOUND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("808830ff-496a-41dc-8b64-334ddaca9435"),
    "Mortal Wound",
    crate::card::CardArt::new("808830ff-496a-41dc-8b64-334ddaca9435", "Kev Walker"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 114 — Natural Order
pub(in crate::card::sets) static NATURAL_ORDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0845f0b0-9413-4ddd-861d-9607636bebc6"),
    "Natural Order",
    CardArt::new("0845f0b0-9413-4ddd-861d-9607636bebc6", "Terese Nielsen"),
    CardSet::Visions,
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
            SpellAdditionalCostDef::sacrifice(
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
    PrintingAnchor::scryfall("76c9bc99-28e3-4d64-8383-2b92011104ed"),
    "Panther Warriors",
    CardArt::new("76c9bc99-28e3-4d64-8383-2b92011104ed", "Cecil Fernando"),
    CardSet::Visions,
    // Five mana for six power and no defence at all: it wins every race and
    // loses every fight it does not start.
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Cat", "Warrior"], 6, 3),
);

// VIS 116 — Quirion Druid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QUIRION_DRUID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8ca5319a-5c26-487f-ba87-d317633122ba"),
    "Quirion Druid",
    crate::card::CardArt::new("8ca5319a-5c26-487f-ba87-d317633122ba", "John Matson"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 117 — Quirion Ranger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QUIRION_RANGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("56efe72c-6d7f-44f6-ac74-01af9305c4b6"),
    "Quirion Ranger",
    crate::card::CardArt::new("56efe72c-6d7f-44f6-ac74-01af9305c4b6", "Tom Kyffin"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 118 — River Boa
pub(in crate::card::sets) static RIVER_BOA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2e9d5aaf-b7e8-4676-aec8-7d29a0169a2c"),
    "River Boa",
    CardArt::new("2e9d5aaf-b7e8-4676-aec8-7d29a0169a2c", "Steve White"),
    CardSet::Visions,
    // Unblockable against blue and hard to kill in combat, which is why a
    // two-drop bear has been a sideboard card for thirty years.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Snake"], 2, 1).with_abilities(&[
        abilities::landwalk(BasicLandType::Island),
        abilities::regenerate_self(
            "{G}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{G}"))],
        ),
    ]),
);

// VIS 119 — Rowen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROWEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("07144d84-f7f3-4101-805d-07cce8342a64"),
    "Rowen",
    crate::card::CardArt::new("07144d84-f7f3-4101-805d-07cce8342a64", "Jon J Muth"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 120 — Spider Climb
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIDER_CLIMB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a1818812-4cb8-4fe1-98c0-b40086b4991c"),
    "Spider Climb",
    crate::card::CardArt::new("a1818812-4cb8-4fe1-98c0-b40086b4991c", "Ron Spencer"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 121 — Stampeding Wildebeests
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STAMPEDING_WILDEBEESTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ddb5f524-fad6-4a63-b20f-3348a844fefa"),
    "Stampeding Wildebeests",
    crate::card::CardArt::new("ddb5f524-fad6-4a63-b20f-3348a844fefa", "Randy Gallegos"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 122 — Summer Bloom
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUMMER_BLOOM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("35d78f4e-d95d-49bc-9971-06a68a4e35fd"),
    "Summer Bloom",
    crate::card::CardArt::new("35d78f4e-d95d-49bc-9971-06a68a4e35fd", "Nicola Leonard"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 123 — Uktabi Orangutan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UKTABI_ORANGUTAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("101c7d58-43cc-4ebd-87f1-2016fbff56dd"),
    "Uktabi Orangutan",
    crate::card::CardArt::new("101c7d58-43cc-4ebd-87f1-2016fbff56dd", "Una Fricker"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 124 — Warthog
pub(in crate::card::sets) static WARTHOG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dd2510b8-52d6-4d2e-89a5-31b27b732dd8"),
    "Warthog",
    CardArt::new("dd2510b8-52d6-4d2e-89a5-31b27b732dd8", "Steve White"),
    CardSet::Visions,
    // A green creature with swampwalk, which is Visions asking green to
    // attack the colour it is worst against.
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Boar"], 3, 2)
        .with_ability(abilities::landwalk(BasicLandType::Swamp)),
);

// VIS 125 — Wind Shear
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WIND_SHEAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b8324f44-c7f5-41ee-bc8d-16822bd8942f"),
    "Wind Shear",
    crate::card::CardArt::new("b8324f44-c7f5-41ee-bc8d-16822bd8942f", "John Matson"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 126 — Army Ants
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARMY_ANTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7e129be5-e2c5-4f69-b8e8-539ac2085c7a"),
    "Army Ants",
    crate::card::CardArt::new(
        "7e129be5-e2c5-4f69-b8e8-539ac2085c7a",
        "Geofrey Darrow & I. Rabarot",
    ),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 127 — Breathstealer's Crypt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BREATHSTEALER_S_CRYPT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f87ace53-d77c-4df5-b200-4be2ac2b7fdb"),
    "Breathstealer's Crypt",
    crate::card::CardArt::new("f87ace53-d77c-4df5-b200-4be2ac2b7fdb", "Blackie del Rio"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 128 — Corrosion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CORROSION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("176122b2-f60f-4150-8c0c-757c8f8914d2"),
    "Corrosion",
    crate::card::CardArt::new("176122b2-f60f-4150-8c0c-757c8f8914d2", "Michael Danza"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 129 — Femeref Enchantress
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FEMEREF_ENCHANTRESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("20ba72c7-7957-4d02-b41e-c0132fe1f2e6"),
    "Femeref Enchantress",
    crate::card::CardArt::new(
        "20ba72c7-7957-4d02-b41e-c0132fe1f2e6",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 130 — Firestorm Hellkite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FIRESTORM_HELLKITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("def23574-4a41-4323-84d9-49f58b2ca322"),
    "Firestorm Hellkite",
    crate::card::CardArt::new("def23574-4a41-4323-84d9-49f58b2ca322", "Pete Venters"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 131 — Guiding Spirit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GUIDING_SPIRIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5f96d184-0ef8-40f7-98bc-bd4c53c57072"),
    "Guiding Spirit",
    crate::card::CardArt::new("5f96d184-0ef8-40f7-98bc-bd4c53c57072", "Terese Nielsen"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 132 — Mundungu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MUNDUNGU: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d6e320ca-848b-4743-93f1-ec04ef1ce402"),
    "Mundungu",
    crate::card::CardArt::new("d6e320ca-848b-4743-93f1-ec04ef1ce402", "Terese Nielsen"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 133 — Pygmy Hippo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PYGMY_HIPPO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2e3f6220-6ead-46b4-8663-57609ef5a12e"),
    "Pygmy Hippo",
    crate::card::CardArt::new("2e3f6220-6ead-46b4-8663-57609ef5a12e", "Steve White"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 134 — Righteous War
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIGHTEOUS_WAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bbcacb8e-1aff-4807-b70c-a17d6703d279"),
    "Righteous War",
    crate::card::CardArt::new("bbcacb8e-1aff-4807-b70c-a17d6703d279", "Ian Miller"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 135 — Scalebane's Elite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCALEBANE_S_ELITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b3bff610-783a-46b7-bd15-061da41027bb"),
    "Scalebane's Elite",
    crate::card::CardArt::new("b3bff610-783a-46b7-bd15-061da41027bb", "Steve Luke"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 136 — Simoon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SIMOON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("642d9239-82e0-4696-ad99-10796042d1f8"),
    "Simoon",
    crate::card::CardArt::new("642d9239-82e0-4696-ad99-10796042d1f8", "Randy Gallegos"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 137 — Squandered Resources
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SQUANDERED_RESOURCES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fcddbea7-3025-47b1-a597-2d2b2711fb81"),
    "Squandered Resources",
    crate::card::CardArt::new("fcddbea7-3025-47b1-a597-2d2b2711fb81", "Romas Kukalis"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 138 — Suleiman's Legacy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SULEIMAN_S_LEGACY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3a15e970-e605-425a-b4ec-391d9cacde38"),
    "Suleiman's Legacy",
    crate::card::CardArt::new("3a15e970-e605-425a-b4ec-391d9cacde38", "Kaja Foglio"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 139 — Tempest Drake
pub(in crate::card::sets) static TEMPEST_DRAKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("54aa5262-d0d9-4b4a-8027-00393568b3df"),
    "Tempest Drake",
    CardArt::new("54aa5262-d0d9-4b4a-8027-00393568b3df", "Gerry Grace"),
    CardSet::Visions,
    // Flying and vigilance on a 2/2: it attacks and still holds the air,
    // which is more than either colour bought alone.
    CardRules::new_creature(mana_cost!("{1}{W}{U}"), &["Drake"], 2, 2)
        .with_abilities(&[abilities::flying(), abilities::vigilance()]),
);

// VIS 140 — Viashivan Dragon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIASHIVAN_DRAGON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7172ef0b-ca9e-47cf-8ec6-2d8cb18f2283"),
    "Viashivan Dragon",
    crate::card::CardArt::new("7172ef0b-ca9e-47cf-8ec6-2d8cb18f2283", "Ian Miller"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 141 — Anvil of Bogardan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANVIL_OF_BOGARDAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7ff965dd-54b4-4f21-a52f-81c0dd1e691e"),
    "Anvil of Bogardan",
    crate::card::CardArt::new("7ff965dd-54b4-4f21-a52f-81c0dd1e691e", "Roger Raupp"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 142 — Brass-Talon Chimera
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRASS_TALON_CHIMERA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("200c9655-e51c-4b63-96cf-7f3fba3ec75c"),
    "Brass-Talon Chimera",
    crate::card::CardArt::new("200c9655-e51c-4b63-96cf-7f3fba3ec75c", "Mike Dringenberg"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 143 — Diamond Kaleidoscope
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIAMOND_KALEIDOSCOPE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("548ff852-274d-4068-818d-58a883e74a5f"),
    "Diamond Kaleidoscope",
    crate::card::CardArt::new("548ff852-274d-4068-818d-58a883e74a5f", "Ron Spencer"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 144 — Dragon Mask
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRAGON_MASK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f098e329-adc8-42dd-b779-d00d9ccc3dbd"),
    "Dragon Mask",
    crate::card::CardArt::new("f098e329-adc8-42dd-b779-d00d9ccc3dbd", "Craig Hooper"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 145 — Helm of Awakening
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HELM_OF_AWAKENING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("41bba882-39b8-42db-9a01-54c6712b8019"),
    "Helm of Awakening",
    crate::card::CardArt::new("41bba882-39b8-42db-9a01-54c6712b8019", "Adam Rex"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 146 — Iron-Heart Chimera
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IRON_HEART_CHIMERA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5899a575-a97d-4850-b55c-22ad6900ba20"),
    "Iron-Heart Chimera",
    crate::card::CardArt::new("5899a575-a97d-4850-b55c-22ad6900ba20", "Mike Dringenberg"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 147 — Juju Bubble
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JUJU_BUBBLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a5fa8208-7d65-4f8f-b07e-f5c3a66e1143"),
    "Juju Bubble",
    crate::card::CardArt::new("a5fa8208-7d65-4f8f-b07e-f5c3a66e1143", "Donato Giancola"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 148 — Lead-Belly Chimera
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEAD_BELLY_CHIMERA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5d89b377-80d2-42a0-b84e-a455a72ed9fe"),
    "Lead-Belly Chimera",
    crate::card::CardArt::new("5d89b377-80d2-42a0-b84e-a455a72ed9fe", "Mike Dringenberg"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 149 — Magma Mine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGMA_MINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1aecc3df-7ce6-419c-b3d6-60fc28bfe941"),
    "Magma Mine",
    crate::card::CardArt::new("1aecc3df-7ce6-419c-b3d6-60fc28bfe941", "Ron Spencer"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 150 — Matopi Golem
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MATOPI_GOLEM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("92378d6f-89ee-49dc-8964-0e9c55daeffc"),
    "Matopi Golem",
    crate::card::CardArt::new("92378d6f-89ee-49dc-8964-0e9c55daeffc", "Tom Kyffin"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 151 — Phyrexian Marauder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_MARAUDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("29a75dc8-1c24-4063-8944-d7e71b4a5755"),
    "Phyrexian Marauder",
    crate::card::CardArt::new("29a75dc8-1c24-4063-8944-d7e71b4a5755", "David Seeley"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 152 — Phyrexian Walker
pub(in crate::card::sets) static PHYREXIAN_WALKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9f8a3979-2947-4692-8b2f-d4c07c534777"),
    "Phyrexian Walker",
    CardArt::new("9f8a3979-2947-4692-8b2f-d4c07c534777", "Bryan Talbot"),
    CardSet::Visions,
    // A free 0/3. It blocks, it is an artifact, and it is a body -- which is
    // three things that some deck somewhere wants for nothing.
    CardRules::new_artifact_creature(mana_cost!("{0}"), &["Phyrexian", "Construct"], 0, 3),
);

// VIS 153 — Sands of Time
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SANDS_OF_TIME: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a782ee95-bde4-41f4-a947-b073cc4c1e7c"),
    "Sands of Time",
    crate::card::CardArt::new("a782ee95-bde4-41f4-a947-b073cc4c1e7c", "Paul Lee"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 154 — Sisay's Ring
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SISAY_S_RING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a08becd3-ca5e-4150-8d28-52436a3eaffd"),
    "Sisay's Ring",
    crate::card::CardArt::new("a08becd3-ca5e-4150-8d28-52436a3eaffd", "Donato Giancola"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 155 — Snake Basket
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNAKE_BASKET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bfda9a16-9cdb-494a-b662-ac24e3b89d0c"),
    "Snake Basket",
    crate::card::CardArt::new("bfda9a16-9cdb-494a-b662-ac24e3b89d0c", "Roger Raupp"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 156 — Teferi's Puzzle Box
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEFERI_S_PUZZLE_BOX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1377dab4-b814-46cc-a097-24a3cf8d0f8f"),
    "Teferi's Puzzle Box",
    crate::card::CardArt::new("1377dab4-b814-46cc-a097-24a3cf8d0f8f", "Kaja Foglio"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 157 — Tin-Wing Chimera
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIN_WING_CHIMERA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3375dcc6-9399-48eb-9aa4-7b40c3686cc5"),
    "Tin-Wing Chimera",
    crate::card::CardArt::new("3375dcc6-9399-48eb-9aa4-7b40c3686cc5", "Mike Dringenberg"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 158 — Triangle of War
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRIANGLE_OF_WAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4c1d7d4d-bed7-4d28-a304-ad33f42e9831"),
    "Triangle of War",
    crate::card::CardArt::new("4c1d7d4d-bed7-4d28-a304-ad33f42e9831", "Ian Miller"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 159 — Wand of Denial
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAND_OF_DENIAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0b1c856f-6d29-4bfc-976e-7875d60abd52"),
    "Wand of Denial",
    crate::card::CardArt::new("0b1c856f-6d29-4bfc-976e-7875d60abd52", "Steve Luke"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 160 — Coral Atoll
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CORAL_ATOLL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5d7c4619-e5af-4aa0-bd3f-6bf0e1fdc1fc"),
    "Coral Atoll",
    crate::card::CardArt::new("5d7c4619-e5af-4aa0-bd3f-6bf0e1fdc1fc", "John Avon"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 161 — Dormant Volcano
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DORMANT_VOLCANO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6aa92be7-883f-42bd-8623-00eb2df28a98"),
    "Dormant Volcano",
    crate::card::CardArt::new("6aa92be7-883f-42bd-8623-00eb2df28a98", "John Avon"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 162 — Everglades
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EVERGLADES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c1f2eaf7-7f08-446b-892f-5a844f74808f"),
    "Everglades",
    crate::card::CardArt::new("c1f2eaf7-7f08-446b-892f-5a844f74808f", "Bob Eggleton"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 163 — Griffin Canyon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRIFFIN_CANYON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("705d8194-3ad0-41b7-ae32-9c0cd8cd46b9"),
    "Griffin Canyon",
    crate::card::CardArt::new("705d8194-3ad0-41b7-ae32-9c0cd8cd46b9", "Stuart Griffin"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 164 — Jungle Basin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JUNGLE_BASIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cc3146db-2f86-4728-9af1-ff651f871652"),
    "Jungle Basin",
    crate::card::CardArt::new("cc3146db-2f86-4728-9af1-ff651f871652", "John Avon"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 165 — Karoo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAROO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d786815c-53ec-483e-ad56-382778a57b1a"),
    "Karoo",
    crate::card::CardArt::new("d786815c-53ec-483e-ad56-382778a57b1a", "Zina Saunders"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 166 — Quicksand (reprint)

// VIS 167 — Undiscovered Paradise
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNDISCOVERED_PARADISE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5f6e8830-5e62-4945-8b73-60f0628d38e7"),
    "Undiscovered Paradise",
    crate::card::CardArt::new("5f6e8830-5e62-4945-8b73-60f0628d38e7", "David O'Connor"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
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
    &KNIGHT_OF_THE_MISTS,
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
    &UNDISCOVERED_PARADISE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&catalog_avr::ARCHANGEL), // VIS 1
    PrintingRecord::reprint(&catalog_rtr::INSPIRATION), // VIS 35
    PrintingRecord::reprint(&catalog_mh1::MAN_O_WAR), // VIS 37
    PrintingRecord::reprint(&catalog_wwk::QUICKSAND), // VIS 166
];
