use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityPredicateDef, AbilityTargetDef,
    AbilityTargetPredicate, AddManaEffectDef, AppliedEffectDef, CardArt, CardBehavior, CardRules,
    CardSet, CardType, ComparisonDef, EffectDef, EffectDurationDef, EffectExecutionDef,
    EffectRecipientDef, KeywordAbility, ManaColor, ObjectPredicateDef, ObjectQueryDef,
    PlayerRelation, TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind,
    ZonePlacement, abilities, cards,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

// DRK 1 — Angry Mob
// Audit: blocked — Needs a characteristic-layer effect or dynamic value for “During your turn, Angry Mob's power and toughness are each equal to 2 plus the number of Swamps your opponents control. During turns other than yours, Angry Mob's power and toughness are…”.

// DRK 2 — Blood of the Martyr
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “Until end of turn, if damage would be dealt to any creature, you may have that damage dealt to you instead”.

// DRK 3 — Brainwash
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Enchanted creature can't attack unless its controller pays {3}”.

// DRK 4 — Cleansing
// Audit: blocked — Needs linked sacrifice/destruction accounting for “For each land, destroy that land unless any player pays 1 life”.

// DRK 5 — Dust to Dust
pub(in crate::card::sets) static DUST_TO_DUST: CardRecord = CardRecord::new(
    cards::DUST_TO_DUST,
    "Dust to Dust",
    CardArt::new("ade075fd-73ee-4d12-a2da-48e5938043af", "Drew Tucker"),
    CardSet::TheDark,
    CardRules::new_sorcery(mana_cost!("{1}{W}{W}")).with_abilities(&[AbilityDef::custom_full(
        "Exile two target artifacts.",
        CardBehavior::DustToDust,
        "Artifact targeting and exile are implemented by the legacy spell resolver.",
    )]),
);

// DRK 6 — Exorcist
pub(in crate::card::sets) static EXORCIST: CardRecord = CardRecord::new(
    cards::EXORCIST,
    "Exorcist",
    CardArt::new("184b7d52-e991-4668-9f6a-bcded97f51ac", "Drew Tucker"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Human", "Cleric"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}{W}, {T}: Destroy target black creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{W}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::Black),
                ]),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
        ),
    ]),
);

// DRK 7 — Fasting
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “If you would begin your draw step, you may skip that step instead. If you do, you gain 2 life”.

// DRK 8 — Festival
// Audit: blocked — Needs a spell-casting timing condition tied to the active turn and step for “Cast this spell only during an opponent's upkeep”.

// DRK 9 — Fire and Brimstone
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Fire and Brimstone deals 4 damage to target player who attacked this turn and 4 damage to you”.

