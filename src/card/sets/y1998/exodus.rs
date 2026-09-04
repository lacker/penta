//! EXO card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::sets::y2011::innistrad as catalog_isd;
use crate::card::sets::y2011::magic_2012 as catalog_m12;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, ActivationTimingDef,
    AddManaEffectDef, AppliedEffectDef, AppliedRuleDef, BattlefieldEntryModificationDef,
    BlockRestrictionDef, BlockRestrictionMatchDef, BlockRestrictionSubjectDef, CardArt, CardRules,
    CardSet, CardSupertype, CardType, ChoiceVisibilityDef, ChooseDef, ChooseForEachPlayerDef,
    ClassifyObjectsDef, ComparisonDef, ConditionalStaticEffectDef, CostQuantityDef, CounterKind,
    DiscardSelectionDef, EffectChoiceDef, EffectDef, EffectRecipientDef, IfNoObjectsDef,
    KeywordAbility, LikelihoodDef, ManaColor, MillUntilDef, MoveObjectsDef, ObjectChoiceBindingDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetCountConditionDef, ObjectSetDef,
    ObjectSetPredicateDef, PerPlayerSelectionDef, PlayActionMatcherDef, PlayRestrictionDef,
    PlayerRefDef, PlayerRelation, PlayerSetDef, ReplacementChoiceDef, ReplacementEffectDef,
    ResolvedEffectDurationDef, RevealObjectsDef, ScaledValueDef, SpellAdditionalCostDef,
    StaticApplyDef, TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind,
    ZonePlacement, abilities,
};
use crate::ids::ParentBinding;
use crate::{TargetIndex, mana_cost};

// EXO 1 — Allay
pub(in crate::card::sets) static ALLAY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f20a1c6d-ec6a-4bd6-b3b2-b997f71d41fc"),
    "Allay",
    crate::card::CardArt::new("f20a1c6d-ec6a-4bd6-b3b2-b997f71d41fc", "Randy Gallegos"),
    crate::card::CardSet::Exodus,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[
        abilities::buyback(mana_cost!("{3}")),
        AbilityDef::destroy_target(
            "Destroy target enchantment.",
            &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                CardType::Enchantment,
            )),
            true,
        ),
    ]),
);

// EXO 2 — Angelic Blessing
pub(in crate::card::sets) static ANGELIC_BLESSING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("31dda640-2a00-437e-855f-173c487e7395"),
    "Angelic Blessing",
    crate::card::CardArt::new("ed3c8bae-953f-4bb4-a78d-02e4e354e53c", "Mark Zug"),
    crate::card::CardSet::Exodus,
    CardRules::new_sorcery(mana_cost!("{2}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +3/+3 and gains flying until end of turn.",
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
                AppliedEffectDef::add_ability(&abilities::flying()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// EXO 3 — Cataclysm
// Audit: unsupported — Needs each player to make overlapping survivor choices
// across four permanent types before one simultaneous sacrifice.
pub(in crate::card::sets) static CATACLYSM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("024ae668-a1ae-4020-89c8-acbd8bd0a691"),
    "Cataclysm",
    crate::card::CardArt::new("024ae668-a1ae-4020-89c8-acbd8bd0a691", "Jim Nelson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 4 — Charging Paladin
pub(in crate::card::sets) static CHARGING_PALADIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("29db1bbf-a6cf-460c-bec8-dbd682157af4"),
    "Charging Paladin",
    crate::card::CardArt::new("851f3f72-2923-4432-898a-02679a8b320f", "Ciruelo"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Knight"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever this creature attacks, it gets +0/+3 until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(3),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// EXO 5 — Convalescence
pub(in crate::card::sets) static CONVALESCENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0fd49a61-42ba-400a-8ca9-9f6058bf85ca"),
    "Convalescence",
    crate::card::CardArt::new(
        "0fd49a61-42ba-400a-8ca9-9f6058bf85ca",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Exodus,
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_ability(AbilityDef::triggered_if(
        "At the beginning of your upkeep, if you have 10 or less life, you gain 1 life.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::You,
        },
        &TriggerConditionDef::ControllerLifeAtMost(10),
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )),
);

// EXO 6 — Exalted Dragon
// Audit: unsupported — Needs sacrificing a land as a declaration-time attack cost;
// ordinary activated and spell costs do not run while attackers are declared.
pub(in crate::card::sets) static EXALTED_DRAGON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7537bab3-4bac-4b83-9ad3-dfcb4ff19d6d"),
    "Exalted Dragon",
    crate::card::CardArt::new("7537bab3-4bac-4b83-9ad3-dfcb4ff19d6d", "Matthew D. Wilson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 7 — High Ground
pub(in crate::card::sets) static HIGH_GROUND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1c5239dc-f51b-48c0-91a2-ed6551aaff32"),
    "High Ground",
    crate::card::CardArt::new("1c5239dc-f51b-48c0-91a2-ed6551aaff32", "rk post"),
    crate::card::CardSet::Exodus,
    CardRules::new_enchantment(mana_cost!("{W}")).with_ability(AbilityDef::static_ability(
        "Each creature you control can block an additional creature each combat.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::MayBlockAdditionalCreatures(1)),
        },
    )),
);

// EXO 8 — Keeper of the Light
// Audit: unsupported — Needs a target-player predicate comparing live life totals.
pub(in crate::card::sets) static KEEPER_OF_THE_LIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("06eda847-c599-4163-b48b-aa76b153ed86"),
    "Keeper of the Light",
    crate::card::CardArt::new(
        "06eda847-c599-4163-b48b-aa76b153ed86",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 9 — Kor Chant
// Audit: unsupported — Needs a damage-redirection shield keyed to a source chosen
// as the spell resolves; current shields cannot bind that source choice.
pub(in crate::card::sets) static KOR_CHANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8dc61cc3-0312-44f4-9c23-4fc37c3fbbd5"),
    "Kor Chant",
    crate::card::CardArt::new("8dc61cc3-0312-44f4-9c23-4fc37c3fbbd5", "John Matson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 10 — Limited Resources
pub(in crate::card::sets) static LIMITED_RESOURCES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("20ae3609-a3cc-486c-94f6-b8f647adfb47"),
    "Limited Resources",
    crate::card::CardArt::new("20ae3609-a3cc-486c-94f6-b8f647adfb47", "Keith Parkinson"),
    crate::card::CardSet::Exodus,
    CardRules::new_enchantment(mana_cost!("{W}")).with_abilities(&[
        abilities::enters_trigger(
            "When this enchantment enters, each player chooses five lands they control and sacrifices the rest.",
            EffectDef::ChooseForEachPlayer(ChooseForEachPlayerDef {
                player: EffectRecipientDef::EachPlayer,
                candidates: ObjectPredicateDef::HasType(CardType::Land),
                zone: ZoneKind::Battlefield,
                selection: PerPlayerSelectionDef::Count(ValueDef::Constant(5)),
                visibility: ChoiceVisibilityDef::Public,
                chosen: Binding!("limited_resources_lands_kept"),
                unchosen: Binding!("limited_resources_lands_sacrificed"),
                then: &EffectDef::Sacrifice {
                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!(
                        "limited_resources_lands_sacrificed"
                    ))),
                },
            }),
        ),
        AbilityDef::static_ability(
            "Players can't play lands as long as ten or more lands are on the battlefield.",
            EffectDef::ConditionalStatic(ConditionalStaticEffectDef {
                condition: ObjectSetCountConditionDef {
                    objects: &ObjectSetDef::Query(ObjectQueryDef::new(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                    )),
                    predicate: ObjectSetPredicateDef {
                        filter: None,
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 10,
                    },
                },
                then: StaticApplyDef {
                    recipient: EffectRecipientDef::EachPlayer,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(
                        PlayRestrictionDef::new(
                            PlayActionMatcherDef::PlayLand,
                            ObjectPredicateDef::Any,
                        ),
                    )),
                },
            }),
        ),
    ]),
);

// EXO 11 — Oath of Lieges
pub(in crate::card::sets) static OATH_OF_LIEGES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("470a2092-eeda-4557-8cee-ac401b61a225"),
    "Oath of Lieges",
    crate::card::CardArt::new("470a2092-eeda-4557-8cee-ac401b61a225", "Mark Zug"),
    crate::card::CardSet::Exodus,
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_ability(
        AbilityDef::triggered_with_targets(
            "At the beginning of each player's upkeep, that player chooses target player who controls more lands than they do and is their opponent. The first player may search their library for a basic land card, put that card onto the battlefield, then shuffle.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::Any,
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerWithMoreObjectsThanChooser {
                    relation: PlayerRelation::Opponent,
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    zones: &[ZoneKind::Battlefield],
                },
            )
            .chosen_by_event_player()],
            EffectDef::SearchZone {
                player: EffectRecipientDef::EventPlayer,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::Supertype(CardSupertype::Basic),
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
        ),
    ),
);

// EXO 12 — Paladin en-Vec
pub(in crate::card::sets) static PALADIN_EN_VEC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bf1ea89d-4b9d-455f-a7f4-a26026e0c272"),
    "Paladin en-Vec",
    crate::card::CardArt::new("bf1ea89d-4b9d-455f-a7f4-a26026e0c272", "Randy Elliott"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Human", "Knight"], 2, 2).with_abilities(&[
        abilities::first_strike(),
        abilities::protection_from_color(ManaColor::Black),
        abilities::protection_from_color(ManaColor::Red),
    ]),
);

// EXO 13 — Peace of Mind
pub(in crate::card::sets) static PEACE_OF_MIND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c383f12f-da06-4ef0-bf8e-6a8a9cfcc74c"),
    "Peace of Mind",
    crate::card::CardArt::new("c383f12f-da06-4ef0-bf8e-6a8a9cfcc74c", "Randy Elliott"),
    crate::card::CardSet::Exodus,
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_ability(AbilityDef::activated(
        "{W}, Discard a card: You gain 3 life.",
        &[
            AbilityCostDef::Mana(mana_cost!("{W}")),
            AbilityCostDef::DiscardCardMatching(ObjectPredicateDef::Any),
        ],
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(3),
        },
    )),
);

// EXO 14 — Pegasus Stampede
pub(in crate::card::sets) static PEGASUS_STAMPEDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3b941576-8254-4d69-85ae-c748c7921ce5"),
    "Pegasus Stampede",
    crate::card::CardArt::new("3b941576-8254-4d69-85ae-c748c7921ce5", "Mark Zug"),
    crate::card::CardSet::Exodus,
    CardRules::new_sorcery(mana_cost!("{1}{W}")).with_abilities(&[
        abilities::buyback_with_additional_cost(
            "Buyback—Sacrifice a land. (You may sacrifice a land in addition to any other costs as you cast this spell. If you do, put this card into your hand as it resolves.)",
            &SpellAdditionalCostDef::sacrifice(
                ObjectPredicateDef::HasType(CardType::Land),
                CostQuantityDef::Fixed(1),
            ),
        ),
        AbilityDef::spell(
            "Create a 1/1 white Pegasus creature token with flying.",
            EffectDef::CreateToken {
                token: crate::card::TokenCharacteristics::creature(
                    &["Pegasus"],
                    &[ManaColor::White],
                    1,
                    1,
                )
                .with_abilities(&[abilities::flying()]),
                copy: None,
                controller: None,
                count: ValueDef::Constant(1),
                tapped: false,
                attacking: false,
                counters: None,
                created: None,
            },
        ),
    ]),
);

// EXO 15 — Penance
// Audit: unsupported — Needs hand-to-library placement as an atomic activated cost
// and a one-event prevention shield keyed to a source chosen on resolution.
pub(in crate::card::sets) static PENANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1f3db848-8394-43bd-a236-264641033a6d"),
    "Penance",
    crate::card::CardArt::new("1f3db848-8394-43bd-a236-264641033a6d", "Terese Nielsen"),
    crate::card::CardSet::Exodus,
    CardRules::unsupported(),
);

// EXO 16 — Reaping the Rewards
pub(in crate::card::sets) static REAPING_THE_REWARDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("379b0495-8795-4b21-9d0a-dc4e10098de2"),
    "Reaping the Rewards",
    crate::card::CardArt::new("379b0495-8795-4b21-9d0a-dc4e10098de2", "Heather Hudson"),
    crate::card::CardSet::Exodus,
    CardRules::new_instant(mana_cost!("{W}")).with_abilities(&[
        abilities::buyback_with_additional_cost(
            "Buyback—Sacrifice a land. (You may sacrifice a land in addition to any other costs as you cast this spell. If you do, put this card into your hand as it resolves.)",
            &SpellAdditionalCostDef::sacrifice(
                ObjectPredicateDef::HasType(CardType::Land),
                CostQuantityDef::Fixed(1),
            ),
        ),
        AbilityDef::spell(
            "You gain 2 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// EXO 17 — Reconnaissance
pub(in crate::card::sets) static RECONNAISSANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a16012d8-703c-4385-8769-13e3caba3fc6"),
    "Reconnaissance",
    crate::card::CardArt::new("a16012d8-703c-4385-8769-13e3caba3fc6", "Val Mayerik"),
    crate::card::CardSet::Exodus,
    CardRules::new_enchantment(mana_cost!("{W}")).with_ability(AbilityDef::activated_with_targets(
        "{0}: Remove target attacking creature you control from combat and untap it.",
        &[AbilityCostDef::Mana(mana_cost!("{0}"))],
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Attacking,
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::RemoveFromCombat {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ]),
    )),
);

// EXO 18 — Shackles
pub(in crate::card::sets) static SHACKLES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c5315668-b8ef-49ab-a8f5-144adc7bcd84"),
    "Shackles",
    crate::card::CardArt::new("c5315668-b8ef-49ab-a8f5-144adc7bcd84", "Heather Hudson"),
    crate::card::CardSet::Exodus,
    CardRules::new_enchantment(mana_cost!("{2}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature doesn't untap during its controller's untap step.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
                },
            ),
            AbilityDef::activated(
                "{W}: Return this Aura to its owner's hand.",
                &[AbilityCostDef::Mana(mana_cost!("{W}"))],
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Source,
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
            ),
        ]),
);

