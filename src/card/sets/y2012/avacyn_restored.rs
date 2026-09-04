//! Avacyn Restored card records used by the built-in ISD–M14 Standard deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::sets::y1993::alpha;
use crate::card::sets::y2003::mirrodin as catalog_mrd;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, ActivationTimingDef,
    AddManaEffectDef, AppliedEffectDef, AppliedRuleDef, BasicLandType, CardArt, CardRules, CardSet,
    CardSupertype, CardType, ChoiceVisibilityDef, ChooseDef, ColorChoiceOperationDef, ColorSet,
    ComparisonDef, ControlDurationDef, CostModificationDef, CounterKind, CreatedTokensDef,
    CreatureTypeSetDef, DamageEventMatcherDef, DamageKindDef, DamagePreventionDef,
    DamageRecipientMatcherDef, DamageSourceMatcherDef, DiscardSelectionDef, DividedTotal,
    EffectChoiceDef, EffectDef, EffectPaymentDef, EffectRecipientDef, InstalledTriggerDef,
    KeywordAbility, ManaColor, ManaRestrictionDef, ManaSpendEffectDef, MoveObjectsDef,
    ObjectChoiceBindingDef, ObjectCollectionSourceDef, ObjectPredicateDef, ObjectQueryDef,
    ObjectRefDef, ObjectSetDef, ObjectSetFilterDef, PayOrDef, PlayerRefDef, PlayerRelation,
    PlayerSetDef, ReplacementChoiceDef, ReplacementEffectDef, ResolvedEffectDurationDef,
    SacrificedAmountDef, ScaledValueDef, TargetChooserDef, TriggerConditionDef, TriggerEventDef,
    TurnStepDef, ValueDef, ZoneChangeEventMatcherDef, ZoneKind, ZonePlacement, abilities,
};
use crate::{ParentBinding, TargetIndex, mana_cost};

/// Exile and return in one resolution, and the return names your control
/// rather than the card's owner. The two differ exactly when the creature
/// was stolen, which is when a blink is worth the mana.
static BLINK_UNDER_YOUR_CONTROL: [EffectDef; 2] = [
    EffectDef::ExileLinkedToSource {
        until_source_leaves: false,
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        face_down: false,
        then: None,
    },
    EffectDef::ReturnLinkedExiles {
        object: ObjectPredicateDef::Any,
        counters: None,
        zone: ZoneKind::Battlefield,
        grant: None,
        controller: Some(PlayerRelation::You),
        transformed: false,
    },
];

/// "This creature can block only creatures with flying."
static BLOCKS_ONLY_FLYERS: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::Source,
    effect: AppliedEffectDef::Rule(AppliedRuleDef::can_block_only(
        ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
    )),
};

static ANY_CREATURE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Creature),
)];

/// "If you control the creature with the greatest power or tied for the
/// greatest power." A tie counts, so this asks whether anything is strictly
/// bigger rather than whether one creature stands alone.
static CONTROLS_THE_BIGGEST: TriggerConditionDef =
    TriggerConditionDef::ControlsGreatestPowerCreature;

/// "Creatures with power less than this creature's power can't block it."
/// The comparison is against the source's current power, so pumping it widens
/// the restriction.
static WEAKER_THAN_SOURCE: ObjectPredicateDef =
    ObjectPredicateDef::PowerLessThan(ValueDef::SourcePower);

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

// AVR 1 — Angel of Glory's Rise
pub(in crate::card::sets) static ANGEL_OF_GLORY_S_RISE: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Angel of Glory's Rise",
    "7a8be765-0949-491c-875c-0385fb83e4b9",
    "James Ryman",
    CardRules::new_creature(mana_cost!("{5}{W}{W}"), &["Angel"], 4, 6).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, exile all Zombies, then return all Human creature cards from your graveyard to the battlefield.",
            EffectDef::Sequence(&[
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::Subtype("Zombie"),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    zone: ZoneKind::Exile,
                    placement: ZonePlacement::Top,
                },
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype("Human"),
                        ]),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    ),
                    zone: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                },
            ]),
        ),
    ]),
);

// AVR 2 — Angel of Jubilation
// Audit: unsupported — Needs a static prohibition on paying life or sacrificing creatures specifically to cast spells and activate abilities.
pub(in crate::card::sets) static ANGEL_OF_JUBILATION: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Angel of Jubilation",
    "16c5dfed-4dee-4e48-a445-89f03d7794e6",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// AVR 3 — Angel's Mercy (reprint)
const ANGELS_MERCY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::ANGELS_MERCY,
    "7a437999-26ae-49fa-8647-c8c2b4640702",
    "Greg Staples",
);

// AVR 4 — Angelic Wall (reprint)
const ANGELIC_WALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1998::portal_second_age::ANGELIC_WALL,
    "d7b2450d-87a7-46dc-b43a-2db2abeca44f",
    "Allen Williams",
);

// AVR 5 — Archangel (reprint)
const ARCHANGEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::visions::ARCHANGEL,
    "3741b2a7-7bda-481a-b8f8-9b04c96035b0",
    "Cynthia Sheppard",
);

// AVR 6 — Avacyn, Angel of Hope
pub(in crate::card::sets) static AVACYN_ANGEL_OF_HOPE: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Avacyn, Angel of Hope",
    "ba149706-cd17-4da6-8403-ccfe2d6cb437",
    "Jason Chan",
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
pub(in crate::card::sets) static BANISHING_STROKE: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Banishing Stroke",
    "238d8437-1abd-4bb7-8b5b-54f959bc2c79",
    "Igor Kieryluk",
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
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Library,
                placement: ZonePlacement::Bottom,
            },
        ),
        abilities::miracle(mana_cost!("{W}")),
    ]),
);

// AVR 8 — Builder's Blessing
pub(in crate::card::sets) static BUILDERS_BLESSING: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Builder's Blessing",
    "2ad27af1-b482-40d5-9dbb-11201ffa0410",
    "John Stanko",
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
pub(in crate::card::sets) static CALL_TO_SERVE: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Call to Serve",
    "ce4a3e80-6e95-4346-8ab8-eecc1a09ca24",
    "Jaime Jones",
    CardRules::new_enchantment(mana_cost!("{1}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant nonblack creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::Black)),
                    ]),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+2, has flying, and is an Angel in addition to its other types.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                        AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&[
                            "Angel",
                        ])),
                    ]),
                },
            ),
        ]),
);

// AVR 10 — Cathars' Crusade
pub(in crate::card::sets) static CATHARS_CRUSADE: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Cathars' Crusade",
    "78154978-9e7d-44e9-a03f-c578072a8ff7",
    "Karl Kopinski",
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
pub(in crate::card::sets) static CATHEDRAL_SANCTIFIER: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Cathedral Sanctifier",
    "76cac47a-9e83-4039-8d80-fa9bdadb7527",
    "Michael C. Hayes",
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Cleric"], 1, 1).with_ability(
        abilities::enters_trigger(
            "When this creature enters, you gain 3 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ),
    ),
);

// AVR 12 — Cloudshift
pub(in crate::card::sets) static CLOUDSHIFT: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Cloudshift",
    "35b06c8f-5f08-43bd-a548-2a98ba30fd41",
    "Howard Lyon",
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Exile target creature you control, then return that card to the battlefield under your control.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
        )],
        EffectDef::Sequence(&BLINK_UNDER_YOUR_CONTROL),
    )),
);

// AVR 13 — Commander's Authority
pub(in crate::card::sets) static COMMANDERS_AUTHORITY: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Commander's Authority",
    "08ef4383-11e7-4426-a04a-058570f46e47",
    "Johannes Voss",
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
                    effect: AppliedEffectDef::add_ability(&AbilityDef::triggered(
                        "At the beginning of your upkeep, create a 1/1 white Human creature token.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::Upkeep,
                            player: PlayerRelation::You,
                        },
                        EffectDef::create_creature_token(&["Human"], &[ManaColor::White], 1, 1).with_art(CardArt::new(
                            "8894949b-f190-461e-996a-cf2b39f08a5d",
                            "Michael C. Hayes",
                        )),
                    )),
                },
            ),
        ]),
);

// AVR 14 — Cursebreak
pub(in crate::card::sets) static CURSEBREAK: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Cursebreak",
    "c71a0883-316c-4870-a029-25f16952fbc0",
    "Sam Wolfe Connelly",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target enchantment. You gain 2 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Enchantment),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ]),
    )),
);

// AVR 15 — Defang
pub(in crate::card::sets) static DEFANG: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Defang",
    "f12fee0b-c237-4188-a718-1572f72c63ba",
    "Steven Belledin",
    CardRules::new_enchantment(mana_cost!("{1}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Prevent all damage that would be dealt by enchanted creature.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    // Every damage event the creature is the source of, not only the combat
                    // ones: a Defanged creature's activated abilities are as harmless as its
                    // attacks.
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::PreventDamage(
                        DamageEventMatcherDef {
                            kind: DamageKindDef::Any,
                            source: DamageSourceMatcherDef::AffectedObject,
                            recipient: DamageRecipientMatcherDef::Any,
                        },
                    )),
                },
            ),
        ]),
);

// AVR 16 — Defy Death
pub(in crate::card::sets) static DEFY_DEATH: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Defy Death",
    "028028d7-80ff-4d63-8b84-795f257a3456",
    "Karl Kopinski",
    CardRules::new_sorcery(mana_cost!("{3}{W}{W}")).with_ability(
        AbilityDef::spell_with_targets(
            "Return target creature card from your graveyard to the battlefield. If it's an Angel, put two +1/+1 counters on it.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::WithZoneMoveResult {
                effect: &EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                },
                binding: ParentBinding,
                then: &EffectDef::AddCounters {
                    object: EffectRecipientDef::objects(ObjectSetDef::Matching {
                        objects: &ObjectSetDef::ZoneChangeSuccessorsOfBinding(
                            ParentBinding,
                        ),
                        object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::Subtype(
                            "Angel",
                        )),
                    }),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(2),
                },
            },
        ),
    ),
);

// AVR 17 — Devout Chaplain
pub(in crate::card::sets) static DEVOUT_CHAPLAIN: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Devout Chaplain",
    "84ceb7f1-14b7-4102-ade2-fbeb835d3804",
    "Lucas Graciano",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Cleric"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{T}, Tap two untapped Humans you control: Exile target artifact or enchantment.",
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::TapPermanents {
                    object: ObjectPredicateDef::Subtype("Human"),
                    controller: PlayerRelation::You,
                    count: 2,
                },
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                ]),
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
        ),
    ),
);

// AVR 18 — Divine Deflection
// Audit: unsupported — Needs a duration-scoped prevention shield that tracks the amount prevented and redirects exactly that amount to a chosen target.
pub(in crate::card::sets) static DIVINE_DEFLECTION: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Divine Deflection",
    "ec06e42f-e294-44ef-8fa8-1d1a4c2090d8",
    "Steve Prescott",
    crate::card::CardRules::unsupported(),
);

// AVR 19 — Emancipation Angel
pub(in crate::card::sets) static EMANCIPATION_ANGEL: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Emancipation Angel",
    "7a4bc00e-28ca-4152-b832-f36425d2b615",
    "Scott Chou",
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Angel"], 3, 3).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, return a permanent you control to its owner's hand.",
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::PermanentsControlledBy(PlayerRefDef::EffectController),
                exclude: None,
                minimum: 1,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::MoveToZone {
                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
            }),
        ),
    ]),
);

// AVR 20 — Entreat the Angels
pub(in crate::card::sets) static ENTREAT_THE_ANGELS: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Entreat the Angels",
    "31292616-70e6-4d19-a883-e63ad860f50c",
    "Todd Lockwood",
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
pub(in crate::card::sets) static FARBOG_EXPLORER: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Farbog Explorer",
    "489c6a2f-38b4-4ff9-95f7-431384480ed9",
    "Scott Chou",
    // A white creature with swampwalk, which is the joke: it is unblockable
    // only against the colour least likely to want to block it.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Scout"], 2, 3)
        .with_ability(abilities::landwalk(BasicLandType::Swamp)),
);

// AVR 22 — Goldnight Commander
pub(in crate::card::sets) static GOLDNIGHT_COMMANDER: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Goldnight Commander",
    "c6ebec82-9d4a-4e78-b923-37c3a52133e7",
    "Chris Rahn",
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

// AVR 23 — Goldnight Redeemer
pub(in crate::card::sets) static GOLDNIGHT_REDEEMER: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Goldnight Redeemer",
    "df5656e3-5f53-41f8-9f24-04caad5e4ca3",
    "Karl Kopinski",
    CardRules::new_creature(mana_cost!("{4}{W}{W}"), &["Angel"], 4, 4).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, you gain 2 life for each other creature you control.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Scaled(&ScaledValueDef::new(
                    // "Other creatures you control", so the Redeemer's own arrival is not among
                    // them even though it is on the battlefield as the trigger resolves.
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    2,
                )),
            },
        ),
    ]),
);

// AVR 24 — Herald of War
pub(in crate::card::sets) static HERALD_OF_WAR: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Herald of War",
    "77e92bbb-c22d-4879-9437-b87a3ff70a2d",
    "Eric Deschamps",
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Angel"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever this creature attacks, put a +1/+1 counter on it.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::static_ability(
            "Angel spells and Human spells you cast cost {1} less to cast for each +1/+1 counter on this creature.",
            EffectDef::ModifyCost(CostModificationDef::reduce_spell(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Subtype("Angel"),
                    ObjectPredicateDef::Subtype("Human"),
                ]),
                PlayerRelation::You,
                ValueDef::CountersOnSource(CounterKind::PlusOnePlusOne),
            )),
        ),
    ]),
);

// AVR 25 — Holy Justiciar
pub(in crate::card::sets) static HOLY_JUSTICIAR: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Holy Justiciar",
    "640cad49-1db3-4611-a80d-7ce95f000fad",
    "David Rapoza",
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
                    condition: &TriggerConditionDef::TargetMatches {
                        slot: TargetIndex::PRIMARY,
                        object: ObjectPredicateDef::Subtype("Zombie"),
                    },
                    then: &EffectDef::MoveToZone {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        zone: ZoneKind::Exile,
                        placement: ZonePlacement::Top,
                    },
                },
            ]),
        ),
    ),
);

// AVR 26 — Leap of Faith
pub(in crate::card::sets) static LEAP_OF_FAITH: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Leap of Faith",
    "7ba52aed-440c-4b32-8f25-0c5364441712",
    "Gabor Szikszai",
    CardRules::new_instant(mana_cost!("{2}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gains flying until end of turn. Prevent all damage that would be dealt to that creature this turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::flying()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::to(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )),
);

// AVR 27 — Midnight Duelist
pub(in crate::card::sets) static MIDNIGHT_DUELIST: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Midnight Duelist",
    "2371bd0c-ca38-4a62-b525-bef4d1ca0646",
    "Bud Cook",
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 1, 2).with_ability(
        AbilityDef::keyword(
            "Protection from Vampires",
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::Subtype("Vampire")),
        ),
    ),
);

// AVR 28 — Midvast Protector
pub(in crate::card::sets) static MIDVAST_PROTECTOR: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Midvast Protector",
    "d4f6214f-90cb-4575-b221-3c8d0ed65ffe",
    "James Ryman",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Wizard"], 2, 3).with_ability(
        abilities::enters_trigger_with_targets("When this creature enters, target creature you control gains protection from the color of your choice until end of turn.", &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )], EffectDef::ChooseColor {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                operation: ColorChoiceOperationDef::ProtectionFromChosenColor,
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            }),
    ),
);

