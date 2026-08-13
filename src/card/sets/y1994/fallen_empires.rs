use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AnimationDef, AppliedEffectDef, BasicLandType, BattlefieldEntryModificationDef, CardArt,
    CardBehavior, CardRules, CardSet, CardType, CounterKind, DiscardSelectionDef, EffectDef,
    EffectDurationDef, EffectRecipientDef, ManaColor, ObjectPredicateDef, PlayerRelation,
    ReplacementEffectDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, abilities, cards,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

// FEM 1a — Combat Medic
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “{1}{W}: Prevent the next 1 damage that would be dealt to any target this turn”.

// FEM 2 — Farrel's Mantle
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Whenever enchanted creature attacks and isn't blocked, its controller may have it deal damage equal to its power plus 2 to another target creature. If that player does, the attacking…”.

// FEM 3a — Farrel's Zealot
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Whenever this creature attacks and isn't blocked, you may have it deal 3 damage to target creature. If you do, this creature assigns no combat damage this turn”.

// FEM 4 — Farrelite Priest
// Audit: blocked — Needs the mana-ability runtime to pay this ability's mana activation cost for “{1}: Add {W}. If this ability has been activated four or more times this turn, sacrifice this creature at the beginning of the next end step”.

// FEM 5 — Hand of Justice
// Audit: blocked — Needs a persistent tap/untap restriction or event relation for “{T}, Tap three untapped white creatures you control: Destroy target creature”.

// FEM 6 — Heroism
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “Sacrifice a white creature: For each attacking red creature, prevent all combat damage that would be dealt by that creature this turn unless its controller pays {2}{R}”.

// FEM 7a — Icatian Infantry
// Audit: blocked — Needs full banding group declaration, blocking, and combat-damage assignment semantics.

// FEM 8a — Icatian Javelineers
pub(in crate::card::sets) static ICATIAN_JAVELINEERS: CardRecord = CardRecord::new(
    cards::ICATIAN_JAVELINEERS,
    "Icatian Javelineers",
    CardArt::new("f04b8356-2384-4743-80dd-f15ca7ec65f7", "Melissa A. Benson"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 1, 1).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with a javelin counter on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::Javelin,
                    amount: 1,
                },
            ),
        ),
        AbilityDef::activated_with_targets(
            "{T}, Remove a javelin counter from this creature: It deals 1 damage to any target.",
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::RemoveCountersFromSource {
                    kind: CounterKind::Javelin,
                    amount: 1,
                },
            ],
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

