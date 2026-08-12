use super::{CardRecord, PrintingRecord};
use crate::Format;
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    AddManaEffectDef, AnimationDef, AppliedEffectDef, CardArt, CardBehavior, CardChoiceSourceDef,
    CardRules, CardSet, CardType, ComparisonDef, DiscardSelectionDef, EffectDef, EffectDurationDef,
    EffectExecutionDef, EffectRecipientDef, ManaColor, ObjectPredicateDef, ObjectQueryDef,
    PaymentDef, PlayerRelation, TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef,
    ZoneKind, ZonePlacement, abilities, cards,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

// ARN 1 — Abu Ja'far
// Audit: blocked — Needs a duration-scoped prohibition on creating or applying regeneration shields for “When this creature dies, destroy all creatures blocking or blocked by it. They can't be regenerated”.

// ARN 2 — Army of Allah
pub(in crate::card::sets) static ARMY_OF_ALLAH: CardRecord = CardRecord::new(
    cards::ARMY_OF_ALLAH,
    "Army of Allah",
    CardArt::new("3d170015-b125-49a6-a15e-8fd116bbcb14", "Brian Snõddy"),
    CardSet::ArabianNights,
    CardRules::new_instant(mana_cost!("{1}{W}{W}")).with_abilities(&[AbilityDef::spell(
        "Attacking creatures get +2/+0 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::Attacking,
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
            effect: AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Constant(2),
                toughness: ValueDef::Constant(0),
            },
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// ARN 3 — Camel
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “As long as this creature is attacking, prevent all damage Deserts would deal to this creature and to creatures banded with this creature”.

// ARN 4 — Eye for an Eye
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “The next time a source of your choice would deal damage to you this turn, instead that source deals that much damage to you and Eye for an Eye deals that much damage to that source's…”.

// ARN 5 — Jihad
// Audit: blocked — Needs a persistent dynamic characteristic choice and predicates that consume it for “White creatures get +2/+1 as long as the chosen player controls a nontoken permanent of the chosen color”.

// ARN 6 — King Suleiman
pub(in crate::card::sets) static KING_SULEIMAN: CardRecord = CardRecord::new(
    cards::KING_SULEIMAN,
    "King Suleiman",
    CardArt::new("4d3dce0f-2168-4f63-b2f9-156a11beeea7", "Mark Poole"),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Noble"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: Destroy target Djinn or Efreet.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Subtype("Djinn"),
                    ObjectPredicateDef::Subtype("Efreet"),
                ]),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
        ),
    ]),
);

// ARN 7 — Moorish Cavalry
pub(in crate::card::sets) static MOORISH_CAVALRY: CardRecord = CardRecord::new(
    cards::MOORISH_CAVALRY,
    "Moorish Cavalry",
    CardArt::new("f86f0781-7614-4779-a58d-f13ce96bdf33", "Dameon Willich"),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Human", "Knight"], 3, 3)
        .with_abilities(&[abilities::trample()]),
);