// DRK 10 — Holy Light
pub(in crate::card::sets) static HOLY_LIGHT: CardRecord = CardRecord::new(
    cards::HOLY_LIGHT,
    "Holy Light",
    CardArt::new("c3c8a850-bc99-4679-a316-45ecdea696b2", "Drew Tucker"),
    CardSet::TheDark,
    CardRules::new_instant(mana_cost!("{2}{W}")).with_abilities(&[AbilityDef::spell(
        "Nonwhite creatures get -1/-1 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::White)),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
            effect: AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Constant(-1),
                toughness: ValueDef::Constant(-1),
            },
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// DRK 11 — Knights of Thorn
// Audit: blocked — Needs the named-color protection rules and Aura self-retention exception for “Protection from red; banding”.

// DRK 12 — Martyr's Cry
// Audit: blocked — Needs a zone-object query and identity-preserving continuation for “Exile all white creatures. For each creature exiled this way, its controller draws a card”.

// DRK 13 — Miracle Worker
// Audit: blocked — Needs linked sacrifice/destruction accounting for “{T}: Destroy target Aura attached to a creature you control”.

// DRK 14 — Morale
pub(in crate::card::sets) static MORALE: CardRecord = CardRecord::new(
    cards::MORALE,
    "Morale",
    CardArt::new("c4104546-abd9-4bfb-a65e-5928cdd4522f", "Mark Poole"),
    CardSet::TheDark,
    CardRules::new_instant(mana_cost!("{1}{W}{W}")).with_abilities(&[AbilityDef::spell(
        "Attacking creatures get +1/+1 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::Attacking,
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
            effect: AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Constant(1),
                toughness: ValueDef::Constant(1),
            },
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// DRK 15 — Pikemen
// Audit: blocked — Needs full banding group declaration, blocking, and combat-damage assignment semantics.

// DRK 16 — Preacher
// Audit: blocked — Needs duration-aware control-changing continuous effects for “{T}: For as long as this creature remains tapped, gain control of target creature of an opponent's choice they control”.

// DRK 17 — Squire
pub(in crate::card::sets) static SQUIRE: CardRecord = CardRecord::new(
    cards::SQUIRE,
    "Squire",
    CardArt::new("374df061-ebd2-4f1f-9a6e-7940a49197a9", "Dennis Detwiller"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Soldier"], 1, 2),
);

// DRK 18 — Tivadar's Crusade
pub(in crate::card::sets) static TIVADARS_CRUSADE: CardRecord = CardRecord::new(
    cards::TIVADARS_CRUSADE,
    "Tivadar's Crusade",
    CardArt::new("8b6da540-6803-47e5-9af0-7ae8e2f84b6c", "Dennis Detwiller"),
    CardSet::TheDark,
    CardRules::new_sorcery(mana_cost!("{1}{W}{W}")).with_abilities(&[AbilityDef::spell(
        "Destroy all Goblins.",
        EffectDef::Destroy {
            object: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::Subtype("Goblin"),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
            can_regenerate: true,
        },
    )]),
);

// DRK 19 — Witch Hunter
pub(in crate::card::sets) static WITCH_HUNTER: CardRecord = CardRecord::new(
    cards::WITCH_HUNTER,
    "Witch Hunter",
    CardArt::new("4eef9bb7-cd3c-422e-a93b-90d98684675a", "Jesper Myrfors"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Human", "Cleric"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: This creature deals 1 damage to target player or planeswalker.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_with_targets(
            "{1}{W}{W}, {T}: Return target creature an opponent controls to its owner's hand.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{W}{W}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                controller: None,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// DRK 20 — Amnesia
// Audit: blocked — Needs ordered-library inspection, selection, and visibility handling for “Target player reveals their hand and discards all nonland cards”.

// DRK 21 — Apprentice Wizard
// Audit: blocked — Needs the mana-ability runtime to pay this ability's mana activation cost for “{U}, {T}: Add {C}{C}{C}”.

// DRK 22 — Dance of Many
// Audit: blocked — Needs copiable-value or rules-text mutation support for “When this enchantment enters, create a token that's a copy of target nontoken creature”.

// DRK 23 — Deep Water
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “{U}: Until end of turn, if you tap a land you control for mana, it produces {U} instead of any other type”.

// DRK 24 — Drowned
// Audit: blocked — Needs regeneration shields and their destroy-event replacement procedure for “{B}: Regenerate this creature”.

// DRK 25 — Electric Eel
pub(in crate::card::sets) static ELECTRIC_EEL: CardRecord = CardRecord::new(
    cards::ELECTRIC_EEL,
    "Electric Eel",
    CardArt::new("b8834c18-0e4e-4785-9d15-b33345e3789b", "Anson Maddocks"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{U}"), &["Fish"], 1, 1).with_abilities(&[
        AbilityDef::triggered(
            "When this creature enters, it deals 1 damage to you.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "{R}{R}: This creature gets +2/+0 until end of turn and deals 1 damage to you.",
            &[AbilityCostDef::Mana(mana_cost!("{R}{R}"))],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(2),
                        toughness: ValueDef::Constant(0),
                    },
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ]),
);

// DRK 26 — Erosion
// Audit: blocked — Needs an upkeep trigger whose event player is derived from the attached permanent's current controller for “At the beginning of the upkeep of enchanted land's controller, destroy that land unless that player pays {1} or 1 life”.

// DRK 27 — Flood
// Audit: partial — Its flying predicate omits flying granted by static continuous effects.
pub(in crate::card::sets) static FLOOD: CardRecord = CardRecord::new(
    cards::FLOOD,
    "Flood",
    CardArt::new("fabc3267-b59b-4f36-8873-5b4b072711ca", "Dennis Detwiller"),
    CardSet::TheDark,
    CardRules::new_enchantment(mana_cost!("{U}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{U}{U}: Tap target creature without flying.",
            &[AbilityCostDef::Mana(mana_cost!("{U}{U}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                        crate::card::KeywordAbility::Flying,
                    )),
                ]),
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "The target predicate sees printed and temporary flying but not flying granted by static continuous effects.",
        )),
    ]),
);

// DRK 28 — Ghost Ship
// Audit: blocked — Needs regeneration shields and their destroy-event replacement procedure for “{U}{U}{U}: Regenerate this creature”.

// DRK 29 — Giant Shark
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Whenever this creature blocks or becomes blocked by a creature that has been dealt damage this turn, this creature gets +2/+0 and gains trample until end of turn”.

// DRK 30 — Leviathan
// Audit: blocked — Needs a persistent tap/untap restriction or event relation for “This creature enters tapped and doesn't untap during your untap step”.

// DRK 31 — Mana Vortex
// Audit: blocked — Needs linked sacrifice/destruction accounting for “At the beginning of each player's upkeep, that player sacrifices a land of their choice”.

// DRK 32 — Merfolk Assassin
// Audit: blocked — Needs the printed landwalk variant and its defending-player land/blocking semantics for “{T}: Destroy target creature with islandwalk”.

// DRK 33 — Mind Bomb
// Audit: blocked — Needs an ordered choice for each player to discard up to three cards and damage derived from each actual discard count.

// DRK 34 — Psychic Allergy
// Audit: blocked — Needs a persistent dynamic characteristic choice and predicates that consume it for “At the beginning of each opponent's upkeep, this enchantment deals X damage to that player, where X is the number of nontoken permanents of the chosen color they control”.

// DRK 35 — Riptide
pub(in crate::card::sets) static RIPTIDE: CardRecord = CardRecord::new(
    cards::RIPTIDE,
    "Riptide",
    CardArt::new(
        "b0f11ae4-e30e-441d-bb64-439930d9997c",
        "Randy Asplund-Faith",
    ),
    CardSet::TheDark,
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[AbilityDef::spell(
        "Tap all blue creatures.",
        EffectDef::Tap {
            object: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::Blue),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
        },
    )]),
);

// DRK 36 — Sunken City
pub(in crate::card::sets) static SUNKEN_CITY: CardRecord = CardRecord::new(
    cards::SUNKEN_CITY,
    "Sunken City",
    CardArt::new("f1e0f9ec-2b06-4bda-8b80-a716d82d1f13", "Jesper Myrfors"),
    CardSet::TheDark,
    CardRules::new_enchantment(mana_cost!("{U}{U}")).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, sacrifice this enchantment unless you pay {U}{U}.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::UnlessPaid {
                cost: mana_cost!("{U}{U}"),
                otherwise: &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            },
        ),
        AbilityDef::static_ability(
            "Blue creatures get +1/+1.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Color(ManaColor::Blue),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::Any,
                },
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(1),
                },
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
    ]),
);

