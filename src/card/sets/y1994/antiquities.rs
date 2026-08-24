use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    ActivationTimingDef, AddManaEffectDef, AppliedEffectDef, AppliedRuleDef,
    BattlefieldEntryModificationDef, CardArt, CardBehavior, CardRules, CardSet, CardType,
    CardTypeSet, ChoiceVisibilityDef, ConditionDef, CounterKind, CreatureTypeSetDef,
    DamageEventMatcherDef, DamagePreventionDef, DamageSourceGroupDef, DiscardSelectionDef,
    EffectDef, EffectExecutionDef, EffectPaymentDef, EffectRecipientDef, InstalledTriggerDef,
    KeywordAbility, ManaColor, ManaRestrictionDef, ObjectPredicateDef, ObjectQueryDef, PayOrDef,
    PlayerRelation, PlayerSetDef, ReplacementEffectDef, ResolvedEffectDurationDef, ScaledValueDef,
    TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

/// Each land names the other two, and a name predicate compares the name a
/// record carries. That is "Urza's Power Plant" without a hyphen, which is
/// both the printed name and the current Oracle one; the reference text of
/// the other two lands still hyphenates it, and the rules text here follows
/// the record so that what a reader sees and what the engine matches agree.
const fn controls_named(name: &'static str) -> ConditionDef {
    ConditionDef::Exists(ObjectQueryDef::matching(
        ObjectPredicateDef::Named(name),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    ))
}

// ATQ 1 — Argivian Archaeologist
pub(in crate::card::sets) static ARGIVIAN_ARCHAEOLOGIST: CardRecord =
    CardRecord::new_with_legacy_id(
        375,
        "Argivian Archaeologist",
        CardArt::new("ce83a3cb-467d-44f6-a051-4855c8cf52a6", "Amy Weber"),
        CardSet::Antiquities,
        CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Human", "Artificer"], 1, 1)
            .with_abilities(&[AbilityDef::activated_with_targets(
                "{W}{W}, {T}: Return target artifact card from your graveyard to your hand.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{W}{W}")),
                    AbilityCostDef::TapSource,
                ],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Artifact),
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::You),
                    },
                )],
                EffectDef::MoveToZone {
                    counters: None,
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    arrival_effect: None,
                    attachment: None,
                    controller: None,
                },
            )]),
    );

// ATQ 2 — Argivian Blacksmith
pub(in crate::card::sets) static ARGIVIAN_BLACKSMITH: CardRecord = CardRecord::new_with_legacy_id(
    1440,
    "Argivian Blacksmith",
    CardArt::new("5f604338-5ee4-4c47-ad5a-5c805c96c8de", "Kerstin Kaman"),
    CardSet::Antiquities,
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Human", "Artificer"], 2, 2)
        .with_ability(AbilityDef::activated_with_targets(
        "{T}: Prevent the next 2 damage that would be dealt to target artifact creature this turn.",
        &[AbilityCostDef::TapSource],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Creature),
            ]),
        )],
        EffectDef::PreventDamage {
            prevention: DamagePreventionDef::amount(
                DamageEventMatcherDef::to(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
                ValueDef::Constant(2),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// ATQ 3 — Artifact Ward
// Audit: metadata-only — Needs a targeting restriction keyed to the source's card type for “Enchanted creature can't be the target of abilities from artifact sources”; the artifact-source damage prevention and the artifact-creature blocking restriction both exist.
pub(in crate::card::sets) static ARTIFACT_WARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b3a5101a-ec66-4658-950c-9ad49c29b836"),
    "Artifact Ward",
    crate::card::CardArt::new("b3a5101a-ec66-4658-950c-9ad49c29b836", "Douglas Shuler"),
    crate::card::CardSet::Antiquities,
    crate::card::CardRules::unsupported(),
);

// ATQ 4 — Circle of Protection: Artifacts
pub(in crate::card::sets) static CIRCLE_OF_PROTECTION_ARTIFACTS: CardRecord = CardRecord::new_with_legacy_id(
    1451,
    "Circle of Protection: Artifacts",
    CardArt::new("22ebd5a3-fef8-4097-b038-89a6cb38227d", "Pete Venters"),
    CardSet::Antiquities,
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_ability(
        abilities::circle_of_protection(
            "{2}: The next time an artifact source of your choice would deal damage to you this turn, prevent that damage.",
            &[AbilityCostDef::Mana(mana_cost!("{2}"))],
            ObjectPredicateDef::HasType(CardType::Artifact),
        ),
    ),
);

// ATQ 5 — Damping Field
pub(in crate::card::sets) static DAMPING_FIELD: CardRecord = CardRecord::new_with_legacy_id(
    1735,
    "Damping Field",
    CardArt::new("229b1109-4a8d-49d1-9c28-04799aa719a7", "Justin Hampton"),
    CardSet::Antiquities,
    CardRules::new_enchantment(mana_cost!("{2}{W}")).with_ability(AbilityDef::static_ability(
        "Players can't untap more than one artifact during their untap steps.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::players(PlayerSetDef::All),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::UntapAtMostOne(
                ObjectPredicateDef::HasType(CardType::Artifact),
            )),
        },
    )),
);

// ATQ 6 — Martyrs of Korlis
/// "As long as this creature is untapped": the condition rides on the
/// recipient, so tapping it turns the redirection off and untapping turns it
/// back on without the creature being touched.
static MARTYRS_UNTAPPED: EffectRecipientDef = EffectRecipientDef::matching_objects(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::Source,
        ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
    ]),
    &[ZoneKind::Battlefield],
    PlayerRelation::Any,
);

pub(in crate::card::sets) static MARTYRS_OF_KORLIS: CardRecord = CardRecord::new_with_legacy_id(
    1685,
    "Martyrs of Korlis",
    CardArt::new(
        "bde037b9-4947-4ff7-8ea4-e9f1a7e4ab88",
        "Margaret Organ-Kean",
    ),
    CardSet::Antiquities,
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Human"], 1, 6).with_ability(
        AbilityDef::static_ability(
            "As long as this creature is untapped, all damage that would be dealt to you by \
             artifacts is dealt to this creature instead.",
            EffectDef::StaticApply {
                recipient: MARTYRS_UNTAPPED,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::RedirectPlayerDamageToThis(
                    DamageSourceGroupDef::Artifacts,
                )),
            },
        ),
    ),
);

// ATQ 7 — Reverse Polarity
/// Only what artifacts dealt counts, and it counts twice.
static REVERSE_POLARITY_DOUBLED: ScaledValueDef = ScaledValueDef::new(
    ValueDef::DamageTakenThisTurn {
        player: PlayerRelation::You,
        source: Some(DamageSourceGroupDef::Artifacts),
    },
    2,
);

pub(in crate::card::sets) static REVERSE_POLARITY: CardRecord = CardRecord::new_with_legacy_id(
    1715,
    "Reverse Polarity",
    CardArt::new("da7ed8ba-3886-4779-a9b3-6892a7ed3527", "Justin Hampton"),
    CardSet::Antiquities,
    CardRules::new_instant(mana_cost!("{W}{W}")).with_ability(AbilityDef::spell(
        "You gain X life, where X is twice the damage dealt to you so far this turn by \
         artifacts.",
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Scaled(&REVERSE_POLARITY_DOUBLED),
        },
    )),
);

// ATQ 8 — Drafna's Restoration
// Audit: metadata-only — Needs ordered-library inspection, selection, and visibility handling for “Put any number of target artifact cards from target player's graveyard on top of their library in any order”.
pub(in crate::card::sets) static DRAFNA_S_RESTORATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4be2aa3b-207b-4d21-abfb-6788520c7676"),
    "Drafna's Restoration",
    crate::card::CardArt::new("4be2aa3b-207b-4d21-abfb-6788520c7676", "Amy Weber"),
    crate::card::CardSet::Antiquities,
    crate::card::CardRules::unsupported(),
);

// ATQ 9 — Energy Flux
static ENERGY_FLUX_GRANTED_ABILITY: AbilityDef = AbilityDef::triggered(
    "At the beginning of your upkeep, sacrifice this artifact unless you pay {2}.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    EffectDef::PayOr(PayOrDef::unless_mana(
        mana_cost!("{2}"),
        &EffectDef::Sacrifice {
            object: EffectRecipientDef::Source,
        },
    )),
);

pub(in crate::card::sets) static ENERGY_FLUX: CardRecord = CardRecord::new_with_legacy_id(
    113,
    "Energy Flux",
    CardArt::new("bd1f624b-e8f2-462f-838a-7cb9e8fda988", "Kaja Foglio"),
    CardSet::Antiquities,
    CardRules::new_enchantment(mana_cost!("{2}{U}"))
    .with_abilities(&[AbilityDef::static_ability(
        "All artifacts have \"At the beginning of your upkeep, sacrifice this artifact unless you pay {2}.\"",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Artifact), &[ZoneKind::Battlefield], PlayerRelation::Any),
            effect: AppliedEffectDef::add_ability(&ENERGY_FLUX_GRANTED_ABILITY),
        },
    )]),
);