// EXO 19 — Shield Mate
pub(in crate::card::sets) static SHIELD_MATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b49261bb-66b5-4226-9001-02d045fbcbce"),
    "Shield Mate",
    crate::card::CardArt::new("b49261bb-66b5-4226-9001-02d045fbcbce", "Randy Elliott"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "Sacrifice this creature: Target creature gets +0/+4 until end of turn.",
            &[AbilityCostDef::SacrificeSource],
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
    ),
);

// EXO 20 — Soltari Visionary
pub(in crate::card::sets) static SOLTARI_VISIONARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1a3ae384-7b60-4264-9dc1-1613917168ca"),
    "Soltari Visionary",
    crate::card::CardArt::new("1a3ae384-7b60-4264-9dc1-1613917168ca", "Adam Rex"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Soltari", "Cleric"], 2, 2)
        .with_abilities(&[
            abilities::shadow(),
            AbilityDef::triggered_with_targets(
                "Whenever this creature deals damage to a player, destroy target enchantment that player controls.",
                TriggerEventDef::damage_to_player(
                    ObjectPredicateDef::Source,
                    PlayerRelation::Any,
                ),
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::EventPlayer),
                    ]),
                )],
                EffectDef::destroy_target(TargetIndex::PRIMARY, true),
            ),
        ]),
);

// EXO 21 — Soul Warden
pub(in crate::card::sets) static SOUL_WARDEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d5ee24ee-4d28-4634-bd43-90eff15c16dd"),
    "Soul Warden",
    crate::card::CardArt::new("d5ee24ee-4d28-4634-bd43-90eff15c16dd", "Randy Gallegos"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Cleric"], 1, 1).with_ability(
        AbilityDef::triggered(
            "Whenever another creature enters, you gain 1 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// EXO 22 — Standing Troops
pub(in crate::card::sets) static STANDING_TROOPS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("135e258a-71d8-45dd-9307-91111aa34bde"),
    "Standing Troops",
    crate::card::CardArt::new("135e258a-71d8-45dd-9307-91111aa34bde", "Daren Bader"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 1, 4)
        .with_ability(abilities::vigilance()),
);

// EXO 23 — Treasure Hunter
pub(in crate::card::sets) static TREASURE_HUNTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("06452630-621b-498e-8f25-ecfe544d4213"),
    "Treasure Hunter",
    crate::card::CardArt::new("06452630-621b-498e-8f25-ecfe544d4213", "Adam Rex"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human"], 2, 2).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, you may return target artifact card from your graveyard to your hand.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Artifact),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
                1,
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
    ),
);

// EXO 24 — Wall of Nets
pub(in crate::card::sets) static WALL_OF_NETS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c1da8e79-365d-4a36-87c5-648085828f9f"),
    "Wall of Nets",
    crate::card::CardArt::new("c1da8e79-365d-4a36-87c5-648085828f9f", "Terese Nielsen"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Wall"], 0, 7).with_abilities(&[
        abilities::defender(),
        AbilityDef::triggered(
            "At end of combat, exile all creatures blocked by this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::EndOfCombat,
                player: PlayerRelation::Any,
            },
            EffectDef::ExileLinkedToSource {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::BlockingSource,
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                face_down: false,
                until_source_leaves: false,
                then: None,
            },
        ),
        AbilityDef::triggered(
            "When this creature leaves the battlefield, return all cards exiled with it to the battlefield under their owners' control.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                None,
            ),
            EffectDef::ReturnLinkedExiles {
                object: ObjectPredicateDef::Any,
                counters: None,
                zone: ZoneKind::Battlefield,
                grant: None,
                controller: None,
                transformed: false,
            },
        ),
    ]),
);

// EXO 25 — Welkin Hawk
pub(in crate::card::sets) static WELKIN_HAWK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8166253c-c6ac-4b5e-9746-09ce3774c66b"),
    "Welkin Hawk",
    crate::card::CardArt::new("8166253c-c6ac-4b5e-9746-09ce3774c66b", "Rob Alexander"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Bird"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::dies_trigger(
            "When this creature dies, you may search your library for a card named Welkin Hawk, reveal that card, put it into your hand, then shuffle.",
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::HasName(ObjectRefDef::Source),
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

// EXO 26 — Zealots en-Dal
pub(in crate::card::sets) static ZEALOTS_EN_DAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6a9fb486-1d6a-478e-af6e-fd8539dc646d"),
    "Zealots en-Dal",
    crate::card::CardArt::new("6a9fb486-1d6a-478e-af6e-fd8539dc646d", "Brom"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Soldier"], 2, 4).with_ability(
        AbilityDef::triggered_if(
            "At the beginning of your upkeep, if all nonland permanents you control are white, you gain 1 life.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::Not(&TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::controlled_by(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::White)),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerSetDef::Related(PlayerRelation::You),
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            }),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// EXO 27 — Aether Tide
pub(in crate::card::sets) static AETHER_TIDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9aab7526-5825-4f31-92ff-be25ab5af2f5"),
    "Aether Tide",
    crate::card::CardArt::new("9aab7526-5825-4f31-92ff-be25ab5af2f5", "Andrew Robinson"),
    crate::card::CardSet::Exodus,
    CardRules::new_sorcery(mana_cost!("{X}{U}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, discard X creature cards.\nReturn X target creatures to their owners' hands.",
            &[AbilityTargetDef::exactly_chosen_x(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            SpellAdditionalCostDef::discard(
                ObjectPredicateDef::HasType(CardType::Creature),
                CostQuantityDef::ChosenX,
            ),
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
    ),
);

// EXO 28 — Cunning
// Audit: unsupported — Needs an attack-or-block event whose subject is the permanent
// enchanted by the source Aura, followed by a source-linked cleanup-step trigger.
pub(in crate::card::sets) static CUNNING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("52f36bb8-5a97-4596-8ca3-707665770c76"),
    "Cunning",
    crate::card::CardArt::new("52f36bb8-5a97-4596-8ca3-707665770c76", "Kev Walker"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 29 — Curiosity (reprint)

// EXO 30 — Dominating Licid
// Audit: unsupported — Licid animation needs a reversible creature-to-Aura type
// change, attachment, ability loss, and special action to end the effect.
pub(in crate::card::sets) static DOMINATING_LICID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e3e03323-43e8-4ddc-a874-211a97fd7648"),
    "Dominating Licid",
    crate::card::CardArt::new("e3e03323-43e8-4ddc-a874-211a97fd7648", "Heather Hudson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 31 — Ephemeron
pub(in crate::card::sets) static EPHEMERON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f2cdcd3b-6df5-481a-a244-1fc2545d1356"),
    "Ephemeron",
    crate::card::CardArt::new("f2cdcd3b-6df5-481a-a244-1fc2545d1356", "Keith Parkinson"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{4}{U}{U}"), &["Illusion"], 4, 4).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "Discard a card: Return this creature to its owner's hand.",
            &[AbilityCostDef::DiscardCardMatching(ObjectPredicateDef::Any)],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Source,
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// EXO 32 — Equilibrium
pub(in crate::card::sets) static EQUILIBRIUM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("460b2ec6-0180-4214-acca-c9eed778ef50"),
    "Equilibrium",
    crate::card::CardArt::new("460b2ec6-0180-4214-acca-c9eed778ef50", "Jeff Miracola"),
    crate::card::CardSet::Exodus,
    CardRules::new_enchantment(mana_cost!("{1}{U}{U}")).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever you cast a creature spell, you may pay {1}. If you do, return target creature to its owner's hand.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::PayOr(crate::card::PayOrDef::optional(
                crate::card::EffectPaymentDef::mana(
                    PlayerSetDef::Related(PlayerRelation::You),
                    mana_cost!("{1}"),
                ),
                &EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
            )),
        ),
    ),
);

// EXO 33 — Ertai, Wizard Adept
pub(in crate::card::sets) static ERTAI_WIZARD_ADEPT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("91971e19-61ce-45ac-b700-9ffca5091a27"),
    "Ertai, Wizard Adept",
    crate::card::CardArt::new("91971e19-61ce-45ac-b700-9ffca5091a27", "Terese Nielsen"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Wizard"], 1, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::activated_with_targets(
            "{2}{U}{U}, {T}: Counter target spell.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{U}{U}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Spell,
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::counter_target(TargetIndex::PRIMARY),
        )),
);

// EXO 34 — Fade Away
// Audit: unsupported — Needs a per-creature loop in which each current controller
// chooses between paying mana and sacrificing one of their permanents.
pub(in crate::card::sets) static FADE_AWAY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a6f9103e-dcc2-4f7a-a8ca-eaa831f5f83b"),
    "Fade Away",
    crate::card::CardArt::new("a6f9103e-dcc2-4f7a-a8ca-eaa831f5f83b", "Jeff Miracola"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 35 — Forbid
pub(in crate::card::sets) static FORBID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("29df5ef7-d679-4543-bdb7-3984155c87e0"),
    "Forbid",
    crate::card::CardArt::new("29df5ef7-d679-4543-bdb7-3984155c87e0", "Scott Kirschner"),
    crate::card::CardSet::Exodus,
    CardRules::new_instant(mana_cost!("{1}{U}{U}")).with_abilities(&[
        abilities::buyback_with_additional_cost(
            "Buyback—Discard two cards. (You may discard two cards in addition to any other costs as you cast this spell. If you do, put this card into your hand as it resolves.)",
            &SpellAdditionalCostDef::discard(
                ObjectPredicateDef::Any,
                CostQuantityDef::Fixed(2),
            ),
        ),
        AbilityDef::counter_target(
            "Counter target spell.",
            &AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            }),
        ),
    ]),
);

// EXO 36 — Keeper of the Mind
// Audit: unsupported — The target comparison needs a two-card hand-size margin;
// the existing player-object comparator expresses only strictly more objects.
pub(in crate::card::sets) static KEEPER_OF_THE_MIND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7bc232d4-ab4f-4d88-a9ec-72403d05ec04"),
    "Keeper of the Mind",
    crate::card::CardArt::new("7bc232d4-ab4f-4d88-a9ec-72403d05ec04", "Matthew D. Wilson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 37 — Killer Whale
pub(in crate::card::sets) static KILLER_WHALE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d932f6d3-4918-4a41-836c-4eaa6cfac049"),
    "Killer Whale",
    crate::card::CardArt::new("d932f6d3-4918-4a41-836c-4eaa6cfac049", "Stephen Daniele"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{3}{U}{U}"), &["Whale"], 3, 5).with_ability(
        AbilityDef::activated(
            "{U}: This creature gains flying until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::flying()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// EXO 38 — Mana Breach
pub(in crate::card::sets) static MANA_BREACH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3a97f019-5ad9-4520-ba79-2c9b259748d9"),
    "Mana Breach",
    crate::card::CardArt::new("3a97f019-5ad9-4520-ba79-2c9b259748d9", "Rebecca Guay"),
    crate::card::CardSet::Exodus,
    CardRules::new_enchantment(mana_cost!("{2}{U}")).with_ability(AbilityDef::triggered(
        "Whenever a player casts a spell, that player returns a land they control to its owner's hand.",
        TriggerEventDef::spell_cast(ObjectPredicateDef::Spell),
        EffectDef::Choose(ChooseDef {
            binding: ObjectChoiceBindingDef::Object(ParentBinding),
            unchosen: None,
            chooser: PlayerRefDef::EventPlayer,
            candidates: ObjectSetDef::Query(ObjectQueryDef::controlled_by(
                ObjectPredicateDef::HasType(CardType::Land),
                &[ZoneKind::Battlefield],
                PlayerSetDef::One(PlayerRefDef::EventPlayer),
            )),
            exclude: None,
            minimum: 1,
            maximum: 1,
            visibility: ChoiceVisibilityDef::Public,
            then: &EffectDef::MoveToZone {
                object: EffectRecipientDef::object(ObjectRefDef::Binding(ParentBinding)),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        }),
    )),
);

// EXO 39 — Merfolk Looter (reprint)

// EXO 40 — Mind Over Matter
pub(in crate::card::sets) static MIND_OVER_MATTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6e091dd6-149f-46ea-bae0-224e79e3aacb"),
    "Mind Over Matter",
    crate::card::CardArt::new("6e091dd6-149f-46ea-bae0-224e79e3aacb", "Keith Parkinson"),
    crate::card::CardSet::Exodus,
    CardRules::new_enchantment(mana_cost!("{2}{U}{U}{U}{U}")).with_ability(
        AbilityDef::activated_with_targets(
            "Discard a card: You may tap or untap target artifact, creature, or land.",
            &[AbilityCostDef::DiscardCardMatching(ObjectPredicateDef::Any)],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Land),
                ]),
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::ChooseEffect {
                    player: EffectRecipientDef::Controller,
                    choices: &[
                        EffectChoiceDef {
                            label: "Tap it",
                            effect: EffectDef::Tap {
                                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            },
                        },
                        EffectChoiceDef {
                            label: "Untap it",
                            effect: EffectDef::Untap {
                                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            },
                        },
                    ],
                },
            },
        ),
    ),
);

