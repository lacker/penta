//! Avacyn Restored card records used by the built-in ISD–DGM Standard deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::sets::y1993::alpha;
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    AddManaEffectDef, AppliedEffectDef, AppliedRuleDef, BasicLandType, CardArt, CardBehavior,
    CardRules, CardSet, CardSupertype, CardType, ChoiceVisibilityDef, ColorChoiceOperationDef,
    ColorSet, ComparisonDef, ControlDurationDef, CounterKind, CreatureTypeSetDef,
    DamageEventMatcherDef, DamageKindDef, DamagePreventionDef, DamageRecipientMatcherDef,
    DamageSourceMatcherDef, DiscardSelectionDef, DividedTotal, EffectDef, EffectPaymentDef,
    EffectRecipientDef, KeywordAbility, ManaColor, ManaRestrictionDef, ManaSpendEffectDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, PayOrDef, PlayerRefDef, PlayerRelation,
    PlayerSetDef, ProtectedCreatureType, ReplacementChoiceDef, ReplacementEffectDef,
    ResolvedEffectDurationDef, SacrificedAmountDef, ScaledValueDef, SpellAdditionalCostCountDef,
    SpellAdditionalCostDef, SpendModeDef, TriggerConditionDef, TriggerEventDef, TurnStepDef,
    ValueDef, ZoneChangeEventMatcherDef, ZoneKind, ZonePlacement, abilities,
};
use crate::{TargetIndex, mana_cost};

// AVR 1 — Angel of Glory's Rise
// Audit: blocked — Needs simultaneous batch movement for all Zombies and all returned Humans rather than processing each object as a separate zone change.

// AVR 2 — Angel of Jubilation
// Audit: blocked — Needs a static prohibition on paying life or sacrificing creatures specifically to cast spells and activate abilities.

// AVR 3 — Angel's Mercy
pub(in crate::card::sets) static ANGELS_MERCY: CardRecord = CardRecord::new_with_legacy_id(
    750,
    "Angel's Mercy",
    CardArt::new("7a437999-26ae-49fa-8647-c8c2b4640702", "Greg Staples"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{2}{W}{W}")).with_ability(AbilityDef::spell(
        "You gain 7 life.",
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(7),
        },
    )),
);

// AVR 4 — Angelic Wall
pub(in crate::card::sets) static ANGELIC_WALL: CardRecord = CardRecord::new_with_legacy_id(
    751,
    "Angelic Wall",
    CardArt::new("d7b2450d-87a7-46dc-b43a-2db2abeca44f", "Allen Williams"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Wall"], 0, 4)
        .with_abilities(&[abilities::defender(), abilities::flying()]),
);

// AVR 5 — Archangel
pub(in crate::card::sets) static ARCHANGEL: CardRecord = CardRecord::new_with_legacy_id(
    752,
    "Archangel",
    CardArt::new("3741b2a7-7bda-481a-b8f8-9b04c96035b0", "Cynthia Sheppard"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{5}{W}{W}"), &["Angel"], 5, 5)
        .with_abilities(&[abilities::flying(), abilities::vigilance()]),
);

// AVR 6 — Avacyn, Angel of Hope
pub(in crate::card::sets) static AVACYN_ANGEL_OF_HOPE: CardRecord = CardRecord::new_with_legacy_id(
    753,
    "Avacyn, Angel of Hope",
    CardArt::new("ba149706-cd17-4da6-8403-ccfe2d6cb437", "Jason Chan"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{5}{W}{W}{W}"), &["Angel"], 8, 8)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            abilities::vigilance(),
            abilities::indestructible(),
            AbilityDef::static_ability(
                "Other permanents you control have indestructible.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                },
            ),
        ]),
);

// AVR 7 — Banishing Stroke
pub(in crate::card::sets) static BANISHING_STROKE: CardRecord = CardRecord::new_with_legacy_id(
    754,
    "Banishing Stroke",
    CardArt::new("238d8437-1abd-4bb7-8b5b-54f959bc2c79", "Igor Kieryluk"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{5}{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Put target artifact, creature, or enchantment on the bottom of its owner's library.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                ]),
            )],
            EffectDef::MoveToZone {
                counters: None,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Library,
                placement: ZonePlacement::Bottom,
                arrival_effect: None,
                attachment: None,
                controller: None,
            },
        ),
        abilities::miracle(mana_cost!("{W}")),
    ]),
);

// AVR 8 — Builder's Blessing
pub(in crate::card::sets) static BUILDERS_BLESSING: CardRecord = CardRecord::new_with_legacy_id(
    1920,
    "Builder's Blessing",
    CardArt::new("2ad27af1-b482-40d5-9dbb-11201ffa0410", "John Stanko"),
    CardSet::AvacynRestored,
    // Read continuously, so a creature loses the toughness the moment it taps
    // -- including as it is declared as an attacker.
    CardRules::new_enchantment(mana_cost!("{3}{W}")).with_ability(AbilityDef::static_ability(
        "Untapped creatures you control get +0/+2.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(0),
                ValueDef::Constant(2),
            ),
        },
    )),
);

// AVR 9 — Call to Serve
// Audit: blocked — Needs an attachment-scoped effect that adds the Angel subtype without replacing the creature's existing types.

// AVR 10 — Cathars' Crusade
pub(in crate::card::sets) static CATHARS_CRUSADE: CardRecord = CardRecord::new_with_legacy_id(
    755,
    "Cathars' Crusade",
    CardArt::new("78154978-9e7d-44e9-a03f-c578072a8ff7", "Karl Kopinski"),
    CardSet::AvacynRestored,
    CardRules::new_enchantment(mana_cost!("{3}{W}{W}")).with_ability(AbilityDef::triggered(
        "Whenever a creature you control enters, put a +1/+1 counter on each creature you control.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ]),
            None,
            Some(ZoneKind::Battlefield),
        ),
        EffectDef::AddCounters {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            kind: CounterKind::PlusOnePlusOne,
            amount: ValueDef::Constant(1),
        },
    )),
);

// AVR 11 — Cathedral Sanctifier
pub(in crate::card::sets) static CATHEDRAL_SANCTIFIER: CardRecord = CardRecord::new_with_legacy_id(
    756,
    "Cathedral Sanctifier",
    CardArt::new("76cac47a-9e83-4039-8d80-fa9bdadb7527", "Michael C. Hayes"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Cleric"], 1, 1).with_ability(
        AbilityDef::triggered(
            "When this creature enters, you gain 3 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ),
    ),
);

// AVR 12 — Cloudshift
// Audit: blocked — Linked exile returns a blinked permanent under its owner rather than preserving the spell controller required for a stolen creature.

static COMMANDERS_AUTHORITY_UPKEEP: AbilityDef = AbilityDef::triggered(
    "At the beginning of your upkeep, create a 1/1 white Human creature token.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    EffectDef::create_creature_token(&["Human"], &[ManaColor::White], 1, 1).with_art(CardArt::new(
        "8894949b-f190-461e-996a-cf2b39f08a5d",
        "Michael C. Hayes",
    )),
);

// AVR 13 — Commander's Authority
pub(in crate::card::sets) static COMMANDERS_AUTHORITY: CardRecord = CardRecord::new_with_legacy_id(
    757,
    "Commander's Authority",
    CardArt::new("08ef4383-11e7-4426-a04a-058570f46e47", "Johannes Voss"),
    CardSet::AvacynRestored,
    CardRules::new_enchantment(mana_cost!("{4}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature has \"At the beginning of your upkeep, create a 1/1 white Human creature token.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&COMMANDERS_AUTHORITY_UPKEEP),
                },
            ),
        ]),
);

// AVR 14 — Cursebreak
pub(in crate::card::sets) static CURSEBREAK: CardRecord = CardRecord::new_with_legacy_id(
    758,
    "Cursebreak",
    CardArt::new("c71a0883-316c-4870-a029-25f16952fbc0", "Sam Wolfe Connelly"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target enchantment. You gain 2 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Enchantment),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ]),
    )),
);

/// Every damage event the creature is the source of, not only the combat
/// ones: a Defanged creature's activated abilities are as harmless as its
/// attacks.
static DEFANG_SHIELD: DamageEventMatcherDef = DamageEventMatcherDef {
    kind: DamageKindDef::Any,
    source: DamageSourceMatcherDef::AffectedObject,
    recipient: DamageRecipientMatcherDef::Any,
};

// AVR 15 — Defang
pub(in crate::card::sets) static DEFANG: CardRecord = CardRecord::new_with_legacy_id(
    1749,
    "Defang",
    CardArt::new("4dfdca4d-d2f6-40b3-8973-2caec0e849e4", "Steven Belledin"),
    CardSet::AvacynRestored,
    CardRules::new_enchantment(mana_cost!("{1}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Prevent all damage that would be dealt by enchanted creature.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::PreventDamage(DEFANG_SHIELD)),
                },
            ),
        ]),
);

// AVR 16 — Defy Death
// Audit: blocked — Needs a continuation that retains the moved graveyard target's new object identity for the Angel test and +1/+1 counters.

// AVR 17 — Devout Chaplain
// Audit: blocked — Needs an activation cost that taps two separately chosen untapped Humans you control.

// AVR 18 — Divine Deflection
// Audit: blocked — Needs a duration-scoped prevention shield that tracks the amount prevented and redirects exactly that amount to a chosen target.

// AVR 19 — Emancipation Angel
// Audit: blocked — Needs a resolving non-target choice of a permanent you control to return to its owner's hand.

// AVR 20 — Entreat the Angels
pub(in crate::card::sets) static ENTREAT_THE_ANGELS: CardRecord = CardRecord::new_with_legacy_id(
    759,
    "Entreat the Angels",
    CardArt::new("31292616-70e6-4d19-a883-e63ad860f50c", "Todd Lockwood"),
    CardSet::AvacynRestored,
    CardRules::new_sorcery(mana_cost!("{X}{X}{W}{W}{W}")).with_abilities(&[
        AbilityDef::spell(
            "Create X 4/4 white Angel creature tokens with flying.",
            EffectDef::create_creature_token(&["Angel"], &[ManaColor::White], 4, 4)
                .with_abilities(&[abilities::flying()])
                .with_art(CardArt::new(
                    "68dd1682-a5d5-4323-b876-66a86c311c43",
                    "Anthony Palumbo",
                ))
                .with_count(ValueDef::ChosenX),
        ),
        abilities::miracle(mana_cost!("{X}{W}{W}")),
    ]),
);

// AVR 21 — Farbog Explorer
pub(in crate::card::sets) static FARBOG_EXPLORER: CardRecord = CardRecord::new_with_legacy_id(
    1875,
    "Farbog Explorer",
    CardArt::new("489c6a2f-38b4-4ff9-95f7-431384480ed9", "Scott Chou"),
    CardSet::AvacynRestored,
    // A white creature with swampwalk, which is the joke: it is unblockable
    // only against the colour least likely to want to block it.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Scout"], 2, 3)
        .with_ability(abilities::landwalk(BasicLandType::Swamp)),
);

// AVR 22 — Goldnight Commander
pub(in crate::card::sets) static GOLDNIGHT_COMMANDER: CardRecord = CardRecord::new_with_legacy_id(
    760,
    "Goldnight Commander",
    CardArt::new("c6ebec82-9d4a-4e78-b923-37c3a52133e7", "Chris Rahn"),
    CardSet::AvacynRestored,
    CardRules::new_creature(
        mana_cost!("{3}{W}"),
        &["Human", "Cleric", "Soldier"],
        2,
        2,
    )
    .with_ability(AbilityDef::triggered(
        "Whenever another creature you control enters, creatures you control get +1/+1 until end of turn.",
        TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
            ]), None, Some(ZoneKind::Battlefield)),
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::You),
            effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

/// "Other creatures you control", so the Redeemer's own arrival is not among
/// them even though it is on the battlefield as the trigger resolves.
static GOLDNIGHT_REDEEMER_OTHERS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
    ]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

static GOLDNIGHT_REDEEMER_AMOUNT: ScaledValueDef = ScaledValueDef::new(
    ValueDef::CountMatchingObjects(&GOLDNIGHT_REDEEMER_OTHERS),
    2,
);

// AVR 23 — Goldnight Redeemer
pub(in crate::card::sets) static GOLDNIGHT_REDEEMER: CardRecord = CardRecord::new_with_legacy_id(
    1876,
    "Goldnight Redeemer",
    CardArt::new("df5656e3-5f53-41f8-9f24-04caad5e4ca3", "Karl Kopinski"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{4}{W}{W}"), &["Angel"], 4, 4).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "When this creature enters, you gain 2 life for each other creature you control.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Scaled(&GOLDNIGHT_REDEEMER_AMOUNT),
            },
        ),
    ]),
);

// AVR 24 — Herald of War
// Audit: blocked — Needs a battlefield static cost reduction for other Angel and Human spells whose amount is the source's +1/+1-counter count.

static HOLY_JUSTICIAR_ZOMBIE: TriggerConditionDef = TriggerConditionDef::TargetMatches {
    slot: TargetIndex::PRIMARY,
    object: ObjectPredicateDef::Subtype("Zombie"),
};

// AVR 25 — Holy Justiciar
pub(in crate::card::sets) static HOLY_JUSTICIAR: CardRecord = CardRecord::new_with_legacy_id(
    761,
    "Holy Justiciar",
    CardArt::new("640cad49-1db3-4611-a80d-7ce95f000fad", "David Rapoza"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Cleric"], 2, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{2}{W}, {T}: Tap target creature. If that creature is a Zombie, exile it.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{W}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::IfCondition {
                    condition: &HOLY_JUSTICIAR_ZOMBIE,
                    then: &EffectDef::MoveToZone {
                        counters: None,
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        zone: ZoneKind::Exile,
                        placement: ZonePlacement::Top,
                        arrival_effect: None,
                        attachment: None,
                        controller: None,
                    },
                },
            ]),
        ),
    ),
);

static LEAP_OF_FAITH_FLYING: AbilityDef = abilities::flying();
static LEAP_OF_FAITH_EFFECTS: [EffectDef; 2] = [
    EffectDef::Apply {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        effect: AppliedEffectDef::add_ability(&LEAP_OF_FAITH_FLYING),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
    EffectDef::PreventDamage {
        prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::to(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
        )),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
];

// AVR 26 — Leap of Faith
pub(in crate::card::sets) static LEAP_OF_FAITH: CardRecord = CardRecord::new_with_legacy_id(
    1496,
    "Leap of Faith",
    CardArt::new("7ba52aed-440c-4b32-8f25-0c5364441712", "Gabor Szikszai"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{2}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gains flying until end of turn. Prevent all damage that would be dealt to that creature this turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&LEAP_OF_FAITH_EFFECTS),
    )),
);

// AVR 27 — Midnight Duelist
pub(in crate::card::sets) static MIDNIGHT_DUELIST: CardRecord = CardRecord::new_with_legacy_id(
    1909,
    "Midnight Duelist",
    CardArt::new("2371bd0c-ca38-4a62-b525-bef4d1ca0646", "Bud Cook"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 1, 2).with_ability(
        abilities::protection_from_creature_type(
            "Protection from Vampires",
            ProtectedCreatureType::Vampire,
        ),
    ),
);

