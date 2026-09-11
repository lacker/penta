//! Weatherlight cards used by the staged Premodern deck tranche.

use super::CardRecord;
use super::PrintingRecord;
use crate::KeywordAbility;
use crate::ParentBinding;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityPredicateDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BasicLandType;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::CardNameDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::CastTimingPermissionDef;
use crate::card::CombineObjectsDef;
use crate::card::ComparisonDef;
use crate::card::CostDef;
use crate::card::CostModificationDef;
use crate::card::CostQuantityDef;
use crate::card::CounterKind;
use crate::card::CreatureTypeSetDef;
use crate::card::DamageEventMatcherDef;
use crate::card::DamageKindDef;
use crate::card::DamageRecipientMatcherDef;
use crate::card::DamageSourceMatcherDef;
use crate::card::DiscardSelectionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::HalvedValueDef;
use crate::card::ManaColor;
use crate::card::MoveObjectsDef;
use crate::card::MoveToZoneCostDef;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PayOrDef;
use crate::card::PlayActionMatcherDef;
use crate::card::PlayRestrictionDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::RevealAndClassifyCardsDef;
use crate::card::RoundingDef;
use crate::card::SacrificedAmountDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new("WTH", "weatherlight");

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// WTH 1 — Abeyance
pub(in crate::card::sets) static ABEYANCE: CardRecord = CardRecord::new(
    "Abeyance",
    "125a355d-bfcf-4125-aa6c-35e7dea6f63e",
    "Thomas Gianni",
// A counterspell that replaces itself and stops the next one too: the
    // deck holding it is buying one turn without interaction.
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Until end of turn, target player can't cast instant or sorcery spells, and that player can't activate abilities that aren't mana abilities.\nDraw a card.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::target_players(TargetIndex::PRIMARY),
                // Both halves of the same lock, applied to the same player for the same
                // turn: no instants or sorceries, and no activations but mana abilities.
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(PlayRestrictionDef::new(
                        PlayActionMatcherDef::CastSpell,
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Instant),
                            ObjectPredicateDef::HasType(CardType::Sorcery),
                        ]),
                    ))),
                    AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(PlayRestrictionDef::new(
                        PlayActionMatcherDef::ActivateNonManaAbility,
                        ObjectPredicateDef::Any,
                    ))),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// WTH 2 — Alabaster Dragon (reprint)
const ALABASTER_DRAGON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::ALABASTER_DRAGON,
    "3a2fcc23-ac09-4ada-b194-424739c9c734",
    "Bob Eggleton",
);

// WTH 3 — Alms
// Audit: unsupported — Needs an activation cost that exiles specifically the top card of a graveyard.
pub(in crate::card::sets) static ALMS: CardRecord = CardRecord::new(
    "Alms",
    "97382dd8-2754-4ca3-8ba8-d655acaf22ac",
    "Rogério Vilela",
    crate::card::CardRules::unsupported(),
);

// WTH 4 — Angelic Renewal
pub(in crate::card::sets) static ANGELIC_RENEWAL: CardRecord = CardRecord::new(
    "Angelic Renewal",
    "7dddde7d-8565-45a7-a1db-f2dea2a6a3ba",
    "Rebecca Guay",
CardRules::new_enchantment(mana_cost!("{1}{W}")).with_ability(AbilityDef::triggered(
        "Whenever a creature is put into your graveyard from the battlefield, you may sacrifice this enchantment. If you do, return that card to the battlefield.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::OwnedBy(PlayerRelation::You),
            ]),
            Some(ZoneKind::Battlefield),
            Some(ZoneKind::Graveyard),
        ),
        EffectDef::PayOr(PayOrDef::optional(
            &[crate::card::CostDef::sacrifice_permanent(
                ObjectPredicateDef::Source,
            )],
            &EffectDef::MoveToZone {
                object: EffectRecipientDef::TriggeringZoneChangeResult,
                zone: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
            },
        )),
    )),
);

// WTH 5 — Ardent Militia (reprint)
const ARDENT_MILITIA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::ARDENT_MILITIA,
    "bb212ca5-bbb5-4c83-9a7b-9d5ab451e032",
    "Zina Saunders",
);

// WTH 6 — Argivian Find
pub(in crate::card::sets) static ARGIVIAN_FIND: CardRecord = CardRecord::new(
    "Argivian Find",
    "89f23295-ad0a-4e2d-ae04-1a9c065e575d",
    "Roger Raupp",
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Return target artifact or enchantment card from your graveyard to your hand.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                ]),
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

// WTH 7 — Aura of Silence
/// The tax names spells an opponent casts, so it never touches your own.
static OPPONENTS_ARTIFACTS_AND_ENCHANTMENTS: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasType(CardType::Artifact),
    ObjectPredicateDef::HasType(CardType::Enchantment),
]);

pub(in crate::card::sets) static AURA_OF_SILENCE: CardRecord = CardRecord::new(
    "Aura of Silence",
    "57e6c366-b8c7-4f66-b8e1-82dc69c0081c",
    "D. Alexander Gregory",
    // It taxes while it sits and answers something on the way out, so the
    // opponent pays either way.
    CardRules::new_enchantment(mana_cost!("{1}{W}{W}")).with_abilities(&[
        AbilityDef::static_ability(
            "Artifact and enchantment spells your opponents cast cost {2} more to cast.",
            EffectDef::ModifyCost(CostModificationDef::increase_spell(
                OPPONENTS_ARTIFACTS_AND_ENCHANTMENTS,
                PlayerRelation::Opponent,
                mana_cost!("{2}"),
            )),
        ),
        AbilityDef::activated_with_targets(
            "Sacrifice this enchantment: Destroy target artifact or enchantment.",
            &[CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                OPPONENTS_ARTIFACTS_AND_ENCHANTMENTS,
            )],
            EffectDef::destroy_target(TargetIndex::PRIMARY),
        ),
    ]),
);

// WTH 8 — Benalish Infantry
pub(in crate::card::sets) static BENALISH_INFANTRY: CardRecord = CardRecord::new(
    "Benalish Infantry",
    "e8472303-b8ee-402b-a9ea-49abe2e01152",
    "Dan Frazier",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 1, 3)
        .with_ability(abilities::banding()),
);

// WTH 9 — Benalish Knight
pub(in crate::card::sets) static BENALISH_KNIGHT: CardRecord = CardRecord::new(
    "Benalish Knight",
    "c2c184bb-6c7d-4118-a111-ef27171cfee6",
    "Zina Saunders",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Knight"], 2, 2)
        .with_abilities(&[abilities::flash(), abilities::first_strike()]),
);

// WTH 10 — Benalish Missionary
// Audit: unsupported — Needs a turn-long combat-damage prevention shield keyed to one targeted blocked creature.
pub(in crate::card::sets) static BENALISH_MISSIONARY: CardRecord = CardRecord::new(
    "Benalish Missionary",
    "e9ac1992-6212-4f05-af16-c892dfc40643",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// WTH 11 — Debt of Loyalty
// Audit: unsupported — Needs to gain control only when this spell's regeneration shield actually replaces destruction.
pub(in crate::card::sets) static DEBT_OF_LOYALTY: CardRecord = CardRecord::new(
    "Debt of Loyalty",
    "d19ed33b-42d4-4a5d-a763-cfb43348769c",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// WTH 12 — Duskrider Falcon
pub(in crate::card::sets) static DUSKRIDER_FALCON: CardRecord = CardRecord::new(
    "Duskrider Falcon",
    "bee3a23a-6ecf-439c-8637-e096fa8c1a80",
    "Cecil Fernando",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Bird"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::protection_from_color(ManaColor::Black),
    ]),
);

// WTH 13 — Empyrial Armor
pub(in crate::card::sets) static EMPYRIAL_ARMOR: CardRecord = CardRecord::new(
    "Empyrial Armor",
    "5518a79f-bcae-417a-b01b-b6ff572be0be",
    "D. Alexander Gregory",
    CardRules::new_enchantment(mana_cost!("{1}{W}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+1 for each card in your hand.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::CardsInHandAbove {
                            player: PlayerRelation::You,
                            threshold: 0,
                        },
                        ValueDef::CardsInHandAbove {
                            player: PlayerRelation::You,
                            threshold: 0,
                        },
                    ),
                },
            ),
        ]),
);

// WTH 14 — Foriysian Brigade
pub(in crate::card::sets) static FORIYSIAN_BRIGADE: CardRecord = CardRecord::new(
    "Foriysian Brigade",
    "0d11b6ef-3a24-4709-a62f-c5e062a6cee1",
    "Kev Walker",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Soldier"], 2, 4).with_ability(
        AbilityDef::static_ability(
            "This creature can block an additional creature each combat.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayBlockAdditionalCreatures(1)),
            },
        ),
    ),
);

// WTH 15 — Gerrard's Wisdom
pub(in crate::card::sets) static GERRARD_S_WISDOM: CardRecord = CardRecord::new(
    "Gerrard's Wisdom",
    "f81defa5-edb4-4f1f-b13c-7cfb34511138",
    "Heather Hudson",
    CardRules::new_sorcery(mana_cost!("{2}{W}{W}")).with_ability(AbilityDef::spell(
        "You gain 2 life for each card in your hand.",
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Scaled(&crate::card::ScaledValueDef::new(
                ValueDef::CardsInHandAbove {
                    player: PlayerRelation::You,
                    threshold: 0,
                },
                2,
            )),
        },
    )),
);

// WTH 16 — Guided Strike
pub(in crate::card::sets) static GUIDED_STRIKE: CardRecord = CardRecord::new(
    "Guided Strike",
    "c6e8ec37-abe8-45a9-a1a0-6d4e37c74c45",
    "Gary Leach",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +1/+0 and gains first strike until end of turn.\nDraw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
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
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// WTH 17 — Heavy Ballista
pub(in crate::card::sets) static HEAVY_BALLISTA: CardRecord = CardRecord::new(
    "Heavy Ballista",
    "bdfe3eed-e415-4b28-8b4d-e50a19235683",
    "Ron Spencer",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Soldier"], 2, 3).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: This creature deals 2 damage to target attacking or blocking creature.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AttackingOrBlocking,
                ]),
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(2),
            ),
        ),
    ),
);

// WTH 18 — Inner Sanctum
pub(in crate::card::sets) static INNER_SANCTUM: CardRecord = CardRecord::new(
    "Inner Sanctum",
    "2298faae-370e-4b87-bf32-d20c2282a928",
    "D. Alexander Gregory",
    CardRules::new_enchantment(mana_cost!("{1}{W}{W}")).with_abilities(&[
        abilities::cumulative_upkeep(&[CostDef::life(2)]),
        AbilityDef::static_ability(
            "Prevent all damage that would be dealt to creatures you control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::PreventDamage(
                    DamageEventMatcherDef {
                        kind: DamageKindDef::Any,
                        source: DamageSourceMatcherDef::Any,
                        recipient: DamageRecipientMatcherDef::AffectedObject,
                    },
                )),
            },
        ),
    ]),
);

// WTH 19 — Kithkin Armor
// Audit: unsupported — Needs a power-based blocking restriction and a chosen-source prevention shield for the attached creature.
pub(in crate::card::sets) static KITHKIN_ARMOR: CardRecord = CardRecord::new(
    "Kithkin Armor",
    "395e7882-0429-46aa-8e38-be707067c588",
    "Charles Gillespie",
    crate::card::CardRules::unsupported(),
);

// WTH 20 — Master of Arms
pub(in crate::card::sets) static MASTER_OF_ARMS: CardRecord = CardRecord::new(
    "Master of Arms",
    "ac97ff43-c0b6-4f67-ad09-5ba8710c681a",
    "Dan Frazier",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 2, 2).with_abilities(&[
        abilities::first_strike(),
        AbilityDef::activated_with_targets(
            "{1}{W}: Tap target creature blocking this creature.",
            &[CostDef::Mana(mana_cost!("{1}{W}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::BlockingSource,
                ]),
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// WTH 21 — Mistmoon Griffin
pub(in crate::card::sets) static MISTMOON_GRIFFIN: CardRecord = CardRecord::new(
    "Mistmoon Griffin",
    "8ec71a29-19db-4747-8276-7fd4d563d4df",
    "David A. Cherry",
CardRules::new_creature(mana_cost!("{3}{W}"), &["Griffin"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::dies_trigger(
            "When this creature dies, exile it, then return the top creature card of your graveyard to the battlefield.",
            EffectDef::Sequence(&[
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::TriggeringZoneChangeResult,
                    zone: ZoneKind::Exile,
                    placement: ZonePlacement::Top,
                },
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::objects(ObjectSetDef::TopOfGraveyardMatching {
                        player: PlayerRefDef::EffectController,
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                    }),
                    zone: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                },
            ]),
        ),
    ]),
);

// WTH 22 — Peacekeeper
pub(in crate::card::sets) static PEACEKEEPER: CardRecord = CardRecord::new(
    "Peacekeeper",
    "592a5683-5f2f-4933-9fc3-5f7773f72f93",
    "Donato Giancola",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human"], 1, 1).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, sacrifice this creature unless you pay {1}{W}.",
            TriggerEventDef::StepBegins {
                step: crate::card::TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::PayOr(PayOrDef::unless(
                &[CostDef::Mana(mana_cost!("{1}{W}"))],
                &EffectDef::sacrifice(EffectRecipientDef::Source),
            )),
        ),
        AbilityDef::static_ability(
            "Creatures can't attack.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::new(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                ))),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
            },
        ),
    ]),
);

