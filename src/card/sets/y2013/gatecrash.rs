//! Gatecrash card records used by the built-in ISD–RTR Standard decks.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    AddManaEffectDef, AppliedEffectDef, BasicLandType, BattlefieldEntryModificationDef, CardArt,
    CardRules, CardSet, CardSupertype, CardType, ComparisonDef, CostDef, CounterKind,
    DiscardSelectionDef, DividedTotal, EffectDef, EffectDurationDef, EffectRecipientDef,
    HybridPair, KeywordAbility, ManaColor, ManaCost, ObjectPredicateDef, ObjectQueryDef,
    PaymentDef, PlayerRelation, ReplacementEffectDef, ReplacementEventDef, TriggerConditionDef,
    TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement, abilities, cards,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

/// One opponent means one life, so "that much" is the same constant on both
/// halves.
static EXTORT_DRAIN: EffectDef = EffectDef::Sequence(&[
    EffectDef::LoseLife {
        recipient: EffectRecipientDef::Opponent,
        amount: ValueDef::Constant(1),
    },
    EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
]);
static EXTORT_COSTS: [CostDef; 1] = [CostDef::Mana(ManaCost::hybrid_pair(
    HybridPair::WhiteBlack,
    1,
))];

const fn extort() -> AbilityDef {
    AbilityDef::triggered(
        "Extort (Whenever you cast a spell, you may pay {W/B}. If you do, each opponent loses 1 life and you gain that much life.)",
        TriggerEventDef::SpellCast(ObjectPredicateDef::ControlledBy(PlayerRelation::You)),
        EffectDef::OptionalPayment {
            payment: PaymentDef::new(PlayerRelation::You, &EXTORT_COSTS),
            if_paid: &EXTORT_DRAIN,
        },
    )
}

// GTC 1 — Aerial Maneuver
pub(in crate::card::sets) static AERIAL_MANEUVER: CardRecord = CardRecord::new(
    cards::AERIAL_MANEUVER,
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
                AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(1),
                },
                AppliedEffectDef::GrantAbility(&abilities::flying()),
                AppliedEffectDef::GrantAbility(&abilities::first_strike()),
            ]),
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// GTC 2 — Angelic Edict
pub(in crate::card::sets) static ANGELIC_EDICT: CardRecord = CardRecord::new(
    cards::ANGELIC_EDICT,
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
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Exile,
            placement: ZonePlacement::Top,
            controller: None,
        },
    )),
);

// GTC 3 — Angelic Skirmisher
// Audit: blocked — Needs a resolving keyword choice on a beginning-of-combat trigger and a temporary mass grant of the chosen ability.

// GTC 4 — Assault Griffin
pub(in crate::card::sets) static ASSAULT_GRIFFIN: CardRecord = CardRecord::new(
    cards::ASSAULT_GRIFFIN,
    "Assault Griffin",
    CardArt::new("704286a5-e3a8-4517-85e5-6447c5c2530f", "Eric Velhagen"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Griffin"], 3, 2)
        .with_ability(abilities::flying()),
);

// GTC 5 — Basilica Guards
pub(in crate::card::sets) static BASILICA_GUARDS: CardRecord = CardRecord::new(
    cards::BASILICA_GUARDS,
    "Basilica Guards",
    CardArt::new("2be39fed-4b39-4027-9c80-f2186f7dd941", "Dan Murayama Scott"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 1, 4)
        .with_abilities(&[abilities::defender(), extort()]),
);

// GTC 6 — Blind Obedience
pub(in crate::card::sets) static BLIND_OBEDIENCE: CardRecord = CardRecord::new(
    cards::BLIND_OBEDIENCE,
    "Blind Obedience",
    CardArt::new("07c3e78d-d917-4552-842f-feff99c059e0", "Seb McKinnon"),
    CardSet::Gatecrash,
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_abilities(&[
        extort(),
        AbilityDef::replacement_for(
            "Artifacts and creatures your opponents control enter tapped.",
            ReplacementEventDef::ObjectEntersBattlefield {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]),
                controller: PlayerRelation::Opponent,
            },
            EffectDef::Replacement(ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::Tapped,
            )),
        ),
    ]),
);

// GTC 7 — Boros Elite
// Audit: blocked — Battalion is a trigger-time restriction, but declarative trigger conditions are rechecked on resolution as intervening-if conditions.

// GTC 8 — Court Street Denizen
pub(in crate::card::sets) static COURT_STREET_DENIZEN: CardRecord = CardRecord::new(
    cards::COURT_STREET_DENIZEN,
    "Court Street Denizen",
    CardArt::new("ca6a5cb3-b6e5-4879-83b5-4ad590a5467a", "Volkan Baǵa"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 2, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever another white creature you control enters, tap target creature an opponent controls.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::White),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
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
// Audit: blocked — Battalion is a trigger-time restriction, but declarative trigger conditions are rechecked on resolution as intervening-if conditions.

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
pub(in crate::card::sets) static DEBTORS_PULPIT: CardRecord = CardRecord::new(
    cards::DEBTORS_PULPIT,
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
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::GrantAbility(&DEBTORS_PULPIT_TAP),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// GTC 11 — Dutiful Thrull
// Audit: blocked — Regeneration shields and the destroy replacement they create are not declarative effects.

// GTC 12 — Frontline Medic
// Audit: blocked — Battalion needs a trigger-time-only condition, and no target predicate recognizes a spell with X in its mana cost.

// GTC 13 — Gideon, Champion of Justice
// Audit: blocked — Needs dynamic loyalty addition, loyalty-sized animation, all-damage prevention, and mass exile of every other permanent.

// GTC 14 — Guardian of the Gateless
// Audit: blocked — Needs blocking any number of creatures and a trigger amount equal to how many creatures the source is blocking.

// GTC 15 — Guildscorn Ward
// Audit: blocked — Protection can currently be expressed only from a specific color, not from multicolored objects.

static HOLD_THE_GATES_GATES: ObjectQueryDef = ObjectQueryDef {
    object: ObjectPredicateDef::Subtype("Gate"),
    zones: &[ZoneKind::Battlefield],
    controller: PlayerRelation::You,
};

// GTC 16 — Hold the Gates
pub(in crate::card::sets) static HOLD_THE_GATES: CardRecord = CardRecord::new(
    cards::HOLD_THE_GATES,
    "Hold the Gates",
    CardArt::new("48fd52d0-0e41-48d5-b96f-4c6409788c18", "Zoltan Boros"),
    CardSet::Gatecrash,
    CardRules::new_enchantment(mana_cost!("{2}{W}")).with_ability(AbilityDef::static_ability(
        "Creatures you control get +0/+1 for each Gate you control and have vigilance.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::You,
            },
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(0),
                    toughness: ValueDef::CountMatchingObjects(&HOLD_THE_GATES_GATES),
                },
                AppliedEffectDef::GrantAbility(&abilities::vigilance()),
            ]),
            duration: EffectDurationDef::WhileSourceRemainsInZone,
        },
    )),
);

// GTC 17 — Holy Mantle
// Audit: blocked — Protection from creatures is not expressible by the color-specific protection keyword primitive.