// AVR 28 — Midvast Protector
pub(in crate::card::sets) static MIDVAST_PROTECTOR: CardRecord = CardRecord::new_with_legacy_id(
    1995,
    "Midvast Protector",
    CardArt::new("d4f6214f-90cb-4575-b221-3c8d0ed65ffe", "James Ryman"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Wizard"], 2, 3).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature enters, target creature you control gains protection from the color of your choice until end of turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::ChooseColor {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                operation: ColorChoiceOperationDef::ProtectionFromChosenColor,
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// AVR 29 — Moonlight Geist
pub(in crate::card::sets) static MOONLIGHT_GEIST: CardRecord = CardRecord::new_with_legacy_id(
    762,
    "Moonlight Geist",
    CardArt::new("4cf4c4cf-df35-4725-81ca-d62b70b8d0dd", "Dan Murayama Scott"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Spirit"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{3}{W}: Prevent all combat damage that would be dealt to and dealt by this creature this turn.",
            &[AbilityCostDef::Mana(mana_cost!("{3}{W}"))],
            EffectDef::Sequence(&[
                EffectDef::PreventDamage {
                    prevention: DamagePreventionDef::unlimited(
                        DamageEventMatcherDef::combat_to(EffectRecipientDef::Source),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::PreventDamage {
                    prevention: DamagePreventionDef::unlimited(
                        DamageEventMatcherDef::combat_from(ObjectRefDef::Source),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ]),
);

// AVR 30 — Moorland Inquisitor
pub(in crate::card::sets) static MOORLAND_INQUISITOR: CardRecord = CardRecord::new_with_legacy_id(
    763,
    "Moorland Inquisitor",
    CardArt::new("581dbbea-9995-4e4b-ba5c-d6d5597e4ace", "David Palumbo"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Soldier"], 2, 2).with_ability(
        AbilityDef::activated(
            "{2}{W}: This creature gains first strike until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

static NEARHEATH_PILGRIM_GRANTED: AbilityDef = abilities::lifelink();

static NEARHEATH_PILGRIM_BONUS: EffectDef = EffectDef::StaticApply {
    recipient: SOULBOND_PAIR_RECIPIENT,
    effect: AppliedEffectDef::add_ability(&NEARHEATH_PILGRIM_GRANTED),
};

// AVR 31 — Nearheath Pilgrim
pub(in crate::card::sets) static NEARHEATH_PILGRIM: CardRecord = CardRecord::new_with_legacy_id(
    1943,
    "Nearheath Pilgrim",
    CardArt::new("d81d6fe0-c7c2-46a6-811c-f121284937ea", "Erica Yang"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Cleric"], 2, 1).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, both creatures have \
             lifelink.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &NEARHEATH_PILGRIM_BONUS,
            },
        ),
    ]),
);

// AVR 32 — Restoration Angel
// Audit: partial — Linked exile returns a blinked permanent under its owner rather than this ability's controller when the target was stolen.
pub(in crate::card::sets) static RESTORATION_ANGEL: CardRecord = CardRecord::new_with_legacy_id(
    203,
    "Restoration Angel",
    CardArt::new("c2ad8639-e586-47f4-baca-2a1af5aa281b", "Johannes Voss"),
    CardSet::AvacynRestored,
    CardRules::new_creature(
        mana_cost!("{3}{W}"),
        &["Angel"],
        3,
        4,
    )
    .with_abilities(&[
        abilities::flash(),
        abilities::flying(),
        AbilityDef::triggered_with_targets("When this creature enters, you may exile target non-Angel creature you control, then return that card to the battlefield under your control.", TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)), &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Angel")),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
        )], // The exile links the card to this Angel and the return drains
            // that link immediately, so the creature blinks within one
            // resolution. The card comes back under its owner's control,
            // which is the printed controller for every creature this can
            // legally target unless control of it was already stolen.
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Sequence(&[
                    EffectDef::ExileLinkedToSource {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                    EffectDef::ReturnLinkedExiles {
                        object: ObjectPredicateDef::Any,
                        counters: None,
                        arrival_effect: None,
                        zone: ZoneKind::Battlefield,
                        grant: None,
                        controller: None,
                        transformed: false,
                    },
                ]),
            },
        )
            .with_coverage(AbilityCoverageDef::partial(
                "Linked exile returns a stolen target under its owner rather than this ability's controller.",
            )),
    ]),
);

// AVR 33 — Riders of Gavony
// Audit: blocked — Needs protection from creatures of a dynamically chosen creature type.

// AVR 34 — Righteous Blow
pub(in crate::card::sets) static RIGHTEOUS_BLOW: CardRecord = CardRecord::new_with_legacy_id(
    764,
    "Righteous Blow",
    CardArt::new("9b640fdc-7a19-475e-858f-e159f61e154e", "Clint Cearley"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Righteous Blow deals 2 damage to target attacking or blocking creature.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::AttackingOrBlocking,
            ]),
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(2),
        },
    )),
);

// AVR 35 — Seraph of Dawn
pub(in crate::card::sets) static SERAPH_OF_DAWN: CardRecord = CardRecord::new_with_legacy_id(
    765,
    "Seraph of Dawn",
    CardArt::new("5da345bd-8f2b-4966-97f5-c0e4c6cfe3b7", "Todd Lockwood"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Angel"], 2, 4)
        .with_abilities(&[abilities::flying(), abilities::lifelink()]),
);

static SILVERBLADE_PALADIN_GRANTED: AbilityDef = abilities::double_strike();

static SILVERBLADE_PALADIN_BONUS: EffectDef = EffectDef::StaticApply {
    recipient: SOULBOND_PAIR_RECIPIENT,
    effect: AppliedEffectDef::add_ability(&SILVERBLADE_PALADIN_GRANTED),
};

// AVR 36 — Silverblade Paladin
pub(in crate::card::sets) static SILVERBLADE_PALADIN: CardRecord = CardRecord::new_with_legacy_id(
    1932,
    "Silverblade Paladin",
    CardArt::new("16298ca0-80d4-4299-a550-500b7ef6ac67", "Jason Chan"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Human", "Knight"], 2, 2).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, both creatures have \\
             double strike.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &SILVERBLADE_PALADIN_BONUS,
            },
        ),
    ]),
);

static SPECTRAL_GATEGUARDS_GRANTED: AbilityDef = abilities::vigilance();

static SPECTRAL_GATEGUARDS_BONUS: EffectDef = EffectDef::StaticApply {
    recipient: SOULBOND_PAIR_RECIPIENT,
    effect: AppliedEffectDef::add_ability(&SPECTRAL_GATEGUARDS_GRANTED),
};

// AVR 37 — Spectral Gateguards
pub(in crate::card::sets) static SPECTRAL_GATEGUARDS: CardRecord = CardRecord::new_with_legacy_id(
    1933,
    "Spectral Gateguards",
    CardArt::new("f774e0eb-5c05-4a9e-8ab7-9ee4c7741591", "Wayne England"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Spirit", "Soldier"], 2, 5).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, both creatures have \\
             vigilance.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &SPECTRAL_GATEGUARDS_BONUS,
            },
        ),
    ]),
);

// AVR 38 — Terminus
// Audit: partial — Permanents move to library one at a time, and the engine does not offer each owner the required ordering choice for multiple cards put on the bottom.
pub(in crate::card::sets) static TERMINUS: CardRecord = CardRecord::new_with_legacy_id(
    225,
    "Terminus",
    CardArt::new("0982ea7e-05a4-4e40-98ab-ea9aa6c7342e", "James Paick"),
    CardSet::AvacynRestored,
    CardRules::new_sorcery(mana_cost!("{4}{W}{W}")).with_abilities(&[
        AbilityDef::spell(
            "Put all creatures on the bottom of their owners' libraries.",
            EffectDef::MoveToZone {
                counters: None,
                object: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::Any),
                zone: ZoneKind::Library,
                controller: None,
                placement: ZonePlacement::Bottom,
                arrival_effect: None,
                attachment: None,
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "Permanents move to library one at a time, and the engine does not offer each owner the required ordering choice for multiple cards put on the bottom.",
        )),
        abilities::miracle(mana_cost!("{W}")),
    ]),
);

// AVR 39 — Thraben Valiant
pub(in crate::card::sets) static THRABEN_VALIANT: CardRecord = CardRecord::new_with_legacy_id(
    766,
    "Thraben Valiant",
    CardArt::new("20558f69-9240-49b9-9695-caf75ee2db1b", "Jason Chan"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Soldier"], 2, 1)
        .with_ability(abilities::vigilance()),
);

// AVR 40 — Voice of the Provinces
pub(in crate::card::sets) static VOICE_OF_THE_PROVINCES: CardRecord =
    CardRecord::new_with_legacy_id(
        767,
        "Voice of the Provinces",
        CardArt::new("b785276b-3778-49f3-b46f-a1f3d91db097", "Igor Kieryluk"),
        CardSet::AvacynRestored,
        CardRules::new_creature(mana_cost!("{4}{W}{W}"), &["Angel"], 3, 3).with_abilities(&[
            abilities::flying(),
            AbilityDef::triggered(
                "When this creature enters, create a 1/1 white Human creature token.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::create_creature_token(&["Human"], &[ManaColor::White], 1, 1).with_art(
                    CardArt::new("8894949b-f190-461e-996a-cf2b39f08a5d", "Michael C. Hayes"),
                ),
            ),
        ]),
    );

// AVR 41 — Zealous Strike
pub(in crate::card::sets) static ZEALOUS_STRIKE: CardRecord = CardRecord::new_with_legacy_id(
    768,
    "Zealous Strike",
    CardArt::new("ae8a01fb-dd47-44de-b528-8b7ca4b3388b", "Bud Cook"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +2/+2 and gains first strike until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                AppliedEffectDef::add_ability(&abilities::first_strike()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// AVR 42 — Alchemist's Apprentice
pub(in crate::card::sets) static ALCHEMISTS_APPRENTICE: CardRecord = CardRecord::new_with_legacy_id(
    769,
    "Alchemist's Apprentice",
    CardArt::new("31abba67-1241-4fb3-88b5-4c4668ec5f25", "David Palumbo"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Human", "Wizard"], 1, 1).with_ability(
        AbilityDef::activated(
            "Sacrifice this creature: Draw a card.",
            &[AbilityCostDef::SacrificeSource],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// AVR 43 — Amass the Components
// Audit: blocked — Needs a resolving hand-card choice after drawing and a continuation that puts the chosen card on the bottom of its owner's library.

/// The only one of these that discounts both sides of the table, which is
/// what the caster relation is for.
static ARCANE_MELEE_SPELLS: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasType(CardType::Instant),
    ObjectPredicateDef::HasType(CardType::Sorcery),
]);

// AVR 44 — Arcane Melee
pub(in crate::card::sets) static ARCANE_MELEE: CardRecord = CardRecord::new_with_legacy_id(
    1761,
    "Arcane Melee",
    CardArt::new("f70eb8ee-3810-4cff-b87f-b6cf7849c018", "Jaime Jones"),
    CardSet::AvacynRestored,
    CardRules::new_enchantment(mana_cost!("{4}{U}")).with_ability(AbilityDef::static_ability(
        "Instant and sorcery spells cost {2} less to cast.",
        EffectDef::ReduceMatchingSpellCostBy {
            spell: ARCANE_MELEE_SPELLS,
            caster: PlayerRelation::Any,
            amount: ValueDef::Constant(2),
        },
    )),
);

// AVR 45 — Captain of the Mists
// Audit: blocked — Needs a tap-or-untap choice on a single activated ability; the shared modal vocabulary currently covers spells only.

/// The tap and the skip are separate: a creature already tapped still owes
/// the untap step it misses, which is what the second clause is for.
static CRIPPLING_CHILL_EFFECT: [EffectDef; 3] = [
    EffectDef::Tap {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::SkipNextUntapSteps {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        count: 1,
    },
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
];

// AVR 46 — Crippling Chill
pub(in crate::card::sets) static CRIPPLING_CHILL: CardRecord = CardRecord::new_with_legacy_id(
    1849,
    "Crippling Chill",
    CardArt::new("79791bd9-aded-48d9-866d-9f7bd6848905", "Svetlin Velinov"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Tap target creature. It doesn't untap during its controller's next untap step. Draw a \
         card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&CRIPPLING_CHILL_EFFECT),
    )),
);

// AVR 47 — Deadeye Navigator
// Audit: blocked — Needs soulbond pairing state, paired-object identity, and an activated blink ability granted to both paired creatures.

// AVR 48 — Devastation Tide
// Audit: blocked — Needs simultaneous batch movement for all nonland permanents rather than processing each battlefield exit separately.

static DREADWATERS_LANDS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Land),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

// AVR 49 — Dreadwaters
pub(in crate::card::sets) static DREADWATERS: CardRecord = CardRecord::new_with_legacy_id(
    770,
    "Dreadwaters",
    CardArt::new("88245a41-d4d5-46bf-969f-48d4dd540e2c", "Cliff Childs"),
    CardSet::AvacynRestored,
    CardRules::new_sorcery(mana_cost!("{3}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target player mills X where X is the number of lands you control.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Mill {
            player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::CountMatchingObjects(&DREADWATERS_LANDS),
            binding: None,
            then: None,
        },
    )),
);

static ELGAUD_SHIELDMATE_GRANTED: AbilityDef = abilities::hexproof();

static ELGAUD_SHIELDMATE_BONUS: EffectDef = EffectDef::StaticApply {
    recipient: SOULBOND_PAIR_RECIPIENT,
    effect: AppliedEffectDef::add_ability(&ELGAUD_SHIELDMATE_GRANTED),
};

// AVR 50 — Elgaud Shieldmate
pub(in crate::card::sets) static ELGAUD_SHIELDMATE: CardRecord = CardRecord::new_with_legacy_id(
    1934,
    "Elgaud Shieldmate",
    CardArt::new("e7d376ef-c900-4abb-9a0b-5eb9369f5739", "Anthony Palumbo"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Human", "Soldier"], 2, 3).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, both creatures have \\
             hexproof.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &ELGAUD_SHIELDMATE_BONUS,
            },
        ),
    ]),
);

// AVR 51 — Favorable Winds
pub(in crate::card::sets) static FAVORABLE_WINDS: CardRecord = CardRecord::new_with_legacy_id(
    1639,
    "Favorable Winds",
    CardArt::new("4cbd57f1-9883-40a4-9b52-1649cee83815", "Winona Nelson"),
    CardSet::AvacynRestored,
    CardRules::new_enchantment(mana_cost!("{1}{U}")).with_ability(AbilityDef::static_ability(
        "Creatures you control with flying get +1/+1.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(1),
            ),
        },
    )),
);

static FETTERGEIST_OTHERS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
    ]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

static FETTERGEIST_SACRIFICE: EffectDef = EffectDef::Sacrifice {
    object: EffectRecipientDef::Source,
};

// AVR 52 — Fettergeist
pub(in crate::card::sets) static FETTERGEIST: CardRecord = CardRecord::new_with_legacy_id(
    1877,
    "Fettergeist",
    CardArt::new("8e89ef0e-1bfe-4e12-90ee-38f993cd8110", "Izzy"),
    CardSet::AvacynRestored,
    // The tax counts other creatures, so a lone Fettergeist is free and each
    // body added beside it costs another mana every upkeep.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Spirit"], 3, 4).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "At the beginning of your upkeep, sacrifice this creature unless you pay {1} for \
             each other creature you control.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::PayOr(PayOrDef {
                payment: EffectPaymentDef::generic_mana(
                    PlayerSetDef::Related(PlayerRelation::You),
                    ValueDef::CountMatchingObjects(&FETTERGEIST_OTHERS),
                ),
                if_paid: None,
                otherwise: Some(&FETTERGEIST_SACRIFICE),
                visibility: ChoiceVisibilityDef::Public,
            }),
        ),
    ]),
);

// AVR 53 — Fleeting Distraction
pub(in crate::card::sets) static FLEETING_DISTRACTION: CardRecord = CardRecord::new_with_legacy_id(
    771,
    "Fleeting Distraction",
    CardArt::new("1ba49d16-e3e4-470a-8ca2-a93a5b358f6e", "Ryan Yee"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets -1/-0 until end of turn. Draw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(0),
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

/// Granted to each creature separately, so each pays its own {2}{U} and
/// untaps only itself.
static GALVANIC_ALCHEMIST_GRANTED: AbilityDef = AbilityDef::activated(
    "{2}{U}: Untap this creature.",
    &[AbilityCostDef::Mana(mana_cost!("{2}{U}"))],
    EffectDef::Untap {
        object: EffectRecipientDef::Source,
    },
);

static GALVANIC_ALCHEMIST_BONUS: EffectDef = EffectDef::StaticApply {
    recipient: SOULBOND_PAIR_RECIPIENT,
    effect: AppliedEffectDef::add_ability(&GALVANIC_ALCHEMIST_GRANTED),
};

// AVR 54 — Galvanic Alchemist
pub(in crate::card::sets) static GALVANIC_ALCHEMIST: CardRecord = CardRecord::new_with_legacy_id(
    1944,
    "Galvanic Alchemist",
    CardArt::new("b0e24d65-0e6f-4978-8de1-c5e4acac12fb", "Svetlin Velinov"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Wizard"], 1, 4).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, each of those creatures \
             has \"{2}{U}: Untap this creature.\"",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &GALVANIC_ALCHEMIST_BONUS,
            },
        ),
    ]),
);

// AVR 55 — Geist Snatch
pub(in crate::card::sets) static GEIST_SNATCH: CardRecord = CardRecord::new_with_legacy_id(
    772,
    "Geist Snatch",
    CardArt::new("b6dac5db-ef96-4bd5-aabc-e5ae2b95c8c3", "Dan Murayama Scott"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{2}{U}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target creature spell. Create a 1/1 blue Spirit creature token with flying.",
        &[AbilityTargetDef::exactly_one_spell(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Graveyard,
            },
            EffectDef::create_creature_token(&["Spirit"], &[ManaColor::Blue], 1, 1)
                .with_abilities(&[abilities::flying()])
                .with_art(CardArt::new(
                    "44c14591-f807-40cf-9c00-4c94b85fff44",
                    "Dan Murayama Scott",
                )),
        ]),
    )),
);