// WTH 23 — Revered Unicorn
// Audit: unsupported — Leave-the-battlefield triggers cannot yet read the source's last-known counter count.
pub(in crate::card::sets) static REVERED_UNICORN: CardRecord = CardRecord::new(
    "Revered Unicorn",
    "8c642dd2-1a3e-4b08-917e-6e8aed358b72",
    "David A. Cherry",
    crate::card::CardRules::unsupported(),
);

// WTH 24 — Serenity
pub(in crate::card::sets) static SERENITY: CardRecord = CardRecord::new(
    "Serenity",
    "dca975ab-b3ee-4584-9f92-860b4c2369f3",
    "Cliff Nielsen",
CardRules::new_enchantment(mana_cost!("{1}{W}")).with_ability(AbilityDef::triggered(
        "At the beginning of your upkeep, destroy all artifacts and enchantments. They can't be regenerated.",
        TriggerEventDef::StepBegins {
            step: crate::card::TurnStepDef::Upkeep,
            player: PlayerRelation::You,
        },
        EffectDef::WithRule {
            rule: AppliedRuleDef::CannotRegenerate,
            effect: &EffectDef::Destroy {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                then: None,
            },
        },
    )),
);

// WTH 25 — Serra's Blessing
pub(in crate::card::sets) static SERRA_S_BLESSING: CardRecord = CardRecord::new(
    "Serra's Blessing",
    "2794cca9-3df0-4864-8a98-4de71a2bcf17",
    "Rebecca Guay",
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_ability(AbilityDef::static_ability(
        "Creatures you control have vigilance.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::add_ability(&abilities::vigilance()),
        },
    )),
);

// WTH 26 — Soul Shepherd
pub(in crate::card::sets) static SOUL_SHEPHERD: CardRecord = CardRecord::new(
    "Soul Shepherd",
    "f45a39ba-5fbf-46c3-8dc7-3058ac6d24e8",
    "John Coulthart",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Cleric"], 2, 1).with_ability(
        AbilityDef::activated(
            "{W}, Exile a creature card from your graveyard: You gain 1 life.",
            &[
                CostDef::Mana(mana_cost!("{W}")),
                CostDef::MoveToZone(MoveToZoneCostDef::new(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ZoneKind::Graveyard,
                    ZoneKind::Exile,
                    1,
                )),
            ],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// WTH 27 — Southern Paladin
pub(in crate::card::sets) static SOUTHERN_PALADIN: CardRecord = CardRecord::new(
    "Southern Paladin",
    "2a3c94a1-8455-4521-a0d5-ee2982527b89",
    "Douglas Shuler",
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Human", "Knight"], 3, 3).with_ability(
        AbilityDef::activated_with_targets(
            "{W}{W}, {T}: Destroy target red permanent.",
            &[CostDef::Mana(mana_cost!("{W}{W}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Color(ManaColor::Red),
            )],
            EffectDef::destroy_target(TargetIndex::PRIMARY),
        ),
    ),
);

// WTH 28 — Tariff
// Audit: unsupported — Needs each player to choose a greatest-mana-value creature, then pay that object's full mana cost or sacrifice it.
pub(in crate::card::sets) static TARIFF: CardRecord = CardRecord::new(
    "Tariff",
    "24333832-2a87-4810-9443-ec993468d103",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// WTH 29 — Volunteer Reserves
pub(in crate::card::sets) static VOLUNTEER_RESERVES: CardRecord = CardRecord::new(
    "Volunteer Reserves",
    "5344911f-25e8-45ce-87b9-607e42db0139",
    "Kev Walker",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Soldier"], 2, 4).with_abilities(&[
        abilities::banding(),
        abilities::cumulative_upkeep(&[CostDef::mana(mana_cost!("{1}"))]),
    ]),
);

// WTH 30 — Abduction
// Audit: unsupported — Needs an Aura-host death trigger that returns that exact zone-change successor under its owner's control.
pub(in crate::card::sets) static ABDUCTION: CardRecord = CardRecord::new(
    "Abduction",
    "ac81264d-0e03-44ac-8ff5-049b9aaebcca",
    "Colin MacNeil",
    crate::card::CardRules::unsupported(),
);

// WTH 31 — Abjure
pub(in crate::card::sets) static ABJURE: CardRecord = CardRecord::new(
    "Abjure",
    "fbad9449-d09c-4fd0-b2ad-2aa3a29e03bf",
    "Ted Naifeh",
CardRules::new_instant(mana_cost!("{U}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, sacrifice a blue permanent.\nCounter target spell.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            })],
            CostDef::sacrifice(
                ObjectPredicateDef::Color(ManaColor::Blue),
                CostQuantityDef::Fixed(1),
            ),
            EffectDef::counter_target(TargetIndex::PRIMARY),
        ),
    ),
);

// WTH 32 — Ancestral Knowledge
// Audit: unsupported — Needs ordered top-ten inspection with a chosen subset exiled and the remainder reordered on top.
pub(in crate::card::sets) static ANCESTRAL_KNOWLEDGE: CardRecord = CardRecord::new(
    "Ancestral Knowledge",
    "05b90d72-00ac-4423-8cdf-e1471c6cd0ae",
    "Colin MacNeil",
    crate::card::CardRules::unsupported(),
);

// WTH 33 — Apathy
// Audit: unsupported — Needs random discard as an optional effect payment by the attached creature's controller.
pub(in crate::card::sets) static APATHY: CardRecord = CardRecord::new(
    "Apathy",
    "adf3a6fe-e234-4c3f-96fc-3eb5eb22c0b8",
    "Phil Foglio",
    crate::card::CardRules::unsupported(),
);

// WTH 34 — Argivian Restoration
pub(in crate::card::sets) static ARGIVIAN_RESTORATION: CardRecord = CardRecord::new(
    "Argivian Restoration",
    "9f1a9d35-1b2a-44a2-9bbc-8529a7487905",
    "Roger Raupp",
    CardRules::new_sorcery(mana_cost!("{2}{U}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return target artifact card from your graveyard to the battlefield.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Artifact),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            },
        )],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
        },
    )),
);

// WTH 35 — Avizoa
// Audit: unsupported — Needs a consumable effect that skips the controller's next untap step.
pub(in crate::card::sets) static AVIZOA: CardRecord = CardRecord::new(
    "Avizoa",
    "a993986c-e8f1-41b1-86e6-c72021c53b87",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// WTH 36 — Cloud Djinn
pub(in crate::card::sets) static CLOUD_DJINN: CardRecord = CardRecord::new(
    "Cloud Djinn",
    "c857a151-45fe-43af-a9be-a93d26f220f3",
    "Mike Dringenberg",
    CardRules::new_creature(mana_cost!("{5}{U}"), &["Djinn"], 5, 4).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "This creature can block only creatures with flying.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::can_block_only(
                    ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Flying),
                )),
            },
        ),
    ]),
);

// WTH 37 — Disrupt
pub(in crate::card::sets) static DISRUPT: CardRecord = CardRecord::new(
    "Disrupt",
    "c6cc89b0-9acf-452b-ac1a-bc7e90eb32fc",
    "Adam Rex",
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target instant or sorcery spell unless its controller pays {1}.\nDraw a card.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Spell,
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                ]),
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            abilities::counter_target_unless_paid(&[CostDef::GenericMana(ValueDef::Constant(1))]),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// WTH 38 — Ertai's Familiar
// Audit: unsupported — Needs phasing, phase-transition triggers, and a turn-bounded effect that suppresses the next phase-out.
pub(in crate::card::sets) static ERTAI_S_FAMILIAR: CardRecord = CardRecord::new(
    "Ertai's Familiar",
    "354c9de7-0cdf-4302-9d1a-ae17eca13053",
    "Kipling West",
    crate::card::CardRules::unsupported(),
);

// WTH 39 — Flux (reprint)
const FLUX_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::FLUX,
    "368b28e4-a367-4a38-866d-c3768bd9b7ad",
    "Richard Kane Ferguson",
);

// WTH 40 — Fog Elemental
// Audit: unsupported — Needs a delayed end-of-combat sacrifice created by attacking or blocking.
pub(in crate::card::sets) static FOG_ELEMENTAL: CardRecord = CardRecord::new(
    "Fog Elemental",
    "28b454d0-7dc7-419f-aefa-f20f37444658",
    "Jon J Muth",
    crate::card::CardRules::unsupported(),
);

// WTH 41 — Mana Chains
pub(in crate::card::sets) static MANA_CHAINS: CardRecord = CardRecord::new(
    "Mana Chains",
    "77802038-0d86-4911-97ed-e6bd2ed55e23",
    "Bryan Talbot",
CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature has \"Cumulative upkeep {1}.\" (At the beginning of its controller's upkeep, that player puts an age counter on it, then sacrifices it unless they pay its upkeep cost for each age counter on it.)",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(
                        &abilities::cumulative_upkeep(
                            &[CostDef::mana(mana_cost!(
                                "{1}"
                            ))],
                        )
                        .override_text("Cumulative upkeep {1}."),
                    ),
                },
            ),
        ]),
);

// WTH 42 — Manta Ray
// Audit: unsupported — Needs Island-dependent attack permission, blue-only blocking, and the no-Island state trigger.
pub(in crate::card::sets) static MANTA_RAY: CardRecord = CardRecord::new(
    "Manta Ray",
    "80f74884-9b82-419d-9e97-c947a6b7d09f",
    "Una Fricker",
    crate::card::CardRules::unsupported(),
);

// WTH 43 — Merfolk Traders
pub(in crate::card::sets) static MERFOLK_TRADERS: CardRecord = CardRecord::new(
    "Merfolk Traders",
    "ebacbf23-4b69-481c-aaf7-5de7b4a6db6f",
    "DiTerlizzi",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Merfolk"], 1, 2).with_ability(
        abilities::enters_trigger(
            "When this creature enters, draw a card, then discard a card.",
            EffectDef::Sequence(&[
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            ]),
        ),
    ),
);

// WTH 44 — Noble Benefactor
// Audit: unsupported — Needs APNAP optional library searches with independent searched-player tracking and shuffles.
pub(in crate::card::sets) static NOBLE_BENEFACTOR: CardRecord = CardRecord::new(
    "Noble Benefactor",
    "bd221f30-1773-4e05-a40f-022a9306ef89",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// WTH 45 — Ophidian
// Audit: unsupported — Needs an unblocked-attacker choice that suppresses this creature's combat-damage assignment when accepted.
pub(in crate::card::sets) static OPHIDIAN: CardRecord = CardRecord::new(
    "Ophidian",
    "0de0a010-76a7-460f-bb4e-a152c10c3bb7",
    "Cliff Nielsen",
    crate::card::CardRules::unsupported(),
);

// WTH 46 — Paradigm Shift
pub(in crate::card::sets) static PARADIGM_SHIFT: CardRecord = CardRecord::new(
    "Paradigm Shift",
    "e64a17a8-091d-4029-908e-31d6a050b479",
    "Cliff Nielsen",
    CardRules::new_sorcery(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell(
        "Exile all cards from your library. Then shuffle your graveyard into your library.",
        EffectDef::Sequence(&[
            EffectDef::MoveToZone {
                object: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::owned_by(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Library],
                    PlayerSetDef::Related(PlayerRelation::You),
                ))),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
            EffectDef::MoveToZone {
                object: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::owned_by(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Graveyard],
                    PlayerSetDef::Related(PlayerRelation::You),
                ))),
                zone: ZoneKind::Library,
                placement: ZonePlacement::Top,
            },
            EffectDef::ShuffleLibrary {
                player: EffectRecipientDef::Controller,
            },
        ]),
    )),
);

