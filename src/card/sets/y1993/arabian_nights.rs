use super::{CardRecord, PrintingRecord};
use crate::Format;
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    ActivationTimingDef, AddManaEffectDef, AppliedEffectDef, AppliedRuleDef, BasicLandType,
    CardArt, CardBehavior, CardChoiceSourceDef, CardRules, CardSet, CardType, ChoiceVisibilityDef,
    ComparisonDef, ControlDurationDef, CounterKind, DamageEventMatcherDef, DamageKindDef,
    DamageLimitDef, DamagePreventionDef, DamageRecipientMatcherDef, DamageSourceMatcherDef,
    DiscardSelectionDef, EffectDef, EffectExecutionDef, EffectPaymentCostDef, EffectPaymentDef,
    EffectRecipientDef, InstalledTriggerDef, KeywordAbility, LikelihoodDef, ManaColor,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, PayOrDef, PlayActionMatcherDef,
    PlayRestrictionDef, PlayerRefDef, PlayerRelation, PlayerSetDef, ResolvedEffectDurationDef,
    SacrificedAmountDef, TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind,
    ZonePlacement, abilities, cards,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

static DEFENDER_CONTROLS_AN_ISLAND: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
    &[ZoneKind::Battlefield],
    PlayerRelation::Opponent,
);

static YOU_CONTROL_NO_ISLANDS: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef::matching(
        ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    ),
    comparison: ComparisonDef::Equal,
    amount: 0,
};

static ELEPHANT_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::Subtype("Elephant"),
)];

/// Both sides of whatever block it was in. One direction is read off the
/// candidate and the other off Abu Ja'far, whose own record is last-known by
/// the time the trigger resolves -- it is dead, which is what set this off.
static ABU_JAFARS_COMPANIONS: EffectRecipientDef = EffectRecipientDef::matching_objects(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::BlockedBySource,
            ObjectPredicateDef::BlockingSource,
        ]),
    ]),
    &[ZoneKind::Battlefield],
    PlayerRelation::Any,
);

// ARN 1 — Abu Ja'far
pub(in crate::card::sets) static ABU_JAFAR: CardRecord = CardRecord::new(
    cards::ABU_JAFAR,
    "Abu Ja'far",
    CardArt::new("949634bd-2f5a-4be7-ad24-d7039a57b6d6", "Ken Meyer, Jr."),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{W}"), &["Human"], 0, 1).with_ability(
        AbilityDef::triggered(
            "When this creature dies, destroy all creatures blocking or blocked by it. They \
             can't be regenerated.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::Destroy {
                object: ABU_JAFARS_COMPANIONS,
                can_regenerate: false,
            },
        ),
    ),
);