// GTC 18 — Knight of Obligation
pub(in crate::card::sets) static KNIGHT_OF_OBLIGATION: CardRecord = CardRecord::new(
    cards::KNIGHT_OF_OBLIGATION,
    "Knight of Obligation",
    CardArt::new("0c2a1100-a2e6-4ef5-a8e3-2aca552d6b66", "Ryan Barger"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Knight"], 2, 4)
        .with_abilities(&[abilities::vigilance(), extort()]),
);

// GTC 19 — Knight Watch
pub(in crate::card::sets) static KNIGHT_WATCH: CardRecord = CardRecord::new(
    cards::KNIGHT_WATCH,
    "Knight Watch",
    CardArt::new("cd492072-9a8c-4d55-ac71-3c8efaa3fc87", "Matt Stewart"),
    CardSet::Gatecrash,
    CardRules::new_sorcery(mana_cost!("{4}{W}")).with_ability(AbilityDef::spell(
        "Create two 2/2 white Knight creature tokens with vigilance.",
        EffectDef::CreateToken {
            token: cards::KNIGHT_TOKEN_2_2_WHITE,
            count: ValueDef::Constant(2),
        },
    )),
);

// GTC 20 — Luminate Primordial
pub(in crate::card::sets) static LUMINATE_PRIMORDIAL: CardRecord = CardRecord::new(
    cards::LUMINATE_PRIMORDIAL,
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
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
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
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Exile,
                    placement: ZonePlacement::Top,
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

// GTC 21 — Murder Investigation
// Audit: blocked — Needs an Aura trigger keyed to the attached permanent dying and its last-known power.

// GTC 22 — Nav Squad Commandos
// Audit: blocked — Battalion is a trigger-time restriction, but declarative trigger conditions are rechecked on resolution as intervening-if conditions.

// GTC 23 — Righteous Charge
pub(in crate::card::sets) static RIGHTEOUS_CHARGE: CardRecord = CardRecord::new(
    cards::RIGHTEOUS_CHARGE,
    "Righteous Charge",
    CardArt::new("f52cb325-4f16-4cf3-9999-feafe0fde8c2", "Svetlin Velinov"),
    CardSet::Gatecrash,
    CardRules::new_sorcery(mana_cost!("{1}{W}{W}")).with_ability(AbilityDef::spell(
        "Creatures you control get +2/+2 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::You,
            },
            effect: AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Constant(2),
                toughness: ValueDef::Constant(2),
            },
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// GTC 24 — Shielded Passage
// Audit: blocked — The prevention primitives cover combat damage or damage from a matching permanent, not all damage to one creature this turn.

// GTC 25 — Smite
// Audit: blocked — Target predicates can recognize attacking or blocking creatures, but not a creature that is currently blocked.

// GTC 26 — Syndic of Tithes
pub(in crate::card::sets) static SYNDIC_OF_TITHES: CardRecord = CardRecord::new(
    cards::SYNDIC_OF_TITHES,
    "Syndic of Tithes",
    CardArt::new("2bafaa3b-eeaa-427f-9a73-6a1c98d257ca", "Steve Prescott"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Cleric"], 2, 2)
        .with_ability(extort()),
);

// GTC 27 — Urbis Protector
pub(in crate::card::sets) static URBIS_PROTECTOR: CardRecord = CardRecord::new(
    cards::URBIS_PROTECTOR,
    "Urbis Protector",
    CardArt::new("acf932ac-5ea5-491b-b555-5e9ea971d93d", "Steve Argyle"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{4}{W}{W}"), &["Human", "Cleric"], 1, 1).with_ability(
        AbilityDef::triggered(
            "When this creature enters, create a 4/4 white Angel creature token with flying.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::CreateToken {
                token: cards::ANGEL_TOKEN_4_4_WHITE,
                count: ValueDef::Constant(1),
            },
        ),
    ),
);

// GTC 28 — Zarichi Tiger
pub(in crate::card::sets) static ZARICHI_TIGER: CardRecord = CardRecord::new(
    cards::ZARICHI_TIGER,
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
// Audit: blocked — Needs a simultaneous multi-object move to return all attacking creatures without resolving the zone changes sequentially.

// GTC 30 — Agoraphobia
pub(in crate::card::sets) static AGORAPHOBIA: CardRecord = CardRecord::new(
    cards::AGORAPHOBIA,
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
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(-5),
                        toughness: ValueDef::Constant(0),
                    },
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
            AbilityDef::activated(
                "{2}{U}: Return this Aura to its owner's hand.",
                &[AbilityCostDef::Mana(mana_cost!("{2}{U}"))],
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Source,
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    controller: None,
                },
            ),
        ]),
);

// GTC 31 — Clinging Anemones
// Audit: blocked — Evolve needs a trigger comparison against the entering creature's current power and toughness.

// GTC 32 — Cloudfin Raptor
// Audit: blocked — Evolve needs a trigger comparison against the entering creature's current power and toughness.

// GTC 33 — Diluvian Primordial
// Audit: blocked — Needs casting a targeted graveyard card without paying its mana cost and replacing that spell card's later graveyard move with exile.

// GTC 34 — Enter the Infinite
// Audit: blocked — Needs a dynamic library-sized draw, a non-target hand choice to put on top, and a temporary no-maximum-hand-size rule.

// GTC 35 — Frilled Oculus
// Audit: blocked — Activated abilities have no once-per-turn activation restriction independent of a trigger condition.

// GTC 36 — Gridlock
// Audit: blocked — Target declarations cannot require exactly X targets from the spell's chosen X value.

// GTC 37 — Hands of Binding
// Audit: blocked — Needs the next-untap-step skip effect and the cipher encoding and free-copy-casting procedure.

// GTC 38 — Incursion Specialist
// Audit: blocked — Needs a second-spell-this-turn trigger event and a temporary unblockable clause on the source.

// GTC 39 — Keymaster Rogue
// Audit: blocked — Its entry trigger needs a mandatory non-target choice of a creature you control to return.

// GTC 40 — Last Thoughts
// Audit: blocked — Cipher encoding and casting encoded spell copies without paying their mana costs are unavailable.

// GTC 41 — Leyline Phantom
// Audit: blocked — Needs a trigger for the source dealing combat damage to any recipient and source survival through combat damage.

// GTC 42 — Metropolis Sprite
pub(in crate::card::sets) static METROPOLIS_SPRITE: CardRecord = CardRecord::new(
    cards::METROPOLIS_SPRITE,
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
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(-1),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// GTC 43 — Mindeye Drake
pub(in crate::card::sets) static MINDEYE_DRAKE: CardRecord = CardRecord::new(
    cards::MINDEYE_DRAKE,
    "Mindeye Drake",
    CardArt::new("947f44b0-91be-4115-b499-57893f0f69a9", "Lars Grant-West"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Drake"], 2, 5).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "When this creature dies, target player mills five cards.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: Some(ZoneKind::Battlefield),
                to: Some(ZoneKind::Graveyard),
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(5),
            },
        ),
    ]),
);

// GTC 44 — Rapid Hybridization
// Audit: blocked — Token creation always uses the resolving spell's controller, not the destroyed creature's controller.

// GTC 45 — Realmwright
// Audit: blocked — Needs choosing and storing one basic land type and a continuous land-type grant keyed to that choice.

// GTC 46 — Sage's Row Denizen
pub(in crate::card::sets) static SAGES_ROW_DENIZEN: CardRecord = CardRecord::new(
    cards::SAGES_ROW_DENIZEN,
    "Sage's Row Denizen",
    CardArt::new("063e6df9-2287-485a-ab46-fa4a38783884", "Svetlin Velinov"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Vedalken", "Wizard"], 2, 3).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever another blue creature you control enters, target player mills two cards.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::Blue),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

// GTC 47 — Sapphire Drake
// Audit: blocked — Continuous recipient predicates cannot test whether a creature currently has a +1/+1 counter.

// GTC 48 — Scatter Arc
pub(in crate::card::sets) static SCATTER_ARC: CardRecord = CardRecord::new(
    cards::SCATTER_ARC,
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
// Audit: blocked — Evolve and moving a +1/+1 counter between two permanents are not declarative procedures.

// GTC 50 — Simic Manipulator
// Audit: blocked — Needs evolve, a variable counter-removal cost, a target-power limit based on counters removed, and indefinite control change.

// GTC 51 — Skygames
// Audit: blocked — Granted activated abilities cannot carry an activate-only-as-a-sorcery timing restriction.

// GTC 52 — Spell Rupture
// Audit: blocked — No value computes the greatest power among creatures you control for the counter-unless payment.

// GTC 53 — Stolen Identity
// Audit: blocked — Needs token copies of a target and cipher's encoding and free-copy-casting procedure.

// GTC 54 — Totally Lost
pub(in crate::card::sets) static TOTALLY_LOST: CardRecord = CardRecord::new(
    cards::TOTALLY_LOST,
    "Totally Lost",
    CardArt::new("ec8e4142-7c46-4d2f-aaa6-6410f323d9f0", "David Palumbo"),
    CardSet::Gatecrash,
    CardRules::new_instant(mana_cost!("{4}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Put target nonland permanent on top of its owner's library.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
        )],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Library,
            placement: ZonePlacement::Top,
            controller: None,
        },
    )),
);

// GTC 55 — Voidwalk
// Audit: blocked — Cipher's encoding and free-copy-casting procedure are unavailable, even though the initial delayed blink is expressible.

// GTC 56 — Way of the Thief
// Audit: blocked — Needs a conditional attachment-scoped unblockable effect while the Aura's controller controls a Gate.

// GTC 57 — Balustrade Spy
// Audit: blocked — Needs revealing cards until a land is found and moving the whole revealed group to a graveyard.

// GTC 58 — Basilica Screecher
pub(in crate::card::sets) static BASILICA_SCREECHER: CardRecord = CardRecord::new(
    cards::BASILICA_SCREECHER,
    "Basilica Screecher",
    CardArt::new("d233c6bc-c4dd-482d-b0f4-87359acab7cb", "Christine Choi"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Bat"], 1, 2)
        .with_abilities(&[abilities::flying(), extort()]),
);

static CONTAMINATED_GROUND_TRIGGER: AbilityDef = AbilityDef::triggered(
    "Whenever enchanted land becomes tapped, its controller loses 2 life.",
    TriggerEventDef::BecomesTapped(ObjectPredicateDef::Source),
    EffectDef::LoseLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(2),
    },
);

// GTC 59 — Contaminated Ground
pub(in crate::card::sets) static CONTAMINATED_GROUND: CardRecord = CardRecord::new(
    cards::CONTAMINATED_GROUND,
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
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::SetLandTypes(&[BasicLandType::Swamp]),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
            AbilityDef::static_ability(
                "Whenever enchanted land becomes tapped, its controller loses 2 life.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::GrantAbility(&CONTAMINATED_GROUND_TRIGGER),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// GTC 60 — Corpse Blockade
pub(in crate::card::sets) static CORPSE_BLOCKADE: CardRecord = CardRecord::new(
    cards::CORPSE_BLOCKADE,
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
                effect: AppliedEffectDef::GrantAbility(&abilities::deathtouch()),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// GTC 61 — Crypt Ghast
pub(in crate::card::sets) static CRYPT_GHAST: CardRecord = CardRecord::new(
    cards::CRYPT_GHAST,
    "Crypt Ghast",
    CardArt::new("3795a4e7-646f-4bb7-b154-2610eb740e8d", "Chris Rahn"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Spirit"], 2, 2).with_abilities(&[
        extort(),
        AbilityDef::triggered_mana(
            "Whenever you tap a Swamp for mana, add an additional {B}.",
            TriggerEventDef::TappedForMana(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Swamp]),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black)),
        ),
    ]),
);