// ATQ 10 — Hurkyl's Recall
pub(in crate::card::sets) static HURKYLS_RECALL: CardRecord = CardRecord::new_with_legacy_id(
    115,
    "Hurkyl's Recall",
    CardArt::new("f32373dd-06d8-45d1-8777-3b1411bcb30a", "NéNé Thomas"),
    CardSet::Antiquities,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Return all artifacts target player owns to their hand.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::MoveToZone {
            counters: None,
            object: EffectRecipientDef::objects_owned_by_target(
                ObjectPredicateDef::HasType(CardType::Artifact),
                TargetIndex::PRIMARY,
            ),
            zone: ZoneKind::Hand,
            controller: None,
            placement: ZonePlacement::Top,
            arrival_effect: None,
            attachment: None,
        },
    )]),
);

// ATQ 11 — Power Artifact
// Audit: metadata-only — Needs cost/mana provenance or dynamic payment support for “Enchanted artifact's activated abilities cost {2} less to activate. This effect can't reduce the mana in that cost to less than one mana”.
pub(in crate::card::sets) static POWER_ARTIFACT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e48bc89e-6da5-43da-b4e0-60d5f850199c"),
    "Power Artifact",
    crate::card::CardArt::new("e48bc89e-6da5-43da-b4e0-60d5f850199c", "Douglas Shuler"),
    crate::card::CardSet::Antiquities,
    crate::card::CardRules::unsupported(),
);

// ATQ 12 — Reconstruction
pub(in crate::card::sets) static RECONSTRUCTION: CardRecord = CardRecord::new_with_legacy_id(
    376,
    "Reconstruction",
    CardArt::new("1aa2d27b-cc25-4baa-86f4-4db45b30e2a4", "Anson Maddocks"),
    CardSet::Antiquities,
    CardRules::new_sorcery(mana_cost!("{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Return target artifact card from your graveyard to your hand.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Artifact),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            },
        )],
        EffectDef::MoveToZone {
            counters: None,
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            arrival_effect: None,
            attachment: None,
            controller: None,
        },
    )]),
);

// ATQ 13 — Sage of Lat-Nam
pub(in crate::card::sets) static SAGE_OF_LAT_NAM: CardRecord = CardRecord::new_with_legacy_id(
    122,
    "Sage of Lat-Nam",
    CardArt::new("b4ff60ce-073c-46b8-807c-8b40467b960c", "Pete Venters"),
    CardSet::Antiquities,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Human", "Artificer"], 1, 2).with_abilities(&[
        AbilityDef::activated(
            "{T}, Sacrifice an artifact: Draw a card.",
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Artifact),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// ATQ 14 — Transmute Artifact
// Audit: metadata-only — Needs the complete qualified library-search and post-search continuation for “Sacrifice an artifact. If you do, search your library for an artifact card. If that card's mana value is less than or equal to the sacrificed artifact's mana value, put it onto the…”.
pub(in crate::card::sets) static TRANSMUTE_ARTIFACT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6eab6765-eba3-4844-81ca-ae37a6e903df"),
    "Transmute Artifact",
    crate::card::CardArt::new("6eab6765-eba3-4844-81ca-ae37a6e903df", "Anson Maddocks"),
    crate::card::CardSet::Antiquities,
    crate::card::CardRules::unsupported(),
);

// ATQ 15 — Artifact Possession
// Audit: metadata-only — Needs a trigger event for a player activating an ability of a named permanent, including inspection of whether {T} is among its costs. The tap half is available.
pub(in crate::card::sets) static ARTIFACT_POSSESSION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("587d6ac8-fad8-49e0-862e-636e06628ff9"),
    "Artifact Possession",
    crate::card::CardArt::new("587d6ac8-fad8-49e0-862e-636e06628ff9", "Christopher Rush"),
    crate::card::CardSet::Antiquities,
    crate::card::CardRules::unsupported(),
);

// ATQ 16 — Gate to Phyrexia
pub(in crate::card::sets) static GATE_TO_PHYREXIA: CardRecord = CardRecord::new_with_legacy_id(
    1461,
    "Gate to Phyrexia",
    CardArt::new("1f372950-6693-4838-80ef-8fd9aa3e0349", "Sandra Everingham"),
    CardSet::Antiquities,
    CardRules::new_enchantment(mana_cost!("{B}{B}")).with_ability(
        AbilityDef::activated_with_targets(
            "Sacrifice a creature: Destroy target artifact. Activate only during your upkeep and \
             only once each turn.",
            &[AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::You,
            }],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Artifact),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
        )
        .with_activation_timing(ActivationTimingDef::YourUpkeep)
        .once_each_turn(),
    ),
);

// ATQ 17 — Haunting Wind
// Audit: metadata-only — Needs artifact tap and non-tap activated-ability events, including inspection of the triggering activation's costs.
pub(in crate::card::sets) static HAUNTING_WIND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a2f6ef2f-a3a2-4e1f-b7eb-59abc8414114"),
    "Haunting Wind",
    crate::card::CardArt::new("a2f6ef2f-a3a2-4e1f-b7eb-59abc8414114", "Jeff A. Menges"),
    crate::card::CardSet::Antiquities,
    crate::card::CardRules::unsupported(),
);

// ATQ 18 — Phyrexian Gremlins
static GREMLIN_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Artifact),
)];

pub(in crate::card::sets) static PHYREXIAN_GREMLINS: CardRecord = CardRecord::new_with_legacy_id(
    1682,
    "Phyrexian Gremlins",
    CardArt::new("21a985a9-5612-4844-982e-fd1aa6249770", "Amy Weber"),
    CardSet::Antiquities,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Phyrexian", "Gremlin"], 1, 1).with_abilities(
        &[
            AbilityDef::static_ability(
                "You may choose not to untap this creature during your untap step.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayChooseNotToUntap),
                },
            ),
            AbilityDef::activated_with_targets(
                "{T}: Tap target artifact. It doesn't untap during its controller's untap step \
                 for as long as this creature remains tapped.",
                &[AbilityCostDef::TapSource],
                &GREMLIN_TARGET,
                EffectDef::Sequence(&[
                    EffectDef::Tap {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
                        duration: ResolvedEffectDurationDef::WhileSourceTapped,
                    },
                ]),
            ),
        ],
    ),
);

// ATQ 19 — Priest of Yawgmoth
// Audit: metadata-only — Needs cost/mana provenance or dynamic payment support for “{T}, Sacrifice an artifact: Add an amount of {B} equal to the sacrificed artifact's mana value”.
pub(in crate::card::sets) static PRIEST_OF_YAWGMOTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c9fd4054-42fc-4f95-a6f7-369a5da43dd5"),
    "Priest of Yawgmoth",
    crate::card::CardArt::new("c9fd4054-42fc-4f95-a6f7-369a5da43dd5", "Mark Tedin"),
    crate::card::CardSet::Antiquities,
    crate::card::CardRules::unsupported(),
);

// ATQ 20 — Xenic Poltergeist
/// Animation is a type and a base size together. Both numbers are the same
/// value, read off the artifact the ability pointed at, and frozen as the
/// ability resolves -- an artifact's mana value does not move afterwards.
static XENIC_POLTERGEIST_ANIMATION: [AppliedEffectDef; 2] = [
    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
    AppliedEffectDef::set_base_power_toughness(
        ValueDef::TargetManaValue(TargetIndex::PRIMARY),
        ValueDef::TargetManaValue(TargetIndex::PRIMARY),
    ),
];

static XENIC_POLTERGEIST_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Artifact),
        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
    ]),
)];

pub(in crate::card::sets) static XENIC_POLTERGEIST: CardRecord = CardRecord::new_with_legacy_id(
    1815,
    "Xenic Poltergeist",
    CardArt::new("5149ffff-d38f-458e-bcfa-a4b6b332a0b4", "Dan Frazier"),
    CardSet::Antiquities,
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Spirit"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Until your next upkeep, target noncreature artifact becomes an artifact \
             creature with power and toughness each equal to its mana value.",
            &[AbilityCostDef::TapSource],
            &XENIC_POLTERGEIST_TARGET,
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&XENIC_POLTERGEIST_ANIMATION),
                duration: ResolvedEffectDurationDef::UntilYourNextUpkeep,
            },
        ),
    ),
);

// ATQ 21 — Yawgmoth Demon
// Audit: metadata-only — Needs an optional artifact-sacrifice choice whose declined or impossible branch taps the source and deals damage.
pub(in crate::card::sets) static YAWGMOTH_DEMON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("04bbd231-0d5f-4cbf-92a7-10d2c5c4b82c"),
    "Yawgmoth Demon",
    crate::card::CardArt::new("04bbd231-0d5f-4cbf-92a7-10d2c5c4b82c", "Sandra Everingham"),
    crate::card::CardSet::Antiquities,
    crate::card::CardRules::unsupported(),
);

// ATQ 22 — Artifact Blast
pub(in crate::card::sets) static ARTIFACT_BLAST: CardRecord = CardRecord::new_with_legacy_id(
    377,
    "Artifact Blast",
    CardArt::new("1506d99d-7b2e-4101-84a5-c950dadb263a", "Mark Poole"),
    CardSet::Antiquities,
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[AbilityDef::counter_target(
        "Counter target artifact spell.",
        &AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::HasType(CardType::Artifact)),
    )]),
);

