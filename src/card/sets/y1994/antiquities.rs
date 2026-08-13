use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    AddManaEffectDef, AnimationDef, AppliedEffectDef, BattlefieldEntryModificationDef, CardArt,
    CardBehavior, CardRules, CardSet, CardType, CardTypeSet, CounterKind, DiscardSelectionDef,
    EffectDef, EffectDurationDef, EffectExecutionDef, EffectRecipientDef, KeywordAbility,
    ManaColor, ManaRestrictionDef, ObjectPredicateDef, PaymentDef, PlayerRelation,
    ReplacementEffectDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement,
    abilities, cards,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

static ENERGY_FLUX_GRANTED_ABILITY: AbilityDef = AbilityDef::triggered(
    "At the beginning of your upkeep, sacrifice this artifact unless you pay {2}.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    EffectDef::UnlessPaid {
        cost: mana_cost!("{2}"),
        otherwise: &EffectDef::Sacrifice {
            object: EffectRecipientDef::Source,
        },
    },
);

// ATQ 1 — Argivian Archaeologist
pub(in crate::card::sets) static ARGIVIAN_ARCHAEOLOGIST: CardRecord = CardRecord::new(
    cards::ARGIVIAN_ARCHAEOLOGIST,
    "Argivian Archaeologist",
    CardArt::new("ce83a3cb-467d-44f6-a051-4855c8cf52a6", "Amy Weber"),
    CardSet::Antiquities,
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Human", "Artificer"], 1, 1).with_abilities(
        &[AbilityDef::activated_with_targets(
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
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
                controller: None,
            },
        )],
    ),
);

// ATQ 2 — Argivian Blacksmith
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “{T}: Prevent the next 2 damage that would be dealt to target artifact creature this turn”.

// ATQ 3 — Artifact Ward
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “Prevent all damage that would be dealt to enchanted creature by artifact sources”.

// ATQ 4 — Circle of Protection: Artifacts
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “{2}: The next time an artifact source of your choice would deal damage to you this turn, prevent that damage”.

// ATQ 5 — Damping Field
// Audit: blocked — Needs a persistent tap/untap restriction or event relation for “Players can't untap more than one artifact during their untap steps”.

// ATQ 6 — Martyrs of Korlis
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “As long as this creature is untapped, all damage that would be dealt to you by artifacts is dealt to this creature instead”.

// ATQ 7 — Reverse Polarity
// Audit: blocked — Needs damage-history/source tracking or card-specific damage processing for “You gain X life, where X is twice the damage dealt to you so far this turn by artifacts”.

// ATQ 8 — Drafna's Restoration
// Audit: blocked — Needs ordered-library inspection, selection, and visibility handling for “Put any number of target artifact cards from target player's graveyard on top of their library in any order”.

// ATQ 9 — Energy Flux
pub(in crate::card::sets) static ENERGY_FLUX: CardRecord = CardRecord::new(
    cards::ENERGY_FLUX,
    "Energy Flux",
    CardArt::new("bd1f624b-e8f2-462f-838a-7cb9e8fda988", "Kaja Foglio"),
    CardSet::Antiquities,
    CardRules::new_enchantment(mana_cost!("{2}{U}"))
    .with_abilities(&[AbilityDef::static_ability(
        "All artifacts have \"At the beginning of your upkeep, sacrifice this artifact unless you pay {2}.\"",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasType(CardType::Artifact),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
            effect: AppliedEffectDef::GrantAbility(&ENERGY_FLUX_GRANTED_ABILITY),
            duration: EffectDurationDef::WhileSourceRemainsInZone,
        },
    )]),
);

// ATQ 10 — Hurkyl's Recall
pub(in crate::card::sets) static HURKYLS_RECALL: CardRecord = CardRecord::new(
    cards::HURKYLS_RECALL,
    "Hurkyl's Recall",
    CardArt::new("f32373dd-06d8-45d1-8777-3b1411bcb30a", "NéNé Thomas"),
    CardSet::Antiquities,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Return all artifacts target player owns to their hand.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::ObjectsOwnedByTarget {
                object: ObjectPredicateDef::HasType(CardType::Artifact),
                slot: TargetIndex::PRIMARY,
            },
            zone: ZoneKind::Hand,
            controller: None,
            placement: ZonePlacement::Top,
        },
    )]),
);