// AVR 56 — Ghostform
pub(in crate::card::sets) static GHOSTFORM: CardRecord = CardRecord::new_with_legacy_id(
    773,
    "Ghostform",
    CardArt::new("1f6a20ba-6691-4844-9685-dfcd4184224e", "Scott Chou"),
    CardSet::AvacynRestored,
    CardRules::new_sorcery(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Up to two target creatures can't be blocked this turn.",
        &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
            2,
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeBlockedBy(
                ObjectPredicateDef::Any,
            )),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// AVR 57 — Ghostly Flicker
// Audit: blocked — Linked exile returns stolen permanents under their owners rather than the spell controller, and the effect must preserve that controller for both targets.

// AVR 58 — Ghostly Touch
// Audit: blocked — Needs a tap-or-untap choice inside the triggered ability granted by an Aura.

// AVR 59 — Gryff Vanguard
pub(in crate::card::sets) static GRYFF_VANGUARD: CardRecord = CardRecord::new_with_legacy_id(
    774,
    "Gryff Vanguard",
    CardArt::new("b7238136-c8de-4949-9b54-ff75094e0569", "Jason Chan"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Human", "Knight"], 3, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "When this creature enters, draw a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// AVR 60 — Havengul Skaab
// Audit: blocked — Needs a resolving non-target choice of another creature you control to return to its owner's hand.

// AVR 61 — Infinite Reflection
// Audit: blocked — Needs attachment-derived copy effects for existing creatures and an entry replacement that copies the currently enchanted creature.

// AVR 62 — Into the Void
pub(in crate::card::sets) static INTO_THE_VOID: CardRecord = CardRecord::new_with_legacy_id(
    775,
    "Into the Void",
    CardArt::new("5ddd1050-8abd-4dfe-9e52-5b56af358653", "Daarken"),
    CardSet::AvacynRestored,
    CardRules::new_sorcery(mana_cost!("{3}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return up to two target creatures to their owners' hands.",
        &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
            2,
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
    )),
);

// AVR 63 — Latch Seeker
pub(in crate::card::sets) static LATCH_SEEKER: CardRecord = CardRecord::new_with_legacy_id(
    776,
    "Latch Seeker",
    CardArt::new("3e4e7589-9cee-4d57-8648-ce733781bfb2", "Vincent Proce"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Spirit"], 3, 1).with_ability(
        AbilityDef::static_ability(
            "This creature can't be blocked.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeBlockedBy(
                    ObjectPredicateDef::Any,
                )),
            },
        ),
    ),
);

// AVR 64 — Lone Revenant
// Audit: blocked — Needs ordered bottom-of-library placement for the unchosen cards after the conditional top-four selection.

// AVR 65 — Lunar Mystic
pub(in crate::card::sets) static LUNAR_MYSTIC: CardRecord = CardRecord::new_with_legacy_id(
    777,
    "Lunar Mystic",
    CardArt::new("f346d236-528c-4164-9995-74cdc56597a9", "Wesley Burt"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Human", "Wizard"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever you cast an instant spell, you may pay {1}. If you do, draw a card.",
            TriggerEventDef::SpellCast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Instant),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::PayOr(PayOrDef::optional(
                EffectPaymentDef::mana(
                    PlayerSetDef::Related(PlayerRelation::You),
                    mana_cost!("{1}"),
                ),
                &EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            )),
        ),
    ),
);

static MASS_APPEAL_HUMANS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::Subtype("Human"),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

// AVR 66 — Mass Appeal
pub(in crate::card::sets) static MASS_APPEAL: CardRecord = CardRecord::new_with_legacy_id(
    778,
    "Mass Appeal",
    CardArt::new(
        "dfe9ae51-fd2b-45ca-a780-725f51f897b2",
        "Christopher Moeller",
    ),
    CardSet::AvacynRestored,
    CardRules::new_sorcery(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell(
        "Draw a card for each Human you control.",
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::CountMatchingObjects(&MASS_APPEAL_HUMANS),
        },
    )),
);

// AVR 67 — Mist Raven
pub(in crate::card::sets) static MIST_RAVEN: CardRecord = CardRecord::new_with_legacy_id(
    779,
    "Mist Raven",
    CardArt::new("0d98f0c4-021a-407a-8b0c-5500d804f959", "John Avon"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Bird"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "When this creature enters, return target creature to its owner's hand.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
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

// AVR 68 — Misthollow Griffin
// Audit: blocked — Needs a cast permission and play-option source zone for casting this card from exile.

/// Exile and return in one resolution, and the return names your control
/// rather than the card's owner. The two differ exactly when the creature
/// was stolen, which is when a blink is worth the mana.
static BLINK_UNDER_YOUR_CONTROL: [EffectDef; 2] = [
    EffectDef::ExileLinkedToSource {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::ReturnLinkedExiles {
        object: ObjectPredicateDef::Any,
        counters: None,
        arrival_effect: None,
        zone: ZoneKind::Battlefield,
        grant: None,
        controller: Some(PlayerRelation::You),
        transformed: false,
    },
];

// AVR 69 — Nephalia Smuggler
pub(in crate::card::sets) static NEPHALIA_SMUGGLER: CardRecord = CardRecord::new_with_legacy_id(
    2000,
    "Nephalia Smuggler",
    CardArt::new("1a531b2f-2a9e-4cc9-aea6-9dce239f5511", "Matt Stewart"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{U}"), &["Human", "Rogue"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{3}{U}, {T}: Exile another target creature you control, then return that card to the battlefield under your control.",
            &[
                AbilityCostDef::Mana(mana_cost!("{3}{U}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&BLINK_UNDER_YOUR_CONTROL),
        ),
    ),
);

// AVR 70 — Outwit
// Audit: blocked — Needs a stack-object predicate for a spell that currently targets a player.

// AVR 71 — Peel from Reality
pub(in crate::card::sets) static PEEL_FROM_REALITY: CardRecord = CardRecord::new_with_legacy_id(
    780,
    "Peel from Reality",
    CardArt::new("7f41285b-5961-4653-96a0-fb6d27111390", "Jason Felix"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(
        AbilityDef::spell_with_targets(
            "Return target creature you control and target creature you don't control to their owners' hands.",
            &[
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                }),
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                }),
            ],
            EffectDef::Sequence(&[
                EffectDef::MoveToZone {
                    counters: None,
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    arrival_effect: None,
                    attachment: None,
                    controller: None,
                },
                EffectDef::MoveToZone {
                    counters: None,
                    object: EffectRecipientDef::Target(TargetIndex(1)),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    arrival_effect: None,
                    attachment: None,
                    controller: None,
                },
            ]),
        ),
    ),
);

// AVR 72 — Rotcrown Ghoul
pub(in crate::card::sets) static ROTCROWN_GHOUL: CardRecord = CardRecord::new_with_legacy_id(
    781,
    "Rotcrown Ghoul",
    CardArt::new("f13b5ba6-0de1-4f5c-867b-57e2c10bde8e", "Dave Kendall"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Zombie"], 3, 3).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature dies, target player mills five cards.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(5),
                binding: None,
                then: None,
            },
        ),
    ),
);

/// "This creature can block only creatures with flying."
static BLOCKS_ONLY_FLYERS: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::Source,
    effect: AppliedEffectDef::Rule(AppliedRuleDef::CanBlockOnly(
        ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
    )),
};

// AVR 73 — Scrapskin Drake
pub(in crate::card::sets) static SCRAPSKIN_DRAKE: CardRecord = CardRecord::new_with_legacy_id(
    1599,
    "Scrapskin Drake",
    CardArt::new("c9f03bae-1d23-43ea-9079-4b09d61bbadd", "Kev Walker"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Zombie", "Drake"], 2, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "This creature can block only creatures with flying.",
            BLOCKS_ONLY_FLYERS,
        ),
    ]),
);

// AVR 74 — Second Guess
// Audit: blocked — Needs a target predicate or casting-history relation for the second spell cast during the current turn.

// AVR 75 — Spectral Prison
// Audit: blocked — Needs an event for the enchanted creature becoming the target of a spell and an attachment-derived event subject.

// AVR 76 — Spirit Away
// Audit: blocked — Needs an attachment-scoped continuous control-changing effect.

static STERN_MENTOR_GRANTED: AbilityDef = AbilityDef::activated_with_targets(
    "{T}: Target player mills two cards.",
    &[AbilityCostDef::TapSource],
    &[AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::Player(PlayerRelation::Any),
    )],
    EffectDef::Mill {
        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: ValueDef::Constant(2),
        binding: None,
        then: None,
    },
);

static STERN_MENTOR_BONUS: EffectDef = EffectDef::StaticApply {
    recipient: SOULBOND_PAIR_RECIPIENT,
    effect: AppliedEffectDef::add_ability(&STERN_MENTOR_GRANTED),
};

// AVR 77 — Stern Mentor
pub(in crate::card::sets) static STERN_MENTOR: CardRecord = CardRecord::new_with_legacy_id(
    1945,
    "Stern Mentor",
    CardArt::new("ffe4d34f-68f0-4d79-9aab-58c5304224d9", "Igor Kieryluk"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Human", "Wizard"], 2, 2).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, each of those creatures \
             has \"{T}: Target player mills two cards.\"",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &STERN_MENTOR_BONUS,
            },
        ),
    ]),
);

// AVR 78 — Stolen Goods
// Audit: blocked — Needs repeat-until library exile plus temporary permission to cast the resulting card without paying its mana cost.

// AVR 79 — Tamiyo, the Moon Sage
// Audit: blocked — Needs next-untap-step duration, a tapped-creature count, maximum-hand-size modification, and graveyard-entry triggers from every zone.

/// Damage of any kind to an opponent, not only combat damage, and granted to
/// each creature so either connecting draws.
static TANDEM_LOOKOUT_GRANTED: AbilityDef = AbilityDef::triggered(
    "Whenever this creature deals damage to an opponent, draw a card.",
    TriggerEventDef::damage_to_player(ObjectPredicateDef::Source, PlayerRelation::Opponent),
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
);

static TANDEM_LOOKOUT_BONUS: EffectDef = EffectDef::StaticApply {
    recipient: SOULBOND_PAIR_RECIPIENT,
    effect: AppliedEffectDef::add_ability(&TANDEM_LOOKOUT_GRANTED),
};

// AVR 80 — Tandem Lookout
pub(in crate::card::sets) static TANDEM_LOOKOUT: CardRecord = CardRecord::new_with_legacy_id(
    1946,
    "Tandem Lookout",
    CardArt::new("83564e67-2677-4955-a3b9-3b221dbb100b", "Kev Walker"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Scout"], 2, 1).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as Tandem Lookout is paired with another creature, each of those creatures \
             has \"Whenever this creature deals damage to an opponent, draw a card.\"",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &TANDEM_LOOKOUT_BONUS,
            },
        ),
    ]),
);

// AVR 81 — Temporal Mastery
pub(in crate::card::sets) static TEMPORAL_MASTERY: CardRecord = CardRecord::new_with_legacy_id(
    1693,
    "Temporal Mastery",
    CardArt::new("266e5267-2288-4bb0-8c54-0c556521cec3", "Franz Vohwinkel"),
    CardSet::AvacynRestored,
    CardRules::new_sorcery(mana_cost!("{5}{U}{U}")).with_abilities(&[
        AbilityDef::spell(
            "Take an extra turn after this one. Exile Temporal Mastery.",
            EffectDef::TakeExtraTurn {
                player: EffectRecipientDef::Controller,
            },
        )
        .with_resolution_destination(crate::SpellResolutionDestinationDef::Exile),
        abilities::miracle(mana_cost!("{1}{U}")),
    ]),
);

// AVR 82 — Vanishment
pub(in crate::card::sets) static VANISHMENT: CardRecord = CardRecord::new_with_legacy_id(
    782,
    "Vanishment",
    CardArt::new("dece40c1-790c-4471-a790-1d356b345603", "Daarken"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{4}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Put target nonland permanent on top of its owner's library.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
            )],
            EffectDef::MoveToZone {
                counters: None,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Library,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
                controller: None,
            },
        ),
        abilities::miracle(mana_cost!("{U}")),
    ]),
);

static WINGCRAFTER_GRANTED: AbilityDef = abilities::flying();

static WINGCRAFTER_BONUS: EffectDef = EffectDef::StaticApply {
    recipient: SOULBOND_PAIR_RECIPIENT,
    effect: AppliedEffectDef::add_ability(&WINGCRAFTER_GRANTED),
};

// AVR 83 — Wingcrafter
pub(in crate::card::sets) static WINGCRAFTER: CardRecord = CardRecord::new_with_legacy_id(
    1935,
    "Wingcrafter",
    CardArt::new("04a3059f-92f2-4163-b79a-154118a4e36d", "Matt Stewart"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{U}"), &["Human", "Wizard"], 1, 1).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, both creatures have \\
             flying.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &WINGCRAFTER_BONUS,
            },
        ),
    ]),
);

// AVR 84 — Appetite for Brains
// Audit: blocked — Needs a hidden-hand card choice constrained by mana value, followed by exile of the chosen card.

// AVR 85 — Barter in Blood
// Audit: blocked — Needs each player to make one resolving choice of two creatures, or a continuation across two sacrifice choices.

// AVR 86 — Blood Artist
pub(in crate::card::sets) static BLOOD_ARTIST: CardRecord = CardRecord::new_with_legacy_id(
    783,
    "Blood Artist",
    CardArt::new("2e1fb442-68ff-4249-8e44-87edf6fae211", "Johannes Voss"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Vampire"], 0, 1).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever this creature or another creature dies, target player loses 1 life and you gain 1 life.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::HasType(CardType::Creature), Some(ZoneKind::Battlefield), Some(ZoneKind::Graveyard)),
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                PlayerRelation::Any,
            ))],
            EffectDef::Sequence(&[
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ),
);