// ATQ 23 — Atog
pub(in crate::card::sets) static ATOG: CardRecord = CardRecord::new_with_legacy_id(
    2,
    "Atog",
    CardArt::new("2249fc40-4412-48fd-800a-7ea3678aee3f", "Jesper Myrfors"),
    CardSet::Antiquities,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Atog"], 1, 2).with_abilities(&[
        AbilityDef::activated(
            "Sacrifice an artifact: This creature gets +2/+2 until end of turn.",
            &[AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::HasType(CardType::Artifact),
                controller: PlayerRelation::You,
            }],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ATQ 23† — Atog (alternate printing)

// ATQ 24 — Detonate
/// The mana value is read off the spell's own X, so what Detonate can hit
/// depends on what was paid for it.
static DETONATE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Artifact),
        ObjectPredicateDef::ManaValueEqualTo(ValueDef::ChosenX),
    ]),
)];

/// The damage reads the controller as the spell resolves, so it still lands
/// even though the artifact has just been destroyed.
static DETONATE_EFFECT: [EffectDef; 2] = [
    EffectDef::Destroy {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        can_regenerate: false,
        then: None,
    },
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
        amount: ValueDef::ChosenX,
    },
];

pub(in crate::card::sets) static DETONATE: CardRecord = CardRecord::new_with_legacy_id(
    8,
    "Detonate",
    CardArt::new(
        "ffd7eb90-ae95-49df-898a-9510187bce1c",
        "Randy Asplund-Faith",
    ),
    CardSet::Antiquities,
    CardRules::new_sorcery(mana_cost!("{X}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Destroy target artifact with mana value X. It can't be regenerated. Detonate deals X damage to that artifact's controller.",
            &DETONATE_TARGET,
            EffectDef::Sequence(&DETONATE_EFFECT),
        ),
    ]),
);

// ATQ 25 — Dwarven Weaponsmith
pub(in crate::card::sets) static DWARVEN_WEAPONSMITH: CardRecord = CardRecord::new_with_legacy_id(
    1458,
    "Dwarven Weaponsmith",
    CardArt::new("0848d94a-2704-460f-986b-b192dd6d26b7", "Mark Poole"),
    CardSet::Antiquities,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Dwarf", "Artificer"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice an artifact: Put a +1/+1 counter on target creature. Activate only \
             during your upkeep.",
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Artifact),
                    controller: PlayerRelation::You,
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
        )
        .with_activation_timing(ActivationTimingDef::YourUpkeep),
    ),
);

// ATQ 26 — Goblin Artisans
// Audit: metadata-only — Needs a deterministic recorded coin-flip choice and both result branches for “{T}: Flip a coin. If you win the flip, draw a card. If you lose the flip, counter target artifact spell you control that isn't the target of an ability from another creature named Goblin…”.
pub(in crate::card::sets) static GOBLIN_ARTISANS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6669d96e-9a7b-4427-a477-f4e76831f593"),
    "Goblin Artisans",
    crate::card::CardArt::new("6669d96e-9a7b-4427-a477-f4e76831f593", "Julie Baroh"),
    crate::card::CardSet::Antiquities,
    crate::card::CardRules::unsupported(),
);

// ATQ 27 — Orcish Mechanics
pub(in crate::card::sets) static ORCISH_MECHANICS: CardRecord = CardRecord::new_with_legacy_id(
    37,
    "Orcish Mechanics",
    CardArt::new("5e34fc6b-5f00-4a22-9ee2-afc1caf99961", "Pete Venters"),
    CardSet::Antiquities,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Orc"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice an artifact: This creature deals 2 damage to any target.",
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Artifact),
                    controller: PlayerRelation::You,
                },
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// ATQ 28 — Shatterstorm
pub(in crate::card::sets) static SHATTERSTORM: CardRecord = CardRecord::new_with_legacy_id(
    378,
    "Shatterstorm",
    CardArt::new("0987461a-45c0-4956-8627-cd27a7e038d0", "Dan Frazier"),
    CardSet::Antiquities,
    CardRules::new_sorcery(mana_cost!("{2}{R}{R}")).with_abilities(&[AbilityDef::spell(
        "Destroy all artifacts. They can't be regenerated.",
        EffectDef::Destroy {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Artifact),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            can_regenerate: false,
            then: None,
        },
    )]),
);

// ATQ 29 — Argothian Pixies
/// Both halves of the Pixies read the same set, so the blocking restriction
/// and the prevention cannot drift apart.
static ARTIFACT_CREATURE: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Artifact),
    ObjectPredicateDef::HasType(CardType::Creature),
]);

pub(in crate::card::sets) static ARGOTHIAN_PIXIES: CardRecord = CardRecord::new_with_legacy_id(
    108,
    "Argothian Pixies",
    CardArt::new("5712e87a-2381-4f5b-a853-6973841f9bf1", "Amy Weber"),
    CardSet::Antiquities,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Faerie"], 2, 1).with_abilities(&[
        AbilityDef::static_ability(
            "This creature can't be blocked by artifact creatures.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ARTIFACT_CREATURE,
                )),
            },
        ),
        AbilityDef::static_ability(
            "Prevent all damage that would be dealt to this creature by artifact creatures.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::prevent_damage_from(ARTIFACT_CREATURE),
            },
        ),
    ]),
);

// ATQ 30 — Argothian Treefolk
pub(in crate::card::sets) static ARGOTHIAN_TREEFOLK: CardRecord = CardRecord::new_with_legacy_id(
    1418,
    "Argothian Treefolk",
    CardArt::new("8db8882e-4db6-4e3c-9e9e-8c71d557a071", "Amy Weber"),
    CardSet::Antiquities,
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Treefolk"], 3, 5).with_abilities(&[
        AbilityDef::static_ability(
            "Prevent all damage that would be dealt to this creature by artifact sources.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::prevent_damage_from(ObjectPredicateDef::HasType(
                    CardType::Artifact,
                )),
            },
        ),
    ]),
);

// ATQ 31 — Citanul Druid
pub(in crate::card::sets) static CITANUL_DRUID: CardRecord = CardRecord::new_with_legacy_id(
    379,
    "Citanul Druid",
    CardArt::new("f8a130dc-3b1f-4fae-8459-b26bb5647fec", "Jeff A. Menges"),
    CardSet::Antiquities,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Druid"], 1, 1).with_abilities(&[
        AbilityDef::triggered(
            "Whenever an opponent casts an artifact spell, put a +1/+1 counter on this creature.",
            TriggerEventDef::SpellCast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
            ])),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// ATQ 32 — Crumble
pub(in crate::card::sets) static CRUMBLE: CardRecord = CardRecord::new_with_legacy_id(
    380,
    "Crumble",
    CardArt::new("d2101f86-8d3c-4ba8-ac42-bd3df0644280", "Jesper Myrfors"),
    CardSet::Antiquities,
    CardRules::new_instant(mana_cost!("{G}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Destroy target artifact. It can't be regenerated. That artifact's controller gains life equal to its mana value.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Artifact),
            )],
            EffectDef::Sequence(&[
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: false,
                    then: None,
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                    amount: ValueDef::TargetManaValue(TargetIndex::PRIMARY),
                },
            ]),
        ),
    ]),
);

// ATQ 33 — Gaea's Avenger
// Audit: partial — Its power and toughness are a battlefield-only continuous effect rather than a characteristic-defining ability, so they read as printed in every other zone.
static ARTIFACTS_YOUR_OPPONENTS_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Artifact),
    &[ZoneKind::Battlefield],
    PlayerRelation::Opponent,
);

pub(in crate::card::sets) static GAEAS_AVENGER: CardRecord = CardRecord::new_with_legacy_id(
    1468,
    "Gaea's Avenger",
    CardArt::new("39d763bd-b0a9-46ba-bcd2-9304063446f2", "Pete Venters"),
    CardSet::Antiquities,
    // The printed "1 plus" is the body itself, so the counted bonus only has
    // to supply the rest.
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Treefolk"], 1, 1).with_ability(
        AbilityDef::static_ability(
            "Gaea's Avenger's power and toughness are each equal to 1 plus the number of \
             artifacts your opponents control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::CountMatchingObjects(&ARTIFACTS_YOUR_OPPONENTS_CONTROL),
                    ValueDef::CountMatchingObjects(&ARTIFACTS_YOUR_OPPONENTS_CONTROL),
                ),
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "A characteristic-defining ability sets power and toughness in every zone. This is a \
             battlefield-only continuous effect, so the value is right wherever the card is \
             played and absent for anything reading it in another zone.",
        )),
    ),
);

// ATQ 34 — Powerleech
// Audit: metadata-only — Needs opponent-artifact tap and non-tap activated-ability events, including inspection of activation costs.
pub(in crate::card::sets) static POWERLEECH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ae1d7b09-3a1f-410f-b330-04ae768b0455"),
    "Powerleech",
    crate::card::CardArt::new("ae1d7b09-3a1f-410f-b330-04ae768b0455", "Christopher Rush"),
    crate::card::CardSet::Antiquities,
    crate::card::CardRules::unsupported(),
);