// WTH 47 — Pendrell Mists
pub(in crate::card::sets) static PENDRELL_MISTS: CardRecord = CardRecord::new(
    "Pendrell Mists",
    "b902b972-3a93-4e4e-aa77-02ada81e6b95",
    "Andrew Robinson",
CardRules::new_enchantment(mana_cost!("{3}{U}")).with_ability(AbilityDef::static_ability(
        "All creatures have \"At the beginning of your upkeep, sacrifice this creature unless you pay {1}.\"",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::add_ability(&AbilityDef::triggered(
                "At the beginning of your upkeep, sacrifice this creature unless you pay {1}.",
                TriggerEventDef::StepBegins {
                    step: crate::card::TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::PayOr(PayOrDef::unless(
                    &[CostDef::Mana(mana_cost!("{1}"))],
                    &EffectDef::sacrifice(EffectRecipientDef::Source),
                )),
            )),
        },
    )),
);

// WTH 48 — Phantom Warrior (reprint)
const PHANTOM_WARRIOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::PHANTOM_WARRIOR,
    "b414c9f8-ee46-4368-a8dc-0767c645a9c1",
    "John Matson",
);

// WTH 49 — Phantom Wings
pub(in crate::card::sets) static PHANTOM_WINGS: CardRecord = CardRecord::new(
    "Phantom Wings",
    "a0db4c6c-aa51-487b-a591-78d93c67c775",
    "Una Fricker",
    CardRules::new_enchantment(mana_cost!("{1}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature has flying.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::flying()),
                },
            ),
            AbilityDef::activated(
                "Sacrifice this Aura: Return enchanted creature to its owner's hand.",
                &[CostDef::SacrificeSource],
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::AttachedPermanent,
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
            ),
        ]),
);

// WTH 50 — Psychic Vortex
pub(in crate::card::sets) static PSYCHIC_VORTEX: CardRecord = CardRecord::new(
    "Psychic Vortex",
    "3bc2a419-7122-4eeb-bb64-738a647cfd82",
    "Steve Luke",
    CardRules::new_enchantment(mana_cost!("{2}{U}{U}")).with_abilities(&[
        abilities::cumulative_upkeep(&[CostDef::draw_cards(1)]),
        AbilityDef::triggered(
            "At the beginning of your end step, sacrifice a land and discard your hand.",
            TriggerEventDef::StepBegins {
                step: crate::card::TurnStepDef::End,
                player: PlayerRelation::You,
            },
            EffectDef::Sequence(&[
                EffectDef::SacrificeOfChoice {
                    player: EffectRecipientDef::Controller,
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    count: ValueDef::Constant(1),
                    then: None,
                    amount: SacrificedAmountDef::Power,
                    otherwise: None,
                    optional: false,
                },
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::CardsInHandAbove {
                        player: PlayerRelation::You,
                        threshold: 0,
                    },
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            ]),
        ),
    ]),
);

// WTH 51 — Relearn
pub(in crate::card::sets) static RELEARN: CardRecord = CardRecord::new(
    "Relearn",
    "902f8480-8ae7-4b5f-abdf-1bd46066049e",
    "Zina Saunders",
    CardRules::new_sorcery(mana_cost!("{1}{U}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return target instant or sorcery card from your graveyard to your hand.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
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

// WTH 52 — Sage Owl
pub(in crate::card::sets) static SAGE_OWL: CardRecord = CardRecord::new(
    "Sage Owl",
    "3ee2d6a1-8b1e-47e5-9720-5683ac458250",
    "Mark Poole",
    // Two mana to arrange the next four draws, on a body that also blocks a
    // flier. Small, but it makes every subsequent draw a known quantity.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Bird"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, look at the top four cards of your library, then \
             put them back in any order.",
            abilities::look_at_top_cards_and_reorder(
                PlayerRefDef::EffectController,
                ValueDef::Constant(4),
            ),
        ),
    ]),
);

// WTH 53 — Teferi's Veil
// Audit: unsupported — Needs phasing plus a delayed end-of-combat phase-out for each attacking creature.
pub(in crate::card::sets) static TEFERI_S_VEIL: CardRecord = CardRecord::new(
    "Teferi's Veil",
    "cbf39b80-d972-4f79-902f-cc613c32e446",
    "Brom",
    crate::card::CardRules::unsupported(),
);

// WTH 54 — Timid Drake
pub(in crate::card::sets) static TIMID_DRAKE: CardRecord = CardRecord::new(
    "Timid Drake",
    "01bbdbd8-1517-4bfd-926b-465a32724082",
    "Mike Dringenberg",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Drake"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "When another creature enters, return this creature to its owner's hand.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Source,
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// WTH 55 — Tolarian Drake
// Audit: unsupported — Phasing is not represented by the shared keyword/runtime vocabulary.
pub(in crate::card::sets) static TOLARIAN_DRAKE: CardRecord = CardRecord::new(
    "Tolarian Drake",
    "e04bba8a-48ee-4981-adc2-4f82c0f2c1bd",
    "Mark Harrison",
    crate::card::CardRules::unsupported(),
);

// WTH 56 — Tolarian Entrancer
// Audit: unsupported — Needs to bind each individual blocking creature and gain control of it at end of combat.
pub(in crate::card::sets) static TOLARIAN_ENTRANCER: CardRecord = CardRecord::new(
    "Tolarian Entrancer",
    "c29dd04a-b3aa-48b6-beef-3314344b84a6",
    "Bryan Talbot",
    crate::card::CardRules::unsupported(),
);

// WTH 57 — Tolarian Serpent
pub(in crate::card::sets) static TOLARIAN_SERPENT: CardRecord = CardRecord::new(
    "Tolarian Serpent",
    "9236a857-c4ca-4de2-a4a2-e0914d16b54b",
    "Stuart Griffin",
    CardRules::new_creature(mana_cost!("{5}{U}{U}"), &["Serpent"], 7, 7).with_ability(
        AbilityDef::triggered(
            "At the beginning of your upkeep, mill seven cards.",
            TriggerEventDef::StepBegins {
                step: crate::card::TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::Mill {
                player: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(7),
            },
        ),
    ),
);

// WTH 58 — Vodalian Illusionist
// Audit: unsupported — Needs a phase-out effect and phasing runtime support.
pub(in crate::card::sets) static VODALIAN_ILLUSIONIST: CardRecord = CardRecord::new(
    "Vodalian Illusionist",
    "9ce0e28b-9fd6-4763-8d6b-952b530358ab",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// WTH 59 — Abyssal Gatekeeper
pub(in crate::card::sets) static ABYSSAL_GATEKEEPER: CardRecord = CardRecord::new(
    "Abyssal Gatekeeper",
    "1734df5a-7d3a-46c7-a0ad-adbbd1be958f",
    "Mark Tedin",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Horror"], 1, 1).with_ability(
        abilities::dies_trigger(
            "When this creature dies, each player sacrifices a creature of their choice.",
            EffectDef::SacrificeOfChoice {
                player: EffectRecipientDef::EachPlayer,
                object: ObjectPredicateDef::HasType(CardType::Creature),
                count: ValueDef::Constant(1),
                then: None,
                amount: SacrificedAmountDef::Power,
                otherwise: None,
                optional: false,
            },
        ),
    ),
);

// WTH 60 — Agonizing Memories
// Audit: unsupported — Needs hidden-hand selection of two cards followed by an ordered move to the top of that player's library.
pub(in crate::card::sets) static AGONIZING_MEMORIES: CardRecord = CardRecord::new(
    "Agonizing Memories",
    "be277367-a58e-429e-af1b-58163becf861",
    "Mike Dringenberg",
    crate::card::CardRules::unsupported(),
);

// WTH 61 — Barrow Ghoul
// Audit: unsupported — Needs an upkeep payment that exiles specifically the top creature card of a graveyard.
pub(in crate::card::sets) static BARROW_GHOUL: CardRecord = CardRecord::new(
    "Barrow Ghoul",
    "f7055007-83dd-40fe-b2a1-4b3132f636db",
    "Bryan Talbot",
    crate::card::CardRules::unsupported(),
);

// WTH 62 — Bone Dancer
// Audit: unsupported — Needs an unblocked-attacker choice, defending-player graveyard top-card lookup, and combat-damage suppression.
pub(in crate::card::sets) static BONE_DANCER: CardRecord = CardRecord::new(
    "Bone Dancer",
    "207bb4cd-4525-47e0-b412-0d0e29717d44",
    "Scott Kirschner",
    crate::card::CardRules::unsupported(),
);

// WTH 63 — Buried Alive
pub(in crate::card::sets) static BURIED_ALIVE: CardRecord = CardRecord::new(
    "Buried Alive",
    "56b92eb5-72b0-46b4-8b16-8a7a7ac80f56",
    "Brian Horton",
CardRules::new_sorcery(mana_cost!("{2}{B}")).with_ability(AbilityDef::spell(
        "Search your library for up to three creature cards, put them into your graveyard, then shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::HasType(CardType::Creature),
            minimum: 0,
            maximum: ValueDef::Constant(3),
            reveal: false,
            destination: ZoneKind::Graveyard,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            attachment: None,
            binding: None,
            then: None,
        },
    )),
);

// WTH 64 — Circling Vultures
// Audit: unsupported — Needs a hand-zone special action and an upkeep payment that exiles the top creature card of a graveyard.
pub(in crate::card::sets) static CIRCLING_VULTURES: CardRecord = CardRecord::new(
    "Circling Vultures",
    "8dae8e49-c2b6-4965-9249-49f93449d271",
    "Una Fricker",
    crate::card::CardRules::unsupported(),
);

// WTH 65 — Coils of the Medusa
// Audit: unsupported — Needs the set of non-Wall creatures currently blocking the attached creature.
pub(in crate::card::sets) static COILS_OF_THE_MEDUSA: CardRecord = CardRecord::new(
    "Coils of the Medusa",
    "502bfb38-4a37-4053-af20-d5606ffc67c8",
    "Darbury Stenderu",
    crate::card::CardRules::unsupported(),
);

// WTH 66 — Doomsday
pub(in crate::card::sets) static DOOMSDAY: CardRecord = CardRecord::new(
    "Doomsday",
    "5b3c6d87-9383-450b-bba5-33435b6b0d08",
    "Adrian Smith",
// A five-card library you built yourself, and half your life for it. The
    // deck that plays it is not trying to survive the exile -- it is trying
    // to draw the five cards it just stacked and win on the spot.
    CardRules::new_sorcery(mana_cost!("{B}{B}{B}")).with_ability(AbilityDef::spell(
        "Search your library and graveyard for five cards and exile the rest. Put the chosen cards on top of your library in any order. You lose half your life, rounded up.",
        // The search and the life are one clause resolving in order, and the order
        // matters: the five cards are chosen while the library still exists.
        EffectDef::Sequence(&[
            EffectDef::SearchZonesAndExileRest {
                player: EffectRecipientDef::Controller,
                zones: &[ZoneKind::Library, ZoneKind::Graveyard],
                count: 5,
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                // Half the life you have, rounded up: at twenty that is ten, and the deck
                // casting this intends to win before losing the other ten.
                amount: ValueDef::Halved(&HalvedValueDef::new(ValueDef::LifeTotal(PlayerRelation::You), RoundingDef::Up)),
            },
        ]),
    )),
);

// WTH 67 — Fatal Blow
pub(in crate::card::sets) static FATAL_BLOW: CardRecord = CardRecord::new(
    "Fatal Blow",
    "044dc7c2-6198-4526-b79a-f3d8ee7a157a",
    "George Pratt",
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target creature that was dealt damage this turn. It can't be regenerated.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::WasDealtDamageThisTurn,
            ]),
        )],
        EffectDef::WithRule {
            rule: AppliedRuleDef::CannotRegenerate,
            effect: &EffectDef::destroy_target(TargetIndex::PRIMARY),
        },
    )),
);

// WTH 68 — Festering Evil
pub(in crate::card::sets) static FESTERING_EVIL: CardRecord = CardRecord::new(
    "Festering Evil",
    "2d688bda-fee2-496d-9793-794c2568b54e",
    "John Matson",
CardRules::new_enchantment(mana_cost!("{3}{B}{B}")).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, this enchantment deals 1 damage to each creature and each player.",
            TriggerEventDef::StepBegins {
                step: crate::card::TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::Sequence(&[
                EffectDef::damage(
                    EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    ValueDef::Constant(1),
                ),
                EffectDef::damage(EffectRecipientDef::EachPlayer, ValueDef::Constant(1)),
            ]),
        ),
        AbilityDef::activated(
            "{B}{B}, Sacrifice this enchantment: It deals 3 damage to each creature and each player.",
            &[
                CostDef::Mana(mana_cost!("{B}{B}")),
                CostDef::SacrificeSource,
            ],
            EffectDef::Sequence(&[
                EffectDef::damage(
                    EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    ValueDef::Constant(3),
                ),
                EffectDef::damage(EffectRecipientDef::EachPlayer, ValueDef::Constant(3)),
            ]),
        ),
    ]),
);