// GTC 62 — Death's Approach
// Audit: blocked — The static value vocabulary cannot count creature cards in the attached creature's controller's graveyard.

// GTC 63 — Devour Flesh
// Audit: blocked — The sacrifice-choice continuation exposes the sacrificed creature's power, not its last-known toughness.

// GTC 64 — Dying Wish
// Audit: blocked — Needs an Aura trigger keyed to the attached permanent dying and its last-known power.

// GTC 65 — Gateway Shade
// Audit: blocked — Costs can tap only the ability source, not a chosen untapped Gate you control.

// GTC 66 — Grisly Spectacle
pub(in crate::card::sets) static GRISLY_SPECTACLE: CardRecord = CardRecord::new(
    cards::GRISLY_SPECTACLE,
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
                },
            ]),
        ),
    ),
);

// GTC 67 — Gutter Skulk
pub(in crate::card::sets) static GUTTER_SKULK: CardRecord = CardRecord::new(
    cards::GUTTER_SKULK,
    "Gutter Skulk",
    CardArt::new("830c7c77-20c4-429f-88c7-b85ab7a0e38b", "Mark Winters"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Zombie", "Rat"], 2, 2),
);

// GTC 68 — Horror of the Dim
pub(in crate::card::sets) static HORROR_OF_THE_DIM: CardRecord = CardRecord::new(
    cards::HORROR_OF_THE_DIM,
    "Horror of the Dim",
    CardArt::new("f5d36c9d-967e-42dc-890c-0485b12f704f", "Jack Wang"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Horror"], 3, 4).with_ability(
        AbilityDef::activated(
            "{U}: This creature gains hexproof until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::GrantAbility(&abilities::hexproof()),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// GTC 69 — Illness in the Ranks
// Audit: blocked — Continuous recipient predicates cannot distinguish creature tokens from nontoken creatures.

// GTC 70 — Killing Glare
// Audit: blocked — Target predicates cannot compare a creature's power with the spell's chosen X value.

// GTC 71 — Lord of the Void
// Audit: blocked — Needs combat-damage-player subject capture, top-seven exile, and a non-target creature-card choice from the exiled group.

// GTC 72 — Mental Vapors
// Audit: blocked — Cipher encoding and casting encoded spell copies without paying their mana costs are unavailable.

// GTC 73 — Midnight Recovery
// Audit: blocked — Cipher encoding and casting encoded spell copies without paying their mana costs are unavailable.

// GTC 74 — Ogre Slumlord
// Audit: blocked — Needs a nontoken-creature death predicate and a static deathtouch grant to Rats.

// GTC 75 — Sepulchral Primordial
pub(in crate::card::sets) static SEPULCHRAL_PRIMORDIAL: CardRecord = CardRecord::new(
    cards::SEPULCHRAL_PRIMORDIAL,
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
        AbilityDef::triggered_with_targets("When this creature enters, for each opponent, you may put up to one target creature card from that player's graveyard onto the battlefield under your control.", TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            }, &[AbilityTargetDef::up_to(
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
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Battlefield,
                controller: Some(PlayerRelation::You),
                placement: ZonePlacement::Top,
            }),
    ]),
);

// GTC 76 — Shadow Alley Denizen
pub(in crate::card::sets) static SHADOW_ALLEY_DENIZEN: CardRecord = CardRecord::new(
    cards::SHADOW_ALLEY_DENIZEN,
    "Shadow Alley Denizen",
    CardArt::new("985997ae-59bc-49d7-87ca-e63ed9706fdf", "Cynthia Sheppard"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{B}"), &["Vampire", "Rogue"], 1, 1).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever another black creature you control enters, target creature gains intimidate until end of turn.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::Black),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::GrantAbility(&abilities::intimidate()),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// GTC 77 — Shadow Slice
// Audit: blocked — Cipher encoding and casting encoded spell copies without paying their mana costs are unavailable.

// GTC 78 — Slate Street Ruffian
// Audit: blocked — There is no trigger event for a creature becoming blocked or a captured defending player.

// GTC 79 — Smog Elemental
// Audit: blocked — A continuous HasKeyword(Flying) recipient predicate ignores abilities granted by other static effects.

// GTC 80 — Syndicate Enforcer
pub(in crate::card::sets) static SYNDICATE_ENFORCER: CardRecord = CardRecord::new(
    cards::SYNDICATE_ENFORCER,
    "Syndicate Enforcer",
    CardArt::new("cde6ee2e-a114-4935-8345-d3e264f9fc26", "Steven Belledin"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Human", "Rogue"], 3, 2).with_ability(extort()),
);

// GTC 81 — Thrull Parasite
// Audit: blocked — Counter-removal costs and effects require a fixed CounterKind, not choosing any counter on the target.

// GTC 82 — Undercity Informer
// Audit: blocked — Needs revealing cards until a land is found and moving the whole revealed group to a graveyard.

// GTC 83 — Undercity Plague
// Audit: blocked — Needs cipher plus a discard decision that resumes into a permanent-sacrifice choice before later effects resolve.

static WIGHT_CREATURE_CARDS: ObjectQueryDef = ObjectQueryDef {
    object: ObjectPredicateDef::HasType(CardType::Creature),
    zones: &[ZoneKind::Graveyard],
    controller: PlayerRelation::Opponent,
};

// GTC 84 — Wight of Precinct Six
pub(in crate::card::sets) static WIGHT_OF_PRECINCT_SIX: CardRecord = CardRecord::new(
    cards::WIGHT_OF_PRECINCT_SIX,
    "Wight of Precinct Six",
    CardArt::new("b04644ba-5962-4e64-bc53-92941c5b6715", "Ryan Barger"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Zombie"], 1, 1).with_ability(
        AbilityDef::static_ability(
            "This creature gets +1/+1 for each creature card in your opponents' graveyards.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::CountMatchingObjects(&WIGHT_CREATURE_CARDS),
                    toughness: ValueDef::CountMatchingObjects(&WIGHT_CREATURE_CARDS),
                },
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
    ),
);

// GTC 85 — Act of Treason
pub(in crate::card::sets) static ACT_OF_TREASON: CardRecord = CardRecord::new(
    cards::ACT_OF_TREASON,
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
                EffectDef::GainControlThisTurn {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::GrantAbility(&abilities::haste()),
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ),
);

// GTC 86 — Bomber Corps
// Audit: blocked — Battalion is a trigger-time restriction, but declarative trigger conditions are rechecked on resolution as intervening-if conditions.

// GTC 87 — Cinder Elemental
pub(in crate::card::sets) static CINDER_ELEMENTAL: CardRecord = CardRecord::new(
    cards::CINDER_ELEMENTAL,
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
// Audit: blocked — Costs can tap only the ability source, not a chosen untapped Gate you control.

// GTC 89 — Ember Beast
// Audit: blocked — Combat constraints cannot require another creature to attack or block alongside the source.

// GTC 90 — Firefist Striker
// Audit: blocked — Battalion is a trigger-time restriction, and no resolving effect can make a creature unable to block for the turn.

// GTC 91 — Five-Alarm Fire
// Audit: blocked — Needs a trigger for any creature you control dealing combat damage and an executable blaze-counter removal cost.

// GTC 92 — Foundry Street Denizen
pub(in crate::card::sets) static FOUNDRY_STREET_DENIZEN: CardRecord = CardRecord::new(
    cards::FOUNDRY_STREET_DENIZEN,
    "Foundry Street Denizen",
    CardArt::new("0befed63-07ba-4728-9078-57bbccbeeeb1", "Raoul Vitale"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin", "Warrior"], 1, 1).with_ability(
        AbilityDef::triggered(
            "Whenever another red creature you control enters, this creature gets +1/+0 until end of turn.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::Red),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(0),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// GTC 93 — Furious Resistance
// Audit: blocked — Target predicates recognize attacking or blocking together, but cannot require specifically a blocking creature.

// GTC 94 — Hellkite Tyrant
// Audit: blocked — Needs indefinite control of a target player's artifacts and a win-the-game effect under a twenty-artifact upkeep condition.

static HELLRAISER_ATTACKS: AbilityDef =
    abilities::attacks_each_combat_if_able("This creature attacks each combat if able.");

// GTC 95 — Hellraiser Goblin
pub(in crate::card::sets) static HELLRAISER_GOBLIN: CardRecord = CardRecord::new(
    cards::HELLRAISER_GOBLIN,
    "Hellraiser Goblin",
    CardArt::new("156941e7-9169-47aa-b04d-37ca78c54f7c", "Karl Kopinski"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Berserker"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "Creatures you control have haste and attack each combat if able.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::You,
                },
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::GrantAbility(&abilities::haste()),
                    AppliedEffectDef::GrantAbility(&HELLRAISER_ATTACKS),
                ]),
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
    ),
);

// GTC 96 — Homing Lightning
pub(in crate::card::sets) static HOMING_LIGHTNING: CardRecord = CardRecord::new(
    cards::HOMING_LIGHTNING,
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
// Audit: blocked — Battalion is trigger-time-only, menace-style token blocking restrictions are unavailable, and its mass grants cannot be conditioned exactly.

// GTC 98 — Madcap Skills
// Audit: blocked — Menace is not an executable minimum-blocker constraint or grantable keyword.

// GTC 99 — Mark for Death
// Audit: blocked — Needs turn-long must-block and cannot-block constraints scoped to one opponent's creatures.

static MASSIVE_RAID_CREATURES: ObjectQueryDef = ObjectQueryDef {
    object: ObjectPredicateDef::HasType(CardType::Creature),
    zones: &[ZoneKind::Battlefield],
    controller: PlayerRelation::You,
};

// GTC 100 — Massive Raid
pub(in crate::card::sets) static MASSIVE_RAID: CardRecord = CardRecord::new(
    cards::MASSIVE_RAID,
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
pub(in crate::card::sets) static MOLTEN_PRIMORDIAL: CardRecord = CardRecord::new(
    cards::MOLTEN_PRIMORDIAL,
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
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
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
                EffectDef::GainControlThisTurn {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::GrantAbility(&abilities::haste()),
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ]),
);

// GTC 102 — Mugging
// Audit: blocked — No resolving effect can make a creature unable to block for the turn.

// GTC 103 — Ripscale Predator
// Audit: blocked — Menace is not an executable minimum-blocker constraint.

// GTC 104 — Scorchwalker
pub(in crate::card::sets) static SCORCHWALKER: CardRecord = CardRecord::new(
    cards::SCORCHWALKER,
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
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(5),
                    toughness: ValueDef::Constant(1),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// GTC 105 — Skinbrand Goblin
pub(in crate::card::sets) static SKINBRAND_GOBLIN: CardRecord = CardRecord::new(
    cards::SKINBRAND_GOBLIN,
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
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(2),
                    toughness: ValueDef::Constant(1),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// GTC 106 — Skullcrack
// Audit: blocked — Needs turn-long prohibitions on life gain and damage prevention.

// GTC 107 — Structural Collapse
// Audit: blocked — Needs two distinct resolving permanent choices and a continuation that deals damage after both sacrifices.

// GTC 108 — Tin Street Market
// Audit: blocked — Discarding a card is not supported as a declarative activated-ability cost.

// GTC 109 — Towering Thunderfist
pub(in crate::card::sets) static TOWERING_THUNDERFIST: CardRecord = CardRecord::new(
    cards::TOWERING_THUNDERFIST,
    "Towering Thunderfist",
    CardArt::new("d68e9280-cb1a-48e1-a91e-217e101f19c5", "Zoltan Boros"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Giant", "Soldier"], 4, 4).with_ability(
        AbilityDef::activated(
            "{W}: This creature gains vigilance until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::GrantAbility(&abilities::vigilance()),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// GTC 110 — Viashino Shanktail
pub(in crate::card::sets) static VIASHINO_SHANKTAIL: CardRecord = CardRecord::new(
    cards::VIASHINO_SHANKTAIL,
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
                        AppliedEffectDef::ModifyPowerToughness {
                            power: ValueDef::Constant(3),
                            toughness: ValueDef::Constant(1),
                        },
                        AppliedEffectDef::GrantAbility(&abilities::first_strike()),
                    ]),
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// GTC 111 — Warmind Infantry
// Audit: blocked — Battalion is a trigger-time restriction, but declarative trigger conditions are rechecked on resolution as intervening-if conditions.

// GTC 112 — Wrecking Ogre
pub(in crate::card::sets) static WRECKING_OGRE: CardRecord = CardRecord::new(
    cards::WRECKING_OGRE,
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
                        AppliedEffectDef::ModifyPowerToughness {
                            power: ValueDef::Constant(3),
                            toughness: ValueDef::Constant(3),
                        },
                        AppliedEffectDef::GrantAbility(&abilities::double_strike()),
                    ]),
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// GTC 113 — Adaptive Snapjaw
// Audit: blocked — Evolve needs a trigger comparison against the entering creature's current power and toughness.

// GTC 114 — Alpha Authority
// Audit: blocked — Combat constraints cannot limit an attacker to at most one blocker.

// GTC 115 — Burst of Strength
pub(in crate::card::sets) static BURST_OF_STRENGTH: CardRecord = CardRecord::new(
    cards::BURST_OF_STRENGTH,
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
// Audit: blocked — Evolve needs a trigger comparison against the entering creature's current power and toughness.

// GTC 117 — Crowned Ceratok
// Audit: blocked — Continuous recipient predicates cannot test whether a creature currently has a +1/+1 counter.

// GTC 118 — Disciple of the Old Ways
pub(in crate::card::sets) static DISCIPLE_OF_THE_OLD_WAYS: CardRecord = CardRecord::new(
    cards::DISCIPLE_OF_THE_OLD_WAYS,
    "Disciple of the Old Ways",
    CardArt::new("3c62b3ee-db2b-45c3-87d5-5d917ea4baeb", "Anthony Palumbo"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Warrior"], 2, 2).with_ability(
        AbilityDef::activated(
            "{R}: This creature gains first strike until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::GrantAbility(&abilities::first_strike()),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// GTC 119 — Experiment One
// Audit: blocked — Evolve and regeneration shields are not declarative procedures.

// GTC 120 — Forced Adaptation
pub(in crate::card::sets) static FORCED_ADAPTATION: CardRecord = CardRecord::new(
    cards::FORCED_ADAPTATION,
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
// Audit: blocked — Token creation cannot create a token copy of the source permanent.

// GTC 122 — Greenside Watcher
pub(in crate::card::sets) static GREENSIDE_WATCHER: CardRecord = CardRecord::new(
    cards::GREENSIDE_WATCHER,
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
// Audit: blocked — Evolve is unavailable even though mana production from a fixed counter kind is expressible.

// GTC 124 — Hindervines
// Audit: blocked — Prevention effects cannot select combat-damage sources based on having no +1/+1 counters.

// GTC 125 — Ivy Lane Denizen
pub(in crate::card::sets) static IVY_LANE_DENIZEN: CardRecord = CardRecord::new(
    cards::IVY_LANE_DENIZEN,
    "Ivy Lane Denizen",
    CardArt::new("b95be874-93c0-4e05-9e5a-fe8f38bcb445", "Winona Nelson"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Elf", "Warrior"], 2, 3).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever another green creature you control enters, put a +1/+1 counter on target creature.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::Green),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
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
// Audit: blocked — No value computes the greatest power among creatures you control for a dynamically sized token.

// Naturalize first debuted in Onslaught; its GTC printing is registered in ADDITIONAL_PRINTINGS.

// GTC 128 — Ooze Flux
// Audit: blocked — Needs removing an arbitrary number of +1/+1 counters distributed among creatures and a token sized by the amount removed.

// GTC 129 — Predator's Rapport
// Audit: blocked — Values expose target power but not target toughness or addition of the two characteristics.

// GTC 130 — Rust Scarab
// Audit: blocked — Needs a becomes-blocked trigger and a target constrained to the captured defending player.

// GTC 131 — Scab-Clan Charger
pub(in crate::card::sets) static SCAB_CLAN_CHARGER: CardRecord = CardRecord::new(
    cards::SCAB_CLAN_CHARGER,
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
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(2),
                    toughness: ValueDef::Constant(4),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// GTC 132 — Serene Remembrance
// Audit: blocked — Needs linked targets from one graveyard plus moving the resolving spell itself and all chosen cards into different owners' libraries before shuffling.

// GTC 133 — Skarrg Goliath
pub(in crate::card::sets) static SKARRG_GOLIATH: CardRecord = CardRecord::new(
    cards::SKARRG_GOLIATH,
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
                    AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(9),
                        toughness: ValueDef::Constant(9),
                    },
                    AppliedEffectDef::GrantAbility(&abilities::trample()),
                ]),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// GTC 134 — Slaughterhorn
pub(in crate::card::sets) static SLAUGHTERHORN: CardRecord = CardRecord::new(
    cards::SLAUGHTERHORN,
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
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(3),
                    toughness: ValueDef::Constant(2),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// GTC 135 — Spire Tracer
// Audit: blocked — A HasKeyword(Flying or Reach) blocking predicate ignores abilities supplied by continuous static effects.

// GTC 136 — Sylvan Primordial
// Audit: blocked — Needs a successful-destruction continuation that searches for a Forest, puts it onto the battlefield tapped, and repeats per opponent.

// GTC 137 — Tower Defense
pub(in crate::card::sets) static TOWER_DEFENSE: CardRecord = CardRecord::new(
    cards::TOWER_DEFENSE,
    "Tower Defense",
    CardArt::new("857e1eb2-f3f2-4c7f-9965-da9d7e385223", "Seb McKinnon"),
    CardSet::Gatecrash,
    CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell(
        "Creatures you control get +0/+5 and gain reach until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::You,
            },
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(0),
                    toughness: ValueDef::Constant(5),
                },
                AppliedEffectDef::GrantAbility(&abilities::reach()),
            ]),
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// GTC 138 — Verdant Haven
// Audit: blocked — Triggered mana abilities cannot make the resolving one-of-five-colors choice required by the enchanted land's mana trigger.

// GTC 139 — Wasteland Viper
pub(in crate::card::sets) static WASTELAND_VIPER: CardRecord = CardRecord::new(
    cards::WASTELAND_VIPER,
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
                    AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(1),
                        toughness: ValueDef::Constant(2),
                    },
                    AppliedEffectDef::GrantAbility(&abilities::deathtouch()),
                ]),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// GTC 140 — Wildwood Rebirth
pub(in crate::card::sets) static WILDWOOD_REBIRTH: CardRecord = CardRecord::new(
    cards::WILDWOOD_REBIRTH,
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
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            controller: None,
        },
    )),
);

// GTC 141 — Alms Beast
// Audit: blocked — Needs a dynamic combat-relation grant of lifelink to creatures blocking or blocked by the source.

// GTC 142 — Assemble the Legion
pub(in crate::card::sets) static ASSEMBLE_THE_LEGION: CardRecord = CardRecord::new(
    cards::ASSEMBLE_THE_LEGION,
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
                EffectDef::CreateToken {
                    token: cards::SOLDIER_TOKEN_1_1_RED_WHITE,
                    count: ValueDef::CountersOnSource(CounterKind::Muster),
                },
            ]),
        ),
    ),
);

// GTC 143 — Aurelia, the Warleader
pub(in crate::card::sets) static AURELIA_THE_WARLEADER: CardRecord = CardRecord::new(
    cards::AURELIA_THE_WARLEADER,
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
            TriggerEventDef::AttacksFirstTimeThisTurn(ObjectPredicateDef::Source),
            EffectDef::Sequence(&[
                EffectDef::Untap {
                    object: EffectRecipientDef::MatchingObjects {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: PlayerRelation::You,
                    },
                },
                EffectDef::AdditionalCombatPhase,
            ]),
        ),
    ]),
);

// GTC 144 — Aurelia's Fury
// Audit: partial — Damage and target division work, but tap and cast-lock effects apply to selected targets even when prevention means they were not dealt damage this way.
pub(in crate::card::sets) static AURELIAS_FURY: CardRecord = CardRecord::new(
    cards::AURELIAS_FURY,
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
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::CannotCastNoncreatureSpellsThisTurn {
                    player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ]),
        )
        .with_coverage(AbilityCoverageDef::partial(
            "Tap and cast-lock effects cannot be conditioned on damage actually being dealt after prevention.",
        )),
    ),
);

// GTC 145 — Bane Alley Broker
// Audit: blocked — Needs face-down linked exile from hand, permission to look at those cards, and a non-target choice to return one.

static FOUR_BIOVISIONARIES: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef {
        object: ObjectPredicateDef::SharesNameWithSource,
        zones: &[ZoneKind::Battlefield],
        controller: PlayerRelation::You,
    },
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 4,
};