// ATQ 11 — Power Artifact
// Audit: blocked — Needs cost/mana provenance or dynamic payment support for “Enchanted artifact's activated abilities cost {2} less to activate. This effect can't reduce the mana in that cost to less than one mana”.

// ATQ 12 — Reconstruction
pub(in crate::card::sets) static RECONSTRUCTION: CardRecord = CardRecord::new(
    cards::RECONSTRUCTION,
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
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            controller: None,
        },
    )]),
);

// ATQ 13 — Sage of Lat-Nam
pub(in crate::card::sets) static SAGE_OF_LAT_NAM: CardRecord = CardRecord::new(
    cards::SAGE_OF_LAT_NAM,
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
// Audit: blocked — Needs the complete qualified library-search and post-search continuation for “Sacrifice an artifact. If you do, search your library for an artifact card. If that card's mana value is less than or equal to the sacrificed artifact's mana value, put it onto the…”.

// ATQ 15 — Artifact Possession
// Audit: blocked — Needs a trigger relation for the attached permanent becoming tapped and its controller/characteristics for “Whenever enchanted artifact becomes tapped or a player activates an ability of enchanted artifact without {T} in its activation cost, this Aura deals 2 damage to that artifact's controller”.

// ATQ 16 — Gate to Phyrexia
// Audit: blocked — Needs linked sacrifice/destruction accounting for “Sacrifice a creature: Destroy target artifact. Activate only during your upkeep and only once each turn”.

// ATQ 17 — Haunting Wind
// Audit: blocked — Needs artifact tap and non-tap activated-ability events, including inspection of the triggering activation's costs.

// ATQ 18 — Phyrexian Gremlins
// Audit: blocked — Needs a persistent tap/untap restriction or event relation for “{T}: Tap target artifact. It doesn't untap during its controller's untap step for as long as this creature remains tapped”.

// ATQ 19 — Priest of Yawgmoth
// Audit: blocked — Needs cost/mana provenance or dynamic payment support for “{T}, Sacrifice an artifact: Add an amount of {B} equal to the sacrificed artifact's mana value”.

// ATQ 20 — Xenic Poltergeist
// Audit: blocked — Needs temporary artifact animation with dynamic mana-value base power/toughness lasting through the next upkeep.

// ATQ 21 — Yawgmoth Demon
// Audit: blocked — Needs an optional artifact-sacrifice choice whose declined or impossible branch taps the source and deals damage.

// ATQ 22 — Artifact Blast
pub(in crate::card::sets) static ARTIFACT_BLAST: CardRecord = CardRecord::new(
    cards::ARTIFACT_BLAST,
    "Artifact Blast",
    CardArt::new("1506d99d-7b2e-4101-84a5-c950dadb263a", "Mark Poole"),
    CardSet::Antiquities,
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[AbilityDef::counter_target(
        "Counter target artifact spell.",
        &AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::HasType(CardType::Artifact)),
    )]),
);

// ATQ 23 — Atog
pub(in crate::card::sets) static ATOG: CardRecord = CardRecord::new(
    cards::ATOG,
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
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(2),
                    toughness: ValueDef::Constant(2),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

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
    },
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
        amount: ValueDef::ChosenX,
    },
];