// ARN 8 — Piety
pub(in crate::card::sets) static PIETY: CardRecord = CardRecord::new(
    cards::PIETY,
    "Piety",
    CardArt::new("f649c571-d7ec-4ebc-9e18-b0657cab495b", "Mark Poole"),
    CardSet::ArabianNights,
    CardRules::new_instant(mana_cost!("{2}{W}")).with_abilities(&[AbilityDef::spell(
        "Blocking creatures get +0/+3 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AttackingOrBlocking,
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Attacking),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
            effect: AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Constant(0),
                toughness: ValueDef::Constant(3),
            },
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// ARN 9 — Repentant Blacksmith
pub(in crate::card::sets) static REPENTANT_BLACKSMITH: CardRecord = CardRecord::new(
    cards::REPENTANT_BLACKSMITH,
    "Repentant Blacksmith",
    CardArt::new("61fc30b6-1355-425b-a86f-18f59f83141c", "Drew Tucker"),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human"], 1, 2)
        .with_abilities(&[abilities::protection_from(ManaColor::Red)]),
);

// ARN 10 — Shahrazad
// Audit: blocked — Needs nested-game setup, execution, and result propagation for Shahrazad's library-backed subgame.

// ARN 11 — War Elephant
// Audit: blocked — Needs full banding group declaration, blocking, and combat-damage assignment semantics.

// ARN 12 — Dandân
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “This creature can't attack unless defending player controls an Island”.

// ARN 13 — Fishliver Oil
// Audit: blocked — Needs the printed landwalk variant and its defending-player land/blocking semantics for “Enchanted creature has islandwalk”.

// ARN 14 — Flying Men
pub(in crate::card::sets) static FLYING_MEN: CardRecord = CardRecord::new(
    cards::FLYING_MEN,
    "Flying Men",
    CardArt::new("25ab9a2b-e248-4ae2-aac3-b49fdb3e260a", "Christopher Rush"),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{U}"), &["Human"], 1, 1)
        .with_abilities(&[abilities::flying()]),
);

// ARN 15 — Giant Tortoise
// Audit: blocked — Needs a persistent tap/untap restriction or event relation for “This creature gets +0/+3 as long as it's untapped”.

// ARN 16 — Island Fish Jasconius
// Audit: blocked — Needs a persistent tap/untap restriction or event relation for “This creature doesn't untap during your untap step”.

// ARN 17 — Merchant Ship
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “This creature can't attack unless defending player controls an Island”.

// ARN 18 — Old Man of the Sea
// Audit: blocked — Needs duration-aware control-changing continuous effects for “{T}: Gain control of target creature with power less than or equal to this creature's power for as long as this creature remains tapped and that creature's power remains less than or…”.

// ARN 19 — Serendib Djinn
// Audit: blocked — Needs a chosen-land sacrifice whose sacrificed land subtype controls the follow-up damage branch.

// ARN 20 — Serendib Efreet
pub(in crate::card::sets) static SERENDIB_EFREET: CardRecord = CardRecord::new(
    cards::SERENDIB_EFREET,
    "Serendib Efreet",
    CardArt::new("cf56e862-3169-4f63-acd0-731080fa32f2", "Anson Maddocks"),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Efreet"], 3, 4).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "At the beginning of your upkeep, this creature deals 1 damage to you.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// ARN 21 — Sindbad
// Audit: blocked — Needs ordered-library inspection, selection, and visibility handling for “{T}: Draw a card and reveal it. If it isn't a land card, discard it”.

// ARN 22 — Unstable Mutation
// Audit: blocked — Needs an upkeep trigger whose event player is derived from the attached permanent's current controller for “At the beginning of the upkeep of enchanted creature's controller, put a -1/-1 counter on that creature”.

// ARN 23 — Cuombajj Witches
// Audit: blocked — Needs resolution to pause for an opponent-controlled second target choice after the controller's target is fixed.

// ARN 24 — El-Hajjâj
// Audit: blocked — Needs damage-history/source tracking or card-specific damage processing for “Whenever this creature deals damage, you gain that much life”.

// ARN 25 — Erg Raiders
// Audit: blocked — Needs duration-aware control-changing continuous effects for “At the beginning of your end step, if this creature didn't attack this turn, it deals 2 damage to you unless it came under your control this turn”.

static GUARDIAN_BEAST_ARTIFACTS: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Artifact),
    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
]);

static GUARDIAN_BEAST_PROTECTION: EffectDef = EffectDef::Apply {
    recipient: EffectRecipientDef::MatchingObjects {
        object: GUARDIAN_BEAST_ARTIFACTS,
        zones: &[ZoneKind::Battlefield],
        controller: PlayerRelation::You,
    },
    effect: AppliedEffectDef::Composite(&[
        AppliedEffectDef::CannotBecomeEnchanted,
        AppliedEffectDef::GrantAbility(&abilities::indestructible()),
        AppliedEffectDef::CannotChangeController,
    ]),
    duration: EffectDurationDef::WhileSourceRemainsInZone,
};

// ARN 26 — Guardian Beast
pub(in crate::card::sets) static GUARDIAN_BEAST: CardRecord = CardRecord::new(
    cards::GUARDIAN_BEAST,
    "Guardian Beast",
    CardArt::new("9941f83b-2903-4eab-ac6d-5313e3978fa3", "Ken Meyer, Jr."),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Beast"], 2, 4).with_abilities(&[
        AbilityDef::static_ability(
            "As long as this creature is untapped, noncreature artifacts you control can't be enchanted, they have indestructible, and other players can't gain control of them. This effect doesn't remove Auras already attached to those artifacts.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceUntapped,
                then: &GUARDIAN_BEAST_PROTECTION,
            },
        ),
    ]),
);