// FEM 9 — Icatian Lieutenant
pub(in crate::card::sets) static ICATIAN_LIEUTENANT: CardRecord = CardRecord::new(
    cards::ICATIAN_LIEUTENANT,
    "Icatian Lieutenant",
    CardArt::new("39fec59a-4ade-4c6f-ae7d-911fbe6da26d", "Pete Venters"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Human", "Soldier"], 1, 2).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}{W}: Target Soldier creature gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{W}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Subtype("Soldier"),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(0),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// FEM 10a — Icatian Moneychanger
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “Sacrifice this creature: You gain 1 life for each credit counter on this creature. Activate only during your upkeep”.

// FEM 11 — Icatian Phalanx
// Audit: blocked — Needs full banding group declaration, blocking, and combat-damage assignment semantics.

// FEM 12 — Icatian Priest
pub(in crate::card::sets) static ICATIAN_PRIEST: CardRecord = CardRecord::new(
    cards::ICATIAN_PRIEST,
    "Icatian Priest",
    CardArt::new("d7690cdd-6610-4310-9e93-60dc4db2ae8d", "Drew Tucker"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Cleric"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}{W}{W}: Target creature gets +1/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{W}{W}"))],
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

// FEM 13a — Icatian Scout
pub(in crate::card::sets) static ICATIAN_SCOUT: CardRecord = CardRecord::new(
    cards::ICATIAN_SCOUT,
    "Icatian Scout",
    CardArt::new(
        "86bf4aaa-a9b1-4798-a96b-c3e35afb77f7",
        "Richard Kane Ferguson",
    ),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier", "Scout"], 1, 1)
        .with_abilities(&[AbilityDef::activated_with_targets(
            "{1}, {T}: Target creature gains first strike until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::GrantAbility(&abilities::first_strike()),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        )]),
);

// FEM 14 — Icatian Skirmishers
// Audit: blocked — Needs full banding group declaration, blocking, and combat-damage assignment semantics.

// FEM 15 — Icatian Town
pub(in crate::card::sets) static ICATIAN_TOWN: CardRecord = CardRecord::new(
    cards::ICATIAN_TOWN,
    "Icatian Town",
    CardArt::new("cbb7c28d-0366-4d01-84a2-f1bc9f38aa4a", "Tom Wänerstrand"),
    CardSet::FallenEmpires,
    CardRules::new_sorcery(mana_cost!("{5}{W}")).with_abilities(&[AbilityDef::spell(
        "Create four 1/1 white Citizen creature tokens.",
        EffectDef::CreateToken {
            token: cards::CITIZEN_TOKEN_1_1_WHITE,
            count: ValueDef::Constant(4),
        },
    )]),
);

// FEM 16a — Order of Leitbur
pub(in crate::card::sets) static ORDER_OF_LEITBUR: CardRecord = CardRecord::new(
    cards::ORDER_OF_LEITBUR,
    "Order of Leitbur",
    CardArt::new("ebd6e51e-f042-4673-a898-291607105829", "Bryon Wackwitz"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Human", "Cleric", "Knight"], 2, 1)
        .with_abilities(&[
            abilities::protection_from(ManaColor::Black),
            AbilityDef::activated(
                "{W}: This creature gains first strike until end of turn.",
                &[AbilityCostDef::Mana(mana_cost!("{W}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::GrantAbility(&abilities::first_strike()),
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::activated(
                "{W}{W}: This creature gets +1/+0 until end of turn.",
                &[AbilityCostDef::Mana(mana_cost!("{W}{W}"))],
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

// FEM 17 — Deep Spawn
// Audit: blocked — Needs executable shroud target-legality and a temporary keyword grant for “{U}: This creature gains shroud until end of turn and doesn't untap during your next untap step. Tap this creature”.

// FEM 18a — High Tide
// Audit: blocked — Needs cost/mana provenance or dynamic payment support for “Until end of turn, whenever a player taps an Island for mana, that player adds an additional {U}”.

// FEM 19a — Homarid
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “As long as there are exactly three tide counters on this creature, it gets +1/+1”.

// FEM 20 — Homarid Shaman
pub(in crate::card::sets) static HOMARID_SHAMAN: CardRecord = CardRecord::new(
    cards::HOMARID_SHAMAN,
    "Homarid Shaman",
    CardArt::new("c17c6416-86d6-46ea-aea1-41b98a66b250", "Amy Weber"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Homarid", "Shaman"], 2, 1).with_abilities(
        &[AbilityDef::activated_with_targets(
            "{U}: Tap target green creature.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::Green),
                ]),
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        )],
    ),
);

// FEM 21 — Homarid Spawning Bed
// Audit: blocked — Needs Camarid token creation whose count is the sacrificed creature's mana value.

// FEM 22a — Homarid Warrior
// Audit: blocked — Needs executable shroud target-legality and a temporary keyword grant for “{U}: This creature gains shroud until end of turn and doesn't untap during your next untap step. Tap it”.

// FEM 23a — Merseine
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “Enchanted creature doesn't untap during its controller's untap step if this Aura has a net counter on it”.

// FEM 24 — River Merfolk
pub(in crate::card::sets) static RIVER_MERFOLK: CardRecord = CardRecord::new(
    cards::RIVER_MERFOLK,
    "River Merfolk",
    CardArt::new("27d7fa54-4b89-4a9a-b088-4b89c525c1ea", "Douglas Shuler"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{U}{U}"), &["Merfolk"], 2, 1).with_abilities(&[
        AbilityDef::activated(
            "{U}: This creature gains mountainwalk until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::GrantAbility(&abilities::mountainwalk()),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// FEM 25 — Seasinger
// Audit: blocked — Needs duration-aware control-changing continuous effects for “{T}: Gain control of target creature whose controller controls an Island for as long as you control this creature and this creature remains tapped”.

// FEM 26 — Svyelunite Priest
// Audit: blocked — Needs executable shroud target-legality and a temporary keyword grant for “{U}{U}, {T}: Target creature gains shroud until end of turn. Activate only during your upkeep”.

// FEM 27a — Tidal Flats
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “{U}{U}: For each attacking creature without flying, its controller may pay {1}. If that player doesn't, creatures you control blocking that creature gain first strike until end of turn”.

// FEM 28 — Tidal Influence
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “As long as there are exactly three tide counters on this enchantment, all blue creatures get +2/+0”.

// FEM 29 — Vodalian Knights
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “This creature can't attack unless defending player controls an Island”.

// FEM 30a — Vodalian Mage
pub(in crate::card::sets) static VODALIAN_MAGE: CardRecord = CardRecord::new(
    cards::VODALIAN_MAGE,
    "Vodalian Mage",
    CardArt::new("c107e82b-134a-4f2b-98c2-6537fae6a50d", "Susan Van Camp"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Merfolk", "Wizard"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{U}, {T}: Counter target spell unless its controller pays {1}.",
            &[
                AbilityCostDef::Mana(mana_cost!("{U}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Any)],
            EffectDef::CounterUnlessPaid {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
                zone: ZoneKind::Graveyard,
            },
        ),
    ]),
);

// FEM 31a — Vodalian Soldiers
pub(in crate::card::sets) static VODALIAN_SOLDIERS: CardRecord = CardRecord::new(
    cards::VODALIAN_SOLDIERS,
    "Vodalian Soldiers",
    CardArt::new("7eb50256-9113-4b03-bcef-9aea24be8493", "Melissa A. Benson"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Merfolk", "Soldier"], 1, 2),
);

// FEM 32 — Vodalian War Machine
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Tap an untapped Merfolk you control: This creature can attack this turn as though it didn't have defender”.

// FEM 33a — Armor Thrull
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “{T}, Sacrifice this creature: Put a +1/+2 counter on target creature”.

// FEM 34a — Basal Thrull
pub(in crate::card::sets) static BASAL_THRULL: CardRecord = CardRecord::new(
    cards::BASAL_THRULL,
    "Basal Thrull",
    CardArt::new("0c1d5d13-0160-48cb-8fac-dd86102569b4", "Kaja Foglio"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{B}{B}"), &["Thrull"], 1, 2).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}, Sacrifice this creature: Add {B}{B}.",
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black).with_amount(2)),
        ),
    ]),
);

// FEM 35 — Breeding Pit
pub(in crate::card::sets) static BREEDING_PIT: CardRecord = CardRecord::new(
    cards::BREEDING_PIT,
    "Breeding Pit",
    CardArt::new("a0d7e85f-eba5-4fc5-9fc0-109109d368aa", "Anson Maddocks"),
    CardSet::FallenEmpires,
    CardRules::new_enchantment(mana_cost!("{3}{B}")).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, sacrifice this enchantment unless you pay {B}{B}.",
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
        AbilityDef::triggered(
            "At the beginning of your end step, create a 0/1 black Thrull creature token.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            EffectDef::CreateToken {
                token: cards::THRULL_TOKEN_0_1_BLACK,
                count: ValueDef::Constant(1),
            },
        ),
    ]),
);

// FEM 36 — Derelor
// Audit: blocked — Needs a spell-color predicate in trigger capture for “Black spells you cast cost {B} more to cast”.

// FEM 37 — Ebon Praetor
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “Sacrifice a creature: Remove a -2/-2 counter from this creature. If the sacrificed creature was a Thrull, put a +1/+0 counter on this creature. Activate only during your upkeep and only…”.

// FEM 38a — Hymn to Tourach
pub(in crate::card::sets) static HYMN_TO_TOURACH: CardRecord = CardRecord::new(
    cards::HYMN_TO_TOURACH,
    "Hymn to Tourach",
    CardArt::new("eb9273ea-9a41-42e3-8c9c-0d50b127a818", "Susan Van Camp"),
    CardSet::FallenEmpires,
    CardRules::new_sorcery(mana_cost!("{B}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target player discards two cards at random.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Discard {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(2),
            selection: DiscardSelectionDef::Random,
        },
    )]),
);

// FEM 39a — Initiates of the Ebon Hand
// Audit: blocked — Needs the mana-ability runtime to pay this ability's mana activation cost for “{1}: Add {B}. If this ability has been activated four or more times this turn, sacrifice this creature at the beginning of the next end step”.

// FEM 40a — Mindstab Thrull
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Whenever this creature attacks and isn't blocked, you may sacrifice it. If you do, defending player discards three cards”.

// FEM 41a — Necrite
// Audit: blocked — Needs a duration-scoped prohibition on creating or applying regeneration shields for “Whenever this creature attacks and isn't blocked, you may sacrifice it. If you do, destroy target creature defending player controls. It can't be regenerated”.

// FEM 42a — Order of the Ebon Hand
pub(in crate::card::sets) static ORDER_OF_THE_EBON_HAND: CardRecord = CardRecord::new(
    cards::ORDER_OF_THE_EBON_HAND,
    "Order of the Ebon Hand",
    CardArt::new("9e51f5d8-a7cc-4720-8af5-e002bcfd78a0", "Melissa A. Benson"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{B}{B}"), &["Cleric", "Knight"], 2, 1).with_abilities(&[
        abilities::protection_from(ManaColor::White),
        AbilityDef::activated(
            "{B}: This creature gains first strike until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::GrantAbility(&abilities::first_strike()),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated(
            "{B}{B}: This creature gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{B}{B}"))],
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

// FEM 43 — Soul Exchange
// Audit: blocked — Needs a zone-object query and identity-preserving continuation for “As an additional cost to cast this spell, exile a creature you control”.

// FEM 44 — Thrull Champion
// Audit: blocked — Needs duration-aware control-changing continuous effects for “{T}: Gain control of target Thrull for as long as you control this creature”.

// FEM 45 — Thrull Retainer
// Audit: blocked — Needs regeneration shields and their destroy-event replacement procedure for “Sacrifice this Aura: Regenerate enchanted creature”.

// FEM 46 — Thrull Wizard
// Audit: blocked — Needs a spell-color predicate in trigger capture for “{1}{B}: Counter target black spell unless that spell's controller pays {B} or {3}”.

// FEM 47 — Tourach's Chant
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “Whenever a player puts a Forest onto the battlefield, this enchantment deals 3 damage to that player unless they put a -1/-1 counter on a creature they control”.

// FEM 48 — Tourach's Gate
// Audit: blocked — Needs the clause's conditional recipient set or dynamic modifier value for “Tap enchanted land: Attacking creatures you control get +2/-1 until end of turn. Activate only if enchanted land is untapped”.

// FEM 49a — Brassclaw Orcs
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “This creature can't block creatures with power 2 or greater”.

// FEM 50 — Dwarven Armorer
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “{R}, {T}, Discard a card: Put a +0/+1 counter or a +1/+0 counter on target creature”.

// FEM 51 — Dwarven Catapult
// Audit: blocked — Needs damage divided evenly with downward rounding across a dynamically counted opponent creature set.

// FEM 52 — Dwarven Lieutenant
pub(in crate::card::sets) static DWARVEN_LIEUTENANT: CardRecord = CardRecord::new(
    cards::DWARVEN_LIEUTENANT,
    "Dwarven Lieutenant",
    CardArt::new("ea9a38b1-4676-425a-b40d-4fb478966024", "Jeff A. Menges"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{R}{R}"), &["Dwarf", "Soldier"], 1, 2).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}{R}: Target Dwarf creature gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{R}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Subtype("Dwarf"),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(0),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// FEM 53a — Dwarven Soldier
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Whenever this creature blocks or becomes blocked by one or more Orcs, this creature gets +0/+2 until end of turn”.

// FEM 54a — Goblin Chirurgeon
// Audit: blocked — Needs regeneration shields and their destroy-event replacement procedure for “Sacrifice a Goblin: Regenerate target creature”.

// FEM 55 — Goblin Flotilla
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “At the beginning of each combat, unless you pay {R}, whenever this creature blocks or becomes blocked by a creature this combat, that creature gains first strike until end of turn”.

// FEM 56a — Goblin Grenade
pub(in crate::card::sets) static GOBLIN_GRENADE: CardRecord = CardRecord::new(
    cards::GOBLIN_GRENADE,
    "Goblin Grenade",
    CardArt::new("8837eaba-9602-4f63-9897-85583fcdcf51", "Ron Spencer"),
    CardSet::FallenEmpires,
    CardRules::new_sorcery(mana_cost!("{R}")).with_abilities(&[
        AbilityDef::custom_full(
            "As an additional cost to cast this spell, sacrifice a Goblin.\nGoblin Grenade deals 5 damage to any target.",
            CardBehavior::GoblinGrenade,
            "The additional cost, target selection, and damage are implemented by the legacy spell resolver.",
        ),
    ]),
);

// FEM 57 — Goblin Kites
// Audit: blocked — Needs a deterministic recorded coin-flip choice and both result branches for “{R}: Target creature you control with toughness 2 or less gains flying until end of turn. Flip a coin at the beginning of the next end step. If you lose the flip, sacrifice that creature”.

// FEM 58a — Goblin War Drums
// Audit: blocked — Needs menace as an executable minimum-blocker constraint and external keyword grant for “Creatures you control have menace”.

// FEM 59 — Goblin Warrens
// Audit: blocked — Needs an activated cost that selects and sacrifices two Goblins; only one chosen permanent can currently be sacrificed as a cost.

// FEM 60 — Orcish Captain
// Audit: blocked — Needs a deterministic recorded coin-flip choice and both result branches for “{1}: Flip a coin. If you win the flip, target Orc creature gets +2/+0 until end of turn. If you lose the flip, it gets -0/-2 until end of turn”.

// FEM 61a — Orcish Spy
// Audit: blocked — Needs ordered-library inspection, selection, and visibility handling for “{T}: Look at the top three cards of target player's library”.

// FEM 62a — Orcish Veteran
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “This creature can't block white creatures with power 2 or greater”.

// FEM 63 — Orgg
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “This creature can't attack if defending player controls an untapped creature with power 3 or greater”.

// FEM 64 — Raiding Party
// Audit: blocked — Needs a persistent tap/untap restriction or event relation for “Sacrifice an Orc: Each player may tap any number of untapped white creatures they control. For each creature tapped this way, that player chooses up to two Plains. Then destroy all…”.

// FEM 65a — Elven Fortress
pub(in crate::card::sets) static ELVEN_FORTRESS: CardRecord = CardRecord::new(
    cards::ELVEN_FORTRESS,
    "Elven Fortress",
    CardArt::new("9387105d-46d0-4db0-8980-dd0fded15eef", "Pete Venters"),
    CardSet::FallenEmpires,
    CardRules::new_enchantment(mana_cost!("{G}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}{G}: Target blocking creature gets +0/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{G}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AttackingOrBlocking,
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Attacking),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(0),
                    toughness: ValueDef::Constant(1),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

static THELONITE_DRUID_ANIMATION: AnimationDef = AnimationDef::new(2, 3);

// FEM 66 — Elvish Farmer
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “Remove three spore counters from this creature: Create a 1/1 green Saproling creature token”.

// FEM 67a — Elvish Hunter
// Audit: blocked — Needs a persistent tap/untap restriction or event relation for “{1}{G}, {T}: Target creature doesn't untap during its controller's next untap step”.

// FEM 68a — Elvish Scout
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “{G}, {T}: Untap target attacking creature you control. Prevent all combat damage that would be dealt to and dealt by it this turn”.

// FEM 69 — Feral Thallid
// Audit: blocked — Needs regeneration shields and their destroy-event replacement procedure for “Remove three spore counters from this creature: Regenerate this creature”.

// FEM 70 — Fungal Bloom
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “{G}{G}: Put a spore counter on target Fungus”.

// FEM 71a — Night Soil
// Audit: blocked — Needs a zone-object query and identity-preserving continuation for “{1}, Exile two creature cards from a single graveyard: Create a 1/1 green Saproling creature token”.

// FEM 72a — Spore Cloud
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “Tap all blocking creatures. Prevent all combat damage that would be dealt this turn. Each attacking creature and each blocking creature doesn't untap during its controller's next untap step”.

// FEM 73 — Spore Flower
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “Remove three spore counters from this creature: Prevent all combat damage that would be dealt this turn”.

// FEM 74a — Thallid
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “Remove three spore counters from this creature: Create a 1/1 green Saproling creature token”.

// FEM 75 — Thallid Devourer
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “Remove three spore counters from this creature: Create a 1/1 green Saproling creature token”.

// FEM 76 — Thelon's Chant
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “Whenever a player puts a Swamp onto the battlefield, this enchantment deals 3 damage to that player unless the player puts a -1/-1 counter on a creature they control”.

// FEM 77 — Thelon's Curse
// Audit: blocked — Needs a persistent tap/untap restriction or event relation for “At the beginning of each player's upkeep, that player may choose any number of tapped blue creatures they control and pay {U} for each creature chosen this way. If the player does, untap…”.

// FEM 78 — Thelonite Druid
pub(in crate::card::sets) static THELONITE_DRUID: CardRecord = CardRecord::new(
    cards::THELONITE_DRUID,
    "Thelonite Druid",
    CardArt::new(
        "cd8772dd-513d-4dd0-a5db-5214dc8da4e0",
        "Margaret Organ-Kean",
    ),
    CardSet::FallenEmpires,
    CardRules::new_creature(
        mana_cost!("{2}{G}"),
        &["Human", "Cleric", "Druid"],
        1,
        1,
    )
    .with_ability(AbilityDef::activated(
        "{1}{G}, {T}, Sacrifice a creature: Forests you control become 2/3 creatures until end of turn. They're still lands.",
        &[
            AbilityCostDef::Mana(mana_cost!("{1}{G}")),
            AbilityCostDef::TapSource,
            AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::You,
            },
        ],
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::You,
            },
            effect: AppliedEffectDef::Animate(&THELONITE_DRUID_ANIMATION),
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// FEM 79 — Thelonite Monk
// Audit: blocked — Needs a resolving land-type-setting operation; SetLandTypes currently runs only as a static continuous effect.

// FEM 80a — Thorn Thallid
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “Remove three spore counters from this creature: It deals 1 damage to any target”.

// FEM 81 — Aeolipile
pub(in crate::card::sets) static AEOLIPILE: CardRecord = CardRecord::new(
    cards::AEOLIPILE,
    "Aeolipile",
    CardArt::new("a09030ee-415c-45af-bf08-7623197a314f", "Heather Hudson"),
    CardSet::FallenEmpires,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}, {T}, Sacrifice this artifact: It deals 2 damage to any target.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
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

// FEM 82 — Balm of Restoration
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “• Prevent the next 2 damage that would be dealt to any target this turn”.

// FEM 83 — Conch Horn
// Audit: blocked — Needs ordered-library inspection, selection, and visibility handling for “{1}, {T}, Sacrifice this artifact: Draw two cards, then put a card from your hand on top of your library”.

// FEM 84 — Delif's Cone
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “{T}, Sacrifice this artifact: This turn, when target creature you control attacks and isn't blocked, you may gain life equal to its power. If you do, it assigns no combat damage this turn”.

// FEM 85 — Delif's Cube
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “{2}, {T}: This turn, when target creature you control attacks and isn't blocked, it assigns no combat damage this turn and you put a cube counter on this artifact”.

// FEM 86 — Draconian Cylix
// Audit: blocked — Needs regeneration shields and their destroy-event replacement procedure for “{2}, {T}, Discard a card at random: Regenerate target creature”.

// FEM 87 — Elven Lyre
pub(in crate::card::sets) static ELVEN_LYRE: CardRecord = CardRecord::new(
    cards::ELVEN_LYRE,
    "Elven Lyre",
    CardArt::new("c3a8cd72-04c0-46f7-a249-f1cecddfdc26", "Kaja Foglio"),
    CardSet::FallenEmpires,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}, {T}, Sacrifice this artifact: Target creature gets +2/+2 until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(2),
                    toughness: ValueDef::Constant(2),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// FEM 88 — Implements of Sacrifice
// Audit: blocked — Needs the mana-ability runtime to pay the ability's {1} activation cost.

// FEM 89 — Ring of Renewal
pub(in crate::card::sets) static RING_OF_RENEWAL: CardRecord = CardRecord::new(
    cards::RING_OF_RENEWAL,
    "Ring of Renewal",
    CardArt::new("a532d38a-809b-4132-8690-be15fe23afab", "Douglas Shuler"),
    CardSet::FallenEmpires,
    CardRules::new_artifact(mana_cost!("{5}")).with_abilities(&[AbilityDef::activated(
        "{5}, {T}: Discard a card at random, then draw two cards.",
        &[
            AbilityCostDef::Mana(mana_cost!("{5}")),
            AbilityCostDef::TapSource,
        ],
        EffectDef::Sequence(&[
            EffectDef::Discard {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::Random,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ]),
    )]),
);

// FEM 90 — Spirit Shield
// Audit: blocked — Needs the clause's conditional recipient set or dynamic modifier value for “{2}, {T}: Target creature gets +0/+2 for as long as this artifact remains tapped”.

// FEM 91 — Zelyon Sword
// Audit: blocked — Needs the clause's conditional recipient set or dynamic modifier value for “{3}, {T}: Target creature gets +2/+0 for as long as this artifact remains tapped”.

// FEM 92 — Bottomless Vault
// Audit: blocked — Needs storage counters plus an arbitrary remove-any-number counter cost whose chosen count determines the mana produced.

// FEM 93 — Dwarven Hold
// Audit: blocked — Needs storage counters plus an arbitrary remove-any-number counter cost whose chosen count determines the mana produced.

// FEM 94 — Dwarven Ruins
pub(in crate::card::sets) static DWARVEN_RUINS: CardRecord = CardRecord::new(
    cards::DWARVEN_RUINS,
    "Dwarven Ruins",
    CardArt::new("0dfe1352-27be-4c99-a58f-b961f911f270", "Mark Poole"),
    CardSet::FallenEmpires,
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped.",
            ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
        ),
        AbilityDef::activated_mana(
            "{T}: Add {R}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
        ),
        AbilityDef::activated_mana(
            "{T}, Sacrifice this land: Add {R}{R}.",
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red).with_amount(2)),
        ),
    ]),
);

// FEM 95 — Ebon Stronghold
pub(in crate::card::sets) static EBON_STRONGHOLD: CardRecord = CardRecord::new(
    cards::EBON_STRONGHOLD,
    "Ebon Stronghold",
    CardArt::new("3fb2a11f-a8e4-4acf-871a-11171e3304ef", "Mark Poole"),
    CardSet::FallenEmpires,
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped.",
            ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
        ),
        AbilityDef::activated_mana(
            "{T}: Add {B}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black)),
        ),
        AbilityDef::activated_mana(
            "{T}, Sacrifice this land: Add {B}{B}.",
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black).with_amount(2)),
        ),
    ]),
);

// FEM 96 — Havenwood Battleground
pub(in crate::card::sets) static HAVENWOOD_BATTLEGROUND: CardRecord = CardRecord::new(
    cards::HAVENWOOD_BATTLEGROUND,
    "Havenwood Battleground",
    CardArt::new("9028f200-80dd-4c53-877f-ea380ff417cb", "Mark Poole"),
    CardSet::FallenEmpires,
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped.",
            ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
        ),
        AbilityDef::activated_mana(
            "{T}: Add {G}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
        ),
        AbilityDef::activated_mana(
            "{T}, Sacrifice this land: Add {G}{G}.",
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green).with_amount(2)),
        ),
    ]),
);