// ATQ 24 — Detonate
pub(in crate::card::sets) static DETONATE: CardRecord = CardRecord::new(
    cards::DETONATE,
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
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “{T}, Sacrifice an artifact: Put a +1/+1 counter on target creature. Activate only during your upkeep”.

// ATQ 26 — Goblin Artisans
// Audit: blocked — Needs a deterministic recorded coin-flip choice and both result branches for “{T}: Flip a coin. If you win the flip, draw a card. If you lose the flip, counter target artifact spell you control that isn't the target of an ability from another creature named Goblin…”.

// ATQ 27 — Orcish Mechanics
pub(in crate::card::sets) static ORCISH_MECHANICS: CardRecord = CardRecord::new(
    cards::ORCISH_MECHANICS,
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
pub(in crate::card::sets) static SHATTERSTORM: CardRecord = CardRecord::new(
    cards::SHATTERSTORM,
    "Shatterstorm",
    CardArt::new("0987461a-45c0-4956-8627-cd27a7e038d0", "Dan Frazier"),
    CardSet::Antiquities,
    CardRules::new_sorcery(mana_cost!("{2}{R}{R}")).with_abilities(&[AbilityDef::spell(
        "Destroy all artifacts. They can't be regenerated.",
        EffectDef::Destroy {
            object: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasType(CardType::Artifact),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
            can_regenerate: false,
        },
    )]),
);

/// Both halves of the Pixies read the same set, so the blocking restriction
/// and the prevention cannot drift apart.
static ARTIFACT_CREATURE: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Artifact),
    ObjectPredicateDef::HasType(CardType::Creature),
]);

// ATQ 29 — Argothian Pixies
pub(in crate::card::sets) static ARGOTHIAN_PIXIES: CardRecord = CardRecord::new(
    cards::ARGOTHIAN_PIXIES,
    "Argothian Pixies",
    CardArt::new("5712e87a-2381-4f5b-a853-6973841f9bf1", "Amy Weber"),
    CardSet::Antiquities,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Faerie"], 2, 1).with_abilities(&[
        AbilityDef::static_ability(
            "This creature can't be blocked by artifact creatures.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::CannotBeBlockedBy(ARTIFACT_CREATURE),
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
        AbilityDef::static_ability(
            "Prevent all damage that would be dealt to this creature by artifact creatures.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::PreventDamageFrom(ARTIFACT_CREATURE),
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
    ]),
);

// ATQ 30 — Argothian Treefolk
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “Prevent all damage that would be dealt to this creature by artifact sources”.

// ATQ 31 — Citanul Druid
pub(in crate::card::sets) static CITANUL_DRUID: CardRecord = CardRecord::new(
    cards::CITANUL_DRUID,
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
pub(in crate::card::sets) static CRUMBLE: CardRecord = CardRecord::new(
    cards::CRUMBLE,
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
// Audit: blocked — Needs a characteristic-layer effect or dynamic value for “Gaea's Avenger's power and toughness are each equal to 1 plus the number of artifacts your opponents control”.

// ATQ 34 — Powerleech
// Audit: blocked — Needs opponent-artifact tap and non-tap activated-ability events, including inspection of activation costs.

// ATQ 35 — Titania's Song
// Audit: blocked — Needs static animation of every noncreature artifact with dynamic mana-value power/toughness and ability removal.

// ATQ 36 — Amulet of Kroog
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “{2}, {T}: Prevent the next 1 damage that would be dealt to any target this turn”.

// ATQ 37 — Armageddon Clock
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “At the beginning of your draw step, this artifact deals damage equal to the number of doom counters on it to each player”.

// ATQ 38 — Ashnod's Altar
// Audit: blocked — Needs mana-ability activation to select and sacrifice a different creature; the mana runtime can currently sacrifice only the source.

// ATQ 39 — Ashnod's Battle Gear
// Audit: blocked — Needs a persistent tap/untap restriction or event relation for “{2}, {T}: Target creature you control gets +2/-2 for as long as this artifact remains tapped”.

// ATQ 40 — Ashnod's Transmogrant
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “{T}, Sacrifice this artifact: Put a +1/+1 counter on target nonartifact creature. That creature becomes an artifact in addition to its other types”.

// ATQ 41 — Battering Ram
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Whenever this creature becomes blocked by a Wall, destroy that Wall at end of combat”.

// ATQ 43 — Candelabra of Tawnos
// Audit: blocked — Needs a persistent tap/untap restriction or event relation for “{X}, {T}: Untap X target lands”.

// ATQ 44 — Clay Statue
// Audit: blocked — Needs regeneration shields and their destroy-event replacement procedure for “{2}: Regenerate this creature”.

// ATQ 45 — Clockwork Avian
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “{X}, {T}: Put up to X +1/+0 counters on this creature. This ability can't cause the total number of +1/+0 counters on this creature to be greater than four. Activate only during your upkeep”.

// ATQ 46 — Colossus of Sardia
// Audit: blocked — Needs a persistent tap/untap restriction or event relation for “This creature doesn't untap during your untap step”.

// ATQ 47 — Coral Helm
// Audit: blocked — Needs the clause's conditional recipient set or dynamic modifier value for “{3}, Discard a card at random: Target creature gets +2/+2 until end of turn”.

// ATQ 48 — Cursed Rack
// Audit: blocked — Needs a hidden-zone decision and continuation for “The chosen player's maximum hand size is four”.

// ATQ 49 — Dragon Engine
pub(in crate::card::sets) static DRAGON_ENGINE: CardRecord = CardRecord::new(
    cards::DRAGON_ENGINE,
    "Dragon Engine",
    CardArt::new("07793a71-1106-4303-b620-e403bd378020", "Anson Maddocks"),
    CardSet::Antiquities,
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Construct"], 1, 3).with_abilities(&[
        AbilityDef::activated(
            "{2}: This creature gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{2}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(0),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ATQ 50 — Feldon's Cane
// Audit: blocked — Needs a zone-object query and identity-preserving continuation for “{T}, Exile this artifact: Shuffle your graveyard into your library”.

// ATQ 51 — Golgothian Sylex
// Audit: partial — Its expansion predicate follows physical identity rather than the permanent's current copied name.
pub(in crate::card::sets) static GOLGOTHIAN_SYLEX: CardRecord = CardRecord::new(
    cards::GOLGOTHIAN_SYLEX,
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
            object: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::DebutSet(CardSet::Antiquities),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
        },
    )
    .with_coverage(AbilityCoverageDef::partial(
        "The expansion predicate follows physical identity rather than the permanent's current copied name.",
    ))]),
);