// ARN 27 — Hasran Ogress
pub(in crate::card::sets) static HASRAN_OGRESS: CardRecord = CardRecord::new(
    cards::HASRAN_OGRESS,
    "Hasran Ogress",
    CardArt::new("9f310cf5-0985-4826-9779-19a713089d6d", "Dan Frazier"),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{B}{B}"), &["Ogre"], 3, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature attacks, it deals 3 damage to you unless you pay {2}.",
            TriggerEventDef::Attacks(ObjectPredicateDef::Source),
            EffectDef::UnlessPaid {
                cost: mana_cost!("{2}"),
                otherwise: &EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
            },
        ),
    ]),
);

// ARN 28 — Junún Efreet
pub(in crate::card::sets) static JUNUN_EFREET: CardRecord = CardRecord::new(
    cards::JUNUN_EFREET,
    "Junún Efreet",
    CardArt::new("5f46783a-b91e-4829-a173-5515b09ca615", "Christopher Rush"),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Efreet"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "At the beginning of your upkeep, sacrifice this creature unless you pay {B}{B}.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::UnlessPaid {
                cost: mana_cost!("{B}{B}"),
                otherwise: &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            },
        ),
    ]),
);

// ARN 29 — Juzám Djinn
pub(in crate::card::sets) static JUZAM_DJINN: CardRecord = CardRecord::new(
    cards::JUZAM_DJINN,
    "Juzám Djinn",
    CardArt::new("31bf3f14-b5df-498b-a1bb-965885c82401", "Mark Tedin"),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Djinn"], 5, 5).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, this creature deals 1 damage to you.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

/// Setting the base stats does not remove the target's types, colors,
/// subtypes, or abilities.
static SORCERESS_QUEEN_BASE_STATS: AnimationDef = AnimationDef::new(0, 2);

// ARN 30 — Khabál Ghoul
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “At the beginning of each end step, put a +1/+1 counter on this creature for each creature that died this turn”.

// ARN 31 — Oubliette
// Audit: blocked — Needs a persistent tap/untap restriction or event relation for “When this enchantment enters, target creature phases out until this enchantment leaves the battlefield. Tap that creature as it phases in this way”.

// ARN 32 — Sorceress Queen
// Audit: partial — Setting base power/toughness overwrites added types and subtypes from a prior animation.
pub(in crate::card::sets) static SORCERESS_QUEEN: CardRecord = CardRecord::new(
    cards::SORCERESS_QUEEN,
    "Sorceress Queen",
    CardArt::new("94742003-f0f1-4483-b1a0-e7163995db1b", "Kaja Foglio"),
    CardSet::ArabianNights,
    CardRules::new_creature(
        mana_cost!("{1}{B}{B}"),
        &["Human", "Wizard", "Sorcerer"],
        1,
        1,
    )
    .with_abilities(&[AbilityDef::activated_with_targets(
        "{T}: Target creature other than this creature has base power and toughness 0/2 until end of turn.",
        &[AbilityCostDef::TapSource],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
            ]),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Animate(&SORCERESS_QUEEN_BASE_STATS),
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )
    .with_coverage(AbilityCoverageDef::partial(
        "The base-power/toughness setter overwrites a prior animation's added types and subtypes.",
    ))]),
);

// ARN 33 — Stone-Throwing Devils
pub(in crate::card::sets) static STONE_THROWING_DEVILS: CardRecord = CardRecord::new(
    cards::STONE_THROWING_DEVILS,
    "Stone-Throwing Devils",
    CardArt::new("d1c387dd-1347-4443-91ce-b71f7ccdceba", "Ken Meyer, Jr."),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{B}"), &["Devil"], 1, 1)
        .with_abilities(&[abilities::first_strike()]),
);

// ARN 34 — Aladdin
// Audit: blocked — Needs duration-aware control-changing continuous effects for “{1}{R}{R}, {T}: Gain control of target artifact for as long as you control this creature”.