static WATER_WURM_OPPONENT_ISLAND: ObjectQueryDef = ObjectQueryDef {
    object: ObjectPredicateDef::HasAnyBasicLandType(&[crate::card::BasicLandType::Island]),
    zones: &[ZoneKind::Battlefield],
    controller: PlayerRelation::Opponent,
};

// DRK 37 — Tangle Kelp
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Enchanted creature doesn't untap during its controller's untap step if it attacked during its controller's last turn”.

// DRK 38 — Water Wurm
pub(in crate::card::sets) static WATER_WURM: CardRecord = CardRecord::new(
    cards::WATER_WURM,
    "Water Wurm",
    CardArt::new("e3da4a88-5225-467f-9240-f30bc1eee520", "Ron Spencer"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{U}"), &["Wurm"], 1, 1).with_abilities(&[
        AbilityDef::static_ability(
            "This creature gets +0/+1 as long as an opponent controls an Island.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(0),
                    toughness: ValueDef::AnyMatchingObject(&WATER_WURM_OPPONENT_ISLAND),
                },
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
    ]),
);

// DRK 39 — Ashes to Ashes
pub(in crate::card::sets) static ASHES_TO_ASHES: CardRecord = CardRecord::new(
    cards::ASHES_TO_ASHES,
    "Ashes to Ashes",
    CardArt::new("825496e5-19c7-4f50-8070-0265a58608dc", "Drew Tucker"),
    CardSet::TheDark,
    CardRules::new_sorcery(mana_cost!("{1}{B}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Exile two target nonartifact creatures. Ashes to Ashes deals 5 damage to you.",
            &[AbilityTargetDef {
                predicate: AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Artifact)),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                minimum: 2,
                maximum: 2,
                divided_total: None,
            }],
            EffectDef::Sequence(&[
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Exile,
                    controller: None,
                    placement: ZonePlacement::Top,
                },
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(5),
                },
            ]),
        ),
    ]),
);

// DRK 40 — Banshee
// Audit: blocked — Needs damage-history/source tracking or card-specific damage processing for “{X}, {T}: This creature deals half X damage, rounded down, to any target, and half X damage, rounded up, to you”.

// DRK 41 — Bog Imp
pub(in crate::card::sets) static BOG_IMP: CardRecord = CardRecord::new(
    cards::BOG_IMP,
    "Bog Imp",
    CardArt::new("e3bb7271-634a-4612-9073-7a5438e8c2b8", "Ron Spencer"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Imp"], 1, 1)
        .with_abilities(&[abilities::flying()]),
);

// DRK 42 — Bog Rats
pub(in crate::card::sets) static BOG_RATS: CardRecord = CardRecord::new(
    cards::BOG_RATS,
    "Bog Rats",
    CardArt::new("d64c9153-bc6d-4a64-885f-c039a5487a31", "Ron Spencer"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{B}"), &["Rat"], 1, 1).with_abilities(&[
        AbilityDef::static_ability(
            "This creature can't be blocked by Walls.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::CannotBeBlockedBy(ObjectPredicateDef::Subtype("Wall")),
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
    ]),
);

// DRK 43 — Curse Artifact
// Audit: blocked — Needs an upkeep trigger whose event player is derived from the attached permanent's current controller for “At the beginning of the upkeep of enchanted artifact's controller, this Aura deals 2 damage to that player unless they sacrifice that artifact”.

// DRK 44 — Eater of the Dead
// Audit: blocked — Needs a zone-object query and identity-preserving continuation for “{0}: If this creature is tapped, exile target creature card from a graveyard and untap this creature”.

// DRK 45 — Frankenstein's Monster
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “As this creature enters, exile X creature cards from your graveyard. If you can't, put this creature into its owner's graveyard instead of onto the battlefield. For each creature card…”.

// DRK 46 — Grave Robbers
pub(in crate::card::sets) static GRAVE_ROBBERS: CardRecord = CardRecord::new(
    cards::GRAVE_ROBBERS,
    "Grave Robbers",
    CardArt::new("a131605a-f646-4745-a1e4-48d155a3d94f", "Quinton Hoover"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Human", "Rogue"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{B}, {T}: Exile target artifact card from a graveyard. You gain 2 life.",
            &[
                AbilityCostDef::Mana(mana_cost!("{B}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Artifact),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Exile,
                    controller: None,
                    placement: ZonePlacement::Top,
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ]),
        ),
    ]),
);

// DRK 47 — Inquisition
// Audit: blocked — Needs ordered-library inspection, selection, and visibility handling for “Target player reveals their hand. Inquisition deals damage to that player equal to the number of white cards in their hand”.

