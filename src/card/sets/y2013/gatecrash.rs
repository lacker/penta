//! Gatecrash card records used by the built-in ISD–M14 Standard decks.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::sets::y1998::stronghold as catalog_sth;
use crate::card::sets::y2001::odyssey as catalog_ody;
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    ActivationTimingDef, AddManaEffectDef, AppliedEffectDef, AppliedRuleDef, BasicLandType,
    BattlefieldEntryModificationDef, CardArt, CardRules, CardSet, CardSupertype, CardType,
    CardTypeSet, ChoiceVisibilityDef, ChooseDef, ColorChoiceOperationDef, ColorSet, ComparisonDef,
    ControlDurationDef, CounterKind, CreatureTypeSetDef, DamageEventMatcherDef, DamageKindDef,
    DamagePreventionDef, DamageRecipientMatcherDef, DamageSourceMatcherDef, DiscardSelectionDef,
    DividedTotal, EffectDef, EffectRecipientDef, InstalledTriggerDef, KeywordAbility, ManaColor,
    MillUntilDef, ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef,
    ObjectSetDef, PlayActionMatcherDef, PlayRestrictionDef, PlayerRefDef, PlayerRelation,
    QuantifierDef, ReplacementEffectDef, ReplacementEventDef, ResolvedEffectDurationDef,
    SacrificedAmountDef, SumValueDef, TopCardSelectionDef, TriggerConditionDef, TriggerEventDef,
    TurnPhaseDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::{ObjectBindingIndex, TargetIndex};
use crate::mana_cost;

static MILL_UNTIL_1: MillUntilDef = MillUntilDef {
    player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    object: ObjectPredicateDef::HasType(CardType::Land),
    matched_zone: ZoneKind::Graveyard,
    binding: None,
    then: None,
};

// GTC 1 — Aerial Maneuver
pub(in crate::card::sets) static AERIAL_MANEUVER: CardRecord = CardRecord::new_with_legacy_id(
    1054,
    "Aerial Maneuver",
    CardArt::new("c63e9c49-3fa4-41ad-9eed-19801df103c6", "Scott Chou"),
    CardSet::Gatecrash,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +1/+1 and gains flying and first strike until end of turn.",
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
                AppliedEffectDef::add_ability(&abilities::flying()),
                AppliedEffectDef::add_ability(&abilities::first_strike()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// GTC 2 — Angelic Edict
pub(in crate::card::sets) static ANGELIC_EDICT: CardRecord = CardRecord::new_with_legacy_id(
    1055,
    "Angelic Edict",
    CardArt::new("e24b62d3-c200-4330-a255-92d77f01ba44", "Trevor Claxton"),
    CardSet::Gatecrash,
    CardRules::new_sorcery(mana_cost!("{4}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Exile target creature or enchantment.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasType(CardType::Enchantment),
            ]),
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
    )),
);

// GTC 3 — Angelic Skirmisher
// Audit: metadata-only — Needs a resolving keyword choice on a beginning-of-combat trigger and a temporary mass grant of the chosen ability.
pub(in crate::card::sets) static ANGELIC_SKIRMISHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("beb04702-5cb2-4590-b675-9409ba52a395"),
    "Angelic Skirmisher",
    crate::card::CardArt::new("beb04702-5cb2-4590-b675-9409ba52a395", "David Rapoza"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 4 — Assault Griffin
pub(in crate::card::sets) static ASSAULT_GRIFFIN: CardRecord = CardRecord::new_with_legacy_id(
    1056,
    "Assault Griffin",
    CardArt::new("704286a5-e3a8-4517-85e5-6447c5c2530f", "Eric Velhagen"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Griffin"], 3, 2)
        .with_ability(abilities::flying()),
);

// GTC 5 — Basilica Guards
pub(in crate::card::sets) static BASILICA_GUARDS: CardRecord = CardRecord::new_with_legacy_id(
    1057,
    "Basilica Guards",
    CardArt::new("2be39fed-4b39-4027-9c80-f2186f7dd941", "Dan Murayama Scott"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 1, 4)
        .with_abilities(&[abilities::defender(), abilities::extort()]),
);

// GTC 6 — Blind Obedience
pub(in crate::card::sets) static BLIND_OBEDIENCE: CardRecord = CardRecord::new_with_legacy_id(
    141,
    "Blind Obedience",
    CardArt::new("07c3e78d-d917-4552-842f-feff99c059e0", "Seb McKinnon"),
    CardSet::Gatecrash,
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_abilities(&[
        abilities::extort(),
        AbilityDef::replacement_for(
            "Artifacts and creatures your opponents control enter tapped.",
            ReplacementEventDef::ObjectEntersBattlefield {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]),
                controller: PlayerRelation::Opponent,
                cast: None,
            },
            ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
        ),
    ]),
);

// GTC 7 — Boros Elite
pub(in crate::card::sets) static BOROS_ELITE: CardRecord = CardRecord::new_with_legacy_id(
    1507,
    "Boros Elite",
    CardArt::new("a03974e6-aced-4664-8c5c-3190bb1eb233", "Willian Murai"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 1, 1).with_abilities(&[
        abilities::battalion(
            "Battalion — Whenever this creature and at least two other creatures attack, this creature gets +2/+2 until end of turn.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// GTC 8 — Court Street Denizen
pub(in crate::card::sets) static COURT_STREET_DENIZEN: CardRecord = CardRecord::new_with_legacy_id(
    1058,
    "Court Street Denizen",
    CardArt::new("ca6a5cb3-b6e5-4879-83b5-4ad590a5467a", "Volkan Baǵa"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 2, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever another white creature you control enters, tap target creature an opponent controls.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::White),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]), None, Some(ZoneKind::Battlefield)),
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::Opponent),
                owner: None,
            })],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ),
);

// GTC 9 — Daring Skyjek
pub(in crate::card::sets) static DARING_SKYJEK: CardRecord = CardRecord::new_with_legacy_id(
    1508,
    "Daring Skyjek",
    CardArt::new("6c28412d-9add-4911-8487-c84559006fb0", "Jason Chan"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Knight"], 3, 1).with_abilities(&[
        abilities::battalion(
            "Battalion — Whenever this creature and at least two other creatures attack, this creature gains flying until end of turn.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&BATTALION_FLYING),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

static DEBTORS_PULPIT_TAP: AbilityDef = AbilityDef::activated_with_targets(
    "{T}: Tap target creature.",
    &[AbilityCostDef::TapSource],
    &[AbilityTargetDef::exactly_one_permanent(
        ObjectPredicateDef::HasType(CardType::Creature),
    )],
    EffectDef::Tap {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
);

// GTC 10 — Debtor's Pulpit
pub(in crate::card::sets) static DEBTORS_PULPIT: CardRecord = CardRecord::new_with_legacy_id(
    1059,
    "Debtor's Pulpit",
    CardArt::new("fafb0372-6860-4b0b-b92e-873735489006", "James Paick"),
    CardSet::Gatecrash,
    CardRules::new_enchantment(mana_cost!("{4}{W}"))
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
            AbilityDef::static_ability(
                "Enchanted land has \"{T}: Tap target creature.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&DEBTORS_PULPIT_TAP),
                },
            ),
        ]),
);

// GTC 11 — Dutiful Thrull
pub(in crate::card::sets) static DUTIFUL_THRULL: CardRecord = CardRecord::new_with_legacy_id(
    1494,
    "Dutiful Thrull",
    CardArt::new("8d586143-fac0-463f-96ec-c6b9fd582194", "Daarken"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{W}"), &["Thrull"], 1, 1).with_ability(
        abilities::regenerate_self(
            "{B}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
        ),
    ),
);

// GTC 12 — Frontline Medic
// Audit: metadata-only — Battalion needs a trigger-time-only condition, and no target predicate recognizes a spell with X in its mana cost.
pub(in crate::card::sets) static FRONTLINE_MEDIC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("711a5eca-531d-4b07-b7df-9b06bad491be"),
    "Frontline Medic",
    crate::card::CardArt::new("711a5eca-531d-4b07-b7df-9b06bad491be", "Willian Murai"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 13 — Gideon, Champion of Justice
// Audit: metadata-only — Needs dynamic loyalty addition, loyalty-sized animation, all-damage prevention, and mass exile of every other permanent.
pub(in crate::card::sets) static GIDEON_CHAMPION_OF_JUSTICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("93d0509a-a863-4d9c-b39f-625a8cc1a547"),
    "Gideon, Champion of Justice",
    crate::card::CardArt::new("93d0509a-a863-4d9c-b39f-625a8cc1a547", "David Rapoza"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 14 — Guardian of the Gateless
// Audit: metadata-only — Needs blocking any number of creatures and a trigger amount equal to how many creatures the source is blocking.
pub(in crate::card::sets) static GUARDIAN_OF_THE_GATELESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("86940635-7001-4ecf-b4ee-25aa1e2d81fc"),
    "Guardian of the Gateless",
    crate::card::CardArt::new("86940635-7001-4ecf-b4ee-25aa1e2d81fc", "Wesley Burt"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

static GUILDSCORN_WARD_PROTECTION: AbilityDef = abilities::protection_from_multicolored();

// GTC 15 — Guildscorn Ward
pub(in crate::card::sets) static GUILDSCORN_WARD: CardRecord = CardRecord::new_with_legacy_id(
    1911,
    "Guildscorn Ward",
    CardArt::new("89c5c496-0a3e-40e1-84ac-8ad3a9d8352b", "Ryan Barger"),
    CardSet::Gatecrash,
    // A guild card is two colors and is stopped; a mono-colored one of either
    // of those colors is not, which is what makes this a Ravnica sideboard
    // card rather than a colour hoser.
    CardRules::new_enchantment(mana_cost!("{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature has protection from multicolored.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&GUILDSCORN_WARD_PROTECTION),
                },
            ),
        ]),
);

static HOLD_THE_GATES_GATES: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::Subtype("Gate"),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

// GTC 16 — Hold the Gates
pub(in crate::card::sets) static HOLD_THE_GATES: CardRecord = CardRecord::new_with_legacy_id(
    1060,
    "Hold the Gates",
    CardArt::new("48fd52d0-0e41-48d5-b96f-4c6409788c18", "Zoltan Boros"),
    CardSet::Gatecrash,
    CardRules::new_enchantment(mana_cost!("{2}{W}")).with_ability(AbilityDef::static_ability(
        "Creatures you control get +0/+1 for each Gate you control and have vigilance.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::CountMatchingObjects(&HOLD_THE_GATES_GATES),
                ),
                AppliedEffectDef::add_ability(&abilities::vigilance()),
            ]),
        },
    )),
);

static HOLY_MANTLE_PROTECTION: AbilityDef = AbilityDef::keyword(
    "Protection from creatures",
    KeywordAbility::ProtectionFrom(&ObjectPredicateDef::HasType(CardType::Creature)),
);

static HOLY_MANTLE_GRANT: [AppliedEffectDef; 2] = [
    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
    AppliedEffectDef::add_ability(&HOLY_MANTLE_PROTECTION),
];

// GTC 17 — Holy Mantle
pub(in crate::card::sets) static HOLY_MANTLE: CardRecord = CardRecord::new_with_legacy_id(
    1910,
    "Holy Mantle",
    CardArt::new("95567596-c5b1-426f-bc2c-43306f7221b0", "Maciej Kuciara"),
    CardSet::Gatecrash,
    // The Aura is an enchantment, so the protection it grants does not make
    // its own attachment illegal.
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+2 and has protection from creatures.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&HOLY_MANTLE_GRANT),
                },
            ),
        ]),
);

// GTC 18 — Knight of Obligation
pub(in crate::card::sets) static KNIGHT_OF_OBLIGATION: CardRecord = CardRecord::new_with_legacy_id(
    1061,
    "Knight of Obligation",
    CardArt::new("0c2a1100-a2e6-4ef5-a8e3-2aca552d6b66", "Ryan Barger"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Knight"], 2, 4)
        .with_abilities(&[abilities::vigilance(), abilities::extort()]),
);

// GTC 19 — Knight Watch
pub(in crate::card::sets) static KNIGHT_WATCH: CardRecord = CardRecord::new_with_legacy_id(
    1062,
    "Knight Watch",
    CardArt::new("cd492072-9a8c-4d55-ac71-3c8efaa3fc87", "Matt Stewart"),
    CardSet::Gatecrash,
    CardRules::new_sorcery(mana_cost!("{4}{W}")).with_ability(AbilityDef::spell(
        "Create two 2/2 white Knight creature tokens with vigilance.",
        EffectDef::create_creature_token(&["Knight"], &[ManaColor::White], 2, 2)
            .with_abilities(&[abilities::vigilance()])
            .with_art(CardArt::new(
                "67d3d039-248a-4eb8-be5c-12959b458fea",
                "Matt Stewart",
            ))
            .with_amount(2),
    )),
);

// GTC 20 — Luminate Primordial
pub(in crate::card::sets) static LUMINATE_PRIMORDIAL: CardRecord = CardRecord::new_with_legacy_id(
    1063,
    "Luminate Primordial",
    CardArt::new(
        "b0747b12-c75a-4fdf-a881-f2383a23ccdd",
        "Stephan Martiniere",
    ),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{5}{W}{W}"), &["Avatar"], 4, 7).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::triggered_with_targets(
            "When this creature enters, for each opponent, exile up to one target creature that player controls and that player gains life equal to its power.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)),
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
                1,
            )],
            EffectDef::Sequence(&[
                EffectDef::MoveToZone {
                    counters: None,
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Exile,
                    placement: ZonePlacement::Top,
                    arrival_effect: None,
                    attachment: None,
                    controller: None,
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                    amount: ValueDef::TargetPower(TargetIndex::PRIMARY),
                },
            ]),
        ),
    ]),
);

/// "Enchant creature you control."
static ENCHANT_YOUR_CREATURE: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    },
)];

/// "When enchanted creature dies ..." -- the attached permanent moving from
/// the battlefield to a graveyard, whatever caused it.
static ENCHANTED_CREATURE_DIES: TriggerEventDef = TriggerEventDef::zone_changed(
    ObjectPredicateDef::AttachedToSource,
    Some(ZoneKind::Battlefield),
    Some(ZoneKind::Graveyard),
);

// GTC 21 — Murder Investigation
pub(in crate::card::sets) static MURDER_INVESTIGATION: CardRecord = CardRecord::new_with_legacy_id(
    1611,
    "Murder Investigation",
    CardArt::new("1f3bb284-d10e-4265-92a4-8dcaf118f3c8", "Igor Kieryluk"),
    CardSet::Gatecrash,
    CardRules::new_enchantment(mana_cost!("{1}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature you control", &ENCHANT_YOUR_CREATURE),
            AbilityDef::triggered(
                "When enchanted creature dies, create X 1/1 white Soldier creature tokens, \
                 where X is its power.",
                ENCHANTED_CREATURE_DIES,
                EffectDef::create_creature_token(&["Soldier"], &[ManaColor::White], 1, 1)
                    .with_art(CardArt::new(
                        "944a40e8-5469-4d8b-b044-67ff3382ec92",
                        "Steve Prescott",
                    ))
                    .with_count(ValueDef::TriggeringObjectPower),
            ),
        ]),
);

// GTC 22 — Nav Squad Commandos
pub(in crate::card::sets) static NAV_SQUAD_COMMANDOS: CardRecord = CardRecord::new_with_legacy_id(
    1509,
    "Nav Squad Commandos",
    CardArt::new("9d81d7f8-375f-40f5-98cd-08be08580bef", "Steve Prescott"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Human", "Soldier"], 3, 5).with_ability(
        abilities::battalion(
            "Battalion — Whenever this creature and at least two other creatures attack, this \
             creature gets +1/+1 until end of turn. Untap it.",
            EffectDef::Sequence(&NAV_SQUAD_BATTALION),
        ),
    ),
);

static NAV_SQUAD_BATTALION: [EffectDef; 2] = [
    EffectDef::Apply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::modify_power_toughness(
            ValueDef::Constant(1),
            ValueDef::Constant(1),
        ),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
    EffectDef::Untap {
        object: EffectRecipientDef::Source,
    },
];