// ARN 35 — Ali Baba
pub(in crate::card::sets) static ALI_BABA: CardRecord = CardRecord::new(
    cards::ALI_BABA,
    "Ali Baba",
    CardArt::new("29cd7064-3703-43e0-8702-d1ba13703fd8", "Julie Baroh"),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{R}"), &["Human", "Rogue"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{R}: Tap target Wall.",
            &[AbilityCostDef::Mana(mana_cost!("{R}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Subtype("Wall"),
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// ARN 36 — Ali from Cairo
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “Damage that would reduce your life total to less than 1 reduces it to 1 instead”.

// ARN 37 — Bird Maiden
pub(in crate::card::sets) static BIRD_MAIDEN: CardRecord = CardRecord::new(
    cards::BIRD_MAIDEN,
    "Bird Maiden",
    CardArt::new("5c1ba0b9-db01-447f-90cc-a2fc2c24146e", "Kaja Foglio"),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Bird"], 1, 2)
        .with_abilities(&[abilities::flying()]),
);

// ARN 38 — Desert Nomads
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “Prevent all damage that would be dealt to this creature by Deserts”.

// ARN 39 — Hurr Jackal
// Audit: blocked — Needs a duration-scoped prohibition on creating or applying regeneration shields for “{T}: Target creature can't be regenerated this turn”.

// ARN 40 — Kird Ape
pub(in crate::card::sets) static KIRD_APE: CardRecord = CardRecord::new(
    cards::KIRD_APE,
    "Kird Ape",
    CardArt::new("ebe8845e-df1c-481c-949c-aab84af99a05", "Ken Meyer, Jr."),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{R}"), &["Ape"], 1, 1)
    .with_abilities(&[AbilityDef::custom_full(
        "This creature gets +1/+2 as long as you control a Forest.",
        CardBehavior::KirdApe,
        "The conditional power and toughness bonus is implemented by the legacy characteristic evaluator.",
    )]),
);

// ARN 41 — Magnetic Mountain
// Audit: blocked — Needs a persistent tap/untap restriction or event relation for “At the beginning of each player's upkeep, that player may choose any number of tapped blue creatures they control and pay {4} for each creature chosen this way. If the player does, untap…”.

// ARN 42 — Mijae Djinn
// Audit: blocked — Needs a deterministic recorded coin-flip choice and both result branches for “Whenever this creature attacks, flip a coin. If you lose the flip, remove this creature from combat and tap it”.

// ARN 43 — Rukh Egg
// Audit: partial — Its delayed end-step effect resolves directly rather than becoming an orderable, respondable trigger.
pub(in crate::card::sets) static RUKH_EGG: CardRecord = CardRecord::new(
    cards::RUKH_EGG,
    "Rukh Egg",
    CardArt::new("b28f9e63-e5e4-44b5-a17e-8301ff17c623", "Christopher Rush"),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Bird", "Egg"], 0, 3).with_abilities(&[
        AbilityDef::triggered(
            "When this creature dies, create a 4/4 red Bird creature token with flying at the beginning of the next end step.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: Some(ZoneKind::Battlefield),
                to: Some(ZoneKind::Graveyard),
            },
            EffectDef::AtNextStep {
                step: TurnStepDef::End,
                player: PlayerRelation::Any,
                effect: &EffectDef::CreateToken {
                    token: cards::BIRD_TOKEN_4_4_RED,
                    count: ValueDef::Constant(1),
                },
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "The delayed end-step effect resolves directly instead of creating an orderable, respondable trigger.",
        )),
    ]),
);

// ARN 44 — Ydwen Efreet
// Audit: blocked — Needs a deterministic recorded coin-flip choice and both result branches for “Whenever this creature blocks, flip a coin. If you lose the flip, remove this creature from combat and it can't block this turn. Creatures it was blocking that had become blocked by only…”.

// ARN 45 — Cyclone
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “At the beginning of your upkeep, put a wind counter on this enchantment, then sacrifice this enchantment unless you pay {G} for each wind counter on it. If you pay, this enchantment…”.

// ARN 46 — Desert Twister
pub(in crate::card::sets) static DESERT_TWISTER: CardRecord = CardRecord::new(
    cards::DESERT_TWISTER,
    "Desert Twister",
    CardArt::new("0d77c149-cca2-45c7-bc83-5ba1872ad5e0", "Susan Van Camp"),
    CardSet::ArabianNights,
    CardRules::new_sorcery(mana_cost!("{4}{G}{G}")).with_abilities(&[AbilityDef::destroy_target(
        "Destroy target permanent.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Any),
        true,
    )]),
);

/// The gift is compulsory and goes to an opponent's creature, which is the
/// drawback the Djinn is priced around.
static ERHNAM_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Wall")),
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::Opponent),
        owner: None,
    },
)];