// EXO 41 — Mirozel
pub(in crate::card::sets) static MIROZEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("16417e94-e33f-4ed4-bb3e-52f29f7d441b"),
    "Mirozel",
    crate::card::CardArt::new("16417e94-e33f-4ed4-bb3e-52f29f7d441b", "Jim Nelson"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Illusion"], 2, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "When this creature becomes the target of a spell or ability, return this creature to its owner's hand.",
            TriggerEventDef::BecomesTargetOfSpellOrAbility(ObjectPredicateDef::Any),
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Source,
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// EXO 42 — Oath of Scholars
pub(in crate::card::sets) static OATH_OF_SCHOLARS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d61376ad-21c8-4d34-b37d-ed60877f5d4a"),
    "Oath of Scholars",
    crate::card::CardArt::new("d61376ad-21c8-4d34-b37d-ed60877f5d4a", "Michael Sutfin"),
    crate::card::CardSet::Exodus,
    CardRules::new_enchantment(mana_cost!("{3}{U}")).with_ability(
        AbilityDef::triggered_with_targets(
            "At the beginning of each player's upkeep, that player chooses target player who has more cards in hand than they do and is their opponent. The first player may discard their hand and draw three cards.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::Any,
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerWithMoreObjectsThanChooser {
                    relation: PlayerRelation::Opponent,
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Hand],
                },
            )
            .chosen_by_event_player()],
            EffectDef::May {
                player: EffectRecipientDef::EventPlayer,
                effect: &EffectDef::Sequence(&[
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::EventPlayer,
                        amount: ValueDef::Constant(i32::MAX),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: None,
                    },
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::EventPlayer,
                        amount: ValueDef::Constant(3),
                    },
                ]),
            },
        ),
    ),
);

// EXO 43 — Robe of Mirrors
pub(in crate::card::sets) static ROBE_OF_MIRRORS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("371720a2-ec3f-43a5-9551-c018e164e79f"),
    "Robe of Mirrors",
    crate::card::CardArt::new("371720a2-ec3f-43a5-9551-c018e164e79f", "John Matson"),
    crate::card::CardSet::Exodus,
    CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
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

// EXO 44 — Rootwater Mystic
pub(in crate::card::sets) static ROOTWATER_MYSTIC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("94af81f2-383c-4129-b8dc-60633c3f4ea1"),
    "Rootwater Mystic",
    crate::card::CardArt::new("94af81f2-383c-4129-b8dc-60633c3f4ea1", "Michael Sutfin"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{U}"), &["Merfolk", "Wizard"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{1}{U}: Look at the top card of target player's library.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{U}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            abilities::look_at_top_cards(
                PlayerRefDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
        ),
    ),
);

// EXO 45 — School of Piranha
pub(in crate::card::sets) static SCHOOL_OF_PIRANHA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("71217af5-3538-4e42-9343-3949b5306671"),
    "School of Piranha",
    crate::card::CardArt::new("71217af5-3538-4e42-9343-3949b5306671", "Daren Bader"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Fish"], 3, 3).with_ability(
        AbilityDef::triggered(
            "At the beginning of your upkeep, sacrifice this creature unless you pay {1}{U}.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::PayOr(crate::card::PayOrDef::unless_mana(
                mana_cost!("{1}{U}"),
                &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            )),
        ),
    ),
);

// EXO 46 — Scrivener
pub(in crate::card::sets) static SCRIVENER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8b236bba-160a-4637-a83e-8456834ce59f"),
    "Scrivener",
    crate::card::CardArt::new("8b236bba-160a-4637-a83e-8456834ce59f", "Heather Hudson"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Human", "Wizard"], 2, 2).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, you may return target instant card from your graveyard to your hand.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Instant),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
                1,
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
    ),
);

// EXO 47 — Thalakos Drifters
pub(in crate::card::sets) static THALAKOS_DRIFTERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("468e13d2-6bd7-403c-8e2e-e00917b39597"),
    "Thalakos Drifters",
    crate::card::CardArt::new("468e13d2-6bd7-403c-8e2e-e00917b39597", "Andrew Robinson"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Thalakos"], 3, 3).with_ability(
        AbilityDef::activated(
            "Discard a card: This creature gains shadow until end of turn.",
            &[AbilityCostDef::DiscardCardMatching(ObjectPredicateDef::Any)],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::shadow()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// EXO 48 — Thalakos Scout
pub(in crate::card::sets) static THALAKOS_SCOUT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1703fe9d-ca70-4e8a-9d6a-6173a17d0f04"),
    "Thalakos Scout",
    crate::card::CardArt::new("1703fe9d-ca70-4e8a-9d6a-6173a17d0f04", "John Matson"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(
        mana_cost!("{2}{U}"),
        &["Thalakos", "Soldier", "Scout"],
        2,
        1,
    )
    .with_abilities(&[
        abilities::shadow(),
        AbilityDef::activated(
            "Discard a card: Return this creature to its owner's hand.",
            &[AbilityCostDef::DiscardCardMatching(ObjectPredicateDef::Any)],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Source,
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// EXO 49 — Theft of Dreams
pub(in crate::card::sets) static THEFT_OF_DREAMS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("29019e28-4ef8-4732-9972-0a47305fe303"),
    "Theft of Dreams",
    crate::card::CardArt::new(
        "099da8aa-16b1-4395-8467-1636feb14a8a",
        "Richard Kane Ferguson",
    ),
    crate::card::CardSet::Exodus,
    CardRules::new_sorcery(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Draw a card for each tapped creature target opponent controls.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Opponent),
        )],
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::controlled_by(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Tapped,
                ]),
                &[ZoneKind::Battlefield],
                PlayerSetDef::LegalTargets(TargetIndex::PRIMARY),
            )),
        },
    )),
);

// EXO 50 — Treasure Trove
pub(in crate::card::sets) static TREASURE_TROVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f23ce909-e744-47ca-943d-62d97e97b1ea"),
    "Treasure Trove",
    crate::card::CardArt::new("f23ce909-e744-47ca-943d-62d97e97b1ea", "Michael Sutfin"),
    crate::card::CardSet::Exodus,
    CardRules::new_enchantment(mana_cost!("{2}{U}{U}")).with_ability(AbilityDef::activated(
        "{2}{U}{U}: Draw a card.",
        &[AbilityCostDef::Mana(mana_cost!("{2}{U}{U}"))],
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )),
);

// EXO 51 — Wayward Soul
pub(in crate::card::sets) static WAYWARD_SOUL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("28f96d5d-1d16-40bb-aaa7-8a7dd465d37b"),
    "Wayward Soul",
    crate::card::CardArt::new(
        "28f96d5d-1d16-40bb-aaa7-8a7dd465d37b",
        "M. W. Kaluta & DiTerlizzi",
    ),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Spirit"], 3, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{U}: Put this creature on top of its owner's library.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Source,
                zone: ZoneKind::Library,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// EXO 52 — Whiptongue Frog
pub(in crate::card::sets) static WHIPTONGUE_FROG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6fc17186-e786-46a3-9812-4a6e367e78b9"),
    "Whiptongue Frog",
    crate::card::CardArt::new("6fc17186-e786-46a3-9812-4a6e367e78b9", "Jeff Miracola"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Frog"], 1, 3).with_ability(
        AbilityDef::activated(
            "{U}: This creature gains flying until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::flying()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// EXO 53 — Carnophage
pub(in crate::card::sets) static CARNOPHAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d17c057f-cb1b-4895-831a-fb35c75d3845"),
    "Carnophage",
    crate::card::CardArt::new("d17c057f-cb1b-4895-831a-fb35c75d3845", "Pete Venters"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{B}"), &["Zombie"], 2, 2).with_ability(
        AbilityDef::triggered(
            "At the beginning of your upkeep, tap this creature unless you pay 1 life.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::PayOr(crate::card::PayOrDef::unless(
                crate::card::EffectPaymentDef::life(PlayerSetDef::Related(PlayerRelation::You), 1),
                &EffectDef::Tap {
                    object: EffectRecipientDef::Source,
                },
            )),
        ),
    ),
);

// EXO 54 — Cat Burglar
pub(in crate::card::sets) static CAT_BURGLAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("947109f9-7035-4a2a-bbc2-a2958f8c5d01"),
    "Cat Burglar",
    crate::card::CardArt::new("947109f9-7035-4a2a-bbc2-a2958f8c5d01", "DiTerlizzi"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Kor", "Rogue", "Minion"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{2}{B}, {T}: Target player discards a card. Activate only as a sorcery.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{B}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Discard {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ),
);

// EXO 55 — Culling the Weak
pub(in crate::card::sets) static CULLING_THE_WEAK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("50c33f18-0a5c-4e46-ab0d-6e450915594f"),
    "Culling the Weak",
    crate::card::CardArt::new("50c33f18-0a5c-4e46-ab0d-6e450915594f", "Scott M. Fischer"),
    crate::card::CardSet::Exodus,
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_additional_cost(
        "As an additional cost to cast this spell, sacrifice a creature.\nAdd {B}{B}{B}{B}.",
        &[],
        SpellAdditionalCostDef::sacrifice(
            ObjectPredicateDef::HasType(CardType::Creature),
            CostQuantityDef::Fixed(1),
        ),
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black).with_amount(4)),
    )),
);

// EXO 56 — Cursed Flesh
pub(in crate::card::sets) static CURSED_FLESH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7433b9bf-ee6e-41fe-b826-0d20584198b1"),
    "Cursed Flesh",
    crate::card::CardArt::new("7433b9bf-ee6e-41fe-b826-0d20584198b1", "Ron Spencer"),
    crate::card::CardSet::Exodus,
    CardRules::new_enchantment(mana_cost!("{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets -1/-1 and has fear.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(-1),
                            ValueDef::Constant(-1),
                        ),
                        AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(
                            BlockRestrictionDef::prohibit(
                                BlockRestrictionSubjectDef::Attacker,
                                BlockRestrictionMatchDef::Except(ObjectPredicateDef::AnyOf(&[
                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                    ObjectPredicateDef::Color(ManaColor::Black),
                                ])),
                            ),
                        )),
                    ]),
                },
            ),
        ]),
);

// EXO 57 — Dauthi Cutthroat
pub(in crate::card::sets) static DAUTHI_CUTTHROAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("127b8994-fff8-4500-8ab4-244eeb3ed110"),
    "Dauthi Cutthroat",
    crate::card::CardArt::new("127b8994-fff8-4500-8ab4-244eeb3ed110", "Dermot Power"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Dauthi", "Minion"], 1, 1).with_abilities(&[
        abilities::shadow(),
        AbilityDef::activated_with_targets(
            "{1}{B}, {T}: Destroy target creature with shadow.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{B}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasKeyword(KeywordAbility::Shadow),
                ]),
            )],
            EffectDef::destroy_target(TargetIndex::PRIMARY, true),
        ),
    ]),
);

// EXO 58 — Dauthi Jackal
pub(in crate::card::sets) static DAUTHI_JACKAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("419871bc-f036-4244-8b6c-3857ebe993f3"),
    "Dauthi Jackal",
    crate::card::CardArt::new("419871bc-f036-4244-8b6c-3857ebe993f3", "Adam Rex"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Dauthi", "Jackal"], 2, 1).with_abilities(&[
        abilities::shadow(),
        AbilityDef::activated_with_targets(
            "{B}{B}, Sacrifice this creature: Destroy target blocking creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{B}{B}")),
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Blocking,
                ]),
            )],
            EffectDef::destroy_target(TargetIndex::PRIMARY, true),
        ),
    ]),
);

// EXO 59 — Dauthi Warlord
pub(in crate::card::sets) static DAUTHI_WARLORD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("af3ca689-482a-457d-9744-0bd79981f361"),
    "Dauthi Warlord",
    crate::card::CardArt::new("af3ca689-482a-457d-9744-0bd79981f361", "Kev Walker"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Dauthi", "Soldier"], 0, 1).with_abilities(&[
        abilities::shadow(),
        AbilityDef::static_ability(
            "Dauthi Warlord's power is equal to the number of creatures on the battlefield with shadow.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::define_power(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::new(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasKeyword(KeywordAbility::Shadow),
                        ]),
                        &[ZoneKind::Battlefield],
                    )),
                ),
            },
        ),
    ]),
);