// GTC 23 — Righteous Charge
pub(in crate::card::sets) static RIGHTEOUS_CHARGE: CardRecord = CardRecord::new_with_legacy_id(
    1064,
    "Righteous Charge",
    CardArt::new("f52cb325-4f16-4cf3-9999-feafe0fde8c2", "Svetlin Velinov"),
    CardSet::Gatecrash,
    CardRules::new_sorcery(mana_cost!("{1}{W}{W}")).with_ability(AbilityDef::spell(
        "Creatures you control get +2/+2 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(2),
                ValueDef::Constant(2),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// GTC 24 — Shielded Passage
pub(in crate::card::sets) static SHIELDED_PASSAGE: CardRecord = CardRecord::new_with_legacy_id(
    1497,
    "Shielded Passage",
    CardArt::new("6546b6c4-73b2-41b3-9ff9-316e9ce916e5", "Raymond Swanland"),
    CardSet::Gatecrash,
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Prevent all damage that would be dealt to target creature this turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::PreventDamage {
            prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::to(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
            )),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// GTC 25 — Smite (reprint)

// GTC 26 — Syndic of Tithes
pub(in crate::card::sets) static SYNDIC_OF_TITHES: CardRecord = CardRecord::new_with_legacy_id(
    1065,
    "Syndic of Tithes",
    CardArt::new("2bafaa3b-eeaa-427f-9a73-6a1c98d257ca", "Steve Prescott"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Cleric"], 2, 2)
        .with_ability(abilities::extort()),
);

// GTC 27 — Urbis Protector
pub(in crate::card::sets) static URBIS_PROTECTOR: CardRecord = CardRecord::new_with_legacy_id(
    1066,
    "Urbis Protector",
    CardArt::new("acf932ac-5ea5-491b-b555-5e9ea971d93d", "Steve Argyle"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{4}{W}{W}"), &["Human", "Cleric"], 1, 1).with_ability(
        AbilityDef::triggered(
            "When this creature enters, create a 4/4 white Angel creature token with flying.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::create_creature_token(&["Angel"], &[ManaColor::White], 4, 4)
                .with_abilities(&[abilities::flying()])
                .with_art(CardArt::new(
                    "71766a5a-ce00-4e48-b4f6-0d1a7f5b2691",
                    "Steve Argyle",
                )),
        ),
    ),
);

// GTC 28 — Zarichi Tiger
pub(in crate::card::sets) static ZARICHI_TIGER: CardRecord = CardRecord::new_with_legacy_id(
    1067,
    "Zarichi Tiger",
    CardArt::new("7bf5efe4-d9a0-4704-b5ba-3213c946df37", "Nic Klein"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Cat"], 2, 3).with_ability(
        AbilityDef::activated(
            "{1}{W}, {T}: You gain 2 life.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{W}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

// GTC 29 — Aetherize
// Audit: metadata-only — Needs a simultaneous multi-object move to return all attacking creatures without resolving the zone changes sequentially.
pub(in crate::card::sets) static AETHERIZE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("33303859-c6e0-4ebd-bb5f-44be7f5d7459"),
    "Aetherize",
    crate::card::CardArt::new("33303859-c6e0-4ebd-bb5f-44be7f5d7459", "Ryan Barger"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 30 — Agoraphobia
pub(in crate::card::sets) static AGORAPHOBIA: CardRecord = CardRecord::new_with_legacy_id(
    1068,
    "Agoraphobia",
    CardArt::new("e1a3efab-ee0a-4770-a323-e4bac38e4287", "Jim Murray"),
    CardSet::Gatecrash,
    CardRules::new_enchantment(mana_cost!("{1}{U}"))
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
                "Enchanted creature gets -5/-0.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-5),
                        ValueDef::Constant(0),
                    ),
                },
            ),
            AbilityDef::activated(
                "{2}{U}: Return this Aura to its owner's hand.",
                &[AbilityCostDef::Mana(mana_cost!("{2}{U}"))],
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

// GTC 31 — Clinging Anemones
pub(in crate::card::sets) static CLINGING_ANEMONES: CardRecord = CardRecord::new_with_legacy_id(
    1536,
    "Clinging Anemones",
    CardArt::new("4e183069-096d-4977-8154-e7b60f17a787", "Mike Bierek"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Jellyfish"], 1, 4)
        .with_abilities(&[abilities::defender(), abilities::evolve()]),
);

// GTC 32 — Cloudfin Raptor
pub(in crate::card::sets) static CLOUDFIN_RAPTOR: CardRecord = CardRecord::new_with_legacy_id(
    1537,
    "Cloudfin Raptor",
    CardArt::new("e2931f27-43f9-4e52-aab3-967c26739e43", "Peter Mohrbacher"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{U}"), &["Bird", "Mutant"], 0, 1)
        .with_abilities(&[abilities::flying(), abilities::evolve()]),
);

// GTC 33 — Diluvian Primordial
// Audit: metadata-only — Needs casting a targeted graveyard card without paying its mana cost and replacing that spell card's later graveyard move with exile.
pub(in crate::card::sets) static DILUVIAN_PRIMORDIAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0b7c34af-91de-44c6-a3e2-f48dbb0ce9fd"),
    "Diluvian Primordial",
    crate::card::CardArt::new("0b7c34af-91de-44c6-a3e2-f48dbb0ce9fd", "Stephan Martiniere"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 34 — Enter the Infinite
// Audit: metadata-only — Needs a dynamic library-sized draw, a non-target hand choice to put on top, and a temporary no-maximum-hand-size rule.
pub(in crate::card::sets) static ENTER_THE_INFINITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("612beb8f-2ab1-4a8b-84c5-c47d19d400ab"),
    "Enter the Infinite",
    crate::card::CardArt::new("612beb8f-2ab1-4a8b-84c5-c47d19d400ab", "Terese Nielsen"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 35 — Frilled Oculus
pub(in crate::card::sets) static FRILLED_OCULUS: CardRecord = CardRecord::new_with_legacy_id(
    1847,
    "Frilled Oculus",
    CardArt::new("d9f3a08f-403e-4d6c-87c7-add8170bde8b", "Marco Nelor"),
    CardSet::Gatecrash,
    // The ration is the whole cost of the card: without it, a mana-hungry
    // pump would be limited only by how much green is available.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Homunculus"], 1, 3).with_ability(
        AbilityDef::activated(
            "{1}{G}: This creature gets +2/+2 until end of turn. Activate only once each turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .once_each_turn(),
    ),
);

/// "X target nonland permanents": the count is the X that was paid, so an X
/// larger than the board offers no declaration rather than tapping fewer.
static GRIDLOCK_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_chosen_x(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
)];

// GTC 36 — Gridlock
pub(in crate::card::sets) static GRIDLOCK: CardRecord = CardRecord::new_with_legacy_id(
    1848,
    "Gridlock",
    CardArt::new("b4f5c126-3df9-4771-9e74-4ca33161ac08", "Yeong-Hao Han"),
    CardSet::Gatecrash,
    CardRules::new_instant(mana_cost!("{X}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Tap X target nonland permanents.",
        &GRIDLOCK_TARGETS,
        EffectDef::Tap {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    )),
);

// GTC 37 — Hands of Binding
// Audit: metadata-only — Needs the next-untap-step skip effect and the cipher encoding and free-copy-casting procedure.
pub(in crate::card::sets) static HANDS_OF_BINDING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("afeef50a-f5c9-47ab-ad04-645f49bbae6b"),
    "Hands of Binding",
    crate::card::CardArt::new("afeef50a-f5c9-47ab-ad04-645f49bbae6b", "Raymond Swanland"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

/// Exactly the second, not the second or later: the spell that caused the
/// trigger has already been counted by the time this is read.
static INCURSION_SPECIALIST_SECOND_SPELL: TriggerConditionDef =
    TriggerConditionDef::SpellsCastThisTurn {
        quantifier: QuantifierDef::Any,
        player: PlayerRelation::You,
        comparison: ComparisonDef::Equal,
        amount: 2,
    };

// GTC 38 — Incursion Specialist
pub(in crate::card::sets) static INCURSION_SPECIALIST: CardRecord = CardRecord::new_with_legacy_id(
    2014,
    "Incursion Specialist",
    CardArt::new("290e56e0-e699-413a-9d6a-e740bf460b35", "Svetlin Velinov"),
    CardSet::Gatecrash,
    // A 3/3 that cannot be blocked once a turn goes long enough, which is
    // what makes it the payoff for a hand full of cheap spells.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Human", "Wizard"], 1, 3).with_ability(
        AbilityDef::triggered_if(
            "Whenever you cast your second spell each turn, this creature gets +2/+0 until end of turn and can't be blocked this turn.",
            TriggerEventDef::SpellCast(ObjectPredicateDef::ControlledBy(PlayerRelation::You)),
            &INCURSION_SPECIALIST_SECOND_SPELL,
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

/// Mandatory and unaimed: a minimum of one with no target slot, so the
/// bounce cannot be answered with nothing and cannot be responded to by
/// protecting the creature it will name.
static KEYMASTER_ROGUE_BOUNCE: EffectDef = EffectDef::MoveToZone {
    counters: None,
    object: EffectRecipientDef::object(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
    zone: ZoneKind::Hand,
    controller: None,
    placement: ZonePlacement::Top,
    arrival_effect: None,
    attachment: None,
};

static KEYMASTER_ROGUE_CHOICE: EffectDef = EffectDef::Choose(ChooseDef {
    binding: ObjectChoiceBindingDef::Object(ObjectBindingIndex::PRIMARY),
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
    then: &KEYMASTER_ROGUE_BOUNCE,
});

// GTC 39 — Keymaster Rogue
pub(in crate::card::sets) static KEYMASTER_ROGUE: CardRecord = CardRecord::new_with_legacy_id(
    2013,
    "Keymaster Rogue",
    CardArt::new("970ee9a3-a862-46a7-9aa5-7b6fc4ffa1ab", "Winona Nelson"),
    CardSet::Gatecrash,
    // The bounce is a cost of admission rather than a bonus: with nothing
    // else out, the Rogue returns itself.
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Human", "Rogue"], 3, 2).with_abilities(&[
        AbilityDef::static_ability(
            "This creature can't be blocked.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
            },
        ),
        AbilityDef::triggered(
            "When this creature enters, return a creature you control to its owner's hand.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            KEYMASTER_ROGUE_CHOICE,
        ),
    ]),
);

// GTC 40 — Last Thoughts
// Audit: metadata-only — Cipher encoding and casting encoded spell copies without paying their mana costs are unavailable.
pub(in crate::card::sets) static LAST_THOUGHTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c6033d07-124c-4001-81e1-c6eb99e07fdd"),
    "Last Thoughts",
    crate::card::CardArt::new("c6033d07-124c-4001-81e1-c6eb99e07fdd", "Peter Mohrbacher"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 41 — Leyline Phantom
// Audit: metadata-only — Needs a trigger for the source dealing combat damage to any recipient and source survival through combat damage.
pub(in crate::card::sets) static LEYLINE_PHANTOM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e0867865-d9eb-45d2-a359-064f0f61197b"),
    "Leyline Phantom",
    crate::card::CardArt::new("e0867865-d9eb-45d2-a359-064f0f61197b", "Ryan Yee"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 42 — Metropolis Sprite
pub(in crate::card::sets) static METROPOLIS_SPRITE: CardRecord = CardRecord::new_with_legacy_id(
    1069,
    "Metropolis Sprite",
    CardArt::new("5f349013-0846-4bec-bdf9-47a3706d9989", "Scott Chou"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Faerie", "Rogue"], 1, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{U}: This creature gets +1/-1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(-1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// GTC 43 — Mindeye Drake
pub(in crate::card::sets) static MINDEYE_DRAKE: CardRecord = CardRecord::new_with_legacy_id(
    1070,
    "Mindeye Drake",
    CardArt::new("947f44b0-91be-4115-b499-57893f0f69a9", "Lars Grant-West"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Drake"], 2, 5).with_abilities(&[
        abilities::flying(),
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
    ]),
);

// GTC 44 — Rapid Hybridization
// Audit: metadata-only — Token creation always uses the resolving spell's controller, not the destroyed creature's controller.
pub(in crate::card::sets) static RAPID_HYBRIDIZATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("83557f55-f1ab-4995-9cc1-37be895a59db"),
    "Rapid Hybridization",
    crate::card::CardArt::new("83557f55-f1ab-4995-9cc1-37be895a59db", "Jack Wang"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 45 — Realmwright
// Audit: metadata-only — Needs choosing and storing one basic land type and a continuous land-type grant keyed to that choice.
pub(in crate::card::sets) static REALMWRIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("989c76b2-d130-443d-8534-6525fef404c2"),
    "Realmwright",
    crate::card::CardArt::new("989c76b2-d130-443d-8534-6525fef404c2", "Slawomir Maniak"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 46 — Sage's Row Denizen
pub(in crate::card::sets) static SAGES_ROW_DENIZEN: CardRecord = CardRecord::new_with_legacy_id(
    1071,
    "Sage's Row Denizen",
    CardArt::new("063e6df9-2287-485a-ab46-fa4a38783884", "Svetlin Velinov"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Vedalken", "Wizard"], 2, 3).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever another blue creature you control enters, target player mills two cards.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::Blue),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
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
    ),
);

/// "Each creature you control with a +1/+1 counter on it."
static YOUR_COUNTERED_CREATURES: [ObjectPredicateDef; 2] = [
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::HasCounter(CounterKind::PlusOnePlusOne),
];

static SAPPHIRE_DRAKE_FLYING: AbilityDef = abilities::flying();

// GTC 47 — Sapphire Drake
pub(in crate::card::sets) static SAPPHIRE_DRAKE: CardRecord = CardRecord::new_with_legacy_id(
    1628,
    "Sapphire Drake",
    CardArt::new("d0fe14cf-5d34-47d1-9071-4a532819719b", "Steve Prescott"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{5}{U}"), &["Drake"], 4, 4).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "Each creature you control with a +1/+1 counter on it has flying.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&YOUR_COUNTERED_CREATURES),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&SAPPHIRE_DRAKE_FLYING),
            },
        ),
    ]),
);

// GTC 48 — Scatter Arc
pub(in crate::card::sets) static SCATTER_ARC: CardRecord = CardRecord::new_with_legacy_id(
    1072,
    "Scatter Arc",
    CardArt::new("32ed969f-2c8e-4421-9448-dc5a2afdc81d", "Peter Mohrbacher"),
    CardSet::Gatecrash,
    CardRules::new_instant(mana_cost!("{3}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target noncreature spell. Draw a card.",
        &[AbilityTargetDef::exactly_one_spell(
            ObjectPredicateDef::NoncreatureSpell,
        )],
        EffectDef::Sequence(&[
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Graveyard,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// GTC 49 — Simic Fluxmage
// Audit: metadata-only — Evolve and moving a +1/+1 counter between two permanents are not declarative procedures.
pub(in crate::card::sets) static SIMIC_FLUXMAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("50633b59-6051-4f47-9e27-538fda03b5dd"),
    "Simic Fluxmage",
    crate::card::CardArt::new("50633b59-6051-4f47-9e27-538fda03b5dd", "Karl Kopinski"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 50 — Simic Manipulator
// Audit: metadata-only — Needs evolve, a variable counter-removal cost, a target-power limit based on counters removed, and indefinite control change.
pub(in crate::card::sets) static SIMIC_MANIPULATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e3dff9e6-5e0c-4e5b-8184-f0ae9cf347b3"),
    "Simic Manipulator",
    crate::card::CardArt::new("e3dff9e6-5e0c-4e5b-8184-f0ae9cf347b3", "Maciej Kuciara"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

/// The land taps for this, not the creature, and the sorcery-speed
/// restriction rides on the granted ability rather than on the Aura.
static SKYGAMES_FLIGHT: AbilityDef = abilities::flying();

static SKYGAMES_GRANTED: AbilityDef = AbilityDef::activated_with_targets(
    "{T}: Target creature gains flying until end of turn. Activate only as a sorcery.",
    &[AbilityCostDef::TapSource],
    &[AbilityTargetDef::exactly_one_permanent(
        ObjectPredicateDef::HasType(CardType::Creature),
    )],
    EffectDef::Apply {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        effect: AppliedEffectDef::add_ability(&SKYGAMES_FLIGHT),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
)
.with_activation_timing(ActivationTimingDef::SorcerySpeed);

// GTC 51 — Skygames
pub(in crate::card::sets) static SKYGAMES: CardRecord = CardRecord::new_with_legacy_id(
    1954,
    "Skygames",
    CardArt::new("7ab5bf75-762f-46ef-8304-aacdb248bc5b", "Sam Burley"),
    CardSet::Gatecrash,
    CardRules::new_enchantment(mana_cost!("{1}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant land", &abilities::ENCHANT_LAND_TARGET),
            AbilityDef::static_ability(
                "Enchanted land has \"{T}: Target creature gains flying until end of turn. \
                 Activate only as a sorcery.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&SKYGAMES_GRANTED),
                },
            ),
        ]),
);

/// The tax is whatever your biggest creature is, so this is a counterspell
/// that grows with the board rather than with the turn.
static SPELL_RUPTURE_CREATURES: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

// GTC 52 — Spell Rupture
pub(in crate::card::sets) static SPELL_RUPTURE: CardRecord = CardRecord::new_with_legacy_id(
    2011,
    "Spell Rupture",
    CardArt::new("7267fcec-0879-4743-a45f-35057ccb2596", "Kev Walker"),
    CardSet::Gatecrash,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell unless its controller pays {X}, where X is the greatest power among creatures you control.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        abilities::counter_target_unless_paid(ValueDef::GreatestPowerAmong(
            &SPELL_RUPTURE_CREATURES,
        )),
    )),
);

// GTC 53 — Stolen Identity
// Audit: metadata-only — Needs token copies of a target and cipher's encoding and free-copy-casting procedure.
pub(in crate::card::sets) static STOLEN_IDENTITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4e058447-7cee-4670-b86b-c95bd6b68144"),
    "Stolen Identity",
    crate::card::CardArt::new("4e058447-7cee-4670-b86b-c95bd6b68144", "Clint Cearley"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 54 — Totally Lost
pub(in crate::card::sets) static TOTALLY_LOST: CardRecord = CardRecord::new_with_legacy_id(
    1073,
    "Totally Lost",
    CardArt::new("ec8e4142-7c46-4d2f-aaa6-6410f323d9f0", "David Palumbo"),
    CardSet::Gatecrash,
    CardRules::new_instant(mana_cost!("{4}{U}")).with_ability(AbilityDef::spell_with_targets(
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
    )),
);

// GTC 55 — Voidwalk
// Audit: metadata-only — Cipher's encoding and free-copy-casting procedure are unavailable, even though the initial delayed blink is expressible.
pub(in crate::card::sets) static VOIDWALK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("611d0e10-e767-4e66-b1f1-02f1624fab2b"),
    "Voidwalk",
    crate::card::CardArt::new("611d0e10-e767-4e66-b1f1-02f1624fab2b", "James Ryman"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

/// The Aura's controller, not the creature's, so gifting the creature away
/// leaves the evasion behind with the Gate that pays for it.
static WAY_OF_THE_THIEF_HAS_A_GATE: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef::matching(
        ObjectPredicateDef::Subtype("Gate"),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    ),
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 1,
};

static WAY_OF_THE_THIEF_EVASION: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::AttachedPermanent,
    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
};

// GTC 56 — Way of the Thief
pub(in crate::card::sets) static WAY_OF_THE_THIEF: CardRecord = CardRecord::new_with_legacy_id(
    1958,
    "Way of the Thief",
    CardArt::new("b249ca81-bd8d-4d3d-81d6-15e8d669c416", "Igor Kieryluk"),
    CardSet::Gatecrash,
    // The size is unconditional; only the evasion asks about the Gate.
    CardRules::new_enchantment(mana_cost!("{3}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+2.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature can't be blocked as long as you control a Gate.",
                EffectDef::IfCondition {
                    condition: &WAY_OF_THE_THIEF_HAS_A_GATE,
                    then: &WAY_OF_THE_THIEF_EVASION,
                },
            ),
        ]),
);

/// A library with no land left in it empties, which is the whole reason
/// these two are a combo piece rather than a mill spell.
static MILL_TO_THE_FIRST_LAND: EffectDef = EffectDef::MillUntil(&MILL_UNTIL_1);

// GTC 57 — Balustrade Spy
pub(in crate::card::sets) static BALUSTRADE_SPY: CardRecord = CardRecord::new_with_legacy_id(
    1998,
    "Balustrade Spy",
    CardArt::new("df8a3f05-864d-401d-a2f1-5f58358fe089", "Jaime Jones"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Vampire", "Rogue"], 2, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "When this creature enters, target player reveals cards from the top of their library until they reveal a land card, then puts those cards into their graveyard.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                PlayerRelation::Any,
            ))],
            MILL_TO_THE_FIRST_LAND,
        ),
    ]),
);

// GTC 58 — Basilica Screecher
pub(in crate::card::sets) static BASILICA_SCREECHER: CardRecord = CardRecord::new_with_legacy_id(
    1074,
    "Basilica Screecher",
    CardArt::new("d233c6bc-c4dd-482d-b0f4-87359acab7cb", "Christine Choi"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Bat"], 1, 2)
        .with_abilities(&[abilities::flying(), abilities::extort()]),
);

static CONTAMINATED_GROUND_TRIGGER: AbilityDef = AbilityDef::triggered(
    "Whenever enchanted land becomes tapped, its controller loses 2 life.",
    TriggerEventDef::tapped(ObjectPredicateDef::Source),
    EffectDef::LoseLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(2),
    },
);

// GTC 59 — Contaminated Ground
pub(in crate::card::sets) static CONTAMINATED_GROUND: CardRecord = CardRecord::new_with_legacy_id(
    1075,
    "Contaminated Ground",
    CardArt::new("c2384356-0a62-499a-8b28-085974331368", "Christine Choi"),
    CardSet::Gatecrash,
    CardRules::new_enchantment(mana_cost!("{1}{B}"))
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
            AbilityDef::static_ability(
                "Enchanted land is a Swamp.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::set_basic_land_types(&[BasicLandType::Swamp]),
                },
            ),
            AbilityDef::static_ability(
                "Whenever enchanted land becomes tapped, its controller loses 2 life.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&CONTAMINATED_GROUND_TRIGGER),
                },
            ),
        ]),
);

// GTC 60 — Corpse Blockade
pub(in crate::card::sets) static CORPSE_BLOCKADE: CardRecord = CardRecord::new_with_legacy_id(
    1076,
    "Corpse Blockade",
    CardArt::new("84234e51-e5d6-43d9-89d0-f0398fc6b7fd", "Lucas Graciano"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Zombie"], 1, 4).with_abilities(&[
        abilities::defender(),
        AbilityDef::activated(
            "Sacrifice another creature: This creature gains deathtouch until end of turn.",
            &[AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                controller: PlayerRelation::You,
            }],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::deathtouch()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// GTC 61 — Crypt Ghast
pub(in crate::card::sets) static CRYPT_GHAST: CardRecord = CardRecord::new_with_legacy_id(
    1077,
    "Crypt Ghast",
    CardArt::new("3795a4e7-646f-4bb7-b154-2610eb740e8d", "Chris Rahn"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Spirit"], 2, 2).with_abilities(&[
        abilities::extort(),
        AbilityDef::triggered_mana(
            "Whenever you tap a Swamp for mana, add an additional {B}.",
            TriggerEventDef::tapped_for_mana(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Swamp]),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black)),
        ),
    ]),
);

// GTC 62 — Death's Approach
// Audit: metadata-only — The static value vocabulary cannot count creature cards in the attached creature's controller's graveyard.
pub(in crate::card::sets) static DEATH_S_APPROACH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b0f69c6f-522d-42c8-be7c-ea6d7ffb6a90"),
    "Death's Approach",
    crate::card::CardArt::new("b0f69c6f-522d-42c8-be7c-ea6d7ffb6a90", "Terese Nielsen"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

/// The life follows the sacrifice, so it belongs to the same continuation --
/// and it goes to the player who paid, not to whoever cast the spell.
static DEVOUR_FLESH_PAYOFF: EffectDef = EffectDef::GainLife {
    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    amount: ValueDef::TriggerEventAmount,
};

// GTC 63 — Devour Flesh
pub(in crate::card::sets) static DEVOUR_FLESH: CardRecord = CardRecord::new_with_legacy_id(
    1970,
    "Devour Flesh",
    CardArt::new("88c42ebd-114a-430d-b3a4-ff2fb3093bf5", "Kev Walker"),
    CardSet::Gatecrash,
    // Edict removal that pays its victim: the toughness they gain is often
    // more than the creature was worth to them.
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target player sacrifices a creature. That player gains life equal to that creature's toughness.",
        &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
            PlayerRelation::Any,
        ))],
        EffectDef::SacrificeOfChoice {
            count: ValueDef::Constant(1),
            player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            object: ObjectPredicateDef::HasType(CardType::Creature),
            then: Some(&DEVOUR_FLESH_PAYOFF),
            amount: SacrificedAmountDef::Toughness,
            otherwise: None,
            optional: false,
        },
    )),
);

static DYING_WISH_DRAIN: [EffectDef; 2] = [
    EffectDef::LoseLife {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: ValueDef::TriggeringObjectPower,
    },
    EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::TriggeringObjectPower,
    },
];

// GTC 64 — Dying Wish
pub(in crate::card::sets) static DYING_WISH: CardRecord = CardRecord::new_with_legacy_id(
    1612,
    "Dying Wish",
    CardArt::new("b46e83d3-c66d-42fb-8435-b6c448db01ae", "Scott Chou"),
    CardSet::Gatecrash,
    CardRules::new_enchantment(mana_cost!("{1}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature you control", &ENCHANT_YOUR_CREATURE),
            AbilityDef::triggered_with_targets(
                "When enchanted creature dies, target player loses X life and you gain X life, \
                 where X is its power.",
                ENCHANTED_CREATURE_DIES,
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::Sequence(&DYING_WISH_DRAIN),
            ),
        ]),
);