// ARN 2 — Army of Allah
pub(in crate::card::sets) static ARMY_OF_ALLAH: CardRecord = CardRecord::new(
    cards::ARMY_OF_ALLAH,
    "Army of Allah",
    CardArt::new("3d170015-b125-49a6-a15e-8fd116bbcb14", "Brian Snõddy"),
    CardSet::ArabianNights,
    CardRules::new_instant(mana_cost!("{1}{W}{W}")).with_abilities(&[AbilityDef::spell(
        "Attacking creatures get +2/+0 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::Attacking,
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(2),
                ValueDef::Constant(0),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// ARN 3 — Camel
// Audit: blocked — Needs a prevention that lives on the permanent and reads its combat state, for “As long as this creature is attacking, prevent all damage Deserts would deal to this creature and to creatures banded with this creature”. Deserts and band membership are both readable now.

// ARN 4 — Eye for an Eye
// Audit: blocked — Needs a shield keyed to a source chosen as the ability resolves; prevention shields attach to a recipient and spend on the next damage from any source, not from one named source for “The next time a source of your choice would deal damage to you this turn, instead that source deals that much damage to you and Eye for an Eye deals that much damage to that source's…”.

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
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AttackingOrBlocking,
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Attacking),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(0),
                ValueDef::Constant(3),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
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
pub(in crate::card::sets) static WAR_ELEPHANT: CardRecord = CardRecord::new(
    cards::WAR_ELEPHANT,
    "War Elephant",
    CardArt::new("7416c366-95cc-4799-b6c6-34d8fad8c202", "Kristen Bishop"),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Elephant"], 2, 2)
        .with_abilities(&[abilities::trample(), abilities::banding()]),
);

// ARN 12 — Dandân
pub(in crate::card::sets) static DANDAN: CardRecord = CardRecord::new(
    cards::DANDAN,
    "Dandân",
    CardArt::new("414d3cae-b8cf-4d53-bd6b-1aa83a828ba9", "Drew Tucker"),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{U}{U}"), &["Fish"], 4, 1).with_abilities(&[
        AbilityDef::static_ability(
            "This creature can't attack unless defending player controls an Island.",
            EffectDef::CannotAttackUnless(&DEFENDER_CONTROLS_AN_ISLAND),
        ),
        AbilityDef::triggered_if(
            "When you control no Islands, sacrifice this creature.",
            TriggerEventDef::StateCondition,
            &YOU_CONTROL_NO_ISLANDS,
            EffectDef::Sacrifice {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// ARN 13 — Fishliver Oil
pub(in crate::card::sets) static FISHLIVER_OIL: CardRecord = CardRecord::new(
    cards::FISHLIVER_OIL,
    "Fishliver Oil",
    CardArt::new("deb6ed87-aa07-4b5e-ac40-1e16dc2a817a", "Anson Maddocks"),
    CardSet::ArabianNights,
    CardRules::new_enchantment(mana_cost!("{1}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature has islandwalk.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::landwalk(
                        BasicLandType::Island,
                    )),
                },
            ),
        ]),
);

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
pub(in crate::card::sets) static GIANT_TORTOISE: CardRecord = CardRecord::new(
    cards::GIANT_TORTOISE,
    "Giant Tortoise",
    CardArt::new("096f7ac8-c639-4347-9767-7305eaf490ba", "Kaja Foglio"),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Turtle"], 1, 1).with_ability(
        AbilityDef::static_ability(
            "This creature gets +0/+3 as long as it's untapped.",
            EffectDef::StaticApply {
                // Its own condition: the recipient is the source, but only
                // while untapped, so tapping to attack shrinks it.
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Source,
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(3),
                ),
            },
        ),
    ),
);