// AVR 87 — Bloodflow Connoisseur
pub(in crate::card::sets) static BLOODFLOW_CONNOISSEUR: CardRecord = CardRecord::new_with_legacy_id(
    784,
    "Bloodflow Connoisseur",
    CardArt::new("97485dbf-2f31-4ed2-a6cd-529ca22c9ac5", "Slawomir Maniak"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Vampire"], 1, 1).with_ability(
        AbilityDef::activated(
            "Sacrifice a creature: Put a +1/+1 counter on this creature.",
            &[AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::You,
            }],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

static SACRIFICE_A_CREATURE: SpellAdditionalCostDef = SpellAdditionalCostDef {
    object: ObjectPredicateDef::HasType(CardType::Creature),
    zone: ZoneKind::Battlefield,
    count: 1,
    counted: SpellAdditionalCostCountDef::Printed,
    spend: SpendModeDef::ByZone,
    or: None,
};

// AVR 88 — Bone Splinters
pub(in crate::card::sets) static BONE_SPLINTERS: CardRecord = CardRecord::new_with_legacy_id(
    1962,
    "Bone Splinters",
    CardArt::new("387eda28-f35b-48b0-ba59-773d82902327", "Nils Hamm"),
    CardSet::AvacynRestored,
    // The sacrifice is paid on the way to the stack, so the creature it eats
    // is gone before the target is destroyed -- and the spell can eat the
    // very creature it is aimed at only if something else is left to target.
    CardRules::new_sorcery(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_additional_cost(
        "As an additional cost to cast this spell, sacrifice a creature.\nDestroy target \
             creature.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        SACRIFICE_A_CREATURE,
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            can_regenerate: true,
        },
    )),
);

// AVR 89 — Butcher Ghoul
pub(in crate::card::sets) static BUTCHER_GHOUL: CardRecord = CardRecord::new_with_legacy_id(
    785,
    "Butcher Ghoul",
    CardArt::new(
        "44a91e62-e946-4101-8cef-d1c147caebf2",
        "Christopher Moeller",
    ),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Zombie"], 1, 1)
        .with_ability(abilities::undying()),
);

// AVR 90 — Corpse Traders
// Audit: blocked — Needs a hidden-hand card choice and an activation timing restriction of sorcery speed.

// AVR 91 — Crypt Creeper
pub(in crate::card::sets) static CRYPT_CREEPER: CardRecord = CardRecord::new_with_legacy_id(
    786,
    "Crypt Creeper",
    CardArt::new("0382cb94-0836-4e23-99b7-034faa363203", "Scott Chou"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Zombie"], 2, 1).with_ability(
        AbilityDef::activated_with_targets(
            "Sacrifice this creature: Exile target card from a graveyard.",
            &[AbilityCostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::MoveToZone {
                counters: None,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
                controller: None,
            },
        ),
    ),
);

// AVR 92 — Dark Impostor
// Audit: blocked — Needs the source to acquire every activated ability of the creature cards it exiled.

// AVR 93 — Death Wind
pub(in crate::card::sets) static DEATH_WIND: CardRecord = CardRecord::new_with_legacy_id(
    787,
    "Death Wind",
    CardArt::new("462a0961-cca5-4d63-867f-7426dbef8639", "Tomasz Jedruszek"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{X}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets -X/-X until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Negate(&ValueDef::ChosenX),
                ValueDef::Negate(&ValueDef::ChosenX),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

/// The printed intervening-if condition is checked both as the end step begins
/// and again when the trigger resolves.
static EXACTLY_ONE_CREATURE: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef::matching(
        ObjectPredicateDef::HasType(CardType::Creature),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    ),
    comparison: ComparisonDef::Equal,
    amount: 1,
};

// AVR 94 — Demonic Rising
pub(in crate::card::sets) static DEMONIC_RISING: CardRecord = CardRecord::new_with_legacy_id(
    151,
    "Demonic Rising",
    CardArt::new("a2136a82-b535-47f6-9eee-5b7585ac5cf1", "Trevor Claxton"),
    CardSet::AvacynRestored,
    CardRules::new_enchantment(mana_cost!("{3}{B}{B}")).with_ability(
        AbilityDef::triggered_if(
            "At the beginning of your end step, if you control exactly one creature, create a 5/5 black Demon creature token with flying.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            &EXACTLY_ONE_CREATURE,
            EffectDef::create_creature_token(&["Demon"], &[ManaColor::Black], 5, 5).with_abilities(&[abilities::flying()]).with_art(CardArt::new("6a3fc83f-ab02-4a44-910a-bfadc71cf162", "Kev Walker")),
        ),
    ),
);

// AVR 95 — Demonic Taskmaster
pub(in crate::card::sets) static DEMONIC_TASKMASTER: CardRecord = CardRecord::new_with_legacy_id(
    788,
    "Demonic Taskmaster",
    CardArt::new("fb5d6266-30a7-4360-84bc-22b52fb782b3", "Chris Rahn"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Demon"], 4, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "At the beginning of your upkeep, sacrifice a creature other than this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::SacrificeOfChoice {
                player: EffectRecipientDef::Controller,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                then: None,
                amount: SacrificedAmountDef::Power,
                otherwise: None,
                optional: false,
            },
        ),
    ]),
);

// AVR 96 — Demonlord of Ashmouth
// Audit: blocked — Needs an enters-the-battlefield sacrifice choice whose no-sacrifice branch exiles the source.

// AVR 97 — Descent into Madness
// Audit: blocked — Needs each player to choose a counter-derived number of permanents and/or hand cards to exile in one resolving choice.

/// "In addition to its other colors and types", so both leaves add rather
/// than set. It travels with the move because the permanent that arrives is
/// a new object: nothing following the move would still name it.
static A_BLACK_ZOMBIE_AS_WELL: AppliedEffectDef = AppliedEffectDef::Composite(&[
    AppliedEffectDef::add_colors(ColorSet::from_colors(&[ManaColor::Black])),
    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Zombie"])),
]);

// AVR 98 — Dread Slaver
pub(in crate::card::sets) static DREAD_SLAVER: CardRecord = CardRecord::new_with_legacy_id(
    2004,
    "Dread Slaver",
    CardArt::new("3d8a3abd-a4a2-48e6-b709-1c0240a76c5e", "Dave Kendall"),
    CardSet::AvacynRestored,
    // It keeps whatever it kills, so blocking it is worse than taking five.
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Zombie", "Horror"], 3, 5).with_ability(
        AbilityDef::triggered(
            "Whenever a creature dealt damage by this creature this turn dies, return it to the battlefield under your control. That creature is a black Zombie in addition to its other colors and types.",
            TriggerEventDef::ZoneChanged(
                ZoneChangeEventMatcherDef::new(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                )
                .previously_damaged_by(ObjectRefDef::Source),
            ),
            EffectDef::MoveToZone {
                counters: None,
                object: EffectRecipientDef::TriggeringObject,
                zone: ZoneKind::Battlefield,
                controller: Some(PlayerRelation::You),
                placement: ZonePlacement::Top,
                arrival_effect: Some(&A_BLACK_ZOMBIE_AS_WELL),
                attachment: None,
            },
        ),
    ),
);

// AVR 99 — Driver of the Dead
pub(in crate::card::sets) static DRIVER_OF_THE_DEAD: CardRecord = CardRecord::new_with_legacy_id(
    789,
    "Driver of the Dead",
    CardArt::new("56113cde-4210-46be-bd53-8966c36ef2a3", "James Ryman"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Vampire"], 3, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature dies, return target creature card with mana value 2 or less from your graveyard to the battlefield.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, Some(ZoneKind::Battlefield), Some(ZoneKind::Graveyard)),
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ManaValueAtMost(2),
                ]),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            })],
            EffectDef::MoveToZone {
                counters: None,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
                controller: None,
            },
        ),
    ),
);

/// The largest power among your creatures, which is one creature's size
/// rather than a count of them.
static GREATEST_POWER_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

/// One X read once and spent twice: the drain moves exactly what it takes.
static ESSENCE_HARVEST_DRAIN: [EffectDef; 2] = [
    EffectDef::LoseLife {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: ValueDef::GreatestPowerAmong(&GREATEST_POWER_YOU_CONTROL),
    },
    EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::GreatestPowerAmong(&GREATEST_POWER_YOU_CONTROL),
    },
];

// AVR 100 — Essence Harvest
pub(in crate::card::sets) static ESSENCE_HARVEST: CardRecord = CardRecord::new_with_legacy_id(
    1978,
    "Essence Harvest",
    CardArt::new("7c3fac03-a019-4faa-bc1c-09e3a394fff7", "Daarken"),
    CardSet::AvacynRestored,
    CardRules::new_sorcery(mana_cost!("{2}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target player loses X life and you gain X life, where X is the greatest power among creatures you control.",
        &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
            PlayerRelation::Any,
        ))],
        EffectDef::Sequence(&ESSENCE_HARVEST_DRAIN),
    )),
);

// AVR 101 — Evernight Shade
pub(in crate::card::sets) static EVERNIGHT_SHADE: CardRecord = CardRecord::new_with_legacy_id(
    790,
    "Evernight Shade",
    CardArt::new("1091fadf-97c4-4f87-8466-6a1246a72226", "Nic Klein"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Shade"], 1, 1).with_abilities(&[
        AbilityDef::activated(
            "{B}: This creature gets +1/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::undying(),
    ]),
);

// AVR 102 — Exquisite Blood
// Audit: blocked — Needs a committed life-loss event that also captures nondamage life loss and its amount.

// AVR 103 — Ghoulflesh
// Audit: blocked — The power/toughness modifier is available, but adding black and Zombie to the enchanted creature's characteristics is not.

// AVR 104 — Gloom Surgeon
// Audit: blocked — Needs a combat-damage replacement that prevents the event and exiles exactly that many cards from the top of your library.

// AVR 105 — Grave Exchange
// Audit: blocked — Its sacrifice choice would be nested after another zone move, and the current resolving-decision continuation cannot resume that sequence.

// AVR 106 — Griselbrand
pub(in crate::card::sets) static GRISELBRAND: CardRecord = CardRecord::new_with_legacy_id(
    791,
    "Griselbrand",
    CardArt::new("b51666ae-2aef-4cb1-9cd4-44aec81530f8", "Igor Kieryluk"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{4}{B}{B}{B}{B}"), &["Demon"], 7, 7)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            abilities::lifelink(),
            AbilityDef::activated(
                "Pay 7 life: Draw seven cards.",
                &[AbilityCostDef::PayLife(7)],
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(7),
                },
            ),
        ]),
);

/// "Another nontoken creature", so a token dying is not a card and the Demon
/// stays quiet; the exclusion of itself is the other half.
static HARVESTER_OF_SOULS_DEATH: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
]);

// AVR 107 — Harvester of Souls
pub(in crate::card::sets) static HARVESTER_OF_SOULS: CardRecord = CardRecord::new_with_legacy_id(
    1891,
    "Harvester of Souls",
    CardArt::new("505c0d25-dc1f-402e-9183-01c273efe0e1", "Slawomir Maniak"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Demon"], 5, 5).with_abilities(&[
        abilities::deathtouch(),
        AbilityDef::triggered(
            "Whenever another nontoken creature dies, you may draw a card.",
            TriggerEventDef::zone_changed(
                HARVESTER_OF_SOULS_DEATH,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            },
        ),
    ]),
);

// AVR 108 — Homicidal Seclusion
// Audit: blocked — Needs an exactly-one-creature condition that controls both the affected recipient and a lifelink grant in the static ability layer.

// AVR 109 — Human Frailty
pub(in crate::card::sets) static HUMAN_FRAILTY: CardRecord = CardRecord::new_with_legacy_id(
    792,
    "Human Frailty",
    CardArt::new("1d1de712-86ac-4c03-be86-2403cd121f66", "David Palumbo"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::destroy_target(
        "Destroy target Human creature.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Subtype("Human"),
        ])),
        true,
    )),
);

static ANY_CREATURE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Creature),
)];

// AVR 110 — Hunted Ghoul
pub(in crate::card::sets) static HUNTED_GHOUL: CardRecord = CardRecord::new_with_legacy_id(
    1742,
    "Hunted Ghoul",
    CardArt::new("644509fa-559b-4b84-a67b-ba59797df2ed", "Ryan Pancoast"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{B}"), &["Zombie"], 1, 2).with_ability(
        AbilityDef::static_ability(
            "This creature can't block Humans.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CanBlockOnly(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Human")),
                )),
            },
        ),
    ),
);

// AVR 111 — Killing Wave
// Audit: blocked — Needs a separate pay-X-life-or-sacrifice choice for the controller of every creature.

// AVR 112 — Maalfeld Twins
pub(in crate::card::sets) static MAALFELD_TWINS: CardRecord = CardRecord::new_with_legacy_id(
    793,
    "Maalfeld Twins",
    CardArt::new("c63dd203-bce9-4ab7-8a0c-059d19d384e9", "Mike Sass"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{5}{B}"), &["Zombie"], 4, 4).with_ability(
        AbilityDef::triggered(
            "When this creature dies, create two 2/2 black Zombie creature tokens.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::create_creature_token(&["Zombie"], &[ManaColor::Black], 2, 2)
                .with_art(CardArt::new(
                    "b877c19d-6022-4377-92e7-4511e24eb98e",
                    "Lucas Graciano",
                ))
                .with_amount(2),
        ),
    ),
);

// AVR 113 — Marrow Bats
pub(in crate::card::sets) static MARROW_BATS: CardRecord = CardRecord::new_with_legacy_id(
    1433,
    "Marrow Bats",
    CardArt::new("38dcbad0-267e-411f-8e99-5d90b537bf9b", "Jason A. Engle"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Bat", "Skeleton"], 4, 1).with_abilities(&[
        abilities::flying(),
        abilities::regenerate_self(
            "Pay 4 life: Regenerate this creature.",
            &[AbilityCostDef::PayLife(4)],
        ),
    ]),
);

// AVR 114 — Mental Agony
// Audit: blocked — Needs a continuation that waits for the targeted player's discard choice before applying the printed life loss.

// AVR 115 — Necrobite
pub(in crate::card::sets) static NECROBITE: CardRecord = CardRecord::new_with_legacy_id(
    1434,
    "Necrobite",
    CardArt::new("52e59918-cf12-4d73-a4e0-31f38e792dc4", "Nils Hamm"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{2}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gains deathtouch until end of turn. Regenerate it.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&NECROBITE_EFFECTS),
    )),
);

static NECROBITE_EFFECTS: [EffectDef; 2] = [
    EffectDef::Apply {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        effect: AppliedEffectDef::add_ability(&abilities::deathtouch()),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
    EffectDef::Regenerate {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
];

// AVR 116 — Polluted Dead
pub(in crate::card::sets) static POLLUTED_DEAD: CardRecord = CardRecord::new_with_legacy_id(
    794,
    "Polluted Dead",
    CardArt::new("036c1954-37d3-4787-8df8-f2d0dd39058a", "Jason A. Engle"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Zombie"], 3, 3).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature dies, destroy target land.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Land),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
        ),
    ),
);

// AVR 117 — Predator's Gambit
// Audit: blocked — Needs a no-other-creatures condition that controls a static intimidate grant to the enchanted creature.

// AVR 118 — Renegade Demon
pub(in crate::card::sets) static RENEGADE_DEMON: CardRecord = CardRecord::new_with_legacy_id(
    795,
    "Renegade Demon",
    CardArt::new("395696f8-9be2-4925-852f-b783850e1ca2", "Tomasz Jedruszek"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Demon"], 5, 3),
);

// AVR 119 — Searchlight Geist
pub(in crate::card::sets) static SEARCHLIGHT_GEIST: CardRecord = CardRecord::new_with_legacy_id(
    796,
    "Searchlight Geist",
    CardArt::new("b0dc1a94-0193-464e-a481-730b34b57db5", "Steven Belledin"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Spirit"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{3}{B}: This creature gains deathtouch until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{3}{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::deathtouch()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// AVR 120 — Soulcage Fiend
pub(in crate::card::sets) static SOULCAGE_FIEND: CardRecord = CardRecord::new_with_legacy_id(
    797,
    "Soulcage Fiend",
    CardArt::new("dce1b1d3-9602-42bf-b341-d96976ff1e60", "Jason A. Engle"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Demon"], 3, 2).with_ability(
        AbilityDef::triggered(
            "When this creature dies, each player loses 3 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::EachPlayer,
                amount: ValueDef::Constant(3),
            },
        ),
    ),
);

// AVR 121 — Treacherous Pit-Dweller
// Audit: blocked — Needs a graveyard-to-battlefield trigger and a permanent control change to a targeted opponent.

/// "If you control the creature with the greatest power or tied for the
/// greatest power." A tie counts, so this asks whether anything is strictly
/// bigger rather than whether one creature stands alone.
static CONTROLS_THE_BIGGEST: TriggerConditionDef =
    TriggerConditionDef::ControlsGreatestPowerCreature;

// AVR 122 — Triumph of Cruelty
pub(in crate::card::sets) static TRIUMPH_OF_CRUELTY: CardRecord = CardRecord::new_with_legacy_id(
    1613,
    "Triumph of Cruelty",
    CardArt::new("906618e2-2638-4017-9d6e-e6f282967a81", "Izzy"),
    CardSet::AvacynRestored,
    CardRules::new_enchantment(mana_cost!("{2}{B}")).with_ability(
        AbilityDef::triggered_if_with_targets(
            "At the beginning of your upkeep, target opponent discards a card if you control \
             the creature with the greatest power or tied for the greatest power.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &CONTROLS_THE_BIGGEST,
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::Discard {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ),
    ),
);

// AVR 123 — Undead Executioner
pub(in crate::card::sets) static UNDEAD_EXECUTIONER: CardRecord = CardRecord::new_with_legacy_id(
    798,
    "Undead Executioner",
    CardArt::new("8d330058-16af-4486-aa89-b6be759e35d4", "Dave Kendall"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Zombie"], 2, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature dies, you may have target creature get -2/-2 until end of turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-2),
                        ValueDef::Constant(-2),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            },
        ),
    ),
);

// AVR 124 — Unhallowed Pact
// Audit: blocked — Needs a zone-change trigger whose subject is the permanent currently attached to this Aura.

// AVR 125 — Aggravate
// Audit: blocked — Needs to grant the attack requirement only to creatures actually dealt damage after prevention and replacement effects.

// AVR 126 — Archwing Dragon
pub(in crate::card::sets) static ARCHWING_DRAGON: CardRecord = CardRecord::new_with_legacy_id(
    799,
    "Archwing Dragon",
    CardArt::new("6c6f1a8b-329e-4094-8141-6bc88311a08c", "Daarken"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Dragon"], 4, 4).with_abilities(&[
        abilities::flying(),
        abilities::haste(),
        AbilityDef::triggered(
            "At the beginning of the end step, return this creature to its owner's hand.",
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
        ),
    ]),
);

// AVR 127 — Banners Raised
pub(in crate::card::sets) static BANNERS_RAISED: CardRecord = CardRecord::new_with_legacy_id(
    800,
    "Banners Raised",
    CardArt::new("a7792df3-e2ab-4e60-abee-f24b72807107", "Mike Bierek"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::spell(
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

static BATTLE_HYMN_CREATURES: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

// AVR 128 — Battle Hymn
pub(in crate::card::sets) static BATTLE_HYMN: CardRecord = CardRecord::new_with_legacy_id(
    801,
    "Battle Hymn",
    CardArt::new("43b5d46e-7054-44f8-9a14-b412f2f0ab86", "Nils Hamm"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell(
        "Add {R} for each creature you control.",
        EffectDef::AddManaEqualTo {
            color: ManaColor::Red,
            amount: ValueDef::CountMatchingObjects(&BATTLE_HYMN_CREATURES),
        },
    )),
);

// AVR 129 — Bonfire of the Damned
pub(in crate::card::sets) static BONFIRE_OF_THE_DAMNED: CardRecord = CardRecord::new_with_legacy_id(
    143,
    "Bonfire of the Damned",
    CardArt::new("e60610fe-891d-46de-b556-d03b637dccec", "James Paick"),
    CardSet::AvacynRestored,
    CardRules::new_sorcery(mana_cost!("{X}{X}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Bonfire of the Damned deals X damage to target player or planeswalker and each creature that player or that planeswalker's controller controls.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )],
            EffectDef::Sequence(&[
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::ChosenX,
                },
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::objects_controlled_by_target(ObjectPredicateDef::HasType(CardType::Creature), TargetIndex::PRIMARY),
                    amount: ValueDef::ChosenX,
                },
            ]),
        ),
        abilities::miracle(mana_cost!("{X}{R}")),
    ]),
);

// AVR 130 — Burn at the Stake
// Audit: blocked — Needs a spell additional cost that taps any number of chosen untapped creatures and retains that count for a three-times damage value.

// AVR 131 — Dangerous Wager
// Audit: blocked — Needs a dynamic whole-hand discard amount before the draw.

// AVR 132 — Demolish
pub(in crate::card::sets) static DEMOLISH: CardRecord = CardRecord::new_with_legacy_id(
    802,
    "Demolish",
    CardArt::new("4657aa15-8274-4bd7-afe4-504693064373", "Raymond Swanland"),
    CardSet::AvacynRestored,
    CardRules::new_sorcery(mana_cost!("{3}{R}")).with_ability(AbilityDef::destroy_target(
        "Destroy target artifact or land.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Artifact),
            ObjectPredicateDef::HasType(CardType::Land),
        ])),
        true,
    )),
);