/// "Tap an untapped Gate you control."
static TAP_A_GATE: AbilityCostDef = AbilityCostDef::TapPermanent {
    object: ObjectPredicateDef::Subtype("Gate"),
    controller: PlayerRelation::You,
};

// GTC 65 — Gateway Shade
pub(in crate::card::sets) static GATEWAY_SHADE: CardRecord = CardRecord::new_with_legacy_id(
    1633,
    "Gateway Shade",
    CardArt::new("aa33fc15-3a4f-48bc-be7c-fdec1cb49c10", "Ryan Yee"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Shade"], 1, 1).with_abilities(&[
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
        AbilityDef::activated(
            "Tap an untapped Gate you control: This creature gets +2/+2 until end of turn.",
            &[TAP_A_GATE],
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

// GTC 66 — Grisly Spectacle
pub(in crate::card::sets) static GRISLY_SPECTACLE: CardRecord = CardRecord::new_with_legacy_id(
    1078,
    "Grisly Spectacle",
    CardArt::new("c26d0f6e-e7bd-4206-a0da-1c9c203a73f2", "Zoltan Boros"),
    CardSet::Gatecrash,
    CardRules::new_instant(mana_cost!("{2}{B}{B}")).with_ability(
        AbilityDef::spell_with_targets(
            "Destroy target nonartifact creature. Its controller mills cards equal to that creature's power.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Artifact)),
                ]),
            )],
            EffectDef::Sequence(&[
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                },
                EffectDef::Mill {
                    player: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                    amount: ValueDef::TargetPower(TargetIndex::PRIMARY),
                    binding: None,
                    then: None,
                },
            ]),
        ),
    ),
);

// GTC 67 — Gutter Skulk
pub(in crate::card::sets) static GUTTER_SKULK: CardRecord = CardRecord::new_with_legacy_id(
    1079,
    "Gutter Skulk",
    CardArt::new("830c7c77-20c4-429f-88c7-b85ab7a0e38b", "Mark Winters"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Zombie", "Rat"], 2, 2),
);

// GTC 68 — Horror of the Dim
pub(in crate::card::sets) static HORROR_OF_THE_DIM: CardRecord = CardRecord::new_with_legacy_id(
    1080,
    "Horror of the Dim",
    CardArt::new("f5d36c9d-967e-42dc-890c-0485b12f704f", "Jack Wang"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Horror"], 3, 4).with_ability(
        AbilityDef::activated(
            "{U}: This creature gains hexproof until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// GTC 69 — Illness in the Ranks
pub(in crate::card::sets) static ILLNESS_IN_THE_RANKS: CardRecord = CardRecord::new_with_legacy_id(
    1889,
    "Illness in the Ranks",
    CardArt::new("989a68c1-3b76-4c2d-9db3-23c45be3f9ff", "Nils Hamm"),
    CardSet::Gatecrash,
    // Every creature token, whoever controls it, which is what makes this a
    // sideboard card rather than an anthem for one side.
    CardRules::new_enchantment(mana_cost!("{B}")).with_ability(AbilityDef::static_ability(
        "Creature tokens get -1/-1.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Token,
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-1),
                ValueDef::Constant(-1),
            ),
        },
    )),
);

/// "Power X or less" said with the strict comparison the predicates offer:
/// power is an integer, so at most X and below X plus one are the same set.
static KILLING_GLARE_LIMIT: SumValueDef = SumValueDef {
    left: ValueDef::ChosenX,
    right: ValueDef::Constant(1),
};

// GTC 70 — Killing Glare
pub(in crate::card::sets) static KILLING_GLARE: CardRecord = CardRecord::new_with_legacy_id(
    2016,
    "Killing Glare",
    CardArt::new("f7a4d87d-b844-4f20-8b14-4fd32c53dea5", "Peter Mohrbacher"),
    CardSet::Gatecrash,
    // Scales to whatever it has to answer, and at X of zero it still kills
    // something: a 0/1 blocker is a legal target.
    CardRules::new_instant(mana_cost!("{X}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target creature with power X or less.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::PowerLessThan(ValueDef::Sum(&KILLING_GLARE_LIMIT)),
            ]),
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            can_regenerate: true,
        },
    )),
);

// GTC 71 — Lord of the Void
// Audit: metadata-only — Needs combat-damage-player subject capture, top-seven exile, and a non-target creature-card choice from the exiled group.
pub(in crate::card::sets) static LORD_OF_THE_VOID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("75b83fe5-fd00-4532-bc67-07836abfc99c"),
    "Lord of the Void",
    crate::card::CardArt::new("75b83fe5-fd00-4532-bc67-07836abfc99c", "Chris Rahn"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 72 — Mental Vapors
// Audit: metadata-only — Cipher encoding and casting encoded spell copies without paying their mana costs are unavailable.
pub(in crate::card::sets) static MENTAL_VAPORS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("076e7c58-a6fe-4882-8f8d-698be9a7f22d"),
    "Mental Vapors",
    crate::card::CardArt::new("076e7c58-a6fe-4882-8f8d-698be9a7f22d", "Mark Winters"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 73 — Midnight Recovery
// Audit: metadata-only — Cipher encoding and casting encoded spell copies without paying their mana costs are unavailable.
pub(in crate::card::sets) static MIDNIGHT_RECOVERY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4df40471-1118-4429-83bf-8225ea50b69f"),
    "Midnight Recovery",
    crate::card::CardArt::new("4df40471-1118-4429-83bf-8225ea50b69f", "Peter Mohrbacher"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

static OGRE_SLUMLORD_DEATHTOUCH: AbilityDef = abilities::deathtouch();

/// "Rats you control", with no "other" -- the Slumlord is an Ogre Rogue, so
/// the clause never reaches it anyway, but a Rat it makes is covered.
static OGRE_SLUMLORD_RATS: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::matching_objects(
        ObjectPredicateDef::Subtype("Rat"),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    ),
    effect: AppliedEffectDef::add_ability(&OGRE_SLUMLORD_DEATHTOUCH),
};

static OGRE_SLUMLORD_DEATH: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
]);

// GTC 74 — Ogre Slumlord
pub(in crate::card::sets) static OGRE_SLUMLORD: CardRecord = CardRecord::new_with_legacy_id(
    1894,
    "Ogre Slumlord",
    CardArt::new("29727bd1-9415-408a-99de-dd992e26e767", "Trevor Claxton"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Ogre", "Rogue"], 3, 3).with_abilities(&[
        AbilityDef::triggered(
            "Whenever another nontoken creature dies, you may create a 1/1 black Rat creature \
             token.",
            TriggerEventDef::zone_changed(
                OGRE_SLUMLORD_DEATH,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::create_creature_token(&["Rat"], &[ManaColor::Black], 1, 1)
                    .with_art(CardArt::new(
                        "f1fb8ca6-7351-457a-b2a4-48f57ec3c64a",
                        "Nils Hamm",
                    )),
            },
        ),
        AbilityDef::static_ability("Rats you control have deathtouch.", OGRE_SLUMLORD_RATS),
    ]),
);

// GTC 75 — Sepulchral Primordial
pub(in crate::card::sets) static SEPULCHRAL_PRIMORDIAL: CardRecord = CardRecord::new_with_legacy_id(
    210,
    "Sepulchral Primordial",
    CardArt::new("eb0865cd-d9b4-43ea-87d2-ad5c65fc0459", "Stephan Martiniere"),
    CardSet::Gatecrash,
    CardRules::new_creature(
        mana_cost!("{5}{B}{B}"),
        &["Avatar"],
        5,
        4,
    )
    .with_abilities(&[
        abilities::intimidate(),
        AbilityDef::triggered_with_targets("When this creature enters, for each opponent, you may put up to one target creature card from that player's graveyard onto the battlefield under your control.", TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)), &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::Opponent),
            },
            1,
        )], // One opponent means one target here. Choosing none is already a
            // legal target selection, so the printed "may" adds nothing.
            EffectDef::MoveToZone {
                counters: None,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Battlefield,
                controller: Some(PlayerRelation::You),
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
            }),
    ]),
);

// GTC 76 — Shadow Alley Denizen
pub(in crate::card::sets) static SHADOW_ALLEY_DENIZEN: CardRecord = CardRecord::new_with_legacy_id(
    1081,
    "Shadow Alley Denizen",
    CardArt::new("985997ae-59bc-49d7-87ca-e63ed9706fdf", "Cynthia Sheppard"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{B}"), &["Vampire", "Rogue"], 1, 1).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever another black creature you control enters, target creature gains intimidate until end of turn.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::Black),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]), None, Some(ZoneKind::Battlefield)),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::intimidate()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// GTC 77 — Shadow Slice
// Audit: metadata-only — Cipher encoding and casting encoded spell copies without paying their mana costs are unavailable.
pub(in crate::card::sets) static SHADOW_SLICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0497fbf4-cf09-4028-af16-349ed12ca360"),
    "Shadow Slice",
    crate::card::CardArt::new("0497fbf4-cf09-4028-af16-349ed12ca360", "Raymond Swanland"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 78 — Slate Street Ruffian
// Audit: metadata-only — There is no trigger event for a creature becoming blocked or a captured defending player.
pub(in crate::card::sets) static SLATE_STREET_RUFFIAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("51528a3e-beba-47f8-a524-ad99b1fec308"),
    "Slate Street Ruffian",
    crate::card::CardArt::new("51528a3e-beba-47f8-a524-ad99b1fec308", "Jim Murray"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 79 — Smog Elemental
pub(in crate::card::sets) static SMOG_ELEMENTAL: CardRecord = CardRecord::new_with_legacy_id(
    1646,
    "Smog Elemental",
    CardArt::new("667871d3-0d1b-496b-afbd-7504989798e4", "Yeong-Hao Han"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Elemental"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "Creatures with flying your opponents control get -1/-1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Opponent,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(-1),
                ),
            },
        ),
    ]),
);

// GTC 80 — Syndicate Enforcer
pub(in crate::card::sets) static SYNDICATE_ENFORCER: CardRecord = CardRecord::new_with_legacy_id(
    1082,
    "Syndicate Enforcer",
    CardArt::new("cde6ee2e-a114-4935-8345-d3e264f9fc26", "Steven Belledin"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Human", "Rogue"], 3, 2)
        .with_ability(abilities::extort()),
);

// GTC 81 — Thrull Parasite
// Audit: metadata-only — Counter-removal costs and effects require a fixed CounterKind, not choosing any counter on the target.
pub(in crate::card::sets) static THRULL_PARASITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c38dfb08-ab41-4759-9967-b5a25f18518a"),
    "Thrull Parasite",
    crate::card::CardArt::new("c38dfb08-ab41-4759-9967-b5a25f18518a", "Clint Cearley"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 82 — Undercity Informer
pub(in crate::card::sets) static UNDERCITY_INFORMER: CardRecord = CardRecord::new_with_legacy_id(
    1999,
    "Undercity Informer",
    CardArt::new("822d0f73-cfb0-41d9-b4eb-09c605112a13", "Raymond Swanland"),
    CardSet::Gatecrash,
    // The same effect on a repeatable body, which is what makes it the
    // dangerous half of the pair.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Human", "Rogue"], 2, 3).with_ability(
        AbilityDef::activated_with_targets(
            "{1}, Sacrifice a creature: Target player reveals cards from the top of their library until they reveal a land card, then puts those cards into their graveyard.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    controller: PlayerRelation::You,
                },
            ],
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                PlayerRelation::Any,
            ))],
            MILL_TO_THE_FIRST_LAND,
        ),
    ),
);

// GTC 83 — Undercity Plague
// Audit: metadata-only — Needs cipher plus a discard decision that resumes into a permanent-sacrifice choice before later effects resolve.
pub(in crate::card::sets) static UNDERCITY_PLAGUE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7b25d3bc-33e9-4d1a-855d-38580e67b6cc"),
    "Undercity Plague",
    crate::card::CardArt::new("7b25d3bc-33e9-4d1a-855d-38580e67b6cc", "Vincent Proce"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

static WIGHT_CREATURE_CARDS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Graveyard],
    PlayerRelation::Opponent,
);

// GTC 84 — Wight of Precinct Six
pub(in crate::card::sets) static WIGHT_OF_PRECINCT_SIX: CardRecord = CardRecord::new_with_legacy_id(
    1083,
    "Wight of Precinct Six",
    CardArt::new("b04644ba-5962-4e64-bc53-92941c5b6715", "Ryan Barger"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Zombie"], 1, 1).with_ability(
        AbilityDef::static_ability(
            "This creature gets +1/+1 for each creature card in your opponents' graveyards.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::CountMatchingObjects(&WIGHT_CREATURE_CARDS),
                    ValueDef::CountMatchingObjects(&WIGHT_CREATURE_CARDS),
                ),
            },
        ),
    ),
);

// GTC 85 — Act of Treason
pub(in crate::card::sets) static ACT_OF_TREASON: CardRecord = CardRecord::new_with_legacy_id(
    1084,
    "Act of Treason",
    CardArt::new("a04c8c6f-14e9-427c-918e-208ccd39ec4a", "Matt Stewart"),
    CardSet::Gatecrash,
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Gain control of target creature until end of turn. Untap that creature. It gains haste until end of turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
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
                    effect: AppliedEffectDef::add_ability(&abilities::haste()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ),
);