// DRK 48 — Marsh Gas
pub(in crate::card::sets) static MARSH_GAS: CardRecord = CardRecord::new(
    cards::MARSH_GAS,
    "Marsh Gas",
    CardArt::new("b80ecb15-258b-4fc9-86e4-c2bf01891606", "Douglas Shuler"),
    CardSet::TheDark,
    CardRules::new_instant(mana_cost!("{B}")).with_abilities(&[AbilityDef::spell(
        "All creatures get -2/-0 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
            effect: AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Constant(-2),
                toughness: ValueDef::Constant(0),
            },
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// DRK 49 — Murk Dwellers
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Whenever this creature attacks and isn't blocked, it gets +2/+0 until end of combat”.

// DRK 50 — Nameless Race
// Audit: blocked — Needs a characteristic-layer effect or dynamic value for “Nameless Race's power and toughness are each equal to the life paid as it entered”.

// DRK 51 — Rag Man
// Audit: blocked — Needs seeded random selection with replay-visible provenance for “{B}{B}{B}, {T}: Target opponent reveals their hand and discards a creature card at random. Activate only during your turn”.

// DRK 52 — Season of the Witch
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “At the beginning of the end step, destroy all untapped creatures that didn't attack this turn, except for creatures that couldn't attack”.

// DRK 53 — The Fallen
// Audit: blocked — Needs the printed landwalk variant and its defending-player land/blocking semantics for “At the beginning of your upkeep, this creature deals 1 damage to each opponent and planeswalker it has dealt damage to this game”.

// DRK 54 — Uncle Istvan
// Audit: partial — Its prevention misses creature sources sacrificed before their damage resolves.
pub(in crate::card::sets) static UNCLE_ISTVAN: CardRecord = CardRecord::new(
    cards::UNCLE_ISTVAN,
    "Uncle Istvan",
    CardArt::new("848ad6d5-3a7e-4d6b-9929-36465796871f", "Daniel Gelon"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{1}{B}{B}{B}"), &["Human"], 1, 3).with_abilities(&[
        AbilityDef::static_ability(
            "Prevent all damage that would be dealt to this creature by creatures.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::PreventDamageFrom(ObjectPredicateDef::HasType(
                    CardType::Creature,
                )),
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "Damage prevention recognizes live creature permanents, but not a creature source sacrificed before its damage resolves.",
        )),
    ]),
);

// DRK 55 — Word of Binding
// Audit: blocked — Needs a persistent tap/untap restriction or event relation for “Tap X target creatures”.

// DRK 56 — Worms of the Earth
// Audit: blocked — Needs an any-player upkeep choice between sacrificing two lands and taking damage, followed by conditional self-destruction.

// DRK 57 — Ball Lightning
pub(in crate::card::sets) static BALL_LIGHTNING: CardRecord = CardRecord::new(
    cards::BALL_LIGHTNING,
    "Ball Lightning",
    CardArt::new("c1ba83ab-83f5-421d-bba1-0f925870b5c8", "Quinton Hoover"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{R}{R}{R}"), &["Elemental"], 6, 1).with_abilities(&[
        abilities::trample(),
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

// DRK 58 — Blood Moon
pub(in crate::card::sets) static BLOOD_MOON: CardRecord = CardRecord::new(
    cards::BLOOD_MOON,
    "Blood Moon",
    CardArt::new("78373616-e2d6-4ccf-998f-09f02bea45b4", "Tom Wänerstrand"),
    CardSet::TheDark,
    CardRules::new_enchantment(mana_cost!("{2}{R}")).with_abilities(&[AbilityDef::static_ability(
        "Nonbasic lands are Mountains.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(
                        crate::card::CardSupertype::Basic,
                    )),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
            effect: AppliedEffectDef::SetLandTypes(&[crate::card::BasicLandType::Mountain]),
            duration: EffectDurationDef::WhileSourceRemainsInZone,
        },
    )]),
);

// DRK 59 — Brothers of Fire
pub(in crate::card::sets) static BROTHERS_OF_FIRE: CardRecord = CardRecord::new(
    cards::BROTHERS_OF_FIRE,
    "Brothers of Fire",
    CardArt::new("ba2cc4a6-fdcc-4082-801a-d2c50e560e8d", "Mark Tedin"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Human", "Shaman"], 2, 2).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}{R}{R}: This creature deals 1 damage to any target and 1 damage to you.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{R}{R}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::Sequence(&[
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ]),
);

// DRK 60 — Cave People
pub(in crate::card::sets) static CAVE_PEOPLE: CardRecord = CardRecord::new(
    cards::CAVE_PEOPLE,
    "Cave People",
    CardArt::new("72746a5d-faa1-44b7-97b5-0ef9302a3c13", "Drew Tucker"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Human"], 1, 4).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature attacks, it gets +1/-2 until end of turn.",
            TriggerEventDef::Attacks(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(-2),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated_with_targets(
            "{1}{R}{R}, {T}: Target creature gains mountainwalk until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{R}{R}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::GrantAbility(&abilities::mountainwalk()),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// DRK 61 — Eternal Flame
// Audit: blocked — Needs the printed landwalk variant and its defending-player land/blocking semantics for “Eternal Flame deals X damage to target opponent or planeswalker and half X damage, rounded up, to you, where X is the number of Mountains you control”.

// DRK 62 — Fire Drake
// Audit: blocked — Needs a per-object, per-turn activation quota for “{R}: This creature gets +1/+0 until end of turn. Activate only once each turn”.

// DRK 63 — Fissure
pub(in crate::card::sets) static FISSURE: CardRecord = CardRecord::new(
    cards::FISSURE,
    "Fissure",
    CardArt::new("aa2d778d-d74b-45ec-a86b-5d52ffad6ba5", "Douglas Shuler"),
    CardSet::TheDark,
    CardRules::new_instant(mana_cost!("{3}{R}{R}")).with_abilities(&[AbilityDef::destroy_target(
        "Destroy target creature or land. It can't be regenerated.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::HasType(CardType::Land),
        ])),
        false,
    )]),
);

// DRK 64 — Goblin Caves
// Audit: blocked — Needs a static condition that reads the enchanted land's current basic type before applying the subtype-wide modifier for “As long as enchanted land is a basic Mountain, Goblin creatures get +0/+2”.

// DRK 65 — Goblin Digging Team
pub(in crate::card::sets) static GOBLIN_DIGGING_TEAM: CardRecord = CardRecord::new(
    cards::GOBLIN_DIGGING_TEAM,
    "Goblin Digging Team",
    CardArt::new("8a538b9d-351e-40bb-be11-9ba08c16352b", "Ron Spencer"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice this creature: Destroy target Wall.",
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Subtype("Wall"),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
        ),
    ]),
);