// AVR 29 — Moonlight Geist
pub(in crate::card::sets) static MOONLIGHT_GEIST: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Moonlight Geist",
    "4cf4c4cf-df35-4725-81ca-d62b70b8d0dd",
    "Dan Murayama Scott",
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
pub(in crate::card::sets) static MOORLAND_INQUISITOR: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Moorland Inquisitor",
    "581dbbea-9995-4e4b-ba5c-d6d5597e4ace",
    "David Palumbo",
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

// AVR 31 — Nearheath Pilgrim
pub(in crate::card::sets) static NEARHEATH_PILGRIM: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Nearheath Pilgrim",
    "d81d6fe0-c7c2-46a6-811c-f121284937ea",
    "Erica Yang",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Cleric"], 2, 1).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, both creatures have \
             lifelink.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &EffectDef::StaticApply {
                    recipient: SOULBOND_PAIR_RECIPIENT,
                    effect: AppliedEffectDef::add_ability(&abilities::lifelink()),
                },
            },
        ),
    ]),
);

// AVR 32 — Restoration Angel
pub(in crate::card::sets) static RESTORATION_ANGEL: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Restoration Angel",
    "c2ad8639-e586-47f4-baca-2a1af5aa281b",
    "Johannes Voss",
    CardRules::new_creature(
        mana_cost!("{3}{W}"),
        &["Angel"],
        3,
        4,
    )
    .with_abilities(&[
        abilities::flash(),
        abilities::flying(),
        abilities::enters_trigger_with_targets("When this creature enters, you may exile target non-Angel creature you control, then return that card to the battlefield under your control.", &[AbilityTargetDef::exactly_one(
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
            // resolution.
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Sequence(&[
                    EffectDef::ExileLinkedToSource {
                        until_source_leaves: false,
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
face_down: false,
then: None,
},
                    EffectDef::ReturnLinkedExiles {
                        object: ObjectPredicateDef::Any,
                        counters: None,
                        zone: ZoneKind::Battlefield,
                        grant: None,
                        controller: Some(PlayerRelation::You),
                        transformed: false,
                    },
                ]),
            }),
    ]),
);

// AVR 33 — Riders of Gavony
// Audit: unsupported — Needs protection from creatures of a dynamically chosen creature type.
pub(in crate::card::sets) static RIDERS_OF_GAVONY: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Riders of Gavony",
    "bcc1647b-9271-4426-9938-7eb620ad0769",
    "Volkan Baǵa",
    crate::card::CardRules::unsupported(),
);

// AVR 34 — Righteous Blow
pub(in crate::card::sets) static RIGHTEOUS_BLOW: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Righteous Blow",
    "9b640fdc-7a19-475e-858f-e159f61e154e",
    "Clint Cearley",
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
pub(in crate::card::sets) static SERAPH_OF_DAWN: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Seraph of Dawn",
    "5da345bd-8f2b-4966-97f5-c0e4c6cfe3b7",
    "Todd Lockwood",
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Angel"], 2, 4)
        .with_abilities(&[abilities::flying(), abilities::lifelink()]),
);

// AVR 36 — Silverblade Paladin
pub(in crate::card::sets) static SILVERBLADE_PALADIN: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Silverblade Paladin",
    "16298ca0-80d4-4299-a550-500b7ef6ac67",
    "Jason Chan",
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Human", "Knight"], 2, 2).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, both creatures have \\
             double strike.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &EffectDef::StaticApply {
                    recipient: SOULBOND_PAIR_RECIPIENT,
                    effect: AppliedEffectDef::add_ability(&abilities::double_strike()),
                },
            },
        ),
    ]),
);

// AVR 37 — Spectral Gateguards
pub(in crate::card::sets) static SPECTRAL_GATEGUARDS: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Spectral Gateguards",
    "f774e0eb-5c05-4a9e-8ab7-9ee4c7741591",
    "Wayne England",
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Spirit", "Soldier"], 2, 5).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, both creatures have \\
             vigilance.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &EffectDef::StaticApply {
                    recipient: SOULBOND_PAIR_RECIPIENT,
                    effect: AppliedEffectDef::add_ability(&abilities::vigilance()),
                },
            },
        ),
    ]),
);

// AVR 38 — Terminus
pub(in crate::card::sets) static TERMINUS: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Terminus",
    "0982ea7e-05a4-4e40-98ab-ea9aa6c7342e",
    "James Paick",
    CardRules::new_sorcery(mana_cost!("{4}{W}{W}")).with_abilities(&[
        AbilityDef::spell(
            "Put all creatures on the bottom of their owners' libraries.",
            EffectDef::MoveToZone {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                zone: ZoneKind::Library,
                placement: ZonePlacement::Bottom,
            },
        ),
        abilities::miracle(mana_cost!("{W}")),
    ]),
);

// AVR 39 — Thraben Valiant
pub(in crate::card::sets) static THRABEN_VALIANT: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Thraben Valiant",
    "20558f69-9240-49b9-9695-caf75ee2db1b",
    "Jason Chan",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Soldier"], 2, 1)
        .with_ability(abilities::vigilance()),
);

// AVR 40 — Voice of the Provinces
pub(in crate::card::sets) static VOICE_OF_THE_PROVINCES: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Voice of the Provinces",
    "b785276b-3778-49f3-b46f-a1f3d91db097",
    "Igor Kieryluk",
    CardRules::new_creature(mana_cost!("{4}{W}{W}"), &["Angel"], 3, 3).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, create a 1/1 white Human creature token.",
            EffectDef::create_creature_token(&["Human"], &[ManaColor::White], 1, 1).with_art(
                CardArt::new("8894949b-f190-461e-996a-cf2b39f08a5d", "Michael C. Hayes"),
            ),
        ),
    ]),
);

// AVR 41 — Zealous Strike
pub(in crate::card::sets) static ZEALOUS_STRIKE: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Zealous Strike",
    "ae8a01fb-dd47-44de-b528-8b7ca4b3388b",
    "Bud Cook",
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
pub(in crate::card::sets) static ALCHEMISTS_APPRENTICE: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Alchemist's Apprentice",
    "31abba67-1241-4fb3-88b5-4c4668ec5f25",
    "David Palumbo",
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
pub(in crate::card::sets) static AMASS_THE_COMPONENTS: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Amass the Components",
    "f5b48d60-fc99-4d21-9293-4f7ce1c02928",
    "Matt Stewart",
    CardRules::new_sorcery(mana_cost!("{3}{U}")).with_ability(AbilityDef::spell(
        "Draw three cards, then put a card from your hand on the bottom of your library.",
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Hand],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 1,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Private,
                then: &EffectDef::MoveObjects(MoveObjectsDef {
                    input: ObjectSetDef::Binding(ParentBinding),
                    from: Some(ZoneKind::Hand),
                    zone: ZoneKind::Library,
                    placement: ZonePlacement::Bottom,
                    moved: None,
                    then: &EffectDef::None,
                }),
            }),
        ]),
    )),
);

// AVR 44 — Arcane Melee
pub(in crate::card::sets) static ARCANE_MELEE: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Arcane Melee",
    "32355574-34ed-4427-86d9-72295d32ac4f",
    "Jaime Jones",
    CardRules::new_enchantment(mana_cost!("{4}{U}")).with_ability(AbilityDef::static_ability(
        "Instant and sorcery spells cost {2} less to cast.",
        EffectDef::ModifyCost(CostModificationDef::reduce_spell(
            // The only one of these that discounts both sides of the table, which is
            // what the caster relation is for.
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Instant),
                ObjectPredicateDef::HasType(CardType::Sorcery),
            ]),
            PlayerRelation::Any,
            ValueDef::Constant(2),
        )),
    )),
);

// AVR 45 — Captain of the Mists
pub(in crate::card::sets) static CAPTAIN_OF_THE_MISTS: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Captain of the Mists",
    "c43aa68e-a182-4006-b4d6-b4fc67e68583",
    "Allen Williams",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Wizard"], 2, 3).with_abilities(&[
        AbilityDef::triggered(
            "Whenever another Human you control enters, untap this creature.",
            TriggerEventDef::ZoneChanged(ZoneChangeEventMatcherDef::new(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype("Human"),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            )),
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
        AbilityDef::activated_with_targets(
            "{1}{U}, {T}: You may tap or untap target permanent.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{U}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Any,
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
    ]),
);

// AVR 46 — Crippling Chill
pub(in crate::card::sets) static CRIPPLING_CHILL: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Crippling Chill",
    "79791bd9-aded-48d9-866d-9f7bd6848905",
    "Svetlin Velinov",
    CardRules::new_instant(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Tap target creature. It doesn't untap during its controller's next untap step. Draw a \
         card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        // The tap and the skip are separate: a creature already tapped still owes
        // the untap step it misses, which is what the second clause is for.
        EffectDef::Sequence(&[
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
        ]),
    )),
);

// AVR 47 — Deadeye Navigator
pub(in crate::card::sets) static DEADEYE_NAVIGATOR: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Deadeye Navigator",
    "fa94262b-f740-48fb-a937-75776864c9ee",
    "Tomasz Jedruszek",
    CardRules::new_creature(mana_cost!("{4}{U}{U}"), &["Spirit"], 5, 5).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, each of those creatures has \"{1}{U}: Exile this creature, then return it to the battlefield under your control.\"",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &EffectDef::StaticApply {
                    recipient: SOULBOND_PAIR_RECIPIENT,
                    effect: AppliedEffectDef::add_ability(&AbilityDef::activated(
                        "{1}{U}: Exile this creature, then return it to the battlefield under your control.",
                        &[AbilityCostDef::Mana(mana_cost!("{1}{U}"))],
                        EffectDef::Sequence(&[
                            EffectDef::ExileLinkedToSource {
                                until_source_leaves: false,
                                object: EffectRecipientDef::Source,
                                face_down: false,
                                then: None,
                            },
                            EffectDef::ReturnLinkedExiles {
                                object: ObjectPredicateDef::Any,
                                counters: None,
                                zone: ZoneKind::Battlefield,
                                grant: None,
                                controller: Some(PlayerRelation::You),
                                transformed: false,
                            },
                        ]),
                    )),
                },
            },
        ),
    ]),
);

// AVR 48 — Devastation Tide
pub(in crate::card::sets) static DEVASTATION_TIDE: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Devastation Tide",
    "23b62be6-1a80-4f16-a94a-374203052662",
    "Raymond Swanland",
    CardRules::new_sorcery(mana_cost!("{3}{U}{U}")).with_abilities(&[
        AbilityDef::spell(
            "Return all nonland permanents to their owners' hands.",
            EffectDef::MoveToZone {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
        abilities::miracle(mana_cost!("{1}{U}")),
    ]),
);

// AVR 49 — Dreadwaters
pub(in crate::card::sets) static DREADWATERS: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Dreadwaters",
    "88245a41-d4d5-46bf-969f-48d4dd540e2c",
    "Cliff Childs",
    CardRules::new_sorcery(mana_cost!("{3}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target player mills X where X is the number of lands you control.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Mill {
            player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Land),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            )),
        },
    )),
);

// AVR 50 — Elgaud Shieldmate
pub(in crate::card::sets) static ELGAUD_SHIELDMATE: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Elgaud Shieldmate",
    "e7d376ef-c900-4abb-9a0b-5eb9369f5739",
    "Anthony Palumbo",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Human", "Soldier"], 2, 3).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, both creatures have \\
             hexproof.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &EffectDef::StaticApply {
                    recipient: SOULBOND_PAIR_RECIPIENT,
                    effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
                },
            },
        ),
    ]),
);

// AVR 51 — Favorable Winds
pub(in crate::card::sets) static FAVORABLE_WINDS: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Favorable Winds",
    "4cbd57f1-9883-40a4-9b52-1649cee83815",
    "Winona Nelson",
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

// AVR 52 — Fettergeist
pub(in crate::card::sets) static FETTERGEIST: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Fettergeist",
    "8e89ef0e-1bfe-4e12-90ee-38f993cd8110",
    "Izzy",
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
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                ),
                if_paid: None,
                otherwise: Some(&EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                }),
                visibility: ChoiceVisibilityDef::Public,
                condition: None,
            }),
        ),
    ]),
);

// AVR 53 — Fleeting Distraction (reprint)
const FLEETING_DISTRACTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::rise_of_the_eldrazi::FLEETING_DISTRACTION,
    "1ba49d16-e3e4-470a-8ca2-a93a5b358f6e",
    "Ryan Yee",
);

// AVR 54 — Galvanic Alchemist
pub(in crate::card::sets) static GALVANIC_ALCHEMIST: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Galvanic Alchemist",
    "b0e24d65-0e6f-4978-8de1-c5e4acac12fb",
    "Svetlin Velinov",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Wizard"], 1, 4).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, each of those creatures \
             has \"{2}{U}: Untap this creature.\"",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &EffectDef::StaticApply {
                    recipient: SOULBOND_PAIR_RECIPIENT,
                    // Granted to each creature separately, so each pays its own {2}{U} and
                    // untaps only itself.
                    effect: AppliedEffectDef::add_ability(&AbilityDef::activated(
                        "{2}{U}: Untap this creature.",
                        &[AbilityCostDef::Mana(mana_cost!("{2}{U}"))],
                        EffectDef::Untap {
                            object: EffectRecipientDef::Source,
                        },
                    )),
                },
            },
        ),
    ]),
);

// AVR 55 — Geist Snatch
pub(in crate::card::sets) static GEIST_SNATCH: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Geist Snatch",
    "b6dac5db-ef96-4bd5-aabc-e5ae2b95c8c3",
    "Dan Murayama Scott",
    CardRules::new_instant(mana_cost!("{2}{U}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target creature spell. Create a 1/1 blue Spirit creature token with flying.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Spell,
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]),
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Graveyard,
                placement: ZonePlacement::Top,
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
pub(in crate::card::sets) static GHOSTFORM: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Ghostform",
    "1f6a20ba-6691-4844-9685-dfcd4184224e",
    "Scott Chou",
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
            effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                ObjectPredicateDef::Any,
            )),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// AVR 57 — Ghostly Flicker
pub(in crate::card::sets) static GHOSTLY_FLICKER: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Ghostly Flicker",
    "f0a44373-0c50-4e14-a7c6-0de66796b81e",
    "Raymond Swanland",
    CardRules::new_instant(mana_cost!("{2}{U}")).with_ability(
        AbilityDef::spell_with_targets(
            "Exile two target artifacts, creatures, and/or lands you control, then return those cards to the battlefield under your control.",
            &[AbilityTargetDef {
                predicate: AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Land),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
                chooser: TargetChooserDef::Controller,
                minimum: 2,
                maximum: 2,
                exact_count: None,
                divided_total: None,
                another: false,
                excludes_source: false,
            }],
            EffectDef::Sequence(&BLINK_UNDER_YOUR_CONTROL),
        ),
    ),
);

// AVR 58 — Ghostly Touch
pub(in crate::card::sets) static GHOSTLY_TOUCH: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Ghostly Touch",
    "3ebae54a-47e0-4e82-8a29-b5d9354a748b",
    "Jason Felix",
    CardRules::new_enchantment(mana_cost!("{1}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature has \"Whenever this creature attacks, you may tap or untap target permanent.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&AbilityDef::triggered_with_targets(
                        "Whenever this creature attacks, you may tap or untap target permanent.",
                        TriggerEventDef::attacks(ObjectPredicateDef::Source),
                        &[AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::Any,
                        )],
                        EffectDef::May {
                            player: EffectRecipientDef::Controller,
                            effect: &EffectDef::ChooseEffect {
                                player: EffectRecipientDef::Controller,
                                choices: &[
                                    EffectChoiceDef {
                                        label: "Tap it",
                                        effect: EffectDef::Tap {
                                            object: EffectRecipientDef::Target(
                                                TargetIndex::PRIMARY,
                                            ),
                                        },
                                    },
                                    EffectChoiceDef {
                                        label: "Untap it",
                                        effect: EffectDef::Untap {
                                            object: EffectRecipientDef::Target(
                                                TargetIndex::PRIMARY,
                                            ),
                                        },
                                    },
                                ],
                            },
                        },
                    )),
                },
            ),
        ]),
);