// GTC 146 — Biovisionary
pub(in crate::card::sets) static BIOVISIONARY: CardRecord = CardRecord::new(
    cards::BIOVISIONARY,
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
// Audit: blocked — Needs top-three reveal and partitioning plus a land-card discard cost that a resolving damage ability can identify.

// GTC 148 — Boros Charm
pub(in crate::card::sets) static BOROS_CHARM: CardRecord = CardRecord::new(
    cards::BOROS_CHARM,
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
                    recipient: EffectRecipientDef::MatchingObjects {
                        object: ObjectPredicateDef::Any,
                        zones: &[ZoneKind::Battlefield],
                        controller: PlayerRelation::You,
                    },
                    effect: AppliedEffectDef::GrantAbility(&abilities::indestructible()),
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell_with_targets("Target creature gains double strike until end of turn", &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )], EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::GrantAbility(&abilities::double_strike()),
                    duration: EffectDurationDef::UntilEndOfTurn,
                }),
        ],
    )),
);

// GTC 149 — Call of the Nightwing
// Audit: blocked — Cipher encoding and casting encoded spell copies without paying their mana costs are unavailable.

// GTC 150 — Cartel Aristocrat
// Audit: blocked — Protection needs a resolving color choice, not a fixed color known in the declaration.

// GTC 151 — Clan Defiance
// Audit: blocked — Its flying and nonflying target predicates ignore abilities supplied by continuous static effects.