// ATQ 35 — Titania's Song
// Audit: metadata-only — Needs static animation of every noncreature artifact with dynamic mana-value power/toughness and ability removal.
pub(in crate::card::sets) static TITANIA_S_SONG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("583a53af-2e2a-4f3f-8eab-bd874c6ed80a"),
    "Titania's Song",
    crate::card::CardArt::new("583a53af-2e2a-4f3f-8eab-bd874c6ed80a", "Kerstin Kaman"),
    crate::card::CardSet::Antiquities,
    crate::card::CardRules::unsupported(),
);

// ATQ 36 — Amulet of Kroog
pub(in crate::card::sets) static AMULET_OF_KROOG: CardRecord = CardRecord::new_with_legacy_id(
    1411,
    "Amulet of Kroog",
    CardArt::new(
        "b094f8dd-0184-41a2-9767-e848a6e4eac1",
        "Margaret Organ-Kean",
    ),
    CardSet::Antiquities,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{2}, {T}: Prevent the next 1 damage that would be dealt to any target this turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::amount(
                    DamageEventMatcherDef::to(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ATQ 37 — Armageddon Clock
pub(in crate::card::sets) static ARMAGEDDON_CLOCK: CardRecord = CardRecord::new_with_legacy_id(
    1817,
    "Armageddon Clock",
    CardArt::new("44a31889-6a8d-450c-a73d-381a7ff28bf9", "Amy Weber"),
    CardSet::Antiquities,
    CardRules::new_artifact(mana_cost!("{6}")).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, put a doom counter on this artifact.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::Doom,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::triggered(
            "At the beginning of your draw step, this artifact deals damage equal to the \
             number of doom counters on it to each player.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Draw,
                player: PlayerRelation::You,
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::EachPlayer,
                amount: ValueDef::CountersOnSource(CounterKind::Doom),
            },
        ),
        // Everyone can wind it back, and only in an upkeep -- which is after
        // the counter goes on and before the draw step it pays for.
        AbilityDef::activated(
            "{4}: Remove a doom counter from this artifact. Any player may activate this \
             ability but only during any upkeep step.",
            &[AbilityCostDef::Mana(mana_cost!("{4}"))],
            EffectDef::RemoveCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::Doom,
                amount: ValueDef::Constant(1),
            },
        )
        .with_activation_timing(ActivationTimingDef::AnyUpkeep)
        .open_to_any_player(),
    ]),
);

// ATQ 38 — Ashnod's Altar
// Audit: metadata-only — Needs mana-ability activation to select and sacrifice a different creature; the mana runtime can currently sacrifice only the source.
pub(in crate::card::sets) static ASHNOD_S_ALTAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cdcccb0f-ce96-453b-9e82-41d87f52e58b"),
    "Ashnod's Altar",
    crate::card::CardArt::new("cdcccb0f-ce96-453b-9e82-41d87f52e58b", "Anson Maddocks"),
    crate::card::CardSet::Antiquities,
    crate::card::CardRules::unsupported(),
);

// ATQ 39 — Ashnod's Battle Gear
pub(in crate::card::sets) static ASHNODS_BATTLE_GEAR: CardRecord = CardRecord::new_with_legacy_id(
    1664,
    "Ashnod's Battle Gear",
    CardArt::new("aeeec853-dd3f-4ac3-8b20-c07fada8888f", "Mark Poole"),
    CardSet::Antiquities,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::static_ability(
            "You may choose not to untap this artifact during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayChooseNotToUntap),
            },
        ),
        AbilityDef::activated_with_targets(
            "{2}, {T}: Target creature you control gets +2/-2 for as long as this artifact \
             remains tapped.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(-2),
                ),
                duration: ResolvedEffectDurationDef::WhileSourceTapped,
            },
        ),
    ]),
);

// ATQ 40 — Ashnod's Transmogrant
/// A counter and a type, both permanent: the artifact is gone by the time
/// either lands, so nothing is scoped to it surviving.
static ASHNODS_TRANSMOGRANT_EFFECT: [EffectDef; 2] = [
    EffectDef::AddCounters {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        kind: CounterKind::PlusOnePlusOne,
        amount: ValueDef::Constant(1),
    },
    EffectDef::Apply {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        effect: AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Artifact)),
        duration: ResolvedEffectDurationDef::Permanent,
    },
];

static ASHNODS_TRANSMOGRANT_TARGET: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one_permanent(
        ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Artifact)),
        ]),
    )];

pub(in crate::card::sets) static ASHNODS_TRANSMOGRANT: CardRecord = CardRecord::new_with_legacy_id(
    1810,
    "Ashnod's Transmogrant",
    CardArt::new("2aa5b289-36ba-49b1-a5ac-f23bf71f8241", "Mark Tedin"),
    CardSet::Antiquities,
    CardRules::new_artifact(mana_cost!("{1}")).with_ability(AbilityDef::activated_with_targets(
        "{T}, Sacrifice this artifact: Put a +1/+1 counter on target nonartifact creature. \
         That creature becomes an artifact in addition to its other types.",
        &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
        &ASHNODS_TRANSMOGRANT_TARGET,
        EffectDef::Sequence(&ASHNODS_TRANSMOGRANT_EFFECT),
    )),
);

// ATQ 41 — Battering Ram
static BATTERING_RAM_BANDING: AbilityDef = abilities::banding();

pub(in crate::card::sets) static BATTERING_RAM: CardRecord = CardRecord::new_with_legacy_id(
    1797,
    "Battering Ram",
    CardArt::new("f7a69e35-d209-41c0-aa3c-c78414617075", "Jeff A. Menges"),
    CardSet::Antiquities,
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Construct"], 1, 1).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of combat on your turn, this creature gains banding until end \
             of combat.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&BATTERING_RAM_BANDING),
                duration: ResolvedEffectDurationDef::UntilEndOfCombat,
            },
        ),
        AbilityDef::triggered(
            "Whenever this creature becomes blocked by a Wall, destroy that Wall at end of \
             combat.",
            TriggerEventDef::BecomesBlockedBy {
                blocker: ObjectPredicateDef::Subtype("Wall"),
            },
            abilities::destroy_triggering_object_at_end_of_combat(),
        ),
    ]),
);

// ATQ 42 — Bronze Tablet
// Audit: metadata-only — Needs permanent card-ownership changes plus the opponent's life-payment choice after the linked cards are exiled.
pub(in crate::card::sets) static BRONZE_TABLET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6fb10552-dd47-4f8a-ac7c-8c2b61e56736"),
    "Bronze Tablet",
    crate::card::CardArt::new("6fb10552-dd47-4f8a-ac7c-8c2b61e56736", "Tom Wänerstrand"),
    crate::card::CardSet::Antiquities,
    crate::card::CardRules::unsupported(),
);

// ATQ 43 — Candelabra of Tawnos
/// "X target lands": the count is the X that was paid, not a range chosen
/// afterwards, so an X larger than the number of lands on the battlefield
/// offers no declaration at all.
static CANDELABRA_X_LANDS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_chosen_x(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Land),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
)];

pub(in crate::card::sets) static CANDELABRA_OF_TAWNOS: CardRecord = CardRecord::new_with_legacy_id(
    1829,
    "Candelabra of Tawnos",
    CardArt::new("35a335bf-7358-460f-b7c9-1e8bc4300f64", "Douglas Shuler"),
    CardSet::Antiquities,
    // Any lands, not just your own: the printed text says "lands", which is
    // what makes the card an answer as well as an engine.
    CardRules::new_artifact(mana_cost!("{1}")).with_ability(AbilityDef::activated_with_targets(
        "{X}, {T}: Untap X target lands.",
        &[
            AbilityCostDef::Mana(mana_cost!("{X}")),
            AbilityCostDef::TapSource,
        ],
        &CANDELABRA_X_LANDS,
        EffectDef::Untap {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    )),
);

// ATQ 44 — Clay Statue
pub(in crate::card::sets) static CLAY_STATUE: CardRecord = CardRecord::new_with_legacy_id(
    1375,
    "Clay Statue",
    CardArt::new("64975352-8d35-4d02-94ac-fa0c6ee12409", "Jesper Myrfors"),
    CardSet::Antiquities,
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Golem"], 3, 1).with_abilities(&[
        abilities::regenerate_self(
            "{2}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{2}"))],
        ),
    ]),
);

// ATQ 45 — Clockwork Avian
// Audit: metadata-only — Needs card-specific counter state and counter-consuming effects for “{X}, {T}: Put up to X +1/+0 counters on this creature. This ability can't cause the total number of +1/+0 counters on this creature to be greater than four. Activate only during your upkeep”.
pub(in crate::card::sets) static CLOCKWORK_AVIAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1dea8c2f-4aea-478d-aee7-cba1f74edd6c"),
    "Clockwork Avian",
    crate::card::CardArt::new(
        "1dea8c2f-4aea-478d-aee7-cba1f74edd6c",
        "Randy Asplund-Faith",
    ),
    crate::card::CardSet::Antiquities,
    crate::card::CardRules::unsupported(),
);