// GTC 86 — Bomber Corps
pub(in crate::card::sets) static BOMBER_CORPS: CardRecord = CardRecord::new_with_legacy_id(
    1510,
    "Bomber Corps",
    CardArt::new("9b1675f2-e950-4f3c-9dd3-29ead615ff23", "Chase Stone"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Soldier"], 1, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "Battalion — Whenever this creature and at least two other creatures attack, this \
             creature deals 1 damage to any target.",
            abilities::BATTALION_EVENT,
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

// GTC 87 — Cinder Elemental
pub(in crate::card::sets) static CINDER_ELEMENTAL: CardRecord = CardRecord::new_with_legacy_id(
    1085,
    "Cinder Elemental",
    CardArt::new("8bbf10ce-69e0-4984-91a3-f65df919830d", "Svetlin Velinov"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Elemental"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{X}{R}, {T}, Sacrifice this creature: It deals X damage to any target.",
            &[
                AbilityCostDef::Mana(mana_cost!("{X}{R}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::ChosenX,
            },
        ),
    ),
);

// GTC 88 — Crackling Perimeter
pub(in crate::card::sets) static CRACKLING_PERIMETER: CardRecord = CardRecord::new_with_legacy_id(
    1634,
    "Crackling Perimeter",
    CardArt::new("3323c86c-73bd-4e23-9f80-54bf5c1dd0bc", "Yeong-Hao Han"),
    CardSet::Gatecrash,
    CardRules::new_enchantment(mana_cost!("{1}{R}")).with_ability(AbilityDef::activated(
        "Tap an untapped Gate you control: This enchantment deals 1 damage to each opponent.",
        &[TAP_A_GATE],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Opponent,
            amount: ValueDef::Constant(1),
        },
    )),
);

// GTC 89 — Ember Beast (reprint)

// GTC 90 — Firefist Striker
pub(in crate::card::sets) static FIREFIST_STRIKER: CardRecord = CardRecord::new_with_legacy_id(
    1520,
    "Firefist Striker",
    CardArt::new("ccbc2f22-4500-4c74-a1a2-51d8238c1d16", "Tyler Jacobson"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Soldier"], 2, 1).with_ability(
        AbilityDef::triggered_with_targets(
            "Battalion — Whenever this creature and at least two other creatures attack, target \
             creature can't block this turn.",
            abilities::BATTALION_EVENT,
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

// GTC 91 — Five-Alarm Fire
// Audit: metadata-only — Needs a trigger for any creature you control dealing combat damage and an executable blaze-counter removal cost.
pub(in crate::card::sets) static FIVE_ALARM_FIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fb550d0c-2261-4f41-a2b1-d185f0bce86a"),
    "Five-Alarm Fire",
    crate::card::CardArt::new("fb550d0c-2261-4f41-a2b1-d185f0bce86a", "Karl Kopinski"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 92 — Foundry Street Denizen
pub(in crate::card::sets) static FOUNDRY_STREET_DENIZEN: CardRecord = CardRecord::new_with_legacy_id(
    1086,
    "Foundry Street Denizen",
    CardArt::new("0befed63-07ba-4728-9078-57bbccbeeeb1", "Raoul Vitale"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin", "Warrior"], 1, 1).with_ability(
        AbilityDef::triggered(
            "Whenever another red creature you control enters, this creature gets +1/+0 until end of turn.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::Red),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]), None, Some(ZoneKind::Battlefield)),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(0)),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// GTC 93 — Furious Resistance
// Audit: metadata-only — Target predicates recognize attacking or blocking together, but cannot require specifically a blocking creature.
pub(in crate::card::sets) static FURIOUS_RESISTANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4eeea013-1cf8-4c77-a097-aa69e141e3f4"),
    "Furious Resistance",
    crate::card::CardArt::new("4eeea013-1cf8-4c77-a097-aa69e141e3f4", "Slawomir Maniak"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 94 — Hellkite Tyrant
// Audit: metadata-only — Needs indefinite control of a target player's artifacts and a win-the-game effect under a twenty-artifact upkeep condition.
pub(in crate::card::sets) static HELLKITE_TYRANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0bc3401f-935b-45ce-b1e6-300a5d9dfd4f"),
    "Hellkite Tyrant",
    crate::card::CardArt::new("0bc3401f-935b-45ce-b1e6-300a5d9dfd4f", "Aleksi Briclot"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

static HELLRAISER_ATTACKS: AbilityDef =
    abilities::attacks_each_combat_if_able("This creature attacks each combat if able.");

// GTC 95 — Hellraiser Goblin
pub(in crate::card::sets) static HELLRAISER_GOBLIN: CardRecord = CardRecord::new_with_legacy_id(
    1087,
    "Hellraiser Goblin",
    CardArt::new("156941e7-9169-47aa-b04d-37ca78c54f7c", "Karl Kopinski"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Berserker"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "Creatures you control have haste and attack each combat if able.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_ability(&abilities::haste()),
                    AppliedEffectDef::add_ability(&HELLRAISER_ATTACKS),
                ]),
            },
        ),
    ),
);

// GTC 96 — Homing Lightning
pub(in crate::card::sets) static HOMING_LIGHTNING: CardRecord = CardRecord::new_with_legacy_id(
    1088,
    "Homing Lightning",
    CardArt::new("d6535816-8fa4-4c8b-8677-ac80f769f528", "Slawomir Maniak"),
    CardSet::Gatecrash,
    CardRules::new_instant(mana_cost!("{2}{R}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Homing Lightning deals 4 damage to target creature and each other creature with the same name as that creature.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::ObjectsSharingNameWithTarget(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(4),
            },
        ),
    ),
);

// GTC 97 — Legion Loyalist
// Audit: metadata-only — Battalion is trigger-time-only, menace-style token blocking restrictions are unavailable, and its mass grants cannot be conditioned exactly.
pub(in crate::card::sets) static LEGION_LOYALIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b47f639e-4635-4c26-bb2a-4925f0582c21"),
    "Legion Loyalist",
    crate::card::CardArt::new("b47f639e-4635-4c26-bb2a-4925f0582c21", "Eric Deschamps"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

static MADCAP_SKILLS_EFFECT: [AppliedEffectDef; 2] = [
    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(3), ValueDef::Constant(0)),
    AppliedEffectDef::add_ability(&abilities::menace()),
];

// GTC 98 — Madcap Skills
pub(in crate::card::sets) static MADCAP_SKILLS: CardRecord = CardRecord::new_with_legacy_id(
    1758,
    "Madcap Skills",
    CardArt::new("57b01da4-1a20-44e5-b383-c6a54541ace1", "Anthony Palumbo"),
    CardSet::Gatecrash,
    CardRules::new_enchantment(mana_cost!("{1}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature gets +3/+0 and has menace.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&MADCAP_SKILLS_EFFECT),
                },
            ),
        ]),
);

// GTC 99 — Mark for Death
// Audit: metadata-only — Needs turn-long must-block and cannot-block constraints scoped to one opponent's creatures.
pub(in crate::card::sets) static MARK_FOR_DEATH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("45498dbd-a512-4299-800b-06c15a4fd94e"),
    "Mark for Death",
    crate::card::CardArt::new("45498dbd-a512-4299-800b-06c15a4fd94e", "Mathias Kollros"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

static MASSIVE_RAID_CREATURES: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

// GTC 100 — Massive Raid
pub(in crate::card::sets) static MASSIVE_RAID: CardRecord = CardRecord::new_with_legacy_id(
    1089,
    "Massive Raid",
    CardArt::new("8b16fbd8-fb62-4f75-92b3-a6295d95b327", "Zoltan Boros"),
    CardSet::Gatecrash,
    CardRules::new_instant(mana_cost!("{1}{R}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Massive Raid deals damage to any target equal to the number of creatures you control.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::CountMatchingObjects(&MASSIVE_RAID_CREATURES),
        },
    )),
);

// GTC 101 — Molten Primordial
pub(in crate::card::sets) static MOLTEN_PRIMORDIAL: CardRecord = CardRecord::new_with_legacy_id(
    1090,
    "Molten Primordial",
    CardArt::new(
        "a8f5c7e2-f4da-4cee-a7d0-80b29bb73acd",
        "Stephan Martiniere",
    ),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{5}{R}{R}"), &["Avatar"], 6, 4).with_abilities(&[
        abilities::haste(),
        AbilityDef::triggered_with_targets(
            "When this creature enters, for each opponent, gain control of up to one target creature that player controls until end of turn. Untap those creatures. They gain haste until end of turn.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)),
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
                1,
            )],
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
                    effect: AppliedEffectDef::add_ability(&abilities::haste()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ]),
);

static MUGGING_EFFECTS: [EffectDef; 2] = [
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: ValueDef::Constant(2),
    },
    EffectDef::Apply {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
];

// GTC 102 — Mugging
pub(in crate::card::sets) static MUGGING: CardRecord = CardRecord::new_with_legacy_id(
    1951,
    "Mugging",
    CardArt::new("05ca502f-73a3-42f3-b7ad-f69aa239900a", "Greg Staples"),
    CardSet::Gatecrash,
    // The prohibition lands even when the two damage was not enough to kill,
    // which is the half that makes this more than a small burn spell.
    CardRules::new_sorcery(mana_cost!("{R}")).with_ability(AbilityDef::spell_with_targets(
        "Mugging deals 2 damage to target creature. That creature can't block this turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&MUGGING_EFFECTS),
    )),
);

// GTC 103 — Ripscale Predator
pub(in crate::card::sets) static RIPSCALE_PREDATOR: CardRecord = CardRecord::new_with_legacy_id(
    1757,
    "Ripscale Predator",
    CardArt::new("37c24d58-70ac-40dd-8fd7-bc4f8ba3ac68", "Volkan Baǵa"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{4}{R}{R}"), &["Dinosaur"], 6, 5)
        .with_ability(abilities::menace()),
);

// GTC 104 — Scorchwalker
pub(in crate::card::sets) static SCORCHWALKER: CardRecord = CardRecord::new_with_legacy_id(
    1091,
    "Scorchwalker",
    CardArt::new("14ac6bde-1fef-45f4-b505-80a66b03140a", "Anthony Palumbo"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Elemental"], 5, 1).with_ability(
        abilities::bloodrush(
            mana_cost!("{1}{R}{R}"),
            "Bloodrush — {1}{R}{R}, Discard this card: Target attacking creature gets +5/+1 until end of turn.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Attacking,
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            })],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(5), ValueDef::Constant(1)),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// GTC 105 — Skinbrand Goblin
pub(in crate::card::sets) static SKINBRAND_GOBLIN: CardRecord = CardRecord::new_with_legacy_id(
    1092,
    "Skinbrand Goblin",
    CardArt::new("fe4f9b6c-3ba9-4f4f-8135-f5236195e507", "Marco Nelor"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Warrior"], 2, 1).with_ability(
        abilities::bloodrush(
            mana_cost!("{R}"),
            "Bloodrush — {R}, Discard this card: Target attacking creature gets +2/+1 until end of turn.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Attacking,
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            })],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(1)),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// GTC 106 — Skullcrack
// Audit: metadata-only — Needs turn-long prohibitions on life gain and damage prevention.
pub(in crate::card::sets) static SKULLCRACK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8068a146-f6fe-46f3-a42e-822fbc3502e6"),
    "Skullcrack",
    crate::card::CardArt::new("8068a146-f6fe-46f3-a42e-822fbc3502e6", "Dave Kendall"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 107 — Structural Collapse
// Audit: metadata-only — Needs two distinct resolving permanent choices and a continuation that deals damage after both sacrifices.
pub(in crate::card::sets) static STRUCTURAL_COLLAPSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d10da484-db67-4afc-90ef-6caf7d2e3a75"),
    "Structural Collapse",
    crate::card::CardArt::new("d10da484-db67-4afc-90ef-6caf7d2e3a75", "Sam Burley"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

/// The land taps for this, so the Market turns any spare land into a looter.
static TIN_STREET_MARKET_GRANTED: AbilityDef = AbilityDef::activated(
    "{T}, Discard a card: Draw a card.",
    &[
        AbilityCostDef::TapSource,
        AbilityCostDef::DiscardCardMatching(ObjectPredicateDef::Any),
    ],
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
);

// GTC 108 — Tin Street Market
pub(in crate::card::sets) static TIN_STREET_MARKET: CardRecord = CardRecord::new_with_legacy_id(
    1961,
    "Tin Street Market",
    CardArt::new("1c1f543b-2222-4ef5-b4f7-2c3d2ea27fdc", "Noah Bradley"),
    CardSet::Gatecrash,
    CardRules::new_enchantment(mana_cost!("{4}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant land", &abilities::ENCHANT_LAND_TARGET),
            AbilityDef::static_ability(
                "Enchanted land has \"{T}, Discard a card: Draw a card.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&TIN_STREET_MARKET_GRANTED),
                },
            ),
        ]),
);

// GTC 109 — Towering Thunderfist
pub(in crate::card::sets) static TOWERING_THUNDERFIST: CardRecord = CardRecord::new_with_legacy_id(
    1093,
    "Towering Thunderfist",
    CardArt::new("d68e9280-cb1a-48e1-a91e-217e101f19c5", "Zoltan Boros"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Giant", "Soldier"], 4, 4).with_ability(
        AbilityDef::activated(
            "{W}: This creature gains vigilance until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::vigilance()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// GTC 110 — Viashino Shanktail
pub(in crate::card::sets) static VIASHINO_SHANKTAIL: CardRecord = CardRecord::new_with_legacy_id(
    1094,
    "Viashino Shanktail",
    CardArt::new("f5dd72d5-548a-4b0e-95bb-e6b8d2de0fbe", "Kev Walker"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Lizard", "Warrior"], 3, 1)
        .with_abilities(&[
            abilities::first_strike(),
            abilities::bloodrush(
                mana_cost!("{2}{R}"),
                "Bloodrush — {2}{R}, Discard this card: Target attacking creature gets +3/+1 and gains first strike until end of turn.",
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Attacking,
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                })],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(ValueDef::Constant(3), ValueDef::Constant(1)),
                        AppliedEffectDef::add_ability(&abilities::first_strike()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// GTC 111 — Warmind Infantry
pub(in crate::card::sets) static WARMIND_INFANTRY: CardRecord = CardRecord::new_with_legacy_id(
    1511,
    "Warmind Infantry",
    CardArt::new("d8a5f801-9e55-4e14-85b0-5719521cd9d6", "Greg Staples"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Elemental", "Soldier"], 2, 3).with_abilities(&[
        abilities::battalion(
            "Battalion — Whenever this creature and at least two other creatures attack, this creature gets +2/+0 until end of turn.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(0)),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// GTC 112 — Wrecking Ogre
pub(in crate::card::sets) static WRECKING_OGRE: CardRecord = CardRecord::new_with_legacy_id(
    1095,
    "Wrecking Ogre",
    CardArt::new("87f1a27c-c576-4c34-873f-6faf020c2773", "Nils Hamm"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Ogre", "Warrior"], 3, 3)
        .with_abilities(&[
            abilities::double_strike(),
            abilities::bloodrush(
                mana_cost!("{3}{R}{R}"),
                "Bloodrush — {3}{R}{R}, Discard this card: Target attacking creature gets +3/+3 and gains double strike until end of turn.",
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Attacking,
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                })],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(ValueDef::Constant(3), ValueDef::Constant(3)),
                        AppliedEffectDef::add_ability(&abilities::double_strike()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// GTC 113 — Adaptive Snapjaw
pub(in crate::card::sets) static ADAPTIVE_SNAPJAW: CardRecord = CardRecord::new_with_legacy_id(
    1538,
    "Adaptive Snapjaw",
    CardArt::new("0d3c0c43-2d6d-49b8-a112-07611a23ae69", "Tomasz Jedruszek"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Lizard", "Beast"], 6, 2)
        .with_abilities(&[abilities::evolve()]),
);

// GTC 114 — Alpha Authority
// Audit: metadata-only — Combat constraints cannot limit an attacker to at most one blocker.
pub(in crate::card::sets) static ALPHA_AUTHORITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fbc42ca6-db90-41d0-8a4b-3217fb2c114c"),
    "Alpha Authority",
    crate::card::CardArt::new("fbc42ca6-db90-41d0-8a4b-3217fb2c114c", "Dave Kendall"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 115 — Burst of Strength
pub(in crate::card::sets) static BURST_OF_STRENGTH: CardRecord = CardRecord::new_with_legacy_id(
    1096,
    "Burst of Strength",
    CardArt::new("d1cbd617-9f2e-4882-b8f0-dfc2fced2281", "Marco Nelor"),
    CardSet::Gatecrash,
    CardRules::new_instant(mana_cost!("{G}")).with_ability(AbilityDef::spell_with_targets(
        "Put a +1/+1 counter on target creature and untap it.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ]),
    )),
);

// GTC 116 — Crocanura
pub(in crate::card::sets) static CROCANURA: CardRecord = CardRecord::new_with_legacy_id(
    1539,
    "Crocanura",
    CardArt::new("b459a988-97b0-4370-b89a-2565f8721b60", "Jack Wang"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Crocodile", "Frog"], 1, 3)
        .with_abilities(&[abilities::reach(), abilities::evolve()]),
);

static CROWNED_CERATOK_TRAMPLE: AbilityDef = abilities::trample();

// GTC 117 — Crowned Ceratok
pub(in crate::card::sets) static CROWNED_CERATOK: CardRecord = CardRecord::new_with_legacy_id(
    1629,
    "Crowned Ceratok",
    CardArt::new("a7eacc64-f418-4df0-bd8a-6b0036d0d2a1", "Steve Prescott"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Rhino"], 4, 3).with_abilities(&[
        abilities::trample(),
        AbilityDef::static_ability(
            "Each creature you control with a +1/+1 counter on it has trample.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&YOUR_COUNTERED_CREATURES),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&CROWNED_CERATOK_TRAMPLE),
            },
        ),
    ]),
);

// GTC 118 — Disciple of the Old Ways
pub(in crate::card::sets) static DISCIPLE_OF_THE_OLD_WAYS: CardRecord =
    CardRecord::new_with_legacy_id(
        1097,
        "Disciple of the Old Ways",
        CardArt::new("3c62b3ee-db2b-45c3-87d5-5d917ea4baeb", "Anthony Palumbo"),
        CardSet::Gatecrash,
        CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Warrior"], 2, 2).with_ability(
            AbilityDef::activated(
                "{R}: This creature gains first strike until end of turn.",
                &[AbilityCostDef::Mana(mana_cost!("{R}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ),
    );

// GTC 119 — Experiment One
pub(in crate::card::sets) static EXPERIMENT_ONE: CardRecord = CardRecord::new_with_legacy_id(
    1868,
    "Experiment One",
    CardArt::new("2cc1d8d0-bb43-4962-ad29-bb6478aa986b", "Chase Stone"),
    CardSet::Gatecrash,
    // Evolve banks the counters and the regeneration spends them, so the
    // same resource is both its size and its lives.
    CardRules::new_creature(mana_cost!("{G}"), &["Human", "Ooze"], 1, 1).with_abilities(&[
        abilities::evolve(),
        AbilityDef::activated(
            "Remove two +1/+1 counters from this creature: Regenerate it.",
            &[AbilityCostDef::RemoveCountersFromSource {
                kind: CounterKind::PlusOnePlusOne,
                amount: 2,
            }],
            EffectDef::Regenerate {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// GTC 120 — Forced Adaptation
pub(in crate::card::sets) static FORCED_ADAPTATION: CardRecord = CardRecord::new_with_legacy_id(
    1098,
    "Forced Adaptation",
    CardArt::new("a6527d61-e9d3-44d0-833e-19c072309270", "Trevor Claxton"),
    CardSet::Gatecrash,
    CardRules::new_enchantment(mana_cost!("{G}"))
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
            AbilityDef::triggered(
                "At the beginning of your upkeep, put a +1/+1 counter on enchanted creature.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::AttachedPermanent,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// GTC 121 — Giant Adephage
pub(in crate::card::sets) static GIANT_ADEPHAGE: CardRecord = CardRecord::new_with_legacy_id(
    2012,
    "Giant Adephage",
    CardArt::new("8bae725f-e582-4377-a855-51af035cdac3", "Christine Choi"),
    CardSet::Gatecrash,
    // Every connection doubles the swarm, which is why trample matters more
    // than the seven power does.
    CardRules::new_creature(mana_cost!("{5}{G}{G}"), &["Insect"], 7, 7).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, create a token that's a copy of this creature.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::CreateTokenCopyOf {
                object: EffectRecipientDef::Source,
                base_power_toughness: None,
            },
        ),
    ]),
);

// GTC 122 — Greenside Watcher
pub(in crate::card::sets) static GREENSIDE_WATCHER: CardRecord = CardRecord::new_with_legacy_id(
    1099,
    "Greenside Watcher",
    CardArt::new("e825cb2d-98d7-423d-9ba1-b4d04027027e", "Ryan Barger"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Elf", "Druid"], 2, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Untap target Gate.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Subtype("Gate"),
            )],
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ),
);

// GTC 123 — Gyre Sage
pub(in crate::card::sets) static GYRE_SAGE: CardRecord = CardRecord::new_with_legacy_id(
    1865,
    "Gyre Sage",
    CardArt::new("6345376a-3d2d-4fff-9430-5e90a96e2f0f", "Tyler Jacobson"),
    CardSet::Gatecrash,
    // Evolve feeds the mana ability: the counters it banks are exactly what
    // the tap reads, so a Sage that has never evolved taps for nothing.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Elf", "Druid"], 1, 2).with_abilities(&[
        abilities::evolve(),
        AbilityDef::activated_mana(
            "{T}: Add {G} for each +1/+1 counter on this creature.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddManaEqualTo {
                color: ManaColor::Green,
                amount: ValueDef::CountersOnSource(CounterKind::PlusOnePlusOne),
            },
        ),
    ]),
);

// GTC 124 — Hindervines
// Audit: metadata-only — Prevention effects cannot select combat-damage sources based on having no +1/+1 counters.
pub(in crate::card::sets) static HINDERVINES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f6b85a09-6d43-4798-8164-131d35b65836"),
    "Hindervines",
    crate::card::CardArt::new("f6b85a09-6d43-4798-8164-131d35b65836", "Svetlin Velinov"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 125 — Ivy Lane Denizen
pub(in crate::card::sets) static IVY_LANE_DENIZEN: CardRecord = CardRecord::new_with_legacy_id(
    1100,
    "Ivy Lane Denizen",
    CardArt::new("b95be874-93c0-4e05-9e5a-fe8f38bcb445", "Winona Nelson"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Elf", "Warrior"], 2, 3).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever another green creature you control enters, put a +1/+1 counter on target creature.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::Green),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]), None, Some(ZoneKind::Battlefield)),
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

// GTC 126 — Miming Slime
// Audit: metadata-only — No value computes the greatest power among creatures you control for a dynamically sized token.
pub(in crate::card::sets) static MIMING_SLIME: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("128d0584-89d2-499b-b6c2-2425c4ffcd13"),
    "Miming Slime",
    crate::card::CardArt::new("128d0584-89d2-499b-b6c2-2425c4ffcd13", "Svetlin Velinov"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// Naturalize first debuted in Onslaught; its GTC printing is registered in ADDITIONAL_PRINTINGS.

// GTC 127 — Naturalize (reprint)

// GTC 128 — Ooze Flux
// Audit: metadata-only — Needs removing an arbitrary number of +1/+1 counters distributed among creatures and a token sized by the amount removed.
pub(in crate::card::sets) static OOZE_FLUX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e9554369-7961-4cea-9da0-a1235805a26a"),
    "Ooze Flux",
    crate::card::CardArt::new("e9554369-7961-4cea-9da0-a1235805a26a", "Zoltan Boros"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

/// Power plus toughness, which is why a Wall is a fine thing to aim it at
/// and a Lightning-fast attacker often is not.
static PREDATORS_RAPPORT_TOTAL: SumValueDef = SumValueDef {
    left: ValueDef::TargetPower(TargetIndex::PRIMARY),
    right: ValueDef::TargetToughness(TargetIndex::PRIMARY),
};

// GTC 129 — Predator's Rapport
pub(in crate::card::sets) static PREDATORS_RAPPORT: CardRecord = CardRecord::new_with_legacy_id(
    1972,
    "Predator's Rapport",
    CardArt::new("47324ab7-df78-4859-be0b-2eef5d4f8082", "Matt Stewart"),
    CardSet::Gatecrash,
    CardRules::new_instant(mana_cost!("{2}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Choose target creature you control. You gain life equal to that creature's power plus its toughness.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
        )],
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Sum(&PREDATORS_RAPPORT_TOTAL),
        },
    )),
);

// GTC 130 — Rust Scarab
// Audit: metadata-only — Needs a becomes-blocked trigger and a target constrained to the captured defending player.
pub(in crate::card::sets) static RUST_SCARAB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c57335b5-30e2-431f-ab50-d5f1981783c3"),
    "Rust Scarab",
    crate::card::CardArt::new("c57335b5-30e2-431f-ab50-d5f1981783c3", "Adam Paquette"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 131 — Scab-Clan Charger
pub(in crate::card::sets) static SCAB_CLAN_CHARGER: CardRecord = CardRecord::new_with_legacy_id(
    1101,
    "Scab-Clan Charger",
    CardArt::new("964c88d3-3141-44ab-8856-44a3f08331ea", "Nils Hamm"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Centaur", "Warrior"], 2, 4).with_ability(
        abilities::bloodrush(
            mana_cost!("{1}{G}"),
            "Bloodrush — {1}{G}, Discard this card: Target attacking creature gets +2/+4 until end of turn.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Attacking,
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            })],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(4)),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// GTC 132 — Serene Remembrance
// Audit: metadata-only — Needs linked targets from one graveyard plus moving the resolving spell itself and all chosen cards into different owners' libraries before shuffling.
pub(in crate::card::sets) static SERENE_REMEMBRANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0f3da4b8-9dd6-455d-b24a-3d0207ae5ee8"),
    "Serene Remembrance",
    crate::card::CardArt::new("0f3da4b8-9dd6-455d-b24a-3d0207ae5ee8", "David Palumbo"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 133 — Skarrg Goliath
pub(in crate::card::sets) static SKARRG_GOLIATH: CardRecord = CardRecord::new_with_legacy_id(
    1102,
    "Skarrg Goliath",
    CardArt::new("2b2dcafd-eb72-4f3a-9c1c-ba17fe30bf0f", "Scott Chou"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{6}{G}{G}"), &["Beast"], 9, 9).with_abilities(&[
        abilities::trample(),
        abilities::bloodrush(
            mana_cost!("{5}{G}{G}"),
            "Bloodrush — {5}{G}{G}, Discard this card: Target attacking creature gets +9/+9 and gains trample until end of turn.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Attacking,
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            })],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(9), ValueDef::Constant(9)),
                    AppliedEffectDef::add_ability(&abilities::trample()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// GTC 134 — Slaughterhorn
pub(in crate::card::sets) static SLAUGHTERHORN: CardRecord = CardRecord::new_with_legacy_id(
    1103,
    "Slaughterhorn",
    CardArt::new("fb3fcc7a-ff5b-4695-aa86-9166f6cba565", "Steve Prescott"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Beast"], 3, 2).with_ability(
        abilities::bloodrush(
            mana_cost!("{G}"),
            "Bloodrush — {G}, Discard this card: Target attacking creature gets +3/+2 until end of turn.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Attacking,
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            })],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(3), ValueDef::Constant(2)),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// GTC 135 — Spire Tracer
pub(in crate::card::sets) static SPIRE_TRACER: CardRecord = CardRecord::new_with_legacy_id(
    1647,
    "Spire Tracer",
    CardArt::new(
        "428b0d43-94c9-4f7f-b042-ea63f88ac697",
        "Christopher Moeller",
    ),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Scout"], 1, 1).with_ability(
        AbilityDef::static_ability(
            "This creature can't be blocked except by creatures with flying or reach.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Reach),
                    ])),
                )),
            },
        ),
    ),
);

// GTC 136 — Sylvan Primordial
// Audit: metadata-only — Needs a successful-destruction continuation that searches for a Forest, puts it onto the battlefield tapped, and repeats per opponent.
pub(in crate::card::sets) static SYLVAN_PRIMORDIAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0483c869-38dc-4b0b-82f3-dd08a1ab985f"),
    "Sylvan Primordial",
    crate::card::CardArt::new("0483c869-38dc-4b0b-82f3-dd08a1ab985f", "Stephan Martiniere"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 137 — Tower Defense
pub(in crate::card::sets) static TOWER_DEFENSE: CardRecord = CardRecord::new_with_legacy_id(
    1104,
    "Tower Defense",
    CardArt::new("857e1eb2-f3f2-4c7f-9965-da9d7e385223", "Seb McKinnon"),
    CardSet::Gatecrash,
    CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell(
        "Creatures you control get +0/+5 and gain reach until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(5),
                ),
                AppliedEffectDef::add_ability(&abilities::reach()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// GTC 138 — Verdant Haven
// Audit: metadata-only — Triggered mana abilities cannot make the resolving one-of-five-colors choice required by the enchanted land's mana trigger.
pub(in crate::card::sets) static VERDANT_HAVEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c59109a0-fdab-49cb-bbf6-d405de4d1645"),
    "Verdant Haven",
    crate::card::CardArt::new("c59109a0-fdab-49cb-bbf6-d405de4d1645", "Daniel Ljunggren"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 139 — Wasteland Viper
pub(in crate::card::sets) static WASTELAND_VIPER: CardRecord = CardRecord::new_with_legacy_id(
    1105,
    "Wasteland Viper",
    CardArt::new("e4a5b2b8-3890-485f-8731-8f178a2da3d7", "Lucas Graciano"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{G}"), &["Snake"], 1, 2).with_abilities(&[
        abilities::deathtouch(),
        abilities::bloodrush(
            mana_cost!("{G}"),
            "Bloodrush — {G}, Discard this card: Target attacking creature gets +1/+2 and gains deathtouch until end of turn.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Attacking,
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            })],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(2)),
                    AppliedEffectDef::add_ability(&abilities::deathtouch()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// GTC 140 — Wildwood Rebirth
pub(in crate::card::sets) static WILDWOOD_REBIRTH: CardRecord = CardRecord::new_with_legacy_id(
    1106,
    "Wildwood Rebirth",
    CardArt::new("713a93a1-4442-4d5b-ad7a-136b87b5f7ab", "Dan Murayama Scott"),
    CardSet::Gatecrash,
    CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Return target creature card from your graveyard to your hand.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
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
    )),
);

// GTC 141 — Alms Beast
// Audit: metadata-only — Needs a dynamic combat-relation grant of lifelink to creatures blocking or blocked by the source.
pub(in crate::card::sets) static ALMS_BEAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ce441759-cd4c-4bcc-925e-08e8b60853c0"),
    "Alms Beast",
    crate::card::CardArt::new("ce441759-cd4c-4bcc-925e-08e8b60853c0", "Dan Murayama Scott"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 142 — Assemble the Legion
pub(in crate::card::sets) static ASSEMBLE_THE_LEGION: CardRecord = CardRecord::new_with_legacy_id(
    134,
    "Assemble the Legion",
    CardArt::new("43675ed7-ece1-4414-965e-9ebadcbf3dfb", "Eric Deschamps"),
    CardSet::Gatecrash,
    CardRules::new_enchantment(mana_cost!("{3}{R}{W}")).with_ability(
        AbilityDef::triggered(
            "At the beginning of your upkeep, put a muster counter on this enchantment. Then create a 1/1 red and white Soldier creature token with haste for each muster counter on this enchantment.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            // The counter goes on first, so the very first upkeep already
            // musters one Soldier.
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::Muster,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::create_creature_token(&["Soldier"], &[ManaColor::Red, ManaColor::White], 1, 1).with_abilities(&[abilities::haste()]).with_art(CardArt::new("aae7bdfe-fe14-4a18-b2b0-16e9175a0441", "Justine Cruz")).with_count(ValueDef::CountersOnSource(CounterKind::Muster)),
            ]),
        ),
    ),
);

// GTC 143 — Aurelia, the Warleader
pub(in crate::card::sets) static AURELIA_THE_WARLEADER: CardRecord = CardRecord::new_with_legacy_id(
    137,
    "Aurelia, the Warleader",
    CardArt::new("4ec18e35-05e4-4bfc-b32b-c3e71c95a71d", "Slawomir Maniak"),
    CardSet::Gatecrash,
    CardRules::new_creature(
        mana_cost!("{2}{R}{R}{W}{W}"),
        &["Angel"],
        3,
        4,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::flying(),
        abilities::vigilance(),
        abilities::haste(),
        AbilityDef::triggered(
            "Whenever Aurelia attacks for the first time each turn, untap all creatures you control. After this phase, there is an additional combat phase.",
            TriggerEventDef::attacks_first_time_this_turn(ObjectPredicateDef::Source),
            EffectDef::Sequence(&[
                EffectDef::Untap {
                    object: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::You),
                },
                EffectDef::ScheduleTurnPhases(&[TurnPhaseDef::Combat]),
            ]),
        ),
    ]),
);

// GTC 144 — Aurelia's Fury
// Audit: partial — Damage and target division work, but tap and cast-lock effects apply to selected targets even when prevention means they were not dealt damage this way.
pub(in crate::card::sets) static AURELIAS_FURY: CardRecord = CardRecord::new_with_legacy_id(
    136,
    "Aurelia's Fury",
    CardArt::new("1a3465b6-ee7f-4553-bbf1-85fae9734b67", "Tyler Jacobson"),
    CardSet::Gatecrash,
    CardRules::new_instant(mana_cost!("{X}{R}{W}")).with_ability(
        AbilityDef::spell_with_targets(
            "Aurelia's Fury deals X damage divided as you choose among any number of targets. Tap each creature dealt damage this way. Players dealt damage this way can't cast noncreature spells this turn.",
            &[AbilityTargetDef {
                predicate: AbilityTargetPredicate::AnyTarget,
                // "Any number of targets" is however many shares X splits into.
                minimum: 0,
                maximum: u8::MAX,
                divided_total: Some(DividedTotal::ChosenX),
                another: false,
            }],
            // Everything chosen took damage, so the tap and the lock are the
            // same set of targets read again; each ignores what it cannot
            // apply to.
            EffectDef::Sequence(&[
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::DividedAmongTargets,
                },
                EffectDef::Tap {
                    object: EffectRecipientDef::target_objects(TargetIndex::PRIMARY),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::target_players(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(
                        PlayRestrictionDef::new(
                            PlayActionMatcherDef::CastSpell,
                            ObjectPredicateDef::NoncreatureSpell,
                        ),
                    )),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        )
        .with_coverage(AbilityCoverageDef::partial(
            "Tap and cast-lock effects cannot be conditioned on damage actually being dealt after prevention.",
        )),
    ),
);

// GTC 145 — Bane Alley Broker
// Audit: metadata-only — Needs face-down linked exile from hand, permission to look at those and a non-target choice to return one.
pub(in crate::card::sets) static BANE_ALLEY_BROKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7996df7f-70f5-412c-9573-9512e4e131ac"),
    "Bane Alley Broker",
    crate::card::CardArt::new("7996df7f-70f5-412c-9573-9512e4e131ac", "Clint Cearley"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

static FOUR_BIOVISIONARIES: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef::matching(
        ObjectPredicateDef::SharesNameWithSource,
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    ),
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 4,
};

// GTC 146 — Biovisionary
pub(in crate::card::sets) static BIOVISIONARY: CardRecord = CardRecord::new_with_legacy_id(
    1107,
    "Biovisionary",
    CardArt::new("2000b4e8-7887-454e-9d52-211516613dd0", "Ryan Barger"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{1}{G}{U}"), &["Human", "Wizard"], 2, 3)
        .with_ability(AbilityDef::triggered_if(
            "At the beginning of the end step, if you control four or more creatures named Biovisionary, you win the game.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::Any,
            },
            &FOUR_BIOVISIONARIES,
            EffectDef::LoseTheGame {
                player: EffectRecipientDef::Opponent,
            },
        )),
);

// GTC 147 — Borborygmos Enraged
// Audit: metadata-only — Needs top-three reveal and partitioning plus a land-card discard cost that a resolving damage ability can identify.
pub(in crate::card::sets) static BORBORYGMOS_ENRAGED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8644c60f-7d06-4026-bcf3-df054701ca0a"),
    "Borborygmos Enraged",
    crate::card::CardArt::new("8644c60f-7d06-4026-bcf3-df054701ca0a", "Aleksi Briclot"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 148 — Boros Charm
pub(in crate::card::sets) static BOROS_CHARM: CardRecord = CardRecord::new_with_legacy_id(
    144,
    "Boros Charm",
    CardArt::new("d4ddf9cc-40a7-4b4f-bb51-b08171453c9a", "Zoltan Boros"),
    CardSet::Gatecrash,
    CardRules::new_instant(mana_cost!("{R}{W}")).with_ability(AbilityDef::choose_one_spell(
        "Choose one —\n• Boros Charm deals 4 damage to target player or planeswalker.\n• Permanents you control gain indestructible until end of turn.\n• Target creature gains double strike until end of turn.",
        &[
            AbilityDef::spell_with_targets("Boros Charm deals 4 damage to target player or planeswalker", &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )], EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(4),
                }),
            AbilityDef::spell(
                "Permanents you control gain indestructible until end of turn",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::Any, &[ZoneKind::Battlefield], PlayerRelation::You),
                    effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell_with_targets("Target creature gains double strike until end of turn", &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )], EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::double_strike()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                }),
        ],
    )),
);

// GTC 149 — Call of the Nightwing
// Audit: metadata-only — Cipher encoding and casting encoded spell copies without paying their mana costs are unavailable.
pub(in crate::card::sets) static CALL_OF_THE_NIGHTWING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ef30a570-b988-42e8-8910-41fe39ffa260"),
    "Call of the Nightwing",
    crate::card::CardArt::new("ef30a570-b988-42e8-8910-41fe39ffa260", "Adam Paquette"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 150 — Cartel Aristocrat
pub(in crate::card::sets) static CARTEL_ARISTOCRAT: CardRecord = CardRecord::new_with_legacy_id(
    1994,
    "Cartel Aristocrat",
    CardArt::new("25bcfbc0-1401-4e5e-8145-c8936c4ff725", "James Ryman"),
    CardSet::Gatecrash,
    // Unkillable for as long as the bodies last: the colour is named after
    // the removal spell is on the stack.
    CardRules::new_creature(mana_cost!("{W}{B}"), &["Human", "Advisor"], 2, 2).with_ability(
        AbilityDef::activated(
            "Sacrifice another creature: This creature gains protection from the color of your choice until end of turn.",
            &[AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                controller: PlayerRelation::You,
            }],
            EffectDef::ChooseColor {
                object: EffectRecipientDef::Source,
                operation: ColorChoiceOperationDef::ProtectionFromChosenColor,
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// GTC 151 — Clan Defiance
pub(in crate::card::sets) static CLAN_DEFIANCE: CardRecord = CardRecord::new_with_legacy_id(
    1648,
    "Clan Defiance",
    CardArt::new("efa05298-9c94-4179-b75a-49ee2ca92920", "Daarken"),
    CardSet::Gatecrash,
    CardRules::new_sorcery(mana_cost!("{X}{R}{G}")).with_ability(AbilityDef::modal_spell(
        "Choose one or more —\n• Clan Defiance deals X damage to target creature with flying.\n• Clan Defiance deals X damage to target creature without flying.\n• Clan Defiance deals X damage to target player or planeswalker.",
        &[
            AbilityDef::spell_with_targets(
                "Clan Defiance deals X damage to target creature with flying",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                    ]),
                )],
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::ChosenX,
                },
            ),
            AbilityDef::spell_with_targets(
                "Clan Defiance deals X damage to target creature without flying",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                            KeywordAbility::Flying,
                        )),
                    ]),
                )],
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::ChosenX,
                },
            ),
            AbilityDef::spell_with_targets(
                "Clan Defiance deals X damage to target player or planeswalker",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
                )],
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::ChosenX,
                },
            ),
        ],
        1,
        3,
        false,
    )),
);