// WTH 69 — Fledgling Djinn
pub(in crate::card::sets) static FLEDGLING_DJINN: CardRecord = CardRecord::new(
    "Fledgling Djinn",
    "1b0fdf2a-d6d2-42da-8f41-0f67dd0bf4d2",
    "Thomas Gianni",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Djinn"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "At the beginning of your upkeep, this creature deals 1 damage to you.",
            TriggerEventDef::StepBegins {
                step: crate::card::TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::damage(EffectRecipientDef::Controller, ValueDef::Constant(1)),
        ),
    ]),
);

// WTH 70 — Gallowbraid
pub(in crate::card::sets) static GALLOWBRAID: CardRecord = CardRecord::new(
    "Gallowbraid",
    "8df86192-6374-42ac-94bc-95e2e8284bd6",
    "Carl Critchlow",
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Phyrexian", "Horror"], 5, 5)
        .with_supertype(crate::card::CardSupertype::Legendary)
        .with_abilities(&[
            abilities::trample(),
            abilities::cumulative_upkeep(&[CostDef::life(1)]),
        ]),
);

// WTH 71 — Haunting Misery
pub(in crate::card::sets) static HAUNTING_MISERY: CardRecord = CardRecord::new(
    "Haunting Misery",
    "939b83ba-8ba8-4b98-8a13-a037ba7805e9",
    "Gary Leach",
CardRules::new_sorcery(mana_cost!("{1}{B}{B}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, exile X creature cards from your graveyard.\nThis spell deals X damage to target player or planeswalker.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )],
            CostDef::exile(
                ObjectPredicateDef::HasType(CardType::Creature),
                ZoneKind::Graveyard,
                CostQuantityDef::ChosenX,
            ),
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::ChosenX,
            ),
        ),
    ),
);

// WTH 72 — Hidden Horror
pub(in crate::card::sets) static HIDDEN_HORROR: CardRecord = CardRecord::new(
    "Hidden Horror",
    "885dc4c5-2ade-4497-b579-0307c67ac783",
    "Clint Langley",
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Horror"], 4, 4).with_ability(
        abilities::enters_trigger(
            "When this creature enters, sacrifice it unless you discard a creature card.",
            EffectDef::PayOr(PayOrDef::unless(
                &[crate::card::CostDef::discard(ObjectPredicateDef::HasType(
                    CardType::Creature,
                ))],
                &EffectDef::sacrifice(EffectRecipientDef::Source),
            )),
        ),
    ),
);

// WTH 73 — Infernal Tribute
pub(in crate::card::sets) static INFERNAL_TRIBUTE: CardRecord = CardRecord::new(
    "Infernal Tribute",
    "569739b2-f212-4cc9-84db-1be17b3f90fb",
    "Terese Nielsen",
    CardRules::new_enchantment(mana_cost!("{B}{B}{B}")).with_ability(AbilityDef::activated(
        "{2}, Sacrifice a nontoken permanent: Draw a card.",
        &[
            CostDef::Mana(mana_cost!("{2}")),
            CostDef::SacrificePermanent {
                object: ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                controller: PlayerRelation::You,
            },
        ],
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )),
);

// WTH 74 — Mischievous Poltergeist
pub(in crate::card::sets) static MISCHIEVOUS_POLTERGEIST: CardRecord = CardRecord::new(
    "Mischievous Poltergeist",
    "054254ee-29cf-48d7-afbf-cb6de83e513e",
    "DiTerlizzi",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Spirit"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::regenerate_self(
            "Pay 1 life: Regenerate this creature.",
            &[CostDef::PayLife(1)],
        ),
    ]),
);

// WTH 75 — Morinfen
pub(in crate::card::sets) static MORINFEN: CardRecord = CardRecord::new(
    "Morinfen",
    "b5006ad3-16ca-4be3-8d56-d4fe4e9e0a44",
    "Carl Critchlow",
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Phyrexian", "Horror"], 5, 4)
        .with_supertype(crate::card::CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            abilities::cumulative_upkeep(&[CostDef::life(1)]),
        ]),
);

// WTH 76 — Necratog
// Audit: unsupported — Needs an activation cost that exiles specifically the top creature card of a graveyard.
pub(in crate::card::sets) static NECRATOG: CardRecord = CardRecord::new(
    "Necratog",
    "fb19c519-c09a-44a0-8d4b-ab6c15dabdef",
    "Bryan Talbot",
    crate::card::CardRules::unsupported(),
);

// WTH 77 — Odylic Wraith
pub(in crate::card::sets) static ODYLIC_WRAITH: CardRecord = CardRecord::new(
    "Odylic Wraith",
    "3a3b7cd1-051c-43a8-b5f0-72a9d704efbc",
    "Ian Miller",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Wraith"], 2, 2).with_abilities(&[
        abilities::landwalk(BasicLandType::Swamp),
        AbilityDef::triggered(
            "Whenever this creature deals damage to a player, that player discards a card.",
            TriggerEventDef::damage_to_player(ObjectPredicateDef::Source, PlayerRelation::Any),
            EffectDef::Discard {
                recipient: EffectRecipientDef::EventPlayer,
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ),
    ]),
);

// WTH 78 — Razortooth Rats
pub(in crate::card::sets) static RAZORTOOTH_RATS: CardRecord = CardRecord::new(
    "Razortooth Rats",
    "ae869780-27e8-4a6d-9ac6-cdab617725e2",
    "Brian Horton",
    // Three mana for two damage a turn that most decks simply cannot stop,
    // which is the deal fear has always offered.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Rat"], 2, 1).with_ability(abilities::fear()),
);

// WTH 79 — Shadow Rider
pub(in crate::card::sets) static SHADOW_RIDER: CardRecord = CardRecord::new(
    "Shadow Rider",
    "5bfdec24-e689-4cca-a546-a8f5d0929f8d",
    "Pete Venters",
    // Flanking on a 3/3 means a single blocker dies and the Rider lives, so
    // it attacks profitably into almost anything.
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Knight"], 3, 3)
        .with_ability(abilities::flanking()),
);

// WTH 80 — Shattered Crypt
pub(in crate::card::sets) static SHATTERED_CRYPT: CardRecord = CardRecord::new(
    "Shattered Crypt",
    "117df45d-4500-459b-96b5-ca41952580c1",
    "Gary Leach",
    CardRules::new_sorcery(mana_cost!("{X}{B}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Return X target creature cards from your graveyard to your hand. You lose X life.",
        &[AbilityTargetDef::exactly_chosen_x(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::ChosenX,
            },
        ]),
    )),
);

// WTH 81 — Spinning Darkness
// Audit: unsupported — Needs an alternative cost that exiles the top three black cards of the caster's graveyard.
pub(in crate::card::sets) static SPINNING_DARKNESS: CardRecord = CardRecord::new(
    "Spinning Darkness",
    "58e64a8e-84b1-416c-9fa7-8b10130dc9e9",
    "John Coulthart",
    crate::card::CardRules::unsupported(),
);

// WTH 82 — Strands of Night
pub(in crate::card::sets) static STRANDS_OF_NIGHT: CardRecord = CardRecord::new(
    "Strands of Night",
    "872ef62f-e119-470b-b212-9beb48469095",
    "Patrick Kochakji",
CardRules::new_enchantment(mana_cost!("{2}{B}{B}")).with_ability(
        AbilityDef::activated_with_targets(
            "{B}{B}, Pay 2 life, Sacrifice a Swamp: Return target creature card from your graveyard to the battlefield.",
            &[
                CostDef::Mana(mana_cost!("{B}{B}")),
                CostDef::PayLife(2),
                CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Swamp]),
                    controller: PlayerRelation::You,
                },
            ],
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            })],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
            },
        ),
    ),
);

// WTH 83 — Tendrils of Despair
pub(in crate::card::sets) static TENDRILS_OF_DESPAIR: CardRecord = CardRecord::new(
    "Tendrils of Despair",
    "b5d73ddb-bd3c-4625-9f75-ba2079553915",
    "John Coulthart",
CardRules::new_sorcery(mana_cost!("{B}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, sacrifice a creature.\nTarget opponent discards two cards.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                PlayerRelation::Opponent,
            ))],
            CostDef::sacrifice(
                ObjectPredicateDef::HasType(CardType::Creature),
                CostQuantityDef::Fixed(1),
            ),
            EffectDef::Discard {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ),
    ),
);

// WTH 84 — Urborg Justice
// Audit: unsupported — Needs the count of creatures put into the caster's graveyard from the battlefield this turn.
pub(in crate::card::sets) static URBORG_JUSTICE: CardRecord = CardRecord::new(
    "Urborg Justice",
    "39f322ff-0b04-41ce-90cd-9896f941e703",
    "Gary Leach",
    crate::card::CardRules::unsupported(),
);

// WTH 85 — Urborg Stalker
// Audit: unsupported — Needs an each-player upkeep condition and damage recipient tied to that turn's active player.
pub(in crate::card::sets) static URBORG_STALKER: CardRecord = CardRecord::new(
    "Urborg Stalker",
    "2d33e3d5-c608-4ba8-8614-0b9d0385af64",
    "Cliff Nielsen",
    crate::card::CardRules::unsupported(),
);

// WTH 86 — Wave of Terror
pub(in crate::card::sets) static WAVE_OF_TERROR: CardRecord = CardRecord::new(
    "Wave of Terror",
    "d40ab3e7-9abb-4acc-9932-de03b533722f",
    "Adrian Smith",
CardRules::new_enchantment(mana_cost!("{2}{B}")).with_abilities(&[
        abilities::cumulative_upkeep(
            &[CostDef::mana(mana_cost!("{1}"))],
        ),
        AbilityDef::triggered(
            "At the beginning of your draw step, destroy each creature with mana value equal to the number of age counters on this enchantment. They can't be regenerated.",
            TriggerEventDef::StepBegins {
                step: crate::card::TurnStepDef::Draw,
                player: PlayerRelation::You,
            },
            EffectDef::WithRule {
                rule: AppliedRuleDef::CannotRegenerate,
                effect: &EffectDef::Destroy {
                    object: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::ManaValueEqualTo(ValueDef::CountersOnSource(
                                CounterKind::named("age"),
                            )),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    then: None,
                },
            },
        ),
    ]),
);

// WTH 87 — Zombie Scavengers
// Audit: unsupported — Needs an activation cost that exiles specifically the top creature card of a graveyard.
pub(in crate::card::sets) static ZOMBIE_SCAVENGERS: CardRecord = CardRecord::new(
    "Zombie Scavengers",
    "2ec786b1-6097-4e97-99b0-571d6e3e73e7",
    "Patrick Kochakji",
    crate::card::CardRules::unsupported(),
);

// WTH 88 — Aether Flash
pub(in crate::card::sets) static AETHER_FLASH: CardRecord = CardRecord::new(
    "Aether Flash",
    "28f6642d-393d-49a5-8c49-c1f62524ea20",
    "Ron Spencer",
    CardRules::new_enchantment(mana_cost!("{2}{R}{R}")).with_ability(AbilityDef::triggered(
        "Whenever a creature enters, this enchantment deals 2 damage to it.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::HasType(CardType::Creature),
            None,
            Some(ZoneKind::Battlefield),
        ),
        EffectDef::damage(
            EffectRecipientDef::TriggeringZoneChangeResult,
            ValueDef::Constant(2),
        ),
    )),
);

// WTH 89 — Betrothed of Fire
pub(in crate::card::sets) static BETROTHED_OF_FIRE: CardRecord = CardRecord::new(
    "Betrothed of Fire",
    "5e517aa4-d8ba-4a49-bf9f-172bf029fa52",
    "Clint Langley",
    CardRules::new_enchantment(mana_cost!("{1}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::activated(
                "Sacrifice an untapped creature: Enchanted creature gets +2/+0 until end of turn.",
                &[CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
                    ]),
                    controller: PlayerRelation::You,
                }],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::activated(
                "Sacrifice enchanted creature: Creatures you control get +2/+0 until end of turn.",
                &[CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::AttachedToSource,
                    controller: PlayerRelation::You,
                }],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// WTH 90 — Bloodrock Cyclops
pub(in crate::card::sets) static BLOODROCK_CYCLOPS: CardRecord = CardRecord::new(
    "Bloodrock Cyclops",
    "5c642fd9-38f7-4029-ab93-e1dc5636c1ad",
    "Tom Wänerstrand",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Cyclops"], 3, 3)
        .with_ability(abilities::attacks_each_combat_if_able()),
);