// AVR 59 — Gryff Vanguard
pub(in crate::card::sets) static GRYFF_VANGUARD: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Gryff Vanguard",
    "b7238136-c8de-4949-9b54-ff75094e0569",
    "Jason Chan",
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Human", "Knight"], 3, 2).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// AVR 60 — Havengul Skaab
pub(in crate::card::sets) static HAVENGUL_SKAAB: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Havengul Skaab",
    "4c7ff97f-fb06-4a61-98cd-50965a6522d4",
    "Vincent Proce",
    CardRules::new_creature(mana_cost!("{5}{U}"), &["Zombie", "Horror"], 4, 5).with_ability(
        AbilityDef::triggered(
            "Whenever this creature attacks, return another creature you control to its owner's hand.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
                exclude: Some(ObjectRefDef::Source),
                minimum: 1,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::MoveToZone {
                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
            }),
        ),
    ),
);

// AVR 61 — Infinite Reflection
// Audit: unsupported — Needs attachment-derived copy effects for existing creatures and an entry replacement that copies the currently enchanted creature.
pub(in crate::card::sets) static INFINITE_REFLECTION: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Infinite Reflection",
    "42506c54-e9bf-4d0f-8dd5-8b218668c925",
    "Igor Kieryluk",
    crate::card::CardRules::unsupported(),
);

// AVR 62 — Into the Void
pub(in crate::card::sets) static INTO_THE_VOID: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Into the Void",
    "5ddd1050-8abd-4dfe-9e52-5b56af358653",
    "Daarken",
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
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
        },
    )),
);

// AVR 63 — Latch Seeker
pub(in crate::card::sets) static LATCH_SEEKER: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Latch Seeker",
    "3e4e7589-9cee-4d57-8648-ce733781bfb2",
    "Vincent Proce",
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Spirit"], 3, 1).with_ability(
        AbilityDef::static_ability(
            "This creature can't be blocked.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::Any,
                )),
            },
        ),
    ),
);

// AVR 64 — Lone Revenant
pub(in crate::card::sets) static LONE_REVENANT: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Lone Revenant",
    "2e12186e-9c93-4136-9ea3-e8d2ae1ee2e5",
    "Jaime Jones",
    CardRules::new_creature(mana_cost!("{3}{U}{U}"), &["Spirit"], 4, 4).with_abilities(&[
        abilities::hexproof(),
        AbilityDef::triggered_if(
            "Whenever this creature deals combat damage to a player, if you control no other creatures, look at the top four cards of your library. Put one of them into your hand and the rest on the bottom of your library in any order.",
            TriggerEventDef::DamageDealt(DamageEventMatcherDef {
                kind: DamageKindDef::Combat,
                source: DamageSourceMatcherDef::Matching(ObjectPredicateDef::Source),
                recipient: DamageRecipientMatcherDef::Recipients(EffectRecipientDef::EachPlayer),
            }),
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::Equal,
                amount: 0,
            },
            abilities::look_at_top_cards_choose_to_hand_rest_bottom(
                ValueDef::Constant(4),
                ObjectPredicateDef::Any,
                1,
                1,
            ),
        ),
    ]),
);

// AVR 65 — Lunar Mystic
pub(in crate::card::sets) static LUNAR_MYSTIC: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Lunar Mystic",
    "f346d236-528c-4164-9995-74cdc56597a9",
    "Wesley Burt",
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Human", "Wizard"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever you cast an instant spell, you may pay {1}. If you do, draw a card.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
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

// AVR 66 — Mass Appeal
pub(in crate::card::sets) static MASS_APPEAL: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Mass Appeal",
    "dfe9ae51-fd2b-45ca-a780-725f51f897b2",
    "Christopher Moeller",
    CardRules::new_sorcery(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell(
        "Draw a card for each Human you control.",
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                ObjectPredicateDef::Subtype("Human"),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            )),
        },
    )),
);

// AVR 67 — Mist Raven
pub(in crate::card::sets) static MIST_RAVEN: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Mist Raven",
    "0d98f0c4-021a-407a-8b0c-5500d804f959",
    "John Avon",
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Bird"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, return target creature to its owner's hand.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// AVR 68 — Misthollow Griffin
// Audit: unsupported — Needs a cast permission and play-option source zone for casting this card from exile.
pub(in crate::card::sets) static MISTHOLLOW_GRIFFIN: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Misthollow Griffin",
    "2db4fe28-e580-479b-910f-b719d69468b1",
    "Jaime Jones",
    crate::card::CardRules::unsupported(),
);

// AVR 69 — Nephalia Smuggler
pub(in crate::card::sets) static NEPHALIA_SMUGGLER: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Nephalia Smuggler",
    "1a531b2f-2a9e-4cc9-aea6-9dce239f5511",
    "Matt Stewart",
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
pub(in crate::card::sets) static OUTWIT: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Outwit",
    "429f7cf0-579a-4003-b5cf-4baf5d420796",
    "Erica Yang",
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell that targets a player.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Spell,
                    ObjectPredicateDef::HasDeclaredPlayerTarget(PlayerRelation::Any),
                ]),
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::counter_target(TargetIndex::PRIMARY),
    )),
);

// AVR 71 — Peel from Reality (reprint)
const PEEL_FROM_REALITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2005::ravnica_city_of_guilds::PEEL_FROM_REALITY,
    "7f41285b-5961-4653-96a0-fb6d27111390",
    "Jason Felix",
);

// AVR 72 — Rotcrown Ghoul
pub(in crate::card::sets) static ROTCROWN_GHOUL: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Rotcrown Ghoul",
    "f13b5ba6-0de1-4f5c-867b-57e2c10bde8e",
    "Dave Kendall",
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Zombie"], 3, 3).with_ability(
        abilities::dies_trigger_with_targets(
            "When this creature dies, target player mills five cards.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(5),
            },
        ),
    ),
);

// AVR 73 — Scrapskin Drake
pub(in crate::card::sets) static SCRAPSKIN_DRAKE: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Scrapskin Drake",
    "c9f03bae-1d23-43ea-9079-4b09d61bbadd",
    "Kev Walker",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Zombie", "Drake"], 2, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "This creature can block only creatures with flying.",
            BLOCKS_ONLY_FLYERS,
        ),
    ]),
);

// AVR 74 — Second Guess
// Audit: unsupported — Needs a target predicate or casting-history relation for the second spell cast during the current turn.
pub(in crate::card::sets) static SECOND_GUESS: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Second Guess",
    "0d22d093-8e89-4d54-ac04-14c8759de3ea",
    "Karl Kopinski",
    crate::card::CardRules::unsupported(),
);

// AVR 75 — Spectral Prison
pub(in crate::card::sets) static SPECTRAL_PRISON: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Spectral Prison",
    "89d141bc-7307-40c2-a7ed-427caaec5efc",
    "Vincent Proce",
    CardRules::new_enchantment(mana_cost!("{1}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Whenever enchanted creature becomes the target of a spell, sacrifice this Aura.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&AbilityDef::triggered(
                        "Whenever this creature becomes the target of a spell, sacrifice the Aura granting this ability.",
                        TriggerEventDef::BecomesTargetOfSpell(ObjectPredicateDef::Any),
                        EffectDef::Sacrifice {
                            object: EffectRecipientDef::object(ObjectRefDef::AbilityGrantSource),
                        },
                    )),
                },
            ),
        ]),
);

// AVR 76 — Spirit Away
pub(in crate::card::sets) static SPIRIT_AWAY: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Spirit Away",
    "d8823bdc-0467-47f1-9bef-a281b4a7071d",
    "Greg Staples",
    CardRules::new_enchantment(mana_cost!("{5}{U}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "You control enchanted creature.",
                EffectDef::GainControl {
                    object: EffectRecipientDef::AttachedPermanent,
                    controller: PlayerRefDef::EffectController,
                    duration: ControlDurationDef::WhileSourceRemains {
                        while_tapped: false,
                    },
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+2 and has flying.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                    ]),
                },
            ),
        ]),
);

// AVR 77 — Stern Mentor
pub(in crate::card::sets) static STERN_MENTOR: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Stern Mentor",
    "ffe4d34f-68f0-4d79-9aab-58c5304224d9",
    "Igor Kieryluk",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Human", "Wizard"], 2, 2).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, each of those creatures \
             has \"{T}: Target player mills two cards.\"",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &EffectDef::StaticApply {
                    recipient: SOULBOND_PAIR_RECIPIENT,
                    effect: AppliedEffectDef::add_ability(&AbilityDef::activated_with_targets(
                        "{T}: Target player mills two cards.",
                        &[AbilityCostDef::TapSource],
                        &[AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::Player(PlayerRelation::Any),
                        )],
                        EffectDef::Mill {
                            player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            amount: ValueDef::Constant(2),
                        },
                    )),
                },
            },
        ),
    ]),
);

// AVR 78 — Stolen Goods
// Audit: unsupported — Exile-until now exists, but its free-cast permission is resolution-scoped; this card needs the matched card castable through end of turn.
pub(in crate::card::sets) static STOLEN_GOODS: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Stolen Goods",
    "53203dfc-5ad5-4c17-9802-ca2f874d327a",
    "Anthony Francisco",
    crate::card::CardRules::unsupported(),
);

// AVR 79 — Tamiyo, the Moon Sage
// Audit: unsupported — Needs next-untap-step duration, a tapped-creature count, maximum-hand-size modification, and graveyard-entry triggers from every zone.
pub(in crate::card::sets) static TAMIYO_THE_MOON_SAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Tamiyo, the Moon Sage",
    "b9398926-13b9-47b8-b66b-1ab9d06bb704",
    "Eric Deschamps",
    crate::card::CardRules::unsupported(),
);

// AVR 80 — Tandem Lookout
pub(in crate::card::sets) static TANDEM_LOOKOUT: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Tandem Lookout",
    "83564e67-2677-4955-a3b9-3b221dbb100b",
    "Kev Walker",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Scout"], 2, 1).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as Tandem Lookout is paired with another creature, each of those creatures \
             has \"Whenever this creature deals damage to an opponent, draw a card.\"",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &EffectDef::StaticApply {
                    recipient: SOULBOND_PAIR_RECIPIENT,
                    // Damage of any kind to an opponent, not only combat damage, and granted to
                    // each creature so either connecting draws.
                    effect: AppliedEffectDef::add_ability(&AbilityDef::triggered(
                        "Whenever this creature deals damage to an opponent, draw a card.",
                        TriggerEventDef::damage_to_player(
                            ObjectPredicateDef::Source,
                            PlayerRelation::Opponent,
                        ),
                        EffectDef::DrawCards {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(1),
                        },
                    )),
                },
            },
        ),
    ]),
);

// AVR 81 — Temporal Mastery
pub(in crate::card::sets) static TEMPORAL_MASTERY: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Temporal Mastery",
    "266e5267-2288-4bb0-8c54-0c556521cec3",
    "Franz Vohwinkel",
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
pub(in crate::card::sets) static VANISHMENT: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Vanishment",
    "dece40c1-790c-4471-a790-1d356b345603",
    "Daarken",
    CardRules::new_instant(mana_cost!("{4}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Put target nonland permanent on top of its owner's library.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Library,
                placement: ZonePlacement::Top,
            },
        ),
        abilities::miracle(mana_cost!("{U}")),
    ]),
);

// AVR 83 — Wingcrafter
pub(in crate::card::sets) static WINGCRAFTER: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Wingcrafter",
    "04a3059f-92f2-4163-b79a-154118a4e36d",
    "Matt Stewart",
    CardRules::new_creature(mana_cost!("{U}"), &["Human", "Wizard"], 1, 1).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, both creatures have \\
             flying.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &EffectDef::StaticApply {
                    recipient: SOULBOND_PAIR_RECIPIENT,
                    effect: AppliedEffectDef::add_ability(&abilities::flying()),
                },
            },
        ),
    ]),
);

// AVR 84 — Appetite for Brains
pub(in crate::card::sets) static APPETITE_FOR_BRAINS: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Appetite for Brains",
    "062ee892-cce7-42bd-97c7-032cec61faca",
    "Michael C. Hayes",
    CardRules::new_sorcery(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target opponent reveals their hand. You choose a card from it with mana value 4 or greater and exile that card.",
        &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
            PlayerRelation::Opponent,
        ))],
        EffectDef::Choose(ChooseDef {
            binding: ObjectChoiceBindingDef::Objects(ParentBinding),
            unchosen: None,
            chooser: PlayerRefDef::EffectController,
            candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
                ObjectPredicateDef::Not(&ObjectPredicateDef::ManaValueAtMost(3)),
                &[ZoneKind::Hand],
                PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
            )),
            exclude: None,
            minimum: 0,
            maximum: 1,
            visibility: ChoiceVisibilityDef::Public,
            then: &EffectDef::MoveObjects(MoveObjectsDef {
                input: ObjectSetDef::Binding(ParentBinding),
                from: Some(ZoneKind::Hand),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
                moved: None,
                then: &EffectDef::None,
            }),
        }),
    )),
);

// AVR 85 — Barter in Blood (reprint)
const BARTER_IN_BLOOD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mrd::BARTER_IN_BLOOD,
    "39b4fab6-73ce-4a56-a305-4d2e93dbb4ee",
    "Eric Deschamps",
);

// AVR 86 — Blood Artist
pub(in crate::card::sets) static BLOOD_ARTIST: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Blood Artist",
    "2e1fb442-68ff-4249-8e44-87edf6fae211",
    "Johannes Voss",
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
pub(in crate::card::sets) static BLOODFLOW_CONNOISSEUR: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Bloodflow Connoisseur",
    "97485dbf-2f31-4ed2-a6cd-529ca22c9ac5",
    "Slawomir Maniak",
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

// AVR 88 — Bone Splinters (reprint)
const BONE_SPLINTERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2008::shards_of_alara::BONE_SPLINTERS,
    "387eda28-f35b-48b0-ba59-773d82902327",
    "Nils Hamm",
);

// AVR 89 — Butcher Ghoul
pub(in crate::card::sets) static BUTCHER_GHOUL: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Butcher Ghoul",
    "44a91e62-e946-4101-8cef-d1c147caebf2",
    "Christopher Moeller",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Zombie"], 1, 1)
        .with_ability(abilities::undying()),
);

// AVR 90 — Corpse Traders
pub(in crate::card::sets) static CORPSE_TRADERS: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Corpse Traders",
    "df3eed10-7a8f-4c89-8be8-389f979e10b7",
    "Kev Walker",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Human", "Rogue"], 3, 3).with_ability(
        AbilityDef::activated_with_targets(
            "{2}{B}, Sacrifice a creature: Target opponent reveals their hand. You choose a card from it. That player discards that card. Activate only as a sorcery.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{B}")),
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    controller: PlayerRelation::You,
                },
            ],
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                PlayerRelation::Opponent,
            ))],
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Hand],
                    PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                )),
                exclude: None,
                minimum: 0,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::DiscardCards {
                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                        ParentBinding,
                    )),
                },
            }),
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ),
);