// ATQ 52 — Grapeshot Catapult
// Audit: partial — Its flying predicate omits abilities granted or removed by static continuous effects.
pub(in crate::card::sets) static GRAPESHOT_CATAPULT: CardRecord = CardRecord::new(
    cards::GRAPESHOT_CATAPULT,
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
        )
        .with_coverage(AbilityCoverageDef::partial(
            "The flying predicate omits abilities granted or removed by static continuous effects.",
        )),
    ]),
);

// ATQ 53 — Ivory Tower
pub(in crate::card::sets) static IVORY_TOWER: CardRecord = CardRecord::new(
    cards::IVORY_TOWER,
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
pub(in crate::card::sets) static JALUM_TOME: CardRecord = CardRecord::new(
    cards::JALUM_TOME,
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
                },
            ]),
        )]),
);

// ATQ 55 — Mightstone
pub(in crate::card::sets) static MIGHTSTONE: CardRecord = CardRecord::new(
    cards::MIGHTSTONE,
    "Mightstone",
    CardArt::new("b28ba599-5299-4831-a118-1712ada10ef6", "Pete Venters"),
    CardSet::Antiquities,
    CardRules::new_artifact(mana_cost!("{4}")).with_abilities(&[AbilityDef::static_ability(
        "Attacking creatures get +1/+0.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Attacking,
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
            effect: AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Constant(1),
                toughness: ValueDef::Constant(0),
            },
            duration: EffectDurationDef::WhileSourceRemainsInZone,
        },
    )]),
);

// ATQ 56 — Millstone
pub(in crate::card::sets) static MILLSTONE: CardRecord = CardRecord::new(
    cards::MILLSTONE,
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
            },
        ),
    ]),
);

// ATQ 57 — Mishra's War Machine
// Audit: blocked — Needs full banding group declaration, blocking, and combat-damage assignment semantics.

// ATQ 58 — Obelisk of Undoing
pub(in crate::card::sets) static OBELISK_OF_UNDOING: CardRecord = CardRecord::new(
    cards::OBELISK_OF_UNDOING,
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
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
                controller: None,
            },
        ),
    ]),
);