// GTC 152 — Consuming Aberration
// Audit: blocked — Its cast trigger needs every opponent to reveal through a land and move each revealed group to a graveyard.

// GTC 153 — Deathpact Angel
// Audit: blocked — The token's ability needs a non-target graveyard choice of a card with a specific name.

// GTC 154 — Dimir Charm
// Audit: blocked — Needs a power-at-most target predicate with full static-effect semantics and a top-three choose-one/library-and-graveyard procedure.

// GTC 155 — Dinrova Horror
pub(in crate::card::sets) static DINROVA_HORROR: CardRecord = CardRecord::new(
    cards::DINROVA_HORROR,
    "Dinrova Horror",
    CardArt::new("398df5e6-6bda-467a-81e2-91be7e21d715", "Johann Bodin"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{4}{U}{B}"), &["Horror"], 4, 4).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature enters, return target permanent to its owner's hand, then that player discards a card.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Any,
            )],
            EffectDef::Sequence(&[
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    controller: None,
                },
                EffectDef::Discard {
                    recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                },
            ]),
        ),
    ),
);

/// The available damage effects cover most of fight, but they resolve in
/// sequence instead of committing both damage events simultaneously.
static DOMRI_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::activated(
        "+1: Look at the top card of your library. If it's a creature card, you may reveal it and put it into your hand.",
        &[AbilityCostDef::Loyalty(1)],
        EffectDef::LookAtTopAndMayTake {
            player: EffectRecipientDef::Controller,
            object: ObjectPredicateDef::HasType(CardType::Creature),
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
        "The damage events resolve sequentially, and the target declaration cannot enforce that the second creature is another creature.",
    )),
    AbilityDef::activated(
        "−7: You get an emblem with \"Creatures you control have double strike, trample, hexproof, and haste.\"",
        &[AbilityCostDef::Loyalty(-7)],
        EffectDef::CreateEmblem {
            emblem: cards::DOMRI_RADE_EMBLEM,
        },
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
    }),
];

// GTC 156 — Domri Rade
// Audit: partial — The fight ability deals damage sequentially and cannot enforce that its second target is different; its other loyalty abilities are implemented.
pub(in crate::card::sets) static DOMRI_RADE: CardRecord = CardRecord::new(
    cards::DOMRI_RADE,
    "Domri Rade",
    CardArt::new("21b48170-99dd-440f-9954-fc229d6094d3", "Tyler Jacobson"),
    CardSet::Gatecrash,
    CardRules::new_planeswalker(mana_cost!("{1}{R}{G}"), &["Domri"], 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&DOMRI_ABILITIES),
);

// GTC 157 — Drakewing Krasis
pub(in crate::card::sets) static DRAKEWING_KRASIS: CardRecord = CardRecord::new(
    cards::DRAKEWING_KRASIS,
    "Drakewing Krasis",
    CardArt::new("016d1d17-ba5c-4168-9a3d-232bdcc98c80", "Johann Bodin"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{1}{G}{U}"), &["Lizard", "Drake"], 3, 1)
        .with_abilities(&[abilities::flying(), abilities::trample()]),
);

// GTC 158 — Duskmantle Guildmage
// Audit: blocked — Needs a turn-long trigger for every card entering an opponent's graveyard, keyed to that card's owner.

// GTC 159 — Duskmantle Seer
// Audit: blocked — Needs simultaneous per-player top-card reveal, mana-value life loss, and movement to hand with APNAP handling.

// GTC 160 — Elusive Krasis
// Audit: blocked — Evolve needs a trigger comparison against the entering creature's current power and toughness.

// GTC 161 — Executioner's Swing
// Audit: blocked — No target predicate recognizes a creature that dealt damage earlier this turn.

// GTC 162 — Fathom Mage
// Audit: blocked — Evolve and a trigger for a +1/+1 counter being placed on the source are unavailable.

// GTC 163 — Firemane Avenger
// Audit: blocked — Battalion is a trigger-time restriction, but declarative trigger conditions are rechecked on resolution as intervening-if conditions.

// GTC 164 — Fortress Cyclops
// Audit: blocked — Attacks is available, but there is no trigger event for the source blocking.

static FOUNDRY_CHAMPION_CREATURES: ObjectQueryDef = ObjectQueryDef {
    object: ObjectPredicateDef::HasType(CardType::Creature),
    zones: &[ZoneKind::Battlefield],
    controller: PlayerRelation::You,
};

// GTC 165 — Foundry Champion
pub(in crate::card::sets) static FOUNDRY_CHAMPION: CardRecord = CardRecord::new(
    cards::FOUNDRY_CHAMPION,
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
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
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
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(0),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated(
            "{W}: This creature gets +0/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(0),
                    toughness: ValueDef::Constant(1),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// GTC 166 — Frenzied Tilling
// Audit: blocked — Library search cannot put the found basic land onto the battlefield tapped.

// GTC 167 — Ghor-Clan Rampager
pub(in crate::card::sets) static GHOR_CLAN_RAMPAGER: CardRecord = CardRecord::new(
    cards::GHOR_CLAN_RAMPAGER,
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
                    AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(4),
                        toughness: ValueDef::Constant(4),
                    },
                    AppliedEffectDef::GrantAbility(&abilities::trample()),
                ]),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

static GROUND_ASSAULT_LANDS: ObjectQueryDef = ObjectQueryDef {
    object: ObjectPredicateDef::HasType(CardType::Land),
    zones: &[ZoneKind::Battlefield],
    controller: PlayerRelation::You,
};

// GTC 168 — Ground Assault
pub(in crate::card::sets) static GROUND_ASSAULT: CardRecord = CardRecord::new(
    cards::GROUND_ASSAULT,
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
// Audit: blocked — Needs a turn-long cannot-block sweep, indefinite control restoration, and flying predicates with full continuous-effect semantics.

// GTC 170 — Gruul Ragebeast
// Audit: blocked — Fight damage must be simultaneous, and the entering triggering object must become one participant in a targeted fight.

// GTC 171 — High Priest of Penance
pub(in crate::card::sets) static HIGH_PRIEST_OF_PENANCE: CardRecord = CardRecord::new(
    cards::HIGH_PRIEST_OF_PENANCE,
    "High Priest of Penance",
    CardArt::new("84a3ff8d-6d7e-49f0-8d30-7f8c23db568b", "Mark Zug"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{W}{B}"), &["Human", "Cleric"], 1, 1).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever this creature is dealt damage, you may destroy target nonland permanent.",
            TriggerEventDef::DamageDealt {
                source: ObjectPredicateDef::Any,
                recipient: EffectRecipientDef::Source,
            },
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
            )],
            EffectDef::May(&EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            }),
        ),
    ),
);