// ATQ 46 — Colossus of Sardia
pub(in crate::card::sets) static COLOSSUS_OF_SARDIA: CardRecord = CardRecord::new_with_legacy_id(
    1464,
    "Colossus of Sardia",
    CardArt::new("067c44e9-1b23-42fd-9acb-daafb62c32a2", "Jesper Myrfors"),
    CardSet::Antiquities,
    CardRules::new_artifact_creature(mana_cost!("{9}"), &["Golem"], 9, 9).with_abilities(&[
        abilities::trample(),
        AbilityDef::static_ability(
            "This creature doesn't untap during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
        AbilityDef::activated(
            "{9}: Untap this creature. Activate only during your upkeep.",
            &[AbilityCostDef::Mana(mana_cost!("{9}"))],
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        )
        .with_activation_timing(ActivationTimingDef::YourUpkeep),
    ]),
);

// ATQ 47 — Coral Helm
pub(in crate::card::sets) static CORAL_HELM: CardRecord = CardRecord::new_with_legacy_id(
    1807,
    "Coral Helm",
    CardArt::new("6c6df9db-0a46-40a5-ae9d-59f47dae9056", "Amy Weber"),
    CardSet::Antiquities,
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::activated_with_targets(
        "{3}, Discard a card at random: Target creature gets +2/+2 until end of turn.",
        &[
            AbilityCostDef::Mana(mana_cost!("{3}")),
            AbilityCostDef::DiscardCardsAtRandom(1),
        ],
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
    )),
);

// ATQ 48 — Cursed Rack
// Audit: metadata-only — Needs a hidden-zone decision and continuation for “The chosen player's maximum hand size is four”.
pub(in crate::card::sets) static CURSED_RACK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("720d871d-1e7b-482e-bd1e-8ec79519fb86"),
    "Cursed Rack",
    crate::card::CardArt::new("720d871d-1e7b-482e-bd1e-8ec79519fb86", "Richard Thomas"),
    crate::card::CardSet::Antiquities,
    crate::card::CardRules::unsupported(),
);

// ATQ 49 — Dragon Engine
pub(in crate::card::sets) static DRAGON_ENGINE: CardRecord = CardRecord::new_with_legacy_id(
    381,
    "Dragon Engine",
    CardArt::new("07793a71-1106-4303-b620-e403bd378020", "Anson Maddocks"),
    CardSet::Antiquities,
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Construct"], 1, 3).with_abilities(&[
        AbilityDef::activated(
            "{2}: This creature gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{2}"))],
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

// ATQ 50 — Feldon's Cane
/// The documented composition: move the then shuffle the library they
/// arrived in.
static FELDONS_CANE_SHUFFLE: [EffectDef; 2] = [
    EffectDef::MoveToZone {
        counters: None,
        object: EffectRecipientDef::matching_objects(
            ObjectPredicateDef::Any,
            &[ZoneKind::Graveyard],
            PlayerRelation::You,
        ),
        zone: ZoneKind::Library,
        controller: None,
        placement: ZonePlacement::Top,
        arrival_effect: None,
        attachment: None,
    },
    EffectDef::ShuffleLibrary {
        player: EffectRecipientDef::Controller,
    },
];

pub(in crate::card::sets) static FELDONS_CANE: CardRecord = CardRecord::new_with_legacy_id(
    1480,
    "Feldon's Cane",
    CardArt::new("bb6af436-bcfd-4d47-a1aa-e84b587a725a", "Mark Tedin"),
    CardSet::Antiquities,
    CardRules::new_artifact(mana_cost!("{1}")).with_ability(AbilityDef::activated(
        "{T}, Exile this artifact: Shuffle your graveyard into your library.",
        &[AbilityCostDef::TapSource, AbilityCostDef::ExileSource],
        EffectDef::Sequence(&FELDONS_CANE_SHUFFLE),
    )),
);

// ATQ 51 — Golgothian Sylex
// Audit: partial — Its expansion predicate follows physical identity rather than the permanent's current copied name.
pub(in crate::card::sets) static GOLGOTHIAN_SYLEX: CardRecord = CardRecord::new_with_legacy_id(
    382,
    "Golgothian Sylex",
    CardArt::new("856be1dd-a20b-49c2-be9d-7db76c7efd8b", "Kerstin Kaman"),
    CardSet::Antiquities,
    CardRules::new_artifact(mana_cost!("{4}")).with_abilities(&[AbilityDef::activated(
        "{1}, {T}: Each nontoken permanent with a name originally printed in the Antiquities expansion is sacrificed by its controller.",
        &[
            AbilityCostDef::Mana(mana_cost!("{1}")),
            AbilityCostDef::TapSource,
        ],
        EffectDef::Sacrifice {
            object: EffectRecipientDef::matching_objects(ObjectPredicateDef::DebutSet(CardSet::Antiquities), &[ZoneKind::Battlefield], PlayerRelation::Any),
        },
    )
    .with_coverage(AbilityCoverageDef::partial(
        "The expansion predicate follows physical identity rather than the permanent's current copied name.",
    ))]),
);

// ATQ 52 — Grapeshot Catapult
pub(in crate::card::sets) static GRAPESHOT_CATAPULT: CardRecord = CardRecord::new_with_legacy_id(
    383,
    "Grapeshot Catapult",
    CardArt::new("4c7a7348-c82e-453c-975c-e5365e152a3a", "Dan Frazier"),
    CardSet::Antiquities,
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Construct"], 2, 3).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: This creature deals 1 damage to target creature with flying.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                ]),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// ATQ 53 — Ivory Tower
pub(in crate::card::sets) static IVORY_TOWER: CardRecord = CardRecord::new_with_legacy_id(
    50,
    "Ivory Tower",
    CardArt::new(
        "a5f23039-45ca-4c15-af50-bfd40ea26453",
        "Margaret Organ-Kean",
    ),
    CardSet::Antiquities,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, you gain X life, where X is the number of cards in your hand minus 4.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::CardsInHandAbove {
                    player: PlayerRelation::You,
                    threshold: 4,
                },
            },
        ),
    ]),
);

// ATQ 54 — Jalum Tome
pub(in crate::card::sets) static JALUM_TOME: CardRecord = CardRecord::new_with_legacy_id(
    384,
    "Jalum Tome",
    CardArt::new("5a5b7c5a-ee63-4a1b-9a0f-fb0a309168df", "Tom Wänerstrand"),
    CardSet::Antiquities,
    CardRules::new_artifact(mana_cost!("{3}"))
        .with_subtypes(&["Book"])
        .with_abilities(&[AbilityDef::activated(
            "{2}, {T}: Draw a card, then discard a card.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}")),
                AbilityCostDef::TapSource,
            ],
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
        )]),
);

// ATQ 55 — Mightstone
pub(in crate::card::sets) static MIGHTSTONE: CardRecord = CardRecord::new_with_legacy_id(
    385,
    "Mightstone",
    CardArt::new("b28ba599-5299-4831-a118-1712ada10ef6", "Pete Venters"),
    CardSet::Antiquities,
    CardRules::new_artifact(mana_cost!("{4}")).with_abilities(&[AbilityDef::static_ability(
        "Attacking creatures get +1/+0.",
        EffectDef::StaticApply {
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
        },
    )]),
);

// ATQ 56 — Millstone
pub(in crate::card::sets) static MILLSTONE: CardRecord = CardRecord::new_with_legacy_id(
    386,
    "Millstone",
    CardArt::new("107646bc-2181-49f4-8821-1eaa46291855", "Kaja Foglio"),
    CardSet::Antiquities,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{2}, {T}: Target player mills two cards.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
                binding: None,
                then: None,
            },
        ),
    ]),
);

// ATQ 57 — Mishra's War Machine
/// The declined branch, which is one clause rather than two: "if it deals
/// damage to you this way" is only ever true here, so the tap belongs to the
/// same branch as the damage instead of watching for it.
static MISHRAS_WAR_MACHINE_UNPAID: [EffectDef; 2] = [
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(3),
    },
    EffectDef::Tap {
        object: EffectRecipientDef::Source,
    },
];

static MISHRAS_WAR_MACHINE_UNPAID_SEQUENCE: EffectDef =
    EffectDef::Sequence(&MISHRAS_WAR_MACHINE_UNPAID);

pub(in crate::card::sets) static MISHRA_S_WAR_MACHINE: CardRecord = CardRecord::new_with_legacy_id(
    1835,
    "Mishra's War Machine",
    CardArt::new("8f6b4652-a1d4-418f-a89b-6a977a920a9e", "Amy Weber"),
    CardSet::Antiquities,
    CardRules::new_artifact_creature(mana_cost!("{7}"), &["Juggernaut"], 5, 5).with_abilities(&[
        abilities::banding(),
        AbilityDef::triggered(
            "At the beginning of your upkeep, this creature deals 3 damage to you unless you \
             discard a card. If it deals damage to you this way, tap it.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::PayOr(PayOrDef {
                payment: EffectPaymentDef::discard(PlayerSetDef::Related(PlayerRelation::You), 1),
                if_paid: None,
                otherwise: Some(&MISHRAS_WAR_MACHINE_UNPAID_SEQUENCE),
                visibility: ChoiceVisibilityDef::Public,
            }),
        ),
    ]),
);

// ATQ 58 — Obelisk of Undoing
pub(in crate::card::sets) static OBELISK_OF_UNDOING: CardRecord = CardRecord::new_with_legacy_id(
    387,
    "Obelisk of Undoing",
    CardArt::new("1ba61ccd-4429-4f7c-b9f3-30867878d88e", "Tom Wänerstrand"),
    CardSet::Antiquities,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{6}, {T}: Return target permanent you both own and control to your hand.",
            &[
                AbilityCostDef::Mana(mana_cost!("{6}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::MoveToZone {
                counters: None,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
                controller: None,
            },
        ),
    ]),
);