// EXO 60 — Death's Duet
pub(in crate::card::sets) static DEATH_S_DUET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4756b6fd-2bb2-4be1-9b02-851a26ff4303"),
    "Death's Duet",
    crate::card::CardArt::new("4756b6fd-2bb2-4be1-9b02-851a26ff4303", "Keith Parkinson"),
    crate::card::CardSet::Exodus,
    CardRules::new_sorcery(mana_cost!("{2}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Return two target creature cards from your graveyard to your hand.",
        &[AbilityTargetDef::exactly_value(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
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

// EXO 61 — Entropic Specter
pub(in crate::card::sets) static ENTROPIC_SPECTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bdb04d81-b0ab-4bc7-935d-c31005887240"),
    "Entropic Specter",
    crate::card::CardArt::new("bdb04d81-b0ab-4bc7-935d-c31005887240", "Ron Spencer"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Specter", "Spirit"], 0, 0).with_abilities(&[
        abilities::flying(),
        AbilityDef::replacement(
            "As this creature enters, choose an opponent.",
            ReplacementEffectDef::Choose(ReplacementChoiceDef::Player(
                PlayerRelation::Opponent,
            )),
        ),
        AbilityDef::static_ability(
            "Entropic Specter's power and toughness are each equal to the number of cards in the chosen player's hand.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::define_power_toughness(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::owned_by(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Hand],
                        PlayerSetDef::Related(PlayerRelation::ChosenPlayer),
                    )),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::owned_by(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Hand],
                        PlayerSetDef::Related(PlayerRelation::ChosenPlayer),
                    )),
                ),
            },
        ),
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

// EXO 62 — Fugue
pub(in crate::card::sets) static FUGUE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1629cd63-95aa-40b6-aa57-7fb88f569e59"),
    "Fugue",
    crate::card::CardArt::new("1629cd63-95aa-40b6-aa57-7fb88f569e59", "Randy Gallegos"),
    crate::card::CardSet::Exodus,
    CardRules::new_sorcery(mana_cost!("{3}{B}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target player discards three cards.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Discard {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(3),
            selection: DiscardSelectionDef::RecipientChooses,
            then: None,
        },
    )),
);

// EXO 63 — Grollub
pub(in crate::card::sets) static GROLLUB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("47f6301a-d581-4aaf-9993-3013323074aa"),
    "Grollub",
    crate::card::CardArt::new("47f6301a-d581-4aaf-9993-3013323074aa", "Chippy"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Beast"], 3, 3).with_ability(
        AbilityDef::triggered(
            "Whenever this creature is dealt damage, each opponent gains that much life.",
            TriggerEventDef::damage_to_source(),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::TriggerEventAmount,
            },
        ),
    ),
);

// EXO 64 — Hatred
pub(in crate::card::sets) static HATRED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2383a8d9-96fd-4f9a-bcf9-eb81fdb15ead"),
    "Hatred",
    crate::card::CardArt::new("2383a8d9-96fd-4f9a-bcf9-eb81fdb15ead", "Brom"),
    crate::card::CardSet::Exodus,
    CardRules::new_instant(mana_cost!("{3}{B}{B}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, pay X life.\nTarget creature gets +X/+0 until end of turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            SpellAdditionalCostDef::pay_life(CostQuantityDef::ChosenX),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::ChosenX,
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// EXO 65 — Keeper of the Dead
// Audit: unsupported — Needs a target-player predicate comparing creature-card
// counts in two graveyards with a margin of at least two.
pub(in crate::card::sets) static KEEPER_OF_THE_DEAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6b641171-35bc-4945-ada9-3ea28ea9fabf"),
    "Keeper of the Dead",
    crate::card::CardArt::new("6b641171-35bc-4945-ada9-3ea28ea9fabf", "Brom"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 66 — Mind Maggots
pub(in crate::card::sets) static MIND_MAGGOTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c3c92a7f-a250-4497-aa7a-0394e94ef13d"),
    "Mind Maggots",
    crate::card::CardArt::new("c3c92a7f-a250-4497-aa7a-0394e94ef13d", "Ron Spencer"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Insect"], 2, 2).with_ability(
        abilities::enters_trigger(
            "When this creature enters, discard any number of creature cards. For each card discarded this way, put two +1/+1 counters on this creature.",
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Hand],
                    PlayerSetDef::One(PlayerRefDef::EffectController),
                )),
                exclude: None,
                minimum: 0,
                maximum: usize::MAX,
                visibility: ChoiceVisibilityDef::Private,
                then: &EffectDef::Sequence(&[
                    EffectDef::DiscardCards {
                        object: EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                    },
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Scaled(&ScaledValueDef::new(
                            ValueDef::BoundObjectCount(ParentBinding),
                            2,
                        )),
                    },
                ]),
            }),
        ),
    ),
);

// EXO 67 — Nausea
pub(in crate::card::sets) static NAUSEA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a10531d8-fc99-4a2b-94b0-97a25521d725"),
    "Nausea",
    crate::card::CardArt::new("a10531d8-fc99-4a2b-94b0-97a25521d725", "Jeff Miracola"),
    crate::card::CardSet::Exodus,
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_ability(AbilityDef::spell(
        "All creatures get -1/-1 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-1),
                ValueDef::Constant(-1),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// EXO 68 — Necrologia
// Audit: unsupported — Needs a cast restriction for only the caster's end step;
// the X life payment and X-card draw are otherwise declarative.
pub(in crate::card::sets) static NECROLOGIA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8c2ee9d9-20be-46f0-8752-1df50942f59c"),
    "Necrologia",
    crate::card::CardArt::new("8c2ee9d9-20be-46f0-8752-1df50942f59c", "Brom"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 69 — Oath of Ghouls
// Audit: unsupported — Needs an event-player target predicate selecting an opponent
// with fewer matching objects in their graveyard than the chooser.
pub(in crate::card::sets) static OATH_OF_GHOULS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1102f35a-ae62-479d-b61c-31a82978aedd"),
    "Oath of Ghouls",
    crate::card::CardArt::new("1102f35a-ae62-479d-b61c-31a82978aedd", "Brom"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 70 — Pit Spawn
pub(in crate::card::sets) static PIT_SPAWN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("669ad60b-4053-4f07-9072-52e6ff65b4e3"),
    "Pit Spawn",
    crate::card::CardArt::new("669ad60b-4053-4f07-9072-52e6ff65b4e3", "Thomas M. Baxa"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{4}{B}{B}{B}"), &["Demon"], 6, 4).with_abilities(&[
        abilities::first_strike(),
        AbilityDef::triggered(
            "At the beginning of your upkeep, sacrifice this creature unless you pay {B}{B}.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::PayOr(crate::card::PayOrDef::unless_mana(
                mana_cost!("{B}{B}"),
                &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            )),
        ),
        AbilityDef::triggered(
            "Whenever this creature deals damage to a creature, exile that creature.",
            TriggerEventDef::DamageDealt(crate::card::DamageEventMatcherDef {
                recipient: crate::card::DamageRecipientMatcherDef::MatchingObject(
                    ObjectPredicateDef::HasType(CardType::Creature),
                ),
                ..crate::card::DamageEventMatcherDef::from(ObjectRefDef::Source)
            }),
            EffectDef::MoveToZone {
                object: EffectRecipientDef::DamagedObject,
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// EXO 71 — Plaguebearer
pub(in crate::card::sets) static PLAGUEBEARER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a8493df6-9954-4e33-867c-ca4bcf3953b2"),
    "Plaguebearer",
    crate::card::CardArt::new("a8493df6-9954-4e33-867c-ca4bcf3953b2", "Ron Spencer"),
    crate::card::CardSet::Exodus,
    // Audit: unsupported — The shared activated-ability runtime cannot pay a
    // variable-X mana cost, although its target predicate can express mana value X.
    CardRules::unsupported(),
);

// EXO 72 — Recurring Nightmare
pub(in crate::card::sets) static RECURRING_NIGHTMARE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c8173030-1c33-417c-b8e9-79231b6a85a7"),
    "Recurring Nightmare",
    crate::card::CardArt::new("c8173030-1c33-417c-b8e9-79231b6a85a7", "Jeff Laubenstein"),
    crate::card::CardSet::Exodus,
    // The target is chosen as the ability is activated and the costs are
    // paid afterwards, so the creature sacrificed to pay is not the one
    // coming back: what it buys is the creature that died a turn earlier.
    CardRules::new_enchantment(mana_cost!("{2}{B}")).with_ability(
        AbilityDef::activated_with_targets(
            "Sacrifice a creature, Return this enchantment to its owner's hand: Return target \
             creature card from your graveyard to the battlefield. Activate only as a sorcery.",
            // Both halves leave the battlefield to pay, and the enchantment's half is a
            // return rather than a sacrifice: it comes back to hand to be cast again,
            // which is the whole of why the card is banned wherever it is.
            &[
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    controller: PlayerRelation::You,
                },
                AbilityCostDef::ReturnSourceToHand,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
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
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ),
);

// EXO 73 — Scare Tactics
pub(in crate::card::sets) static SCARE_TACTICS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6a9d4e11-ce2e-445a-9536-756a6687d6d7"),
    "Scare Tactics",
    crate::card::CardArt::new("6a9d4e11-ce2e-445a-9536-756a6687d6d7", "DiTerlizzi"),
    crate::card::CardSet::Exodus,
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell(
        "Creatures you control get +1/+0 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(0),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// EXO 74 — Slaughter
pub(in crate::card::sets) static SLAUGHTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8ff06c7d-5e78-4bcf-864b-34487f6555b2"),
    "Slaughter",
    crate::card::CardArt::new("8ff06c7d-5e78-4bcf-864b-34487f6555b2", "Pete Venters"),
    crate::card::CardSet::Exodus,
    CardRules::new_instant(mana_cost!("{2}{B}{B}")).with_abilities(&[
        abilities::buyback_with_additional_cost(
            "Buyback—Pay 4 life. (You may pay 4 life in addition to any other costs as you cast this spell. If you do, put this card into your hand as it resolves.)",
            &SpellAdditionalCostDef::pay_life(CostQuantityDef::Fixed(4)),
        ),
        AbilityDef::destroy_target(
            "Destroy target nonblack creature. It can't be regenerated.",
            &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::Black)),
            ])),
            false,
        ),
    ]),
);

// EXO 75 — Spike Cannibal
// Audit: unsupported — Needs moving every +1/+1 counter from a frozen set of
// creatures onto the source while preserving the total removed.
pub(in crate::card::sets) static SPIKE_CANNIBAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("64003772-c62f-4728-a00c-48c78991c6ae"),
    "Spike Cannibal",
    crate::card::CardArt::new("64003772-c62f-4728-a00c-48c78991c6ae", "Joel Biske"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 76 — Thrull Surgeon
pub(in crate::card::sets) static THRULL_SURGEON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d6e89bf1-42c9-4829-a565-78cac632810b"),
    "Thrull Surgeon",
    crate::card::CardArt::new("d6e89bf1-42c9-4829-a565-78cac632810b", "rk post"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Thrull"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{1}{B}, Sacrifice this creature: Look at target player's hand and choose a card from it. That player discards that card. Activate only as a sorcery.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{B}")),
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Sequence(&[
                EffectDef::LookAtHand {
                    player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Object(ParentBinding),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Hand],
                        PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                    )),
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Private,
                    then: &EffectDef::DiscardCards {
                        object: EffectRecipientDef::object(ObjectRefDef::Binding(ParentBinding)),
                    },
                }),
            ]),
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ),
);