// AVR 133 — Dual Casting
// Audit: blocked — Needs a stack-spell copy effect with optional new targets.

// AVR 134 — Falkenrath Exterminator
pub(in crate::card::sets) static FALKENRATH_EXTERMINATOR: CardRecord = CardRecord::new_with_legacy_id(
    803,
    "Falkenrath Exterminator",
    CardArt::new("40e23909-7e08-4686-ae59-e18e7d4cfd3c", "Winona Nelson"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Vampire", "Archer"], 1, 1).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, put a +1/+1 counter on it.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_with_targets(
            "{2}{R}: This creature deals damage to target creature equal to the number of +1/+1 counters on this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{R}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::CountersOnSource(CounterKind::PlusOnePlusOne),
            },
        ),
    ]),
);

// AVR 135 — Fervent Cathar
pub(in crate::card::sets) static FERVENT_CATHAR: CardRecord = CardRecord::new_with_legacy_id(
    1743,
    "Fervent Cathar",
    CardArt::new("39715fa1-595f-4e3d-84a3-35f2636bccc7", "Steven Belledin"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Knight"], 2, 1).with_abilities(&[
        abilities::haste(),
        AbilityDef::triggered_with_targets(
            "When this creature enters, target creature can't block this turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &ANY_CREATURE_TARGET,
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBlock),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// AVR 136 — Gang of Devils
pub(in crate::card::sets) static GANG_OF_DEVILS: CardRecord = CardRecord::new_with_legacy_id(
    804,
    "Gang of Devils",
    CardArt::new("0430b9fa-3bc6-4183-ad5b-d70ad401fa97", "Erica Yang"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{5}{R}"), &["Devil"], 3, 3).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature dies, it deals 3 damage divided as you choose among one, two, or three targets.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, Some(ZoneKind::Battlefield), Some(ZoneKind::Graveyard)),
            &[AbilityTargetDef {
                predicate: AbilityTargetPredicate::AnyTarget,
                minimum: 1,
                maximum: 3,
                divided_total: Some(DividedTotal::Fixed(3)),
                another: false,
            }],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::DividedAmongTargets,
            },
        ),
    ),
);

// AVR 137 — Guise of Fire
pub(in crate::card::sets) static GUISE_OF_FIRE: CardRecord = CardRecord::new_with_legacy_id(
    805,
    "Guise of Fire",
    CardArt::new("beb10d42-fa19-400c-bad8-ec3827f077bc", "Dave Kendall"),
    CardSet::AvacynRestored,
    CardRules::new_enchantment(mana_cost!("{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/-1 and attacks each combat if able.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(-1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::attacks_each_combat_if_able(
                            "This creature attacks each combat if able.",
                        )),
                    ]),
                },
            ),
        ]),
);

static HANWEIR_LANCER_GRANTED: AbilityDef = abilities::first_strike();

static HANWEIR_LANCER_BONUS: EffectDef = EffectDef::StaticApply {
    recipient: SOULBOND_PAIR_RECIPIENT,
    effect: AppliedEffectDef::add_ability(&HANWEIR_LANCER_GRANTED),
};

// AVR 138 — Hanweir Lancer
pub(in crate::card::sets) static HANWEIR_LANCER: CardRecord = CardRecord::new_with_legacy_id(
    1936,
    "Hanweir Lancer",
    CardArt::new("73884fd1-5be9-4ad5-8c72-c0fbe27ad4c1", "Steve Prescott"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Knight"], 2, 2).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, both creatures have \\
             first strike.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &HANWEIR_LANCER_BONUS,
            },
        ),
    ]),
);

// AVR 139 — Havengul Vampire
pub(in crate::card::sets) static HAVENGUL_VAMPIRE: CardRecord = CardRecord::new_with_legacy_id(
    806,
    "Havengul Vampire",
    CardArt::new("cbc09839-1463-40b8-86bd-fb96797b2633", "James Ryman"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Vampire"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, put a +1/+1 counter on it.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::triggered(
            "Whenever another creature dies, put a +1/+1 counter on this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// AVR 140 — Heirs of Stromkirk
pub(in crate::card::sets) static HEIRS_OF_STROMKIRK: CardRecord = CardRecord::new_with_legacy_id(
    807,
    "Heirs of Stromkirk",
    CardArt::new("ff89ad3b-b154-49e2-a0fd-135279512250", "Winona Nelson"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Vampire"], 2, 2).with_abilities(&[
        abilities::intimidate(),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, put a +1/+1 counter on it.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// AVR 141 — Hound of Griselbrand
pub(in crate::card::sets) static HOUND_OF_GRISELBRAND: CardRecord = CardRecord::new_with_legacy_id(
    808,
    "Hound of Griselbrand",
    CardArt::new("0fe68bce-6207-4fd1-9e82-a18fd2d6ddca", "Svetlin Velinov"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Elemental", "Dog"], 2, 2)
        .with_abilities(&[abilities::double_strike(), abilities::undying()]),
);

static KESSIG_MALCONTENTS_HUMANS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::Subtype("Human"),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

// AVR 142 — Kessig Malcontents
pub(in crate::card::sets) static KESSIG_MALCONTENTS: CardRecord = CardRecord::new_with_legacy_id(
    809,
    "Kessig Malcontents",
    CardArt::new("dce9a30f-a850-4826-a255-ce511d567b60", "John Stanko"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Warrior"], 3, 1).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature enters, it deals damage to target player or planeswalker equal to the number of Humans you control.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::CountMatchingObjects(&KESSIG_MALCONTENTS_HUMANS),
            },
        ),
    ),
);

// AVR 143 — Kruin Striker
pub(in crate::card::sets) static KRUIN_STRIKER: CardRecord = CardRecord::new_with_legacy_id(
    810,
    "Kruin Striker",
    CardArt::new("73e72249-84ea-4e9c-9f64-b67b02ffdf3a", "Christopher Moeller"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Warrior"], 2, 1).with_ability(
        AbilityDef::triggered(
            "Whenever another creature you control enters, this creature gets +1/+0 and gains trample until end of turn.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]), None, Some(ZoneKind::Battlefield)),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(0)),
                    AppliedEffectDef::add_ability(&abilities::trample()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

static LIGHTNING_MAULER_GRANTED: AbilityDef = abilities::haste();

static LIGHTNING_MAULER_BONUS: EffectDef = EffectDef::StaticApply {
    recipient: SOULBOND_PAIR_RECIPIENT,
    effect: AppliedEffectDef::add_ability(&LIGHTNING_MAULER_GRANTED),
};

// AVR 144 — Lightning Mauler
pub(in crate::card::sets) static LIGHTNING_MAULER: CardRecord = CardRecord::new_with_legacy_id(
    1937,
    "Lightning Mauler",
    CardArt::new("241cc968-b93e-4fe3-a66d-7776d29aa023", "Dave Kendall"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Berserker"], 2, 1).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, both creatures have \\
             haste.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &LIGHTNING_MAULER_BONUS,
            },
        ),
    ]),
);

static LIGHTNING_PROWESS_PING: AbilityDef = AbilityDef::activated_with_targets(
    "{T}: This creature deals 1 damage to any target.",
    &[AbilityCostDef::TapSource],
    &[AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::AnyTarget,
    )],
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: ValueDef::Constant(1),
    },
);

// AVR 145 — Lightning Prowess
pub(in crate::card::sets) static LIGHTNING_PROWESS: CardRecord = CardRecord::new_with_legacy_id(
    811,
    "Lightning Prowess",
    CardArt::new("5578e3e2-2460-4dfb-9016-527463f2d918", "David Rapoza"),
    CardSet::AvacynRestored,
    CardRules::new_enchantment(mana_cost!("{2}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature has haste and \"{T}: This creature deals 1 damage to any target.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_ability(&abilities::haste()),
                        AppliedEffectDef::add_ability(&LIGHTNING_PROWESS_PING),
                    ]),
                },
            ),
        ]),
);

// AVR 146 — Mad Prophet
pub(in crate::card::sets) static MAD_PROPHET: CardRecord = CardRecord::new_with_legacy_id(
    1960,
    "Mad Prophet",
    CardArt::new("172383d9-9135-4daa-a647-9d76435d3158", "Wayne Reynolds"),
    CardSet::AvacynRestored,
    // Haste, so the looting starts the turn it lands.
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Human", "Shaman"], 2, 2).with_abilities(&[
        abilities::haste(),
        AbilityDef::activated(
            "{T}, Discard a card: Draw a card.",
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::DiscardCardMatching(ObjectPredicateDef::Any),
            ],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

/// Granted to the host rather than kept on the Aura, so the tap cost is the
/// creature's own and "this turn" is measured from wherever it resolves.
static MALICIOUS_INTENT_ABILITY: AbilityDef = AbilityDef::activated_with_targets(
    "{T}: Target creature can't block this turn.",
    &[AbilityCostDef::TapSource],
    &ANY_CREATURE_TARGET,
    EffectDef::Apply {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBlock),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
);

// AVR 147 — Malicious Intent
pub(in crate::card::sets) static MALICIOUS_INTENT: CardRecord = CardRecord::new_with_legacy_id(
    1744,
    "Malicious Intent",
    CardArt::new("79f4d244-2aaf-4780-ba65-798b090338b4", "Kev Walker"),
    CardSet::AvacynRestored,
    CardRules::new_enchantment(mana_cost!("{1}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature has \"{T}: Target creature can't block this turn.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&MALICIOUS_INTENT_ABILITY),
                },
            ),
        ]),
);

// AVR 148 — Malignus
// Audit: blocked — Needs a characteristic-defining half-highest-opponent-life value and a damage-prevention prohibition for the source.

// AVR 149 — Pillar of Flame
pub(in crate::card::sets) static PILLAR_OF_FLAME: CardRecord = CardRecord::new_with_legacy_id(
    195,
    "Pillar of Flame",
    CardArt::new("c983e879-d9d2-47cc-9958-506711ca80cd", "Karl Kopinski"),
    CardSet::AvacynRestored,
    CardRules::new_sorcery(mana_cost!("{R}")).with_ability(
        AbilityDef::custom_full(
            "Pillar of Flame deals 2 damage to any target. If a creature dealt damage this way would die this turn, exile it instead.",
            CardBehavior::PillarOfFlame,
            "Implemented by the named card-local special behavior.",
        ),
    ),
);

// AVR 150 — Raging Poltergeist
pub(in crate::card::sets) static RAGING_POLTERGEIST: CardRecord = CardRecord::new_with_legacy_id(
    812,
    "Raging Poltergeist",
    CardArt::new("78833788-ffb2-43fc-9345-975f1cd46f38", "Slawomir Maniak"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Spirit"], 6, 1),
);

// AVR 151 — Reforge the Soul
// Audit: blocked — Needs a dynamic whole-hand discard for every player before the seven-card draw.

// AVR 152 — Riot Ringleader
pub(in crate::card::sets) static RIOT_RINGLEADER: CardRecord = CardRecord::new_with_legacy_id(
    813,
    "Riot Ringleader",
    CardArt::new("c043f30b-548f-4c31-a415-0e59c2841dcf", "Gabor Szikszai"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Warrior"], 2, 2)
        .with_ability(AbilityDef::triggered(
        "Whenever this creature attacks, Human creatures you control get +1/+0 until end of turn.",
        TriggerEventDef::attacks(ObjectPredicateDef::Source),
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Subtype("Human"),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(0)),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// AVR 153 — Rite of Ruin
// Audit: blocked — Needs a chosen ordering of three permanent types and six sequential per-player sacrifice choices with retained mode order.

// AVR 154 — Rush of Blood
pub(in crate::card::sets) static RUSH_OF_BLOOD: CardRecord = CardRecord::new_with_legacy_id(
    814,
    "Rush of Blood",
    CardArt::new("a2884824-d138-47f2-913b-32cd475e9584", "Cynthia Sheppard"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{2}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +X/+0 until end of turn, where X is its power.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::TargetPower(TargetIndex::PRIMARY),
                ValueDef::Constant(0),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// AVR 155 — Scalding Devil
pub(in crate::card::sets) static SCALDING_DEVIL: CardRecord = CardRecord::new_with_legacy_id(
    815,
    "Scalding Devil",
    CardArt::new("bbe49a97-dac8-4273-b4dc-45cdf8f5a6e0", "Erica Yang"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Devil"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{2}{R}: This creature deals 1 damage to target player or planeswalker.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{R}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// AVR 156 — Somberwald Vigilante
pub(in crate::card::sets) static SOMBERWALD_VIGILANTE: CardRecord = CardRecord::new_with_legacy_id(
    1753,
    "Somberwald Vigilante",
    CardArt::new("0479b796-a4f8-4001-ad16-705cabcdcef8", "John Stanko"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{R}"), &["Human", "Warrior"], 1, 1).with_ability(
        AbilityDef::triggered(
            "Whenever this creature becomes blocked by a creature, this creature deals 1 damage \
             to that creature.",
            TriggerEventDef::BecomesBlockedBy {
                blocker: ObjectPredicateDef::HasType(CardType::Creature),
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::TriggeringObject,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

static STONEWRIGHT_GRANTED: AbilityDef = AbilityDef::activated(
    "{R}: This creature gets +1/+0 until end of turn.",
    &[AbilityCostDef::Mana(mana_cost!("{R}"))],
    EffectDef::Apply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::modify_power_toughness(
            ValueDef::Constant(1),
            ValueDef::Constant(0),
        ),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
);

static STONEWRIGHT_BONUS: EffectDef = EffectDef::StaticApply {
    recipient: SOULBOND_PAIR_RECIPIENT,
    effect: AppliedEffectDef::add_ability(&STONEWRIGHT_GRANTED),
};

// AVR 157 — Stonewright
pub(in crate::card::sets) static STONEWRIGHT: CardRecord = CardRecord::new_with_legacy_id(
    1947,
    "Stonewright",
    CardArt::new("9564d79d-5f4d-4192-94ee-5e5998011266", "Wesley Burt"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{R}"), &["Human", "Shaman"], 1, 1).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as Stonewright is paired with another creature, each of those creatures has \
             \"{R}: This creature gets +1/+0 until end of turn.\"",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &STONEWRIGHT_BONUS,
            },
        ),
    ]),
);

// AVR 158 — Thatcher Revolt
// Audit: blocked — Needs identity links from the three created tokens to the delayed sacrifice so it does not sacrifice unrelated Human tokens.

// AVR 159 — Thunderbolt
pub(in crate::card::sets) static THUNDERBOLT: CardRecord = CardRecord::new_with_legacy_id(
    1640,
    "Thunderbolt",
    CardArt::new("5845a5bc-6b7d-4bbb-80b3-a0f877b95553", "Anthony Francisco"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(AbilityDef::modal_spell(
        "Choose one —\n• Thunderbolt deals 3 damage to target player or planeswalker.\n• Thunderbolt deals 4 damage to target creature with flying.",
        &[
            AbilityDef::spell_with_targets(
                "Thunderbolt deals 3 damage to target player or planeswalker",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
                )],
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(3),
                },
            ),
            AbilityDef::spell_with_targets(
                "Thunderbolt deals 4 damage to target creature with flying",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                    ]),
                )],
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(4),
                },
            ),
        ],
        1,
        1,
        false,
    )),
);

// AVR 160 — Thunderous Wrath
pub(in crate::card::sets) static THUNDEROUS_WRATH: CardRecord = CardRecord::new_with_legacy_id(
    816,
    "Thunderous Wrath",
    CardArt::new("daa39826-7f89-41cb-a7fe-7f7be817d5cd", "Adam Paquette"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{4}{R}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Thunderous Wrath deals 5 damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(5),
            },
        ),
        abilities::miracle(mana_cost!("{R}")),
    ]),
);