// DRK 66 — Goblin Hero
pub(in crate::card::sets) static GOBLIN_HERO: CardRecord = CardRecord::new(
    cards::GOBLIN_HERO,
    "Goblin Hero",
    CardArt::new("7135a569-e5d3-4a1f-924b-bdb86926b4e1", "Mark Tedin"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin"], 2, 2),
);

/// Any Dwarf you control at all, which is why this is a count of at least one
/// rather than an exact number.
static GOBLINS_OF_THE_FLARG_DWARF_CONDITION: TriggerConditionDef =
    TriggerConditionDef::ObjectCount {
        query: ObjectQueryDef {
            object: ObjectPredicateDef::Subtype("Dwarf"),
            zones: &[ZoneKind::Battlefield],
            controller: PlayerRelation::You,
        },
        comparison: ComparisonDef::GreaterOrEqual,
        amount: 1,
    };

// DRK 67 — Goblin Rock Sled
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “This creature doesn't untap during your untap step if it attacked during your last turn”.

// DRK 68 — Goblin Shrine
// Audit: blocked — Needs a static condition that reads the enchanted land's current basic type before applying the subtype-wide modifier for “As long as enchanted land is a basic Mountain, Goblin creatures get +1/+0”.

// DRK 69 — Goblin Wizard
// Audit: blocked — Needs a hidden-zone decision and continuation for “{T}: You may put a Goblin permanent card from your hand onto the battlefield”.

// DRK 70 — Goblins of the Flarg
pub(in crate::card::sets) static GOBLINS_OF_THE_FLARG: CardRecord = CardRecord::new(
    cards::GOBLINS_OF_THE_FLARG,
    "Goblins of the Flarg",
    CardArt::new("fd333b18-b896-4ab8-9c46-eed4efdd94f2", "Tom Wänerstrand"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin", "Warrior"], 1, 1).with_abilities(&[
        abilities::mountainwalk(),
        AbilityDef::triggered_if(
            "When you control a Dwarf, sacrifice this creature.",
            TriggerEventDef::StateCondition,
            &GOBLINS_OF_THE_FLARG_DWARF_CONDITION,
            EffectDef::Sacrifice {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// DRK 71 — Inferno
pub(in crate::card::sets) static INFERNO: CardRecord = CardRecord::new(
    cards::INFERNO,
    "Inferno",
    CardArt::new(
        "a6b61512-5b24-424c-966f-36b595781e14",
        "Randy Asplund-Faith",
    ),
    CardSet::TheDark,
    CardRules::new_instant(mana_cost!("{5}{R}{R}")).with_abilities(&[AbilityDef::spell(
        "Inferno deals 6 damage to each creature and each player.",
        EffectDef::Sequence(&[
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::Any,
                },
                amount: ValueDef::Constant(6),
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::EachPlayer,
                amount: ValueDef::Constant(6),
            },
        ]),
    )]),
);

// DRK 72 — Mana Clash
// Audit: blocked — Needs a deterministic recorded coin-flip choice and both result branches for “You and target opponent each flip a coin. Mana Clash deals 1 damage to each player whose coin comes up tails. Repeat this process until both players' coins come up heads on the same flip”.

// DRK 73 — Orc General
pub(in crate::card::sets) static ORC_GENERAL: CardRecord = CardRecord::new(
    cards::ORC_GENERAL,
    "Orc General",
    CardArt::new("65a10fd5-506e-46bf-87e6-fde134c0dc04", "Jesper Myrfors"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Orc", "Warrior"], 2, 2).with_abilities(&[
        AbilityDef::activated(
            "{T}, Sacrifice another Orc or Goblin: Other Orc creatures get +1/+1 until end of turn.",
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::Subtype("Orc"),
                            ObjectPredicateDef::Subtype("Goblin"),
                        ]),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::Apply {
                recipient: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Orc"),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::Any,
                },
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(1),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// DRK 74 — Sisters of the Flame
pub(in crate::card::sets) static SISTERS_OF_THE_FLAME: CardRecord = CardRecord::new(
    cards::SISTERS_OF_THE_FLAME,
    "Sisters of the Flame",
    CardArt::new("564e0ccd-decb-48d2-981f-cefa8045340f", "Jesper Myrfors"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Human", "Shaman"], 2, 2).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {R}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
        ),
    ]),
);

// DRK 75 — Carnivorous Plant
pub(in crate::card::sets) static CARNIVOROUS_PLANT: CardRecord = CardRecord::new(
    cards::CARNIVOROUS_PLANT,
    "Carnivorous Plant",
    CardArt::new("6a615650-4da3-4efc-aa5e-c1f2c4f79478", "Quinton Hoover"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Plant", "Wall"], 4, 5)
        .with_abilities(&[abilities::defender()]),
);

// DRK 76 — Elves of Deep Shadow
// Audit: blocked — Needs the mana-ability runtime to pay this ability's mana activation cost for “{T}: Add {B}. This creature deals 1 damage to you”.

// DRK 77 — Gaea's Touch
// Audit: blocked — Needs a hidden-zone decision and continuation for “{0}: You may put a basic Forest card from your hand onto the battlefield. Activate only as a sorcery and only once each turn”.

// DRK 78 — Hidden Path
pub(in crate::card::sets) static HIDDEN_PATH: CardRecord = CardRecord::new(
    cards::HIDDEN_PATH,
    "Hidden Path",
    CardArt::new("cbc93c0b-0ac8-4b8f-b2f6-96887d1acd77", "Rob Alexander"),
    CardSet::TheDark,
    CardRules::new_enchantment(mana_cost!("{2}{G}{G}{G}{G}")).with_abilities(&[
        AbilityDef::static_ability(
            "Green creatures have forestwalk.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Color(ManaColor::Green),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::Any,
                },
                effect: AppliedEffectDef::GrantAbility(&abilities::forestwalk()),
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
    ]),
);

// DRK 79 — Land Leeches
pub(in crate::card::sets) static LAND_LEECHES: CardRecord = CardRecord::new(
    cards::LAND_LEECHES,
    "Land Leeches",
    CardArt::new("ff99543d-86a1-44f8-88ec-aaec071d6c05", "Quinton Hoover"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Leech"], 2, 2)
        .with_abilities(&[abilities::first_strike()]),
);

// DRK 80 — Lurker
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “This creature can't be the target of spells unless it attacked or blocked this turn”.

// DRK 81 — Marsh Viper
// Audit: blocked — Needs player poison counters and the poison-based state check, including this card's counter placement.

// DRK 82 — Niall Silvain
// Audit: blocked — Needs regeneration shields and their destroy-event replacement procedure for “{G}{G}{G}{G}, {T}: Regenerate target creature”.

// DRK 83 — People of the Woods
// Audit: blocked — Needs a characteristic-layer effect or dynamic value for “People of the Woods's toughness is equal to the number of Forests you control”.

// DRK 84 — Savaen Elves
// Audit: blocked — Needs linked sacrifice/destruction accounting for “{G}{G}, {T}: Destroy target Aura attached to a land”.

// DRK 85 — Scarwood Bandits
// Audit: blocked — Needs duration-aware control-changing continuous effects for “{2}{G}, {T}: Unless an opponent pays {2}, gain control of target artifact for as long as this creature remains on the battlefield”.

// DRK 86 — Scarwood Hag
pub(in crate::card::sets) static SCARWOOD_HAG: CardRecord = CardRecord::new(
    cards::SCARWOOD_HAG,
    "Scarwood Hag",
    CardArt::new("ac2655e4-3a4d-4f73-820a-02fab675d42e", "Anson Maddocks"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Hag"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{G}{G}{G}{G}, {T}: Target creature gains forestwalk until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{G}{G}{G}{G}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::GrantAbility(&abilities::forestwalk()),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated_with_targets(
            "{T}: Target creature loses forestwalk until end of turn.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::RemoveAbilities(AbilityPredicateDef::Keyword(
                    KeywordAbility::Forestwalk,
                )),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// DRK 87 — Scavenger Folk
pub(in crate::card::sets) static SCAVENGER_FOLK: CardRecord = CardRecord::new(
    cards::SCAVENGER_FOLK,
    "Scavenger Folk",
    CardArt::new("8e99870c-b2b9-431b-b8a8-3f4a80aa8fa5", "Dennis Detwiller"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{G}"), &["Human"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{G}, {T}, Sacrifice this creature: Destroy target artifact.",
            &[
                AbilityCostDef::Mana(mana_cost!("{G}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Artifact),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
        ),
    ]),
);

// DRK 88 — Spitting Slug
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Whenever this creature blocks or becomes blocked, you may pay {1}{G}. If you do, this creature gains first strike until end of turn. Otherwise, each creature blocking or blocked by this…”.

// DRK 89 — Tracker
pub(in crate::card::sets) static TRACKER: CardRecord = CardRecord::new(
    cards::TRACKER,
    "Tracker",
    CardArt::new("35ffc69e-26f2-434f-8c89-2df108dd984a", "Jeff A. Menges"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human"], 2, 2).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{G}{G}, {T}: This creature deals damage equal to its power to target creature. That creature deals damage equal to its power to this creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{G}{G}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::SourcePower,
                },
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Source,
                    amount: ValueDef::TargetPower(TargetIndex::PRIMARY),
                },
            ]),
        ),
    ]),
);