// ATQ 59 — Onulet
pub(in crate::card::sets) static ONULET: CardRecord = CardRecord::new_with_legacy_id(
    388,
    "Onulet",
    CardArt::new("d77fe8e2-8438-473e-ace5-01baddd2c4ed", "Anson Maddocks"),
    CardSet::Antiquities,
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Construct"], 2, 2).with_abilities(&[
        abilities::dies_trigger(
            "When this creature dies, you gain 2 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// ATQ 60 — Ornithopter
pub(in crate::card::sets) static ORNITHOPTER: CardRecord = CardRecord::new_with_legacy_id(
    389,
    "Ornithopter",
    CardArt::new("59cc9bdb-7cf2-4795-bac7-ffff605c9eb0", "Amy Weber"),
    CardSet::Antiquities,
    CardRules::new_artifact_creature(mana_cost!("{0}"), &["Thopter"], 0, 2)
        .with_abilities(&[abilities::flying()]),
);

// ATQ 61 — Primal Clay
// Audit: metadata-only — Needs a characteristic-layer effect or dynamic value for “As this creature enters, it becomes your choice of a 3/3 artifact creature, a 2/2 artifact creature with flying, or a 1/6 Wall artifact creature with defender in addition to its other types”.
pub(in crate::card::sets) static PRIMAL_CLAY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ab9d0e3f-cf7c-41f8-bcd7-bb08ea8cc2f8"),
    "Primal Clay",
    crate::card::CardArt::new("ab9d0e3f-cf7c-41f8-bcd7-bb08ea8cc2f8", "Kaja Foglio"),
    crate::card::CardSet::Antiquities,
    crate::card::CardRules::unsupported(),
);

// ATQ 62 — Rakalite
static RAKALITE_SHIELD: [EffectDef; 2] = [
    EffectDef::PreventDamage {
        prevention: DamagePreventionDef::amount(
            DamageEventMatcherDef::to(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
            ValueDef::Constant(1),
        ),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
    EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
        "At the beginning of the next end step, return this artifact to its owner's hand.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::End,
            player: PlayerRelation::Any,
        },
        EffectDef::MoveToZone {
            counters: None,
            object: EffectRecipientDef::Source,
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            arrival_effect: None,
            attachment: None,
            controller: None,
        },
    ))),
];

pub(in crate::card::sets) static RAKALITE: CardRecord = CardRecord::new_with_legacy_id(
    1583,
    "Rakalite",
    CardArt::new("0fd7c711-3ff4-4691-914f-242e6737066c", "Christopher Rush"),
    CardSet::Antiquities,
    CardRules::new_artifact(mana_cost!("{6}")).with_ability(AbilityDef::activated_with_targets(
        "{2}: Prevent the next 1 damage that would be dealt to any target this turn. Return \
         this artifact to its owner's hand at the beginning of the next end step.",
        &[AbilityCostDef::Mana(mana_cost!("{2}"))],
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::Sequence(&RAKALITE_SHIELD),
    )),
);

// ATQ 63 — Rocket Launcher
// Audit: metadata-only — Needs continuous-control activation timing and a delayed self-destruction trigger created by activation.
pub(in crate::card::sets) static ROCKET_LAUNCHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d5bb2093-78a8-4a6c-abe7-9a5afc181ec5"),
    "Rocket Launcher",
    crate::card::CardArt::new("d5bb2093-78a8-4a6c-abe7-9a5afc181ec5", "Pete Venters"),
    crate::card::CardSet::Antiquities,
    crate::card::CardRules::unsupported(),
);

// ATQ 64 — Shapeshifter
// Audit: metadata-only — Needs a characteristic-layer effect or dynamic value for “Shapeshifter's power is equal to the last chosen number and its toughness is equal to 7 minus that number”.
pub(in crate::card::sets) static SHAPESHIFTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cc278af4-b60d-41b7-b9d7-36c8aefca1a7"),
    "Shapeshifter",
    crate::card::CardArt::new("cc278af4-b60d-41b7-b9d7-36c8aefca1a7", "Dan Frazier"),
    crate::card::CardSet::Antiquities,
    crate::card::CardRules::unsupported(),
);

// ATQ 65 — Staff of Zegon
pub(in crate::card::sets) static STAFF_OF_ZEGON: CardRecord = CardRecord::new_with_legacy_id(
    390,
    "Staff of Zegon",
    CardArt::new("a6bf858d-bba9-4a16-9045-55384b1de633", "Mark Poole"),
    CardSet::Antiquities,
    CardRules::new_artifact(mana_cost!("{4}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{3}, {T}: Target creature gets -2/-0 until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{3}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ATQ 66 — Su-Chi
pub(in crate::card::sets) static SU_CHI: CardRecord = CardRecord::new_with_legacy_id(
    19,
    "Su-Chi",
    CardArt::new("a64d4f93-0c04-4078-aec0-7e9de92f260f", "Christopher Rush"),
    CardSet::Antiquities,
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Construct"], 4, 4).with_abilities(&[
        abilities::dies_trigger(
            "When this creature dies, add {C}{C}{C}{C}.",
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_amount(4)),
        ),
    ]),
);

// ATQ 67 — Tablet of Epityr
pub(in crate::card::sets) static TABLET_OF_EPITYR: CardRecord = CardRecord::new_with_legacy_id(
    391,
    "Tablet of Epityr",
    CardArt::new("6d7a2718-301f-4191-b348-0c44c7c07d43", "Christopher Rush"),
    CardSet::Antiquities,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[AbilityDef::triggered(
        "Whenever an artifact you control is put into a graveyard from the battlefield, you may pay {1}. If you do, you gain 1 life.",
        TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ]), Some(ZoneKind::Battlefield), Some(ZoneKind::Graveyard)),
        EffectDef::PayOr(PayOrDef::optional(
            EffectPaymentDef::mana(
                PlayerSetDef::Related(PlayerRelation::You),
                mana_cost!("{1}"),
            ),
            &EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        )),
    )]),
);

// ATQ 68 — Tawnos's Coffin
// Audit: metadata-only — Needs card-specific counter state and counter-consuming effects for “{3}, {T}: Exile target creature and all Auras attached to it. Note the number and kind of counters that were on that creature. When this artifact leaves the battlefield or becomes…”.
pub(in crate::card::sets) static TAWNOS_S_COFFIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c27bc1de-8246-4dc8-af51-ec21def9e226"),
    "Tawnos's Coffin",
    crate::card::CardArt::new("c27bc1de-8246-4dc8-af51-ec21def9e226", "Christopher Rush"),
    crate::card::CardSet::Antiquities,
    crate::card::CardRules::unsupported(),
);

// ATQ 69 — Tawnos's Wand
pub(in crate::card::sets) static TAWNOSS_WAND: CardRecord = CardRecord::new_with_legacy_id(
    392,
    "Tawnos's Wand",
    CardArt::new("978f09dd-121a-4da5-ba16-5c03fbdce084", "Douglas Shuler"),
    CardSet::Antiquities,
    CardRules::new_artifact(mana_cost!("{4}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{2}, {T}: Target creature with power 2 or less can't be blocked this turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::PowerAtLeast(3)),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::Any,
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ATQ 70 — Tawnos's Weaponry
pub(in crate::card::sets) static TAWNOSS_WEAPONRY: CardRecord = CardRecord::new_with_legacy_id(
    1665,
    "Tawnos's Weaponry",
    CardArt::new("3035cead-a501-4204-9154-5fd648577d32", "Dan Frazier"),
    CardSet::Antiquities,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::static_ability(
            "You may choose not to untap this artifact during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayChooseNotToUntap),
            },
        ),
        AbilityDef::activated_with_targets(
            "{2}, {T}: Target creature gets +1/+1 for as long as this artifact remains \
             tapped.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::WhileSourceTapped,
            },
        ),
    ]),
);

// ATQ 70† — Tawnos's Weaponry (alternate printing)

// ATQ 71 — Tetravus
// Audit: custom — Needs declarative variable counter-to-token exchange and creator-linked token selection for the reverse exchange.
/// Both of Tetravus's assembly triggers fire at the same moment, so its
/// controller orders them and can answer both in one upkeep.
const UPKEEP: TriggerEventDef = TriggerEventDef::StepBegins {
    step: TurnStepDef::Upkeep,
    player: PlayerRelation::You,
};