// ARN 16 — Island Fish Jasconius
pub(in crate::card::sets) static ISLAND_FISH_JASCONIUS: CardRecord = CardRecord::new(
    cards::ISLAND_FISH_JASCONIUS,
    "Island Fish Jasconius",
    CardArt::new("8537cb0f-4821-417b-80cc-ea57d51ee9b8", "Jesper Myrfors"),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{4}{U}{U}{U}"), &["Fish"], 6, 8).with_abilities(&[
        AbilityDef::static_ability(
            "This creature doesn't untap during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
        AbilityDef::triggered(
            "At the beginning of your upkeep, you may pay {U}{U}{U}. If you do, untap this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::PayOr(PayOrDef::optional(
                EffectPaymentDef::mana(
                    PlayerSetDef::Related(PlayerRelation::You),
                    mana_cost!("{U}{U}{U}"),
                ),
                &EffectDef::Untap {
                    object: EffectRecipientDef::Source,
                },
            )),
        ),
        AbilityDef::static_ability(
            "This creature can't attack unless defending player controls an Island.",
            EffectDef::CannotAttackUnless(&DEFENDER_CONTROLS_AN_ISLAND),
        ),
        AbilityDef::triggered_if(
            "When you control no Islands, sacrifice this creature.",
            TriggerEventDef::StateCondition,
            &YOU_CONTROL_NO_ISLANDS,
            EffectDef::Sacrifice {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// ARN 17 — Merchant Ship
pub(in crate::card::sets) static MERCHANT_SHIP: CardRecord = CardRecord::new(
    cards::MERCHANT_SHIP,
    "Merchant Ship",
    CardArt::new("2b827094-fb2c-46db-b898-02e0c308601f", "Tom Wänerstrand"),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{U}"), &["Human"], 0, 2).with_abilities(&[
        AbilityDef::static_ability(
            "This creature can't attack unless defending player controls an Island.",
            EffectDef::CannotAttackUnless(&DEFENDER_CONTROLS_AN_ISLAND),
        ),
        AbilityDef::triggered(
            "Whenever this creature attacks and isn't blocked, you gain 2 life.",
            TriggerEventDef::AttacksAndIsNotBlocked {
                attacker: ObjectPredicateDef::Source,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
        AbilityDef::triggered_if(
            "When you control no Islands, sacrifice this creature.",
            TriggerEventDef::StateCondition,
            &YOU_CONTROL_NO_ISLANDS,
            EffectDef::Sacrifice {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

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
pub(in crate::card::sets) static UNSTABLE_MUTATION: CardRecord = CardRecord::new(
    cards::UNSTABLE_MUTATION,
    "Unstable Mutation",
    CardArt::new("a79e9236-a39e-471a-b18a-2c2ba16e7774", "Douglas Shuler"),
    CardSet::ArabianNights,
    CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature gets +3/+3.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(3),
                    ),
                },
            ),
            abilities::enchanted_controller_upkeep(
                "At the beginning of the upkeep of enchanted creature's controller, put a \
                 -1/-1 counter on that creature.",
                EffectDef::AddCounters {
                    object: EffectRecipientDef::AttachedPermanent,
                    kind: CounterKind::MinusOneMinusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// ARN 23 — Cuombajj Witches
// Audit: blocked — Needs resolution to pause for an opponent-controlled second target choice after the controller's target is fixed.

// ARN 24 — El-Hajjâj
pub(in crate::card::sets) static EL_HAJJAJ: CardRecord = CardRecord::new(
    cards::EL_HAJJAJ,
    "El-Hajjâj",
    CardArt::new("c4b610d3-2005-4347-bcda-c30b5b7972e5", "Dameon Willich"),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Human", "Wizard"], 1, 1).with_ability(
        AbilityDef::triggered(
            "Whenever this creature deals damage, you gain that much life.",
            TriggerEventDef::damage_dealt_by(ObjectPredicateDef::Source),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::TriggerEventAmount,
            },
        ),
    ),
);

/// Two facts about the permanent itself, read at the end step: whether it went
/// to war, and whether it has been here long enough to be asked. The turn it
/// arrives is free, which is what stops it punishing a player who could not
/// have attacked with it.
static ERG_RAIDERS_IDLED: TriggerConditionDef = TriggerConditionDef::SourceMatches {
    object: ObjectPredicateDef::All(&[
        ObjectPredicateDef::Not(&ObjectPredicateDef::AttackedThisTurn),
        ObjectPredicateDef::Not(&ObjectPredicateDef::CameUnderControlThisTurn),
    ]),
};

static ERG_RAIDERS_TOLL: EffectDef = EffectDef::DealDamage {
    recipient: EffectRecipientDef::Controller,
    amount: ValueDef::Constant(2),
};

// ARN 25 — Erg Raiders
pub(in crate::card::sets) static ERG_RAIDERS: CardRecord = CardRecord::new(
    cards::ERG_RAIDERS,
    "Erg Raiders",
    CardArt::new("35c73a97-531d-4dd5-8236-39b89c183c38", "Dameon Willich"),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Warrior"], 2, 3).with_ability(
        AbilityDef::triggered_if(
            "At the beginning of your end step, if this creature didn't attack this turn, it \
             deals 2 damage to you unless it came under your control this turn.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            &ERG_RAIDERS_IDLED,
            ERG_RAIDERS_TOLL,
        ),
    ),
);

static GUARDIAN_BEAST_ARTIFACTS: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Artifact),
    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
]);

static GUARDIAN_BEAST_PROTECTION: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::matching_objects(
        GUARDIAN_BEAST_ARTIFACTS,
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    ),
    effect: AppliedEffectDef::Composite(&[
        AppliedEffectDef::Rule(AppliedRuleDef::CannotBecomeEnchanted),
        AppliedEffectDef::add_ability(&abilities::indestructible()),
        AppliedEffectDef::Rule(AppliedRuleDef::CannotChangeController),
    ]),
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
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::PayOr(PayOrDef::unless_mana(
                mana_cost!("{2}"),
                &EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
            )),
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
            EffectDef::PayOr(PayOrDef::unless_mana(
                mana_cost!("{B}{B}"),
                &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            )),
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

// ARN 30 — Khabál Ghoul
pub(in crate::card::sets) static KHABAL_GHOUL: CardRecord = CardRecord::new(
    cards::KHABAL_GHOUL,
    "Khabál Ghoul",
    CardArt::new("18607bf6-ce11-41cb-b001-0c9538406ba0", "Douglas Shuler"),
    CardSet::ArabianNights,
    // Each end step, not just yours, so a creature that dies on either turn
    // feeds it. The count is of deaths this turn rather than of bodies in a
    // graveyard, which is why it is tallied as they happen.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Zombie"], 1, 1).with_ability(
        AbilityDef::triggered(
            "At the beginning of each end step, put a +1/+1 counter on this creature for each \
             creature that died this turn.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::Any,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::CreaturesDiedThisTurn,
            },
        ),
    ),
);