// WTH 91 — Bogardan Firefiend
pub(in crate::card::sets) static BOGARDAN_FIREFIEND: CardRecord = CardRecord::new(
    "Bogardan Firefiend",
    "80ff9650-d25f-4c6b-b96e-794b50af3f14",
    "Terese Nielsen",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Elemental", "Spirit"], 2, 1).with_ability(
        abilities::dies_trigger_with_targets(
            "When this creature dies, it deals 2 damage to target creature.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(2),
            ),
        ),
    ),
);

// WTH 92 — Boiling Blood
pub(in crate::card::sets) static BOILING_BLOOD: CardRecord = CardRecord::new(
    "Boiling Blood",
    "3fcb85b6-ab5a-40db-aaae-555315f32877",
    "Cliff Nielsen",
    CardRules::new_instant(mana_cost!("{2}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature attacks this turn if able.\nDraw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(
                    &abilities::attacks_each_combat_if_able()
                        .override_text("This creature attacks this turn if able."),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// WTH 93 — Cinder Giant
pub(in crate::card::sets) static CINDER_GIANT: CardRecord = CardRecord::new(
    "Cinder Giant",
    "de97c939-2c44-4c43-9d66-1087bcee692b",
    "Rogério Vilela",
CardRules::new_creature(mana_cost!("{3}{R}"), &["Giant"], 5, 3).with_ability(
        AbilityDef::triggered(
            "At the beginning of your upkeep, this creature deals 2 damage to each other creature you control.",
            TriggerEventDef::StepBegins {
                step: crate::card::TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::damage(
                EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                ValueDef::Constant(2),
            ),
        ),
    ),
);

// WTH 94 — Cinder Wall
// Audit: unsupported — Needs a delayed end-of-combat destruction effect bound to the blocking event.
pub(in crate::card::sets) static CINDER_WALL: CardRecord = CardRecord::new(
    "Cinder Wall",
    "6c1e429c-2e66-4363-b50a-b12b72efa060",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// WTH 95 — Cone of Flame
pub(in crate::card::sets) static CONE_OF_FLAME: CardRecord = CardRecord::new(
    "Cone of Flame",
    "5713f17a-9a57-41f8-b492-ced876e1a37f",
    "Ron Spencer",
CardRules::new_sorcery(mana_cost!("{3}{R}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "This spell deals 1 damage to any target, 2 damage to another target, and 3 damage to a third target.",
            &[
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget),
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget).another(),
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget).another(),
            ],
            EffectDef::Sequence(&[
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(1),
                ),
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex(1)),
                    ValueDef::Constant(2),
                ),
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex(2)),
                    ValueDef::Constant(3),
                ),
            ]),
        ),
    ),
);

// WTH 96 — Desperate Gambit
// Audit: unsupported — Needs a chosen source, a coin flip, and a consumable next-damage replacement selected by the flip result.
pub(in crate::card::sets) static DESPERATE_GAMBIT: CardRecord = CardRecord::new(
    "Desperate Gambit",
    "f4245160-274e-4c39-9bcd-c64e9a44dfdb",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// WTH 97 — Dwarven Berserker
pub(in crate::card::sets) static DWARVEN_BERSERKER: CardRecord = CardRecord::new(
    "Dwarven Berserker",
    "7bc734e9-fb09-4094-94b6-76c0458649e9",
    "Douglas Shuler",
CardRules::new_creature(mana_cost!("{1}{R}"), &["Dwarf", "Berserker"], 1, 1).with_ability(
        AbilityDef::triggered(
            "Whenever this creature becomes blocked, it gets +3/+0 and gains trample until end of turn.",
            TriggerEventDef::BecomesBlocked(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::add_ability(&abilities::trample()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// WTH 98 — Dwarven Thaumaturgist
pub(in crate::card::sets) static DWARVEN_THAUMATURGIST: CardRecord = CardRecord::new(
    "Dwarven Thaumaturgist",
    "8e68aa29-9f38-48a2-b00a-39aef9d91f6d",
    "Kipling West",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Dwarf", "Shaman"], 1, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Switch target creature's power and toughness until end of turn.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::switch_power_toughness(),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// WTH 99 — Fervor
pub(in crate::card::sets) static FERVOR: CardRecord = CardRecord::new(
    "Fervor",
    "b4df70ea-2b6b-4e25-a564-655989ef16fa",
    "Franz Vohwinkel",
    CardRules::new_enchantment(mana_cost!("{2}{R}")).with_ability(AbilityDef::static_ability(
        "Creatures you control have haste.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::add_ability(&abilities::haste()),
        },
    )),
);

// WTH 100 — Fire Whip
pub(in crate::card::sets) static FIRE_WHIP: CardRecord = CardRecord::new(
    "Fire Whip",
    "3ee194b4-f18f-4ebd-b42f-c7dfef42f22e",
    "Jeff Miracola",
    CardRules::new_enchantment(mana_cost!("{1}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell(
                "Enchant creature you control",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
            ),
            AbilityDef::static_ability(
                "Enchanted creature has \"{T}: This creature deals 1 damage to any target.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&AbilityDef::activated_with_targets(
                        "{T}: This creature deals 1 damage to any target.",
                        &[CostDef::TapSource],
                        &[AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::AnyTarget,
                        )],
                        EffectDef::damage(
                            EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            ValueDef::Constant(1),
                        ),
                    )),
                },
            ),
            AbilityDef::activated_with_targets(
                "Sacrifice this Aura: It deals 1 damage to any target.",
                &[CostDef::SacrificeSource],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::AnyTarget,
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(1),
                ),
            ),
        ]),
);

// WTH 101 — Firestorm
pub(in crate::card::sets) static FIRESTORM: CardRecord = CardRecord::new(
    "Firestorm",
    "e674aa8a-668a-4345-95ee-73a0b87bbcb1",
    "Jeff Miracola",
CardRules::new_instant(mana_cost!("{R}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, discard X cards.\nThis spell deals X damage to each of X targets.",
            &[AbilityTargetDef::exactly_chosen_x(
                AbilityTargetPredicate::AnyTarget,
            )],
            CostDef::discard(ObjectPredicateDef::Any).with_quantity(CostQuantityDef::ChosenX),
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::ChosenX,
            ),
        ),
    ),
);

// WTH 102 — Fit of Rage
pub(in crate::card::sets) static FIT_OF_RAGE: CardRecord = CardRecord::new(
    "Fit of Rage",
    "09e7b9ec-90cf-4d23-af5e-48394398ff06",
    "Douglas Shuler",
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +3/+3 and gains first strike until end of turn.",
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
                AppliedEffectDef::add_ability(&abilities::first_strike()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// WTH 103 — Goblin Bomb
// Audit: unsupported — Needs coin-flip branching and an activation cost that removes exactly five fuse counters atomically.
pub(in crate::card::sets) static GOBLIN_BOMB: CardRecord = CardRecord::new(
    "Goblin Bomb",
    "97e8a436-9fd0-409f-a020-0f9f41602d50",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// WTH 104 — Goblin Grenadiers
// Audit: unsupported — Needs an unblocked-attacker sacrifice choice followed by independently targeted creature and land destruction.
pub(in crate::card::sets) static GOBLIN_GRENADIERS: CardRecord = CardRecord::new(
    "Goblin Grenadiers",
    "5a73db23-727f-4d63-97d7-2ca542276722",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// WTH 105 — Goblin Vandal
pub(in crate::card::sets) static GOBLIN_VANDAL: CardRecord = CardRecord::new(
    "Goblin Vandal",
    "b7ad3b81-f706-4b33-b1ec-7600182a5232",
    "Franz Vohwinkel",
CardRules::new_creature(mana_cost!("{R}"), &["Goblin", "Rogue"], 1, 1).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks and isn't blocked, you may pay {R}. If you do, destroy target artifact defending player controls and this creature assigns no combat damage this turn.",
            TriggerEventDef::AttacksAndIsNotBlocked {
                attacker: ObjectPredicateDef::Source,
            },
            // The artifact has to belong to the player being attacked, which in a
            // two-player game is the only opponent there is.
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Artifact),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::Mana(mana_cost!("{R}"))], // Paying trades the hit for the artifact: the Vandal connects, and then
                // deals nothing because it spent the swing breaking something instead.
                &EffectDef::Sequence(&[
                    EffectDef::Destroy {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        then: None,
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::AssignsNoCombatDamage),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            )),
        ),
    ),
);

// WTH 106 — Heart of Bogardan
pub(in crate::card::sets) static HEART_OF_BOGARDAN: CardRecord = CardRecord::new(
    "Heart of Bogardan",
    "4e30d025-1df9-4a08-b686-037e9cbf23a6",
    "Terese Nielsen",
CardRules::new_enchantment(mana_cost!("{2}{R}{R}")).with_abilities(&[
        abilities::cumulative_upkeep(
            &[CostDef::mana(mana_cost!("{2}"))],
        ),
        AbilityDef::triggered_with_targets(
            "When a player doesn't pay this enchantment's cumulative upkeep, this enchantment deals X damage to target player or planeswalker and each creature that player or that planeswalker's controller controls, where X is twice the number of age counters on this enchantment minus 2.",
            TriggerEventDef::PaymentNotPaid(crate::card::AbilityLabel::CUMULATIVE_UPKEEP),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )],
            EffectDef::Sequence(&[
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Sum(&crate::card::SumValueDef::new(
                        ValueDef::Scaled(&crate::card::ScaledValueDef::new(
                            ValueDef::CountersOnSource(CounterKind::named("age")),
                            2,
                        )),
                        ValueDef::Constant(-2),
                    )),
                ),
                EffectDef::damage(
                    EffectRecipientDef::objects_controlled_by_target(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        TargetIndex::PRIMARY,
                    ),
                    ValueDef::Sum(&crate::card::SumValueDef::new(
                        ValueDef::Scaled(&crate::card::ScaledValueDef::new(
                            ValueDef::CountersOnSource(CounterKind::named("age")),
                            2,
                        )),
                        ValueDef::Constant(-2),
                    )),
                ),
            ]),
        ),
    ]),
);

// WTH 107 — Heat Stroke
// Audit: unsupported — Needs end-of-combat history for every creature that blocked or was blocked this turn.
pub(in crate::card::sets) static HEAT_STROKE: CardRecord = CardRecord::new(
    "Heat Stroke",
    "1baf2a6c-57ec-4b38-8b08-4b3f800dbe99",
    "Andrew Robinson",
    crate::card::CardRules::unsupported(),
);

// WTH 108 — Hurloon Shaman
pub(in crate::card::sets) static HURLOON_SHAMAN: CardRecord = CardRecord::new(
    "Hurloon Shaman",
    "70a359c9-1889-426d-acaf-074cfd9f274d",
    "Scott M. Fischer",
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Minotaur", "Shaman"], 2, 3).with_ability(
        abilities::dies_trigger(
            "When this creature dies, each player sacrifices a land of their choice.",
            EffectDef::SacrificeOfChoice {
                player: EffectRecipientDef::EachPlayer,
                object: ObjectPredicateDef::HasType(CardType::Land),
                count: ValueDef::Constant(1),
                then: None,
                amount: SacrificedAmountDef::Power,
                otherwise: None,
                optional: false,
            },
        ),
    ),
);

// WTH 109 — Lava Hounds
pub(in crate::card::sets) static LAVA_HOUNDS: CardRecord = CardRecord::new(
    "Lava Hounds",
    "896dcaf0-3e52-4189-990f-cabab40ffbd1",
    "Steve White",
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Dog"], 4, 4).with_abilities(&[
        abilities::haste(),
        abilities::enters_trigger(
            "When this creature enters, it deals 4 damage to you.",
            EffectDef::damage(EffectRecipientDef::Controller, ValueDef::Constant(4)),
        ),
    ]),
);

// WTH 110 — Lava Storm
pub(in crate::card::sets) static LAVA_STORM: CardRecord = CardRecord::new(
    "Lava Storm",
    "61fcd58e-e5e2-45f4-9edd-300a871ae5f5",
    "Scott Kirschner",
    CardRules::new_instant(mana_cost!("{3}{R}{R}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "Lava Storm deals 2 damage to each attacking creature.",
                EffectDef::damage(
                    EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::Attacking,
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    ValueDef::Constant(2),
                ),
            ),
            AbilityDef::spell(
                "Lava Storm deals 2 damage to each blocking creature.",
                EffectDef::damage(
                    EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::Blocking,
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    ValueDef::Constant(2),
                ),
            ),
        ],
    )),
);

// WTH 111 — Maraxus of Keld
// Audit: unsupported — Needs a characteristic-defining count of untapped artifacts, creatures, and lands controlled by the source's controller.
pub(in crate::card::sets) static MARAXUS_OF_KELD: CardRecord = CardRecord::new(
    "Maraxus of Keld",
    "59329155-a423-4e8d-a7d4-c99555ff5ed1",
    "Adrian Smith",
    crate::card::CardRules::unsupported(),
);