// AVR 91 — Crypt Creeper (reprint)
const CRYPT_CREEPER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2001::odyssey::CRYPT_CREEPER,
    "0382cb94-0836-4e23-99b7-034faa363203",
    "Scott Chou",
);

// AVR 92 — Dark Impostor
pub(in crate::card::sets) static DARK_IMPOSTOR: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Dark Impostor",
    "8f5e8815-cda8-407d-847c-968b72c061e8",
    "Johannes Voss",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Vampire", "Assassin"], 2, 2).with_abilities(
        &[
            AbilityDef::activated_with_targets(
                "{4}{B}{B}: Exile target creature and put a +1/+1 counter on this creature.",
                &[AbilityCostDef::Mana(mana_cost!("{4}{B}{B}"))],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Sequence(&[
                    EffectDef::ExileLinkedToSource {
                        until_source_leaves: false,
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        face_down: false,
                        then: None,
                    },
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
            AbilityDef::static_ability(
                "This creature has all activated abilities of all creature cards exiled with it.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Characteristic(
                        crate::card::CharacteristicOperationDef::Abilities(
                            crate::card::AbilityOperationDef::AddActivatedAbilitiesOfLinkedExiles(
                                ObjectPredicateDef::HasType(CardType::Creature),
                            ),
                        ),
                    ),
                },
            ),
        ],
    ),
);

// AVR 93 — Death Wind
pub(in crate::card::sets) static DEATH_WIND: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Death Wind",
    "462a0961-cca5-4d63-867f-7426dbef8639",
    "Tomasz Jedruszek",
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

// AVR 94 — Demonic Rising
pub(in crate::card::sets) static DEMONIC_RISING: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Demonic Rising",
    "a2136a82-b535-47f6-9eee-5b7585ac5cf1",
    "Trevor Claxton",
    CardRules::new_enchantment(mana_cost!("{3}{B}{B}")).with_ability(
        AbilityDef::triggered_if(
            "At the beginning of your end step, if you control exactly one creature, create a 5/5 black Demon creature token with flying.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            // The printed intervening-if condition is checked both as the end step begins
            // and again when the trigger resolves.
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::Equal,
                amount: 1,
            },
            EffectDef::create_creature_token(&["Demon"], &[ManaColor::Black], 5, 5).with_abilities(&[abilities::flying()]).with_art(CardArt::new("6a3fc83f-ab02-4a44-910a-bfadc71cf162", "Kev Walker")),
        ),
    ),
);

// AVR 95 — Demonic Taskmaster
pub(in crate::card::sets) static DEMONIC_TASKMASTER: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Demonic Taskmaster",
    "fb5d6266-30a7-4360-84bc-22b52fb782b3",
    "Chris Rahn",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Demon"], 4, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "At the beginning of your upkeep, sacrifice a creature other than this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::SacrificeOfChoice {
                count: ValueDef::Constant(1),
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
pub(in crate::card::sets) static DEMONLORD_OF_ASHMOUTH: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Demonlord of Ashmouth",
    "785da9a3-09af-45aa-bc04-4ab69cfb2ba4",
    "Lucas Graciano",
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Demon"], 5, 4).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, exile it unless you sacrifice another creature.",
            EffectDef::PayOr(PayOrDef::unless(
                EffectPaymentDef {
                    payer: PlayerSetDef::Related(PlayerRelation::You),
                    cost: crate::card::EffectPaymentCostDef::SacrificePermanentMatching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                    ),
                },
                &EffectDef::MoveToZone {
                    object: EffectRecipientDef::Source,
                    zone: ZoneKind::Exile,
                    placement: ZonePlacement::Top,
                },
            )),
        ),
        abilities::undying(),
    ]),
);

// AVR 97 — Descent into Madness
// Audit: unsupported — Needs each player to choose a counter-derived number of permanents and/or hand cards to exile in one resolving choice.
pub(in crate::card::sets) static DESCENT_INTO_MADNESS: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Descent into Madness",
    "fa016bb7-ad8b-40d5-90db-412a9cf19e4e",
    "Anthony Francisco",
    crate::card::CardRules::unsupported(),
);

// AVR 98 — Dread Slaver
pub(in crate::card::sets) static DREAD_SLAVER: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Dread Slaver",
    "3d8a3abd-a4a2-48e6-b709-1c0240a76c5e",
    "Dave Kendall",
    // It keeps whatever it kills, so blocking it is worse than taking five.
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Zombie", "Horror"], 3, 5).with_ability(
        abilities::creature_damaged_by_source_dies_trigger(
            "Whenever a creature dealt damage by this creature this turn dies, return it to the battlefield under your control. That creature is a black Zombie in addition to its other colors and types.",
            EffectDef::WithZoneMoveResult {
                effect: &EffectDef::WithBattlefieldArrival {
                    effect: &EffectDef::MoveToZone {
                        object: EffectRecipientDef::TriggeringZoneChangeResult,
                        zone: ZoneKind::Battlefield,
                        placement: ZonePlacement::Top,
                    },
                    arrival: crate::card::BattlefieldArrivalDef {
                        controller: Some(PlayerRelation::You),
                        ..crate::card::BattlefieldArrivalDef::DEFAULT
                    },
                },
                binding: crate::ParentBinding,
                then: &EffectDef::Apply {
                    recipient: EffectRecipientDef::binding_zone_change_successors(
                        crate::ParentBinding,
                    ),
                    // "In addition to its other colors and types", so both leaves add rather
                    // than set. The follow-up targets the new permanent through the move's
                    // explicit successor binding.
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_colors(ColorSet::from_colors(&[
                            ManaColor::Black,
                        ])),
                        AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&[
                            "Zombie",
                        ])),
                    ]),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
            },
        ),
    ),
);

// AVR 99 — Driver of the Dead
pub(in crate::card::sets) static DRIVER_OF_THE_DEAD: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Driver of the Dead",
    "56113cde-4210-46be-bd53-8966c36ef2a3",
    "James Ryman",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Vampire"], 3, 2).with_ability(
        abilities::dies_trigger_with_targets("When this creature dies, return target creature card with mana value 2 or less from your graveyard to the battlefield.", &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ManaValueAtMost(2),
                ]),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            })], EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
}),
    ),
);

// AVR 100 — Essence Harvest
pub(in crate::card::sets) static ESSENCE_HARVEST: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Essence Harvest",
    "7c3fac03-a019-4faa-bc1c-09e3a394fff7",
    "Daarken",
    CardRules::new_sorcery(mana_cost!("{2}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target player loses X life and you gain X life, where X is the greatest power among creatures you control.",
        &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
            PlayerRelation::Any,
        ))],
        // One X read once and spent twice: the drain moves exactly what it takes.
        EffectDef::Sequence(&[
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: abilities::greatest_power_you_control(),
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: abilities::greatest_power_you_control(),
            },
        ]),
    )),
);

// AVR 101 — Evernight Shade
pub(in crate::card::sets) static EVERNIGHT_SHADE: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Evernight Shade",
    "1091fadf-97c4-4f87-8466-6a1246a72226",
    "Nic Klein",
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
// Audit: unsupported — Needs a committed life-loss event that also captures nondamage life loss and its amount.
pub(in crate::card::sets) static EXQUISITE_BLOOD: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Exquisite Blood",
    "4cd54279-57f5-4cd9-b524-4f094bd2fc36",
    "Cynthia Sheppard",
    crate::card::CardRules::unsupported(),
);

// AVR 103 — Ghoulflesh
pub(in crate::card::sets) static GHOULFLESH: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Ghoulflesh",
    "2eed3d1b-3142-437c-99e9-85ba76e23e6d",
    "Igor Kieryluk",
    CardRules::new_enchantment(mana_cost!("{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets -1/-1 and is a black Zombie in addition to its other colors and types.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(-1),
                            ValueDef::Constant(-1),
                        ),
                        AppliedEffectDef::add_colors(ColorSet::from_colors(&[
                            ManaColor::Black,
                        ])),
                        AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&[
                            "Zombie",
                        ])),
                    ]),
                },
            ),
        ]),
);

// AVR 104 — Gloom Surgeon
// Audit: unsupported — Needs a combat-damage replacement that prevents the event and exiles exactly that many cards from the top of your library.
pub(in crate::card::sets) static GLOOM_SURGEON: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Gloom Surgeon",
    "1b00c711-007d-4c85-9dd9-dd9d52f1649d",
    "Volkan Baǵa",
    crate::card::CardRules::unsupported(),
);

// AVR 105 — Grave Exchange
pub(in crate::card::sets) static GRAVE_EXCHANGE: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Grave Exchange",
    "14f420c4-801b-48e7-a10b-de44a2417265",
    "Sam Wolfe Connelly",
    CardRules::new_sorcery(mana_cost!("{4}{B}{B}")).with_ability(
        AbilityDef::spell_with_targets(
            "Return target creature card from your graveyard to your hand. Target player sacrifices a creature of their choice.",
            &[
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                }),
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                    PlayerRelation::Any,
                )),
            ],
            EffectDef::Sequence(&[
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
                EffectDef::SacrificeOfChoice {
                    player: EffectRecipientDef::Target(TargetIndex(1)),
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    count: ValueDef::Constant(1),
                    then: None,
                    amount: SacrificedAmountDef::Power,
                    otherwise: None,
                    optional: false,
                },
            ]),
        ),
    ),
);

// AVR 106 — Griselbrand
pub(in crate::card::sets) static GRISELBRAND: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Griselbrand",
    "b51666ae-2aef-4cb1-9cd4-44aec81530f8",
    "Igor Kieryluk",
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

// AVR 107 — Harvester of Souls
pub(in crate::card::sets) static HARVESTER_OF_SOULS: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Harvester of Souls",
    "505c0d25-dc1f-402e-9183-01c273efe0e1",
    "Slawomir Maniak",
    CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Demon"], 5, 5).with_abilities(&[
        abilities::deathtouch(),
        AbilityDef::triggered(
            "Whenever another nontoken creature dies, you may draw a card.",
            TriggerEventDef::zone_changed(
                // "Another nontoken creature", so a token dying is not a card and the Demon
                // stays quiet; the exclusion of itself is the other half.
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
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
pub(in crate::card::sets) static HOMICIDAL_SECLUSION: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Homicidal Seclusion",
    "022960a1-8223-4b83-aea2-85359c39f3b8",
    "Cliff Childs",
    CardRules::new_enchantment(mana_cost!("{4}{B}")).with_ability(AbilityDef::static_ability(
        "As long as you control exactly one creature, that creature gets +3/+1 and has \
         lifelink.",
        EffectDef::IfCondition {
            condition: &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::Equal,
                amount: 1,
            },
            then: &EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(1),
                    ),
                    AppliedEffectDef::add_ability(&abilities::lifelink()),
                ]),
            },
        },
    )),
);

// AVR 109 — Human Frailty
pub(in crate::card::sets) static HUMAN_FRAILTY: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Human Frailty",
    "1d1de712-86ac-4c03-be86-2403cd121f66",
    "David Palumbo",
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::destroy_target(
        "Destroy target Human creature.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Subtype("Human"),
        ])),
        true,
    )),
);

// AVR 110 — Hunted Ghoul
pub(in crate::card::sets) static HUNTED_GHOUL: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Hunted Ghoul",
    "bb443cf7-70bd-4df7-b446-107e099c19e3",
    "Ryan Pancoast",
    CardRules::new_creature(mana_cost!("{B}"), &["Zombie"], 1, 2).with_ability(
        AbilityDef::static_ability(
            "This creature can't block Humans.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::can_block_only(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Human")),
                )),
            },
        ),
    ),
);

// AVR 111 — Killing Wave
// Audit: unsupported — Needs a separate pay-X-life-or-sacrifice choice for the controller of every creature.
pub(in crate::card::sets) static KILLING_WAVE: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Killing Wave",
    "33de2371-175e-4f8a-9636-35f996e3cf24",
    "Steve Argyle",
    crate::card::CardRules::unsupported(),
);

// AVR 112 — Maalfeld Twins
pub(in crate::card::sets) static MAALFELD_TWINS: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Maalfeld Twins",
    "c63dd203-bce9-4ab7-8a0c-059d19d384e9",
    "Mike Sass",
    CardRules::new_creature(mana_cost!("{5}{B}"), &["Zombie"], 4, 4).with_ability(
        abilities::dies_trigger(
            "When this creature dies, create two 2/2 black Zombie creature tokens.",
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
pub(in crate::card::sets) static MARROW_BATS: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Marrow Bats",
    "38dcbad0-267e-411f-8e99-5d90b537bf9b",
    "Jason A. Engle",
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Bat", "Skeleton"], 4, 1).with_abilities(&[
        abilities::flying(),
        abilities::regenerate_self(
            "Pay 4 life: Regenerate this creature.",
            &[AbilityCostDef::PayLife(4)],
        ),
    ]),
);

// AVR 114 — Mental Agony
pub(in crate::card::sets) static MENTAL_AGONY: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Mental Agony",
    "4f8a1d51-aa7f-41fd-b97d-56bc48221615",
    "Greg Staples",
    CardRules::new_sorcery(mana_cost!("{3}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target player discards two cards and loses 2 life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::Discard {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ]),
    )),
);

// AVR 115 — Necrobite
pub(in crate::card::sets) static NECROBITE: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Necrobite",
    "52e59918-cf12-4d73-a4e0-31f38e792dc4",
    "Nils Hamm",
    CardRules::new_instant(mana_cost!("{2}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gains deathtouch until end of turn. Regenerate it.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::deathtouch()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Regenerate {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ]),
    )),
);

// AVR 116 — Polluted Dead
pub(in crate::card::sets) static POLLUTED_DEAD: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Polluted Dead",
    "036c1954-37d3-4787-8df8-f2d0dd39058a",
    "Jason A. Engle",
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Zombie"], 3, 3).with_ability(
        abilities::dies_trigger_with_targets(
            "When this creature dies, destroy target land.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Land),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
        ),
    ),
);

// AVR 117 — Predator's Gambit
pub(in crate::card::sets) static PREDATOR_S_GAMBIT: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Predator's Gambit",
    "88810a96-d5f8-4030-93f1-e2ad0d480317",
    "Zoltan Boros",
    CardRules::new_enchantment(mana_cost!("{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(1),
                    ),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature has intimidate as long as no other creatures are on the \
                 battlefield.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ObjectCount {
                        query: ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::AttachedToSource),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        ),
                        comparison: ComparisonDef::Equal,
                        amount: 0,
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::add_ability(&abilities::intimidate()),
                    },
                },
            ),
        ]),
);

// AVR 118 — Renegade Demon
pub(in crate::card::sets) static RENEGADE_DEMON: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Renegade Demon",
    "395696f8-9be2-4925-852f-b783850e1ca2",
    "Tomasz Jedruszek",
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Demon"], 5, 3),
);

// AVR 119 — Searchlight Geist
pub(in crate::card::sets) static SEARCHLIGHT_GEIST: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Searchlight Geist",
    "b0dc1a94-0193-464e-a481-730b34b57db5",
    "Steven Belledin",
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
pub(in crate::card::sets) static SOULCAGE_FIEND: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Soulcage Fiend",
    "dce1b1d3-9602-42bf-b341-d96976ff1e60",
    "Jason A. Engle",
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Demon"], 3, 2).with_ability(
        abilities::dies_trigger(
            "When this creature dies, each player loses 3 life.",
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::EachPlayer,
                amount: ValueDef::Constant(3),
            },
        ),
    ),
);