static ERHNAM_FORESTWALK: AbilityDef = abilities::forestwalk();

// ARN 47 — Drop of Honey
// Audit: blocked — Needs a duration-scoped prohibition on creating or applying regeneration shields for “At the beginning of your upkeep, destroy the creature with the least power. It can't be regenerated. If two or more creatures are tied for least power, you choose one of them”.

// ARN 48 — Erhnam Djinn
pub(in crate::card::sets) static ERHNAM_DJINN: CardRecord = CardRecord::new(
    cards::ERHNAM_DJINN,
    "Erhnam Djinn",
    CardArt::new("42bc0c3f-0a52-4bdc-83da-6484bf3102f3", "Ken Meyer, Jr."),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Djinn"], 4, 5)
    .with_abilities(&[AbilityDef::triggered_with_targets(
        "At the beginning of your upkeep, target non-Wall creature an opponent controls gains forestwalk until your next upkeep. (It can't be blocked as long as defending player controls a Forest.)",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::You,
        },
        &ERHNAM_TARGET,
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::GrantAbility(&ERHNAM_FORESTWALK),
            duration: EffectDurationDef::UntilYourNextUpkeep,
        },
    )]),
);

// ARN 49 — Ghazbán Ogre
// Audit: blocked — Needs duration-aware control-changing continuous effects for “At the beginning of your upkeep, if a player has more life than each other player, the player with the most life gains control of this creature”.

// ARN 50 — Ifh-Bíff Efreet
// Audit: blocked — Needs an activated ability that every player may activate while retaining the permanent as its damage source.

// ARN 51 — Metamorphosis
// Audit: blocked — Needs cost/mana provenance or dynamic payment support for “Add X mana of any one color, where X is 1 plus the sacrificed creature's mana value. Spend this mana only to cast creature spells”.

// ARN 52 — Nafs Asp
// Audit: blocked — Needs a delayed draw-step trigger with an intervening before-step payment window for the damaged player.

// ARN 53 — Sandstorm
pub(in crate::card::sets) static SANDSTORM: CardRecord = CardRecord::new(
    cards::SANDSTORM,
    "Sandstorm",
    CardArt::new("73cba9cd-73d9-442e-bd99-9cba9f398b64", "Brian Snõddy"),
    CardSet::ArabianNights,
    CardRules::new_instant(mana_cost!("{G}")).with_abilities(&[AbilityDef::spell(
        "Sandstorm deals 1 damage to each attacking creature.",
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Attacking,
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
            amount: ValueDef::Constant(1),
        },
    )]),
);

// ARN 54 — Singing Tree
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “{T}: Target attacking creature has base power 0 until end of turn”.

// ARN 55 — Wyluli Wolf
pub(in crate::card::sets) static WYLULI_WOLF: CardRecord = CardRecord::new(
    cards::WYLULI_WOLF,
    "Wyluli Wolf",
    CardArt::new("15ccebe1-ef08-4805-a65f-a1c57abed9f2", "Susan Van Camp"),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Wolf"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: Target creature gets +1/+1 until end of turn.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
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

/// "Nontoken" needs no clause of its own: a token was printed in no
/// expansion, so it never has a name originally printed in this one.
static BOTTLED: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::DebutSet(CardSet::ArabianNights),
    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
]);

/// The casting prohibition has no "other": City in a Bottle was itself
/// printed in Arabian Nights, so a second copy cannot be cast either.
static FROM_THE_BOTTLE: ObjectPredicateDef = ObjectPredicateDef::DebutSet(CardSet::ArabianNights);

static BOTTLED_PERMANENTS_EXIST: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef {
        object: BOTTLED,
        zones: &[ZoneKind::Battlefield],
        controller: PlayerRelation::Any,
    },
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 1,
};