// ATQ 59 — Onulet
pub(in crate::card::sets) static ONULET: CardRecord = CardRecord::new(
    cards::ONULET,
    "Onulet",
    CardArt::new("d77fe8e2-8438-473e-ace5-01baddd2c4ed", "Anson Maddocks"),
    CardSet::Antiquities,
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Construct"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "When this creature dies, you gain 2 life.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: Some(ZoneKind::Battlefield),
                to: Some(ZoneKind::Graveyard),
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// ATQ 60 — Ornithopter
pub(in crate::card::sets) static ORNITHOPTER: CardRecord = CardRecord::new(
    cards::ORNITHOPTER,
    "Ornithopter",
    CardArt::new("59cc9bdb-7cf2-4795-bac7-ffff605c9eb0", "Amy Weber"),
    CardSet::Antiquities,
    CardRules::new_artifact_creature(mana_cost!("{0}"), &["Thopter"], 0, 2)
        .with_abilities(&[abilities::flying()]),
);

// ATQ 61 — Primal Clay
// Audit: blocked — Needs a characteristic-layer effect or dynamic value for “As this creature enters, it becomes your choice of a 3/3 artifact creature, a 2/2 artifact creature with flying, or a 1/6 Wall artifact creature with defender in addition to its other types”.

// ATQ 62 — Rakalite
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “{2}: Prevent the next 1 damage that would be dealt to any target this turn. Return this artifact to its owner's hand at the beginning of the next end step”.

// ATQ 63 — Rocket Launcher
// Audit: blocked — Needs continuous-control activation timing and a delayed self-destruction trigger created by activation.

// ATQ 64 — Shapeshifter
// Audit: blocked — Needs a characteristic-layer effect or dynamic value for “Shapeshifter's power is equal to the last chosen number and its toughness is equal to 7 minus that number”.

// ATQ 65 — Staff of Zegon
pub(in crate::card::sets) static STAFF_OF_ZEGON: CardRecord = CardRecord::new(
    cards::STAFF_OF_ZEGON,
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
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(-2),
                    toughness: ValueDef::Constant(0),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ATQ 66 — Su-Chi
pub(in crate::card::sets) static SU_CHI: CardRecord = CardRecord::new(
    cards::SU_CHI,
    "Su-Chi",
    CardArt::new("a64d4f93-0c04-4078-aec0-7e9de92f260f", "Christopher Rush"),
    CardSet::Antiquities,
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Construct"], 4, 4).with_abilities(&[
        AbilityDef::triggered(
            "When this creature dies, add {C}{C}{C}{C}.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: Some(ZoneKind::Battlefield),
                to: Some(ZoneKind::Graveyard),
            },
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_amount(4)),
        ),
    ]),
);

// ATQ 67 — Tablet of Epityr
pub(in crate::card::sets) static TABLET_OF_EPITYR: CardRecord = CardRecord::new(
    cards::TABLET_OF_EPITYR,
    "Tablet of Epityr",
    CardArt::new("6d7a2718-301f-4191-b348-0c44c7c07d43", "Christopher Rush"),
    CardSet::Antiquities,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[AbilityDef::triggered(
        "Whenever an artifact you control is put into a graveyard from the battlefield, you may pay {1}. If you do, you gain 1 life.",
        TriggerEventDef::ZoneChanged {
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ]),
            from: Some(ZoneKind::Battlefield),
            to: Some(ZoneKind::Graveyard),
        },
        EffectDef::OptionalPayment {
            payment: PaymentDef::new(
                PlayerRelation::You,
                &[AbilityCostDef::Mana(mana_cost!("{1}"))],
            ),
            if_paid: &EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        },
    )]),
);

// ATQ 68 — Tawnos's Coffin
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “{3}, {T}: Exile target creature and all Auras attached to it. Note the number and kind of counters that were on that creature. When this artifact leaves the battlefield or becomes…”.