// EXO 77 — Vampire Hounds
pub(in crate::card::sets) static VAMPIRE_HOUNDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("746bc301-9f08-4d9b-819e-690f6fce6bc8"),
    "Vampire Hounds",
    crate::card::CardArt::new("746bc301-9f08-4d9b-819e-690f6fce6bc8", "Kev Walker"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Vampire", "Dog"], 2, 2).with_ability(
        AbilityDef::activated(
            "Discard a creature card: This creature gets +2/+2 until end of turn.",
            &[AbilityCostDef::DiscardCardMatching(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// EXO 78 — Volrath's Dungeon
pub(in crate::card::sets) static VOLRATH_S_DUNGEON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a4ab28e1-74e1-4c4e-920f-a658c6a44d75"),
    "Volrath's Dungeon",
    crate::card::CardArt::new("a4ab28e1-74e1-4c4e-920f-a658c6a44d75", "Stephen Daniele"),
    crate::card::CardSet::Exodus,
    CardRules::new_enchantment(mana_cost!("{2}{B}{B}")).with_abilities(&[
        AbilityDef::activated(
            "Pay 5 life: Destroy this enchantment. Any player may activate this ability but only during their turn.",
            &[AbilityCostDef::PayLife(5)],
            EffectDef::Destroy {
                object: EffectRecipientDef::Source,
                can_regenerate: true,
                then: None,
            },
        )
        .open_to_any_player()
        .with_activation_timing(ActivationTimingDef::YourTurn),
        AbilityDef::activated_with_targets(
            "Discard a card: Target player puts a card from their hand on top of their library. Activate only as a sorcery.",
            &[AbilityCostDef::DiscardCardMatching(ObjectPredicateDef::Any)],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Object(ParentBinding),
                unchosen: None,
                chooser: PlayerRefDef::Target(TargetIndex::PRIMARY),
                candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Hand],
                    PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                )),
                exclude: None,
                minimum: 1,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Private,
                then: &EffectDef::MoveToZone {
                    object: EffectRecipientDef::object(ObjectRefDef::Binding(ParentBinding)),
                    zone: ZoneKind::Library,
                    placement: ZonePlacement::Top,
                },
            }),
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// EXO 79 — Anarchist
pub(in crate::card::sets) static ANARCHIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a298df66-2075-40a7-bced-457656b6b788"),
    "Anarchist",
    crate::card::CardArt::new("a298df66-2075-40a7-bced-457656b6b788", "Brom"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Human", "Wizard"], 2, 2).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, you may return target sorcery card from your graveyard to your hand.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Sorcery),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
                1,
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
    ),
);

// EXO 80 — Cinder Crawler
pub(in crate::card::sets) static CINDER_CRAWLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a9406050-d76b-4569-a463-e21acaf84166"),
    "Cinder Crawler",
    crate::card::CardArt::new("a9406050-d76b-4569-a463-e21acaf84166", "Jim Nelson"),
    crate::card::CardSet::Exodus,
    // Audit: unsupported — Needs an activated-ability condition for the source
    // being blocked; existing combat predicates do not express that state.
    CardRules::unsupported(),
);

// EXO 81 — Dizzying Gaze
pub(in crate::card::sets) static DIZZYING_GAZE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("71a482cf-a1cd-47b5-a76a-08e03965c679"),
    "Dizzying Gaze",
    crate::card::CardArt::new("71a482cf-a1cd-47b5-a76a-08e03965c679", "Thomas M. Baxa"),
    crate::card::CardSet::Exodus,
    CardRules::new_enchantment(mana_cost!("{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature_you_control(),
            AbilityDef::activated_with_targets(
                "{R}: Enchanted creature deals 1 damage to target creature with flying.",
                &[AbilityCostDef::Mana(mana_cost!("{R}"))],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                    ]),
                )],
                EffectDef::DealDamageFrom {
                    source: ObjectRefDef::AttachedToSource,
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// EXO 82 — Fighting Chance
pub(in crate::card::sets) static FIGHTING_CHANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ca75f6e9-5eee-4904-88c0-71ec730a0f23"),
    "Fighting Chance",
    crate::card::CardArt::new("ca75f6e9-5eee-4904-88c0-71ec730a0f23", "Mike Raabe"),
    crate::card::CardSet::Exodus,
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::spell(
        "For each blocking creature, flip a coin. If you win the flip, prevent all combat damage that would be dealt by that creature this turn.",
        abilities::bind_objects_then(
            crate::card::ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::Query(
                ObjectQueryDef::new(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Blocking,
                    ]),
                    &[ZoneKind::Battlefield],
                ),
            )),
            &EffectDef::ForEachInBinding {
                objects: ParentBinding,
                binding: ParentBinding,
                effect: &EffectDef::Randomized {
                    likelihood: LikelihoodDef::new(0.5),
                    on_success: &EffectDef::PreventDamage {
                        prevention: crate::card::DamagePreventionDef::unlimited(
                            crate::card::DamageEventMatcherDef {
                                kind: crate::card::DamageKindDef::Combat,
                                ..crate::card::DamageEventMatcherDef::from(
                                    ObjectRefDef::Binding(ParentBinding),
                                )
                            },
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                    on_failure: &EffectDef::None,
                },
            },
        ),
    )),
);

// EXO 83 — Flowstone Flood
// Audit: unsupported — Buyback can compose a life payment, but spell additional
// costs cannot currently discard a card at random.
pub(in crate::card::sets) static FLOWSTONE_FLOOD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8bcda003-6fac-4879-87e6-ec0c115630ba"),
    "Flowstone Flood",
    crate::card::CardArt::new("8bcda003-6fac-4879-87e6-ec0c115630ba", "Paolo Parente"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 84 — Furnace Brood
pub(in crate::card::sets) static FURNACE_BROOD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0a79c6d9-96f1-434a-89b8-d773aa77ac5e"),
    "Furnace Brood",
    crate::card::CardArt::new("0a79c6d9-96f1-434a-89b8-d773aa77ac5e", "Jeff Miracola"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Elemental"], 3, 3).with_ability(
        AbilityDef::activated_with_targets(
            "{R}: Target creature can't be regenerated this turn.",
            &[AbilityCostDef::Mana(mana_cost!("{R}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotRegenerate),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// EXO 85 — Keeper of the Flame
// Audit: unsupported — Needs a target-player predicate comparing live life totals.
pub(in crate::card::sets) static KEEPER_OF_THE_FLAME: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9bf246ca-9dfc-400f-8883-acc80ac016e1"),
    "Keeper of the Flame",
    crate::card::CardArt::new("9bf246ca-9dfc-400f-8883-acc80ac016e1", "Terese Nielsen"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 86 — Mage il-Vec
pub(in crate::card::sets) static MAGE_IL_VEC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("04e3e38b-2191-4b92-ae5d-bb9397d24a27"),
    "Mage il-Vec",
    crate::card::CardArt::new("04e3e38b-2191-4b92-ae5d-bb9397d24a27", "John Matson"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Wizard"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{T}, Discard a card at random: This creature deals 1 damage to any target.",
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::DiscardCardsAtRandom(1),
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// EXO 87 — Maniacal Rage
pub(in crate::card::sets) static MANIACAL_RAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f3aa840f-6a70-4674-acb7-ded0ea4397d8"),
    "Maniacal Rage",
    crate::card::CardArt::new("f3aa840f-6a70-4674-acb7-ded0ea4397d8", "Pete Venters"),
    crate::card::CardSet::Exodus,
    CardRules::new_enchantment(mana_cost!("{1}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+2 and can't block.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(
                            BlockRestrictionDef::CANNOT_BLOCK,
                        )),
                    ]),
                },
            ),
        ]),
);

// EXO 88 — Mogg Assassin
pub(in crate::card::sets) static MOGG_ASSASSIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1124725d-e643-43a1-873e-255636c7f334"),
    "Mogg Assassin",
    crate::card::CardArt::new("1124725d-e643-43a1-873e-255636c7f334", "Dermot Power"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Assassin"], 2, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: You choose target creature an opponent controls, and that opponent chooses target creature. Flip a coin. If you win the flip, destroy the creature you chose. If you lose the flip, destroy the creature your opponent chose.",
            &[AbilityCostDef::TapSource],
            &[
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                }),
                AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Creature,
                ))
                .chosen_by_opponent(),
            ],
            EffectDef::Randomized {
                likelihood: LikelihoodDef::new(0.5),
                on_success: &EffectDef::destroy_target(TargetIndex::PRIMARY, true),
                on_failure: &EffectDef::destroy_target(TargetIndex(1), true),
            },
        ),
    ),
);

// EXO 89 — Monstrous Hound
// Audit: unsupported — Needs attack and block restrictions that compare the source
// controller's land count with the current defending or attacking player.
pub(in crate::card::sets) static MONSTROUS_HOUND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7ae4162d-e080-4db1-912f-d53674c76170"),
    "Monstrous Hound",
    crate::card::CardArt::new("d5066b1b-3910-4434-83d6-030851f20bcf", "Dermot Power"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 90 — Oath of Mages
// Audit: unsupported — Needs an event-player target predicate comparing live life
// totals before offering the optional damage.
pub(in crate::card::sets) static OATH_OF_MAGES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ed8708d2-2c73-4da5-b6ff-41c083b59caa"),
    "Oath of Mages",
    crate::card::CardArt::new("ed8708d2-2c73-4da5-b6ff-41c083b59caa", "Keith Parkinson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 91 — Ogre Shaman
pub(in crate::card::sets) static OGRE_SHAMAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cb3224ac-9b60-48cf-9734-86768fd370ac"),
    "Ogre Shaman",
    crate::card::CardArt::new("cb3224ac-9b60-48cf-9734-86768fd370ac", "Paolo Parente"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Ogre", "Shaman"], 3, 3).with_ability(
        AbilityDef::activated_with_targets(
            "{2}, Discard a card at random: This creature deals 2 damage to any target.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}")),
                AbilityCostDef::DiscardCardsAtRandom(1),
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

// EXO 92 — Onslaught
pub(in crate::card::sets) static ONSLAUGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0afaf142-dbca-45bf-aea2-01c53bda635a"),
    "Onslaught",
    crate::card::CardArt::new("0afaf142-dbca-45bf-aea2-01c53bda635a", "Paolo Parente"),
    crate::card::CardSet::Exodus,
    CardRules::new_enchantment(mana_cost!("{R}")).with_ability(AbilityDef::triggered_with_targets(
        "Whenever you cast a creature spell, tap target creature.",
        TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
        ])),
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Tap {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    )),
);

// EXO 93 — Pandemonium
pub(in crate::card::sets) static PANDEMONIUM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5f618231-28bb-4cdd-b887-a8aa186814d5"),
    "Pandemonium",
    crate::card::CardArt::new("5f618231-28bb-4cdd-b887-a8aa186814d5", "Pete Venters"),
    crate::card::CardSet::Exodus,
    CardRules::new_enchantment(mana_cost!("{3}{R}")).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever a creature enters, that creature's controller may have it deal damage equal to its power to any target of their choice.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::HasType(CardType::Creature),
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::AnyTarget,
                1,
            )
            .chosen_by_event_player()],
            EffectDef::DealDamageFrom {
                source: ObjectRefDef::TriggeringObject,
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::TriggeringObjectPower,
            },
        ),
    ),
);

// EXO 94 — Paroxysm
pub(in crate::card::sets) static PAROXYSM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("53320321-4f02-40ee-8171-2375b1d4ed66"),
    "Paroxysm",
    crate::card::CardArt::new("53320321-4f02-40ee-8171-2375b1d4ed66", "Scott Kirschner"),
    crate::card::CardSet::Exodus,
    CardRules::new_enchantment(mana_cost!("{1}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            abilities::enchanted_controller_upkeep(
                "At the beginning of the upkeep of enchanted creature's controller, that player reveals the top card of their library. If that card is a land card, destroy that creature. Otherwise, it gets +3/+3 until end of turn.",
                abilities::bind_top_cards_then(
                    PlayerRefDef::ControllerOf(ObjectRefDef::AttachedToSource),
                    ValueDef::Constant(1),
                    &EffectDef::ClassifyObjects(ClassifyObjectsDef {
                        input: ObjectSetDef::Binding(ParentBinding),
                        object: ObjectPredicateDef::HasType(CardType::Land),
                        matching: Binding!("paroxysm_land"),
                        remainder: Binding!("paroxysm_nonland"),
                        then: &EffectDef::IfNoObjects(IfNoObjectsDef {
                            input: ObjectSetDef::Binding(Binding!("paroxysm_land")),
                            if_empty: &EffectDef::Sequence(&[
                                EffectDef::RevealObjects(RevealObjectsDef {
                                    input: ObjectSetDef::Binding(ParentBinding),
                                    then: &EffectDef::None,
                                }),
                                EffectDef::Apply {
                                    recipient: EffectRecipientDef::AttachedPermanent,
                                    effect: AppliedEffectDef::modify_power_toughness(
                                        ValueDef::Constant(3),
                                        ValueDef::Constant(3),
                                    ),
                                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                                },
                            ]),
                            otherwise: &EffectDef::Sequence(&[
                                EffectDef::RevealObjects(RevealObjectsDef {
                                    input: ObjectSetDef::Binding(ParentBinding),
                                    then: &EffectDef::None,
                                }),
                                EffectDef::Destroy {
                                    object: EffectRecipientDef::AttachedPermanent,
                                    can_regenerate: true,
                                    then: None,
                                },
                            ]),
                        }),
                    }),
                ),
            ),
        ]),
);

// EXO 95 — Price of Progress
// Audit: unsupported — Damage amounts cannot yet be evaluated separately for each
// recipient from that player's own nonbasic-land count.
pub(in crate::card::sets) static PRICE_OF_PROGRESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8e5283db-3e22-4862-9d95-56d03d09c2ae"),
    "Price of Progress",
    crate::card::CardArt::new(
        "8e5283db-3e22-4862-9d95-56d03d09c2ae",
        "Richard Kane Ferguson",
    ),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 96 — Raging Goblin
pub(in crate::card::sets) static RAGING_GOBLIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6c0fa444-5534-4476-8bfa-78b2364f2dd3"),
    "Raging Goblin",
    crate::card::CardArt::new("1f0a166c-f7c0-45b4-aa90-053ce545cfb2", "Brian Snõddy"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin", "Berserker"], 1, 1)
        .with_ability(abilities::haste()),
);