// ARN 31 — Oubliette
// Audit: blocked — Needs a persistent tap/untap restriction or event relation for “When this enchantment enters, target creature phases out until this enchantment leaves the battlefield. Tap that creature as it phases in this way”.

// ARN 32 — Sorceress Queen
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
            effect: AppliedEffectDef::set_base_power_toughness(
                ValueDef::Constant(0),
                ValueDef::Constant(2),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
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
pub(in crate::card::sets) static ALADDIN: CardRecord = CardRecord::new(
    cards::ALADDIN,
    "Aladdin",
    CardArt::new("db52bad2-a3ec-4f6f-9418-12e8c40703f6", "Julie Baroh"),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Human", "Rogue"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{1}{R}{R}, {T}: Gain control of target artifact for as long as you control this \
             creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{R}{R}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Artifact),
            )],
            EffectDef::GainControl {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                duration: ControlDurationDef::WhileSourceRemains {
                    while_tapped: false,
                },
                controller: PlayerRefDef::EffectController,
            },
        ),
    ),
);

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

/// Every source and every kind: the printed text names no exception.
static ALI_FROM_CAIRO_ANY_DAMAGE: DamageEventMatcherDef = DamageEventMatcherDef {
    kind: DamageKindDef::Any,
    source: DamageSourceMatcherDef::Any,
    recipient: DamageRecipientMatcherDef::Any,
};

// ARN 36 — Ali from Cairo
pub(in crate::card::sets) static ALI_FROM_CAIRO: CardRecord = CardRecord::new(
    cards::ALI_FROM_CAIRO,
    "Ali from Cairo",
    CardArt::new("42027613-d261-4ce2-8ba1-7a2480c660f8", "Mark Poole"),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Human"], 0, 1).with_ability(
        AbilityDef::static_ability(
            "Damage that would reduce your life total to less than 1 reduces it to 1 instead.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::LimitDamage {
                    matcher: ALI_FROM_CAIRO_ANY_DAMAGE,
                    limit: DamageLimitDef::LeaveAtLeastLife(1),
                }),
            },
        ),
    ),
);

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
// Audit: blocked — Needs a landwalk naming a land subtype, and a prevention that lives on the permanent rather than being created by a resolving effect, for “Prevent all damage that would be dealt to this creature by Deserts”. Desert itself is now cataloged, so the land type the walk reads exists.

// ARN 39 — Hurr Jackal
pub(in crate::card::sets) static HURR_JACKAL: CardRecord = CardRecord::new(
    cards::HURR_JACKAL,
    "Hurr Jackal",
    CardArt::new("f4aadda8-8577-480d-8186-532d2b173c15", "Drew Tucker"),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{R}"), &["Jackal"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Target creature can't be regenerated this turn.",
            &[AbilityCostDef::TapSource],
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

/// The losing branch: out of combat, and tapped as if it had attacked.
static MIJAE_DJINN_LOST: EffectDef = EffectDef::Sequence(&[
    EffectDef::RemoveFromCombat {
        object: EffectRecipientDef::Source,
    },
    EffectDef::Tap {
        object: EffectRecipientDef::Source,
    },
]);

// ARN 42 — Mijae Djinn
pub(in crate::card::sets) static MIJAE_DJINN: CardRecord = CardRecord::new(
    cards::MIJAE_DJINN,
    "Mijae Djinn",
    CardArt::new("d3ddbe51-cd1a-4b2c-849a-7c82d622122a", "Susan Van Camp"),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{R}{R}{R}"), &["Djinn"], 6, 3).with_ability(
        AbilityDef::triggered(
            "Whenever this creature attacks, flip a coin. If you lose the flip, remove this \
             creature from combat and tap it.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Randomized {
                likelihood: LikelihoodDef::new(0.5),
                on_success: &EffectDef::None,
                on_failure: &MIJAE_DJINN_LOST,
            },
        ),
    ),
);