// AVR 121 — Treacherous Pit-Dweller
pub(in crate::card::sets) static TREACHEROUS_PIT_DWELLER: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Treacherous Pit-Dweller",
    "eec7dfd7-d7b2-44fa-b351-022a19fe81b8",
    "Svetlin Velinov",
    CardRules::new_creature(mana_cost!("{B}{B}"), &["Demon"], 4, 3).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "When this creature enters from a graveyard, target opponent gains control of it.",
            TriggerEventDef::ZoneChanged(ZoneChangeEventMatcherDef::new(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Graveyard),
                Some(ZoneKind::Battlefield),
            )),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::GainControl {
                object: EffectRecipientDef::Source,
                controller: PlayerRefDef::Target(TargetIndex::PRIMARY),
                duration: ControlDurationDef::Indefinitely,
            },
        ),
        abilities::undying(),
    ]),
);

// AVR 122 — Triumph of Cruelty
pub(in crate::card::sets) static TRIUMPH_OF_CRUELTY: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Triumph of Cruelty",
    "906618e2-2638-4017-9d6e-e6f282967a81",
    "Izzy",
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
pub(in crate::card::sets) static UNDEAD_EXECUTIONER: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Undead Executioner",
    "8d330058-16af-4486-aa89-b6be759e35d4",
    "Dave Kendall",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Zombie"], 2, 2).with_ability(
        abilities::dies_trigger_with_targets(
            "When this creature dies, you may have target creature get -2/-2 until end of turn.",
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
pub(in crate::card::sets) static UNHALLOWED_PACT: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Unhallowed Pact",
    "b26d73e6-9138-43b2-8031-6e3b25fa33f9",
    "Volkan Baǵa",
    CardRules::new_enchantment(mana_cost!("{2}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::triggered(
                "When enchanted creature dies, return that card to the battlefield under your control.",
                TriggerEventDef::ZoneChanged(ZoneChangeEventMatcherDef::new(
                    ObjectPredicateDef::AttachedToSource,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                )),
                EffectDef::WithBattlefieldArrival {
                    effect: &EffectDef::MoveToZone {
                        object: EffectRecipientDef::TriggeringZoneChangeResult,
                        zone: ZoneKind::Battlefield,
                        placement: ZonePlacement::Top,
                    },
                    arrival: crate::card::BattlefieldArrivalDef {
                        controller: Some(PlayerRelation::You),
                        ..crate::card::BattlefieldArrivalDef::DEFAULT
                    },
                },
            ),
        ]),
);

// AVR 125 — Aggravate
pub(in crate::card::sets) static AGGRAVATE: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Aggravate",
    "999f40a7-b723-42e1-83c1-f45a72a26dd4",
    "Matt Stewart",
    CardRules::new_instant(mana_cost!("{3}{R}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "This spell deals 1 damage to each creature target player controls. Each creature dealt damage this way attacks this turn if able.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                PlayerRelation::Any,
            ))],
            EffectDef::DealDamageAndApply {
                recipient: EffectRecipientDef::objects_controlled_by_target(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    TargetIndex::PRIMARY,
                ),
                amount: ValueDef::Constant(1),
                applied: AppliedEffectDef::add_ability(
                    &abilities::attacks_each_combat_if_able(
                        "This creature attacks this turn if able.",
                    ),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// AVR 126 — Archwing Dragon
pub(in crate::card::sets) static ARCHWING_DRAGON: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Archwing Dragon",
    "6c6f1a8b-329e-4094-8141-6bc88311a08c",
    "Daarken",
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
                object: EffectRecipientDef::Source,
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// AVR 127 — Banners Raised
pub(in crate::card::sets) static BANNERS_RAISED: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Banners Raised",
    "a7792df3-e2ab-4e60-abee-f24b72807107",
    "Mike Bierek",
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

// AVR 128 — Battle Hymn
pub(in crate::card::sets) static BATTLE_HYMN: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Battle Hymn",
    "43b5d46e-7054-44f8-9a14-b412f2f0ab86",
    "Nils Hamm",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell(
        "Add {R} for each creature you control.",
        EffectDef::AddManaEqualTo {
            color: ManaColor::Red,
            amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            )),
        },
    )),
);

// AVR 129 — Bonfire of the Damned
pub(in crate::card::sets) static BONFIRE_OF_THE_DAMNED: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Bonfire of the Damned",
    "e60610fe-891d-46de-b556-d03b637dccec",
    "James Paick",
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
// Audit: unsupported — Needs a spell additional cost that taps any number of chosen untapped creatures and retains that count for a three-times damage value.
pub(in crate::card::sets) static BURN_AT_THE_STAKE: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Burn at the Stake",
    "d46e9902-81fb-4a3c-9f2f-d3faf031631d",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

// AVR 131 — Dangerous Wager
pub(in crate::card::sets) static DANGEROUS_WAGER: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Dangerous Wager",
    "636c4042-703f-4548-9a0f-cb550c468bf9",
    "Drew Baker",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell(
        "Discard your hand, then draw two cards.",
        EffectDef::Sequence(&[
            EffectDef::Discard {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(i32::MAX),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ]),
    )),
);

// AVR 132 — Demolish (reprint)
const DEMOLISH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2001::odyssey::DEMOLISH,
    "4657aa15-8274-4bd7-afe4-504693064373",
    "Raymond Swanland",
);

// AVR 133 — Dual Casting
pub(in crate::card::sets) static DUAL_CASTING: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Dual Casting",
    "7aa45bfd-7075-470d-8aaa-16e34109eb5a",
    "Johannes Voss",
    CardRules::new_enchantment(mana_cost!("{1}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature has \"{R}, {T}: Copy target instant or sorcery spell you control. You may choose new targets for the copy.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&AbilityDef::activated_with_targets(
                        "{R}, {T}: Copy target instant or sorcery spell you control. You may choose new targets for the copy.",
                        &[
                            AbilityCostDef::Mana(mana_cost!("{R}")),
                            AbilityCostDef::TapSource,
                        ],
                        &[AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::AnyOf(&[
                                    ObjectPredicateDef::HasType(CardType::Instant),
                                    ObjectPredicateDef::HasType(CardType::Sorcery),
                                ]),
                                zones: &[ZoneKind::Stack],
                                controller: Some(PlayerRelation::You),
                                owner: None,
                            },
                        )],
                        EffectDef::CopyStackObject(&crate::card::CopyStackObjectDef {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            controller: PlayerRefDef::EffectController,
                            count: ValueDef::Constant(1),
                            retarget: true,
                            colors: None,
                        }),
                    )),
                },
            ),
        ]),
);

// AVR 134 — Falkenrath Exterminator
pub(in crate::card::sets) static FALKENRATH_EXTERMINATOR: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Falkenrath Exterminator",
    "40e23909-7e08-4686-ae59-e18e7d4cfd3c",
    "Winona Nelson",
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
pub(in crate::card::sets) static FERVENT_CATHAR: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Fervent Cathar",
    "0fceeb14-a22a-4ab6-9e6d-ba375b6865c0",
    "Steven Belledin",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Knight"], 2, 1).with_abilities(&[
        abilities::haste(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, target creature can't block this turn.",
            &ANY_CREATURE_TARGET,
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// AVR 136 — Gang of Devils
pub(in crate::card::sets) static GANG_OF_DEVILS: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Gang of Devils",
    "0430b9fa-3bc6-4183-ad5b-d70ad401fa97",
    "Erica Yang",
    CardRules::new_creature(mana_cost!("{5}{R}"), &["Devil"], 3, 3).with_ability(
        abilities::dies_trigger_with_targets("When this creature dies, it deals 3 damage divided as you choose among one, two, or three targets.", &[AbilityTargetDef {
                predicate: AbilityTargetPredicate::AnyTarget,
                minimum: 1,
                maximum: 3,
                exact_count: None,
                divided_total: Some(DividedTotal::Fixed(3)),
                another: false,
                excludes_source: false,
                chooser: TargetChooserDef::Controller,
            }], EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::DividedAmongTargets,
            }),
    ),
);

// AVR 137 — Guise of Fire
pub(in crate::card::sets) static GUISE_OF_FIRE: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Guise of Fire",
    "beb10d42-fa19-400c-bad8-ec3827f077bc",
    "Dave Kendall",
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

// AVR 138 — Hanweir Lancer
pub(in crate::card::sets) static HANWEIR_LANCER: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Hanweir Lancer",
    "73884fd1-5be9-4ad5-8c72-c0fbe27ad4c1",
    "Steve Prescott",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Knight"], 2, 2).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, both creatures have \\
             first strike.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &EffectDef::StaticApply {
                    recipient: SOULBOND_PAIR_RECIPIENT,
                    effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                },
            },
        ),
    ]),
);

// AVR 139 — Havengul Vampire
pub(in crate::card::sets) static HAVENGUL_VAMPIRE: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Havengul Vampire",
    "cbc09839-1463-40b8-86bd-fb96797b2633",
    "James Ryman",
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
pub(in crate::card::sets) static HEIRS_OF_STROMKIRK: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Heirs of Stromkirk",
    "ff89ad3b-b154-49e2-a0fd-135279512250",
    "Winona Nelson",
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
pub(in crate::card::sets) static HOUND_OF_GRISELBRAND: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Hound of Griselbrand",
    "0fe68bce-6207-4fd1-9e82-a18fd2d6ddca",
    "Svetlin Velinov",
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Elemental", "Dog"], 2, 2)
        .with_abilities(&[abilities::double_strike(), abilities::undying()]),
);

// AVR 142 — Kessig Malcontents
pub(in crate::card::sets) static KESSIG_MALCONTENTS: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Kessig Malcontents",
    "dce9a30f-a850-4826-a255-ce511d567b60",
    "John Stanko",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Warrior"], 3, 1).with_ability(
        abilities::enters_trigger_with_targets("When this creature enters, it deals damage to target player or planeswalker equal to the number of Humans you control.", &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )], EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::Subtype("Human"),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
            }),
    ),
);

// AVR 143 — Kruin Striker
pub(in crate::card::sets) static KRUIN_STRIKER: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Kruin Striker",
    "73e72249-84ea-4e9c-9f64-b67b02ffdf3a",
    "Christopher Moeller",
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

// AVR 144 — Lightning Mauler
pub(in crate::card::sets) static LIGHTNING_MAULER: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Lightning Mauler",
    "241cc968-b93e-4fe3-a66d-7776d29aa023",
    "Dave Kendall",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Berserker"], 2, 1).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, both creatures have \\
             haste.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &EffectDef::StaticApply {
                    recipient: SOULBOND_PAIR_RECIPIENT,
                    effect: AppliedEffectDef::add_ability(&abilities::haste()),
                },
            },
        ),
    ]),
);

// AVR 145 — Lightning Prowess
pub(in crate::card::sets) static LIGHTNING_PROWESS: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Lightning Prowess",
    "5578e3e2-2460-4dfb-9016-527463f2d918",
    "David Rapoza",
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
                        AppliedEffectDef::add_ability(&AbilityDef::activated_with_targets(
                            "{T}: This creature deals 1 damage to any target.",
                            &[AbilityCostDef::TapSource],
                            &[AbilityTargetDef::exactly_one(
                                AbilityTargetPredicate::AnyTarget,
                            )],
                            EffectDef::DealDamage {
                                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                amount: ValueDef::Constant(1),
                            },
                        )),
                    ]),
                },
            ),
        ]),
);

// AVR 146 — Mad Prophet
pub(in crate::card::sets) static MAD_PROPHET: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Mad Prophet",
    "172383d9-9135-4daa-a647-9d76435d3158",
    "Wayne Reynolds",
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

// AVR 147 — Malicious Intent
pub(in crate::card::sets) static MALICIOUS_INTENT: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Malicious Intent",
    "f1dda42d-55eb-46fc-89da-17cc5bfaa823",
    "Kev Walker",
    CardRules::new_enchantment(mana_cost!("{1}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature has \"{T}: Target creature can't block this turn.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    // Granted to the host rather than kept on the Aura, so the tap cost is the
                    // creature's own and "this turn" is measured from wherever it resolves.
                    effect: AppliedEffectDef::add_ability(&AbilityDef::activated_with_targets(
                        "{T}: Target creature can't block this turn.",
                        &[AbilityCostDef::TapSource],
                        &ANY_CREATURE_TARGET,
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    )),
                },
            ),
        ]),
);

// AVR 148 — Malignus
// Audit: unsupported — Needs a characteristic-defining half-highest-opponent-life value and a damage-prevention prohibition for the source.
pub(in crate::card::sets) static MALIGNUS: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Malignus",
    "8a6a7000-4a1d-4cd4-a85e-4b7b20d8e543",
    "Jung Park",
    crate::card::CardRules::unsupported(),
);

// AVR 149 — Pillar of Flame
pub(in crate::card::sets) static PILLAR_OF_FLAME: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Pillar of Flame",
    "c983e879-d9d2-47cc-9958-506711ca80cd",
    "Karl Kopinski",
    CardRules::new_sorcery(mana_cost!("{R}")).with_ability(AbilityDef::spell_with_targets(
            "Pillar of Flame deals 2 damage to any target. If a creature dealt damage this way would die this turn, exile it instead.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)],
            EffectDef::DealDamageAndApply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
                applied: AppliedEffectDef::Rule(AppliedRuleDef::ExileInsteadOfDying),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )),
);

// AVR 150 — Raging Poltergeist
pub(in crate::card::sets) static RAGING_POLTERGEIST: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Raging Poltergeist",
    "78833788-ffb2-43fc-9345-975f1cd46f38",
    "Slawomir Maniak",
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Spirit"], 6, 1),
);

// AVR 151 — Reforge the Soul
pub(in crate::card::sets) static REFORGE_THE_SOUL: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Reforge the Soul",
    "36506caa-2630-46ec-9aa0-e1885749ad90",
    "Jaime Jones",
    CardRules::new_sorcery(mana_cost!("{3}{R}{R}")).with_abilities(&[
        AbilityDef::spell(
            "Each player discards their hand, then draws seven cards.",
            EffectDef::Sequence(&[
                EffectDef::Discard {
                    recipient: EffectRecipientDef::EachPlayer,
                    amount: ValueDef::Constant(i32::MAX),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::EachPlayer,
                    amount: ValueDef::Constant(7),
                },
            ]),
        ),
        abilities::miracle(mana_cost!("{1}{R}")),
    ]),
);

// AVR 152 — Riot Ringleader
pub(in crate::card::sets) static RIOT_RINGLEADER: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Riot Ringleader",
    "c043f30b-548f-4c31-a415-0e59c2841dcf",
    "Gabor Szikszai",
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
// Audit: unsupported — Needs a chosen ordering of three permanent types and six sequential per-player sacrifice choices with retained mode order.
pub(in crate::card::sets) static RITE_OF_RUIN: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Rite of Ruin",
    "89e7fec3-94b5-411e-aeb0-44a64f517986",
    "Clint Cearley",
    crate::card::CardRules::unsupported(),
);

// AVR 154 — Rush of Blood
pub(in crate::card::sets) static RUSH_OF_BLOOD: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Rush of Blood",
    "a2884824-d138-47f2-913b-32cd475e9584",
    "Cynthia Sheppard",
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
pub(in crate::card::sets) static SCALDING_DEVIL: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Scalding Devil",
    "bbe49a97-dac8-4273-b4dc-45cdf8f5a6e0",
    "Erica Yang",
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
pub(in crate::card::sets) static SOMBERWALD_VIGILANTE: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Somberwald Vigilante",
    "9e69eb29-a7e4-4826-9535-9a073136c688",
    "John Stanko",
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