// EXO 97 — Ravenous Baboons
pub(in crate::card::sets) static RAVENOUS_BABOONS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6d00b68b-8b6a-48c9-8911-2a3270897091"),
    "Ravenous Baboons",
    crate::card::CardArt::new("6d00b68b-8b6a-48c9-8911-2a3270897091", "Daren Bader"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Monkey"], 2, 2).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, destroy target nonbasic land.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(CardSupertype::Basic)),
                ]),
            )],
            EffectDef::destroy_target(TargetIndex::PRIMARY, true),
        ),
    ),
);

// EXO 98 — Reckless Ogre
pub(in crate::card::sets) static RECKLESS_OGRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("90d27b79-a22d-48d9-86b2-7ad02cab8697"),
    "Reckless Ogre",
    crate::card::CardArt::new("90d27b79-a22d-48d9-86b2-7ad02cab8697", "Paolo Parente"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Ogre"], 3, 2).with_ability(
        AbilityDef::triggered(
            "Whenever this creature attacks alone, it gets +3/+0 until end of turn.",
            TriggerEventDef::attacks_in_declaration(ObjectPredicateDef::Source, 1, Some(1)),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// EXO 99 — Sabertooth Wyvern
pub(in crate::card::sets) static SABERTOOTH_WYVERN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("84c1d384-d341-4bab-bf71-5dbcf76d51e8"),
    "Sabertooth Wyvern",
    crate::card::CardArt::new("84c1d384-d341-4bab-bf71-5dbcf76d51e8", "Keith Parkinson"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Drake"], 3, 2)
        .with_abilities(&[abilities::flying(), abilities::first_strike()]),
);

// EXO 100 — Scalding Salamander
pub(in crate::card::sets) static SCALDING_SALAMANDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5a0e9433-88d7-4bfc-99a0-ff47807fd594"),
    "Scalding Salamander",
    crate::card::CardArt::new("5a0e9433-88d7-4bfc-99a0-ff47807fd594", "Terese Nielsen"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Salamander"], 2, 1).with_ability(
        AbilityDef::triggered(
            "Whenever this creature attacks, you may have it deal 1 damage to each creature without flying defending player controls.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::DealDamage {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                                KeywordAbility::Flying,
                            )),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::DefendingPlayer,
                    ),
                    amount: ValueDef::Constant(1),
                },
            },
        ),
    ),
);

// EXO 101 — Seismic Assault
pub(in crate::card::sets) static SEISMIC_ASSAULT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cc494af5-4da4-43f5-a193-426ef84d80a7"),
    "Seismic Assault",
    crate::card::CardArt::new("cc494af5-4da4-43f5-a193-426ef84d80a7", "Dermot Power"),
    crate::card::CardSet::Exodus,
    CardRules::new_enchantment(mana_cost!("{R}{R}{R}")).with_ability(
        AbilityDef::activated_with_targets(
            "Discard a land card: This enchantment deals 2 damage to any target.",
            &[AbilityCostDef::DiscardCardMatching(
                ObjectPredicateDef::HasType(CardType::Land),
            )],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

// EXO 102 — Shattering Pulse
pub(in crate::card::sets) static SHATTERING_PULSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("89d3b846-6071-4d65-86ba-da08c4bd0aa1"),
    "Shattering Pulse",
    crate::card::CardArt::new("89d3b846-6071-4d65-86ba-da08c4bd0aa1", "Donato Giancola"),
    crate::card::CardSet::Exodus,
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[
        abilities::buyback(mana_cost!("{3}")),
        AbilityDef::destroy_target(
            "Destroy target artifact.",
            &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                CardType::Artifact,
            )),
            true,
        ),
    ]),
);

// EXO 103 — Sonic Burst
// Audit: unsupported — Spell additional costs cannot currently discard a card at random.
pub(in crate::card::sets) static SONIC_BURST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("05530d5a-dcb6-403e-9e35-224c7b5cf615"),
    "Sonic Burst",
    crate::card::CardArt::new("05530d5a-dcb6-403e-9e35-224c7b5cf615", "Brian Snõddy"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 104 — Spellshock
pub(in crate::card::sets) static SPELLSHOCK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("52db2a78-e1c5-4732-a4ee-04b4c540edbe"),
    "Spellshock",
    crate::card::CardArt::new("52db2a78-e1c5-4732-a4ee-04b4c540edbe", "Thomas M. Baxa"),
    crate::card::CardSet::Exodus,
    CardRules::new_enchantment(mana_cost!("{2}{R}")).with_ability(AbilityDef::triggered(
        "Whenever a player casts a spell, this enchantment deals 2 damage to that player.",
        TriggerEventDef::spell_cast(ObjectPredicateDef::Spell),
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::EventPlayer,
            amount: ValueDef::Constant(2),
        },
    )),
);

// EXO 105 — Avenging Druid
pub(in crate::card::sets) static AVENGING_DRUID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fca9fd31-639a-4fbc-84bd-c3078df29c0a"),
    "Avenging Druid",
    crate::card::CardArt::new("fca9fd31-639a-4fbc-84bd-c3078df29c0a", "Daren Bader"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Druid"], 1, 3).with_ability(
        AbilityDef::triggered(
            "Whenever this creature deals damage to an opponent, you may reveal cards from the top of your library until you reveal a land card. If you do, put that card onto the battlefield and put all other cards revealed this way into your graveyard.",
            TriggerEventDef::damage_to_player(ObjectPredicateDef::Source, PlayerRelation::Opponent),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::MillUntil(&MillUntilDef {
                    player: EffectRecipientDef::Controller,
                    until: ObjectSetPredicateDef::contains(&ObjectPredicateDef::HasType(
                        CardType::Land,
                    )),
                    matched_zone: ZoneKind::Battlefield,
                }),
            },
        ),
    ),
);

// EXO 106 — Bequeathal
pub(in crate::card::sets) static BEQUEATHAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("20aae577-9683-4d9b-bfd5-52702b38d3a7"),
    "Bequeathal",
    crate::card::CardArt::new(
        "20aae577-9683-4d9b-bfd5-52702b38d3a7",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Exodus,
    CardRules::new_enchantment(mana_cost!("{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::triggered(
                "When enchanted creature dies, you draw two cards.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::AttachedToSource,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ),
        ]),
);

// EXO 107 — Cartographer
pub(in crate::card::sets) static CARTOGRAPHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f2c2cc9-37ce-435e-9df2-083d5e3c8c5c"),
    "Cartographer",
    crate::card::CardArt::new("7f2c2cc9-37ce-435e-9df2-083d5e3c8c5c", "Jeff Laubenstein"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human"], 2, 2).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, you may return target land card from your graveyard to your hand.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
                1,
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
    ),
);

// EXO 108 — Crashing Boars
// Audit: unsupported — Needs the defending player to choose a creature and impose a
// source-specific must-block requirement for the rest of the turn.
pub(in crate::card::sets) static CRASHING_BOARS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a2965bd5-4f16-443a-9133-adb92cf0e12b"),
    "Crashing Boars",
    crate::card::CardArt::new("a2965bd5-4f16-443a-9133-adb92cf0e12b", "Ron Spencer"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 109 — Elven Palisade
pub(in crate::card::sets) static ELVEN_PALISADE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b990ffe5-fd2a-4646-bac3-8e52cdc328aa"),
    "Elven Palisade",
    crate::card::CardArt::new("b990ffe5-fd2a-4646-bac3-8e52cdc328aa", "Mark Zug"),
    crate::card::CardSet::Exodus,
    CardRules::new_enchantment(mana_cost!("{G}")).with_ability(AbilityDef::activated_with_targets(
        "Sacrifice a Forest: Target attacking creature gets -3/-0 until end of turn.",
        &[AbilityCostDef::SacrificePermanent {
            object: ObjectPredicateDef::HasAnyBasicLandType(&[crate::card::BasicLandType::Forest]),
            controller: PlayerRelation::You,
        }],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Attacking,
            ]),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-3),
                ValueDef::Constant(0),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// EXO 110 — Elvish Berserker
pub(in crate::card::sets) static ELVISH_BERSERKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dfa69a8e-1b75-4d93-918d-d772cec69e99"),
    "Elvish Berserker",
    crate::card::CardArt::new("dfa69a8e-1b75-4d93-918d-d772cec69e99", "Paolo Parente"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Berserker"], 1, 1).with_ability(
        AbilityDef::triggered(
            "Whenever this creature becomes blocked, it gets +1/+1 until end of turn for each creature blocking it.",
            TriggerEventDef::BecomesBlocked(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Sum(&crate::card::SumValueDef::new(
                        ValueDef::TriggerEventAmount,
                        ValueDef::Constant(1),
                    )),
                    ValueDef::Sum(&crate::card::SumValueDef::new(
                        ValueDef::TriggerEventAmount,
                        ValueDef::Constant(1),
                    )),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// EXO 111 — Jackalope Herd
pub(in crate::card::sets) static JACKALOPE_HERD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cb80105c-d2c0-4f8c-9302-5e6152a60f54"),
    "Jackalope Herd",
    crate::card::CardArt::new("cb80105c-d2c0-4f8c-9302-5e6152a60f54", "Ron Spencer"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Rabbit", "Beast"], 4, 5).with_ability(
        AbilityDef::triggered(
            "When you cast a spell, return this creature to its owner's hand.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(PlayerRelation::You)),
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Source,
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
    ),
);

// EXO 112 — Keeper of the Beasts
pub(in crate::card::sets) static KEEPER_OF_THE_BEASTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cccccf78-8b00-406b-a2b7-0e6ba76703d0"),
    "Keeper of the Beasts",
    crate::card::CardArt::new("cccccf78-8b00-406b-a2b7-0e6ba76703d0", "rk post"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{G}{G}"), &["Human", "Wizard"], 1, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{G}, {T}: Choose target opponent who controls more creatures than you do as you activate this ability. Create a 2/2 green Beast creature token.",
            &[
                AbilityCostDef::Mana(mana_cost!("{G}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerWithMoreObjectsThanChooser {
                    relation: PlayerRelation::Opponent,
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                },
            )],
            EffectDef::CreateToken {
                token: crate::card::TokenCharacteristics::creature(
                    &["Beast"],
                    &[ManaColor::Green],
                    2,
                    2,
                ),
                copy: None,
                controller: None,
                count: ValueDef::Constant(1),
                tapped: false,
                attacking: false,
                counters: None,
                created: None,
            },
        ),
    ),
);

// EXO 113 — Manabond
pub(in crate::card::sets) static MANABOND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("212ca7e7-5ba3-4da7-a2f0-16c721004bac"),
    "Manabond",
    crate::card::CardArt::new("212ca7e7-5ba3-4da7-a2f0-16c721004bac", "Stephen Daniele"),
    crate::card::CardSet::Exodus,
    CardRules::new_enchantment(mana_cost!("{G}")).with_ability(AbilityDef::triggered(
        "At the beginning of your end step, you may reveal your hand and put all land cards from it onto the battlefield. If you do, discard your hand.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::End,
            player: PlayerRelation::You,
        },
        EffectDef::May {
            player: EffectRecipientDef::Controller,
            effect: &EffectDef::Sequence(&[
                EffectDef::RevealHand {
                    player: EffectRecipientDef::Controller,
                },
                EffectDef::MoveObjects(MoveObjectsDef {
                    input: ObjectSetDef::Query(ObjectQueryDef::owned_by(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Hand],
                        PlayerSetDef::One(PlayerRefDef::EffectController),
                    )),
                    from: Some(ZoneKind::Hand),
                    zone: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    moved: None,
                    then: &EffectDef::None,
                }),
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(i32::MAX),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            ]),
        },
    )),
);

// EXO 114 — Mirri, Cat Warrior
pub(in crate::card::sets) static MIRRI_CAT_WARRIOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6d1682dd-5a99-4bee-a2c2-c8735047e1a9"),
    "Mirri, Cat Warrior",
    crate::card::CardArt::new("6d1682dd-5a99-4bee-a2c2-c8735047e1a9", "Daren Bader"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Cat", "Warrior"], 2, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::first_strike(),
            abilities::forestwalk(),
            abilities::vigilance(),
        ]),
);

// EXO 115 — Oath of Druids
pub(in crate::card::sets) static OATH_OF_DRUIDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cf14de50-d123-400c-862e-2c95fd2aa23f"),
    "Oath of Druids",
    CardArt::new("cf14de50-d123-400c-862e-2c95fd2aa23f", "Daren Bader"),
    CardSet::Exodus,
    CardRules::new_enchantment(mana_cost!("{1}{G}")).with_ability(
        AbilityDef::triggered_with_targets(
            "At the beginning of each player's upkeep, that player chooses target player who \
             controls more creatures than they do and is their opponent. The first player may \
             reveal cards from the top of their library until they reveal a creature card. If the \
             first player does, that player puts that card onto the battlefield and all other \
             cards revealed this way into their graveyard.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::Any,
            },
            // "Target player who controls more creatures than they do and is their
            // opponent." The comparison is against whoever is choosing -- the player
            // whose upkeep it is -- rather than against Oath's own controller, which
            // is what makes this a targeting restriction rather than a condition.
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerWithMoreObjectsThanChooser {
                    relation: PlayerRelation::Opponent,
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                },
            )
            .chosen_by_event_player()],
            EffectDef::May {
                player: EffectRecipientDef::EventPlayer,
                effect: &EffectDef::MillUntil(&MillUntilDef {
                    player: EffectRecipientDef::EventPlayer,
                    until: ObjectSetPredicateDef::contains(&ObjectPredicateDef::HasType(
                        CardType::Creature,
                    )),
                    matched_zone: ZoneKind::Battlefield,
                }),
            },
        ),
    ),
);