// ATQ 69 — Tawnos's Wand
// Audit: partial — Its power predicate omits modifiers from static continuous effects.
pub(in crate::card::sets) static TAWNOSS_WAND: CardRecord = CardRecord::new(
    cards::TAWNOSS_WAND,
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
            EffectDef::MakeUnblockableThisTurn {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "The power predicate omits modifiers from static continuous effects.",
        )),
    ]),
);

/// Both of Tetravus's assembly triggers fire at the same moment, so its
/// controller orders them and can answer both in one upkeep.
const UPKEEP: TriggerEventDef = TriggerEventDef::StepBegins {
    step: TurnStepDef::Upkeep,
    player: PlayerRelation::You,
};

// ATQ 70 — Tawnos's Weaponry
// Audit: blocked — Needs the clause's conditional recipient set or dynamic modifier value for “{2}, {T}: Target creature gets +1/+1 for as long as this artifact remains tapped”.

// ATQ 71 — Tetravus
pub(in crate::card::sets) static TETRAVUS: CardRecord = CardRecord::new(
    cards::TETRAVUS,
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
// Audit: blocked — Needs an enter-time player choice stored on the permanent and used by its later upkeep trigger.

// ATQ 73 — Triskelion
pub(in crate::card::sets) static TRISKELION: CardRecord = CardRecord::new(
    cards::TRISKELION,
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
// Audit: blocked — Needs full banding group declaration, blocking, and combat-damage assignment semantics.

// ATQ 75 — Urza's Chalice
pub(in crate::card::sets) static URZAS_CHALICE: CardRecord = CardRecord::new(
    cards::URZAS_CHALICE,
    "Urza's Chalice",
    CardArt::new("f3728537-86d3-42be-9046-90bba1bfafc1", "Jeff A. Menges"),
    CardSet::Antiquities,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[AbilityDef::triggered(
        "Whenever a player casts an artifact spell, you may pay {1}. If you do, you gain 1 life.",
        TriggerEventDef::SpellCast(ObjectPredicateDef::HasType(CardType::Artifact)),
        EffectDef::OptionalPayment {
            payment: PaymentDef::new(
                PlayerRelation::You,
                &[AbilityCostDef::Mana(mana_cost!("{1}"))],
            ),
            if_paid: &EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        },
    )]),
);

// ATQ 76 — Urza's Miter
// Audit: blocked — Needs a zone-object query and identity-preserving continuation for “Whenever an artifact you control is put into a graveyard from the battlefield, if it wasn't sacrificed, you may pay {3}. If you do, draw a card”.

// ATQ 77 — Wall of Spears
pub(in crate::card::sets) static WALL_OF_SPEARS: CardRecord = CardRecord::new(
    cards::WALL_OF_SPEARS,
    "Wall of Spears",
    CardArt::new("b1dda179-c49a-4995-ba5a-db93ac43dbe7", "Sandra Everingham"),
    CardSet::Antiquities,
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Wall"], 2, 3)
        .with_abilities(&[abilities::defender(), abilities::first_strike()]),
);

// ATQ 78 — Weakstone
pub(in crate::card::sets) static WEAKSTONE: CardRecord = CardRecord::new(
    cards::WEAKSTONE,
    "Weakstone",
    CardArt::new("46adf48f-99d2-440e-9129-794584c1ea21", "Justin Hampton"),
    CardSet::Antiquities,
    CardRules::new_artifact(mana_cost!("{4}")).with_abilities(&[AbilityDef::static_ability(
        "Attacking creatures get -1/-0.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Attacking,
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
            effect: AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Constant(-1),
                toughness: ValueDef::Constant(0),
            },
            duration: EffectDurationDef::WhileSourceRemainsInZone,
        },
    )]),
);

// ATQ 79 — Yotian Soldier
pub(in crate::card::sets) static YOTIAN_SOLDIER: CardRecord = CardRecord::new(
    cards::YOTIAN_SOLDIER,
    "Yotian Soldier",
    CardArt::new("27cf53e3-76f6-4831-800e-1259394d779d", "Christopher Rush"),
    CardSet::Antiquities,
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Soldier"], 1, 4)
        .with_abilities(&[abilities::vigilance()]),
);