// DRK 90 — Venom
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Whenever enchanted creature blocks or becomes blocked by a non-Wall creature, destroy the other creature at end of combat”.

// DRK 91 — Whippoorwill
// Audit: blocked — Needs a duration-scoped prohibition on creating or applying regeneration shields for “{G}{G}, {T}: Target creature can't be regenerated this turn. Damage that would be dealt to that creature this turn can't be prevented or dealt instead to another permanent or player.…”.

// DRK 92 — Wormwood Treefolk
// Audit: blocked — Needs the printed landwalk variant and its defending-player land/blocking semantics for “{G}{G}: This creature gains forestwalk until end of turn and deals 2 damage to you”.

// DRK 93 — Marsh Goblins
// Audit: blocked — Needs the printed landwalk variant and its defending-player land/blocking semantics for “Swampwalk”.

// DRK 94 — Scarwood Goblins
pub(in crate::card::sets) static SCARWOOD_GOBLINS: CardRecord = CardRecord::new(
    cards::SCARWOOD_GOBLINS,
    "Scarwood Goblins",
    CardArt::new("5542d236-af43-43b8-b30f-8980d74bbdd0", "Ron Spencer"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{R}{G}"), &["Goblin"], 2, 2),
);

// DRK 95 — Dark Heart of the Wood
pub(in crate::card::sets) static DARK_HEART_OF_THE_WOOD: CardRecord = CardRecord::new(
    cards::DARK_HEART_OF_THE_WOOD,
    "Dark Heart of the Wood",
    CardArt::new("e3d3df64-1e90-4aef-86ae-0062aa23ff30", "Christopher Rush"),
    CardSet::TheDark,
    CardRules::new_enchantment(mana_cost!("{B}{G}")).with_abilities(&[AbilityDef::activated(
        "Sacrifice a Forest: You gain 3 life.",
        &[AbilityCostDef::SacrificePermanent {
            object: ObjectPredicateDef::Subtype("Forest"),
            controller: PlayerRelation::You,
        }],
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(3),
        },
    )]),
);

// DRK 96 — Barl's Cage
// Audit: blocked — Needs a persistent tap/untap restriction or event relation for “{3}: Target creature doesn't untap during its controller's next untap step”.