// EXO 116 — Plated Rootwalla
pub(in crate::card::sets) static PLATED_ROOTWALLA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4bf4da70-c656-4e40-bb0f-68e9dda024c9"),
    "Plated Rootwalla",
    crate::card::CardArt::new("4bf4da70-c656-4e40-bb0f-68e9dda024c9", "Randy Elliott"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Lizard"], 3, 3).with_ability(
        AbilityDef::activated(
            "{2}{G}: This creature gets +3/+3 until end of turn. Activate only once each turn.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(3),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .once_each_turn(),
    ),
);

// EXO 117 — Predatory Hunger
pub(in crate::card::sets) static PREDATORY_HUNGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("db6d9d28-3a05-4dfa-a322-36b4cc2697d4"),
    "Predatory Hunger",
    crate::card::CardArt::new("db6d9d28-3a05-4dfa-a322-36b4cc2697d4", "Brom"),
    crate::card::CardSet::Exodus,
    CardRules::new_enchantment(mana_cost!("{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::triggered(
                "Whenever an opponent casts a creature spell, put a +1/+1 counter on enchanted creature.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                ])),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::AttachedPermanent,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// EXO 118 — Pygmy Troll
pub(in crate::card::sets) static PYGMY_TROLL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7be9714d-125f-4700-879d-b920fe9f1b68"),
    "Pygmy Troll",
    crate::card::CardArt::new("7be9714d-125f-4700-879d-b920fe9f1b68", "Daniel Gelon"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Troll"], 1, 1).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature becomes blocked by a creature, this creature gets +1/+1 until end of turn.",
            TriggerEventDef::BecomesBlockedBy {
                blocker: ObjectPredicateDef::HasType(CardType::Creature),
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::regenerate_self(
            "{G}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{G}"))],
        ),
    ]),
);

// EXO 119 — Rabid Wolverines
pub(in crate::card::sets) static RABID_WOLVERINES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("99121a2b-c735-47be-b01e-cdf59809e7f3"),
    "Rabid Wolverines",
    crate::card::CardArt::new("99121a2b-c735-47be-b01e-cdf59809e7f3", "Daren Bader"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Wolverine"], 4, 4).with_ability(
        AbilityDef::triggered(
            "Whenever this creature becomes blocked by a creature, this creature gets +1/+1 until end of turn.",
            TriggerEventDef::BecomesBlockedBy {
                blocker: ObjectPredicateDef::HasType(CardType::Creature),
            },
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

// EXO 120 — Reclaim (reprint)

// EXO 121 — Resuscitate
pub(in crate::card::sets) static RESUSCITATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f5707560-fcc6-4aca-adce-d41de45f37e8"),
    "Resuscitate",
    crate::card::CardArt::new("f5707560-fcc6-4aca-adce-d41de45f37e8", "Rebecca Guay"),
    crate::card::CardSet::Exodus,
    CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell(
        "Until end of turn, creatures you control gain \"{1}: Regenerate this creature.\"",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::add_ability(&AbilityDef::activated(
                "{1}: Regenerate this creature.",
                &[AbilityCostDef::Mana(mana_cost!("{1}"))],
                EffectDef::Regenerate {
                    object: EffectRecipientDef::Source,
                },
            )),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// EXO 122 — Rootwater Alligator
pub(in crate::card::sets) static ROOTWATER_ALLIGATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3a840bba-4725-45fd-885f-1b3d615dfa97"),
    "Rootwater Alligator",
    crate::card::CardArt::new("3a840bba-4725-45fd-885f-1b3d615dfa97", "Stephen Daniele"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Crocodile"], 3, 2).with_ability(
        abilities::regenerate_self(
            "Sacrifice a Forest: Regenerate this creature.",
            &[AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::HasAnyBasicLandType(&[
                    crate::card::BasicLandType::Forest,
                ]),
                controller: PlayerRelation::You,
            }],
        ),
    ),
);

// EXO 123 — Skyshroud Elite
pub(in crate::card::sets) static SKYSHROUD_ELITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f6a496a4-1b4c-4c5d-99e5-ec40601c759d"),
    "Skyshroud Elite",
    crate::card::CardArt::new("f6a496a4-1b4c-4c5d-99e5-ec40601c759d", "Paolo Parente"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{G}"), &["Elf"], 1, 1).with_ability(
        AbilityDef::static_ability(
            "This creature gets +1/+2 as long as an opponent controls a nonbasic land.",
            EffectDef::ConditionalStatic(ConditionalStaticEffectDef {
                condition: ObjectSetCountConditionDef {
                    objects: &ObjectSetDef::Query(ObjectQueryDef::controlled_by(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(
                                CardSupertype::Basic,
                            )),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerSetDef::Related(PlayerRelation::Opponent),
                    )),
                    predicate: ObjectSetPredicateDef {
                        filter: None,
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 1,
                    },
                },
                then: StaticApplyDef {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(2),
                    ),
                },
            }),
        ),
    ),
);

// EXO 124 — Skyshroud War Beast
pub(in crate::card::sets) static SKYSHROUD_WAR_BEAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("19d809c1-e674-40b8-816d-c45d77c66722"),
    "Skyshroud War Beast",
    crate::card::CardArt::new("19d809c1-e674-40b8-816d-c45d77c66722", "Jim Nelson"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Beast"], 0, 0).with_abilities(&[
        abilities::trample(),
        AbilityDef::replacement(
            "As this creature enters, choose an opponent.",
            ReplacementEffectDef::Choose(ReplacementChoiceDef::Player(
                PlayerRelation::Opponent,
            )),
        ),
        AbilityDef::static_ability(
            "Skyshroud War Beast's power and toughness are each equal to the number of nonbasic lands the chosen player controls.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::define_power_toughness(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::controlled_by(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(
                                CardSupertype::Basic,
                            )),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerSetDef::Related(PlayerRelation::ChosenPlayer),
                    )),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::controlled_by(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(
                                CardSupertype::Basic,
                            )),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerSetDef::Related(PlayerRelation::ChosenPlayer),
                    )),
                ),
            },
        ),
    ]),
);

// EXO 125 — Song of Serenity
pub(in crate::card::sets) static SONG_OF_SERENITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2ba85b2f-37da-4595-9880-8e9f1ddbac09"),
    "Song of Serenity",
    crate::card::CardArt::new("2ba85b2f-37da-4595-9880-8e9f1ddbac09", "DiTerlizzi"),
    crate::card::CardSet::Exodus,
    CardRules::new_enchantment(mana_cost!("{1}{G}")).with_ability(AbilityDef::static_ability(
        "Creatures that are enchanted can't attack or block.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Enchanted,
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::Rule(AppliedRuleDef::AttackRestriction(
                    crate::card::AttackRestrictionDef::CANNOT_ATTACK,
                )),
                AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(
                    BlockRestrictionDef::CANNOT_BLOCK,
                )),
            ]),
        },
    )),
);

// EXO 126 — Spike Hatcher
pub(in crate::card::sets) static SPIKE_HATCHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1f79fb79-37a0-483f-ba19-853cbfffc73d"),
    "Spike Hatcher",
    crate::card::CardArt::new("1f79fb79-37a0-483f-ba19-853cbfffc73d", "Stephen Daniele"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{6}{G}"), &["Spike"], 0, 0).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with six +1/+1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 6,
                },
            ),
        ),
        AbilityDef::activated_with_targets(
            "{2}, Remove a +1/+1 counter from this creature: Put a +1/+1 counter on target creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}")),
                AbilityCostDef::RemoveCountersFromSource {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 1,
                },
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::regenerate_self(
            "{1}, Remove a +1/+1 counter from this creature: Regenerate this creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::RemoveCountersFromSource {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 1,
                },
            ],
        ),
    ]),
);

// EXO 127 — Spike Rogue
// Audit: unsupported — The second activation needs to select another creature and
// remove one of its counters atomically as part of the activation cost.
pub(in crate::card::sets) static SPIKE_ROGUE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f0d9b671-344b-460d-8f65-d65129db91c3"),
    "Spike Rogue",
    crate::card::CardArt::new("f0d9b671-344b-460d-8f65-d65129db91c3", "Heather Hudson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 128 — Spike Weaver
pub(in crate::card::sets) static SPIKE_WEAVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9c561a2a-91c6-4d4b-9f96-bffd43a00478"),
    "Spike Weaver",
    crate::card::CardArt::new("9c561a2a-91c6-4d4b-9f96-bffd43a00478", "Mike Raabe"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Spike"], 0, 0).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with three +1/+1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 3,
                },
            ),
        ),
        AbilityDef::activated_with_targets(
            "{2}, Remove a +1/+1 counter from this creature: Put a +1/+1 counter on target creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}")),
                AbilityCostDef::RemoveCountersFromSource {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 1,
                },
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "{1}, Remove a +1/+1 counter from this creature: Prevent all combat damage that would be dealt this turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::RemoveCountersFromSource {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 1,
                },
            ],
            EffectDef::PreventDamage {
                prevention: crate::card::DamagePreventionDef::unlimited(
                    crate::card::DamageEventMatcherDef::COMBAT,
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// EXO 129 — Survival of the Fittest
pub(in crate::card::sets) static SURVIVAL_OF_THE_FITTEST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c060c178-3c0e-493f-b6f0-ead5b1d6f191"),
    "Survival of the Fittest",
    crate::card::CardArt::new("c060c178-3c0e-493f-b6f0-ead5b1d6f191", "Pete Venters"),
    crate::card::CardSet::Exodus,
    CardRules::new_enchantment(mana_cost!("{1}{G}")).with_ability(AbilityDef::activated(
        "{G}, Discard a creature card: Search your library for a creature card, reveal that card, put it into your hand, then shuffle.",
        &[
            AbilityCostDef::Mana(mana_cost!("{G}")),
            AbilityCostDef::DiscardCardMatching(ObjectPredicateDef::HasType(CardType::Creature)),
        ],
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::HasType(CardType::Creature),
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
    )),
);

// EXO 130 — Wood Elves
pub(in crate::card::sets) static WOOD_ELVES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b7f1fb90-5c85-46a5-802d-248cc0250921"),
    "Wood Elves",
    crate::card::CardArt::new("4716bb55-0821-4809-9bc0-04e299b09549", "Rebecca Guay"),
    crate::card::CardSet::Exodus,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf", "Scout"], 1, 1).with_ability(
        abilities::enters_trigger(
            "When this creature enters, search your library for a Forest card, put that card onto the battlefield, then shuffle.",
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::HasAnyBasicLandType(&[
                    crate::card::BasicLandType::Forest,
                ]),
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
        ),
    ),
);

// EXO 131 — Coat of Arms
// Audit: unsupported — Needs a dynamic per-creature count of other creatures sharing
// at least one subtype with that affected creature.
pub(in crate::card::sets) static COAT_OF_ARMS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9e32c939-1d64-4082-bafe-59dfa9c054f6"),
    "Coat of Arms",
    crate::card::CardArt::new("9e32c939-1d64-4082-bafe-59dfa9c054f6", "Scott M. Fischer"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 132 — Erratic Portal
pub(in crate::card::sets) static ERRATIC_PORTAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2e652007-02f0-424f-b52c-c1540d1939bd"),
    "Erratic Portal",
    crate::card::CardArt::new("2e652007-02f0-424f-b52c-c1540d1939bd", "John Matson"),
    crate::card::CardSet::Exodus,
    CardRules::new_artifact(mana_cost!("{4}")).with_ability(AbilityDef::activated_with_targets(
        "{1}, {T}: Return target creature to its owner's hand unless its controller pays {1}.",
        &[
            AbilityCostDef::Mana(mana_cost!("{1}")),
            AbilityCostDef::TapSource,
        ],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::PayOr(crate::card::PayOrDef::unless(
            crate::card::EffectPaymentDef::mana(
                PlayerSetDef::One(PlayerRefDef::ControllerOf(ObjectRefDef::Target(
                    TargetIndex::PRIMARY,
                ))),
                mana_cost!("{1}"),
            ),
            &EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        )),
    )),
);

// EXO 133 — Medicine Bag
pub(in crate::card::sets) static MEDICINE_BAG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("399c06d5-af2a-47a1-9239-ff14224a026b"),
    "Medicine Bag",
    crate::card::CardArt::new("399c06d5-af2a-47a1-9239-ff14224a026b", "DiTerlizzi"),
    crate::card::CardSet::Exodus,
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::activated_with_targets(
        "{1}, {T}, Discard a card: Regenerate target creature.",
        &[
            AbilityCostDef::Mana(mana_cost!("{1}")),
            AbilityCostDef::TapSource,
            AbilityCostDef::DiscardCardMatching(ObjectPredicateDef::Any),
        ],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Regenerate {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    )),
);