// AVR 161 — Tibalt, the Fiend-Blooded
// Audit: blocked — Needs a target player's hand-size value; its other loyalty effects do not make the whole planeswalker exact without that value.

// AVR 162 — Tyrant of Discord
// Audit: blocked — Needs an opponent's random permanent choice and repeat-until-land sacrifice loop.

// AVR 163 — Uncanny Speed
pub(in crate::card::sets) static UNCANNY_SPEED: CardRecord = CardRecord::new_with_legacy_id(
    817,
    "Uncanny Speed",
    CardArt::new("1d7b747e-446a-4c25-9834-0be8476dc22d", "Raymond Swanland"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +3/+0 and gains haste until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(0),
                ),
                AppliedEffectDef::add_ability(&abilities::haste()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// AVR 164 — Vexing Devil
// Audit: blocked — Needs an opponent choice on resolution, including which opponent in multiplayer, with a sacrifice branch only when one accepts the damage.

// AVR 165 — Vigilante Justice
pub(in crate::card::sets) static VIGILANTE_JUSTICE: CardRecord = CardRecord::new_with_legacy_id(
    818,
    "Vigilante Justice",
    CardArt::new("a9db329b-6248-4082-bfc8-5d2c0db43338", "Steve Prescott"),
    CardSet::AvacynRestored,
    CardRules::new_enchantment(mana_cost!("{3}{R}")).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever a Human you control enters, this enchantment deals 1 damage to any target.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype("Human"),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
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

/// Haste matters here because the permanent has not been under its new
/// controller's control since the turn began.
static HASTE_GRANT: AbilityDef = abilities::haste();

// AVR 166 — Zealous Conscripts
pub(in crate::card::sets) static ZEALOUS_CONSCRIPTS: CardRecord = CardRecord::new_with_legacy_id(
    244,
    "Zealous Conscripts",
    CardArt::new("fc027b11-1ecc-430d-a862-586a14bb23c3", "Steve Prescott"),
    CardSet::AvacynRestored,
    CardRules::new_creature(
        mana_cost!("{4}{R}"),
        &["Human", "Warrior"],
        3,
        3,
    )
    .with_abilities(&[
        abilities::haste(),
        AbilityDef::triggered_with_targets("When this creature enters, gain control of target permanent until end of turn. Untap that permanent. It gains haste until end of turn.", TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)), &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::Any,
        )], // Control first: the untap and the haste are worth having only
            // on a permanent that is already yours to use.
            EffectDef::Sequence(&[
                EffectDef::GainControl {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    duration: ControlDurationDef::UntilEndOfTurn,
                    controller: PlayerRefDef::EffectController,
                },
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&HASTE_GRANT),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ])),
    ]),
);

static ABUNDANT_GROWTH_MANA: AbilityDef = AbilityDef::activated_mana(
    "{T}: Add one mana of any color.",
    &[AbilityCostDef::TapSource],
    EffectDef::AddMana(AddManaEffectDef::any_color()),
);

// AVR 167 — Abundant Growth
pub(in crate::card::sets) static ABUNDANT_GROWTH: CardRecord = CardRecord::new_with_legacy_id(
    819,
    "Abundant Growth",
    CardArt::new("afbc8fd0-dc15-4ac9-b97b-173f7fb66ed7", "Vincent Proce"),
    CardSet::AvacynRestored,
    CardRules::new_enchantment(mana_cost!("{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant land",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Land),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::triggered(
                "When this Aura enters, draw a card.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted land has \"{T}: Add one mana of any color.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&ABUNDANT_GROWTH_MANA),
                },
            ),
        ]),
);

// AVR 168 — Blessings of Nature
// Audit: blocked — Divided target shares are currently implemented only for damage; using them for counters resolves every counter amount as zero.

// AVR 169 — Borderland Ranger
pub(in crate::card::sets) static BORDERLAND_RANGER: CardRecord = CardRecord::new_with_legacy_id(
    820,
    "Borderland Ranger",
    CardArt::new("8f067c26-c51d-44d0-a0af-106b5778f06a", "Zoltan Boros"),
    CardSet::AvacynRestored,
    CardRules::new_creature(
        mana_cost!("{2}{G}"),
        &["Human", "Scout", "Ranger"],
        2,
        2,
    )
    .with_ability(AbilityDef::triggered(
        "When this creature enters, you may search your library for a basic land card, reveal it, put it into your hand, then shuffle.",
        TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)),
        EffectDef::May {
            player: EffectRecipientDef::Controller,
            effect: &EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Supertype(CardSupertype::Basic),
                ]),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: true,
                destination: ZoneKind::Hand,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: false,
                binding: None,
                then: None,
            },
        },
    )),
);

// AVR 170 — Bower Passage
pub(in crate::card::sets) static BOWER_PASSAGE: CardRecord = CardRecord::new_with_legacy_id(
    1641,
    "Bower Passage",
    CardArt::new("b9f0048f-aaa0-4597-b898-ee754d0bbe4b", "Cliff Childs"),
    CardSet::AvacynRestored,
    CardRules::new_enchantment(mana_cost!("{1}{G}")).with_ability(AbilityDef::static_ability(
        "Creatures with flying can't block creatures you control.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeBlockedBy(
                ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
            )),
        },
    )),
);

// AVR 171 — Champion of Lambholt
// Audit: blocked — Needs a blocking predicate that dynamically compares each prospective blocker's power with this creature's current power.

static CRATERHOOF_CREATURES: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

// AVR 172 — Craterhoof Behemoth
pub(in crate::card::sets) static CRATERHOOF_BEHEMOTH: CardRecord = CardRecord::new_with_legacy_id(
    821,
    "Craterhoof Behemoth",
    CardArt::new("a249be17-73ed-4108-89c0-f7e87939beb8", "Chris Rahn"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{5}{G}{G}{G}"), &["Beast"], 5, 5).with_abilities(&[
        abilities::haste(),
        AbilityDef::triggered(
            "When this creature enters, creatures you control gain trample and get +X/+X until end of turn, where X is the number of creatures you control.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)),
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::You),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(ValueDef::CountMatchingObjects(&CRATERHOOF_CREATURES), ValueDef::CountMatchingObjects(&CRATERHOOF_CREATURES)),
                    AppliedEffectDef::add_ability(&abilities::trample()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// AVR 173 — Descendants' Path
// Audit: blocked — Needs a top-card reveal, shared-creature-type test, free-cast permission, and bottom placement when the card is not cast.

static DIREGRAF_ESCORT_GRANTED: AbilityDef = abilities::protection_from_creature_type(
    "Protection from Zombies",
    ProtectedCreatureType::Zombie,
);

static DIREGRAF_ESCORT_BONUS: EffectDef = EffectDef::StaticApply {
    recipient: SOULBOND_PAIR_RECIPIENT,
    effect: AppliedEffectDef::add_ability(&DIREGRAF_ESCORT_GRANTED),
};

// AVR 174 — Diregraf Escort
pub(in crate::card::sets) static DIREGRAF_ESCORT: CardRecord = CardRecord::new_with_legacy_id(
    1948,
    "Diregraf Escort",
    CardArt::new("640e21ad-5064-41bc-886e-2c997f69a3f5", "Ryan Pancoast"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{G}"), &["Human", "Cleric"], 1, 1).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, both creatures have \
             protection from Zombies.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &DIREGRAF_ESCORT_BONUS,
            },
        ),
    ]),
);

static DRUIDS_FAMILIAR_BONUS: EffectDef = EffectDef::StaticApply {
    recipient: SOULBOND_PAIR_RECIPIENT,
    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
};

// AVR 175 — Druid's Familiar
pub(in crate::card::sets) static DRUIDS_FAMILIAR: CardRecord = CardRecord::new_with_legacy_id(
    1938,
    "Druid's Familiar",
    CardArt::new("1d8c794a-7964-41f0-bc9c-27af7cf87aaa", "Adam Paquette"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Bear"], 2, 2).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, each of those creatures \\
             gets +2/+2.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &DRUIDS_FAMILIAR_BONUS,
            },
        ),
    ]),
);

// AVR 176 — Druids' Repository
pub(in crate::card::sets) static DRUIDS_REPOSITORY: CardRecord = CardRecord::new_with_legacy_id(
    822,
    "Druids' Repository",
    CardArt::new("57e6fb62-7ee3-444d-8fd4-c1f44014a05c", "Daarken"),
    CardSet::AvacynRestored,
    CardRules::new_enchantment(mana_cost!("{1}{G}{G}")).with_abilities(&[
        AbilityDef::triggered(
            "Whenever a creature you control attacks, put a charge counter on this enchantment.",
            TriggerEventDef::attacks(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::Charge,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "Remove a charge counter from this enchantment: Add one mana of any color.",
            &[AbilityCostDef::RemoveCountersFromSource {
                kind: CounterKind::Charge,
                amount: 1,
            }],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// AVR 177 — Eaten by Spiders
// Audit: blocked — Needs an attachment relation that finds and destroys every Equipment attached to the targeted creature.

/// Only a creature with soulbond can start a pairing, so every pair contains
/// one -- which makes "paired at all" and "paired with a soulbond creature"
/// the same question for a Treefolk that has no soulbond of its own.
static FLOWERING_LUMBERKNOT_UNPAIRED: TriggerConditionDef = TriggerConditionDef::SourceMatches {
    object: ObjectPredicateDef::Unpaired,
};

static FLOWERING_LUMBERKNOT_SIDELINED: [AppliedEffectDef; 2] = [
    AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
    AppliedEffectDef::Rule(AppliedRuleDef::CannotBlock),
];

static FLOWERING_LUMBERKNOT_RESTRICTION: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::Source,
    effect: AppliedEffectDef::Composite(&FLOWERING_LUMBERKNOT_SIDELINED),
};

// AVR 178 — Flowering Lumberknot
pub(in crate::card::sets) static FLOWERING_LUMBERKNOT: CardRecord = CardRecord::new_with_legacy_id(
    1949,
    "Flowering Lumberknot",
    CardArt::new("78fa2ddc-142b-4562-8812-ecb72e3bae57", "Nic Klein"),
    CardSet::AvacynRestored,
    // A 5/5 for four that does nothing on its own, so the restriction is the
    // card rather than a footnote on it.
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Treefolk"], 5, 5).with_ability(
        AbilityDef::static_ability(
            "This creature can't attack or block unless it's paired with a creature with \
             soulbond.",
            EffectDef::IfCondition {
                condition: &FLOWERING_LUMBERKNOT_UNPAIRED,
                then: &FLOWERING_LUMBERKNOT_RESTRICTION,
            },
        ),
    ),
);

static GEIST_TRAPPERS_GRANTED: AbilityDef = abilities::reach();

static GEIST_TRAPPERS_BONUS: EffectDef = EffectDef::StaticApply {
    recipient: SOULBOND_PAIR_RECIPIENT,
    effect: AppliedEffectDef::add_ability(&GEIST_TRAPPERS_GRANTED),
};

// AVR 179 — Geist Trappers
pub(in crate::card::sets) static GEIST_TRAPPERS: CardRecord = CardRecord::new_with_legacy_id(
    1939,
    "Geist Trappers",
    CardArt::new("26f00a18-0ff7-42e4-be16-aa34fe27093b", "Anthony Palumbo"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Human", "Warrior"], 3, 5).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, both creatures have \\
             reach.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &GEIST_TRAPPERS_BONUS,
            },
        ),
    ]),
);

// AVR 180 — Gloomwidow
pub(in crate::card::sets) static GLOOMWIDOW: CardRecord = CardRecord::new_with_legacy_id(
    1600,
    "Gloomwidow",
    CardArt::new("a016c872-09bd-42e1-94da-f587e8252492", "Svetlin Velinov"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Spider"], 3, 3).with_abilities(&[
        abilities::reach(),
        AbilityDef::static_ability(
            "This creature can block only creatures with flying.",
            BLOCKS_ONLY_FLYERS,
        ),
    ]),
);

// AVR 181 — Grounded
pub(in crate::card::sets) static GROUNDED: CardRecord = CardRecord::new_with_legacy_id(
    823,
    "Grounded",
    CardArt::new("dc4982f0-0ede-4846-82c8-bcf7ad63d099", "Greg Staples"),
    CardSet::AvacynRestored,
    CardRules::new_enchantment(mana_cost!("{1}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature loses flying.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::remove_abilities(
                        crate::card::AbilityPredicateDef::Keyword(KeywordAbility::Flying),
                    ),
                },
            ),
        ]),
);

/// "Creatures with power less than this creature's power can't block it."
/// The comparison is against the source's current power, so pumping it widens
/// the restriction.
static WEAKER_THAN_SOURCE: ObjectPredicateDef =
    ObjectPredicateDef::PowerLessThan(ValueDef::SourcePower);

// AVR 182 — Howlgeist
pub(in crate::card::sets) static HOWLGEIST: CardRecord = CardRecord::new_with_legacy_id(
    1596,
    "Howlgeist",
    CardArt::new("dad60d45-1c99-41d1-a237-c0ee18ce5361", "David Rapoza"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{5}{G}"), &["Spirit", "Wolf"], 4, 2).with_abilities(&[
        AbilityDef::static_ability(
            "Creatures with power less than this creature's power can't block it.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeBlockedBy(
                    WEAKER_THAN_SOURCE,
                )),
            },
        ),
        abilities::undying(),
    ]),
);

// AVR 183 — Joint Assault
// Audit: blocked — Needs soulbond pairing state and the identity of the creature paired with the target.

// AVR 184 — Lair Delve
// Audit: blocked — Needs a mandatory characteristic-filtered top-two split and player-chosen ordering for the cards put on the bottom.

// AVR 185 — Natural End
pub(in crate::card::sets) static NATURAL_END: CardRecord = CardRecord::new_with_legacy_id(
    824,
    "Natural End",
    CardArt::new("95d25235-de1c-4b67-9712-24f0564bd2bf", "Scott Chou"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{2}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target artifact or enchantment. You gain 3 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Enchantment),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ]),
    )),
);

// AVR 186 — Nettle Swine
pub(in crate::card::sets) static NETTLE_SWINE: CardRecord = CardRecord::new_with_legacy_id(
    825,
    "Nettle Swine",
    CardArt::new(
        "75935f0e-9086-485b-b3e6-1a958fd0f2af",
        "Christopher Moeller",
    ),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Boar"], 4, 3),
);

static NIGHTSHADE_PEDDLER_GRANTED: AbilityDef = abilities::deathtouch();

static NIGHTSHADE_PEDDLER_BONUS: EffectDef = EffectDef::StaticApply {
    recipient: SOULBOND_PAIR_RECIPIENT,
    effect: AppliedEffectDef::add_ability(&NIGHTSHADE_PEDDLER_GRANTED),
};

// AVR 187 — Nightshade Peddler
pub(in crate::card::sets) static NIGHTSHADE_PEDDLER: CardRecord = CardRecord::new_with_legacy_id(
    1940,
    "Nightshade Peddler",
    CardArt::new("4d3de66c-2283-458f-9d0d-943027520aa2", "John Stanko"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Druid"], 1, 1).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, both creatures have \\
             deathtouch.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &NIGHTSHADE_PEDDLER_BONUS,
            },
        ),
    ]),
);

static PATHBREAKER_WURM_GRANTED: AbilityDef = abilities::trample();

static PATHBREAKER_WURM_BONUS: EffectDef = EffectDef::StaticApply {
    recipient: SOULBOND_PAIR_RECIPIENT,
    effect: AppliedEffectDef::add_ability(&PATHBREAKER_WURM_GRANTED),
};

// AVR 188 — Pathbreaker Wurm
pub(in crate::card::sets) static PATHBREAKER_WURM: CardRecord = CardRecord::new_with_legacy_id(
    1941,
    "Pathbreaker Wurm",
    CardArt::new("fe65eded-37ef-4cf0-b55c-390d34aab7b8", "Nils Hamm"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Wurm"], 6, 4).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, both creatures have \\
             trample.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &PATHBREAKER_WURM_BONUS,
            },
        ),
    ]),
);