// FEM 97 — Hollow Trees
// Audit: blocked — Needs storage counters plus an arbitrary remove-any-number counter cost whose chosen count determines the mana produced.

// FEM 98 — Icatian Store
// Audit: blocked — Needs storage counters plus an arbitrary remove-any-number counter cost whose chosen count determines the mana produced.

// FEM 99 — Rainbow Vale
// Audit: blocked — Needs duration-aware control-changing continuous effects for “{T}: Add one mana of any color. An opponent gains control of this land at the beginning of the next end step”.

// FEM 100 — Ruins of Trokair
pub(in crate::card::sets) static RUINS_OF_TROKAIR: CardRecord = CardRecord::new(
    cards::RUINS_OF_TROKAIR,
    "Ruins of Trokair",
    CardArt::new("4ce2e734-8cff-4bfe-85f8-17b3e1903f18", "Mark Poole"),
    CardSet::FallenEmpires,
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped.",
            ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
        ),
        AbilityDef::activated_mana(
            "{T}: Add {W}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::White)),
        ),
        AbilityDef::activated_mana(
            "{T}, Sacrifice this land: Add {W}{W}.",
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::White).with_amount(2)),
        ),
    ]),
);

// FEM 101 — Sand Silos
// Audit: blocked — Needs storage counters plus an arbitrary remove-any-number counter cost whose chosen count determines the mana produced.