// EXO 134 — Memory Crystal
// Audit: unsupported — Cost modifiers cannot currently select and reduce only the
// optional buyback component of a spell's total cost.
pub(in crate::card::sets) static MEMORY_CRYSTAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c786ea5b-52ad-4d1b-855e-ce6d0b9af67e"),
    "Memory Crystal",
    crate::card::CardArt::new("c786ea5b-52ad-4d1b-855e-ce6d0b9af67e", "Michael Sutfin"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 135 — Mindless Automaton
pub(in crate::card::sets) static MINDLESS_AUTOMATON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6ddfc5ab-b11b-4ad7-ab46-8ee60d938a5b"),
    "Mindless Automaton",
    crate::card::CardArt::new("6ddfc5ab-b11b-4ad7-ab46-8ee60d938a5b", "Brian Snõddy"),
    crate::card::CardSet::Exodus,
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Construct"], 0, 0).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with two +1/+1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 2,
                },
            ),
        ),
        AbilityDef::activated(
            "{1}, Discard a card: Put a +1/+1 counter on this creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::DiscardCardMatching(ObjectPredicateDef::Any),
            ],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "Remove two +1/+1 counters from this creature: Draw a card.",
            &[AbilityCostDef::RemoveCountersFromSource {
                kind: CounterKind::PlusOnePlusOne,
                amount: 2,
            }],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// EXO 136 — Null Brooch
pub(in crate::card::sets) static NULL_BROOCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d5d5a0c6-916c-428a-ae66-8adc8844e56e"),
    "Null Brooch",
    crate::card::CardArt::new("d5d5a0c6-916c-428a-ae66-8adc8844e56e", "DiTerlizzi"),
    crate::card::CardSet::Exodus,
    CardRules::new_artifact(mana_cost!("{4}")).with_ability(AbilityDef::activated_with_targets(
        "{2}, {T}, Discard your hand: Counter target noncreature spell.",
        &[
            AbilityCostDef::Mana(mana_cost!("{2}")),
            AbilityCostDef::TapSource,
            AbilityCostDef::DiscardHand,
        ],
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::NoncreatureSpell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::counter_target(TargetIndex::PRIMARY),
    )),
);

// EXO 137 — Skyshaper
pub(in crate::card::sets) static SKYSHAPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("234ed934-6ea7-41f6-bd13-3df8662a3a1d"),
    "Skyshaper",
    crate::card::CardArt::new("234ed934-6ea7-41f6-bd13-3df8662a3a1d", "Donato Giancola"),
    crate::card::CardSet::Exodus,
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::activated(
        "Sacrifice this artifact: Creatures you control gain flying until end of turn.",
        &[AbilityCostDef::SacrificeSource],
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::add_ability(&abilities::flying()),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// EXO 138 — Spellbook
pub(in crate::card::sets) static SPELLBOOK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("33fb104c-f8ca-4da2-8f1f-8fe6f291407e"),
    "Spellbook",
    crate::card::CardArt::new("33fb104c-f8ca-4da2-8f1f-8fe6f291407e", "Ciruelo"),
    crate::card::CardSet::Exodus,
    CardRules::new_artifact(mana_cost!("{0}")).with_ability(AbilityDef::static_ability(
        "You have no maximum hand size.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::You)),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::PlayerRule(
                crate::card::PlayerRuleDef::NoMaximumHandSize,
            )),
        },
    )),
);

// EXO 139 — Sphere of Resistance
pub(in crate::card::sets) static SPHERE_OF_RESISTANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("17f4d2a5-bb85-4662-b2dd-a363ec7eab9b"),
    "Sphere of Resistance",
    crate::card::CardArt::new("17f4d2a5-bb85-4662-b2dd-a363ec7eab9b", "Doug Chaffee"),
    crate::card::CardSet::Exodus,
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(abilities::spell_cost_increase(
        "Spells cost {1} more to cast.",
        ObjectPredicateDef::Any,
        PlayerRelation::Any,
        mana_cost!("{1}"),
    )),
);

// EXO 140 — Thopter Squadron
pub(in crate::card::sets) static THOPTER_SQUADRON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d3ac2d30-7c9a-40b3-812e-e77e49229f48"),
    "Thopter Squadron",
    crate::card::CardArt::new("d3ac2d30-7c9a-40b3-812e-e77e49229f48", "Doug Chaffee"),
    crate::card::CardSet::Exodus,
    CardRules::new_artifact_creature(mana_cost!("{5}"), &["Thopter"], 0, 0).with_abilities(&[
        abilities::flying(),
        AbilityDef::as_enters(
            "This creature enters with three +1/+1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 3,
                },
            ),
        ),
        AbilityDef::activated(
            "{1}, Remove a +1/+1 counter from this creature: Create a 1/1 colorless Thopter artifact creature token with flying. Activate only as a sorcery.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::RemoveCountersFromSource {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 1,
                },
            ],
            EffectDef::CreateToken {
                token: crate::card::TokenCharacteristics::artifact_creature(
                    &["Thopter"],
                    &[],
                    1,
                    1,
                )
                .with_abilities(&[abilities::flying()]),
                copy: None,
                controller: None,
                count: ValueDef::Constant(1),
                tapped: false,
                attacking: false,
                counters: None,
                created: None,
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
        AbilityDef::activated(
            "{1}, Sacrifice another Thopter: Put a +1/+1 counter on this creature. Activate only as a sorcery.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Subtype("Thopter"),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// EXO 141 — Transmogrifying Licid
// Audit: unsupported — Licid animation needs a reversible creature-to-Aura type
// change, attachment, ability loss, and special action to end the effect.
pub(in crate::card::sets) static TRANSMOGRIFYING_LICID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1a115563-81da-42f6-95c4-22ae7bb51a0f"),
    "Transmogrifying Licid",
    crate::card::CardArt::new("1a115563-81da-42f6-95c4-22ae7bb51a0f", "Jim Nelson"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

// EXO 142 — Workhorse
pub(in crate::card::sets) static WORKHORSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c2571ff7-0287-4ba2-8365-5ff08de641a2"),
    "Workhorse",
    crate::card::CardArt::new("c2571ff7-0287-4ba2-8365-5ff08de641a2", "DiTerlizzi"),
    crate::card::CardSet::Exodus,
    CardRules::new_artifact_creature(mana_cost!("{6}"), &["Horse"], 0, 0).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with four +1/+1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 4,
                },
            ),
        ),
        AbilityDef::activated_mana(
            "Remove a +1/+1 counter from this creature: Add {C}.",
            &[AbilityCostDef::RemoveCountersFromSource {
                kind: CounterKind::PlusOnePlusOne,
                amount: 1,
            }],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
    ]),
);

// EXO 143 — City of Traitors
pub(in crate::card::sets) static CITY_OF_TRAITORS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a7a8b6b8-b95f-4014-b17a-a6d44d965995"),
    "City of Traitors",
    CardArt::new("a7a8b6b8-b95f-4014-b17a-a6d44d965995", "Kev Walker"),
    CardSet::Exodus,
    // Two mana from one land, for as long as you are willing to stop
    // playing lands. The turn it arrives is free; every turn after is a
    // choice between the mana and the land drop.
    CardRules::new_land(&[]).with_abilities(&[
        // The playing rather than the entering: a land an effect puts onto the
        // battlefield never was played, and the City survives it.
        AbilityDef::triggered(
            "When you play another land, sacrifice this land.",
            TriggerEventDef::LandPlayed {
                land: ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                player: PlayerRelation::You,
            },
            EffectDef::Sacrifice {
                object: EffectRecipientDef::Source,
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {C}{C}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_amount(2)),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ALLAY,
    &ANGELIC_BLESSING,
    &CATACLYSM,
    &CHARGING_PALADIN,
    &CONVALESCENCE,
    &EXALTED_DRAGON,
    &HIGH_GROUND,
    &KEEPER_OF_THE_LIGHT,
    &KOR_CHANT,
    &LIMITED_RESOURCES,
    &OATH_OF_LIEGES,
    &PALADIN_EN_VEC,
    &PEACE_OF_MIND,
    &PEGASUS_STAMPEDE,
    &PENANCE,
    &REAPING_THE_REWARDS,
    &RECONNAISSANCE,
    &SHACKLES,
    &SHIELD_MATE,
    &SOLTARI_VISIONARY,
    &SOUL_WARDEN,
    &STANDING_TROOPS,
    &TREASURE_HUNTER,
    &WALL_OF_NETS,
    &WELKIN_HAWK,
    &ZEALOTS_EN_DAL,
    &AETHER_TIDE,
    &CUNNING,
    &DOMINATING_LICID,
    &EPHEMERON,
    &EQUILIBRIUM,
    &ERTAI_WIZARD_ADEPT,
    &FADE_AWAY,
    &FORBID,
    &KEEPER_OF_THE_MIND,
    &KILLER_WHALE,
    &MANA_BREACH,
    &MIND_OVER_MATTER,
    &MIROZEL,
    &OATH_OF_SCHOLARS,
    &ROBE_OF_MIRRORS,
    &ROOTWATER_MYSTIC,
    &SCHOOL_OF_PIRANHA,
    &SCRIVENER,
    &THALAKOS_DRIFTERS,
    &THALAKOS_SCOUT,
    &THEFT_OF_DREAMS,
    &TREASURE_TROVE,
    &WAYWARD_SOUL,
    &WHIPTONGUE_FROG,
    &CARNOPHAGE,
    &CAT_BURGLAR,
    &CULLING_THE_WEAK,
    &CURSED_FLESH,
    &DAUTHI_CUTTHROAT,
    &DAUTHI_JACKAL,
    &DAUTHI_WARLORD,
    &DEATH_S_DUET,
    &ENTROPIC_SPECTER,
    &FUGUE,
    &GROLLUB,
    &HATRED,
    &KEEPER_OF_THE_DEAD,
    &MIND_MAGGOTS,
    &NAUSEA,
    &NECROLOGIA,
    &OATH_OF_GHOULS,
    &PIT_SPAWN,
    &PLAGUEBEARER,
    &RECURRING_NIGHTMARE,
    &SCARE_TACTICS,
    &SLAUGHTER,
    &SPIKE_CANNIBAL,
    &THRULL_SURGEON,
    &VAMPIRE_HOUNDS,
    &VOLRATH_S_DUNGEON,
    &ANARCHIST,
    &CINDER_CRAWLER,
    &DIZZYING_GAZE,
    &FIGHTING_CHANCE,
    &FLOWSTONE_FLOOD,
    &FURNACE_BROOD,
    &KEEPER_OF_THE_FLAME,
    &MAGE_IL_VEC,
    &MANIACAL_RAGE,
    &MOGG_ASSASSIN,
    &MONSTROUS_HOUND,
    &OATH_OF_MAGES,
    &OGRE_SHAMAN,
    &ONSLAUGHT,
    &PANDEMONIUM,
    &PAROXYSM,
    &PRICE_OF_PROGRESS,
    &RAGING_GOBLIN,
    &RAVENOUS_BABOONS,
    &RECKLESS_OGRE,
    &SABERTOOTH_WYVERN,
    &SCALDING_SALAMANDER,
    &SEISMIC_ASSAULT,
    &SHATTERING_PULSE,
    &SONIC_BURST,
    &SPELLSHOCK,
    &AVENGING_DRUID,
    &BEQUEATHAL,
    &CARTOGRAPHER,
    &CRASHING_BOARS,
    &ELVEN_PALISADE,
    &ELVISH_BERSERKER,
    &JACKALOPE_HERD,
    &KEEPER_OF_THE_BEASTS,
    &MANABOND,
    &MIRRI_CAT_WARRIOR,
    &OATH_OF_DRUIDS,
    &PLATED_ROOTWALLA,
    &PREDATORY_HUNGER,
    &PYGMY_TROLL,
    &RABID_WOLVERINES,
    &RESUSCITATE,
    &ROOTWATER_ALLIGATOR,
    &SKYSHROUD_ELITE,
    &SKYSHROUD_WAR_BEAST,
    &SONG_OF_SERENITY,
    &SPIKE_HATCHER,
    &SPIKE_ROGUE,
    &SPIKE_WEAVER,
    &SURVIVAL_OF_THE_FITTEST,
    &WOOD_ELVES,
    &COAT_OF_ARMS,
    &ERRATIC_PORTAL,
    &MEDICINE_BAG,
    &MEMORY_CRYSTAL,
    &MINDLESS_AUTOMATON,
    &NULL_BROOCH,
    &SKYSHAPER,
    &SPELLBOOK,
    &SPHERE_OF_RESISTANCE,
    &THOPTER_SQUADRON,
    &TRANSMOGRIFYING_LICID,
    &WORKHORSE,
    &CITY_OF_TRAITORS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&catalog_isd::CURIOSITY), // EXO 29
    PrintingRecord::reprint(&catalog_m12::MERFOLK_LOOTER), // EXO 39
    PrintingRecord::reprint(&catalog_m12::RECLAIM),   // EXO 120
];