// AVR 157 — Stonewright
pub(in crate::card::sets) static STONEWRIGHT: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Stonewright",
    "9564d79d-5f4d-4192-94ee-5e5998011266",
    "Wesley Burt",
    CardRules::new_creature(mana_cost!("{R}"), &["Human", "Shaman"], 1, 1).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as Stonewright is paired with another creature, each of those creatures has \
             \"{R}: This creature gets +1/+0 until end of turn.\"",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &EffectDef::StaticApply {
                    recipient: SOULBOND_PAIR_RECIPIENT,
                    effect: AppliedEffectDef::add_ability(&AbilityDef::activated(
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
                    )),
                },
            },
        ),
    ]),
);

// AVR 158 — Thatcher Revolt
pub(in crate::card::sets) static THATCHER_REVOLT: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Thatcher Revolt",
    "ea28bd68-47a8-47c6-be16-75ae622daf0a",
    "Ryan Pancoast",
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_ability(AbilityDef::spell(
        "Create three 1/1 red Human creature tokens with haste. Sacrifice those tokens at the beginning of the next end step.",
        EffectDef::create_creature_token(&["Human"], &[ManaColor::Red], 1, 1)
            .with_abilities(&[abilities::haste()])
            .with_amount(3)
            .with_created_tokens(CreatedTokensDef {
                binding: ParentBinding,
                then: &EffectDef::InstallTrigger(InstalledTriggerDef::once(
                    &AbilityDef::triggered(
                        "At the beginning of the next end step, sacrifice those tokens.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::End,
                            player: PlayerRelation::Any,
                        },
                        EffectDef::Sacrifice {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                ParentBinding,
                            )),
                        },
                    ),
                )),
            }),
    )),
);

// AVR 159 — Thunderbolt (reprint)
const THUNDERBOLT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::weatherlight::THUNDERBOLT,
    "5845a5bc-6b7d-4bbb-80b3-a0f877b95553",
    "Anthony Francisco",
);

// AVR 160 — Thunderous Wrath
pub(in crate::card::sets) static THUNDEROUS_WRATH: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Thunderous Wrath",
    "daa39826-7f89-41cb-a7fe-7f7be817d5cd",
    "Adam Paquette",
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
// Audit: unsupported — Needs a target player's hand-size value; its other loyalty effects do not make the whole planeswalker exact without that value.
pub(in crate::card::sets) static TIBALT_THE_FIEND_BLOODED: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Tibalt, the Fiend-Blooded",
    "5ddb3936-8f5a-498d-a46c-27eb9546c76c",
    "Peter Mohrbacher",
    crate::card::CardRules::unsupported(),
);

// AVR 162 — Tyrant of Discord
// Audit: unsupported — Needs an opponent's random permanent choice and repeat-until-land sacrifice loop.
pub(in crate::card::sets) static TYRANT_OF_DISCORD: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Tyrant of Discord",
    "e8f7c8ea-e9c1-4d78-972c-15c4014915a0",
    "Richard Wright",
    crate::card::CardRules::unsupported(),
);

// AVR 163 — Uncanny Speed
pub(in crate::card::sets) static UNCANNY_SPEED: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Uncanny Speed",
    "1d7b747e-446a-4c25-9834-0be8476dc22d",
    "Raymond Swanland",
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
// Audit: unsupported — Needs an opponent choice on resolution, including which opponent in multiplayer, with a sacrifice branch only when one accepts the damage.
pub(in crate::card::sets) static VEXING_DEVIL: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Vexing Devil",
    "dbbefd98-4b17-4cc2-9ef9-8807f594cb16",
    "Lucas Graciano",
    crate::card::CardRules::unsupported(),
);

// AVR 165 — Vigilante Justice
pub(in crate::card::sets) static VIGILANTE_JUSTICE: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Vigilante Justice",
    "a9db329b-6248-4082-bfc8-5d2c0db43338",
    "Steve Prescott",
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

// AVR 166 — Zealous Conscripts
pub(in crate::card::sets) static ZEALOUS_CONSCRIPTS: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Zealous Conscripts",
    "fc027b11-1ecc-430d-a862-586a14bb23c3",
    "Steve Prescott",
    CardRules::new_creature(
        mana_cost!("{4}{R}"),
        &["Human", "Warrior"],
        3,
        3,
    )
    .with_abilities(&[
        abilities::haste(),
        abilities::enters_trigger_with_targets("When this creature enters, gain control of target permanent until end of turn. Untap that permanent. It gains haste until end of turn.", &[AbilityTargetDef::exactly_one_permanent(
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
                    // Haste matters here because the permanent has not been under its new
                    // controller's control since the turn began.
                    effect: AppliedEffectDef::add_ability(&abilities::haste()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ])),
    ]),
);

// AVR 167 — Abundant Growth
pub(in crate::card::sets) static ABUNDANT_GROWTH: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Abundant Growth",
    "afbc8fd0-dc15-4ac9-b97b-173f7fb66ed7",
    "Vincent Proce",
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
            abilities::enters_trigger(
                "When this Aura enters, draw a card.",
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted land has \"{T}: Add one mana of any color.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&AbilityDef::activated_mana(
                        "{T}: Add one mana of any color.",
                        &[AbilityCostDef::TapSource],
                        EffectDef::AddMana(AddManaEffectDef::any_color()),
                    )),
                },
            ),
        ]),
);

// AVR 168 — Blessings of Nature
// Audit: unsupported — Divided target shares are currently implemented only for damage; using them for counters resolves every counter amount as zero.
pub(in crate::card::sets) static BLESSINGS_OF_NATURE: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Blessings of Nature",
    "d16453f1-c6ca-4288-ab72-315ed9bb0ab0",
    "Anthony Francisco",
    crate::card::CardRules::unsupported(),
);

// AVR 169 — Borderland Ranger (reprint)
const BORDERLAND_RANGER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::BORDERLAND_RANGER,
    "8f067c26-c51d-44d0-a0af-106b5778f06a",
    "Zoltan Boros",
);

// AVR 170 — Bower Passage
pub(in crate::card::sets) static BOWER_PASSAGE: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Bower Passage",
    "b9f0048f-aaa0-4597-b898-ee754d0bbe4b",
    "Cliff Childs",
    CardRules::new_enchantment(mana_cost!("{1}{G}")).with_ability(AbilityDef::static_ability(
        "Creatures with flying can't block creatures you control.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
            )),
        },
    )),
);

// AVR 171 — Champion of Lambholt
pub(in crate::card::sets) static CHAMPION_OF_LAMBHOLT: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Champion of Lambholt",
    "e8ab9cd3-2faf-4500-a2ee-90b3a8d559c4",
    "Christopher Moeller",
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Human", "Warrior"], 1, 1)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Creatures with power less than this creature's power can't block creatures you control.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                        ObjectPredicateDef::PowerLessThan(ValueDef::SourcePower),
                    )),
                },
            ),
            AbilityDef::triggered(
                "Whenever another creature you control enters, put a +1/+1 counter on this creature.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    object: EffectRecipientDef::Source,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// AVR 172 — Craterhoof Behemoth
static CRATERHOOF_CREATURES: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

pub(in crate::card::sets) static CRATERHOOF_BEHEMOTH: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Craterhoof Behemoth",
    "a249be17-73ed-4108-89c0-f7e87939beb8",
    "Chris Rahn",
    CardRules::new_creature(mana_cost!("{5}{G}{G}{G}"), &["Beast"], 5, 5).with_abilities(&[
        abilities::haste(),
        abilities::enters_trigger("When this creature enters, creatures you control gain trample and get +X/+X until end of turn, where X is the number of creatures you control.", EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::You),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(ValueDef::CountMatchingObjects(&CRATERHOOF_CREATURES), ValueDef::CountMatchingObjects(&CRATERHOOF_CREATURES)),
                    AppliedEffectDef::add_ability(&abilities::trample()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            }),
    ]),
);

// AVR 173 — Descendants' Path
// Audit: unsupported — Needs a top-card reveal, shared-creature-type test, free-cast permission, and bottom placement when the card is not cast.
pub(in crate::card::sets) static DESCENDANTS_PATH: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Descendants' Path",
    "ada93208-5cda-4e2d-b9a7-15345e30b831",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// AVR 174 — Diregraf Escort
pub(in crate::card::sets) static DIREGRAF_ESCORT: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Diregraf Escort",
    "640e21ad-5064-41bc-886e-2c997f69a3f5",
    "Ryan Pancoast",
    CardRules::new_creature(mana_cost!("{G}"), &["Human", "Cleric"], 1, 1).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, both creatures have \
             protection from Zombies.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &EffectDef::StaticApply {
                    recipient: SOULBOND_PAIR_RECIPIENT,
                    effect: AppliedEffectDef::add_ability(&AbilityDef::keyword(
                        "Protection from Zombies",
                        KeywordAbility::ProtectionFrom(&ObjectPredicateDef::Subtype("Zombie")),
                    )),
                },
            },
        ),
    ]),
);

// AVR 175 — Druid's Familiar
pub(in crate::card::sets) static DRUIDS_FAMILIAR: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Druid's Familiar",
    "1d8c794a-7964-41f0-bc9c-27af7cf87aaa",
    "Adam Paquette",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Bear"], 2, 2).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, each of those creatures \\
             gets +2/+2.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &EffectDef::StaticApply {
                    recipient: SOULBOND_PAIR_RECIPIENT,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                },
            },
        ),
    ]),
);

// AVR 176 — Druids' Repository
pub(in crate::card::sets) static DRUIDS_REPOSITORY: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Druids' Repository",
    "57e6fb62-7ee3-444d-8fd4-c1f44014a05c",
    "Daarken",
    CardRules::new_enchantment(mana_cost!("{1}{G}{G}")).with_abilities(&[
        AbilityDef::triggered(
            "Whenever a creature you control attacks, put a charge counter on this enchantment.",
            TriggerEventDef::attacks(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("charge"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "Remove a charge counter from this enchantment: Add one mana of any color.",
            &[AbilityCostDef::RemoveCountersFromSource {
                kind: CounterKind::named("charge"),
                amount: 1,
            }],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// AVR 177 — Eaten by Spiders
// Audit: unsupported — Needs an attachment relation that finds and destroys every Equipment attached to the targeted creature.
pub(in crate::card::sets) static EATEN_BY_SPIDERS: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Eaten by Spiders",
    "0efea1b1-f212-4b97-98dd-922f85ab191f",
    "Slawomir Maniak",
    crate::card::CardRules::unsupported(),
);

// AVR 178 — Flowering Lumberknot
pub(in crate::card::sets) static FLOWERING_LUMBERKNOT: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Flowering Lumberknot",
    "78fa2ddc-142b-4562-8812-ecb72e3bae57",
    "Nic Klein",
    // A 5/5 for four that does nothing on its own, so the restriction is the
    // card rather than a footnote on it.
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Treefolk"], 5, 5).with_ability(
        AbilityDef::static_ability(
            "This creature can't attack or block unless it's paired with a creature with \
             soulbond.",
            EffectDef::IfCondition {
                // Only a creature with soulbond can start a pairing, so every pair contains
                // one -- which makes "paired at all" and "paired with a soulbond creature"
                // the same question for a Treefolk that has no soulbond of its own.
                condition: &TriggerConditionDef::SourceMatches {
                    object: ObjectPredicateDef::Unpaired,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
                        AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                    ]),
                },
            },
        ),
    ),
);

// AVR 179 — Geist Trappers
pub(in crate::card::sets) static GEIST_TRAPPERS: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Geist Trappers",
    "26f00a18-0ff7-42e4-be16-aa34fe27093b",
    "Anthony Palumbo",
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Human", "Warrior"], 3, 5).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, both creatures have \\
             reach.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &EffectDef::StaticApply {
                    recipient: SOULBOND_PAIR_RECIPIENT,
                    effect: AppliedEffectDef::add_ability(&abilities::reach()),
                },
            },
        ),
    ]),
);

// AVR 180 — Gloomwidow (reprint)
const GLOOMWIDOW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2008::shadowmoor::GLOOMWIDOW,
    "a016c872-09bd-42e1-94da-f587e8252492",
    "Svetlin Velinov",
);

// AVR 181 — Grounded
pub(in crate::card::sets) static GROUNDED: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Grounded",
    "dc4982f0-0ede-4846-82c8-bcf7ad63d099",
    "Greg Staples",
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

// AVR 182 — Howlgeist
pub(in crate::card::sets) static HOWLGEIST: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Howlgeist",
    "dad60d45-1c99-41d1-a237-c0ee18ce5361",
    "David Rapoza",
    CardRules::new_creature(mana_cost!("{5}{G}"), &["Spirit", "Wolf"], 4, 2).with_abilities(&[
        AbilityDef::static_ability(
            "Creatures with power less than this creature's power can't block it.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    WEAKER_THAN_SOURCE,
                )),
            },
        ),
        abilities::undying(),
    ]),
);

// AVR 183 — Joint Assault
// Audit: unsupported — Needs soulbond pairing state and the identity of the creature paired with the target.
pub(in crate::card::sets) static JOINT_ASSAULT: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Joint Assault",
    "6516c7c6-d49b-49c5-8968-563622c2c8c1",
    "Raymond Swanland",
    crate::card::CardRules::unsupported(),
);

// AVR 184 — Lair Delve
pub(in crate::card::sets) static LAIR_DELVE: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Lair Delve",
    "604948d8-6224-45ca-9ebb-d716644bbfd0",
    "Jason A. Engle",
    CardRules::new_sorcery(mana_cost!("{2}{G}")).with_ability(AbilityDef::spell(
        "Reveal the top two cards of your library. Put all creature and land cards revealed this way into your hand and the rest on the bottom of your library in any order.",
        EffectDef::RevealAndClassifyCards(crate::card::RevealAndClassifyCardsDef {
            source: ObjectCollectionSourceDef::TopCards {
                player: PlayerRefDef::EffectController,
                count: ValueDef::Constant(2),
            },
            object: ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasType(CardType::Land),
            ]),
            matching: Binding!("lair_delved_cards"),
            remainder: Binding!("lair_bottom_cards"),
            then: &EffectDef::Sequence(&[
                EffectDef::MoveObjects(MoveObjectsDef {
                    input: ObjectSetDef::Binding(Binding!("lair_delved_cards")),
                    from: Some(ZoneKind::Library),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    moved: None,
                    then: &EffectDef::None,
                }),
                EffectDef::ChooseObjectOrder(crate::card::ChooseObjectOrderDef {
                    actor: PlayerRefDef::EffectController,
                    input: ObjectSetDef::Binding(Binding!("lair_bottom_cards")),
                    ordered: ParentBinding,
                    placement: ZonePlacement::Bottom,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::MoveObjects(MoveObjectsDef {
                        input: ObjectSetDef::Binding(ParentBinding),
                        from: Some(ZoneKind::Library),
                        zone: ZoneKind::Library,
                        placement: ZonePlacement::Bottom,
                        moved: None,
                        then: &EffectDef::None,
                    }),
                }),
            ]),
        }),
    )),
);

// AVR 185 — Natural End
pub(in crate::card::sets) static NATURAL_END: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Natural End",
    "95d25235-de1c-4b67-9712-24f0564bd2bf",
    "Scott Chou",
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
                then: None,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ]),
    )),
);

// AVR 186 — Nettle Swine
pub(in crate::card::sets) static NETTLE_SWINE: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Nettle Swine",
    "75935f0e-9086-485b-b3e6-1a958fd0f2af",
    "Christopher Moeller",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Boar"], 4, 3),
);