/// Animating keeps the land: the creature and artifact types are added on
/// top of what is printed.
static MISHRAS_FACTORY_ANIMATION: AnimationDef = AnimationDef::new(2, 2)
    .with_types(CardTypeSet::single(CardType::Creature).with(CardType::Artifact))
    .with_subtypes(&["Assembly-Worker"]);

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

// ATQ 80a — Mishra's Factory
pub(in crate::card::sets) static MISHRA_S_FACTORY: CardRecord = CardRecord::new(
    cards::MISHRA_S_FACTORY,
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
                effect: AppliedEffectDef::Animate(&MISHRAS_FACTORY_ANIMATION),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated_with_targets(
            "{T}: Target Assembly-Worker creature gets +1/+1 until end of turn.",
            &[AbilityCostDef::TapSource],
            &MISHRAS_FACTORY_PUMP_TARGET,
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(1),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

static MISHRA_S_WORKSHOP_RESTRICTIONS: [ManaRestrictionDef; 1] = [ManaRestrictionDef::CastSpell(
    ObjectPredicateDef::HasType(CardType::Artifact),
)];

// ATQ 81 — Mishra's Workshop
pub(in crate::card::sets) static MISHRA_S_WORKSHOP: CardRecord = CardRecord::new(
    cards::MISHRA_S_WORKSHOP,
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
pub(in crate::card::sets) static STRIP_MINE: CardRecord = CardRecord::new(
    cards::STRIP_MINE,
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
            },
        ),
    ]),
);

// ATQ 83a — Urza's Mine
// Audit: blocked — Needs the mana-ability runtime to pay this ability's mana activation cost for “{T}: Add {C}. If you control an Urza's Power-Plant and an Urza's Tower, add {C}{C} instead”.

// ATQ 84a — Urza's Power Plant
// Audit: blocked — Needs the mana-ability runtime to pay this ability's mana activation cost for “{T}: Add {C}. If you control an Urza's Mine and an Urza's Tower, add {C}{C} instead”.

// ATQ 85a — Urza's Tower
// Audit: blocked — Needs the mana-ability runtime to pay this ability's mana activation cost for “{T}: Add {C}. If you control an Urza's Mine and an Urza's Power-Plant, add {C}{C}{C} instead”.

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ARGIVIAN_ARCHAEOLOGIST,
    &ENERGY_FLUX,
    &HURKYLS_RECALL,
    &RECONSTRUCTION,
    &SAGE_OF_LAT_NAM,
    &ARTIFACT_BLAST,
    &ATOG,
    &DETONATE,
    &ORCISH_MECHANICS,
    &SHATTERSTORM,
    &ARGOTHIAN_PIXIES,
    &CITANUL_DRUID,
    &CRUMBLE,
    &DRAGON_ENGINE,
    &GOLGOTHIAN_SYLEX,
    &GRAPESHOT_CATAPULT,
    &IVORY_TOWER,
    &JALUM_TOME,
    &MIGHTSTONE,
    &MILLSTONE,
    &OBELISK_OF_UNDOING,
    &ONULET,
    &ORNITHOPTER,
    &STAFF_OF_ZEGON,
    &SU_CHI,
    &TABLET_OF_EPITYR,
    &TAWNOSS_WAND,
    &TETRAVUS,
    &TRISKELION,
    &URZAS_CHALICE,
    &WALL_OF_SPEARS,
    &WEAKSTONE,
    &YOTIAN_SOLDIER,
    &MISHRA_S_FACTORY,
    &MISHRA_S_WORKSHOP,
    &STRIP_MINE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];

#[cfg(test)]
mod tests {
    use super::{ENERGY_FLUX, ENERGY_FLUX_GRANTED_ABILITY};
    use crate::card::{
        AppliedEffectDef, CardEffectStatus, DeclarativeAbilityDef, EffectDef, ImplementationStatus,
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
            clauses[0].effect.definition,
            EffectDef::Apply {
                effect: AppliedEffectDef::GrantAbility(granted),
                ..
            } if granted == &ENERGY_FLUX_GRANTED_ABILITY
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