// ARN 43 — Rukh Egg
pub(in crate::card::sets) static RUKH_EGG: CardRecord = CardRecord::new(
    cards::RUKH_EGG,
    "Rukh Egg",
    CardArt::new("b28f9e63-e5e4-44b5-a17e-8301ff17c623", "Christopher Rush"),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Bird", "Egg"], 0, 3).with_abilities(&[
        AbilityDef::triggered(
            "When this creature dies, create a 4/4 red Bird creature token with flying at the beginning of the next end step.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, Some(ZoneKind::Battlefield), Some(ZoneKind::Graveyard)),
            EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                "At the beginning of the next end step, create a 4/4 red Bird creature token with flying.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::Any,
                },
                EffectDef::create_creature_token(&["Bird"], &[ManaColor::Red], 4, 4)
                    .with_abilities(&[abilities::flying()])
                    .with_art(CardArt::new(
                        "b5489e26-6aec-4706-9c3e-8454878fa6c3",
                        "Edward P. Beard, Jr.",
                    )),
            ))),
        ),
    ]),
);

// ARN 44 — Ydwen Efreet
// Audit: blocked — Needs attackers this creature had blocked alone to become unblocked, which reverses the ordinary rule that removing a blocker leaves them blocked. The flip and the combat removal are available.

/// The damage is one number dealt twice over: every creature and every
/// player, including its own controller and their own board.
static CYCLONE_SWEEP: [EffectDef; 2] = [
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::matching_objects(
            ObjectPredicateDef::HasType(CardType::Creature),
            &[ZoneKind::Battlefield],
            PlayerRelation::Any,
        ),
        amount: ValueDef::CountersOnSource(CounterKind::Wind),
    },
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::EachPlayer,
        amount: ValueDef::CountersOnSource(CounterKind::Wind),
    },
];

static CYCLONE_SWEEP_SEQUENCE: EffectDef = EffectDef::Sequence(&CYCLONE_SWEEP);

static CYCLONE_SACRIFICE: EffectDef = EffectDef::Sacrifice {
    object: EffectRecipientDef::Source,
};

/// The counter goes on first, so the upkeep it lands on is already paying for
/// it: the first upkeep costs {G} rather than nothing.
static CYCLONE_UPKEEP: [EffectDef; 2] = [
    EffectDef::AddCounters {
        object: EffectRecipientDef::Source,
        kind: CounterKind::Wind,
        amount: ValueDef::Constant(1),
    },
    EffectDef::PayOr(PayOrDef {
        payment: EffectPaymentDef {
            payer: PlayerSetDef::Related(PlayerRelation::You),
            cost: EffectPaymentCostDef::ColoredMana {
                color: ManaColor::Green,
                amount: ValueDef::CountersOnSource(CounterKind::Wind),
            },
        },
        if_paid: Some(&CYCLONE_SWEEP_SEQUENCE),
        otherwise: Some(&CYCLONE_SACRIFICE),
        visibility: ChoiceVisibilityDef::Public,
    }),
];

// ARN 45 — Cyclone
pub(in crate::card::sets) static CYCLONE: CardRecord = CardRecord::new(
    cards::CYCLONE,
    "Cyclone",
    CardArt::new("f11684d6-5b74-47a7-a2d0-256c9e437aa6", "Mark Tedin"),
    CardSet::ArabianNights,
    CardRules::new_enchantment(mana_cost!("{2}{G}{G}")).with_ability(AbilityDef::triggered(
        "At the beginning of your upkeep, put a wind counter on this enchantment, then \
         sacrifice this enchantment unless you pay {G} for each wind counter on it. If you \
         pay, this enchantment deals damage equal to the number of wind counters on it to \
         each creature and each player.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::You,
        },
        EffectDef::Sequence(&CYCLONE_UPKEEP),
    )),
);

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
            effect: AppliedEffectDef::add_ability(&ERHNAM_FORESTWALK),
            duration: ResolvedEffectDurationDef::UntilYourNextUpkeep,
        },
    )]),
);