// WTH 112 — Orcish Settlers
// Audit: unsupported — The runtime cannot yet enumerate or pay a repeated-X activation cost while using that X as the exact target count.
pub(in crate::card::sets) static ORCISH_SETTLERS: CardRecord = CardRecord::new(
    "Orcish Settlers",
    "d54764f6-6f65-405c-ba30-1e485ce3fe21",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// WTH 113 — Roc Hatchling
pub(in crate::card::sets) static ROC_HATCHLING: CardRecord = CardRecord::new(
    "Roc Hatchling",
    "25857884-6bb7-4a8e-a08b-fa610af8a5c3",
    "Una Fricker",
    CardRules::new_creature(mana_cost!("{R}"), &["Bird"], 0, 1).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with four shell counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::named("shell"),
                    amount: 4,
                },
            ),
        ),
        AbilityDef::triggered(
            "At the beginning of your upkeep, remove a shell counter from this creature.",
            TriggerEventDef::StepBegins {
                step: crate::card::TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::RemoveCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("shell"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::static_ability(
            "As long as this creature has no shell counters on it, it gets +3/+2 and has flying.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Source,
                        ObjectPredicateDef::CounterCount {
                            kind: CounterKind::named("shell"),
                            comparison: ComparisonDef::Equal,
                            amount: 0,
                        },
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(2),
                    ),
                    AppliedEffectDef::add_ability(&abilities::flying()),
                ]),
            },
        ),
    ]),
);

// WTH 114 — Sawtooth Ogre
// Audit: unsupported — Needs delayed end-of-combat damage to each creature paired with this source by blocking.
pub(in crate::card::sets) static SAWTOOTH_OGRE: CardRecord = CardRecord::new(
    "Sawtooth Ogre",
    "4a237580-f7f6-4d6b-a342-0d11fc0b5a59",
    "Brom",
    crate::card::CardRules::unsupported(),
);

// WTH 115 — Thunderbolt
pub(in crate::card::sets) static THUNDERBOLT: CardRecord = CardRecord::new(
    "Thunderbolt",
    "a0a4b641-2eb3-482b-91a1-236ebe2a7a41",
    "Dylan Martens",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Thunderbolt deals 3 damage to target player or planeswalker.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(3),
                ),
            ),
            AbilityDef::spell_with_targets(
                "Thunderbolt deals 4 damage to target creature with flying.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                    ]),
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(4),
                ),
            ),
        ],
    )),
);

// WTH 116 — Thundermare (reprint)
const THUNDERMARE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::THUNDERMARE,
    "e936e5cb-0a8e-4348-afea-e5f96b19fe23",
    "Bob Eggleton",
);

// WTH 117 — Aboroth
pub(in crate::card::sets) static ABOROTH: CardRecord = CardRecord::new(
    "Aboroth",
    "8c72ac67-e4fb-49a1-b1e5-cd2e414bec28",
    "Brom",
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Elemental"], 9, 9).with_ability(
        abilities::cumulative_upkeep(&[CostDef::put_counters_on_source(
            CounterKind::MinusOneMinusOne,
            1,
        )]),
    ),
);

// WTH 118 — Arctic Wolves
pub(in crate::card::sets) static ARCTIC_WOLVES: CardRecord = CardRecord::new(
    "Arctic Wolves",
    "b5fb56a2-5138-4c31-aa4b-0824a1a24573",
    "Steve White",
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Wolf"], 4, 5).with_abilities(&[
        abilities::cumulative_upkeep(&[CostDef::mana(mana_cost!("{2}"))]),
        abilities::enters_trigger(
            "When this creature enters, draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// WTH 119 — Barishi
pub(in crate::card::sets) static BARISHI: CardRecord = CardRecord::new(
    "Barishi",
    "f263eb80-f8f2-4b32-8e8b-a297de9f3666",
    "Ted Naifeh",
CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Elemental"], 4, 3).with_ability(
        abilities::dies_trigger(
            "When this creature dies, exile it, then shuffle all creature cards from your graveyard into your library.",
            EffectDef::Sequence(&[
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::TriggeringZoneChangeResult,
                    zone: ZoneKind::Exile,
                    placement: ZonePlacement::Top,
                },
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Graveyard],
                            PlayerRelation::You,
                        ),
                    )),
                    zone: ZoneKind::Library,
                    placement: ZonePlacement::Top,
                },
                EffectDef::ShuffleLibrary {
                    player: EffectRecipientDef::Controller,
                },
            ]),
        ),
    ),
);

// WTH 120 — Blossoming Wreath
pub(in crate::card::sets) static BLOSSOMING_WREATH: CardRecord = CardRecord::new(
    "Blossoming Wreath",
    "2f944ad9-c9ce-47b2-80fa-d0f7fcf0fd5d",
    "Brian Durfee",
    CardRules::new_instant(mana_cost!("{G}")).with_ability(AbilityDef::spell(
        "You gain life equal to the number of creature cards in your graveyard.",
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Graveyard],
                PlayerRelation::You,
            )),
        },
    )),
);