// AVR 187 — Nightshade Peddler
pub(in crate::card::sets) static NIGHTSHADE_PEDDLER: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Nightshade Peddler",
    "4d3de66c-2283-458f-9d0d-943027520aa2",
    "John Stanko",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Druid"], 1, 1).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, both creatures have \\
             deathtouch.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &EffectDef::StaticApply {
                    recipient: SOULBOND_PAIR_RECIPIENT,
                    effect: AppliedEffectDef::add_ability(&abilities::deathtouch()),
                },
            },
        ),
    ]),
);

// AVR 188 — Pathbreaker Wurm
pub(in crate::card::sets) static PATHBREAKER_WURM: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Pathbreaker Wurm",
    "fe65eded-37ef-4cf0-b55c-390d34aab7b8",
    "Nils Hamm",
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Wurm"], 6, 4).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, both creatures have \\
             trample.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &EffectDef::StaticApply {
                    recipient: SOULBOND_PAIR_RECIPIENT,
                    effect: AppliedEffectDef::add_ability(&abilities::trample()),
                },
            },
        ),
    ]),
);

// AVR 189 — Primal Surge
// Audit: unsupported — Needs a repeatable top-card exile procedure with a permanent-card branch and a new optional decision on every iteration.
pub(in crate::card::sets) static PRIMAL_SURGE: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Primal Surge",
    "4278f7bd-afff-4a1d-a0bb-bf9ce3ad5a2e",
    "David Rapoza",
    crate::card::CardRules::unsupported(),
);

// AVR 190 — Rain of Thorns
pub(in crate::card::sets) static RAIN_OF_THORNS: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Rain of Thorns",
    "fd1cb530-b9d5-4386-b89e-2acecc8294c8",
    "Sam Burley",
    CardRules::new_sorcery(mana_cost!("{4}{G}{G}")).with_ability(
        AbilityDef::modal_spell(
            "Choose one or more —",
            &[
                AbilityDef::spell_with_targets(
                    "Destroy target artifact.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Artifact),
                    )],
                    EffectDef::Destroy {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        can_regenerate: true,
                        then: None,
                    },
                ),
                AbilityDef::spell_with_targets(
                    "Destroy target enchantment.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                    )],
                    EffectDef::Destroy {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        can_regenerate: true,
                        then: None,
                    },
                ),
                AbilityDef::spell_with_targets(
                    "Destroy target land.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Land),
                    )],
                    EffectDef::Destroy {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        can_regenerate: true,
                        then: None,
                    },
                ),
            ],
        )
        .with_mode_selection(1, 3, false),
    ),
);

// AVR 191 — Revenge of the Hunted
pub(in crate::card::sets) static REVENGE_OF_THE_HUNTED: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Revenge of the Hunted",
    "36f7d663-115c-4ad0-a072-633df054cce4",
    "Christopher Moeller",
    CardRules::new_sorcery(mana_cost!("{4}{G}{G}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature gets +6/+6 and gains trample until end of turn. All creatures able to block it this turn do so.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(6),
                        ValueDef::Constant(6),
                    ),
                    AppliedEffectDef::add_ability(&abilities::trample()),
                    AppliedEffectDef::Rule(AppliedRuleDef::MustBeBlockedBy(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    )),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::miracle(mana_cost!("{G}")),
    ]),
);

// AVR 192 — Sheltering Word
pub(in crate::card::sets) static SHELTERING_WORD: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Sheltering Word",
    "93cd9be4-1ce4-4a7c-b2a6-98d3fde0a92b",
    "Igor Kieryluk",
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
        // The grant and the life are one resolution, and the life is read from the
        // same slot the hexproof went to.
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::TargetToughness(TargetIndex::PRIMARY),
            },
        ]),
    )),
);

// AVR 193 — Snare the Skies
pub(in crate::card::sets) static SNARE_THE_SKIES: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Snare the Skies",
    "28f75827-a144-4fe2-a713-4439ae7567eb",
    "Ryan Yee",
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

// AVR 194 — Somberwald Sage
pub(in crate::card::sets) static SOMBERWALD_SAGE: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Somberwald Sage",
    "409c0272-7a43-4a6c-ab3f-740397b1f5c8",
    "Steve Argyle",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Druid"], 0, 1).with_ability(
        AbilityDef::activated_mana(
            "{T}: Add three mana of any one color. Spend this mana only to cast creature spells.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(
                AddManaEffectDef::any_color()
                    .with_amount(3)
                    .with_restrictions(&[ManaRestrictionDef::CastSpell(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    )]),
            ),
        ),
    ),
);

// AVR 195 — Soul of the Harvest
pub(in crate::card::sets) static SOUL_OF_THE_HARVEST: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Soul of the Harvest",
    "078f5e79-18dd-44e5-a930-8dc288f0b535",
    "Eytan Zana",
    // Nontoken, so a board full of tokens draws nothing -- the Soul rewards
    // creature cards rather than a token engine.
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Elemental"], 6, 6).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered(
            "Whenever another nontoken creature you control enters, you may draw a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
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
pub(in crate::card::sets) static TERRIFYING_PRESENCE: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Terrifying Presence",
    "2e8d0a22-f31b-45c0-85c7-0101aa63c77b",
    "Jaime Jones",
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
pub(in crate::card::sets) static TIMBERLAND_GUIDE: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Timberland Guide",
    "ae80fefb-af78-4f98-8058-71b61e91842f",
    "Zoltan Boros",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Scout"], 1, 1).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, put a +1/+1 counter on target creature.",
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
pub(in crate::card::sets) static TRIUMPH_OF_FEROCITY: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Triumph of Ferocity",
    "7bb41fa6-0cc6-43e5-9aa8-fcd9c781f4ce",
    "James Ryman",
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

// AVR 199 — Trusted Forcemage
pub(in crate::card::sets) static TRUSTED_FORCEMAGE: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Trusted Forcemage",
    "3ee66ef9-10a7-4aab-88f7-84956811cc6c",
    "Cynthia Sheppard",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Shaman"], 2, 2).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, each of those creatures \
             gets +1/+1.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &EffectDef::StaticApply {
                    recipient: SOULBOND_PAIR_RECIPIENT,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            },
        ),
    ]),
);

// AVR 200 — Ulvenwald Tracker
pub(in crate::card::sets) static ULVENWALD_TRACKER: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Ulvenwald Tracker",
    "46199391-a4f5-4532-b89c-b7691b229bd0",
    "Christopher Moeller",
    CardRules::new_creature(mana_cost!("{G}"), &["Human", "Shaman"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{1}{G}, {T}: Target creature you control fights another target creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{G}")),
                AbilityCostDef::TapSource,
            ],
            &[
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                }),
                AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Creature,
                ))
                .another(),
            ],
            EffectDef::Fight {
                first: ObjectRefDef::Target(TargetIndex::PRIMARY),
                second: ObjectRefDef::Target(TargetIndex(1)),
                excess: None,
            },
        ),
    ),
);

// AVR 201 — Vorstclaw
pub(in crate::card::sets) static VORSTCLAW: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Vorstclaw",
    "7591ee4f-9bfe-4419-84df-abf35d85bb94",
    "Lucas Graciano",
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Elemental", "Horror"], 7, 7),
);

// AVR 202 — Wandering Wolf
pub(in crate::card::sets) static WANDERING_WOLF: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Wandering Wolf",
    "ac606ad5-b8d0-4c93-a9de-5e41229a8229",
    "Tomasz Jedruszek",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Wolf"], 2, 1).with_abilities(&[
        AbilityDef::static_ability(
            "Creatures with power less than this creature's power can't block it.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    WEAKER_THAN_SOURCE,
                )),
            },
        ),
    ]),
);

// AVR 203 — Wild Defiance
// Audit: unsupported — Needs an event for a creature becoming the target of an instant or sorcery spell, carrying that creature as the effect recipient.
pub(in crate::card::sets) static WILD_DEFIANCE: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Wild Defiance",
    "7eaa6be7-2c6f-4442-85d1-ae31ad87fd98",
    "Slawomir Maniak",
    crate::card::CardRules::unsupported(),
);

// AVR 204 — Wildwood Geist
pub(in crate::card::sets) static WILDWOOD_GEIST: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Wildwood Geist",
    "4658b4b2-7043-4ca2-96fd-4f663c20c80f",
    "Lars Grant-West",
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Avatar", "Spirit"], 3, 3).with_ability(
        AbilityDef::static_ability(
            "This creature gets +2/+2 as long as it's your turn.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                },
            },
        ),
    ),
);

// AVR 205 — Wolfir Avenger
pub(in crate::card::sets) static WOLFIR_AVENGER: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Wolfir Avenger",
    "88cc00e5-9683-4ccc-a914-c422b76f6014",
    "Daniel Ljunggren",
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Wolf", "Warrior"], 3, 3).with_abilities(&[
        abilities::flash(),
        abilities::regenerate_self(
            "{1}{G}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{G}"))],
        ),
    ]),
);

// AVR 206 — Wolfir Silverheart
pub(in crate::card::sets) static WOLFIR_SILVERHEART: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Wolfir Silverheart",
    "8629c598-11c8-4911-acfb-0643e5feffa8",
    "Raymond Swanland",
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Wolf", "Warrior"], 4, 4).with_abilities(&[
        SOULBOND_ABILITIES[0],
        SOULBOND_ABILITIES[1],
        AbilityDef::static_ability(
            "As long as this creature is paired with another creature, each of those creatures \\
             gets +4/+4.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsPaired,
                then: &EffectDef::StaticApply {
                    recipient: SOULBOND_PAIR_RECIPIENT,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(4),
                        ValueDef::Constant(4),
                    ),
                },
            },
        ),
    ]),
);

// AVR 207 — Yew Spirit
pub(in crate::card::sets) static YEW_SPIRIT: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Yew Spirit",
    "b9320432-4f89-4363-91e6-2e740535cc2e",
    "Dan Murayama Scott",
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
// Audit: unsupported — Needs a blocking trigger and resolving choices of any number of legal Auras across the battlefield, hand, and graveyard to attach to the source.
pub(in crate::card::sets) static BRUNA_LIGHT_OF_ALABASTER: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Bruna, Light of Alabaster",
    "e390bc78-31ad-4131-a9e4-93ee0c7c2f34",
    "Winona Nelson",
    crate::card::CardRules::unsupported(),
);

// AVR 209 — Gisela, Blade of Goldnight
// Audit: unsupported — Needs global damage-event replacements that double opposing damage and prevent half of incoming damage with rounding.
pub(in crate::card::sets) static GISELA_BLADE_OF_GOLDNIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Gisela, Blade of Goldnight",
    "7208fcb8-83e6-44ce-af9a-ca1566825018",
    "Jason Chan",
    crate::card::CardRules::unsupported(),
);

// AVR 210 — Sigarda, Host of Herons
pub(in crate::card::sets) static SIGARDA_HOST_OF_HERONS: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Sigarda, Host of Herons",
    "feccd0e2-fae6-4ced-acdf-4252ed5c56e7",
    "Chris Rahn",
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
pub(in crate::card::sets) static ANGEL_S_TOMB: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Angel's Tomb",
    "28226303-7e67-4b88-adae-2386aff033ec",
    "Dan Murayama Scott",
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::triggered(
        "Whenever a creature you control enters, you may have this artifact become a 3/3 white Angel artifact creature with flying until end of turn.",
        TriggerEventDef::ZoneChanged(ZoneChangeEventMatcherDef::new(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ]),
            None,
            Some(ZoneKind::Battlefield),
        )),
        EffectDef::May {
            player: EffectRecipientDef::Controller,
            effect: &EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_card_types(crate::card::CardTypeSet::single(
                        CardType::Creature,
                    )),
                    AppliedEffectDef::add_colors(ColorSet::from_colors(&[ManaColor::White])),
                    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Angel"])),
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(3),
                    ),
                    AppliedEffectDef::add_ability(&abilities::flying()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        },
    )),
);

// AVR 212 — Angelic Armaments
pub(in crate::card::sets) static ANGELIC_ARMAMENTS: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Angelic Armaments",
    "3fa99b48-469d-4112-bdfd-2391fa439514",
    "Daniel Ljunggren",
    CardRules::new_artifact(mana_cost!("{3}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +2/+2, has flying, and is a white Angel in addition to its other colors and types.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                        AppliedEffectDef::add_colors(ColorSet::from_colors(&[ManaColor::White])),
                        AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Angel"])),
                    ]),
                },
            ),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{4}"))],
                "Equip {4} ({4}: Attach to target creature you control. Equip only as a \
                 sorcery.)",
            ),
        ]),
);

// AVR 213 — Bladed Bracers
pub(in crate::card::sets) static BLADED_BRACERS: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Bladed Bracers",
    "897a5116-043c-46aa-880e-be8dcb1618bc",
    "Ryan Yee",
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
                    condition: &TriggerConditionDef::AttachedPermanentMatches {
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::Subtype("Human"),
                            ObjectPredicateDef::Subtype("Angel"),
                        ]),
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::add_ability(&abilities::vigilance()),
                    },
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
pub(in crate::card::sets) static CONJURERS_CLOSET: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Conjurer's Closet",
    "7378e998-0382-42fc-8606-c6e7fc04b6a4",
    "Jason Felix",
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
// Audit: unsupported — Needs a joint mana-and-multi-permanent tap-cost planner so the three chosen Humans cannot be spent for mana while paying the same activation.
pub(in crate::card::sets) static GALLOWS_AT_WILLOW_HILL: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Gallows at Willow Hill",
    "8a840ee7-5728-4b1b-92ac-54612e5397b3",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// AVR 216 — Haunted Guardian
pub(in crate::card::sets) static HAUNTED_GUARDIAN: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Haunted Guardian",
    "7d97f8b8-bdb0-4d4b-b077-9affe2f9cd91",
    "Daniel Ljunggren",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Construct"], 2, 1)
        .with_abilities(&[abilities::defender(), abilities::first_strike()]),
);

// AVR 217 — Moonsilver Spear
pub(in crate::card::sets) static MOONSILVER_SPEAR: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Moonsilver Spear",
    "0b5efb85-1e5f-40ba-97b1-0ef6ac680330",
    "James Paick",
    CardRules::new_artifact(mana_cost!("{4}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature has first strike.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
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
pub(in crate::card::sets) static NARSTAD_SCRAPPER: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Narstad Scrapper",
    "f808ed9b-95ac-4069-bdca-b100bc816b5b",
    "Steven Belledin",
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
pub(in crate::card::sets) static OTHERWORLD_ATLAS: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Otherworld Atlas",
    "46e4aa67-4643-42ff-8172-200498686494",
    "Sam Wolfe Connelly",
    CardRules::new_artifact(mana_cost!("{4}"))
        .with_subtypes(&["Book"])
        .with_abilities(&[
            AbilityDef::activated(
                "{T}: Put a charge counter on this artifact.",
                &[AbilityCostDef::TapSource],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::named("charge"),
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::activated(
                "{T}: Each player draws a card for each charge counter on this artifact.",
                &[AbilityCostDef::TapSource],
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::EachPlayer,
                    amount: ValueDef::CountersOnSource(CounterKind::named("charge")),
                },
            ),
        ]),
);

// AVR 220 — Scroll of Avacyn
pub(in crate::card::sets) static SCROLL_OF_AVACYN: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Scroll of Avacyn",
    "871e6e2a-7e45-446b-b964-94377eb6ca92",
    "Cliff Childs",
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
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype("Angel"),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
                then: &EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(5),
                },
            },
        ]),
    )),
);