// AVR 189 — Primal Surge
// Audit: blocked — Needs a repeatable top-card exile procedure with a permanent-card branch and a new optional decision on every iteration.

// AVR 190 — Rain of Thorns
pub(in crate::card::sets) static RAIN_OF_THORNS: CardRecord = CardRecord::new_with_legacy_id(
    826,
    "Rain of Thorns",
    CardArt::new("fd1cb530-b9d5-4386-b89e-2acecc8294c8", "Sam Burley"),
    CardSet::AvacynRestored,
    CardRules::new_sorcery(mana_cost!("{4}{G}{G}")).with_ability(AbilityDef::modal_spell(
        "Choose one or more —\n• Destroy target artifact.\n• Destroy target enchantment.\n• Destroy target land.",
        &[
            AbilityDef::spell_with_targets(
                "Destroy target artifact",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                },
            ),
            AbilityDef::spell_with_targets(
                "Destroy target enchantment",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                },
            ),
            AbilityDef::spell_with_targets(
                "Destroy target land",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Land),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                },
            ),
        ],
        1,
        3,
        false,
    )),
);

// AVR 191 — Revenge of the Hunted
// Audit: blocked — Needs a turn-scoped requirement that every creature able to block the target does so.

/// The grant and the life are one resolution, and the life is read from the
/// same slot the hexproof went to.
static SHELTERING_WORD_PROGRAM: [EffectDef; 2] = [
    EffectDef::Apply {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
    EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::TargetToughness(TargetIndex::PRIMARY),
    },
];

// AVR 192 — Sheltering Word
pub(in crate::card::sets) static SHELTERING_WORD: CardRecord = CardRecord::new_with_legacy_id(
    1973,
    "Sheltering Word",
    CardArt::new("93cd9be4-1ce4-4a7c-b2a6-98d3fde0a92b", "Igor Kieryluk"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature you control gains hexproof until end of turn. You gain life equal to that creature's toughness.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
        )],
        EffectDef::Sequence(&SHELTERING_WORD_PROGRAM),
    )),
);

// AVR 193 — Snare the Skies
pub(in crate::card::sets) static SNARE_THE_SKIES: CardRecord = CardRecord::new_with_legacy_id(
    827,
    "Snare the Skies",
    CardArt::new("28f75827-a144-4fe2-a713-4439ae7567eb", "Ryan Yee"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +1/+1 and gains reach until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                AppliedEffectDef::add_ability(&abilities::reach()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

static SOMBERWALD_SAGE_RESTRICTIONS: [ManaRestrictionDef; 1] = [ManaRestrictionDef::CastSpell(
    ObjectPredicateDef::HasType(CardType::Creature),
)];

// AVR 194 — Somberwald Sage
pub(in crate::card::sets) static SOMBERWALD_SAGE: CardRecord = CardRecord::new_with_legacy_id(
    828,
    "Somberwald Sage",
    CardArt::new("409c0272-7a43-4a6c-ab3f-740397b1f5c8", "Steve Argyle"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Druid"], 0, 1).with_ability(
        AbilityDef::activated_mana(
            "{T}: Add three mana of any one color. Spend this mana only to cast creature spells.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(
                AddManaEffectDef::any_color()
                    .with_amount(3)
                    .with_restrictions(&SOMBERWALD_SAGE_RESTRICTIONS),
            ),
        ),
    ),
);

static SOUL_OF_THE_HARVEST_ENTRY: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
    ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
]);

// AVR 195 — Soul of the Harvest
pub(in crate::card::sets) static SOUL_OF_THE_HARVEST: CardRecord = CardRecord::new_with_legacy_id(
    1892,
    "Soul of the Harvest",
    CardArt::new("078f5e79-18dd-44e5-a930-8dc288f0b535", "Eytan Zana"),
    CardSet::AvacynRestored,
    // Nontoken, so a board full of tokens draws nothing -- the Soul rewards
    // creature cards rather than a token engine.
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Elemental"], 6, 6).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered(
            "Whenever another nontoken creature you control enters, you may draw a card.",
            TriggerEventDef::zone_changed(
                SOUL_OF_THE_HARVEST_ENTRY,
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            },
        ),
    ]),
);

// AVR 196 — Terrifying Presence
pub(in crate::card::sets) static TERRIFYING_PRESENCE: CardRecord = CardRecord::new_with_legacy_id(
    1500,
    "Terrifying Presence",
    CardArt::new("2e8d0a22-f31b-45c0-85c7-0101aa63c77b", "Jaime Jones"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Prevent all combat damage that would be dealt by creatures other than target creature this turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::PreventDamage {
            prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::combat_except(
                ObjectRefDef::Target(TargetIndex::PRIMARY),
            )),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// AVR 197 — Timberland Guide
pub(in crate::card::sets) static TIMBERLAND_GUIDE: CardRecord = CardRecord::new_with_legacy_id(
    829,
    "Timberland Guide",
    CardArt::new("ae80fefb-af78-4f98-8058-71b61e91842f", "Zoltan Boros"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Scout"], 1, 1).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature enters, put a +1/+1 counter on target creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// AVR 198 — Triumph of Ferocity
pub(in crate::card::sets) static TRIUMPH_OF_FEROCITY: CardRecord = CardRecord::new_with_legacy_id(
    1614,
    "Triumph of Ferocity",
    CardArt::new("7bb41fa6-0cc6-43e5-9aa8-fcd9c781f4ce", "James Ryman"),
    CardSet::AvacynRestored,
    CardRules::new_enchantment(mana_cost!("{2}{G}")).with_ability(AbilityDef::triggered_if(
        "At the beginning of your upkeep, draw a card if you control the creature with the \
         greatest power or tied for the greatest power.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::You,
        },
        &CONTROLS_THE_BIGGEST,
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )),
);

static SOULBOND_ABILITIES: [AbilityDef; 2] = abilities::soulbond();

/// "Both creatures" and "each of those creatures" name the same pair: the
/// soulbond creature and whatever it is bonded to.
static SOULBOND_PAIR_RECIPIENT: EffectRecipientDef = EffectRecipientDef::matching_objects(
    ObjectPredicateDef::AnyOf(&[
        ObjectPredicateDef::Source,
        ObjectPredicateDef::PairedWithSource,
    ]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

static TRUSTED_FORCEMAGE_BONUS: EffectDef = EffectDef::StaticApply {
    recipient: SOULBOND_PAIR_RECIPIENT,
    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
};

// AVR 199 — Trusted Forcemage
pub(in crate::card::sets) static TRUSTED_FORCEMAGE: CardRecord = CardRecord::new_with_legacy_id(
    1931,
    "Trusted Forcemage",
    CardArt::new("3ee66ef9-10a7-4aab-88f7-84956811cc6c", "Cynthia Sheppard"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Shaman"], 2, 2).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, each of those creatures \
             gets +1/+1.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &TRUSTED_FORCEMAGE_BONUS,
            },
        ),
    ]),
);

// AVR 200 — Ulvenwald Tracker
// Audit: blocked — Needs the simultaneous fight damage procedure and its two-creature target relation.

// AVR 201 — Vorstclaw
pub(in crate::card::sets) static VORSTCLAW: CardRecord = CardRecord::new_with_legacy_id(
    830,
    "Vorstclaw",
    CardArt::new("7591ee4f-9bfe-4419-84df-abf35d85bb94", "Lucas Graciano"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Elemental", "Horror"], 7, 7),
);

// AVR 202 — Wandering Wolf
pub(in crate::card::sets) static WANDERING_WOLF: CardRecord = CardRecord::new_with_legacy_id(
    1597,
    "Wandering Wolf",
    CardArt::new("ac606ad5-b8d0-4c93-a9de-5e41229a8229", "Tomasz Jedruszek"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Wolf"], 2, 1).with_abilities(&[
        AbilityDef::static_ability(
            "Creatures with power less than this creature's power can't block it.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeBlockedBy(
                    WEAKER_THAN_SOURCE,
                )),
            },
        ),
    ]),
);

// AVR 203 — Wild Defiance
// Audit: blocked — Needs an event for a creature becoming the target of an instant or sorcery spell, carrying that creature as the effect recipient.

// AVR 204 — Wildwood Geist
// Audit: blocked — Needs an active-player condition usable by a continuous power/toughness effect.

// AVR 205 — Wolfir Avenger
pub(in crate::card::sets) static WOLFIR_AVENGER: CardRecord = CardRecord::new_with_legacy_id(
    1435,
    "Wolfir Avenger",
    CardArt::new("88cc00e5-9683-4ccc-a914-c422b76f6014", "Daniel Ljunggren"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Wolf", "Warrior"], 3, 3).with_abilities(&[
        abilities::flash(),
        abilities::regenerate_self(
            "{1}{G}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{G}"))],
        ),
    ]),
);

static WOLFIR_SILVERHEART_BONUS: EffectDef = EffectDef::StaticApply {
    recipient: SOULBOND_PAIR_RECIPIENT,
    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(4), ValueDef::Constant(4)),
};

// AVR 206 — Wolfir Silverheart
pub(in crate::card::sets) static WOLFIR_SILVERHEART: CardRecord = CardRecord::new_with_legacy_id(
    1942,
    "Wolfir Silverheart",
    CardArt::new("8629c598-11c8-4911-acfb-0643e5feffa8", "Raymond Swanland"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Wolf", "Warrior"], 4, 4).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, each of those creatures \\
             gets +4/+4.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &WOLFIR_SILVERHEART_BONUS,
            },
        ),
    ]),
);

// AVR 207 — Yew Spirit
pub(in crate::card::sets) static YEW_SPIRIT: CardRecord = CardRecord::new_with_legacy_id(
    831,
    "Yew Spirit",
    CardArt::new("b9320432-4f89-4363-91e6-2e740535cc2e", "Dan Murayama Scott"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Spirit", "Treefolk"], 3, 3).with_ability(
        AbilityDef::activated(
            "{2}{G}{G}: This creature gets +X/+X until end of turn, where X is its power.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{G}{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::SourcePower,
                    ValueDef::SourcePower,
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// AVR 208 — Bruna, Light of Alabaster
// Audit: blocked — Needs a blocking trigger and resolving choices of any number of legal Auras across the battlefield, hand, and graveyard to attach to the source.

// AVR 209 — Gisela, Blade of Goldnight
// Audit: blocked — Needs global damage-event replacements that double opposing damage and prevent half of incoming damage with rounding.

// AVR 210 — Sigarda, Host of Herons
pub(in crate::card::sets) static SIGARDA_HOST_OF_HERONS: CardRecord = CardRecord::new_with_legacy_id(
    212,
    "Sigarda, Host of Herons",
    CardArt::new("feccd0e2-fae6-4ced-acdf-4252ed5c56e7", "Chris Rahn"),
    CardSet::AvacynRestored,
    CardRules::new_creature(
        mana_cost!("{2}{G}{W}{W}"),
        &["Angel"],
        5,
        5,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::flying(),
        abilities::hexproof(),
        AbilityDef::static_ability(
            "Spells and abilities your opponents control can't cause you to sacrifice permanents.",
            EffectDef::CannotBeForcedToSacrifice,
        ),
    ]),
);

// AVR 211 — Angel's Tomb
// Audit: blocked — Needs its optional creature-entry trigger authored as one end-of-turn composite characteristic effect.

static ANGELIC_ARMAMENTS_FLYING: AbilityDef = abilities::flying();

static ANGELIC_ARMAMENTS_BONUS: [AppliedEffectDef; 4] = [
    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
    AppliedEffectDef::add_ability(&ANGELIC_ARMAMENTS_FLYING),
    AppliedEffectDef::add_colors(ColorSet::from_colors(&[ManaColor::White])),
    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Angel"])),
];

// AVR 212 — Angelic Armaments
pub(in crate::card::sets) static ANGELIC_ARMAMENTS: CardRecord = CardRecord::new_with_legacy_id(
    2308,
    "Angelic Armaments",
    CardArt::new(
        "3fa99b48-469d-4112-bdfd-2391fa439514",
        "Daniel Ljunggren",
    ),
    CardSet::AvacynRestored,
    CardRules::new_artifact(mana_cost!("{3}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +2/+2, has flying, and is a white Angel in addition to its other colors and types.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&ANGELIC_ARMAMENTS_BONUS),
                },
            ),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{4}"))],
                "Equip {4} ({4}: Attach to target creature you control. Equip only as a \
                 sorcery.)",
            ),
        ]),
);

static BLADED_BRACERS_VIGILANCE: AbilityDef = abilities::vigilance();

static EQUIPPED_CREATURE_IS_HUMAN_OR_ANGEL: TriggerConditionDef =
    TriggerConditionDef::AttachedPermanentMatches {
        object: ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::Subtype("Human"),
            ObjectPredicateDef::Subtype("Angel"),
        ]),
    };

static BLADED_BRACERS_VIGILANCE_GRANT: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::AttachedPermanent,
    effect: AppliedEffectDef::add_ability(&BLADED_BRACERS_VIGILANCE),
};

// AVR 213 — Bladed Bracers
pub(in crate::card::sets) static BLADED_BRACERS: CardRecord = CardRecord::new_with_legacy_id(
    1926,
    "Bladed Bracers",
    CardArt::new("897a5116-043c-46aa-880e-be8dcb1618bc", "Ryan Yee"),
    CardSet::AvacynRestored,
    // The size is unconditional and only the vigilance reads the type, so
    // moving the Bracers onto a Zombie keeps the +1/+1.
    CardRules::new_artifact(mana_cost!("{1}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
            AbilityDef::static_ability(
                "As long as equipped creature is a Human or an Angel, it has vigilance.",
                EffectDef::IfCondition {
                    condition: &EQUIPPED_CREATURE_IS_HUMAN_OR_ANGEL,
                    then: &BLADED_BRACERS_VIGILANCE_GRANT,
                },
            ),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{2}"))],
                "Equip {2} ({2}: Attach to target creature you control. Equip only as a \
                 sorcery.)",
            ),
        ]),
);

// AVR 214 — Conjurer's Closet
pub(in crate::card::sets) static CONJURERS_CLOSET: CardRecord = CardRecord::new_with_legacy_id(
    2001,
    "Conjurer's Closet",
    CardArt::new("7378e998-0382-42fc-8606-c6e7fc04b6a4", "Jason Felix"),
    CardSet::AvacynRestored,
    // The same blink every turn, for free, which is what makes five mana
    // worth it on a board of entry triggers.
    CardRules::new_artifact(mana_cost!("{5}")).with_ability(AbilityDef::triggered_with_targets(
        "At the beginning of your end step, you may exile target creature you control, then return that card to the battlefield under your control.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::End,
            player: PlayerRelation::You,
        },
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
        )],
        EffectDef::May {
            player: EffectRecipientDef::Controller,
            effect: &EffectDef::Sequence(&BLINK_UNDER_YOUR_CONTROL),
        },
    )),
);

// AVR 215 — Gallows at Willow Hill
// Audit: blocked — Needs an activation cost that taps three separately chosen untapped Humans you control.

// AVR 216 — Haunted Guardian
pub(in crate::card::sets) static HAUNTED_GUARDIAN: CardRecord = CardRecord::new_with_legacy_id(
    832,
    "Haunted Guardian",
    CardArt::new("7d97f8b8-bdb0-4d4b-b077-9affe2f9cd91", "Daniel Ljunggren"),
    CardSet::AvacynRestored,
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Construct"], 2, 1)
        .with_abilities(&[abilities::defender(), abilities::first_strike()]),
);

static MOONSILVER_SPEAR_FIRST_STRIKE: AbilityDef = abilities::first_strike();

// AVR 217 — Moonsilver Spear
pub(in crate::card::sets) static MOONSILVER_SPEAR: CardRecord = CardRecord::new_with_legacy_id(
    2309,
    "Moonsilver Spear",
    CardArt::new("0b5efb85-1e5f-40ba-97b1-0ef6ac680330", "James Paick"),
    CardSet::AvacynRestored,
    CardRules::new_artifact(mana_cost!("{4}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature has first strike.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&MOONSILVER_SPEAR_FIRST_STRIKE),
                },
            ),
            AbilityDef::triggered(
                "Whenever equipped creature attacks, create a 4/4 white Angel creature token with flying.",
                TriggerEventDef::attacks(ObjectPredicateDef::AttachedToSource),
                EffectDef::create_creature_token(&["Angel"], &[ManaColor::White], 4, 4).with_abilities(&[abilities::flying()]).with_art(CardArt::new(
                        "68dd1682-a5d5-4323-b876-66a86c311c43",
                        "Anthony Palumbo",
                    )),
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{4}"))], "Equip {4}"),
        ]),
);