// WTH 121 — Briar Shield
pub(in crate::card::sets) static BRIAR_SHIELD: CardRecord = CardRecord::new(
    "Briar Shield",
    "68100ac2-9677-4eb5-93dc-54e49b15985d",
    "Scott Kirschner",
    CardRules::new_enchantment(mana_cost!("{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
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
                "Sacrifice this Aura: Enchanted creature gets +3/+3 until end of turn.",
                &[CostDef::SacrificeSource],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(3),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// WTH 122 — Call of the Wild
pub(in crate::card::sets) static CALL_OF_THE_WILD: CardRecord = CardRecord::new(
    "Call of the Wild",
    "a742bc7c-7f0d-4dff-b229-f16d54fe1347",
    "Brom",
CardRules::new_enchantment(mana_cost!("{2}{G}{G}")).with_ability(AbilityDef::activated(
        "{2}{G}{G}: Reveal the top card of your library. If it's a creature card, put it onto the battlefield. Otherwise, put it into your graveyard.",
        &[CostDef::Mana(mana_cost!("{2}{G}{G}"))],
        EffectDef::RevealAndClassifyCards(RevealAndClassifyCardsDef {
            source: ObjectCollectionSourceDef::TopCards {
                player: PlayerRefDef::EffectController,
                count: ValueDef::Constant(1),
            },
            object: ObjectPredicateDef::HasType(CardType::Creature),
            matching: crate::Binding!("call_of_the_wild_creature"),
            remainder: crate::Binding!("call_of_the_wild_other"),
            then: &EffectDef::Sequence(&[
                EffectDef::MoveObjects(MoveObjectsDef {
                    input: ObjectSetDef::Binding(crate::Binding!("call_of_the_wild_creature")),
                    from: Some(ZoneKind::Library),
                    zone: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    moved: None,
                    then: &EffectDef::None,
                }),
                EffectDef::MoveObjects(MoveObjectsDef {
                    input: ObjectSetDef::Binding(crate::Binding!("call_of_the_wild_other")),
                    from: Some(ZoneKind::Library),
                    zone: ZoneKind::Graveyard,
                    placement: ZonePlacement::Top,
                    moved: None,
                    then: &EffectDef::None,
                }),
            ]),
        }),
    )),
);

// WTH 123 — Choking Vines
// Audit: unsupported — Needs declare-blockers-only cast timing and a rule effect that marks targeted attackers blocked without blockers.
pub(in crate::card::sets) static CHOKING_VINES: CardRecord = CardRecord::new(
    "Choking Vines",
    "6cc4a7ee-f6f0-454a-9074-5988fdee1f34",
    "Ted Naifeh",
    crate::card::CardRules::unsupported(),
);

// WTH 124 — Dense Foliage
// Audit: unsupported — Needs a spell-only targeting prohibition; hexproof/shroud would incorrectly stop abilities too.
pub(in crate::card::sets) static DENSE_FOLIAGE: CardRecord = CardRecord::new(
    "Dense Foliage",
    "c60a2035-59cb-426e-b2ae-45d8d6ce0bb8",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// WTH 125 — Downdraft
pub(in crate::card::sets) static DOWNDRAFT: CardRecord = CardRecord::new(
    "Downdraft",
    "ab4ced80-926a-4e4d-8ebd-d4fe7374a6ad",
    "John Matson",
    CardRules::new_enchantment(mana_cost!("{2}{G}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{G}: Target creature loses flying until end of turn.",
            &[CostDef::Mana(mana_cost!("{G}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::remove_abilities(AbilityPredicateDef::Keyword(
                    crate::card::KeywordAbility::Flying,
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated(
            "Sacrifice this enchantment: It deals 2 damage to each creature with flying.",
            &[CostDef::SacrificeSource],
            EffectDef::damage(
                EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Flying),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                ValueDef::Constant(2),
            ),
        ),
    ]),
);

// WTH 126 — Fallow Wurm
pub(in crate::card::sets) static FALLOW_WURM: CardRecord = CardRecord::new(
    "Fallow Wurm",
    "1ba02b6f-6010-47a4-8670-406391a52a68",
    "Stephen L. Walsh",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Wurm"], 4, 4).with_ability(
        abilities::enters_trigger(
            "When this creature enters, sacrifice it unless you discard a land card.",
            EffectDef::PayOr(PayOrDef::unless(
                &[crate::card::CostDef::discard(ObjectPredicateDef::HasType(
                    CardType::Land,
                ))],
                &EffectDef::sacrifice(EffectRecipientDef::Source),
            )),
        ),
    ),
);

// WTH 127 — Familiar Ground
// Audit: unsupported — Needs a per-attacker maximum of one blocking creature.
pub(in crate::card::sets) static FAMILIAR_GROUND: CardRecord = CardRecord::new(
    "Familiar Ground",
    "f993f517-999f-4ee6-8ffb-946bffdcf7fe",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// WTH 128 — Fungus Elemental
pub(in crate::card::sets) static FUNGUS_ELEMENTAL: CardRecord = CardRecord::new(
    "Fungus Elemental",
    "4336bfd1-27a4-414d-b6fe-f186a0563dc0",
    "Scott M. Fischer",
CardRules::new_creature(mana_cost!("{3}{G}"), &["Fungus", "Elemental"], 3, 3).with_ability(
        AbilityDef::activated(
            "{G}, Sacrifice a Forest: Put a +2/+2 counter on this creature. Activate only if this creature entered this turn.",
            &[
                CostDef::Mana(mana_cost!("{G}")),
                CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::power_toughness(2, 2),
                amount: ValueDef::Constant(1),
            },
        )
        .with_activation_condition(&TriggerConditionDef::SourceMatches {
            object: ObjectPredicateDef::EnteredThisTurn,
        }),
    ),
);

// WTH 129 — Gaea's Blessing
// Audit: unsupported — Needs graveyard targets constrained to a separately targeted player's graveyard plus ordered library insertion.
pub(in crate::card::sets) static GAEA_S_BLESSING: CardRecord = CardRecord::new(
    "Gaea's Blessing",
    "ee83d511-57e0-40fb-a4db-62f6c2c39888",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// WTH 130 — Harvest Wurm
// Audit: unsupported — Needs an enters payment that moves a chosen basic land card from the controller's graveyard to hand.
pub(in crate::card::sets) static HARVEST_WURM: CardRecord = CardRecord::new(
    "Harvest Wurm",
    "9d21139d-edfc-4140-aa43-d4165331d7f3",
    "Stephen L. Walsh",
    crate::card::CardRules::unsupported(),
);

// WTH 131 — Liege of the Hollows
// Audit: unsupported — Needs APNAP chosen-any-amount mana payments with per-player paid amounts retained for token creation.
pub(in crate::card::sets) static LIEGE_OF_THE_HOLLOWS: CardRecord = CardRecord::new(
    "Liege of the Hollows",
    "dff4512b-8244-4e38-bffb-0062a97d9531",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// WTH 132 — Llanowar Behemoth
pub(in crate::card::sets) static LLANOWAR_BEHEMOTH: CardRecord = CardRecord::new(
    "Llanowar Behemoth",
    "3d5d9bd0-7ce9-4a1e-a8b2-5c1dbb014917",
    "Hannibal King",
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Elemental"], 4, 4).with_ability(
        AbilityDef::activated(
            "Tap an untapped creature you control: This creature gets +1/+1 until end of turn.",
            &[CostDef::TapPermanents {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::You,
                count: 1,
            }],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// WTH 133 — Llanowar Druid
pub(in crate::card::sets) static LLANOWAR_DRUID: CardRecord = CardRecord::new(
    "Llanowar Druid",
    "ffad279c-762a-42cf-ac20-f4e48734c194",
    "Pete Venters",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Elf", "Druid"], 1, 2).with_ability(
        AbilityDef::activated(
            "{T}, Sacrifice this creature: Untap all Forests.",
            &[CostDef::TapSource, CostDef::SacrificeSource],
            EffectDef::Untap {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
            },
        ),
    ),
);

// WTH 134 — Llanowar Sentinel
pub(in crate::card::sets) static LLANOWAR_SENTINEL: CardRecord = CardRecord::new(
    "Llanowar Sentinel",
    "6f37ea4b-66e2-4ad5-ae7f-d02fd59131bd",
    "Douglas Shuler",
CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf"], 2, 3).with_ability(
        abilities::enters_trigger(
            "When this creature enters, you may pay {1}{G}. If you do, search your library for a card named Llanowar Sentinel, put that card onto the battlefield, then shuffle.",
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::Mana(mana_cost!("{1}{G}"))],
                &EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::NameEquals(CardNameDef::Literal("Llanowar Sentinel")),
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
            )),
        ),
    ),
);

// WTH 135 — Mwonvuli Ooze
pub(in crate::card::sets) static MWONVULI_OOZE: CardRecord = CardRecord::new(
    "Mwonvuli Ooze",
    "aa9c6f65-93a1-4913-87e7-a17ebfcc7780",
    "Zina Saunders",
CardRules::new_creature(mana_cost!("{G}"), &["Ooze"], 0, 0).with_abilities(&[
        abilities::cumulative_upkeep(
            &[CostDef::mana(mana_cost!("{2}"))],
        )
            .override_text(
                "Cumulative upkeep {2} (At the beginning of your upkeep, put an age counter on this permanent, then sacrifice it unless you pay {2} for each age counter on it.)",
            ),
        AbilityDef::static_ability(
            "This creature's power and toughness are each equal to 1 plus twice the number of age counters on it.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::set_base_power_toughness(
                    ValueDef::Sum(&crate::card::SumValueDef::new(
                        ValueDef::Constant(1),
                        ValueDef::Scaled(&crate::card::ScaledValueDef::new(
                            ValueDef::CountersOnSource(CounterKind::named("age")),
                            2,
                        )),
                    )),
                    ValueDef::Sum(&crate::card::SumValueDef::new(
                        ValueDef::Constant(1),
                        ValueDef::Scaled(&crate::card::ScaledValueDef::new(
                            ValueDef::CountersOnSource(CounterKind::named("age")),
                            2,
                        )),
                    )),
                ),
            },
        ),
    ]),
);

// WTH 136 — Nature's Kiss
// Audit: unsupported — Needs an activation cost that exiles specifically the top card of a graveyard.
pub(in crate::card::sets) static NATURE_S_KISS: CardRecord = CardRecord::new(
    "Nature's Kiss",
    "64b09c44-d463-45a9-9fa2-89407c21200b",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// WTH 137 — Nature's Resurgence
// Audit: unsupported — Needs per-player creature-card counts to feed distinct draw amounts during one resolution.
pub(in crate::card::sets) static NATURE_S_RESURGENCE: CardRecord = CardRecord::new(
    "Nature's Resurgence",
    "2df9fb85-f7fa-4617-87bd-4d457c830f46",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// WTH 138 — Redwood Treefolk (reprint)
const REDWOOD_TREEFOLK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::REDWOOD_TREEFOLK,
    "0274e162-33e4-4604-a6ea-51fc1a5c6a04",
    "Phil Foglio",
);

// WTH 139 — Rogue Elephant
pub(in crate::card::sets) static ROGUE_ELEPHANT: CardRecord = CardRecord::new(
    "Rogue Elephant",
    "1b622b2f-84ad-4203-97fa-35af09e1c370",
    "Steve White",
    CardRules::new_creature(mana_cost!("{G}"), &["Elephant"], 3, 3).with_ability(
        abilities::enters_trigger(
            "When this creature enters, sacrifice it unless you sacrifice a Forest.",
            EffectDef::PayOr(PayOrDef::unless(
                &[crate::card::CostDef::sacrifice_permanent(
                    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
                )],
                &EffectDef::sacrifice(EffectRecipientDef::Source),
            )),
        ),
    ),
);

// WTH 140 — Striped Bears
pub(in crate::card::sets) static STRIPED_BEARS: CardRecord = CardRecord::new(
    "Striped Bears",
    "0bf54365-56ae-485d-b931-784a4cf9d8f2",
    "Una Fricker",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Bear"], 2, 2).with_ability(
        abilities::enters_trigger(
            "When this creature enters, draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// WTH 141 — Sylvan Hierophant
// Audit: unsupported — Needs a dies trigger that excludes its own zone-change result from the graveyard target set.
pub(in crate::card::sets) static SYLVAN_HIEROPHANT: CardRecord = CardRecord::new(
    "Sylvan Hierophant",
    "432a6908-0ee3-45c5-9089-b7f8cf1184bb",
    "Brian Durfee",
    crate::card::CardRules::unsupported(),
);

// WTH 142 — Tranquil Grove
pub(in crate::card::sets) static TRANQUIL_GROVE: CardRecord = CardRecord::new(
    "Tranquil Grove",
    "c4a145f2-b59d-4728-922c-9bc228451432",
    "Dylan Martens",
    CardRules::new_enchantment(mana_cost!("{1}{G}")).with_ability(AbilityDef::activated(
        "{1}{G}{G}: Destroy all other enchantments.",
        &[CostDef::Mana(mana_cost!("{1}{G}{G}"))],
        EffectDef::Destroy {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            then: None,
        },
    )),
);

// WTH 143 — Uktabi Efreet
pub(in crate::card::sets) static UKTABI_EFREET: CardRecord = CardRecord::new(
    "Uktabi Efreet",
    "3678a224-d314-4108-8a39-de0c1b635b5c",
    "Alan Rabinowitz",
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Efreet"], 5, 4).with_ability(
        abilities::cumulative_upkeep(&[CostDef::mana(mana_cost!("{G}"))]),
    ),
);

// WTH 144 — Veteran Explorer
// Audit: unsupported — Needs APNAP optional searches whose chosen cards enter tapped and whose searched players shuffle independently.
pub(in crate::card::sets) static VETERAN_EXPLORER: CardRecord = CardRecord::new(
    "Veteran Explorer",
    "bdac36f2-99ce-4d48-90fa-aa7439778ffc",
    "David A. Cherry",
    crate::card::CardRules::unsupported(),
);

// WTH 145 — Vitalize
pub(in crate::card::sets) static VITALIZE: CardRecord = CardRecord::new(
    "Vitalize",
    "d6ee4997-4b1a-4e03-88ac-63b451bb7b38",
    "Pete Venters",
    CardRules::new_instant(mana_cost!("{G}")).with_ability(AbilityDef::spell(
        "Untap all creatures you control.",
        EffectDef::Untap {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
        },
    )),
);

// WTH 146 — Bubble Matrix
pub(in crate::card::sets) static BUBBLE_MATRIX: CardRecord = CardRecord::new(
    "Bubble Matrix",
    "0ca9c239-84ff-4527-aa23-bdb11856744c",
    "Brom",
    CardRules::new_artifact(mana_cost!("{4}")).with_ability(AbilityDef::static_ability(
        "Prevent all damage that would be dealt to creatures.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::prevent_damage_from(ObjectPredicateDef::Any),
        },
    )),
);

// WTH 147 — Bösium Strip
// Audit: unsupported — Needs cast permission restricted to the current top instant or sorcery card of the graveyard plus linked exile replacement.
pub(in crate::card::sets) static BOSIUM_STRIP: CardRecord = CardRecord::new(
    "Bösium Strip",
    "3884bede-df28-42e8-9ac9-ae03118b1985",
    "Steve Luke",
    crate::card::CardRules::unsupported(),
);

// WTH 148 — Chimeric Sphere
pub(in crate::card::sets) static CHIMERIC_SPHERE: CardRecord = CardRecord::new(
    "Chimeric Sphere",
    "cc96857c-b38e-4614-9838-cacd3700e3ee",
    "Colin MacNeil",
CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::activated(
            "{2}: Until end of turn, this artifact becomes a 2/1 Construct artifact creature with flying.",
            &[CostDef::Mana(mana_cost!("{2}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                    AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&[
                        "Construct",
                    ])),
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(1),
                    ),
                    AppliedEffectDef::add_ability(&abilities::flying()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated(
            "{2}: Until end of turn, this artifact becomes a 3/2 Construct artifact creature and loses flying.",
            &[CostDef::Mana(mana_cost!("{2}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                    AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&[
                        "Construct",
                    ])),
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(2),
                    ),
                    AppliedEffectDef::remove_abilities(AbilityPredicateDef::Keyword(
                        crate::card::KeywordAbility::Flying,
                    )),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// WTH 149 — Dingus Staff
pub(in crate::card::sets) static DINGUS_STAFF: CardRecord = CardRecord::new(
    "Dingus Staff",
    "065b4358-5dee-4f13-bff9-8254bdb92069",
    "Richard Kane Ferguson",
    CardRules::new_artifact(mana_cost!("{4}")).with_ability(abilities::dies_trigger_matching(
        "Whenever a creature dies, this artifact deals 2 damage to that creature's controller.",
        ObjectPredicateDef::HasType(CardType::Creature),
        EffectDef::damage(
            EffectRecipientDef::ControllerOfTriggeringObject,
            ValueDef::Constant(2),
        ),
    )),
);

// WTH 150 — Jabari's Banner
pub(in crate::card::sets) static JABARI_S_BANNER: CardRecord = CardRecord::new(
    "Jabari's Banner",
    "3d51a496-1ca6-4286-bdbe-990d43196a25",
    "Mark Harrison",
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::activated_with_targets(
        "{1}, {T}: Target creature gains flanking until end of turn.",
        &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::add_ability(&abilities::flanking()),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// WTH 151 — Jangling Automaton
// Audit: unsupported — Needs the defending player's identity for an attack trigger that untaps all creatures they control.
pub(in crate::card::sets) static JANGLING_AUTOMATON: CardRecord = CardRecord::new(
    "Jangling Automaton",
    "2e2a427b-9869-4059-aeeb-d9b97b324e4e",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// WTH 152 — Mana Web
// Audit: unsupported — Needs mana-production equivalence between a triggering land and every other land the same opponent controls.
pub(in crate::card::sets) static MANA_WEB: CardRecord = CardRecord::new(
    "Mana Web",
    "2c72ec90-dacc-496f-a7f5-f18bfce5eb3e",
    "Hannibal King",
    crate::card::CardRules::unsupported(),
);

// WTH 153 — Mind Stone
pub(in crate::card::sets) static MIND_STONE: CardRecord = CardRecord::new(
    "Mind Stone",
    "162e81d3-6cd4-4cb8-8ed8-cfbd8d34ca71",
    "Adam Rex",
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {C}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
        AbilityDef::activated(
            "{1}, {T}, Sacrifice this artifact: Draw a card.",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// WTH 154 — Null Rod
pub(in crate::card::sets) static NULL_ROD: CardRecord = CardRecord::new(
    "Null Rod",
    "bc45f2cb-c256-4a0f-879a-c7db5b1a0b94",
    "Anson Maddocks",
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::static_ability(
        "Activated abilities of artifacts can't be activated.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Artifact),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::cannot_activate_abilities(AbilityPredicateDef::Any),
        },
    )),
);

// WTH 155 — Phyrexian Furnace
pub(in crate::card::sets) static PHYREXIAN_FURNACE: CardRecord = CardRecord::new(
    "Phyrexian Furnace",
    "e98bca31-8c05-430b-b5d7-331bdc55710a",
    "George Pratt",
    // The tap mode eats a graveyard from the bottom, one card a turn; the
    // sacrifice mode answers the one card that actually mattered.
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: Exile the bottom card of target player's graveyard.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::objects(ObjectSetDef::BottomOfGraveyard(
                    PlayerRefDef::Target(TargetIndex::PRIMARY),
                )),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
        ),
        AbilityDef::activated_with_targets(
            "{1}, Sacrifice this artifact: Exile target card from a graveyard. Draw a card.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::SacrificeSource],
            // Any card in any graveyard, which is what the sacrifice mode reaches. The
            // tap mode needs no target beyond the player, because a graveyard has only
            // one bottom card.
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Sequence(
                &const {
                    [
                        EffectDef::MoveToZone {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            zone: ZoneKind::Exile,
                            placement: ZonePlacement::Top,
                        },
                        EffectDef::DrawCards {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(1),
                        },
                    ]
                },
            ),
        ),
    ]),
);

// WTH 156 — Serrated Biskelion
pub(in crate::card::sets) static SERRATED_BISKELION: CardRecord = CardRecord::new(
    "Serrated Biskelion",
    "c449126c-ac01-4a90-b967-8c3ad112091b",
    "Ron Spencer",
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Construct"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Put a -1/-1 counter on this creature and a -1/-1 counter on target creature.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::CombineObjects(CombineObjectsDef {
                inputs: &[
                    ObjectSetDef::One(ObjectRefDef::Source),
                    ObjectSetDef::LegalTargets(TargetIndex::PRIMARY),
                ],
                combined: ParentBinding,
                then: &EffectDef::AddCounters {
                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                    kind: CounterKind::MinusOneMinusOne,
                    amount: ValueDef::Constant(1),
                },
            }),
        ),
    ),
);