// GTC 172 — Hydroform
// Audit: blocked — Animation can add Elemental while retaining land subtypes, but cannot replace prior creature subtypes without also erasing the retained land subtypes.

// GTC 173 — Kingpin's Pet
pub(in crate::card::sets) static KINGPINS_PET: CardRecord = CardRecord::new(
    cards::KINGPINS_PET,
    "Kingpin's Pet",
    CardArt::new("3465cf63-4f10-4b53-9703-69746364dbc7", "Mark Zug"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{1}{W}{B}"), &["Thrull"], 2, 2)
        .with_abilities(&[abilities::flying(), extort()]),
);

// GTC 174 — Lazav, Dimir Mastermind
// Audit: blocked — Copy effects cannot copy a creature card from a graveyard while retaining the source's name, legendary supertype, hexproof, and trigger.

// GTC 175 — Martial Glory
pub(in crate::card::sets) static MARTIAL_GLORY: CardRecord = CardRecord::new(
    cards::MARTIAL_GLORY,
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
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(3),
                        toughness: ValueDef::Constant(0),
                    },
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex(1)),
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(0),
                        toughness: ValueDef::Constant(3),
                    },
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ),
);

// GTC 176 — Master Biomancer
// Audit: blocked — Entry replacement effects cannot read this source's power or add a creature subtype to another entering creature.

// GTC 177 — Merciless Eviction
// Audit: blocked — Needs a simultaneous mass zone move for each selectable permanent type.

// GTC 178 — Mind Grind
// Audit: blocked — Needs each opponent to reveal through X lands and move every revealed group to a graveyard, plus the X-cannot-be-zero cast restriction.

// GTC 179 — Mortus Strider
pub(in crate::card::sets) static MORTUS_STRIDER: CardRecord = CardRecord::new(
    cards::MORTUS_STRIDER,
    "Mortus Strider",
    CardArt::new("d644eb6e-cc49-4834-bc2c-53f6a4ceb451", "Tomasz Jedruszek"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{1}{U}{B}"), &["Skeleton"], 1, 1).with_ability(
        AbilityDef::triggered(
            "When this creature dies, return it to its owner's hand.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: Some(ZoneKind::Battlefield),
                to: Some(ZoneKind::Graveyard),
            },
            EffectDef::MoveToZone {
                object: EffectRecipientDef::TriggeringObject,
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
                controller: None,
            },
        ),
    ),
);

// GTC 180 — Mystic Genesis
// Audit: blocked — Token definitions have fixed characteristics, and no continuation can size a newly created Ooze from the countered spell's mana value.

// GTC 181 — Nimbus Swimmer
// Audit: blocked — Battlefield-entry counter modifications take fixed amounts and cannot read the creature spell's chosen X.

// GTC 182 — Obzedat, Ghost Council
pub(in crate::card::sets) static OBZEDAT_GHOST_COUNCIL: CardRecord = CardRecord::new(
    cards::OBZEDAT_GHOST_COUNCIL,
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
        AbilityDef::triggered_with_targets("When Obzedat enters, target opponent loses 2 life and you gain 2 life.", TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            }, &[AbilityTargetDef::exactly_one(
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
            EffectDef::May(&EffectDef::Sequence(&[
                EffectDef::ExileLinkedToSource {
                    object: EffectRecipientDef::Source,
                },
                // Queued before the exile takes effect would be the same:
                // both read the source from the resolving ability, which the
                // exile does not disturb.
                EffectDef::AtNextStep {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                    effect: &EffectDef::ReturnLinkedExiles {
                        zone: ZoneKind::Battlefield,
                        grant: Some(KeywordAbility::Haste),
                    },
                },
            ])),
        ),
    ]),
);

// GTC 183 — One Thousand Lashes
// Audit: blocked — Needs attack, block, and activated-ability prohibitions plus an upkeep trigger keyed to the attached creature's controller.

// GTC 184 — Ordruun Veteran
// Audit: blocked — Battalion is a trigger-time restriction, but declarative trigger conditions are rechecked on resolution as intervening-if conditions.

// GTC 185 — Orzhov Charm
// Audit: blocked — Needs returning all Auras attached to one target, a target-toughness life-loss value, and a graveyard target predicate with dynamic mana-value semantics.

// GTC 186 — Paranoid Delusions
// Audit: blocked — Cipher encoding and casting encoded spell copies without paying their mana costs are unavailable.

// GTC 187 — Primal Visitation
pub(in crate::card::sets) static PRIMAL_VISITATION: CardRecord = CardRecord::new(
    cards::PRIMAL_VISITATION,
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
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::ModifyPowerToughness {
                            power: ValueDef::Constant(3),
                            toughness: ValueDef::Constant(3),
                        },
                        AppliedEffectDef::GrantAbility(&abilities::haste()),
                    ]),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// GTC 188 — Prime Speaker Zegana
// Audit: blocked — Entry replacement effects cannot read the greatest power among other creatures, and the entry trigger needs the source's post-entry power as a draw count.

// GTC 189 — Psychic Strike
pub(in crate::card::sets) static PSYCHIC_STRIKE: CardRecord = CardRecord::new(
    cards::PSYCHIC_STRIKE,
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
            },
        ]),
    )),
);

// GTC 190 — Purge the Profane
// Audit: blocked — Recipient-chosen discard suspends for a decision, so a following sequence effect would gain life before the printed discard finishes.

// GTC 191 — Rubblehulk
// Audit: blocked — Needs a characteristic-defining power/toughness ability that functions in every zone and a bloodrush value that counts lands at resolution.

// GTC 192 — Ruination Wurm
pub(in crate::card::sets) static RUINATION_WURM: CardRecord = CardRecord::new(
    cards::RUINATION_WURM,
    "Ruination Wurm",
    CardArt::new("ce04d1ee-2605-472d-b3ee-24800342e9af", "Dave Kendall"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{4}{R}{G}"), &["Wurm"], 7, 6),
);

// GTC 193 — Shambleshark
// Audit: blocked — Evolve needs a trigger comparison against the entering creature's current power and toughness.

// GTC 194 — Signal the Clans
// Audit: blocked — Needs an exactly-three library search, distinct-name validation, random selection, and shuffling the unselected cards.

// GTC 195 — Simic Charm
pub(in crate::card::sets) static SIMIC_CHARM: CardRecord = CardRecord::new(
    cards::SIMIC_CHARM,
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
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(3),
                        toughness: ValueDef::Constant(3),
                    },
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell(
                "Permanents you control gain hexproof until end of turn",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::MatchingObjects {
                        object: ObjectPredicateDef::Any,
                        zones: &[ZoneKind::Battlefield],
                        controller: PlayerRelation::You,
                    },
                    effect: AppliedEffectDef::GrantAbility(&abilities::hexproof()),
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell_with_targets(
                "Return target creature to its owner's hand",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    controller: None,
                },
            ),
        ],
    )),
);

// GTC 196 — Skarrg Guildmage
// Audit: blocked — Its land animation cannot replace prior creature subtypes while retaining land subtypes; selective subtype replacement is unavailable.

// GTC 197 — Skyknight Legionnaire
pub(in crate::card::sets) static SKYKNIGHT_LEGIONNAIRE: CardRecord = CardRecord::new(
    cards::SKYKNIGHT_LEGIONNAIRE,
    "Skyknight Legionnaire",
    CardArt::new("ae8c9948-b52e-4d07-a72a-99ab6be05cc6", "Anthony Palumbo"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{1}{R}{W}"), &["Human", "Knight"], 2, 2)
        .with_abilities(&[abilities::flying(), abilities::haste()]),
);

// GTC 198 — Soul Ransom
// Audit: blocked — Needs indefinite control from an Aura and an activated ability restricted to opponents that makes the Aura's controller sacrifice it before drawing.

// GTC 199 — Spark Trooper
pub(in crate::card::sets) static SPARK_TROOPER: CardRecord = CardRecord::new(
    cards::SPARK_TROOPER,
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
pub(in crate::card::sets) static SUNHOME_GUILDMAGE: CardRecord = CardRecord::new(
    cards::SUNHOME_GUILDMAGE,
    "Sunhome Guildmage",
    CardArt::new("42d1122a-099b-49bf-9b53-52429658816a", "Eric Deschamps"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{R}{W}"), &["Human", "Wizard"], 2, 2).with_abilities(&[
        AbilityDef::activated(
            "{1}{R}{W}: Creatures you control get +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{R}{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::You,
                },
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(0),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated(
            "{2}{R}{W}: Create a 1/1 red and white Soldier creature token with haste.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{R}{W}"))],
            EffectDef::CreateToken {
                token: cards::SOLDIER_TOKEN_1_1_RED_WHITE,
                count: ValueDef::Constant(1),
            },
        ),
    ]),
);