// DRK 97 — Bone Flute
pub(in crate::card::sets) static BONE_FLUTE: CardRecord = CardRecord::new(
    cards::BONE_FLUTE,
    "Bone Flute",
    CardArt::new("63a31de0-d764-4ff6-a85f-027e1e58d86c", "Christopher Rush"),
    CardSet::TheDark,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[AbilityDef::activated(
        "{2}, {T}: All creatures get -1/-0 until end of turn.",
        &[
            AbilityCostDef::Mana(mana_cost!("{2}")),
            AbilityCostDef::TapSource,
        ],
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
            effect: AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Constant(-1),
                toughness: ValueDef::Constant(0),
            },
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// DRK 98 — Book of Rass
pub(in crate::card::sets) static BOOK_OF_RASS: CardRecord = CardRecord::new(
    cards::BOOK_OF_RASS,
    "Book of Rass",
    CardArt::new("5a391ada-e9e3-45db-ae84-17421ac6b44d", "Sandra Everingham"),
    CardSet::TheDark,
    CardRules::new_artifact(mana_cost!("{6}"))
        .with_subtypes(&["Book"])
        .with_abilities(&[AbilityDef::activated(
            "{2}, Pay 2 life: Draw a card.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}")),
                AbilityCostDef::PayLife(2),
            ],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        )]),
);

// DRK 99 — Coal Golem
// Audit: blocked — Needs the mana-ability runtime to pay this ability's mana activation cost for “{3}, Sacrifice this creature: Add {R}{R}{R}”.

// DRK 100 — Dark Sphere
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “{T}, Sacrifice this artifact: The next time a source of your choice would deal damage to you this turn, prevent half that damage, rounded down”.

// DRK 101 — Diabolic Machine
// Audit: blocked — Needs regeneration shields and their destroy-event replacement procedure for “{3}: Regenerate this creature”.

// DRK 102 — Fellwar Stone
pub(in crate::card::sets) static FELLWAR_STONE: CardRecord = CardRecord::new(
    cards::FELLWAR_STONE,
    "Fellwar Stone",
    CardArt::new("dc47e322-f8b8-4685-b035-fda0cc433e6b", "Quinton Hoover"),
    CardSet::TheDark,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[AbilityDef::activated_mana(
        "{T}: Add one mana of any color that a land an opponent controls could produce.",
        &[AbilityCostDef::TapSource],
        EffectDef::Special("Add one mana of a color an opponent's land could produce"),
    )
    .with_effect_execution(EffectExecutionDef::Custom(CardBehavior::FellwarStone))
    .with_coverage(AbilityCoverageDef::explained_complete(
        "The available colors are computed dynamically from an opponent's lands.",
    ))
    .with_legacy_procedure()]),
);

// DRK 103 — Fountain of Youth
pub(in crate::card::sets) static FOUNTAIN_OF_YOUTH: CardRecord = CardRecord::new(
    cards::FOUNTAIN_OF_YOUTH,
    "Fountain of Youth",
    CardArt::new("2b60eb23-cb9a-4203-86fb-60e47dbd870b", "Daniel Gelon"),
    CardSet::TheDark,
    CardRules::new_artifact(mana_cost!("{0}")).with_abilities(&[AbilityDef::activated(
        "{2}, {T}: You gain 1 life.",
        &[
            AbilityCostDef::Mana(mana_cost!("{2}")),
            AbilityCostDef::TapSource,
        ],
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )]),
);

// DRK 104 — Living Armor
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “{T}, Sacrifice this artifact: Put X +0/+1 counters on target creature, where X is that creature's mana value”.

// DRK 105 — Necropolis
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “Exile a creature card from your graveyard: Put X +0/+1 counters on this creature, where X is the exiled card's mana value”.

// DRK 106 — Reflecting Mirror
// Audit: blocked — Needs a stack-spell target-change effect plus an activation cost derived from twice that spell's mana value.

// DRK 107 — Runesword
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “{3}, {T}: Target attacking creature gets +2/+0 until end of turn. When that creature leaves the battlefield this turn, sacrifice this artifact. If the creature deals damage to a creature…”.

// DRK 108 — Scarecrow
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “{6}, {T}: Prevent all damage that would be dealt to you this turn by creatures with flying”.

// DRK 109 — Skull of Orm
pub(in crate::card::sets) static SKULL_OF_ORM: CardRecord = CardRecord::new(
    cards::SKULL_OF_ORM,
    "Skull of Orm",
    CardArt::new("aa1d9bb5-972a-4705-bf22-0fa1e974dd26", "Tom Wänerstrand"),
    CardSet::TheDark,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{5}, {T}: Return target enchantment card from your graveyard to your hand.",
            &[
                AbilityCostDef::Mana(mana_cost!("{5}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Enchantment),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                controller: None,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// DRK 110 — Standing Stones
// Audit: blocked — Needs cost/mana provenance or dynamic payment support for “{1}, {T}, Pay 1 life: Add one mana of any color”.

// DRK 111 — Stone Calendar
// Audit: blocked — Needs a battlefield-wide static spell-cost reduction; the available generic cost reducer applies only to the source card in hand.

// DRK 112 — Tormod's Crypt
pub(in crate::card::sets) static TORMODS_CRYPT: CardRecord = CardRecord::new(
    cards::TORMODS_CRYPT,
    "Tormod's Crypt",
    CardArt::new("79be5dc2-fab0-4ca1-a044-83e599ed1b41", "Christopher Rush"),
    CardSet::TheDark,
    CardRules::new_artifact(mana_cost!("{0}")).with_ability(AbilityDef::activated_with_targets(
        "{T}, Sacrifice this artifact: Exile target player's graveyard.",
        &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::CardsOwnedByTarget {
                object: ObjectPredicateDef::Any,
                zones: &[ZoneKind::Graveyard],
                slot: TargetIndex::PRIMARY,
            },
            zone: ZoneKind::Exile,
            placement: ZonePlacement::Top,
            controller: None,
        },
    )),
);

// DRK 113 — Tower of Coireall
// Audit: metadata-only — Needs a temporary blocking restriction limited to Wall blockers; the available effect would make the creature completely unblockable.
pub(in crate::card::sets) static TOWER_OF_COIREALL: CardRecord = CardRecord::new(
    cards::TOWER_OF_COIREALL,
    "Tower of Coireall",
    CardArt::new("64c19977-ac7d-4ce7-925c-33a7503420f5", "Dan Frazier"),
    CardSet::TheDark,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[AbilityDef::not_implemented(
        "{T}: Target creature can't be blocked by Walls this turn.",
        "Temporary blocking restrictions cannot currently be limited to one blocker subtype; the available effect would make the creature completely unblockable.",
    )]),
);

/// The Maze does not remove the creature from combat: it stays an attacker,
/// keeps whatever is blocking it, and simply exchanges no combat damage.
static MAZE_OF_ITH_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Attacking,
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
)];