// AVR 221 — Scroll of Griselbrand
pub(in crate::card::sets) static SCROLL_OF_GRISELBRAND: CardRecord = CardRecord::new(
    crate::card::CardSet::AvacynRestored,
    "Scroll of Griselbrand",
    "2263ceaf-49d2-40fe-86b8-146271b11e46",
    "Cliff Childs",
    CardRules::new_artifact(mana_cost!("{1}")).with_ability(AbilityDef::activated_with_targets(
        "{1}, Sacrifice this artifact: Target opponent discards a card. If you control a Demon, that player loses 3 life.",
        &[
            AbilityCostDef::Mana(mana_cost!("{1}")),
            AbilityCostDef::SacrificeSource,
        ],
        &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
            PlayerRelation::Opponent,
        ))],
        EffectDef::Sequence(&[
            EffectDef::Discard {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
            EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ObjectCount {
                        query: ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype("Demon"),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 1,
                    },
                    then: &EffectDef::LoseLife {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(3),
                    },
            },
        ]),
    )),
);

// AVR 222 — Tormentor's Trident
pub(in crate::card::sets) static TORMENTORS_TRIDENT: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Tormentor's Trident",
    "9543d454-27d6-42ba-aad8-54811d180cfb",
    "Anthony Palumbo",
    // The requirement travels with the Equipment, so unequipping is how the
    // creature stops being forced to attack.
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +3/+0 and attacks each combat if able.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(3),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::add_ability(&abilities::attacks_each_combat_if_able(
                            "Attacks each combat if able",
                        )),
                    ]),
                },
            ),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{3}"))],
                "Equip {3} ({3}: Attach to target creature you control. Equip only as a \
                 sorcery.)",
            ),
        ]),
);

// AVR 223 — Vanguard's Shield
pub(in crate::card::sets) static VANGUARDS_SHIELD: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Vanguard's Shield",
    "ce8d9db6-5737-4a1f-ae4e-75821a602784",
    "Ryan Pancoast",
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +0/+3 and can block an additional creature each combat.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(0),
                            ValueDef::Constant(3),
                        ),
                        AppliedEffectDef::Rule(AppliedRuleDef::MayBlockAdditionalCreatures(1)),
                    ]),
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
pub(in crate::card::sets) static VESSEL_OF_ENDLESS_REST: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Vessel of Endless Rest",
    "ec733373-3f68-47ad-ac35-6f39092f1e26",
    "John Avon",
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        abilities::enters_trigger_with_targets("When this artifact enters, put target card from a graveyard on the bottom of its owner's library.", &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Any,
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: None,
            })], EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Library,
                placement: ZonePlacement::Bottom,
}),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// AVR 225 — Alchemist's Refuge
pub(in crate::card::sets) static ALCHEMISTS_REFUGE: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Alchemist's Refuge",
    "c767a897-52e3-4401-8104-930157bb2b02",
    "Dan Murayama Scott",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{G}{U}, {T}: You may cast spells this turn as though they had flash.",
            &[
                AbilityCostDef::Mana(mana_cost!("{G}{U}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayCastAsThoughItHadFlash(
                    crate::card::CastTimingPermissionDef::new(ObjectPredicateDef::Any),
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// AVR 226 — Cavern of Souls
pub(in crate::card::sets) static CAVERN_OF_SOULS: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Cavern of Souls",
    "1381c8f1-a292-4bdf-b20c-a5c2a169ee84",
    "Cliff Childs",
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
                .with_restrictions(&[ManaRestrictionDef::CastCreatureSpellOfChosenType])
                .with_spend_effects(&[ManaSpendEffectDef::ApplyToPaidSpell(
                        AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered),
                    )]),
            ),
        ),
    ]),
);

// AVR 227 — Desolate Lighthouse
pub(in crate::card::sets) static DESOLATE_LIGHTHOUSE: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Desolate Lighthouse",
    "16fb45bc-6152-4b01-9831-a8e80b1c1852",
    "Scott Chou",
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
pub(in crate::card::sets) static SERAPH_SANCTUARY: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Seraph Sanctuary",
    "f903b04a-2733-4ce7-9d83-9db8d5e1e10d",
    "David Palumbo",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_trigger(
            "When this land enters, you gain 1 life.",
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
pub(in crate::card::sets) static SLAYERS_STRONGHOLD: CardRecord = CardRecord::new(
    CardSet::AvacynRestored,
    "Slayers' Stronghold",
    "939a4351-3ec7-4e6c-8cdd-766bfd670391",
    "Karl Kopinski",
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

// AVR 230 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::PLAINS,
    "f090db87-b7a9-4c88-a211-495b27ae37c9",
    "Adam Paquette",
);

// AVR 231 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::PLAINS,
    1,
    "91348123-e2d0-4acb-ab4e-ec17652b7853",
    "Jung Park",
);

// AVR 232 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::PLAINS,
    2,
    "9ca16bd5-e261-4229-a92d-7cd55654dd11",
    "Eytan Zana",
);

// AVR 233 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ISLAND,
    "25934479-a47e-45b8-bc35-fc4b659b0d68",
    "James Paick",
);

// AVR 234 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::ISLAND,
    1,
    "22f920a5-74ea-4b94-8822-5867e6d5017a",
    "Adam Paquette",
);

// AVR 235 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::ISLAND,
    2,
    "5cf3685e-0ed3-4dc3-9033-8faa10b17c27",
    "Jung Park",
);

// AVR 236 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SWAMP,
    "339c3f05-444c-4e0a-a556-fc09417ee984",
    "James Paick",
);

// AVR 237 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::SWAMP,
    1,
    "26c57bee-2810-467c-8ed7-6cecb5cbc379",
    "Adam Paquette",
);

// AVR 238 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::SWAMP,
    2,
    "06a570b2-bcab-4500-b790-252baaf1f6d8",
    "Jung Park",
);

// AVR 239 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MOUNTAIN,
    "f53b7e7e-494b-4346-b18e-0e879bba7cec",
    "James Paick",
);

// AVR 240 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::MOUNTAIN,
    1,
    "94b728a4-c3a9-408e-8333-8266a02c64fa",
    "Adam Paquette",
);

// AVR 241 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::MOUNTAIN,
    2,
    "0e468809-d639-43f0-8834-16e921547dee",
    "Eytan Zana",
);

// AVR 242 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::FOREST,
    "8097c9ce-8fe9-4150-9810-52f6c92c6099",
    "James Paick",
);

// AVR 243 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::FOREST,
    1,
    "9f104987-f678-4ba4-b7f5-69ae7fdc01a3",
    "Jung Park",
);

// AVR 244 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::FOREST,
    2,
    "990f4974-fcef-46d1-8baa-6a215f7f3292",
    "Eytan Zana",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANGEL_OF_GLORY_S_RISE,
    &ANGEL_OF_JUBILATION,
    &AVACYN_ANGEL_OF_HOPE,
    &BANISHING_STROKE,
    &BUILDERS_BLESSING,
    &CALL_TO_SERVE,
    &CATHARS_CRUSADE,
    &CATHEDRAL_SANCTIFIER,
    &CLOUDSHIFT,
    &COMMANDERS_AUTHORITY,
    &CURSEBREAK,
    &DEFANG,
    &DEFY_DEATH,
    &DEVOUT_CHAPLAIN,
    &DIVINE_DEFLECTION,
    &EMANCIPATION_ANGEL,
    &ENTREAT_THE_ANGELS,
    &FARBOG_EXPLORER,
    &GOLDNIGHT_COMMANDER,
    &GOLDNIGHT_REDEEMER,
    &HERALD_OF_WAR,
    &HOLY_JUSTICIAR,
    &LEAP_OF_FAITH,
    &MIDNIGHT_DUELIST,
    &MIDVAST_PROTECTOR,
    &MOONLIGHT_GEIST,
    &MOORLAND_INQUISITOR,
    &NEARHEATH_PILGRIM,
    &RESTORATION_ANGEL,
    &RIDERS_OF_GAVONY,
    &RIGHTEOUS_BLOW,
    &SERAPH_OF_DAWN,
    &SILVERBLADE_PALADIN,
    &SPECTRAL_GATEGUARDS,
    &TERMINUS,
    &THRABEN_VALIANT,
    &VOICE_OF_THE_PROVINCES,
    &ZEALOUS_STRIKE,
    &ALCHEMISTS_APPRENTICE,
    &AMASS_THE_COMPONENTS,
    &ARCANE_MELEE,
    &CAPTAIN_OF_THE_MISTS,
    &CRIPPLING_CHILL,
    &DEADEYE_NAVIGATOR,
    &DEVASTATION_TIDE,
    &DREADWATERS,
    &ELGAUD_SHIELDMATE,
    &FAVORABLE_WINDS,
    &FETTERGEIST,
    &GALVANIC_ALCHEMIST,
    &GEIST_SNATCH,
    &GHOSTFORM,
    &GHOSTLY_FLICKER,
    &GHOSTLY_TOUCH,
    &GRYFF_VANGUARD,
    &HAVENGUL_SKAAB,
    &INFINITE_REFLECTION,
    &INTO_THE_VOID,
    &LATCH_SEEKER,
    &LONE_REVENANT,
    &LUNAR_MYSTIC,
    &MASS_APPEAL,
    &MIST_RAVEN,
    &MISTHOLLOW_GRIFFIN,
    &NEPHALIA_SMUGGLER,
    &OUTWIT,
    &ROTCROWN_GHOUL,
    &SCRAPSKIN_DRAKE,
    &SECOND_GUESS,
    &SPECTRAL_PRISON,
    &SPIRIT_AWAY,
    &STERN_MENTOR,
    &STOLEN_GOODS,
    &TAMIYO_THE_MOON_SAGE,
    &TANDEM_LOOKOUT,
    &TEMPORAL_MASTERY,
    &VANISHMENT,
    &WINGCRAFTER,
    &APPETITE_FOR_BRAINS,
    &BLOOD_ARTIST,
    &BLOODFLOW_CONNOISSEUR,
    &BUTCHER_GHOUL,
    &CORPSE_TRADERS,
    &DARK_IMPOSTOR,
    &DEATH_WIND,
    &DEMONIC_RISING,
    &DEMONIC_TASKMASTER,
    &DEMONLORD_OF_ASHMOUTH,
    &DESCENT_INTO_MADNESS,
    &DREAD_SLAVER,
    &DRIVER_OF_THE_DEAD,
    &ESSENCE_HARVEST,
    &EVERNIGHT_SHADE,
    &EXQUISITE_BLOOD,
    &GHOULFLESH,
    &GLOOM_SURGEON,
    &GRAVE_EXCHANGE,
    &GRISELBRAND,
    &HARVESTER_OF_SOULS,
    &HOMICIDAL_SECLUSION,
    &HUMAN_FRAILTY,
    &HUNTED_GHOUL,
    &KILLING_WAVE,
    &MAALFELD_TWINS,
    &MARROW_BATS,
    &MENTAL_AGONY,
    &NECROBITE,
    &POLLUTED_DEAD,
    &PREDATOR_S_GAMBIT,
    &RENEGADE_DEMON,
    &SEARCHLIGHT_GEIST,
    &SOULCAGE_FIEND,
    &TREACHEROUS_PIT_DWELLER,
    &TRIUMPH_OF_CRUELTY,
    &UNDEAD_EXECUTIONER,
    &UNHALLOWED_PACT,
    &AGGRAVATE,
    &ARCHWING_DRAGON,
    &BANNERS_RAISED,
    &BATTLE_HYMN,
    &BONFIRE_OF_THE_DAMNED,
    &BURN_AT_THE_STAKE,
    &DANGEROUS_WAGER,
    &DUAL_CASTING,
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
    &MALIGNUS,
    &PILLAR_OF_FLAME,
    &RAGING_POLTERGEIST,
    &REFORGE_THE_SOUL,
    &RIOT_RINGLEADER,
    &RITE_OF_RUIN,
    &RUSH_OF_BLOOD,
    &SCALDING_DEVIL,
    &SOMBERWALD_VIGILANTE,
    &STONEWRIGHT,
    &THATCHER_REVOLT,
    &THUNDEROUS_WRATH,
    &TIBALT_THE_FIEND_BLOODED,
    &TYRANT_OF_DISCORD,
    &UNCANNY_SPEED,
    &VEXING_DEVIL,
    &VIGILANTE_JUSTICE,
    &ZEALOUS_CONSCRIPTS,
    &ABUNDANT_GROWTH,
    &BLESSINGS_OF_NATURE,
    &BOWER_PASSAGE,
    &CHAMPION_OF_LAMBHOLT,
    &CRATERHOOF_BEHEMOTH,
    &DESCENDANTS_PATH,
    &DIREGRAF_ESCORT,
    &DRUIDS_FAMILIAR,
    &DRUIDS_REPOSITORY,
    &EATEN_BY_SPIDERS,
    &FLOWERING_LUMBERKNOT,
    &GEIST_TRAPPERS,
    &GROUNDED,
    &HOWLGEIST,
    &JOINT_ASSAULT,
    &LAIR_DELVE,
    &NATURAL_END,
    &NETTLE_SWINE,
    &NIGHTSHADE_PEDDLER,
    &PATHBREAKER_WURM,
    &PRIMAL_SURGE,
    &RAIN_OF_THORNS,
    &REVENGE_OF_THE_HUNTED,
    &SHELTERING_WORD,
    &SNARE_THE_SKIES,
    &SOMBERWALD_SAGE,
    &SOUL_OF_THE_HARVEST,
    &TERRIFYING_PRESENCE,
    &TIMBERLAND_GUIDE,
    &TRIUMPH_OF_FEROCITY,
    &TRUSTED_FORCEMAGE,
    &ULVENWALD_TRACKER,
    &VORSTCLAW,
    &WANDERING_WOLF,
    &WILD_DEFIANCE,
    &WILDWOOD_GEIST,
    &WOLFIR_AVENGER,
    &WOLFIR_SILVERHEART,
    &YEW_SPIRIT,
    &BRUNA_LIGHT_OF_ALABASTER,
    &GISELA_BLADE_OF_GOLDNIGHT,
    &SIGARDA_HOST_OF_HERONS,
    &ANGEL_S_TOMB,
    &ANGELIC_ARMAMENTS,
    &BLADED_BRACERS,
    &CONJURERS_CLOSET,
    &GALLOWS_AT_WILLOW_HILL,
    &HAUNTED_GUARDIAN,
    &MOONSILVER_SPEAR,
    &NARSTAD_SCRAPPER,
    &OTHERWORLD_ATLAS,
    &SCROLL_OF_AVACYN,
    &SCROLL_OF_GRISELBRAND,
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
    ANGELS_MERCY_REPRINT,
    ANGELIC_WALL_REPRINT,
    ARCHANGEL_REPRINT,
    FLEETING_DISTRACTION_REPRINT,
    PEEL_FROM_REALITY_REPRINT,
    BARTER_IN_BLOOD_REPRINT,
    BONE_SPLINTERS_REPRINT,
    CRYPT_CREEPER_REPRINT,
    DEMOLISH_REPRINT,
    THUNDERBOLT_REPRINT,
    BORDERLAND_RANGER_REPRINT,
    GLOOMWIDOW_REPRINT,
    PLAINS_REPRINT,
    PLAINS_ALTERNATE_1,
    PLAINS_ALTERNATE_2,
    ISLAND_REPRINT,
    ISLAND_ALTERNATE_1,
    ISLAND_ALTERNATE_2,
    SWAMP_REPRINT,
    SWAMP_ALTERNATE_1,
    SWAMP_ALTERNATE_2,
    MOUNTAIN_REPRINT,
    MOUNTAIN_ALTERNATE_1,
    MOUNTAIN_ALTERNATE_2,
    FOREST_REPRINT,
    FOREST_ALTERNATE_1,
    FOREST_ALTERNATE_2,
];