// GTC 152 — Consuming Aberration
// Audit: metadata-only — Its cast trigger needs every opponent to reveal through a land and move each revealed group to a graveyard.
pub(in crate::card::sets) static CONSUMING_ABERRATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ca2a714e-4254-4138-ad36-4ae11da17289"),
    "Consuming Aberration",
    crate::card::CardArt::new("6354de66-f7f8-4e33-98d0-52624d3d7828", "Karl Kopinski"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 153 — Deathpact Angel
// Audit: metadata-only — The token's ability needs a non-target graveyard choice of a card with a specific name.
pub(in crate::card::sets) static DEATHPACT_ANGEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("81cfc3c5-6d69-443e-a506-76b94178979b"),
    "Deathpact Angel",
    crate::card::CardArt::new("81cfc3c5-6d69-443e-a506-76b94178979b", "Jason Chan"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 154 — Dimir Charm
// Audit: metadata-only — Needs a power-at-most target predicate with full static-effect semantics and a top-three choose-one/library-and-graveyard procedure.
pub(in crate::card::sets) static DIMIR_CHARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9f6bc1da-3969-4f19-b072-4ed79f906fef"),
    "Dimir Charm",
    crate::card::CardArt::new("f3f4cfa7-8ee4-4a85-9e6a-65a7541f62c1", "Zoltan Boros"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 155 — Dinrova Horror
pub(in crate::card::sets) static DINROVA_HORROR: CardRecord = CardRecord::new_with_legacy_id(
    1108,
    "Dinrova Horror",
    CardArt::new("398df5e6-6bda-467a-81e2-91be7e21d715", "Johann Bodin"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{4}{U}{B}"), &["Horror"], 4, 4).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature enters, return target permanent to its owner's hand, then that player discards a card.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Any,
            )],
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
                EffectDef::Discard {
                    recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            ]),
        ),
    ),
);

/// The available damage effects cover most of fight, but they resolve in
/// sequence instead of committing both damage events simultaneously.
static DOMRI_DOUBLE_STRIKE: AbilityDef = abilities::double_strike();
static DOMRI_TRAMPLE: AbilityDef = abilities::trample();
static DOMRI_HEXPROOF: AbilityDef = abilities::hexproof();
static DOMRI_HASTE: AbilityDef = abilities::haste();

static DOMRI_EMBLEM_KEYWORDS: [AppliedEffectDef; 4] = [
    AppliedEffectDef::add_ability(&DOMRI_DOUBLE_STRIKE),
    AppliedEffectDef::add_ability(&DOMRI_TRAMPLE),
    AppliedEffectDef::add_ability(&DOMRI_HEXPROOF),
    AppliedEffectDef::add_ability(&DOMRI_HASTE),
];

static DOMRI_RADE_EMBLEM_ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
    "Creatures you control have double strike, trample, hexproof, and haste.",
    EffectDef::StaticApply {
        recipient: EffectRecipientDef::matching_objects(
            ObjectPredicateDef::HasType(CardType::Creature),
            &[ZoneKind::Battlefield],
            PlayerRelation::You,
        ),
        effect: AppliedEffectDef::Composite(&DOMRI_EMBLEM_KEYWORDS),
    },
)];

static DOMRI_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::activated(
        "+1: Look at the top card of your library. If it's a creature card, you may reveal it and put it into your hand.",
        &[AbilityCostDef::Loyalty(1)],
        EffectDef::LookAtTopAndSelect {
            player: EffectRecipientDef::Controller,
            looker: EffectRecipientDef::Controller,
            selection: &TopCardSelectionDef {
                count: ValueDef::Constant(1),
                object: Some(ObjectPredicateDef::HasType(CardType::Creature)),
                minimum: 0,
                maximum: 1,
                select_all_matching: false,
                reveal_selected: true,
                selected_zone: ZoneKind::Hand,
                selected_placement: ZonePlacement::Top,
                rest_zone: ZoneKind::Library,
                rest_placement: ZonePlacement::Top,
                rest_random_order: false,
                rest_counters: None,
                selected_order_follows_choice: false,
                then: None,
                selected_hidden: false,
                selected_linked_to_source: false,
                selected_face_down: None,
            },
        },
    ),
    AbilityDef::activated_with_targets(
        "−2: Target creature you control fights another target creature.",
        &[AbilityCostDef::Loyalty(-2)],
        &DOMRI_FIGHT_TARGETS,
        EffectDef::Sequence(&[
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex(1)),
                amount: ValueDef::TargetPower(TargetIndex::PRIMARY),
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::TargetPower(TargetIndex(1)),
            },
        ]),
    )
    .with_coverage(AbilityCoverageDef::partial(
        "The two damage events resolve one after the other rather than at the same time.",
    )),
    AbilityDef::activated(
        "−7: You get an emblem with \"Creatures you control have double strike, trample, hexproof, and haste.\"",
        &[AbilityCostDef::Loyalty(-7)],
        EffectDef::create_emblem("Domri Rade emblem", &DOMRI_RADE_EMBLEM_ABILITIES),
    ),
];

static DOMRI_FIGHT_TARGETS: [AbilityTargetDef; 2] = [
    AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    }),
    AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    })
    .another(),
];

// GTC 156 — Domri Rade
// Audit: partial — The fight ability deals its two damage sequentially rather than at once; its other loyalty abilities are implemented.
pub(in crate::card::sets) static DOMRI_RADE: CardRecord = CardRecord::new_with_legacy_id(
    157,
    "Domri Rade",
    CardArt::new("21b48170-99dd-440f-9954-fc229d6094d3", "Tyler Jacobson"),
    CardSet::Gatecrash,
    CardRules::new_planeswalker(mana_cost!("{1}{R}{G}"), &["Domri"], 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&DOMRI_ABILITIES),
);

// GTC 157 — Drakewing Krasis
pub(in crate::card::sets) static DRAKEWING_KRASIS: CardRecord = CardRecord::new_with_legacy_id(
    1109,
    "Drakewing Krasis",
    CardArt::new("016d1d17-ba5c-4168-9a3d-232bdcc98c80", "Johann Bodin"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{1}{G}{U}"), &["Lizard", "Drake"], 3, 1)
        .with_abilities(&[abilities::flying(), abilities::trample()]),
);

// GTC 158 — Duskmantle Guildmage
// Audit: metadata-only — Needs a turn-long trigger for every card entering an opponent's graveyard, keyed to that card's owner.
pub(in crate::card::sets) static DUSKMANTLE_GUILDMAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9a1509ff-387e-4ccd-bda0-86e8738a98fb"),
    "Duskmantle Guildmage",
    crate::card::CardArt::new("9a1509ff-387e-4ccd-bda0-86e8738a98fb", "Slawomir Maniak"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 159 — Duskmantle Seer
// Audit: metadata-only — Needs simultaneous per-player top-card reveal, mana-value life loss, and movement to hand with APNAP handling.
pub(in crate::card::sets) static DUSKMANTLE_SEER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("63711861-87e0-4a63-8b7b-f834aa5f3f18"),
    "Duskmantle Seer",
    crate::card::CardArt::new("63711861-87e0-4a63-8b7b-f834aa5f3f18", "Kev Walker"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 160 — Elusive Krasis
pub(in crate::card::sets) static ELUSIVE_KRASIS: CardRecord = CardRecord::new_with_legacy_id(
    1541,
    "Elusive Krasis",
    CardArt::new("dd62e422-e5e2-4736-9ed3-d2dc693f6f8f", "Wesley Burt"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{1}{G}{U}"), &["Fish", "Mutant"], 0, 4).with_abilities(&[
        abilities::cannot_be_blocked("This creature can't be blocked."),
        abilities::evolve(),
    ]),
);

static EXECUTIONERS_SWING_TARGET: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one_permanent(
        ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::DealtDamageThisTurn,
        ]),
    )];

// GTC 161 — Executioner's Swing
pub(in crate::card::sets) static EXECUTIONERS_SWING: CardRecord = CardRecord::new_with_legacy_id(
    1906,
    "Executioner's Swing",
    CardArt::new("2122586d-9b23-47c2-8b00-e673aa0310f0", "Karl Kopinski"),
    CardSet::Gatecrash,
    // -5/-5 rather than destruction, so a big enough creature walks away and
    // indestructible does not save a small one.
    CardRules::new_instant(mana_cost!("{W}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature that dealt damage this turn gets -5/-5 until end of turn.",
        &EXECUTIONERS_SWING_TARGET,
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-5),
                ValueDef::Constant(-5),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// GTC 162 — Fathom Mage
// Audit: metadata-only — Evolve and a trigger for a +1/+1 counter being placed on the source are unavailable.
pub(in crate::card::sets) static FATHOM_MAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f67f79ac-f28d-4244-994e-44cb8819db61"),
    "Fathom Mage",
    crate::card::CardArt::new("4fa311f1-f11e-492d-9f18-e7489f950be7", "Ryan Pancoast"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 163 — Firemane Avenger
pub(in crate::card::sets) static FIREMANE_AVENGER: CardRecord = CardRecord::new_with_legacy_id(
    1512,
    "Firemane Avenger",
    CardArt::new("e244c198-efdc-492a-9c52-76aac006de9d", "Wayne Reynolds"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{2}{R}{W}"), &["Angel"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "Battalion — Whenever this creature and at least two other creatures attack, this \
             creature deals 3 damage to any target and you gain 3 life.",
            abilities::BATTALION_EVENT,
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::Sequence(&FIREMANE_AVENGER_BATTALION),
        ),
    ]),
);

static FIREMANE_AVENGER_BATTALION: [EffectDef; 2] = [
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: ValueDef::Constant(3),
    },
    EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(3),
    },
];