// ARN 56 — Aladdin's Lamp
// Audit: blocked — Needs seeded random selection with replay-visible provenance for “{X}, {T}: The next time you would draw a card this turn, instead look at the top X cards of your library, put all but one of them on the bottom of your library in a random order, then…”.

// ARN 57 — Aladdin's Ring
pub(in crate::card::sets) static ALADDINS_RING: CardRecord = CardRecord::new(
    cards::ALADDINS_RING,
    "Aladdin's Ring",
    CardArt::new("bb2b74a2-cb74-4b54-b9c6-78c63f14cf5b", "Dan Frazier"),
    CardSet::ArabianNights,
    CardRules::new_artifact(mana_cost!("{8}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{8}, {T}: This artifact deals 4 damage to any target.",
            &[
                AbilityCostDef::Mana(mana_cost!("{8}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(4),
            },
        ),
    ]),
);

// ARN 58 — Bottle of Suleiman
// Audit: blocked — Needs a deterministic recorded coin-flip choice and both result branches for “{1}, Sacrifice this artifact: Flip a coin. If you win the flip, create a 5/5 colorless Djinn artifact creature token with flying. If you lose the flip, this artifact deals 5 damage to you”.

// ARN 59 — Brass Man
pub(in crate::card::sets) static BRASS_MAN: CardRecord = CardRecord::new(
    cards::BRASS_MAN,
    "Brass Man",
    CardArt::new("1a364362-e42b-415c-9d95-b6ec7139f5e7", "Christopher Rush"),
    CardSet::ArabianNights,
    CardRules::new_artifact_creature(mana_cost!("{1}"), &["Construct"], 1, 3).with_abilities(&[
        AbilityDef::static_ability(
            "This creature doesn't untap during your untap step.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::DoesNotUntapDuringUntapStep,
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
        AbilityDef::triggered(
            "At the beginning of your upkeep, you may pay {1}. If you do, untap this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::OptionalPayment {
                payment: PaymentDef::new(
                    PlayerRelation::You,
                    &[AbilityCostDef::Mana(mana_cost!("{1}"))],
                ),
                if_paid: &EffectDef::Untap {
                    object: EffectRecipientDef::Source,
                },
            },
        ),
    ]),
);

// ARN 60 — City in a Bottle
pub(in crate::card::sets) static CITY_IN_A_BOTTLE: CardRecord = CardRecord::new(
    cards::CITY_IN_A_BOTTLE,
    "City in a Bottle",
    CardArt::new("9598b346-a47d-4c4c-9571-156824e86b9c", "Drew Tucker"),
    CardSet::ArabianNights,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::triggered_if(
            "Whenever one or more other nontoken permanents with a name originally printed in the Arabian Nights expansion are on the battlefield, their controllers sacrifice them.",
            TriggerEventDef::StateCondition,
            &BOTTLED_PERMANENTS_EXIST,
            EffectDef::Sacrifice {
                object: EffectRecipientDef::MatchingObjects {
                    object: BOTTLED,
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::Any,
                },
            },
        ),
        AbilityDef::static_ability(
            "Players can't cast spells or play lands with a name originally printed in the Arabian Nights expansion.",
            EffectDef::PlayersCantPlay(&FROM_THE_BOTTLE),
        ),
    ]),
);

// ARN 61 — Dancing Scimitar
pub(in crate::card::sets) static DANCING_SCIMITAR: CardRecord = CardRecord::new(
    cards::DANCING_SCIMITAR,
    "Dancing Scimitar",
    CardArt::new("1eb2e494-1414-4d1f-91d2-7cb20acdb128", "Anson Maddocks"),
    CardSet::ArabianNights,
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Spirit"], 1, 5)
        .with_abilities(&[abilities::flying()]),
);

// ARN 62 — Ebony Horse
pub(in crate::card::sets) static EBONY_HORSE: CardRecord = CardRecord::new(
    cards::EBONY_HORSE,
    "Ebony Horse",
    CardArt::new("9ae81ec7-2b7d-4301-8114-032be5e6b663", "Dameon Willich"),
    CardSet::ArabianNights,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{2}, {T}: Untap target attacking creature you control. Prevent all combat damage that would be dealt to and dealt by that creature this turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}")),
                AbilityCostDef::TapSource,
            ],
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
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::PreventCombatDamageThisTurn {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ]),
        ),
    ]),
);