static MAZE_OF_ITH_EFFECT: [EffectDef; 2] = [
    EffectDef::Untap {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::PreventCombatDamageThisTurn {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
];

// DRK 114 — Wand of Ith
// Audit: blocked — Needs seeded random selection with replay-visible provenance for “{3}, {T}: Target player reveals a card at random from their hand. If it's a land card, that player discards it unless they pay 1 life. If it isn't a land card, the player discards it…”.

// DRK 115 — War Barge
// Audit: blocked — Needs a duration-scoped prohibition on creating or applying regeneration shields for “{3}: Target creature gains islandwalk until end of turn. When this artifact leaves the battlefield this turn, destroy that creature. A creature destroyed this way can't be regenerated”.

// DRK 116 — City of Shadows
// Audit: blocked — Needs the mana-ability runtime to pay this ability's mana activation cost for “{T}: Add {C} for each storage counter on this land”.

// DRK 117 — Maze of Ith
pub(in crate::card::sets) static MAZE_OF_ITH: CardRecord = CardRecord::new(
    cards::MAZE_OF_ITH,
    "Maze of Ith",
    CardArt::new("42dcceee-2a47-4eaa-a6a3-2931b3d50244", "Anson Maddocks"),
    CardSet::TheDark,
    CardRules::new_land(&[]).with_abilities(&[AbilityDef::activated_with_targets(
        "{T}: Untap target attacking creature. Prevent all combat damage that would be dealt to and dealt by that creature this turn.",
        &[AbilityCostDef::TapSource],
        &MAZE_OF_ITH_TARGET,
        EffectDef::Sequence(&MAZE_OF_ITH_EFFECT),
    )]),
);

// DRK 118 — Safe Haven
pub(in crate::card::sets) static SAFE_HAVEN: CardRecord = CardRecord::new(
    cards::SAFE_HAVEN,
    "Safe Haven",
    CardArt::new("0d48fb47-1bed-4791-a014-504515f3d36f", "Christopher Rush"),
    CardSet::TheDark,
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{2}, {T}: Exile target creature you control.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::ExileLinkedToSource {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
        AbilityDef::triggered(
            "At the beginning of your upkeep, you may sacrifice this land. If you do, return each card exiled with this land to the battlefield under its owner's control.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::SacrificeOfChoice {
                player: EffectRecipientDef::Controller,
                object: ObjectPredicateDef::Source,
                then: Some(&EffectDef::ReturnLinkedExiles {
                    zone: ZoneKind::Battlefield,
                    grant: None,
                }),
                optional: true,
            },
        ),
    ]),
);

// DRK 119 — Sorrow's Path
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “{T}: Choose two target blocking creatures controlled by the same opponent. If each of those creatures could block all creatures that the other is blocking, remove both of them from…”.

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &DUST_TO_DUST,
    &EXORCIST,
    &HOLY_LIGHT,
    &MORALE,
    &SQUIRE,
    &TIVADARS_CRUSADE,
    &WITCH_HUNTER,
    &ELECTRIC_EEL,
    &FLOOD,
    &RIPTIDE,
    &SUNKEN_CITY,
    &WATER_WURM,
    &ASHES_TO_ASHES,
    &BOG_IMP,
    &BOG_RATS,
    &GRAVE_ROBBERS,
    &MARSH_GAS,
    &UNCLE_ISTVAN,
    &BALL_LIGHTNING,
    &BLOOD_MOON,
    &BROTHERS_OF_FIRE,
    &CAVE_PEOPLE,
    &FISSURE,
    &GOBLIN_DIGGING_TEAM,
    &GOBLIN_HERO,
    &GOBLINS_OF_THE_FLARG,
    &INFERNO,
    &ORC_GENERAL,
    &SISTERS_OF_THE_FLAME,
    &CARNIVOROUS_PLANT,
    &HIDDEN_PATH,
    &LAND_LEECHES,
    &SCARWOOD_HAG,
    &SCAVENGER_FOLK,
    &TRACKER,
    &SCARWOOD_GOBLINS,
    &DARK_HEART_OF_THE_WOOD,
    &BONE_FLUTE,
    &BOOK_OF_RASS,
    &FELLWAR_STONE,
    &FOUNTAIN_OF_YOUTH,
    &SKULL_OF_ORM,
    &TORMODS_CRYPT,
    &TOWER_OF_COIREALL,
    &MAZE_OF_ITH,
    &SAFE_HAVEN,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