pub(in crate::card::sets) static TETRAVUS: CardRecord = CardRecord::new_with_legacy_id(
    126,
    "Tetravus",
    CardArt::new("23eb19f9-2e8f-4bf0-9bf8-868e6da70e2d", "Mark Tedin"),
    CardSet::Antiquities,
    CardRules::new_artifact_creature(mana_cost!("{6}"), &["Construct"], 1, 1)
    .with_abilities(&[
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
        AbilityDef::triggered(
            "At the beginning of your upkeep, you may remove any number of +1/+1 counters from this creature. If you do, create that many 1/1 colorless Tetravite artifact creature tokens. They each have flying and \"This token can't be enchanted.\"",
            UPKEEP,
            EffectDef::None,
        )
        .with_effect_execution(EffectExecutionDef::Custom(CardBehavior::TetravusDetach))
        .with_coverage(AbilityCoverageDef::explained_complete(
            "The upkeep trigger uses the shared stack; a card-local resolver asks how many counters to trade and links each token it creates back to this permanent.",
        )),
        AbilityDef::triggered(
            "At the beginning of your upkeep, you may exile any number of tokens created with this creature. If you do, put that many +1/+1 counters on this creature.",
            UPKEEP,
            EffectDef::None,
        )
        .with_effect_execution(EffectExecutionDef::Custom(CardBehavior::TetravusAssemble))
        .with_coverage(AbilityCoverageDef::explained_complete(
            "The upkeep trigger uses the shared stack; a card-local resolver offers only the tokens this permanent created and returns one counter per token exiled.",
        )),
    ]),
);

// ATQ 72 — The Rack
// Audit: metadata-only — Needs an enter-time player choice stored on the permanent and used by its later upkeep trigger.
pub(in crate::card::sets) static THE_RACK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ec0686ba-1277-4412-a397-7a6227808311"),
    "The Rack",
    crate::card::CardArt::new("ec0686ba-1277-4412-a397-7a6227808311", "Richard Thomas"),
    crate::card::CardSet::Antiquities,
    crate::card::CardRules::unsupported(),
);

// ATQ 73 — Triskelion
pub(in crate::card::sets) static TRISKELION: CardRecord = CardRecord::new_with_legacy_id(
    43,
    "Triskelion",
    CardArt::new("a79c99e1-722a-44b6-8fa3-2be3f0c193d8", "Douglas Shuler"),
    CardSet::Antiquities,
    CardRules::new_artifact_creature(mana_cost!("{6}"), &["Construct"], 1, 1).with_abilities(&[
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
            "Remove a +1/+1 counter from this creature: It deals 1 damage to any target.",
            &[AbilityCostDef::RemoveCountersFromSource {
                kind: CounterKind::PlusOnePlusOne,
                amount: 1,
            }],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// ATQ 74 — Urza's Avenger
// Audit: metadata-only — Needs a modal activated ability for “gains your choice of banding, flying, first strike, or trample”; modes are currently a spell-only shape. Each of the four grants is implemented on its own.
pub(in crate::card::sets) static URZA_S_AVENGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("448e1811-fb16-4390-ac22-b7066a4a019c"),
    "Urza's Avenger",
    crate::card::CardArt::new("448e1811-fb16-4390-ac22-b7066a4a019c", "Amy Weber"),
    crate::card::CardSet::Antiquities,
    crate::card::CardRules::unsupported(),
);

// ATQ 75 — Urza's Chalice
pub(in crate::card::sets) static URZAS_CHALICE: CardRecord = CardRecord::new_with_legacy_id(
    393,
    "Urza's Chalice",
    CardArt::new("f3728537-86d3-42be-9046-90bba1bfafc1", "Jeff A. Menges"),
    CardSet::Antiquities,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[AbilityDef::triggered(
        "Whenever a player casts an artifact spell, you may pay {1}. If you do, you gain 1 life.",
        TriggerEventDef::SpellCast(ObjectPredicateDef::HasType(CardType::Artifact)),
        EffectDef::PayOr(PayOrDef::optional(
            EffectPaymentDef::mana(
                PlayerSetDef::Related(PlayerRelation::You),
                mana_cost!("{1}"),
            ),
            &EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        )),
    )]),
);

// ATQ 76 — Urza's Miter
// Audit: metadata-only — Needs a zone-object query and identity-preserving continuation for “Whenever an artifact you control is put into a graveyard from the battlefield, if it wasn't sacrificed, you may pay {3}. If you do, draw a card”.
pub(in crate::card::sets) static URZA_S_MITER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("438f0c61-a61d-4a9e-b21f-4e86420c7913"),
    "Urza's Miter",
    crate::card::CardArt::new(
        "438f0c61-a61d-4a9e-b21f-4e86420c7913",
        "Randy Asplund-Faith",
    ),
    crate::card::CardSet::Antiquities,
    crate::card::CardRules::unsupported(),
);

// ATQ 77 — Wall of Spears
pub(in crate::card::sets) static WALL_OF_SPEARS: CardRecord = CardRecord::new_with_legacy_id(
    394,
    "Wall of Spears",
    CardArt::new("b1dda179-c49a-4995-ba5a-db93ac43dbe7", "Sandra Everingham"),
    CardSet::Antiquities,
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Wall"], 2, 3)
        .with_abilities(&[abilities::defender(), abilities::first_strike()]),
);

// ATQ 78 — Weakstone
pub(in crate::card::sets) static WEAKSTONE: CardRecord = CardRecord::new_with_legacy_id(
    395,
    "Weakstone",
    CardArt::new("46adf48f-99d2-440e-9129-794584c1ea21", "Justin Hampton"),
    CardSet::Antiquities,
    CardRules::new_artifact(mana_cost!("{4}")).with_abilities(&[AbilityDef::static_ability(
        "Attacking creatures get -1/-0.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Attacking,
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-1),
                ValueDef::Constant(0),
            ),
        },
    )]),
);

// ATQ 79 — Yotian Soldier
pub(in crate::card::sets) static YOTIAN_SOLDIER: CardRecord = CardRecord::new_with_legacy_id(
    396,
    "Yotian Soldier",
    CardArt::new("27cf53e3-76f6-4831-800e-1259394d779d", "Christopher Rush"),
    CardSet::Antiquities,
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Soldier"], 1, 4)
        .with_abilities(&[abilities::vigilance()]),
);

// ATQ 80a — Mishra's Factory
/// Animating keeps the land: the creature and artifact types are added on
/// top of what is printed.
static MISHRAS_FACTORY_ANIMATION: [AppliedEffectDef; 3] = [
    AppliedEffectDef::add_card_types(
        CardTypeSet::single(CardType::Creature).with(CardType::Artifact),
    ),
    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Assembly-Worker"])),
    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
];

/// The pump reaches any Assembly-Worker, including a Factory that has already
/// animated itself and a second Factory across the table.
static MISHRAS_FACTORY_PUMP_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Subtype("Assembly-Worker"),
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
)];

pub(in crate::card::sets) static MISHRA_S_FACTORY: CardRecord = CardRecord::new_with_legacy_id(
    31,
    "Mishra's Factory",
    CardArt::new("a696c5b6-f216-454d-8029-74e84bbd1428", "Kaja Foglio & Phil Foglio"),
    CardSet::Antiquities,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{1}: This land becomes a 2/2 Assembly-Worker artifact creature until end of turn. It's still a land.",
            &[AbilityCostDef::Mana(mana_cost!("{1}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&MISHRAS_FACTORY_ANIMATION),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated_with_targets(
            "{T}: Target Assembly-Worker creature gets +1/+1 until end of turn.",
            &[AbilityCostDef::TapSource],
            &MISHRAS_FACTORY_PUMP_TARGET,
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ATQ 80b — Mishra's Factory (alternate printing)

// ATQ 80c — Mishra's Factory (alternate printing)

// ATQ 80d — Mishra's Factory (alternate printing)

// ATQ 81 — Mishra's Workshop
static MISHRA_S_WORKSHOP_RESTRICTIONS: [ManaRestrictionDef; 1] = [ManaRestrictionDef::CastSpell(
    ObjectPredicateDef::HasType(CardType::Artifact),
)];

pub(in crate::card::sets) static MISHRA_S_WORKSHOP: CardRecord = CardRecord::new_with_legacy_id(
    83,
    "Mishra's Workshop",
    CardArt::new("135de5c7-6ac9-4b68-8f1a-97f120a4b125", "Kaja Foglio"),
    CardSet::Antiquities,
    CardRules::new_land(&[]).with_abilities(&[AbilityDef::activated_mana(
        "{T}: Add {C}{C}{C}. Spend this mana only to cast artifact spells.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(
            AddManaEffectDef::one(ManaColor::Colorless)
                .with_amount(3)
                .with_restrictions(&MISHRA_S_WORKSHOP_RESTRICTIONS),
        ),
    )]),
);

// ATQ 82a — Strip Mine
pub(in crate::card::sets) static STRIP_MINE: CardRecord = CardRecord::new_with_legacy_id(
    39,
    "Strip Mine",
    CardArt::new("e7880157-7f27-4f1b-9cdc-ab36a6252376", "Daniel Gelon"),
    CardSet::Antiquities,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice this land: Destroy target land.",
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
        ),
    ]),
);

// ATQ 82b — Strip Mine (alternate printing)

// ATQ 82c — Strip Mine (alternate printing)

// ATQ 82d — Strip Mine (alternate printing)

// ATQ 83a — Urza's Mine
pub(in crate::card::sets) static URZA_S_MINE: CardRecord = CardRecord::new_with_legacy_id(
    1830,
    "Urza's Mine",
    CardArt::new("ddf85792-470b-4b42-99ac-9cb43a575523", "Anson Maddocks"),
    CardSet::Antiquities,
    CardRules::new_land(&[]).with_ability(AbilityDef::activated_mana(
        "{T}: Add {C}. If you control an Urza's Power Plant and an Urza's Tower, add {C}{C} \
         instead.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(
            AddManaEffectDef::one(ManaColor::Colorless).with_amount_override(
                ConditionDef::All(&[
                    controls_named("Urza's Power Plant"),
                    controls_named("Urza's Tower"),
                ]),
                2,
            ),
        ),
    )),
);