// GTC 164 — Fortress Cyclops
pub(in crate::card::sets) static FORTRESS_CYCLOPS: CardRecord = CardRecord::new_with_legacy_id(
    1754,
    "Fortress Cyclops",
    CardArt::new("bbfa583c-754e-4a63-aa1c-d1c1a5882f40", "Maciej Kuciara"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{3}{R}{W}"), &["Cyclops", "Soldier"], 3, 3).with_abilities(
        &[
            AbilityDef::triggered(
                "Whenever this creature attacks, it gets +3/+0 until end of turn.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::triggered(
                "Whenever this creature blocks, it gets +0/+3 until end of turn.",
                TriggerEventDef::Blocks {
                    blocked: ObjectPredicateDef::HasType(CardType::Creature),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(0),
                        ValueDef::Constant(3),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    ),
);

static FOUNDRY_CHAMPION_CREATURES: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

// GTC 165 — Foundry Champion
pub(in crate::card::sets) static FOUNDRY_CHAMPION: CardRecord = CardRecord::new_with_legacy_id(
    1110,
    "Foundry Champion",
    CardArt::new("84e39703-db78-4d3d-aacd-5396848253ed", "Todd Lockwood"),
    CardSet::Gatecrash,
    CardRules::new_creature(
        mana_cost!("{4}{R}{W}"),
        &["Elemental", "Soldier"],
        4,
        4,
    )
    .with_abilities(&[
        AbilityDef::triggered_with_targets(
            "When this creature enters, it deals damage to any target equal to the number of creatures you control.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)),
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::CountMatchingObjects(&FOUNDRY_CHAMPION_CREATURES),
            },
        ),
        AbilityDef::activated(
            "{R}: This creature gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(0)),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated(
            "{W}: This creature gets +0/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(0), ValueDef::Constant(1)),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

static FRENZIED_TILLING_PROGRAM: [EffectDef; 2] = [
    EffectDef::Destroy {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        can_regenerate: true,
    },
    // Tapped, so the land it fetches does not pay for anything this turn --
    // which is the whole reason a five-mana Stone Rain is playable.
    EffectDef::SearchZone {
        player: EffectRecipientDef::Controller,
        source: ZoneKind::Library,
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Land),
            ObjectPredicateDef::Supertype(CardSupertype::Basic),
        ]),
        minimum: 0,
        maximum: ValueDef::Constant(1),
        reveal: true,
        destination: ZoneKind::Battlefield,
        placement: ZonePlacement::Top,
        shuffle: true,
        enters_tapped: true,
        binding: None,
        then: None,
    },
];

// GTC 166 — Frenzied Tilling
pub(in crate::card::sets) static FRENZIED_TILLING: CardRecord = CardRecord::new_with_legacy_id(
    1980,
    "Frenzied Tilling",
    CardArt::new("03bce9a7-6215-43ff-b4d0-55f96f683aba", "Noah Bradley"),
    CardSet::Gatecrash,
    CardRules::new_sorcery(mana_cost!("{3}{R}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target land. Search your library for a basic land card, put that card onto the battlefield tapped, then shuffle.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Land),
        )],
        EffectDef::Sequence(&FRENZIED_TILLING_PROGRAM),
    )),
);

// GTC 167 — Ghor-Clan Rampager
pub(in crate::card::sets) static GHOR_CLAN_RAMPAGER: CardRecord = CardRecord::new_with_legacy_id(
    168,
    "Ghor-Clan Rampager",
    CardArt::new("382048ec-0bf5-49a5-90d5-f80fbda08962", "Charles Urbach"),
    CardSet::Gatecrash,
    CardRules::new_creature(
        mana_cost!("{2}{R}{G}"),
        &["Beast"],
        4,
        4,
    )
    .with_abilities(&[
        abilities::trample(),
        abilities::bloodrush(
            mana_cost!("{R}{G}"),
            "Bloodrush — {R}{G}, Discard this card: Target attacking creature gets +4/+4 and gains trample until end of turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Attacking,
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(4), ValueDef::Constant(4)),
                    AppliedEffectDef::add_ability(&abilities::trample()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

static GROUND_ASSAULT_LANDS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Land),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

// GTC 168 — Ground Assault
pub(in crate::card::sets) static GROUND_ASSAULT: CardRecord = CardRecord::new_with_legacy_id(
    1111,
    "Ground Assault",
    CardArt::new("a4220348-f030-4639-b1a9-6f61ac6bb6a8", "Karl Kopinski"),
    CardSet::Gatecrash,
    CardRules::new_sorcery(mana_cost!("{R}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Ground Assault deals damage to target creature equal to the number of lands you control.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::CountMatchingObjects(&GROUND_ASSAULT_LANDS),
        },
    )),
);

// GTC 169 — Gruul Charm
// Audit: metadata-only — Needs a turn-long cannot-block sweep, indefinite control restoration, and flying predicates with full continuous-effect semantics.
pub(in crate::card::sets) static GRUUL_CHARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9235afe5-0a6b-43c2-921c-18524cf032f1"),
    "Gruul Charm",
    crate::card::CardArt::new("9235afe5-0a6b-43c2-921c-18524cf032f1", "Zoltan Boros"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 170 — Gruul Ragebeast
// Audit: metadata-only — Fight damage must be simultaneous, and the entering triggering object must become one participant in a targeted fight.
pub(in crate::card::sets) static GRUUL_RAGEBEAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("080ef367-7904-4e5c-a8b4-1fb62f951f3e"),
    "Gruul Ragebeast",
    crate::card::CardArt::new("080ef367-7904-4e5c-a8b4-1fb62f951f3e", "Dave Kendall"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 171 — High Priest of Penance
pub(in crate::card::sets) static HIGH_PRIEST_OF_PENANCE: CardRecord =
    CardRecord::new_with_legacy_id(
        1112,
        "High Priest of Penance",
        CardArt::new("84a3ff8d-6d7e-49f0-8d30-7f8c23db568b", "Mark Zug"),
        CardSet::Gatecrash,
        CardRules::new_creature(mana_cost!("{W}{B}"), &["Human", "Cleric"], 1, 1).with_ability(
            AbilityDef::triggered_with_targets(
                "Whenever this creature is dealt damage, you may destroy target nonland permanent.",
                TriggerEventDef::damage_to_source(),
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                )],
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::Destroy {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        can_regenerate: true,
                    },
                },
            ),
        ),
    );

// GTC 172 — Hydroform
// Audit: metadata-only — Needs its target-land animation authored from the shared card-type, creature-type, power/toughness, and ability operations.
pub(in crate::card::sets) static HYDROFORM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2c4e1f41-4aed-451b-bbaa-6cc6780cd6c9"),
    "Hydroform",
    crate::card::CardArt::new("2c4e1f41-4aed-451b-bbaa-6cc6780cd6c9", "Howard Lyon"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 173 — Kingpin's Pet
pub(in crate::card::sets) static KINGPINS_PET: CardRecord = CardRecord::new_with_legacy_id(
    1113,
    "Kingpin's Pet",
    CardArt::new("3465cf63-4f10-4b53-9703-69746364dbc7", "Mark Zug"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{1}{W}{B}"), &["Thrull"], 2, 2)
        .with_abilities(&[abilities::flying(), abilities::extort()]),
);

// GTC 174 — Lazav, Dimir Mastermind
// Audit: metadata-only — Copy effects cannot copy a creature card from a graveyard while retaining the source's name, legendary supertype, hexproof, and trigger.
pub(in crate::card::sets) static LAZAV_DIMIR_MASTERMIND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("69c8fcdb-4798-4961-995a-e128a3ff431a"),
    "Lazav, Dimir Mastermind",
    crate::card::CardArt::new("69c8fcdb-4798-4961-995a-e128a3ff431a", "David Rapoza"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 175 — Martial Glory
pub(in crate::card::sets) static MARTIAL_GLORY: CardRecord = CardRecord::new_with_legacy_id(
    1114,
    "Martial Glory",
    CardArt::new("3690c96c-70a3-45e5-84d7-5c82809a8f45", "Raymond Swanland"),
    CardSet::Gatecrash,
    CardRules::new_instant(mana_cost!("{R}{W}")).with_ability(
        AbilityDef::spell_with_targets(
            "Target creature gets +3/+0 until end of turn. Target creature gets +0/+3 until end of turn.",
            &[
                AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Creature,
                )),
                AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Creature,
                )),
            ],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(3), ValueDef::Constant(0)),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex(1)),
                    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(0), ValueDef::Constant(3)),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ),
);

// GTC 176 — Master Biomancer
// Audit: metadata-only — Entry replacement effects cannot read this source's power or add a creature subtype to another entering creature.
pub(in crate::card::sets) static MASTER_BIOMANCER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1a47da7c-80f3-4b98-aaac-778c34a35cb6"),
    "Master Biomancer",
    crate::card::CardArt::new("1a47da7c-80f3-4b98-aaac-778c34a35cb6", "Willian Murai"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 177 — Merciless Eviction
// Audit: metadata-only — Needs a simultaneous mass zone move for each selectable permanent type.
pub(in crate::card::sets) static MERCILESS_EVICTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d9876a4c-714b-47e5-9589-148a623af96a"),
    "Merciless Eviction",
    crate::card::CardArt::new("d9876a4c-714b-47e5-9589-148a623af96a", "Richard Wright"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 178 — Mind Grind
// Audit: metadata-only — Needs each opponent to reveal through X lands and move every revealed group to a graveyard, plus the X-cannot-be-zero cast restriction.
pub(in crate::card::sets) static MIND_GRIND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("671eb9e8-1a69-4570-8972-2e7de371cef4"),
    "Mind Grind",
    crate::card::CardArt::new("671eb9e8-1a69-4570-8972-2e7de371cef4", "Daarken"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 179 — Mortus Strider
pub(in crate::card::sets) static MORTUS_STRIDER: CardRecord = CardRecord::new_with_legacy_id(
    1115,
    "Mortus Strider",
    CardArt::new("d644eb6e-cc49-4834-bc2c-53f6a4ceb451", "Tomasz Jedruszek"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{1}{U}{B}"), &["Skeleton"], 1, 1).with_ability(
        AbilityDef::triggered(
            "When this creature dies, return it to its owner's hand.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::MoveToZone {
                counters: None,
                object: EffectRecipientDef::TriggeringObject,
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
                controller: None,
            },
        ),
    ),
);

// GTC 180 — Mystic Genesis
// Audit: metadata-only — Authored token characteristics are static, and no continuation can size a newly created Ooze from the countered spell's mana value.
pub(in crate::card::sets) static MYSTIC_GENESIS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ae1dd1ac-1a1e-485d-a11f-d1323a69f95e"),
    "Mystic Genesis",
    crate::card::CardArt::new("ae1dd1ac-1a1e-485d-a11f-d1323a69f95e", "Mike Bierek"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 181 — Nimbus Swimmer
// Audit: metadata-only — Battlefield-entry counter modifications take fixed amounts and cannot read the creature spell's chosen X.
pub(in crate::card::sets) static NIMBUS_SWIMMER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d691cd3b-afe5-4f28-95a9-125475515126"),
    "Nimbus Swimmer",
    crate::card::CardArt::new("d691cd3b-afe5-4f28-95a9-125475515126", "Howard Lyon"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 182 — Obzedat, Ghost Council
pub(in crate::card::sets) static OBZEDAT_GHOST_COUNCIL: CardRecord = CardRecord::new_with_legacy_id(
    193,
    "Obzedat, Ghost Council",
    CardArt::new("4cc198d8-1f27-482d-8f5d-21e02c59797a", "Svetlin Velinov"),
    CardSet::Gatecrash,
    CardRules::new_creature(
        mana_cost!("{1}{W}{W}{B}{B}"),
        &["Spirit", "Advisor"],
        5,
        5,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        AbilityDef::triggered_with_targets("When Obzedat enters, target opponent loses 2 life and you gain 2 life.", TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)), &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Opponent),
        )], EffectDef::Sequence(&[
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(2),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ])),
        AbilityDef::triggered(
            "At the beginning of your end step, you may exile Obzedat. If you do, return it to the battlefield under its owner's control at the beginning of your next upkeep. It gains haste.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Sequence(&[
                EffectDef::ExileLinkedToSource {
                    object: EffectRecipientDef::Source,
                },
                // Queued before the exile takes effect would be the same:
                // both read the source from the resolving ability, which the
                // exile does not disturb.
                EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                    "At the beginning of your next upkeep, return the exiled cards to the battlefield under their owner's control. It gains haste.",
                    TriggerEventDef::StepBegins {
                        step: TurnStepDef::Upkeep,
                        player: PlayerRelation::You,
                    },
                    EffectDef::ReturnLinkedExiles {
                        object: ObjectPredicateDef::Any,
                        counters: None,
                        arrival_effect: None,
                        zone: ZoneKind::Battlefield,
                        grant: Some(KeywordAbility::Haste),
                        controller: None,
                        transformed: false,
                    },
                ))),
                ]),
            },
        ),
    ]),
);

static ONE_THOUSAND_LASHES_PROHIBITIONS: [AppliedEffectDef; 3] = [
    AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
    AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
    AppliedEffectDef::Rule(AppliedRuleDef::CannotActivateAbilities),
];

// GTC 183 — One Thousand Lashes
pub(in crate::card::sets) static ONE_THOUSAND_LASHES: CardRecord = CardRecord::new_with_legacy_id(
    1955,
    "One Thousand Lashes",
    CardArt::new("eef2d548-477b-4be1-b946-6df6aac2ee6e", "Daarken"),
    CardSet::Gatecrash,
    // The drain follows the creature rather than the Aura, so stealing the
    // creature takes the life loss with it.
    CardRules::new_enchantment(mana_cost!("{2}{W}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature can't attack or block, and its activated abilities can't be \
                 activated.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&ONE_THOUSAND_LASHES_PROHIBITIONS),
                },
            ),
            abilities::enchanted_controller_upkeep(
                "At the beginning of the upkeep of enchanted creature's controller, that player \
                 loses 1 life.",
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                        ObjectRefDef::AttachedToSource,
                    )),
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// GTC 184 — Ordruun Veteran
pub(in crate::card::sets) static ORDRUUN_VETERAN: CardRecord = CardRecord::new_with_legacy_id(
    1513,
    "Ordruun Veteran",
    CardArt::new("20fea3f6-e64a-4964-86bc-c0b8fef0ab25", "Greg Staples"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{2}{R}{W}"), &["Minotaur", "Soldier"], 3, 1).with_abilities(&[
        abilities::battalion(
            "Battalion — Whenever this creature and at least two other creatures attack, this creature gains double strike until end of turn.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&BATTALION_DOUBLE_STRIKE),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// GTC 185 — Orzhov Charm
// Audit: metadata-only — Needs returning all Auras attached to one target, a target-toughness life-loss value, and a graveyard target predicate with dynamic mana-value semantics.
pub(in crate::card::sets) static ORZHOV_CHARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8ca44265-5e1b-4fbf-9002-52b2ce9b7448"),
    "Orzhov Charm",
    crate::card::CardArt::new("8ca44265-5e1b-4fbf-9002-52b2ce9b7448", "Zoltan Boros"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 186 — Paranoid Delusions
// Audit: metadata-only — Cipher encoding and casting encoded spell copies without paying their mana costs are unavailable.
pub(in crate::card::sets) static PARANOID_DELUSIONS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("af406038-91ce-41fe-8b6d-55408a96d0a2"),
    "Paranoid Delusions",
    crate::card::CardArt::new(
        "af406038-91ce-41fe-8b6d-55408a96d0a2",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 187 — Primal Visitation
pub(in crate::card::sets) static PRIMAL_VISITATION: CardRecord = CardRecord::new_with_legacy_id(
    1116,
    "Primal Visitation",
    CardArt::new(
        "2dd518e8-047a-4df0-a0b8-ba116d048fa8",
        "Christopher Moeller",
    ),
    CardSet::Gatecrash,
    CardRules::new_enchantment(mana_cost!("{3}{R}{G}"))
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
                "Enchanted creature gets +3/+3 and has haste.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(3),
                            ValueDef::Constant(3),
                        ),
                        AppliedEffectDef::add_ability(&abilities::haste()),
                    ]),
                },
            ),
        ]),
);

// GTC 188 — Prime Speaker Zegana
// Audit: metadata-only — Entry replacement effects cannot read the greatest power among other creatures, and the entry trigger needs the source's post-entry power as a draw count.
pub(in crate::card::sets) static PRIME_SPEAKER_ZEGANA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f30dfb8e-f540-45ab-a4e8-63425099646a"),
    "Prime Speaker Zegana",
    crate::card::CardArt::new("f30dfb8e-f540-45ab-a4e8-63425099646a", "Willian Murai"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 189 — Psychic Strike
pub(in crate::card::sets) static PSYCHIC_STRIKE: CardRecord = CardRecord::new_with_legacy_id(
    1117,
    "Psychic Strike",
    CardArt::new("0d87927c-80a6-4146-92a5-58c510ce7958", "Mathias Kollros"),
    CardSet::Gatecrash,
    CardRules::new_instant(mana_cost!("{1}{U}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell. Its controller mills two cards.",
        &[AbilityTargetDef::exactly_one_spell(
            ObjectPredicateDef::Spell,
        )],
        EffectDef::Sequence(&[
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Graveyard,
            },
            EffectDef::Mill {
                player: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
                binding: None,
                then: None,
            },
        ]),
    )),
);

// GTC 190 — Purge the Profane
// Audit: metadata-only — Recipient-chosen discard suspends for a decision, so a following sequence effect would gain life before the printed discard finishes.
pub(in crate::card::sets) static PURGE_THE_PROFANE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("937ee156-0105-4291-8c67-d03a59c24679"),
    "Purge the Profane",
    crate::card::CardArt::new("937ee156-0105-4291-8c67-d03a59c24679", "Michael C. Hayes"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 191 — Rubblehulk
// Audit: metadata-only — Needs a characteristic-defining power/toughness ability that functions in every zone and a bloodrush value that counts lands at resolution.
pub(in crate::card::sets) static RUBBLEHULK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3422b1bf-c5b6-4d1c-bcea-8e81b83b996e"),
    "Rubblehulk",
    crate::card::CardArt::new("8c502590-f780-4512-8067-7c66f16f8c9d", "Raymond Swanland"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 192 — Ruination Wurm
pub(in crate::card::sets) static RUINATION_WURM: CardRecord = CardRecord::new_with_legacy_id(
    1118,
    "Ruination Wurm",
    CardArt::new("ce04d1ee-2605-472d-b3ee-24800342e9af", "Dave Kendall"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{4}{R}{G}"), &["Wurm"], 7, 6),
);

// GTC 193 — Shambleshark
pub(in crate::card::sets) static SHAMBLESHARK: CardRecord = CardRecord::new_with_legacy_id(
    1540,
    "Shambleshark",
    CardArt::new("07855a17-4e68-4257-af7f-275c9fb0a9b8", "Wesley Burt"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{G}{U}"), &["Shark", "Crab"], 2, 1)
        .with_abilities(&[abilities::flash(), abilities::evolve()]),
);

// GTC 194 — Signal the Clans
// Audit: metadata-only — Needs an exactly-three library search, distinct-name validation, random selection, and shuffling the unselected cards.
pub(in crate::card::sets) static SIGNAL_THE_CLANS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("34261992-db62-49e3-95bc-1b2960868de4"),
    "Signal the Clans",
    crate::card::CardArt::new("34261992-db62-49e3-95bc-1b2960868de4", "Dave Kendall"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 195 — Simic Charm
pub(in crate::card::sets) static SIMIC_CHARM: CardRecord = CardRecord::new_with_legacy_id(
    1119,
    "Simic Charm",
    CardArt::new("97c27bdd-77f5-4e93-8f54-93a204fc980a", "Zoltan Boros"),
    CardSet::Gatecrash,
    CardRules::new_instant(mana_cost!("{G}{U}")).with_ability(AbilityDef::choose_one_spell(
        "Choose one —\n• Target creature gets +3/+3 until end of turn.\n• Permanents you control gain hexproof until end of turn.\n• Return target creature to its owner's hand.",
        &[
            AbilityDef::spell_with_targets(
                "Target creature gets +3/+3 until end of turn",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(3), ValueDef::Constant(3)),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell(
                "Permanents you control gain hexproof until end of turn",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::Any, &[ZoneKind::Battlefield], PlayerRelation::You),
                    effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell_with_targets(
                "Return target creature to its owner's hand",
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
        ],
    )),
);

// GTC 196 — Skarrg Guildmage
// Audit: metadata-only — Needs its land animation migrated to the shared card-type, creature-type, and power/toughness operations.
pub(in crate::card::sets) static SKARRG_GUILDMAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("00f206c0-a8a7-4ca0-b88f-4736c3dac588"),
    "Skarrg Guildmage",
    crate::card::CardArt::new("00f206c0-a8a7-4ca0-b88f-4736c3dac588", "Aleksi Briclot"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 197 — Skyknight Legionnaire
pub(in crate::card::sets) static SKYKNIGHT_LEGIONNAIRE: CardRecord = CardRecord::new_with_legacy_id(
    1120,
    "Skyknight Legionnaire",
    CardArt::new("ae8c9948-b52e-4d07-a72a-99ab6be05cc6", "Anthony Palumbo"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{1}{R}{W}"), &["Human", "Knight"], 2, 2)
        .with_abilities(&[abilities::flying(), abilities::haste()]),
);

// GTC 198 — Soul Ransom
// Audit: metadata-only — Needs indefinite control from an Aura and an activated ability restricted to opponents that makes the Aura's controller sacrifice it before drawing.
pub(in crate::card::sets) static SOUL_RANSOM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fd705732-cce9-4d11-85ca-be49381dbaa8"),
    "Soul Ransom",
    crate::card::CardArt::new("fd705732-cce9-4d11-85ca-be49381dbaa8", "Steve Argyle"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 199 — Spark Trooper
pub(in crate::card::sets) static SPARK_TROOPER: CardRecord = CardRecord::new_with_legacy_id(
    1121,
    "Spark Trooper",
    CardArt::new("09eb69b5-b8e2-48c6-8c27-cb0108df8dad", "James Ryman"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{1}{R}{R}{W}"), &["Elemental", "Soldier"], 6, 1)
        .with_abilities(&[
            abilities::trample(),
            abilities::lifelink(),
            abilities::haste(),
            AbilityDef::triggered(
                "At the beginning of the end step, sacrifice this creature.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::Any,
                },
                EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            ),
        ]),
);

// GTC 200 — Sunhome Guildmage
pub(in crate::card::sets) static SUNHOME_GUILDMAGE: CardRecord = CardRecord::new_with_legacy_id(
    1122,
    "Sunhome Guildmage",
    CardArt::new("42d1122a-099b-49bf-9b53-52429658816a", "Eric Deschamps"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{R}{W}"), &["Human", "Wizard"], 2, 2).with_abilities(&[
        AbilityDef::activated(
            "{1}{R}{W}: Creatures you control get +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{R}{W}"))],
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
        ),
        AbilityDef::activated(
            "{2}{R}{W}: Create a 1/1 red and white Soldier creature token with haste.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{R}{W}"))],
            EffectDef::create_creature_token(
                &["Soldier"],
                &[ManaColor::Red, ManaColor::White],
                1,
                1,
            )
            .with_abilities(&[abilities::haste()])
            .with_art(CardArt::new(
                "aae7bdfe-fe14-4a18-b2b0-16e9175a0441",
                "Justine Cruz",
            )),
        ),
    ]),
);

// GTC 201 — Treasury Thrull
pub(in crate::card::sets) static TREASURY_THRULL: CardRecord = CardRecord::new_with_legacy_id(
    1123,
    "Treasury Thrull",
    CardArt::new("f013e6f0-85d0-4c8e-a10b-7beea572c32d", "Mark Zug"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{4}{W}{B}"), &["Thrull"], 4, 4).with_abilities(&[
        abilities::extort(),
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, you may return target artifact, creature, or enchantment card from your graveyard to your hand.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                ]),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            })],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::MoveToZone {
                    counters: None,
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    arrival_effect: None,
                    attachment: None,
                    controller: None,
                },
            },
        ),
    ]),
);