// FEM 102 — Svyelunite Temple
pub(in crate::card::sets) static SVYELUNITE_TEMPLE: CardRecord = CardRecord::new(
    cards::SVYELUNITE_TEMPLE,
    "Svyelunite Temple",
    CardArt::new("8b3fde62-ab21-459b-9c5d-01aa6fe1d08e", "Mark Poole"),
    CardSet::FallenEmpires,
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped.",
            ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
        ),
        AbilityDef::activated_mana(
            "{T}: Add {U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue)),
        ),
        AbilityDef::activated_mana(
            "{T}, Sacrifice this land: Add {U}{U}.",
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue).with_amount(2)),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ICATIAN_JAVELINEERS,
    &ICATIAN_LIEUTENANT,
    &ICATIAN_PRIEST,
    &ICATIAN_SCOUT,
    &ICATIAN_TOWN,
    &ORDER_OF_LEITBUR,
    &HOMARID_SHAMAN,
    &RIVER_MERFOLK,
    &VODALIAN_MAGE,
    &VODALIAN_SOLDIERS,
    &BASAL_THRULL,
    &BREEDING_PIT,
    &HYMN_TO_TOURACH,
    &ORDER_OF_THE_EBON_HAND,
    &DWARVEN_LIEUTENANT,
    &GOBLIN_GRENADE,
    &ELVEN_FORTRESS,
    &THELONITE_DRUID,
    &AEOLIPILE,
    &ELVEN_LYRE,
    &RING_OF_RENEWAL,
    &DWARVEN_RUINS,
    &EBON_STRONGHOLD,
    &HAVENWOOD_BATTLEGROUND,
    &RUINS_OF_TROKAIR,
    &SVYELUNITE_TEMPLE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