// ARN 63 — Flying Carpet
pub(in crate::card::sets) static FLYING_CARPET: CardRecord = CardRecord::new(
    cards::FLYING_CARPET,
    "Flying Carpet",
    CardArt::new("4b71ff49-ee0a-4065-9131-380468d62a30", "Mark Tedin"),
    CardSet::ArabianNights,
    CardRules::new_artifact(mana_cost!("{4}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{2}, {T}: Target creature gains flying until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::GrantAbility(&abilities::flying()),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ARN 64 — Jandor's Ring
// Audit: blocked — Needs a hidden-zone decision and continuation for “{2}, {T}, Discard the last card you drew this turn: Draw a card”.

// ARN 65 — Jandor's Saddlebags
pub(in crate::card::sets) static JANDORS_SADDLEBAGS: CardRecord = CardRecord::new(
    cards::JANDORS_SADDLEBAGS,
    "Jandor's Saddlebags",
    CardArt::new("bc4f4b92-7d4e-4b03-8cb4-e6b356c338b4", "Dameon Willich"),
    CardSet::ArabianNights,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{3}, {T}: Untap target creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{3}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// ARN 66 — Jeweled Bird
// Audit: blocked — Needs an ante zone, ante ownership queries, and the source-moving ante procedure for “Ante this artifact. If you do, put all other cards you own from the ante into your graveyard, then draw a card”.

// ARN 67 — Pyramids
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “• The next time target land would be destroyed this turn, remove all damage marked on it instead”.

static RING_ORACLE_SOURCES: [CardChoiceSourceDef; 1] = [CardChoiceSourceDef::OutsideGame];
static RING_OLD_SCHOOL_SOURCES: [CardChoiceSourceDef; 2] = [
    CardChoiceSourceDef::Zone(ZoneKind::Exile),
    CardChoiceSourceDef::OutsideGame,
];
static RING_ORACLE_CHOICE: EffectDef = EffectDef::ChooseCards {
    player: EffectRecipientDef::Controller,
    sources: &RING_ORACLE_SOURCES,
    object: ObjectPredicateDef::Any,
    minimum: 1,
    maximum: 1,
    reveal: false,
    destination: ZoneKind::Hand,
    placement: ZonePlacement::Top,
};
static RING_OLD_SCHOOL_CHOICE: EffectDef = EffectDef::ChooseCards {
    player: EffectRecipientDef::Controller,
    sources: &RING_OLD_SCHOOL_SOURCES,
    object: ObjectPredicateDef::Any,
    minimum: 1,
    maximum: 1,
    reveal: false,
    destination: ZoneKind::Hand,
    placement: ZonePlacement::Top,
};
static RING_FORMAT_CHOICE: EffectDef = EffectDef::IfFormat {
    format: Format::OldSchool9394,
    then: &RING_OLD_SCHOOL_CHOICE,
    otherwise: &RING_ORACLE_CHOICE,
};

// ARN 68 — Ring of Ma'rûf
pub(in crate::card::sets) static RING_OF_MARUF: CardRecord = CardRecord::new(
    cards::RING_OF_MARUF,
    "Ring of Ma'rûf",
    CardArt::new("fcc1004f-7cee-420a-9f0e-2986ed3ab852", "Dan Frazier"),
    CardSet::ArabianNights,
    CardRules::new_artifact(mana_cost!("{5}")).with_ability(AbilityDef::activated(
        "{5}, {T}, Exile this artifact: The next time you would draw a card this turn, instead put a card you own from outside the game into your hand.",
        &[
            AbilityCostDef::Mana(mana_cost!("{5}")),
            AbilityCostDef::TapSource,
            AbilityCostDef::ExileSource,
        ],
        EffectDef::ReplaceNextDrawThisTurn {
            player: EffectRecipientDef::Controller,
            effect: &RING_FORMAT_CHOICE,
        },
    )),
);

// ARN 69 — Sandals of Abdallah
// Audit: blocked — Needs the printed landwalk variant and its defending-player land/blocking semantics for “{2}, {T}: Target creature gains islandwalk until end of turn. When that creature dies this turn, destroy this artifact”.

// ARN 70 — Bazaar of Baghdad
pub(in crate::card::sets) static BAZAAR_OF_BAGHDAD: CardRecord = CardRecord::new(
    cards::BAZAAR_OF_BAGHDAD,
    "Bazaar of Baghdad",
    CardArt::new("ff37b863-f8c4-4584-8cc2-ac0e096e583f", "Jeff A. Menges"),
    CardSet::ArabianNights,
    CardRules::new_land(&[]).with_abilities(&[AbilityDef::activated(
        "{T}: Draw two cards, then discard three cards.",
        &[AbilityCostDef::TapSource],
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
            EffectDef::Discard {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
                selection: DiscardSelectionDef::RecipientChooses,
            },
        ]),
    )]),
);

// ARN 71 — City of Brass
pub(in crate::card::sets) static CITY_OF_BRASS: CardRecord = CardRecord::new(
    cards::CITY_OF_BRASS,
    "City of Brass",
    CardArt::new("f4e32327-380d-471e-813b-4c27477787ce", "Mark Tedin"),
    CardSet::ArabianNights,
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this land becomes tapped, it deals 1 damage to you.",
            TriggerEventDef::BecomesTapped(ObjectPredicateDef::Source),
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// ARN 72 — Desert
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “{T}: This land deals 1 damage to target attacking creature. Activate only during the end of combat step”.

// ARN 73 — Diamond Valley
// Audit: blocked — Needs a characteristic-layer effect or dynamic value for “{T}, Sacrifice a creature: You gain life equal to the sacrificed creature's toughness”.

// ARN 74 — Elephant Graveyard
// Audit: blocked — Needs regeneration shields and their destroy-event replacement procedure for “{T}: Regenerate target Elephant”.

// ARN 75 — Island of Wak-Wak
// Audit: blocked — Needs a characteristic-layer effect or dynamic value for “{T}: Target creature with flying has base power 0 until end of turn”.

// ARN 76 — Library of Alexandria
pub(in crate::card::sets) static LIBRARY_OF_ALEXANDRIA: CardRecord = CardRecord::new(
    cards::LIBRARY_OF_ALEXANDRIA,
    "Library of Alexandria",
    CardArt::new("ee266113-34ce-4189-84e7-ee2c86a2722c", "Mark Poole"),
    CardSet::ArabianNights,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{T}: Draw a card. Activate only if you have exactly seven cards in hand.",
            &[AbilityCostDef::TapSource],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        )
        .with_effect_execution(EffectExecutionDef::Custom(CardBehavior::LibraryOfAlexandria))
        .with_coverage(AbilityCoverageDef::explained_complete(
            "The seven-card activation restriction and card draw are implemented by the card-local activated-action resolver.",
        ))
        .with_legacy_procedure(),
    ]),
);