// ATQ 83b — Urza's Mine (alternate printing)

// ATQ 83c — Urza's Mine (alternate printing)

// ATQ 83d — Urza's Mine (alternate printing)

// ATQ 84a — Urza's Power Plant
pub(in crate::card::sets) static URZA_S_POWER_PLANT: CardRecord = CardRecord::new_with_legacy_id(
    1831,
    "Urza's Power Plant",
    CardArt::new("94896e0b-859c-47e4-bf27-35ed37b841e0", "Mark Tedin"),
    CardSet::Antiquities,
    CardRules::new_land(&[]).with_ability(AbilityDef::activated_mana(
        "{T}: Add {C}. If you control an Urza's Mine and an Urza's Tower, add {C}{C} instead.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(
            AddManaEffectDef::one(ManaColor::Colorless).with_amount_override(
                ConditionDef::All(&[
                    controls_named("Urza's Mine"),
                    controls_named("Urza's Tower"),
                ]),
                2,
            ),
        ),
    )),
);

// ATQ 84b — Urza's Power Plant (alternate printing)

// ATQ 84c — Urza's Power Plant (alternate printing)

// ATQ 84d — Urza's Power Plant (alternate printing)

// ATQ 85a — Urza's Tower
pub(in crate::card::sets) static URZA_S_TOWER: CardRecord = CardRecord::new_with_legacy_id(
    1832,
    "Urza's Tower",
    CardArt::new("8ed85655-fc59-4a57-bcf9-75e1899dff78", "Mark Poole"),
    CardSet::Antiquities,
    CardRules::new_land(&[]).with_ability(AbilityDef::activated_mana(
        "{T}: Add {C}. If you control an Urza's Mine and an Urza's Power Plant, add {C}{C}{C} \
         instead.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(
            AddManaEffectDef::one(ManaColor::Colorless).with_amount_override(
                ConditionDef::All(&[
                    controls_named("Urza's Mine"),
                    controls_named("Urza's Power Plant"),
                ]),
                3,
            ),
        ),
    )),
);

// ATQ 85b — Urza's Tower (alternate printing)

// ATQ 85c — Urza's Tower (alternate printing)

// ATQ 85d — Urza's Tower (alternate printing)

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ARGIVIAN_ARCHAEOLOGIST,
    &ARGIVIAN_BLACKSMITH,
    &ARTIFACT_WARD,
    &CIRCLE_OF_PROTECTION_ARTIFACTS,
    &DAMPING_FIELD,
    &MARTYRS_OF_KORLIS,
    &REVERSE_POLARITY,
    &DRAFNA_S_RESTORATION,
    &ENERGY_FLUX,
    &HURKYLS_RECALL,
    &POWER_ARTIFACT,
    &RECONSTRUCTION,
    &SAGE_OF_LAT_NAM,
    &TRANSMUTE_ARTIFACT,
    &ARTIFACT_POSSESSION,
    &GATE_TO_PHYREXIA,
    &HAUNTING_WIND,
    &PHYREXIAN_GREMLINS,
    &PRIEST_OF_YAWGMOTH,
    &XENIC_POLTERGEIST,
    &YAWGMOTH_DEMON,
    &ARTIFACT_BLAST,
    &ATOG,
    &DETONATE,
    &DWARVEN_WEAPONSMITH,
    &GOBLIN_ARTISANS,
    &ORCISH_MECHANICS,
    &SHATTERSTORM,
    &ARGOTHIAN_PIXIES,
    &ARGOTHIAN_TREEFOLK,
    &CITANUL_DRUID,
    &CRUMBLE,
    &GAEAS_AVENGER,
    &POWERLEECH,
    &TITANIA_S_SONG,
    &AMULET_OF_KROOG,
    &ARMAGEDDON_CLOCK,
    &ASHNOD_S_ALTAR,
    &ASHNODS_BATTLE_GEAR,
    &ASHNODS_TRANSMOGRANT,
    &BATTERING_RAM,
    &BRONZE_TABLET,
    &CANDELABRA_OF_TAWNOS,
    &CLAY_STATUE,
    &CLOCKWORK_AVIAN,
    &COLOSSUS_OF_SARDIA,
    &CORAL_HELM,
    &CURSED_RACK,
    &DRAGON_ENGINE,
    &FELDONS_CANE,
    &GOLGOTHIAN_SYLEX,
    &GRAPESHOT_CATAPULT,
    &IVORY_TOWER,
    &JALUM_TOME,
    &MIGHTSTONE,
    &MILLSTONE,
    &MISHRA_S_WAR_MACHINE,
    &OBELISK_OF_UNDOING,
    &ONULET,
    &ORNITHOPTER,
    &PRIMAL_CLAY,
    &RAKALITE,
    &ROCKET_LAUNCHER,
    &SHAPESHIFTER,
    &STAFF_OF_ZEGON,
    &SU_CHI,
    &TABLET_OF_EPITYR,
    &TAWNOS_S_COFFIN,
    &TAWNOSS_WAND,
    &TAWNOSS_WEAPONRY,
    &TETRAVUS,
    &THE_RACK,
    &TRISKELION,
    &URZA_S_AVENGER,
    &URZAS_CHALICE,
    &URZA_S_MITER,
    &WALL_OF_SPEARS,
    &WEAKSTONE,
    &YOTIAN_SOLDIER,
    &MISHRA_S_FACTORY,
    &MISHRA_S_WORKSHOP,
    &STRIP_MINE,
    &URZA_S_MINE,
    &URZA_S_POWER_PLANT,
    &URZA_S_TOWER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::alternate(&ATOG, 1),               // ATQ 23†
    PrintingRecord::alternate(&TAWNOSS_WEAPONRY, 1),   // ATQ 70†
    PrintingRecord::alternate(&MISHRA_S_FACTORY, 1),   // ATQ 80b
    PrintingRecord::alternate(&MISHRA_S_FACTORY, 2),   // ATQ 80c
    PrintingRecord::alternate(&MISHRA_S_FACTORY, 3),   // ATQ 80d
    PrintingRecord::alternate(&STRIP_MINE, 1),         // ATQ 82b
    PrintingRecord::alternate(&STRIP_MINE, 2),         // ATQ 82c
    PrintingRecord::alternate(&STRIP_MINE, 3),         // ATQ 82d
    PrintingRecord::alternate(&URZA_S_MINE, 1),        // ATQ 83b
    PrintingRecord::alternate(&URZA_S_MINE, 2),        // ATQ 83c
    PrintingRecord::alternate(&URZA_S_MINE, 3),        // ATQ 83d
    PrintingRecord::alternate(&URZA_S_POWER_PLANT, 1), // ATQ 84b
    PrintingRecord::alternate(&URZA_S_POWER_PLANT, 2), // ATQ 84c
    PrintingRecord::alternate(&URZA_S_POWER_PLANT, 3), // ATQ 84d
    PrintingRecord::alternate(&URZA_S_TOWER, 1),       // ATQ 85b
    PrintingRecord::alternate(&URZA_S_TOWER, 2),       // ATQ 85c
    PrintingRecord::alternate(&URZA_S_TOWER, 3),       // ATQ 85d
];

#[cfg(test)]
mod tests {
    use super::{ENERGY_FLUX, ENERGY_FLUX_GRANTED_ABILITY};
    use crate::card::{
        AbilityOperationDef, AppliedEffectDef, CardEffectStatus, CharacteristicOperationDef,
        DeclarativeAbilityDef, EffectDef, ImplementationStatus,
    };

    #[test]
    fn energy_flux_grants_every_artifact_a_real_upkeep_tax() {
        let definition = ENERGY_FLUX.definition();
        let clauses = definition.rules.ability_clauses();
        assert_eq!(clauses.len(), 1);
        assert_eq!(
            clauses[0].text,
            "All artifacts have \"At the beginning of your upkeep, sacrifice this artifact unless you pay {2}.\""
        );
        assert!(matches!(
            clauses[0].definition,
            DeclarativeAbilityDef::Static(_)
        ));
        assert_eq!(clauses[0].coverage.status, ImplementationStatus::Complete);
        assert!(matches!(
            clauses[0].declarative_effect(),
            Some(EffectDef::StaticApply {
                effect: AppliedEffectDef::Characteristic(
                    CharacteristicOperationDef::Abilities(AbilityOperationDef::Add(granted))
                ),
                ..
            }) if granted == &ENERGY_FLUX_GRANTED_ABILITY
        ));
        assert!(matches!(
            ENERGY_FLUX_GRANTED_ABILITY.definition,
            DeclarativeAbilityDef::Triggered(_)
        ));
        assert_eq!(
            ENERGY_FLUX_GRANTED_ABILITY.coverage.status,
            ImplementationStatus::Complete
        );
        assert_eq!(
            definition.implementation_status(),
            ImplementationStatus::Complete
        );
        assert_eq!(
            definition.play_options[0].effect_status,
            CardEffectStatus::Implemented
        );
    }
}