// GTC 202 — Truefire Paladin
pub(in crate::card::sets) static TRUEFIRE_PALADIN: CardRecord = CardRecord::new_with_legacy_id(
    1124,
    "Truefire Paladin",
    CardArt::new("39610192-6d3c-4d03-9c3e-cda966c924b1", "Michael C. Hayes"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{R}{W}"), &["Human", "Knight"], 2, 2).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::activated(
            "{R}{W}: This creature gets +2/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{R}{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated(
            "{R}{W}: This creature gains first strike until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{R}{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// GTC 203 — Unexpected Results
// Audit: metadata-only — Needs shuffle-then-reveal branching, casting a nonland without paying its mana cost, or putting a land onto the battlefield and returning the resolving spell.
pub(in crate::card::sets) static UNEXPECTED_RESULTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("65e10a22-6070-48d7-99ab-45f770f16fd1"),
    "Unexpected Results",
    crate::card::CardArt::new("65e10a22-6070-48d7-99ab-45f770f16fd1", "Mike Bierek"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 204 — Urban Evolution
// Audit: metadata-only — Needs a turn-long permission to play one additional land.
pub(in crate::card::sets) static URBAN_EVOLUTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9fcd6fac-2cde-4a89-b484-b910be2dcecf"),
    "Urban Evolution",
    crate::card::CardArt::new("9fcd6fac-2cde-4a89-b484-b910be2dcecf", "Eytan Zana"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 205 — Vizkopa Confessor
// Audit: metadata-only — Needs an arbitrary life payment, partial hand reveal, and a resolving choice by the ability controller from the revealed group.
pub(in crate::card::sets) static VIZKOPA_CONFESSOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cac639b9-6c20-45ac-a61f-082bdcebdb83"),
    "Vizkopa Confessor",
    crate::card::CardArt::new("cac639b9-6c20-45ac-a61f-082bdcebdb83", "Ryan Pancoast"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 206 — Vizkopa Guildmage
// Audit: metadata-only — Needs an installed-trigger lifetime that expires at cleanup for a printed “this turn” effect.
pub(in crate::card::sets) static VIZKOPA_GUILDMAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("41f505a3-2c54-43a9-b4a9-43a1451b36f6"),
    "Vizkopa Guildmage",
    crate::card::CardArt::new("41f505a3-2c54-43a9-b4a9-43a1451b36f6", "Tyler Jacobson"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 207 — Whispering Madness
// Audit: metadata-only — Needs simultaneous whole-hand discard with the greatest discarded count, plus cipher encoding and free copy casting.
pub(in crate::card::sets) static WHISPERING_MADNESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("64e4b0cc-e611-4a4b-8392-b37bfc3a77e1"),
    "Whispering Madness",
    crate::card::CardArt::new("64e4b0cc-e611-4a4b-8392-b37bfc3a77e1", "Clint Cearley"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 208 — Wojek Halberdiers
pub(in crate::card::sets) static WOJEK_HALBERDIERS: CardRecord = CardRecord::new_with_legacy_id(
    1514,
    "Wojek Halberdiers",
    CardArt::new("423f0870-dc1c-4cd8-b92c-6d5f92abbaec", "Nic Klein"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{R}{W}"), &["Human", "Soldier"], 3, 2).with_abilities(&[
        abilities::battalion(
            "Battalion — Whenever this creature and at least two other creatures attack, this creature gains first strike until end of turn.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&BATTALION_FIRST_STRIKE),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// GTC 209 — Zameck Guildmage
// Audit: metadata-only — Needs a turn-long entry replacement on future creatures and removing a +1/+1 counter from a chosen creature as a cost.
pub(in crate::card::sets) static ZAMECK_GUILDMAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ed0bdf8f-9086-4e30-8865-bb1eb55516f8"),
    "Zameck Guildmage",
    crate::card::CardArt::new("feeaf99b-7720-42e3-8cb1-23218b646458", "Chase Stone"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 210 — Zhur-Taa Swine
pub(in crate::card::sets) static ZHUR_TAA_SWINE: CardRecord = CardRecord::new_with_legacy_id(
    1125,
    "Zhur-Taa Swine",
    CardArt::new("cef93050-2f24-4c85-a00b-796e53868ac1", "Yeong-Hao Han"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{3}{R}{G}"), &["Boar"], 5, 4).with_ability(
        abilities::bloodrush(
            mana_cost!("{1}{R}{G}"),
            "Bloodrush — {1}{R}{G}, Discard this card: Target attacking creature gets +5/+4 until end of turn.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Attacking,
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            })],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(5), ValueDef::Constant(4)),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// GTC 211 — Arrows of Justice
pub(in crate::card::sets) static ARROWS_OF_JUSTICE: CardRecord = CardRecord::new_with_legacy_id(
    1126,
    "Arrows of Justice",
    CardArt::new("c64a15f4-6e2f-4479-95da-8805ce2091fa", "James Ryman"),
    CardSet::Gatecrash,
    CardRules::new_instant(mana_cost!("{2}{R/W}")).with_ability(AbilityDef::spell_with_targets(
        "Arrows of Justice deals 4 damage to target attacking or blocking creature.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AttackingOrBlocking,
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(4),
        },
    )),
);

// GTC 212 — Beckon Apparition
pub(in crate::card::sets) static BECKON_APPARITION: CardRecord = CardRecord::new_with_legacy_id(
    1127,
    "Beckon Apparition",
    CardArt::new("8b2ef9c5-ca6f-4243-bd38-2b325257831c", "Cliff Childs"),
    CardSet::Gatecrash,
    CardRules::new_instant(mana_cost!("{W/B}")).with_ability(
        AbilityDef::spell_with_targets(
            "Exile target card from a graveyard. Create a 1/1 white and black Spirit creature token with flying.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Any,
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: None,
            })],
            EffectDef::Sequence(&[
                EffectDef::MoveToZone {
                    counters: None,
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Exile,
                    placement: ZonePlacement::Top,
                    arrival_effect: None,
                    attachment: None,
                    controller: None,
                },
                EffectDef::create_creature_token(&["Spirit"], &[ManaColor::White, ManaColor::Black], 1, 1).with_abilities(&[abilities::flying()]).with_art(CardArt::new("91f3a4b0-0992-4245-b245-033ad1083a93", "Cliff Childs")),
            ]),
        ),
    ),
);

// GTC 213 — Biomass Mutation
// Audit: metadata-only — Temporary continuous effects cannot set base power and toughness to the spell's chosen X.
pub(in crate::card::sets) static BIOMASS_MUTATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fbe0a11d-0390-437e-8ece-3229863c76db"),
    "Biomass Mutation",
    crate::card::CardArt::new("fbe0a11d-0390-437e-8ece-3229863c76db", "Dan Murayama Scott"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 214 — Bioshift
// Audit: metadata-only — Needs choosing a variable number of +1/+1 counters to move and constraining two targets to have the same controller.
pub(in crate::card::sets) static BIOSHIFT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6e18f7a9-2af6-467a-8f62-5f7da83a3c92"),
    "Bioshift",
    crate::card::CardArt::new("6e18f7a9-2af6-467a-8f62-5f7da83a3c92", "Scott Chou"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 215 — Boros Reckoner
pub(in crate::card::sets) static BOROS_RECKONER: CardRecord = CardRecord::new_with_legacy_id(
    145,
    "Boros Reckoner",
    CardArt::new("82a18b07-38b8-4854-9735-3cfe83b11bf1", "Howard Lyon"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{R/W}{R/W}{R/W}"), &["Minotaur", "Wizard"], 3, 3)
        .with_abilities(&[
            AbilityDef::triggered_with_targets(
                "Whenever this creature is dealt damage, it deals that much damage to any target.",
                TriggerEventDef::damage_to_source(),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::AnyTarget,
                )],
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::TriggerEventAmount,
                },
            ),
            AbilityDef::activated(
                "{R/W}: This creature gains first strike until end of turn.",
                &[AbilityCostDef::Mana(mana_cost!("{R/W}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// GTC 216 — Burning-Tree Emissary
pub(in crate::card::sets) static BURNING_TREE_EMISSARY: CardRecord = CardRecord::new_with_legacy_id(
    1128,
    "Burning-Tree Emissary",
    CardArt::new("899d5f35-3613-4c69-9176-13baf442fb50", "Izzy"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{R/G}{R/G}"), &["Human", "Shaman"], 2, 2).with_ability(
        AbilityDef::triggered(
            "When this creature enters, add {R}{G}.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::Sequence(&[
                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
            ]),
        ),
    ),
);

// GTC 217 — Coerced Confession
// Audit: metadata-only — Needs a linked count of creature cards among exactly the cards milled by the preceding effect.
pub(in crate::card::sets) static COERCED_CONFESSION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d76102f1-b05f-472b-81cf-eae424c55638"),
    "Coerced Confession",
    crate::card::CardArt::new("d76102f1-b05f-472b-81cf-eae424c55638", "Mathias Kollros"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 218 — Deathcult Rogue
pub(in crate::card::sets) static DEATHCULT_ROGUE: CardRecord = CardRecord::new_with_legacy_id(
    1129,
    "Deathcult Rogue",
    CardArt::new("a4c186d2-e631-4811-83ea-fdb54e730a5d", "David Palumbo"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{1}{U/B}{U/B}"), &["Human", "Rogue"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "This creature can't be blocked except by Rogues.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Rogue")),
                )),
            },
        ),
    ),
);

// GTC 219 — Gift of Orzhova
pub(in crate::card::sets) static GIFT_OF_ORZHOVA: CardRecord = CardRecord::new_with_legacy_id(
    1130,
    "Gift of Orzhova",
    CardArt::new("3d759ef8-cb04-4769-9944-85793af3f6e8", "Johannes Voss"),
    CardSet::Gatecrash,
    CardRules::new_enchantment(mana_cost!("{1}{W/B}{W/B}"))
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
                "Enchanted creature gets +1/+1 and has flying and lifelink.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                        AppliedEffectDef::add_ability(&abilities::lifelink()),
                    ]),
                },
            ),
        ]),
);

// GTC 220 — Immortal Servitude
// Audit: metadata-only — Needs a simultaneous multi-card graveyard-to-battlefield move filtered by the spell's chosen X.
pub(in crate::card::sets) static IMMORTAL_SERVITUDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("88366762-4be0-422a-a0ff-ed046d30afe1"),
    "Immortal Servitude",
    crate::card::CardArt::new("88366762-4be0-422a-a0ff-ed046d30afe1", "Seb McKinnon"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 221 — Merfolk of the Depths
pub(in crate::card::sets) static MERFOLK_OF_THE_DEPTHS: CardRecord = CardRecord::new_with_legacy_id(
    1131,
    "Merfolk of the Depths",
    CardArt::new("bddb2e15-a53e-4647-a627-6c7032429fca", "Scott Chou"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{4}{G/U}{G/U}"), &["Merfolk", "Soldier"], 4, 2)
        .with_ability(abilities::flash()),
);

// GTC 222 — Nightveil Specter
// Audit: metadata-only — Needs combat-damage-player capture, linked face-up exile, and permission to play lands or cast spells exiled by this source.
pub(in crate::card::sets) static NIGHTVEIL_SPECTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bd7305f2-6e91-4d30-961b-b74a830d6eb3"),
    "Nightveil Specter",
    crate::card::CardArt::new("e3754b8c-16d2-41e3-b41b-4b2e70833e82", "Min Yum"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 223 — Pit Fight
// Audit: metadata-only — Fight requires two simultaneous damage events and the target declaration cannot enforce “another” target creature.
pub(in crate::card::sets) static PIT_FIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3624332e-60fc-4819-a0f1-d7ec41c6518b"),
    "Pit Fight",
    crate::card::CardArt::new("3624332e-60fc-4819-a0f1-d7ec41c6518b", "Matt Stewart"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

static RUBBLEBELT_ATTACKERS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::Attacking,
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

// GTC 224 — Rubblebelt Raiders
pub(in crate::card::sets) static RUBBLEBELT_RAIDERS: CardRecord = CardRecord::new_with_legacy_id(
    1132,
    "Rubblebelt Raiders",
    CardArt::new("2dec7d6a-2362-4c62-bd81-35bba6086f7d", "Chippy"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{1}{R/G}{R/G}{R/G}"), &["Human", "Warrior"], 3, 3)
        .with_ability(AbilityDef::triggered(
            "Whenever this creature attacks, put a +1/+1 counter on it for each attacking creature you control.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::CountMatchingObjects(&RUBBLEBELT_ATTACKERS),
            },
        )),
);

// GTC 225 — Shattering Blow
pub(in crate::card::sets) static SHATTERING_BLOW: CardRecord = CardRecord::new_with_legacy_id(
    1133,
    "Shattering Blow",
    CardArt::new("a77058d9-d2b5-424a-bfe2-070b754051cb", "Steve Prescott"),
    CardSet::Gatecrash,
    CardRules::new_instant(mana_cost!("{1}{R/W}")).with_ability(AbilityDef::spell_with_targets(
        "Exile target artifact.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Artifact),
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
    )),
);

/// Narrower than a blanket shield: it stops what its blockers deal and
/// nothing else, so anything that is not in the block with it still lands.
static ARMORED_TRANSPORT_SHIELD: DamageEventMatcherDef = DamageEventMatcherDef {
    kind: DamageKindDef::Combat,
    source: DamageSourceMatcherDef::Matching(ObjectPredicateDef::BlockingSource),
    recipient: DamageRecipientMatcherDef::AffectedObject,
};

// GTC 226 — Armored Transport
pub(in crate::card::sets) static ARMORED_TRANSPORT: CardRecord = CardRecord::new_with_legacy_id(
    1746,
    "Armored Transport",
    CardArt::new("0f42fe85-d455-4e24-bd0e-8a3ec3d112aa", "Cliff Childs"),
    CardSet::Gatecrash,
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Construct"], 2, 1).with_ability(
        AbilityDef::static_ability(
            "Prevent all combat damage that would be dealt to this creature by creatures \
             blocking it.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::PreventDamage(
                    ARMORED_TRANSPORT_SHIELD,
                )),
            },
        ),
    ),
);

/// The Keyrune animation, identical across the cycle: it keeps its artifact
/// type, gains a creature type and colours, and takes a printed body. Only
/// the granted keyword differs enough to stay at the call site.
const fn keyrune_animation(
    power: i32,
    toughness: i32,
    creature_types: &'static [&'static str],
    colors: ColorSet,
) -> [AppliedEffectDef; 4] {
    [
        AppliedEffectDef::add_card_types(
            CardTypeSet::single(CardType::Creature).with(CardType::Artifact),
        ),
        AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(creature_types)),
        AppliedEffectDef::set_colors(colors),
        AppliedEffectDef::set_base_power_toughness(
            ValueDef::Constant(power),
            ValueDef::Constant(toughness),
        ),
    ]
}

static BOROS_KEYRUNE_ANIMATION: [AppliedEffectDef; 4] = keyrune_animation(
    1,
    1,
    &["Soldier"],
    ColorSet::from_colors(&[ManaColor::Red, ManaColor::White]),
);

// GTC 227 — Boros Keyrune
pub(in crate::card::sets) static BOROS_KEYRUNE: CardRecord = CardRecord::new_with_legacy_id(
    1988,
    "Boros Keyrune",
    CardArt::new("c4b65847-fee2-4e00-b598-7363059ec3ff", "Daniel Ljunggren"),
    CardSet::Gatecrash,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {R} or {W}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Red,
                ManaColor::White,
            ])),
        ),
        AbilityDef::activated(
            "{R}{W}: This artifact becomes a 1/1 red and white Soldier artifact creature with double strike until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{R}{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::Composite(&BOROS_KEYRUNE_ANIMATION),
                    AppliedEffectDef::add_ability(&abilities::double_strike()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

/// The animation and the evasion are one effect for one duration, so both
/// lapse together at end of turn.
static DIMIR_KEYRUNE_ANIMATION: [AppliedEffectDef; 5] = [
    AppliedEffectDef::add_card_types(
        CardTypeSet::single(CardType::Creature).with(CardType::Artifact),
    ),
    AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Horror"])),
    AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Blue, ManaColor::Black])),
    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
    AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
];