// ARN 49 — Ghazbán Ogre
// Audit: blocked — Needs duration-aware control-changing continuous effects for “At the beginning of your upkeep, if a player has more life than each other player, the player with the most life gains control of this creature”.

/// Hurricane in miniature, and it catches the Efreet too: it flies, so its
/// own ability hits it.
static IFH_BIFF_STRIKE: [EffectDef; 2] = [
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::matching_objects(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
            ]),
            &[ZoneKind::Battlefield],
            PlayerRelation::Any,
        ),
        amount: ValueDef::Constant(1),
    },
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::EachPlayer,
        amount: ValueDef::Constant(1),
    },
];

// ARN 50 — Ifh-Bíff Efreet
pub(in crate::card::sets) static IFH_BIFF_EFREET: CardRecord = CardRecord::new(
    cards::IFH_BIFF_EFREET,
    "Ifh-Bíff Efreet",
    CardArt::new("c0b10fb7-8667-42bf-aeb6-35767a82917b", "Jesper Myrfors"),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Efreet"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{G}: This creature deals 1 damage to each creature with flying and each player. \
             Any player may activate this ability.",
            &[AbilityCostDef::Mana(mana_cost!("{G}"))],
            EffectDef::Sequence(&IFH_BIFF_STRIKE),
        )
        .open_to_any_player(),
    ]),
);

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
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Attacking,
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            amount: ValueDef::Constant(1),
        },
    )]),
);