// WTH 157 — Steel Golem
pub(in crate::card::sets) static STEEL_GOLEM: CardRecord = CardRecord::new(
    "Steel Golem",
    "9aa927e0-5a65-4ac1-8eca-c000bb8080e7",
    "Donato Giancola",
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Golem"], 3, 4).with_ability(
        AbilityDef::static_ability(
            "You can't cast creature spells.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(
                    PlayRestrictionDef::new(
                        PlayActionMatcherDef::CastSpell,
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ),
                )),
            },
        ),
    ),
);

// WTH 158 — Straw Golem
pub(in crate::card::sets) static STRAW_GOLEM: CardRecord = CardRecord::new(
    "Straw Golem",
    "43d62479-92ac-43e2-a3d3-b41dfe0fbb20",
    "Bryan Talbot",
    CardRules::new_artifact_creature(mana_cost!("{1}"), &["Golem"], 2, 3).with_ability(
        AbilityDef::triggered(
            "When an opponent casts a creature spell, sacrifice this creature.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
            ])),
            EffectDef::sacrifice(EffectRecipientDef::Source),
        ),
    ),
);

// WTH 159 — Thran Forge
pub(in crate::card::sets) static THRAN_FORGE: CardRecord = CardRecord::new(
    "Thran Forge",
    "b9c9691b-bee8-4251-8275-5f6ba14a8ecd",
    "Mark Poole",
CardRules::new_artifact(mana_cost!("{3}")).with_ability(
        AbilityDef::activated_with_targets(
            "{2}: Until end of turn, target nonartifact creature gets +1/+0 and becomes an artifact in addition to its other types.",
            &[CostDef::Mana(mana_cost!("{2}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Artifact)),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Artifact)),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// WTH 160 — Thran Tome
// Audit: unsupported — Needs ordered top-three reveal, an opponent's choice from that revealed group, and drawing the remainder.
pub(in crate::card::sets) static THRAN_TOME: CardRecord = CardRecord::new(
    "Thran Tome",
    "63db7360-fe6e-430f-bfee-a2f80bcb6fec",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// WTH 161 — Touchstone
pub(in crate::card::sets) static TOUCHSTONE: CardRecord = CardRecord::new(
    "Touchstone",
    "923afe8a-e82c-4b93-bb42-8f5073acae13",
    "George Pratt",
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::activated_with_targets(
        "{T}: Tap target artifact you don't control.",
        &[CostDef::TapSource],
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Artifact),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::Opponent),
                owner: None,
            },
        )],
        EffectDef::Tap {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    )),
);

// WTH 162 — Well of Knowledge
// Audit: unsupported — Needs an activation restriction tied specifically to each player's draw step.
pub(in crate::card::sets) static WELL_OF_KNOWLEDGE: CardRecord = CardRecord::new(
    "Well of Knowledge",
    "5184b967-f474-4c9b-9a20-65ddb0d6e4f8",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// WTH 163 — Xanthic Statue
pub(in crate::card::sets) static XANTHIC_STATUE: CardRecord = CardRecord::new(
    "Xanthic Statue",
    "8becb285-cd91-4de0-af59-ddaa7d8c5366",
    "Hannibal King",
CardRules::new_artifact(mana_cost!("{8}")).with_ability(AbilityDef::activated(
        "{5}: Until end of turn, this artifact becomes an 8/8 Golem artifact creature with trample.",
        &[CostDef::Mana(mana_cost!("{5}"))],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Golem"])),
                AppliedEffectDef::set_base_power_toughness(
                    ValueDef::Constant(8),
                    ValueDef::Constant(8),
                ),
                AppliedEffectDef::add_ability(&abilities::trample()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// WTH 164 — Gemstone Mine
pub(in crate::card::sets) static GEMSTONE_MINE: CardRecord = CardRecord::new(
    "Gemstone Mine",
    "09507f7f-c58f-4f57-b878-b39811a5b619",
    "Brom",
// Three activations of perfect mana, and then nothing: the deck that
    // plays four of these is buying the first three turns, not the tenth.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters with three mining counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::named("mining"),
                    amount: 3,
                },
            ),
        ),
        AbilityDef::activated_mana(
            "{T}, Remove a mining counter from this land: Add one mana of any color. If there are no mining counters on this land, sacrifice it.",
            &[
                CostDef::TapSource,
                CostDef::RemoveCountersFromSource {
                    kind: CounterKind::named("mining"),
                    amount: 1,
                },
            ],
            EffectDef::AddMana(
                AddManaEffectDef::any_color().sacrificing_source_when_out_of(CounterKind::named("mining")),
            ),
        ),
    ]),
);

// WTH 165 — Lotus Vale
// Audit: unsupported — Needs an enters replacement that atomically sacrifices two untapped lands or moves this land to its owner's graveyard.
pub(in crate::card::sets) static LOTUS_VALE: CardRecord = CardRecord::new(
    "Lotus Vale",
    "2e5cd12a-2a07-44a8-8eac-de00d26fe9e3",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// WTH 166 — Scorched Ruins
// Audit: unsupported — Needs an enters replacement that atomically sacrifices two untapped lands or moves this land to its owner's graveyard.
pub(in crate::card::sets) static SCORCHED_RUINS: CardRecord = CardRecord::new(
    "Scorched Ruins",
    "75a4e843-937c-47fb-8768-0f42c5cb4e4f",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// WTH 167 — Winding Canyons
pub(in crate::card::sets) static WINDING_CANYONS: CardRecord = CardRecord::new(
    "Winding Canyons",
    "f26672a8-f4ff-4c64-bb3e-f5072bbc9e3e",
    "John Avon",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{2}, {T}: You may cast creature spells this turn as though they had flash.",
            &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayCastAsThoughItHadFlash(
                    CastTimingPermissionDef::new(ObjectPredicateDef::HasType(CardType::Creature)),
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ABEYANCE,
    &ALMS,
    &ANGELIC_RENEWAL,
    &ARGIVIAN_FIND,
    &AURA_OF_SILENCE,
    &BENALISH_INFANTRY,
    &BENALISH_KNIGHT,
    &BENALISH_MISSIONARY,
    &DEBT_OF_LOYALTY,
    &DUSKRIDER_FALCON,
    &EMPYRIAL_ARMOR,
    &FORIYSIAN_BRIGADE,
    &GERRARD_S_WISDOM,
    &GUIDED_STRIKE,
    &HEAVY_BALLISTA,
    &INNER_SANCTUM,
    &KITHKIN_ARMOR,
    &MASTER_OF_ARMS,
    &MISTMOON_GRIFFIN,
    &PEACEKEEPER,
    &REVERED_UNICORN,
    &SERENITY,
    &SERRA_S_BLESSING,
    &SOUL_SHEPHERD,
    &SOUTHERN_PALADIN,
    &TARIFF,
    &VOLUNTEER_RESERVES,
    &ABDUCTION,
    &ABJURE,
    &ANCESTRAL_KNOWLEDGE,
    &APATHY,
    &ARGIVIAN_RESTORATION,
    &AVIZOA,
    &CLOUD_DJINN,
    &DISRUPT,
    &ERTAI_S_FAMILIAR,
    &FOG_ELEMENTAL,
    &MANA_CHAINS,
    &MANTA_RAY,
    &MERFOLK_TRADERS,
    &NOBLE_BENEFACTOR,
    &OPHIDIAN,
    &PARADIGM_SHIFT,
    &PENDRELL_MISTS,
    &PHANTOM_WINGS,
    &PSYCHIC_VORTEX,
    &RELEARN,
    &SAGE_OWL,
    &TEFERI_S_VEIL,
    &TIMID_DRAKE,
    &TOLARIAN_DRAKE,
    &TOLARIAN_ENTRANCER,
    &TOLARIAN_SERPENT,
    &VODALIAN_ILLUSIONIST,
    &ABYSSAL_GATEKEEPER,
    &AGONIZING_MEMORIES,
    &BARROW_GHOUL,
    &BONE_DANCER,
    &BURIED_ALIVE,
    &CIRCLING_VULTURES,
    &COILS_OF_THE_MEDUSA,
    &DOOMSDAY,
    &FATAL_BLOW,
    &FESTERING_EVIL,
    &FLEDGLING_DJINN,
    &GALLOWBRAID,
    &HAUNTING_MISERY,
    &HIDDEN_HORROR,
    &INFERNAL_TRIBUTE,
    &MISCHIEVOUS_POLTERGEIST,
    &MORINFEN,
    &NECRATOG,
    &ODYLIC_WRAITH,
    &RAZORTOOTH_RATS,
    &SHADOW_RIDER,
    &SHATTERED_CRYPT,
    &SPINNING_DARKNESS,
    &STRANDS_OF_NIGHT,
    &TENDRILS_OF_DESPAIR,
    &URBORG_JUSTICE,
    &URBORG_STALKER,
    &WAVE_OF_TERROR,
    &ZOMBIE_SCAVENGERS,
    &AETHER_FLASH,
    &BETROTHED_OF_FIRE,
    &BLOODROCK_CYCLOPS,
    &BOGARDAN_FIREFIEND,
    &BOILING_BLOOD,
    &CINDER_GIANT,
    &CINDER_WALL,
    &CONE_OF_FLAME,
    &DESPERATE_GAMBIT,
    &DWARVEN_BERSERKER,
    &DWARVEN_THAUMATURGIST,
    &FERVOR,
    &FIRE_WHIP,
    &FIRESTORM,
    &FIT_OF_RAGE,
    &GOBLIN_BOMB,
    &GOBLIN_GRENADIERS,
    &GOBLIN_VANDAL,
    &HEART_OF_BOGARDAN,
    &HEAT_STROKE,
    &HURLOON_SHAMAN,
    &LAVA_HOUNDS,
    &LAVA_STORM,
    &MARAXUS_OF_KELD,
    &ORCISH_SETTLERS,
    &ROC_HATCHLING,
    &SAWTOOTH_OGRE,
    &THUNDERBOLT,
    &ABOROTH,
    &ARCTIC_WOLVES,
    &BARISHI,
    &BLOSSOMING_WREATH,
    &BRIAR_SHIELD,
    &CALL_OF_THE_WILD,
    &CHOKING_VINES,
    &DENSE_FOLIAGE,
    &DOWNDRAFT,
    &FALLOW_WURM,
    &FAMILIAR_GROUND,
    &FUNGUS_ELEMENTAL,
    &GAEA_S_BLESSING,
    &HARVEST_WURM,
    &LIEGE_OF_THE_HOLLOWS,
    &LLANOWAR_BEHEMOTH,
    &LLANOWAR_DRUID,
    &LLANOWAR_SENTINEL,
    &MWONVULI_OOZE,
    &NATURE_S_KISS,
    &NATURE_S_RESURGENCE,
    &ROGUE_ELEPHANT,
    &STRIPED_BEARS,
    &SYLVAN_HIEROPHANT,
    &TRANQUIL_GROVE,
    &UKTABI_EFREET,
    &VETERAN_EXPLORER,
    &VITALIZE,
    &BUBBLE_MATRIX,
    &BOSIUM_STRIP,
    &CHIMERIC_SPHERE,
    &DINGUS_STAFF,
    &JABARI_S_BANNER,
    &JANGLING_AUTOMATON,
    &MANA_WEB,
    &MIND_STONE,
    &NULL_ROD,
    &PHYREXIAN_FURNACE,
    &SERRATED_BISKELION,
    &STEEL_GOLEM,
    &STRAW_GOLEM,
    &THRAN_FORGE,
    &THRAN_TOME,
    &TOUCHSTONE,
    &WELL_OF_KNOWLEDGE,
    &XANTHIC_STATUE,
    &GEMSTONE_MINE,
    &LOTUS_VALE,
    &SCORCHED_RUINS,
    &WINDING_CANYONS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    ALABASTER_DRAGON_REPRINT,
    ARDENT_MILITIA_REPRINT,
    FLUX_REPRINT,
    PHANTOM_WARRIOR_REPRINT,
    THUNDERMARE_REPRINT,
    REDWOOD_TREEFOLK_REPRINT,
];