// AVR 218 — Narstad Scrapper
pub(in crate::card::sets) static NARSTAD_SCRAPPER: CardRecord = CardRecord::new_with_legacy_id(
    833,
    "Narstad Scrapper",
    CardArt::new("f808ed9b-95ac-4069-bdca-b100bc816b5b", "Steven Belledin"),
    CardSet::AvacynRestored,
    CardRules::new_artifact_creature(mana_cost!("{5}"), &["Construct"], 3, 3).with_ability(
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
    ),
);

// AVR 219 — Otherworld Atlas
pub(in crate::card::sets) static OTHERWORLD_ATLAS: CardRecord = CardRecord::new_with_legacy_id(
    834,
    "Otherworld Atlas",
    CardArt::new("46e4aa67-4643-42ff-8172-200498686494", "Sam Wolfe Connelly"),
    CardSet::AvacynRestored,
    CardRules::new_artifact(mana_cost!("{4}"))
        .with_subtypes(&["Book"])
        .with_abilities(&[
            AbilityDef::activated(
                "{T}: Put a charge counter on this artifact.",
                &[AbilityCostDef::TapSource],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::Charge,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::activated(
                "{T}: Each player draws a card for each charge counter on this artifact.",
                &[AbilityCostDef::TapSource],
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::EachPlayer,
                    amount: ValueDef::CountersOnSource(CounterKind::Charge),
                },
            ),
        ]),
);

static CONTROLS_AN_ANGEL: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef::matching(
        ObjectPredicateDef::Subtype("Angel"),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    ),
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 1,
};

// AVR 220 — Scroll of Avacyn
pub(in crate::card::sets) static SCROLL_OF_AVACYN: CardRecord = CardRecord::new_with_legacy_id(
    835,
    "Scroll of Avacyn",
    CardArt::new("871e6e2a-7e45-446b-b964-94377eb6ca92", "Cliff Childs"),
    CardSet::AvacynRestored,
    CardRules::new_artifact(mana_cost!("{1}")).with_ability(AbilityDef::activated(
        "{1}, Sacrifice this artifact: Draw a card. If you control an Angel, you gain 5 life.",
        &[
            AbilityCostDef::Mana(mana_cost!("{1}")),
            AbilityCostDef::SacrificeSource,
        ],
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
            EffectDef::IfCondition {
                condition: &CONTROLS_AN_ANGEL,
                then: &EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(5),
                },
            },
        ]),
    )),
);

// AVR 221 — Scroll of Griselbrand
// Audit: blocked — Needs a continuation that waits for the opponent's discard choice before checking for a Demon and applying the printed life loss.

static TORMENTORS_TRIDENT_REQUIREMENT: AbilityDef =
    abilities::attacks_each_combat_if_able("Attacks each combat if able");

static TORMENTORS_TRIDENT_BONUS: [AppliedEffectDef; 2] = [
    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(3), ValueDef::Constant(0)),
    AppliedEffectDef::add_ability(&TORMENTORS_TRIDENT_REQUIREMENT),
];

// AVR 222 — Tormentor's Trident
pub(in crate::card::sets) static TORMENTORS_TRIDENT: CardRecord = CardRecord::new_with_legacy_id(
    1928,
    "Tormentor's Trident",
    CardArt::new("9543d454-27d6-42ba-aad8-54811d180cfb", "Anthony Palumbo"),
    CardSet::AvacynRestored,
    // The requirement travels with the Equipment, so unequipping is how the
    // creature stops being forced to attack.
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +3/+0 and attacks each combat if able.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&TORMENTORS_TRIDENT_BONUS),
                },
            ),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{3}"))],
                "Equip {3} ({3}: Attach to target creature you control. Equip only as a \
                 sorcery.)",
            ),
        ]),
);

static VANGUARDS_SHIELD_BONUS: [AppliedEffectDef; 2] = [
    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(0), ValueDef::Constant(3)),
    AppliedEffectDef::Rule(AppliedRuleDef::MayBlockAdditionalCreatures(1)),
];

// AVR 223 — Vanguard's Shield
pub(in crate::card::sets) static VANGUARDS_SHIELD: CardRecord = CardRecord::new_with_legacy_id(
    1929,
    "Vanguard's Shield",
    CardArt::new("ce8d9db6-5737-4a1f-ae4e-75821a602784", "Ryan Pancoast"),
    CardSet::AvacynRestored,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +0/+3 and can block an additional creature each combat.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&VANGUARDS_SHIELD_BONUS),
                },
            ),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{3}"))],
                "Equip {3} ({3}: Attach to target creature you control. Equip only as a \
                 sorcery.)",
            ),
        ]),
);

// AVR 224 — Vessel of Endless Rest
pub(in crate::card::sets) static VESSEL_OF_ENDLESS_REST: CardRecord = CardRecord::new_with_legacy_id(
    836,
    "Vessel of Endless Rest",
    CardArt::new("ec733373-3f68-47ad-ac35-6f39092f1e26", "John Avon"),
    CardSet::AvacynRestored,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "When this artifact enters, put target card from a graveyard on the bottom of its owner's library.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)),
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Any,
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: None,
            })],
            EffectDef::MoveToZone {
                counters: None,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Library,
                placement: ZonePlacement::Bottom,
                arrival_effect: None,
                attachment: None,
                controller: None,
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// AVR 225 — Alchemist's Refuge
// Audit: partial — Needs a turn-scoped permission allowing every spell you cast to be cast as though it had flash.
pub(in crate::card::sets) static ALCHEMISTS_REFUGE: CardRecord = CardRecord::new_with_legacy_id(
    1694,
    "Alchemist's Refuge",
    CardArt::new(
        "c767a897-52e3-4401-8104-930157bb2b02",
        "Dan Murayama Scott",
    ),
    CardSet::AvacynRestored,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{G}{U}, {T}: You may cast spells this turn as though they had flash.",
            &[
                AbilityCostDef::Mana(mana_cost!("{G}{U}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::Special("Grant flash to every spell cast this turn"),
        )
        .with_coverage(AbilityCoverageDef::metadata_only(
            "The available flash grant applies only to the next sorcery spell, not every spell cast this turn.",
        )),
    ]),
);

static CAVERN_COLORED_MANA_RESTRICTIONS: [ManaRestrictionDef; 1] =
    [ManaRestrictionDef::CastCreatureSpellOfChosenType];

static CAVERN_COLORED_MANA_SPEND_EFFECTS: [ManaSpendEffectDef; 1] =
    [ManaSpendEffectDef::ApplyToPaidSpell(
        AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered),
    )];

// AVR 226 — Cavern of Souls
pub(in crate::card::sets) static CAVERN_OF_SOULS: CardRecord = CardRecord::new_with_legacy_id(
    147,
    "Cavern of Souls",
    CardArt::new("1381c8f1-a292-4bdf-b20c-a5c2a169ee84", "Cliff Childs"),
    CardSet::AvacynRestored,
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::replacement(
            "As this land enters, choose a creature type.",
            ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                crate::card::BattlefieldEntryScalarChoiceDef::CREATURE_TYPE,
            )),
        ),
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color. Spend this mana only to cast a creature spell of the chosen type, and that spell can't be countered.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(
                AddManaEffectDef::any_color()
                .with_restrictions(&CAVERN_COLORED_MANA_RESTRICTIONS)
                .with_spend_effects(&CAVERN_COLORED_MANA_SPEND_EFFECTS),
            ),
        ),
    ]),
);

// AVR 227 — Desolate Lighthouse
pub(in crate::card::sets) static DESOLATE_LIGHTHOUSE: CardRecord = CardRecord::new_with_legacy_id(
    837,
    "Desolate Lighthouse",
    CardArt::new("16fb45bc-6152-4b01-9831-a8e80b1c1852", "Scott Chou"),
    CardSet::AvacynRestored,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{1}{U}{R}, {T}: Draw a card, then discard a card.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{U}{R}")),
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
        ),
    ]),
);

// AVR 228 — Seraph Sanctuary
pub(in crate::card::sets) static SERAPH_SANCTUARY: CardRecord = CardRecord::new_with_legacy_id(
    838,
    "Seraph Sanctuary",
    CardArt::new("f903b04a-2733-4ce7-9d83-9db8d5e1e10d", "David Palumbo"),
    CardSet::AvacynRestored,
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::triggered(
            "When this land enters, you gain 1 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::triggered(
            "Whenever an Angel you control enters, you gain 1 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype("Angel"),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::tap_for(ManaColor::Colorless),
    ]),
);

// AVR 229 — Slayers' Stronghold
pub(in crate::card::sets) static SLAYERS_STRONGHOLD: CardRecord = CardRecord::new_with_legacy_id(
    839,
    "Slayers' Stronghold",
    CardArt::new("939a4351-3ec7-4e6c-8cdd-766bfd670391", "Karl Kopinski"),
    CardSet::AvacynRestored,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{R}{W}, {T}: Target creature gets +2/+0 and gains vigilance and haste until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{R}{W}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(0)),
                    AppliedEffectDef::add_ability(&abilities::vigilance()),
                    AppliedEffectDef::add_ability(&abilities::haste()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANGELS_MERCY,
    &ANGELIC_WALL,
    &ARCHANGEL,
    &AVACYN_ANGEL_OF_HOPE,
    &BANISHING_STROKE,
    &BUILDERS_BLESSING,
    &CATHARS_CRUSADE,
    &CATHEDRAL_SANCTIFIER,
    &COMMANDERS_AUTHORITY,
    &CURSEBREAK,
    &DEFANG,
    &ENTREAT_THE_ANGELS,
    &FARBOG_EXPLORER,
    &GOLDNIGHT_COMMANDER,
    &GOLDNIGHT_REDEEMER,
    &HOLY_JUSTICIAR,
    &LEAP_OF_FAITH,
    &MIDNIGHT_DUELIST,
    &MIDVAST_PROTECTOR,
    &MOONLIGHT_GEIST,
    &MOORLAND_INQUISITOR,
    &NEARHEATH_PILGRIM,
    &RESTORATION_ANGEL,
    &RIGHTEOUS_BLOW,
    &SERAPH_OF_DAWN,
    &SILVERBLADE_PALADIN,
    &SPECTRAL_GATEGUARDS,
    &TERMINUS,
    &THRABEN_VALIANT,
    &VOICE_OF_THE_PROVINCES,
    &ZEALOUS_STRIKE,
    &ALCHEMISTS_APPRENTICE,
    &ARCANE_MELEE,
    &CRIPPLING_CHILL,
    &DREADWATERS,
    &ELGAUD_SHIELDMATE,
    &FAVORABLE_WINDS,
    &FETTERGEIST,
    &FLEETING_DISTRACTION,
    &GALVANIC_ALCHEMIST,
    &GEIST_SNATCH,
    &GHOSTFORM,
    &GRYFF_VANGUARD,
    &INTO_THE_VOID,
    &LATCH_SEEKER,
    &LUNAR_MYSTIC,
    &MASS_APPEAL,
    &MIST_RAVEN,
    &NEPHALIA_SMUGGLER,
    &PEEL_FROM_REALITY,
    &ROTCROWN_GHOUL,
    &SCRAPSKIN_DRAKE,
    &STERN_MENTOR,
    &TANDEM_LOOKOUT,
    &TEMPORAL_MASTERY,
    &VANISHMENT,
    &WINGCRAFTER,
    &BLOOD_ARTIST,
    &BLOODFLOW_CONNOISSEUR,
    &BONE_SPLINTERS,
    &BUTCHER_GHOUL,
    &CRYPT_CREEPER,
    &DEATH_WIND,
    &DEMONIC_RISING,
    &DEMONIC_TASKMASTER,
    &DREAD_SLAVER,
    &DRIVER_OF_THE_DEAD,
    &ESSENCE_HARVEST,
    &EVERNIGHT_SHADE,
    &GRISELBRAND,
    &HARVESTER_OF_SOULS,
    &HUMAN_FRAILTY,
    &HUNTED_GHOUL,
    &MAALFELD_TWINS,
    &MARROW_BATS,
    &NECROBITE,
    &POLLUTED_DEAD,
    &RENEGADE_DEMON,
    &SEARCHLIGHT_GEIST,
    &SOULCAGE_FIEND,
    &TRIUMPH_OF_CRUELTY,
    &UNDEAD_EXECUTIONER,
    &ARCHWING_DRAGON,
    &BANNERS_RAISED,
    &BATTLE_HYMN,
    &BONFIRE_OF_THE_DAMNED,
    &DEMOLISH,
    &FALKENRATH_EXTERMINATOR,
    &FERVENT_CATHAR,
    &GANG_OF_DEVILS,
    &GUISE_OF_FIRE,
    &HANWEIR_LANCER,
    &HAVENGUL_VAMPIRE,
    &HEIRS_OF_STROMKIRK,
    &HOUND_OF_GRISELBRAND,
    &KESSIG_MALCONTENTS,
    &KRUIN_STRIKER,
    &LIGHTNING_MAULER,
    &LIGHTNING_PROWESS,
    &MAD_PROPHET,
    &MALICIOUS_INTENT,
    &PILLAR_OF_FLAME,
    &RAGING_POLTERGEIST,
    &RIOT_RINGLEADER,
    &RUSH_OF_BLOOD,
    &SCALDING_DEVIL,
    &SOMBERWALD_VIGILANTE,
    &STONEWRIGHT,
    &THUNDERBOLT,
    &THUNDEROUS_WRATH,
    &UNCANNY_SPEED,
    &VIGILANTE_JUSTICE,
    &ZEALOUS_CONSCRIPTS,
    &ABUNDANT_GROWTH,
    &BORDERLAND_RANGER,
    &BOWER_PASSAGE,
    &CRATERHOOF_BEHEMOTH,
    &DIREGRAF_ESCORT,
    &DRUIDS_FAMILIAR,
    &DRUIDS_REPOSITORY,
    &FLOWERING_LUMBERKNOT,
    &GEIST_TRAPPERS,
    &GLOOMWIDOW,
    &GROUNDED,
    &HOWLGEIST,
    &NATURAL_END,
    &NETTLE_SWINE,
    &NIGHTSHADE_PEDDLER,
    &PATHBREAKER_WURM,
    &RAIN_OF_THORNS,
    &SHELTERING_WORD,
    &SNARE_THE_SKIES,
    &SOMBERWALD_SAGE,
    &SOUL_OF_THE_HARVEST,
    &TERRIFYING_PRESENCE,
    &TIMBERLAND_GUIDE,
    &TRIUMPH_OF_FEROCITY,
    &TRUSTED_FORCEMAGE,
    &VORSTCLAW,
    &WANDERING_WOLF,
    &WOLFIR_AVENGER,
    &WOLFIR_SILVERHEART,
    &YEW_SPIRIT,
    &SIGARDA_HOST_OF_HERONS,
    &ANGELIC_ARMAMENTS,
    &BLADED_BRACERS,
    &CONJURERS_CLOSET,
    &HAUNTED_GUARDIAN,
    &MOONSILVER_SPEAR,
    &NARSTAD_SCRAPPER,
    &OTHERWORLD_ATLAS,
    &SCROLL_OF_AVACYN,
    &TORMENTORS_TRIDENT,
    &VANGUARDS_SHIELD,
    &VESSEL_OF_ENDLESS_REST,
    &ALCHEMISTS_REFUGE,
    &CAVERN_OF_SOULS,
    &DESOLATE_LIGHTHOUSE,
    &SERAPH_SANCTUARY,
    &SLAYERS_STRONGHOLD,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&alpha::PLAINS),        // AVR 230
    PrintingRecord::alternate(&alpha::PLAINS, 1),   // AVR 231
    PrintingRecord::alternate(&alpha::PLAINS, 2),   // AVR 232
    PrintingRecord::reprint(&alpha::ISLAND),        // AVR 233
    PrintingRecord::alternate(&alpha::ISLAND, 1),   // AVR 234
    PrintingRecord::alternate(&alpha::ISLAND, 2),   // AVR 235
    PrintingRecord::reprint(&alpha::SWAMP),         // AVR 236
    PrintingRecord::alternate(&alpha::SWAMP, 1),    // AVR 237
    PrintingRecord::alternate(&alpha::SWAMP, 2),    // AVR 238
    PrintingRecord::reprint(&alpha::MOUNTAIN),      // AVR 239
    PrintingRecord::alternate(&alpha::MOUNTAIN, 1), // AVR 240
    PrintingRecord::alternate(&alpha::MOUNTAIN, 2), // AVR 241
    PrintingRecord::reprint(&alpha::FOREST),        // AVR 242
    PrintingRecord::alternate(&alpha::FOREST, 1),   // AVR 243
    PrintingRecord::alternate(&alpha::FOREST, 2),   // AVR 244
];