// ARN 78 — Oasis
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “{T}: Prevent the next 1 damage that would be dealt to target creature this turn”.

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ARMY_OF_ALLAH,
    &KING_SULEIMAN,
    &MOORISH_CAVALRY,
    &PIETY,
    &REPENTANT_BLACKSMITH,
    &FLYING_MEN,
    &SERENDIB_EFREET,
    &GUARDIAN_BEAST,
    &HASRAN_OGRESS,
    &JUNUN_EFREET,
    &JUZAM_DJINN,
    &SORCERESS_QUEEN,
    &STONE_THROWING_DEVILS,
    &ALI_BABA,
    &BIRD_MAIDEN,
    &KIRD_APE,
    &RUKH_EGG,
    &DESERT_TWISTER,
    &ERHNAM_DJINN,
    &SANDSTORM,
    &WYLULI_WOLF,
    &ALADDINS_RING,
    &BRASS_MAN,
    &CITY_IN_A_BOTTLE,
    &DANCING_SCIMITAR,
    &EBONY_HORSE,
    &FLYING_CARPET,
    &JANDORS_SADDLEBAGS,
    &RING_OF_MARUF,
    &BAZAAR_OF_BAGHDAD,
    &CITY_OF_BRASS,
    &LIBRARY_OF_ALEXANDRIA,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