// GTC 228 — Dimir Keyrune
pub(in crate::card::sets) static DIMIR_KEYRUNE: CardRecord = CardRecord::new_with_legacy_id(
    1959,
    "Dimir Keyrune",
    CardArt::new("4d91bb34-5d8d-48c9-ad19-d28884e083bc", "Daniel Ljunggren"),
    CardSet::Gatecrash,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {U} or {B}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Blue,
                ManaColor::Black,
            ])),
        ),
        AbilityDef::activated(
            "{U}{B}: This artifact becomes a 2/2 blue and black Horror artifact creature until \
             end of turn and can't be blocked this turn.",
            &[AbilityCostDef::Mana(mana_cost!("{U}{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&DIMIR_KEYRUNE_ANIMATION),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// GTC 229 — Glaring Spotlight
// Audit: metadata-only — Needs a rule override that lets your effects target opposing hexproof creatures as though they lacked hexproof.
pub(in crate::card::sets) static GLARING_SPOTLIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a6070239-54a7-49d4-bd3c-d5c4cda971db"),
    "Glaring Spotlight",
    crate::card::CardArt::new("a6070239-54a7-49d4-bd3c-d5c4cda971db", "Adam Paquette"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

static GRUUL_KEYRUNE_ANIMATION: [AppliedEffectDef; 4] = keyrune_animation(
    3,
    2,
    &["Beast"],
    ColorSet::from_colors(&[ManaColor::Red, ManaColor::Green]),
);

// GTC 230 — Gruul Keyrune
pub(in crate::card::sets) static GRUUL_KEYRUNE: CardRecord = CardRecord::new_with_legacy_id(
    1989,
    "Gruul Keyrune",
    CardArt::new("7cf96f1c-066e-4fde-acb8-4674842fb6c2", "Daniel Ljunggren"),
    CardSet::Gatecrash,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {R} or {G}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Red,
                ManaColor::Green,
            ])),
        ),
        AbilityDef::activated(
            "{R}{G}: This artifact becomes a 3/2 red and green Beast artifact creature with trample until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{R}{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::Composite(&GRUUL_KEYRUNE_ANIMATION),
                    AppliedEffectDef::add_ability(&abilities::trample()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// GTC 231 — Illusionist's Bracers
// Audit: metadata-only — Needs the equip procedure plus copying a nonmana activated ability of the equipped creature with optional new targets.
pub(in crate::card::sets) static ILLUSIONIST_S_BRACERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cc06cf32-5d66-4772-845d-ff7649396092"),
    "Illusionist's Bracers",
    crate::card::CardArt::new("cc06cf32-5d66-4772-845d-ff7649396092", "Svetlin Velinov"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 232 — Millennial Gargoyle
pub(in crate::card::sets) static MILLENNIAL_GARGOYLE: CardRecord = CardRecord::new_with_legacy_id(
    1134,
    "Millennial Gargoyle",
    CardArt::new("98d1bc6e-84aa-4973-924a-6688b742bafa", "Seb McKinnon"),
    CardSet::Gatecrash,
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Gargoyle"], 2, 2)
        .with_ability(abilities::flying()),
);

static ORZHOV_KEYRUNE_ANIMATION: [AppliedEffectDef; 4] = keyrune_animation(
    1,
    4,
    &["Thrull"],
    ColorSet::from_colors(&[ManaColor::White, ManaColor::Black]),
);

// GTC 233 — Orzhov Keyrune
pub(in crate::card::sets) static ORZHOV_KEYRUNE: CardRecord = CardRecord::new_with_legacy_id(
    1990,
    "Orzhov Keyrune",
    CardArt::new("fd5c0f38-916a-4f6c-b678-7447cb0709e0", "Daniel Ljunggren"),
    CardSet::Gatecrash,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {W} or {B}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Black,
            ])),
        ),
        AbilityDef::activated(
            "{W}{B}: This artifact becomes a 1/4 white and black Thrull artifact creature with lifelink until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{W}{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::Composite(&ORZHOV_KEYRUNE_ANIMATION),
                    AppliedEffectDef::add_ability(&abilities::lifelink()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// GTC 234 — Prophetic Prism
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PROPHETIC_PRISM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cfb90d44-8cb1-4b83-b2f2-92c19d6304fb"),
    "Prophetic Prism",
    crate::card::CardArt::new("b15b29a2-9e6f-45b7-8af5-f09779aae58e", "Daniel Ljunggren"),
    crate::card::CardSet::Gatecrash,
    crate::card::CardRules::unsupported(),
);

// GTC 235 — Razortip Whip
pub(in crate::card::sets) static RAZORTIP_WHIP: CardRecord = CardRecord::new_with_legacy_id(
    1135,
    "Razortip Whip",
    CardArt::new("24619d3f-1051-4d8e-9c8d-70a12621b282", "James Paick"),
    CardSet::Gatecrash,
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::activated_with_targets(
        "{1}, {T}: This artifact deals 1 damage to target opponent or planeswalker.",
        &[
            AbilityCostDef::Mana(mana_cost!("{1}")),
            AbilityCostDef::TapSource,
        ],
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Opponent),
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(1),
        },
    )),
);

// GTC 236 — Riot Gear
pub(in crate::card::sets) static RIOT_GEAR: CardRecord = CardRecord::new_with_legacy_id(
    1922,
    "Riot Gear",
    CardArt::new("d3be1289-76f9-40b3-9387-b76a8b8d8797", "Jack Wang"),
    CardSet::Gatecrash,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +1/+2.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(2),
                    ),
                },
            ),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{2}"))],
                "Equip {2} ({2}: Attach to target creature you control. Equip only as a \
                 sorcery.)",
            ),
        ]),
);

static SIMIC_KEYRUNE_ANIMATION: [AppliedEffectDef; 4] = keyrune_animation(
    2,
    3,
    &["Crab"],
    ColorSet::from_colors(&[ManaColor::Green, ManaColor::Blue]),
);

// GTC 237 — Simic Keyrune
pub(in crate::card::sets) static SIMIC_KEYRUNE: CardRecord = CardRecord::new_with_legacy_id(
    1991,
    "Simic Keyrune",
    CardArt::new("d039aae6-38f5-42b5-a530-e0dd03abc7d5", "Daniel Ljunggren"),
    CardSet::Gatecrash,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {G} or {U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Green,
                ManaColor::Blue,
            ])),
        ),
        AbilityDef::activated(
            "{G}{U}: This artifact becomes a 2/3 green and blue Crab artifact creature with hexproof until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{G}{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::Composite(&SIMIC_KEYRUNE_ANIMATION),
                    AppliedEffectDef::add_ability(&abilities::hexproof()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

static SKYBLINDER_STAFF_BONUS: [AppliedEffectDef; 2] = [
    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(0)),
    AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
        ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
    )),
];

// GTC 238 — Skyblinder Staff
pub(in crate::card::sets) static SKYBLINDER_STAFF: CardRecord = CardRecord::new_with_legacy_id(
    1592,
    "Skyblinder Staff",
    CardArt::new("d1602ee8-019d-4dde-8d31-042207017615", "Mark Winters"),
    CardSet::Gatecrash,
    CardRules::new_artifact(mana_cost!("{1}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +1/+0 and can't be blocked by creatures with flying.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&SKYBLINDER_STAFF_BONUS),
                },
            ),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{3}"))],
                "Equip {3} ({3}: Attach to target creature you control. Equip only as a \
                 sorcery.)",
            ),
        ]),
);

// GTC 239 — Boros Guildgate
pub(in crate::card::sets) static BOROS_GUILDGATE: CardRecord = CardRecord::new_with_legacy_id(
    1136,
    "Boros Guildgate",
    CardArt::new("a0b447a8-524b-4bda-b975-7e194fd741fb", "Noah Bradley"),
    CardSet::Gatecrash,
    CardRules::new_land(&["Gate"]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::activated_mana(
            "{T}: Add {R} or {W}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Red,
                ManaColor::White,
            ])),
        ),
    ]),
);

// GTC 240 — Breeding Pool
pub(in crate::card::sets) static BREEDING_POOL: CardRecord = CardRecord::new_with_legacy_id(
    1137,
    "Breeding Pool",
    CardArt::new("ece3bcdd-cb33-4923-b919-ba57a327d3cd", "Mike Bierek"),
    CardSet::Gatecrash,
    CardRules::new_land(&["Forest", "Island"]).with_ability(abilities::shock_land_enters()),
);

// GTC 241 — Dimir Guildgate
pub(in crate::card::sets) static DIMIR_GUILDGATE: CardRecord = CardRecord::new_with_legacy_id(
    1138,
    "Dimir Guildgate",
    CardArt::new("4951bf75-1a88-4c85-b4e9-063d84f1dabf", "Cliff Childs"),
    CardSet::Gatecrash,
    CardRules::new_land(&["Gate"]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::activated_mana(
            "{T}: Add {U} or {B}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Blue,
                ManaColor::Black,
            ])),
        ),
    ]),
);

// GTC 242 — Godless Shrine
pub(in crate::card::sets) static GODLESS_SHRINE: CardRecord = CardRecord::new_with_legacy_id(
    171,
    "Godless Shrine",
    CardArt::new("6fd672bb-18cf-44e3-8dda-5310b1e0fffe", "Cliff Childs"),
    CardSet::Gatecrash,
    CardRules::new_land(&["Plains", "Swamp"]).with_ability(abilities::shock_land_enters()),
);

// GTC 243 — Gruul Guildgate
pub(in crate::card::sets) static GRUUL_GUILDGATE: CardRecord = CardRecord::new_with_legacy_id(
    1139,
    "Gruul Guildgate",
    CardArt::new("99c54269-8798-4023-836f-640103e08ce0", "Randy Gallegos"),
    CardSet::Gatecrash,
    CardRules::new_land(&["Gate"]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::activated_mana(
            "{T}: Add {R} or {G}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Red,
                ManaColor::Green,
            ])),
        ),
    ]),
);

// GTC 244 — Orzhov Guildgate
pub(in crate::card::sets) static ORZHOV_GUILDGATE: CardRecord = CardRecord::new_with_legacy_id(
    1140,
    "Orzhov Guildgate",
    CardArt::new("000d609c-deb7-4bd7-9c1d-e20fb3ed4f5f", "John Avon"),
    CardSet::Gatecrash,
    CardRules::new_land(&["Gate"]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::activated_mana(
            "{T}: Add {W} or {B}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Black,
            ])),
        ),
    ]),
);

// GTC 245 — Sacred Foundry
pub(in crate::card::sets) static SACRED_FOUNDRY: CardRecord = CardRecord::new_with_legacy_id(
    207,
    "Sacred Foundry",
    CardArt::new("0a26d900-c652-4f9c-8681-a35c5f8b1937", "Sam Burley"),
    CardSet::Gatecrash,
    CardRules::new_land(&["Mountain", "Plains"]).with_ability(abilities::shock_land_enters()),
);

// GTC 246 — Simic Guildgate
pub(in crate::card::sets) static SIMIC_GUILDGATE: CardRecord = CardRecord::new_with_legacy_id(
    1141,
    "Simic Guildgate",
    CardArt::new("1ce3f6f2-c52c-4fb8-afa0-b1ea723bb4c6", "Svetlin Velinov"),
    CardSet::Gatecrash,
    CardRules::new_land(&["Gate"]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::activated_mana(
            "{T}: Add {G} or {U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Green,
                ManaColor::Blue,
            ])),
        ),
    ]),
);

// GTC 247 — Stomping Ground
pub(in crate::card::sets) static STOMPING_GROUND: CardRecord = CardRecord::new_with_legacy_id(
    218,
    "Stomping Ground",
    CardArt::new("f29f3415-971c-4a5d-aae9-3893f4bdab1e", "David Palumbo"),
    CardSet::Gatecrash,
    CardRules::new_land(&["Mountain", "Forest"]).with_ability(abilities::shock_land_enters()),
);

// GTC 248 — Thespian's Stage
pub(in crate::card::sets) static THESPIANS_STAGE: CardRecord = CardRecord::new_with_legacy_id(
    251,
    "Thespian's Stage",
    CardArt::new("b6f27909-e5cd-44c0-91c4-21624f692fd9", "John Avon"),
    CardSet::Gatecrash,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{2}, {T}: This land becomes a copy of target land, except it has this ability.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::BecomeCopyOf {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                copier: None,
                retain_source_ability: true,
                added_types: CardTypeSet::EMPTY,
                duration: None,
            },
        ),
    ]),
);

// GTC 249 — Watery Grave
pub(in crate::card::sets) static WATERY_GRAVE: CardRecord = CardRecord::new_with_legacy_id(
    1142,
    "Watery Grave",
    CardArt::new("47fde349-010e-4a2e-838e-e924dbeec355", "Raymond Swanland"),
    CardSet::Gatecrash,
    CardRules::new_land(&["Island", "Swamp"]).with_ability(abilities::shock_land_enters()),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AERIAL_MANEUVER,
    &ANGELIC_EDICT,
    &ANGELIC_SKIRMISHER,
    &ASSAULT_GRIFFIN,
    &BASILICA_GUARDS,
    &BLIND_OBEDIENCE,
    &BOROS_ELITE,
    &COURT_STREET_DENIZEN,
    &DARING_SKYJEK,
    &DEBTORS_PULPIT,
    &DUTIFUL_THRULL,
    &FRONTLINE_MEDIC,
    &GIDEON_CHAMPION_OF_JUSTICE,
    &GUARDIAN_OF_THE_GATELESS,
    &GUILDSCORN_WARD,
    &HOLD_THE_GATES,
    &HOLY_MANTLE,
    &KNIGHT_OF_OBLIGATION,
    &KNIGHT_WATCH,
    &LUMINATE_PRIMORDIAL,
    &MURDER_INVESTIGATION,
    &NAV_SQUAD_COMMANDOS,
    &RIGHTEOUS_CHARGE,
    &SHIELDED_PASSAGE,
    &SYNDIC_OF_TITHES,
    &URBIS_PROTECTOR,
    &ZARICHI_TIGER,
    &AETHERIZE,
    &AGORAPHOBIA,
    &CLINGING_ANEMONES,
    &CLOUDFIN_RAPTOR,
    &DILUVIAN_PRIMORDIAL,
    &ENTER_THE_INFINITE,
    &FRILLED_OCULUS,
    &GRIDLOCK,
    &HANDS_OF_BINDING,
    &INCURSION_SPECIALIST,
    &KEYMASTER_ROGUE,
    &LAST_THOUGHTS,
    &LEYLINE_PHANTOM,
    &METROPOLIS_SPRITE,
    &MINDEYE_DRAKE,
    &RAPID_HYBRIDIZATION,
    &REALMWRIGHT,
    &SAGES_ROW_DENIZEN,
    &SAPPHIRE_DRAKE,
    &SCATTER_ARC,
    &SIMIC_FLUXMAGE,
    &SIMIC_MANIPULATOR,
    &SKYGAMES,
    &SPELL_RUPTURE,
    &STOLEN_IDENTITY,
    &TOTALLY_LOST,
    &VOIDWALK,
    &WAY_OF_THE_THIEF,
    &BALUSTRADE_SPY,
    &BASILICA_SCREECHER,
    &CONTAMINATED_GROUND,
    &CORPSE_BLOCKADE,
    &CRYPT_GHAST,
    &DEATH_S_APPROACH,
    &DEVOUR_FLESH,
    &DYING_WISH,
    &GATEWAY_SHADE,
    &GRISLY_SPECTACLE,
    &GUTTER_SKULK,
    &HORROR_OF_THE_DIM,
    &ILLNESS_IN_THE_RANKS,
    &KILLING_GLARE,
    &LORD_OF_THE_VOID,
    &MENTAL_VAPORS,
    &MIDNIGHT_RECOVERY,
    &OGRE_SLUMLORD,
    &SEPULCHRAL_PRIMORDIAL,
    &SHADOW_ALLEY_DENIZEN,
    &SHADOW_SLICE,
    &SLATE_STREET_RUFFIAN,
    &SMOG_ELEMENTAL,
    &SYNDICATE_ENFORCER,
    &THRULL_PARASITE,
    &UNDERCITY_INFORMER,
    &UNDERCITY_PLAGUE,
    &WIGHT_OF_PRECINCT_SIX,
    &ACT_OF_TREASON,
    &BOMBER_CORPS,
    &CINDER_ELEMENTAL,
    &CRACKLING_PERIMETER,
    &FIREFIST_STRIKER,
    &FIVE_ALARM_FIRE,
    &FOUNDRY_STREET_DENIZEN,
    &FURIOUS_RESISTANCE,
    &HELLKITE_TYRANT,
    &HELLRAISER_GOBLIN,
    &HOMING_LIGHTNING,
    &LEGION_LOYALIST,
    &MADCAP_SKILLS,
    &MARK_FOR_DEATH,
    &MASSIVE_RAID,
    &MOLTEN_PRIMORDIAL,
    &MUGGING,
    &RIPSCALE_PREDATOR,
    &SCORCHWALKER,
    &SKINBRAND_GOBLIN,
    &SKULLCRACK,
    &STRUCTURAL_COLLAPSE,
    &TIN_STREET_MARKET,
    &TOWERING_THUNDERFIST,
    &VIASHINO_SHANKTAIL,
    &WARMIND_INFANTRY,
    &WRECKING_OGRE,
    &ADAPTIVE_SNAPJAW,
    &ALPHA_AUTHORITY,
    &BURST_OF_STRENGTH,
    &CROCANURA,
    &CROWNED_CERATOK,
    &DISCIPLE_OF_THE_OLD_WAYS,
    &EXPERIMENT_ONE,
    &FORCED_ADAPTATION,
    &GIANT_ADEPHAGE,
    &GREENSIDE_WATCHER,
    &GYRE_SAGE,
    &HINDERVINES,
    &IVY_LANE_DENIZEN,
    &MIMING_SLIME,
    &OOZE_FLUX,
    &PREDATORS_RAPPORT,
    &RUST_SCARAB,
    &SCAB_CLAN_CHARGER,
    &SERENE_REMEMBRANCE,
    &SKARRG_GOLIATH,
    &SLAUGHTERHORN,
    &SPIRE_TRACER,
    &SYLVAN_PRIMORDIAL,
    &TOWER_DEFENSE,
    &VERDANT_HAVEN,
    &WASTELAND_VIPER,
    &WILDWOOD_REBIRTH,
    &ALMS_BEAST,
    &ASSEMBLE_THE_LEGION,
    &AURELIA_THE_WARLEADER,
    &AURELIAS_FURY,
    &BANE_ALLEY_BROKER,
    &BIOVISIONARY,
    &BORBORYGMOS_ENRAGED,
    &BOROS_CHARM,
    &CALL_OF_THE_NIGHTWING,
    &CARTEL_ARISTOCRAT,
    &CLAN_DEFIANCE,
    &CONSUMING_ABERRATION,
    &DEATHPACT_ANGEL,
    &DIMIR_CHARM,
    &DINROVA_HORROR,
    &DOMRI_RADE,
    &DRAKEWING_KRASIS,
    &DUSKMANTLE_GUILDMAGE,
    &DUSKMANTLE_SEER,
    &ELUSIVE_KRASIS,
    &EXECUTIONERS_SWING,
    &FATHOM_MAGE,
    &FIREMANE_AVENGER,
    &FORTRESS_CYCLOPS,
    &FOUNDRY_CHAMPION,
    &FRENZIED_TILLING,
    &GHOR_CLAN_RAMPAGER,
    &GROUND_ASSAULT,
    &GRUUL_CHARM,
    &GRUUL_RAGEBEAST,
    &HIGH_PRIEST_OF_PENANCE,
    &HYDROFORM,
    &KINGPINS_PET,
    &LAZAV_DIMIR_MASTERMIND,
    &MARTIAL_GLORY,
    &MASTER_BIOMANCER,
    &MERCILESS_EVICTION,
    &MIND_GRIND,
    &MORTUS_STRIDER,
    &MYSTIC_GENESIS,
    &NIMBUS_SWIMMER,
    &OBZEDAT_GHOST_COUNCIL,
    &ONE_THOUSAND_LASHES,
    &ORDRUUN_VETERAN,
    &ORZHOV_CHARM,
    &PARANOID_DELUSIONS,
    &PRIMAL_VISITATION,
    &PRIME_SPEAKER_ZEGANA,
    &PSYCHIC_STRIKE,
    &PURGE_THE_PROFANE,
    &RUBBLEHULK,
    &RUINATION_WURM,
    &SHAMBLESHARK,
    &SIGNAL_THE_CLANS,
    &SIMIC_CHARM,
    &SKARRG_GUILDMAGE,
    &SKYKNIGHT_LEGIONNAIRE,
    &SOUL_RANSOM,
    &SPARK_TROOPER,
    &SUNHOME_GUILDMAGE,
    &TREASURY_THRULL,
    &TRUEFIRE_PALADIN,
    &UNEXPECTED_RESULTS,
    &URBAN_EVOLUTION,
    &VIZKOPA_CONFESSOR,
    &VIZKOPA_GUILDMAGE,
    &WHISPERING_MADNESS,
    &WOJEK_HALBERDIERS,
    &ZAMECK_GUILDMAGE,
    &ZHUR_TAA_SWINE,
    &ARROWS_OF_JUSTICE,
    &BECKON_APPARITION,
    &BIOMASS_MUTATION,
    &BIOSHIFT,
    &BOROS_RECKONER,
    &BURNING_TREE_EMISSARY,
    &COERCED_CONFESSION,
    &DEATHCULT_ROGUE,
    &GIFT_OF_ORZHOVA,
    &IMMORTAL_SERVITUDE,
    &MERFOLK_OF_THE_DEPTHS,
    &NIGHTVEIL_SPECTER,
    &PIT_FIGHT,
    &RUBBLEBELT_RAIDERS,
    &SHATTERING_BLOW,
    &ARMORED_TRANSPORT,
    &BOROS_KEYRUNE,
    &DIMIR_KEYRUNE,
    &GLARING_SPOTLIGHT,
    &GRUUL_KEYRUNE,
    &ILLUSIONIST_S_BRACERS,
    &MILLENNIAL_GARGOYLE,
    &ORZHOV_KEYRUNE,
    &PROPHETIC_PRISM,
    &RAZORTIP_WHIP,
    &RIOT_GEAR,
    &SIMIC_KEYRUNE,
    &SKYBLINDER_STAFF,
    &BOROS_GUILDGATE,
    &BREEDING_POOL,
    &DIMIR_GUILDGATE,
    &GODLESS_SHRINE,
    &GRUUL_GUILDGATE,
    &ORZHOV_GUILDGATE,
    &SACRED_FOUNDRY,
    &SIMIC_GUILDGATE,
    &STOMPING_GROUND,
    &THESPIANS_STAGE,
    &WATERY_GRAVE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&catalog_sth::SMITE), // GTC 25
    PrintingRecord::reprint(&catalog_ody::EMBER_BEAST), // GTC 89
    PrintingRecord::reprint(&crate::card::sets::y2002::onslaught::NATURALIZE), // GTC 127
];

static BATTALION_FLYING: AbilityDef = abilities::flying();
static BATTALION_DOUBLE_STRIKE: AbilityDef = abilities::double_strike();
static BATTALION_FIRST_STRIKE: AbilityDef = abilities::first_strike();