// ARN 54 — Singing Tree
pub(in crate::card::sets) static SINGING_TREE: CardRecord = CardRecord::new(
    cards::SINGING_TREE,
    "Singing Tree",
    CardArt::new("3003bf1e-8085-45d8-882b-c449109e7631", "Rob Alexander"),
    CardSet::ArabianNights,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Plant"], 0, 3).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Target attacking creature has base power 0 until end of turn.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Attacking,
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::set_base_power(ValueDef::Constant(0)),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

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
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
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
    query: ObjectQueryDef::matching(BOTTLED, &[ZoneKind::Battlefield], PlayerRelation::Any),
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
pub(in crate::card::sets) static BOTTLE_OF_SULEIMAN: CardRecord = CardRecord::new(
    cards::BOTTLE_OF_SULEIMAN,
    "Bottle of Suleiman",
    CardArt::new("c474cd6b-5610-49eb-ac98-918d900efe8b", "Jesper Myrfors"),
    CardSet::ArabianNights,
    CardRules::new_artifact(mana_cost!("{4}")).with_ability(AbilityDef::activated(
        "{1}, Sacrifice this artifact: Flip a coin. If you win the flip, create a 5/5 colorless \
         Djinn artifact creature token with flying. If you lose the flip, this artifact deals 5 \
         damage to you.",
        &[
            AbilityCostDef::Mana(mana_cost!("{1}")),
            AbilityCostDef::SacrificeSource,
        ],
        EffectDef::Randomized {
            likelihood: LikelihoodDef::new(0.5),
            on_success: &BOTTLE_OF_SULEIMAN_WON,
            on_failure: &BOTTLE_OF_SULEIMAN_LOST,
        },
    )),
);

static BOTTLE_OF_SULEIMAN_WON: EffectDef =
    EffectDef::create_artifact_creature_token(&["Djinn"], &[], 5, 5)
        .with_abilities(&[abilities::flying()]);

static BOTTLE_OF_SULEIMAN_LOST: EffectDef = EffectDef::DealDamage {
    recipient: EffectRecipientDef::Controller,
    amount: ValueDef::Constant(5),
};

// ARN 59 — Brass Man
pub(in crate::card::sets) static BRASS_MAN: CardRecord = CardRecord::new(
    cards::BRASS_MAN,
    "Brass Man",
    CardArt::new("1a364362-e42b-415c-9d95-b6ec7139f5e7", "Christopher Rush"),
    CardSet::ArabianNights,
    CardRules::new_artifact_creature(mana_cost!("{1}"), &["Construct"], 1, 3).with_abilities(&[
        AbilityDef::static_ability(
            "This creature doesn't untap during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
        AbilityDef::triggered(
            "At the beginning of your upkeep, you may pay {1}. If you do, untap this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::PayOr(PayOrDef::optional(
                EffectPaymentDef::mana(
                    PlayerSetDef::Related(PlayerRelation::You),
                    mana_cost!("{1}"),
                ),
                &EffectDef::Untap {
                    object: EffectRecipientDef::Source,
                },
            )),
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
                object: EffectRecipientDef::matching_objects(BOTTLED, &[ZoneKind::Battlefield], PlayerRelation::Any),
            },
        ),
        AbilityDef::static_ability(
            "Players can't cast spells or play lands with a name originally printed in the Arabian Nights expansion.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::EachPlayer,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(
                    PlayRestrictionDef::new(PlayActionMatcherDef::Any, FROM_THE_BOTTLE),
                )),
            },
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
                EffectDef::Sequence(&[
                    EffectDef::PreventDamage {
                        prevention: DamagePreventionDef::unlimited(
                            DamageEventMatcherDef::combat_to(EffectRecipientDef::Target(
                                TargetIndex::PRIMARY,
                            )),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                    EffectDef::PreventDamage {
                        prevention: DamagePreventionDef::unlimited(
                            DamageEventMatcherDef::combat_from(ObjectRefDef::Target(
                                TargetIndex::PRIMARY,
                            )),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
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
                effect: AppliedEffectDef::add_ability(&abilities::flying()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
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
    arrival_effect: None,
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
    arrival_effect: None,
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
// Audit: blocked — Needs a delayed trigger armed on the granted creature dying later this turn; granting islandwalk itself is available.

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
                then: None,
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
            TriggerEventDef::tapped(ObjectPredicateDef::Source),
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

/// "Target attacking creature", which the end-of-combat window still has
/// standing in front of it: combat damage is dealt, but nothing is removed
/// from combat until the step finishes.
static DESERT_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::Attacking,
    ]),
)];

// ARN 72 — Desert
pub(in crate::card::sets) static DESERT: CardRecord = CardRecord::new(
    cards::DESERT,
    "Desert",
    CardArt::new("201155ea-f474-4e13-acda-cb071a6ca977", "Jesper Myrfors"),
    CardSet::ArabianNights,
    CardRules::new_land(&[])
        .with_subtypes(&["Desert"])
        .with_abilities(&[
            abilities::tap_for(ManaColor::Colorless),
            AbilityDef::activated_with_targets(
                "{T}: This land deals 1 damage to target attacking creature. Activate only \
                 during the end of combat step.",
                &[AbilityCostDef::TapSource],
                &DESERT_TARGET,
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
            )
            .with_activation_timing(ActivationTimingDef::EndOfCombat),
        ]),
);

/// The life is whatever the creature's toughness was, which is last-known by
/// the time this runs -- the creature is already gone, which is the point.
static DIAMOND_VALLEY_PAYOFF: EffectDef = EffectDef::GainLife {
    recipient: EffectRecipientDef::Controller,
    amount: ValueDef::TriggerEventAmount,
};

// ARN 73 — Diamond Valley
pub(in crate::card::sets) static DIAMOND_VALLEY: CardRecord = CardRecord::new(
    cards::DIAMOND_VALLEY,
    "Diamond Valley",
    CardArt::new("16674f11-6cd8-41f6-ae6a-f8578187287c", "Brian Snõddy"),
    CardSet::ArabianNights,
    CardRules::new_land(&[]).with_ability(AbilityDef::activated(
        "{T}, Sacrifice a creature: You gain life equal to the sacrificed creature's toughness.",
        &[AbilityCostDef::TapSource],
        EffectDef::SacrificeOfChoice {
            player: EffectRecipientDef::Controller,
            object: ObjectPredicateDef::HasType(CardType::Creature),
            then: Some(&DIAMOND_VALLEY_PAYOFF),
            amount: SacrificedAmountDef::Toughness,
            otherwise: None,
            optional: false,
        },
    )),
);

// ARN 74 — Elephant Graveyard
pub(in crate::card::sets) static ELEPHANT_GRAVEYARD: CardRecord = CardRecord::new(
    cards::ELEPHANT_GRAVEYARD,
    "Elephant Graveyard",
    CardArt::new("18348df2-9037-4db4-bddb-76dc933229bf", "Rob Alexander"),
    CardSet::ArabianNights,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{T}: Regenerate target Elephant.",
            &[AbilityCostDef::TapSource],
            &ELEPHANT_TARGET,
            EffectDef::Regenerate {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

static ISLAND_OF_WAK_WAK_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
    ]),
)];

// ARN 75 — Island of Wak-Wak
pub(in crate::card::sets) static ISLAND_OF_WAK_WAK: CardRecord = CardRecord::new(
    cards::ISLAND_OF_WAK_WAK,
    "Island of Wak-Wak",
    CardArt::new("f09cbd18-79f1-49a0-a3bd-b380ff5ecf03", "Douglas Shuler"),
    CardSet::ArabianNights,
    CardRules::new_land(&[]).with_ability(AbilityDef::activated_with_targets(
        "{T}: Target creature with flying has base power 0 until end of turn.",
        &[AbilityCostDef::TapSource],
        &ISLAND_OF_WAK_WAK_TARGET,
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::set_base_power(ValueDef::Constant(0)),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

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
pub(in crate::card::sets) static OASIS: CardRecord = CardRecord::new(
    cards::OASIS,
    "Oasis",
    CardArt::new("6f38565e-88b9-433d-b0e9-a3b9734f183f", "Brian Snõddy"),
    CardSet::ArabianNights,
    CardRules::new_land(&[]).with_ability(AbilityDef::activated_with_targets(
        "{T}: Prevent the next 1 damage that would be dealt to target creature this turn.",
        &[AbilityCostDef::TapSource],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::PreventDamage {
            prevention: DamagePreventionDef::amount(
                DamageEventMatcherDef::to(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
                ValueDef::Constant(1),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ABU_JAFAR,
    &ARMY_OF_ALLAH,
    &KING_SULEIMAN,
    &MOORISH_CAVALRY,
    &PIETY,
    &REPENTANT_BLACKSMITH,
    &WAR_ELEPHANT,
    &DANDAN,
    &FISHLIVER_OIL,
    &FLYING_MEN,
    &GIANT_TORTOISE,
    &ISLAND_FISH_JASCONIUS,
    &MERCHANT_SHIP,
    &SERENDIB_EFREET,
    &UNSTABLE_MUTATION,
    &EL_HAJJAJ,
    &ERG_RAIDERS,
    &GUARDIAN_BEAST,
    &HASRAN_OGRESS,
    &JUNUN_EFREET,
    &JUZAM_DJINN,
    &KHABAL_GHOUL,
    &SORCERESS_QUEEN,
    &STONE_THROWING_DEVILS,
    &ALADDIN,
    &ALI_BABA,
    &ALI_FROM_CAIRO,
    &BIRD_MAIDEN,
    &HURR_JACKAL,
    &KIRD_APE,
    &MIJAE_DJINN,
    &RUKH_EGG,
    &CYCLONE,
    &DESERT_TWISTER,
    &ERHNAM_DJINN,
    &IFH_BIFF_EFREET,
    &SANDSTORM,
    &SINGING_TREE,
    &WYLULI_WOLF,
    &ALADDINS_RING,
    &BOTTLE_OF_SULEIMAN,
    &BRASS_MAN,
    &CITY_IN_A_BOTTLE,
    &DANCING_SCIMITAR,
    &EBONY_HORSE,
    &FLYING_CARPET,
    &JANDORS_SADDLEBAGS,
    &RING_OF_MARUF,
    &BAZAAR_OF_BAGHDAD,
    &CITY_OF_BRASS,
    &DESERT,
    &DIAMOND_VALLEY,
    &ELEPHANT_GRAVEYARD,
    &ISLAND_OF_WAK_WAK,
    &LIBRARY_OF_ALEXANDRIA,
    &OASIS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