// GTC 201 — Treasury Thrull
pub(in crate::card::sets) static TREASURY_THRULL: CardRecord = CardRecord::new(
    cards::TREASURY_THRULL,
    "Treasury Thrull",
    CardArt::new("f013e6f0-85d0-4c8e-a10b-7beea572c32d", "Mark Zug"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{4}{W}{B}"), &["Thrull"], 4, 4).with_abilities(&[
        extort(),
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, you may return target artifact, creature, or enchantment card from your graveyard to your hand.",
            TriggerEventDef::Attacks(ObjectPredicateDef::Source),
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
            EffectDef::May(&EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
                controller: None,
            }),
        ),
    ]),
);

// GTC 202 — Truefire Paladin
pub(in crate::card::sets) static TRUEFIRE_PALADIN: CardRecord = CardRecord::new(
    cards::TRUEFIRE_PALADIN,
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
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(2),
                    toughness: ValueDef::Constant(0),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated(
            "{R}{W}: This creature gains first strike until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{R}{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::GrantAbility(&abilities::first_strike()),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// GTC 203 — Unexpected Results
// Audit: blocked — Needs shuffle-then-reveal branching, casting a nonland without paying its mana cost, or putting a land onto the battlefield and returning the resolving spell.

// GTC 204 — Urban Evolution
// Audit: blocked — Needs a turn-long permission to play one additional land.

// GTC 205 — Vizkopa Confessor
// Audit: blocked — Needs an arbitrary life payment, partial hand reveal, and a resolving choice by the ability controller from the revealed group.

// GTC 206 — Vizkopa Guildmage
// Audit: blocked — TriggerUntilYourNextTurn survives cleanup and can trigger after a printed “this turn” effect should have expired.

// GTC 207 — Whispering Madness
// Audit: blocked — Needs simultaneous whole-hand discard with the greatest discarded count, plus cipher encoding and free copy casting.

// GTC 208 — Wojek Halberdiers
// Audit: blocked — Battalion is a trigger-time restriction, but declarative trigger conditions are rechecked on resolution as intervening-if conditions.

// GTC 209 — Zameck Guildmage
// Audit: blocked — Needs a turn-long entry replacement on future creatures and removing a +1/+1 counter from a chosen creature as a cost.

// GTC 210 — Zhur-Taa Swine
pub(in crate::card::sets) static ZHUR_TAA_SWINE: CardRecord = CardRecord::new(
    cards::ZHUR_TAA_SWINE,
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
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(5),
                    toughness: ValueDef::Constant(4),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// GTC 211 — Arrows of Justice
pub(in crate::card::sets) static ARROWS_OF_JUSTICE: CardRecord = CardRecord::new(
    cards::ARROWS_OF_JUSTICE,
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
pub(in crate::card::sets) static BECKON_APPARITION: CardRecord = CardRecord::new(
    cards::BECKON_APPARITION,
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
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Exile,
                    placement: ZonePlacement::Top,
                    controller: None,
                },
                EffectDef::CreateToken {
                    token: cards::SPIRIT_TOKEN_1_1_WHITE_BLACK,
                    count: ValueDef::Constant(1),
                },
            ]),
        ),
    ),
);

// GTC 213 — Biomass Mutation
// Audit: blocked — Temporary continuous effects cannot set base power and toughness to the spell's chosen X.

// GTC 214 — Bioshift
// Audit: blocked — Needs choosing a variable number of +1/+1 counters to move and constraining two targets to have the same controller.

// GTC 215 — Boros Reckoner
pub(in crate::card::sets) static BOROS_RECKONER: CardRecord = CardRecord::new(
    cards::BOROS_RECKONER,
    "Boros Reckoner",
    CardArt::new("82a18b07-38b8-4854-9735-3cfe83b11bf1", "Howard Lyon"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{R/W}{R/W}{R/W}"), &["Minotaur", "Wizard"], 3, 3)
        .with_abilities(&[
            AbilityDef::triggered_with_targets(
                "Whenever this creature is dealt damage, it deals that much damage to any target.",
                TriggerEventDef::DamageDealt {
                    source: ObjectPredicateDef::Any,
                    recipient: EffectRecipientDef::Source,
                },
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
                    effect: AppliedEffectDef::GrantAbility(&abilities::first_strike()),
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// GTC 216 — Burning-Tree Emissary
pub(in crate::card::sets) static BURNING_TREE_EMISSARY: CardRecord = CardRecord::new(
    cards::BURNING_TREE_EMISSARY,
    "Burning-Tree Emissary",
    CardArt::new("899d5f35-3613-4c69-9176-13baf442fb50", "Izzy"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{R/G}{R/G}"), &["Human", "Shaman"], 2, 2).with_ability(
        AbilityDef::triggered(
            "When this creature enters, add {R}{G}.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::Sequence(&[
                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
            ]),
        ),
    ),
);

// GTC 217 — Coerced Confession
// Audit: blocked — Needs a linked count of creature cards among exactly the cards milled by the preceding effect.

// GTC 218 — Deathcult Rogue
pub(in crate::card::sets) static DEATHCULT_ROGUE: CardRecord = CardRecord::new(
    cards::DEATHCULT_ROGUE,
    "Deathcult Rogue",
    CardArt::new("a4c186d2-e631-4811-83ea-fdb54e730a5d", "David Palumbo"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{1}{U/B}{U/B}"), &["Human", "Rogue"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "This creature can't be blocked except by Rogues.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::CannotBeBlockedBy(ObjectPredicateDef::Not(
                    &ObjectPredicateDef::Subtype("Rogue"),
                )),
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
    ),
);

// GTC 219 — Gift of Orzhova
pub(in crate::card::sets) static GIFT_OF_ORZHOVA: CardRecord = CardRecord::new(
    cards::GIFT_OF_ORZHOVA,
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
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::ModifyPowerToughness {
                            power: ValueDef::Constant(1),
                            toughness: ValueDef::Constant(1),
                        },
                        AppliedEffectDef::GrantAbility(&abilities::flying()),
                        AppliedEffectDef::GrantAbility(&abilities::lifelink()),
                    ]),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// GTC 220 — Immortal Servitude
// Audit: blocked — Needs a simultaneous multi-card graveyard-to-battlefield move filtered by the spell's chosen X.

// GTC 221 — Merfolk of the Depths
pub(in crate::card::sets) static MERFOLK_OF_THE_DEPTHS: CardRecord = CardRecord::new(
    cards::MERFOLK_OF_THE_DEPTHS,
    "Merfolk of the Depths",
    CardArt::new("bddb2e15-a53e-4647-a627-6c7032429fca", "Scott Chou"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{4}{G/U}{G/U}"), &["Merfolk", "Soldier"], 4, 2)
        .with_ability(abilities::flash()),
);

// GTC 222 — Nightveil Specter
// Audit: blocked — Needs combat-damage-player capture, linked face-up exile, and permission to play lands or cast spells exiled by this source.

// GTC 223 — Pit Fight
// Audit: blocked — Fight requires two simultaneous damage events and the target declaration cannot enforce “another” target creature.

static RUBBLEBELT_ATTACKERS: ObjectQueryDef = ObjectQueryDef {
    object: ObjectPredicateDef::Attacking,
    zones: &[ZoneKind::Battlefield],
    controller: PlayerRelation::You,
};

// GTC 224 — Rubblebelt Raiders
pub(in crate::card::sets) static RUBBLEBELT_RAIDERS: CardRecord = CardRecord::new(
    cards::RUBBLEBELT_RAIDERS,
    "Rubblebelt Raiders",
    CardArt::new("2dec7d6a-2362-4c62-bd81-35bba6086f7d", "Chippy"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{1}{R/G}{R/G}{R/G}"), &["Human", "Warrior"], 3, 3)
        .with_ability(AbilityDef::triggered(
            "Whenever this creature attacks, put a +1/+1 counter on it for each attacking creature you control.",
            TriggerEventDef::Attacks(ObjectPredicateDef::Source),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::CountMatchingObjects(&RUBBLEBELT_ATTACKERS),
            },
        )),
);

// GTC 225 — Shattering Blow
pub(in crate::card::sets) static SHATTERING_BLOW: CardRecord = CardRecord::new(
    cards::SHATTERING_BLOW,
    "Shattering Blow",
    CardArt::new("a77058d9-d2b5-424a-bfe2-070b754051cb", "Steve Prescott"),
    CardSet::Gatecrash,
    CardRules::new_instant(mana_cost!("{1}{R/W}")).with_ability(AbilityDef::spell_with_targets(
        "Exile target artifact.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Artifact),
        )],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Exile,
            placement: ZonePlacement::Top,
            controller: None,
        },
    )),
);

// GTC 226 — Armored Transport
// Audit: blocked — Damage prevention cannot select only creatures currently blocking this source.

// GTC 227 — Boros Keyrune
// Audit: blocked — Animation cannot set the keyrune's red-white colors while retaining its printed mana ability and exact subtype behavior.

// GTC 228 — Dimir Keyrune
// Audit: blocked — Animation cannot set blue-black colors while retaining printed abilities, and the temporary unblockable clause must be part of that animation.

// GTC 229 — Glaring Spotlight
// Audit: blocked — Needs a rule override that lets your effects target opposing hexproof creatures as though they lacked hexproof.

// GTC 230 — Gruul Keyrune
// Audit: blocked — Animation cannot set the keyrune's red-green colors while retaining its printed mana ability and exact subtype behavior.

// GTC 231 — Illusionist's Bracers
// Audit: blocked — Needs the equip procedure plus copying a nonmana activated ability of the equipped creature with optional new targets.

// GTC 232 — Millennial Gargoyle
pub(in crate::card::sets) static MILLENNIAL_GARGOYLE: CardRecord = CardRecord::new(
    cards::MILLENNIAL_GARGOYLE,
    "Millennial Gargoyle",
    CardArt::new("98d1bc6e-84aa-4973-924a-6688b742bafa", "Seb McKinnon"),
    CardSet::Gatecrash,
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Gargoyle"], 2, 2)
        .with_ability(abilities::flying()),
);

// GTC 233 — Orzhov Keyrune
// Audit: blocked — Animation cannot set the keyrune's white-black colors while retaining its printed mana ability and exact subtype behavior.

// GTC 234 — Prophetic Prism
// Audit: blocked — The mana-ability procedure cannot combine a mana payment with a tap cost before choosing one of five output colors.

// GTC 235 — Razortip Whip
pub(in crate::card::sets) static RAZORTIP_WHIP: CardRecord = CardRecord::new(
    cards::RAZORTIP_WHIP,
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
// Audit: blocked — Equipment attachment and the equip special action are unavailable.

// GTC 237 — Simic Keyrune
// Audit: blocked — Animation cannot set the keyrune's green-blue colors while retaining its printed mana ability and exact subtype behavior.

// GTC 238 — Skyblinder Staff
// Audit: blocked — Needs Equipment attachment and a blocker predicate whose flying test includes abilities granted by static effects.

// GTC 239 — Boros Guildgate
pub(in crate::card::sets) static BOROS_GUILDGATE: CardRecord = CardRecord::new(
    cards::BOROS_GUILDGATE,
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
pub(in crate::card::sets) static BREEDING_POOL: CardRecord = CardRecord::new(
    cards::BREEDING_POOL,
    "Breeding Pool",
    CardArt::new("ece3bcdd-cb33-4923-b919-ba57a327d3cd", "Mike Bierek"),
    CardSet::Gatecrash,
    CardRules::new_land(&["Forest", "Island"]).with_ability(abilities::shock_land_enters()),
);

// GTC 241 — Dimir Guildgate
pub(in crate::card::sets) static DIMIR_GUILDGATE: CardRecord = CardRecord::new(
    cards::DIMIR_GUILDGATE,
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
pub(in crate::card::sets) static GODLESS_SHRINE: CardRecord = CardRecord::new(
    cards::GODLESS_SHRINE,
    "Godless Shrine",
    CardArt::new("6fd672bb-18cf-44e3-8dda-5310b1e0fffe", "Cliff Childs"),
    CardSet::Gatecrash,
    CardRules::new_land(&["Plains", "Swamp"]).with_ability(abilities::shock_land_enters()),
);

// GTC 243 — Gruul Guildgate
pub(in crate::card::sets) static GRUUL_GUILDGATE: CardRecord = CardRecord::new(
    cards::GRUUL_GUILDGATE,
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
pub(in crate::card::sets) static ORZHOV_GUILDGATE: CardRecord = CardRecord::new(
    cards::ORZHOV_GUILDGATE,
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
pub(in crate::card::sets) static SACRED_FOUNDRY: CardRecord = CardRecord::new(
    cards::SACRED_FOUNDRY,
    "Sacred Foundry",
    CardArt::new("0a26d900-c652-4f9c-8681-a35c5f8b1937", "Sam Burley"),
    CardSet::Gatecrash,
    CardRules::new_land(&["Mountain", "Plains"]).with_ability(abilities::shock_land_enters()),
);

// GTC 246 — Simic Guildgate
pub(in crate::card::sets) static SIMIC_GUILDGATE: CardRecord = CardRecord::new(
    cards::SIMIC_GUILDGATE,
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
pub(in crate::card::sets) static STOMPING_GROUND: CardRecord = CardRecord::new(
    cards::STOMPING_GROUND,
    "Stomping Ground",
    CardArt::new("f29f3415-971c-4a5d-aae9-3893f4bdab1e", "David Palumbo"),
    CardSet::Gatecrash,
    CardRules::new_land(&["Mountain", "Forest"]).with_ability(abilities::shock_land_enters()),
);

// GTC 248 — Thespian's Stage
pub(in crate::card::sets) static THESPIANS_STAGE: CardRecord = CardRecord::new(
    cards::THESPIANS_STAGE,
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
                retain_source_ability: true,
            },
        ),
    ]),
);

// GTC 249 — Watery Grave
pub(in crate::card::sets) static WATERY_GRAVE: CardRecord = CardRecord::new(
    cards::WATERY_GRAVE,
    "Watery Grave",
    CardArt::new("47fde349-010e-4a2e-838e-e924dbeec355", "Raymond Swanland"),
    CardSet::Gatecrash,
    CardRules::new_land(&["Island", "Swamp"]).with_ability(abilities::shock_land_enters()),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AERIAL_MANEUVER,
    &ANGELIC_EDICT,
    &ASSAULT_GRIFFIN,
    &BASILICA_GUARDS,
    &BLIND_OBEDIENCE,
    &COURT_STREET_DENIZEN,
    &DEBTORS_PULPIT,
    &HOLD_THE_GATES,
    &KNIGHT_OF_OBLIGATION,
    &KNIGHT_WATCH,
    &LUMINATE_PRIMORDIAL,
    &RIGHTEOUS_CHARGE,
    &SYNDIC_OF_TITHES,
    &URBIS_PROTECTOR,
    &ZARICHI_TIGER,
    &AGORAPHOBIA,
    &METROPOLIS_SPRITE,
    &MINDEYE_DRAKE,
    &SAGES_ROW_DENIZEN,
    &SCATTER_ARC,
    &TOTALLY_LOST,
    &BASILICA_SCREECHER,
    &CONTAMINATED_GROUND,
    &CORPSE_BLOCKADE,
    &CRYPT_GHAST,
    &GRISLY_SPECTACLE,
    &GUTTER_SKULK,
    &HORROR_OF_THE_DIM,
    &SEPULCHRAL_PRIMORDIAL,
    &SHADOW_ALLEY_DENIZEN,
    &SYNDICATE_ENFORCER,
    &WIGHT_OF_PRECINCT_SIX,
    &ACT_OF_TREASON,
    &CINDER_ELEMENTAL,
    &FOUNDRY_STREET_DENIZEN,
    &HELLRAISER_GOBLIN,
    &HOMING_LIGHTNING,
    &MASSIVE_RAID,
    &MOLTEN_PRIMORDIAL,
    &SCORCHWALKER,
    &SKINBRAND_GOBLIN,
    &TOWERING_THUNDERFIST,
    &VIASHINO_SHANKTAIL,
    &WRECKING_OGRE,
    &BURST_OF_STRENGTH,
    &DISCIPLE_OF_THE_OLD_WAYS,
    &FORCED_ADAPTATION,
    &GREENSIDE_WATCHER,
    &IVY_LANE_DENIZEN,
    &SCAB_CLAN_CHARGER,
    &SKARRG_GOLIATH,
    &SLAUGHTERHORN,
    &TOWER_DEFENSE,
    &WASTELAND_VIPER,
    &WILDWOOD_REBIRTH,
    &ASSEMBLE_THE_LEGION,
    &AURELIA_THE_WARLEADER,
    &AURELIAS_FURY,
    &BIOVISIONARY,
    &BOROS_CHARM,
    &DINROVA_HORROR,
    &DOMRI_RADE,
    &DRAKEWING_KRASIS,
    &FOUNDRY_CHAMPION,
    &GHOR_CLAN_RAMPAGER,
    &GROUND_ASSAULT,
    &HIGH_PRIEST_OF_PENANCE,
    &KINGPINS_PET,
    &MARTIAL_GLORY,
    &MORTUS_STRIDER,
    &OBZEDAT_GHOST_COUNCIL,
    &PRIMAL_VISITATION,
    &PSYCHIC_STRIKE,
    &RUINATION_WURM,
    &SIMIC_CHARM,
    &SKYKNIGHT_LEGIONNAIRE,
    &SPARK_TROOPER,
    &SUNHOME_GUILDMAGE,
    &TREASURY_THRULL,
    &TRUEFIRE_PALADIN,
    &ZHUR_TAA_SWINE,
    &ARROWS_OF_JUSTICE,
    &BECKON_APPARITION,
    &BOROS_RECKONER,
    &BURNING_TREE_EMISSARY,
    &DEATHCULT_ROGUE,
    &GIFT_OF_ORZHOVA,
    &MERFOLK_OF_THE_DEPTHS,
    &RUBBLEBELT_RAIDERS,
    &SHATTERING_BLOW,
    &MILLENNIAL_GARGOYLE,
    &RAZORTIP_WHIP,
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
    PrintingRecord::reprint(&crate::card::sets::y2002::onslaught::NATURALIZE), // GTC 127
];
