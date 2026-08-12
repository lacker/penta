//! Dark Ascension card records used by the built-in ISD–RTR Standard deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    AppliedEffectDef, CardArt, CardComposition, CardEffectStatus, CardPart, CardRules, CardSet,
    CardStructure, CardSupertype, CardType, ComparisonDef, ConditionalValueDef, CounterKind,
    DiscardSelectionDef, DoubleFacedKind, EffectDef, EffectDurationDef, EffectRecipientDef,
    ManaColor, ObjectPredicateDef, ObjectQueryDef, PlayOptionDef, PlayerRelation, QuantifierDef,
    SpellForm, TopCardSelectionDef, TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef,
    ZoneKind, ZonePlacement, abilities, cards,
};
use crate::ids::{CardPartId, PlayOptionId, TargetIndex};
use crate::mana_cost;

// DKA 1 — Archangel's Light
// Audit: blocked — Needs a single life-gain amount equal to twice a graveyard-card count; sequencing two gains would incorrectly create two life-gain events.

// DKA 2 — Bar the Door
pub(in crate::card::sets) static BAR_THE_DOOR: CardRecord = CardRecord::new(
    cards::BAR_THE_DOOR,
    "Bar the Door",
    CardArt::new("b593f544-2d82-4237-b9a9-88503b5036cc", "Ryan Pancoast"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{2}{W}")).with_ability(AbilityDef::spell(
        "Creatures you control get +0/+4 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::You,
            },
            effect: AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Constant(0),
                toughness: ValueDef::Constant(4),
            },
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// DKA 3 — Break of Day
// Audit: blocked — Needs a controller-life threshold condition to grant indestructible only during fateful hour.

static BURDEN_OF_GUILT_TAP: AbilityDef = AbilityDef::activated(
    "{1}: Tap enchanted creature.",
    &[AbilityCostDef::Mana(mana_cost!("{1}"))],
    EffectDef::Tap {
        object: EffectRecipientDef::AttachedPermanent,
    },
);

// DKA 4 — Burden of Guilt
pub(in crate::card::sets) static BURDEN_OF_GUILT: CardRecord = CardRecord::new(
    cards::BURDEN_OF_GUILT,
    "Burden of Guilt",
    CardArt::new("d7440288-6c55-4502-bf20-3c5b50a2de5a", "John Stanko"),
    CardSet::DarkAscension,
    CardRules::new_enchantment(mana_cost!("{W}"))
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
            BURDEN_OF_GUILT_TAP,
        ]),
);

// DKA 5 — Curse of Exhaustion
// Audit: blocked — Needs Auras that target and attach to players, plus a per-turn spell-casting limit for the enchanted player.

// DKA 6 — Elgaud Inquisitor
pub(in crate::card::sets) static ELGAUD_INQUISITOR: CardRecord = CardRecord::new(
    cards::ELGAUD_INQUISITOR,
    "Elgaud Inquisitor",
    CardArt::new("c342e1da-7ab9-4e29-96e6-77d820a45ede", "Slawomir Maniak"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Cleric"], 2, 2).with_abilities(&[
        abilities::lifelink(),
        AbilityDef::triggered(
            "When this creature dies, create a 1/1 white Spirit creature token with flying.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: Some(ZoneKind::Battlefield),
                to: Some(ZoneKind::Graveyard),
            },
            EffectDef::CreateToken {
                token: cards::SPIRIT_TOKEN_1_1_WHITE,
                count: ValueDef::Constant(1),
            },
        ),
    ]),
);

// DKA 7 — Faith's Shield
// Audit: blocked — Needs a recorded color choice, temporary protection from that choice, and the fateful-hour controller-life branch.

// DKA 8 — Gather the Townsfolk
// Audit: blocked — Needs a controller-life threshold to choose two versus five Human tokens.

// DKA 9 — Gavony Ironwright
// Audit: blocked — Needs a controller-life threshold continuous condition for the fateful-hour anthem.

// DKA 10 — Hollowhenge Spirit
// Audit: blocked — Needs an effect that removes a chosen attacking or blocking creature from combat.

// DKA 11 — Increasing Devotion
// Audit: blocked — Needs a cast-from-graveyard condition to choose five versus ten Human tokens.

// DKA 12 — Lingering Souls
pub(in crate::card::sets) static LINGERING_SOULS: CardRecord = CardRecord::new(
    cards::LINGERING_SOULS,
    "Lingering Souls",
    CardArt::new("891a92d7-9ccf-4de1-8286-aa5254f27ba9", "Bud Cook"),
    CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{2}{W}")).with_abilities(&[
        AbilityDef::spell(
            "Create two 1/1 white Spirit creature tokens with flying.",
            EffectDef::CreateToken {
                token: cards::SPIRIT_TOKEN_1_1_WHITE,
                count: ValueDef::Constant(2),
            },
        ),
        abilities::flashback(mana_cost!("{1}{B}")),
    ]),
);

// DKA 13 — Loyal Cathar
// Audit: blocked — Needs a delayed end-step return that brings the dead card back transformed and tapped.

// DKA 14 — Midnight Guard
pub(in crate::card::sets) static MIDNIGHT_GUARD: CardRecord = CardRecord::new(
    cards::MIDNIGHT_GUARD,
    "Midnight Guard",
    CardArt::new("2264b760-c527-470d-bad0-d8baaf543631", "Jason A. Engle"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 2, 3).with_ability(
        AbilityDef::triggered(
            "Whenever another creature enters, untap this creature.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
    ),
);

// DKA 15 — Niblis of the Mist
pub(in crate::card::sets) static NIBLIS_OF_THE_MIST: CardRecord = CardRecord::new(
    cards::NIBLIS_OF_THE_MIST,
    "Niblis of the Mist",
    CardArt::new("08aea6e3-c8a8-4964-b95d-4c639da55de1", "Igor Kieryluk"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Spirit"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "When this creature enters, you may tap target creature.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            },
        ),
    ]),
);

// DKA 16 — Niblis of the Urn
pub(in crate::card::sets) static NIBLIS_OF_THE_URN: CardRecord = CardRecord::new(
    cards::NIBLIS_OF_THE_URN,
    "Niblis of the Urn",
    CardArt::new("11bf2ff7-0f8d-47ea-adfd-af299e793a37", "Igor Kieryluk"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Spirit"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, you may tap target creature.",
            TriggerEventDef::Attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            },
        ),
    ]),
);

// DKA 17 — Ray of Revelation
pub(in crate::card::sets) static RAY_OF_REVELATION: CardRecord = CardRecord::new(
    cards::RAY_OF_REVELATION,
    "Ray of Revelation",
    CardArt::new("d7e2c5a4-cf92-46bd-9033-8036436488cb", "Cliff Childs"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Destroy target enchantment.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Enchantment),
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
        abilities::flashback(mana_cost!("{G}")),
    ]),
);

// DKA 18 — Requiem Angel
pub(in crate::card::sets) static REQUIEM_ANGEL: CardRecord = CardRecord::new(
    cards::REQUIEM_ANGEL,
    "Requiem Angel",
    CardArt::new("5385925d-05ad-4f2e-bd2c-8de6c088ed05", "Eric Deschamps"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{5}{W}"), &["Angel"], 5, 5).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever another non-Spirit creature you control dies, create a 1/1 white Spirit creature token with flying.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Spirit")),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                from: Some(ZoneKind::Battlefield),
                to: Some(ZoneKind::Graveyard),
            },
            EffectDef::CreateToken {
                token: cards::SPIRIT_TOKEN_1_1_WHITE,
                count: ValueDef::Constant(1),
            },
        ),
    ]),
);

// DKA 19 — Sanctuary Cat
pub(in crate::card::sets) static SANCTUARY_CAT: CardRecord = CardRecord::new(
    cards::SANCTUARY_CAT,
    "Sanctuary Cat",
    CardArt::new("96865440-01ad-40f2-90d7-9ecd0b4efecc", "David Palumbo"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{W}"), &["Cat"], 1, 2),
);

// DKA 20 — Séance
// Audit: blocked — Needs temporary copy tokens that add the Spirit subtype and a delayed end-step exile linked to each token.

// DKA 21 — Silverclaw Griffin
pub(in crate::card::sets) static SILVERCLAW_GRIFFIN: CardRecord = CardRecord::new(
    cards::SILVERCLAW_GRIFFIN,
    "Silverclaw Griffin",
    CardArt::new("54528722-a6aa-4567-9cd1-e4a97adec7d0", "Daniel Ljunggren"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Griffin"], 3, 2)
        .with_abilities(&[abilities::flying(), abilities::first_strike()]),
);

// DKA 22 — Skillful Lunge
pub(in crate::card::sets) static SKILLFUL_LUNGE: CardRecord = CardRecord::new(
    cards::SKILLFUL_LUNGE,
    "Skillful Lunge",
    CardArt::new("2a28abc1-3e75-4db4-baa1-b47abdb7453b", "Jason Felix"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +2/+0 and gains first strike until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(2),
                    toughness: ValueDef::Constant(0),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::GrantAbility(&abilities::first_strike()),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )),
);

// DKA 23 — Sudden Disappearance
// Audit: blocked — Needs one delayed trigger to return an arbitrary mass-exiled group at the next end step after the spell source has left the stack.

// DKA 24 — Thalia, Guardian of Thraben
// Audit: blocked — Needs a continuous generic-cost increase for noncreature spells.

// DKA 25 — Thraben Doomsayer
// Audit: blocked — Needs a controller-life threshold continuous anthem for its otherwise declarative Human-token ability.

// DKA 26 — Thraben Heretic
pub(in crate::card::sets) static THRABEN_HERETIC: CardRecord = CardRecord::new(
    cards::THRABEN_HERETIC,
    "Thraben Heretic",
    CardArt::new("f8cc36df-040b-4f29-bcc1-f5600803f71d", "James Ryman"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Wizard"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Exile target creature card from a graveyard.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
                controller: None,
            },
        ),
    ),
);

// DKA 27 — Artful Dodge
pub(in crate::card::sets) static ARTFUL_DODGE: CardRecord = CardRecord::new(
    cards::ARTFUL_DODGE,
    "Artful Dodge",
    CardArt::new("de6ce6aa-e19f-4299-9807-e68920e63c73", "Tomasz Jedruszek"),
    CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature can't be blocked this turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::MakeUnblockableThisTurn {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
        abilities::flashback(mana_cost!("{U}")),
    ]),
);

// DKA 28 — Beguiler of Wills
// Audit: blocked — Needs a target-power predicate bounded by the dynamic number of creatures you control, plus permanent control change.

// DKA 29 — Bone to Ash
pub(in crate::card::sets) static BONE_TO_ASH: CardRecord = CardRecord::new(
    cards::BONE_TO_ASH,
    "Bone to Ash",
    CardArt::new("c4a75cef-9551-45e2-b1ff-80662c76ec20", "Clint Cearley"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{2}{U}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target creature spell.\nDraw a card.",
        &[AbilityTargetDef::exactly_one_spell(
            ObjectPredicateDef::HasType(CardType::Creature),
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

// DKA 30 — Call to the Kindred
// Audit: blocked — Needs a top-five selection predicate that dynamically shares a creature type with the enchanted creature, plus arbitrary bottom ordering.

// DKA 31 — Chant of the Skifsang
pub(in crate::card::sets) static CHANT_OF_THE_SKIFSANG: CardRecord = CardRecord::new(
    cards::CHANT_OF_THE_SKIFSANG,
    "Chant of the Skifsang",
    CardArt::new("6e604b2e-f257-465d-9342-6eb55b2334c5", "Nils Hamm"),
    CardSet::DarkAscension,
    CardRules::new_enchantment(mana_cost!("{2}{U}"))
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
                "Enchanted creature gets -13/-0.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(-13),
                        toughness: ValueDef::Constant(0),
                    },
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// DKA 32 — Chill of Foreboding
pub(in crate::card::sets) static CHILL_OF_FOREBODING: CardRecord = CardRecord::new(
    cards::CHILL_OF_FOREBODING,
    "Chill of Foreboding",
    CardArt::new("0abd6534-92bb-44e3-88c2-6709f1a4f29c", "Terese Nielsen"),
    CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{2}{U}")).with_abilities(&[
        AbilityDef::spell(
            "Each player mills five cards.",
            EffectDef::Mill {
                player: EffectRecipientDef::EachPlayer,
                amount: ValueDef::Constant(5),
            },
        ),
        abilities::flashback(mana_cost!("{7}{U}")),
    ]),
);

// DKA 33 — Counterlash
// Audit: blocked — Needs a post-counter optional cast from hand without paying mana, filtered by a card type shared with the countered spell.

// DKA 34 — Curse of Echoes
// Audit: blocked — Needs player-attached Auras and optional spell copies with independently reselectable targets for every other player.

// DKA 35 — Divination
pub(in crate::card::sets) static DIVINATION: CardRecord = CardRecord::new(
    cards::DIVINATION,
    "Divination",
    CardArt::new("4a1340f1-85a4-4551-9871-bb00db6d97a8", "Scott Chou"),
    CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell(
        "Draw two cards.",
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(2),
        },
    )),
);

// DKA 36 — Dungeon Geists
// Audit: partial — A does-not-untap effect lasting until the source leaves is outside the shared applied-effect runtime boundary for another permanent.
pub(in crate::card::sets) static DUNGEON_GEISTS: CardRecord = CardRecord::new(
    cards::DUNGEON_GEISTS,
    "Dungeon Geists",
    CardArt::new("b715da2e-c816-4c14-8522-811c97c66fed", "Nils Hamm"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Spirit"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "When this creature enters, tap target creature an opponent controls. That creature doesn't untap during its controller's untap step for as long as you control this creature.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
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
        )
        .with_coverage(AbilityCoverageDef::partial(
            "The target is tapped, but applying a does-not-untap effect to it until this source leaves is not supported by the shared runtime.",
        )),
    ]),
);

// DKA 37 — Geralf's Mindcrusher
pub(in crate::card::sets) static GERALFS_MINDCRUSHER: CardRecord = CardRecord::new(
    cards::GERALFS_MINDCRUSHER,
    "Geralf's Mindcrusher",
    CardArt::new("68ac8b5f-4d95-43fc-bf23-10247986a746", "Steven Belledin"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{4}{U}{U}"), &["Zombie", "Horror"], 5, 5).with_abilities(
        &[
            AbilityDef::triggered_with_targets(
                "When this creature enters, target player mills five cards.",
                TriggerEventDef::ZoneChanged {
                    object: ObjectPredicateDef::Source,
                    from: None,
                    to: Some(ZoneKind::Battlefield),
                },
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::Mill {
                    player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(5),
                },
            ),
            abilities::undying(),
        ],
    ),
);

// DKA 38 — Griptide
pub(in crate::card::sets) static GRIPTIDE: CardRecord = CardRecord::new(
    cards::GRIPTIDE,
    "Griptide",
    CardArt::new("27f92b74-86bb-4bb3-8f78-640984698f28", "Igor Kieryluk"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{3}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Put target creature on top of its owner's library.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Library,
            placement: ZonePlacement::Top,
            controller: None,
        },
    )),
);

// DKA 39 — Havengul Runebinder
pub(in crate::card::sets) static HAVENGUL_RUNEBINDER: CardRecord = CardRecord::new(
    cards::HAVENGUL_RUNEBINDER,
    "Havengul Runebinder",
    CardArt::new("de766c12-eb2c-466a-8630-8242a153eb1f", "Bud Cook"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Human", "Wizard"], 2, 2).with_ability(
        AbilityDef::activated(
            "{2}{U}, {T}, Exile a creature card from your graveyard: Create a 2/2 black Zombie creature token, then put a +1/+1 counter on each Zombie creature you control.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{U}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::ExileCardFromGraveyard(ObjectPredicateDef::HasType(
                    CardType::Creature,
                )),
            ],
            EffectDef::Sequence(&[
                EffectDef::CreateToken {
                    token: cards::ZOMBIE_TOKEN_2_2_BLACK,
                    count: ValueDef::Constant(1),
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::MatchingObjects {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype("Zombie"),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: PlayerRelation::You,
                    },
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ),
);

// DKA 40 — Headless Skaab
// Audit: blocked — Needs a nonmana additional casting cost that selects and exiles a creature card from your graveyard.

// DKA 41 — Increasing Confusion
// Audit: blocked — Needs a cast-from-graveyard condition that doubles the chosen X mill amount without producing a second mill event.

// DKA 42 — Mystic Retrieval
pub(in crate::card::sets) static MYSTIC_RETRIEVAL: CardRecord = CardRecord::new(
    cards::MYSTIC_RETRIEVAL,
    "Mystic Retrieval",
    CardArt::new("281a685a-bd02-43bf-8700-2207c65bbbb1", "Scott Chou"),
    CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{3}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Return target instant or sorcery card from your graveyard to your hand.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
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
        ),
        abilities::flashback(mana_cost!("{2}{R}")),
    ]),
);

// DKA 43 — Nephalia Seakite
pub(in crate::card::sets) static NEPHALIA_SEAKITE: CardRecord = CardRecord::new(
    cards::NEPHALIA_SEAKITE,
    "Nephalia Seakite",
    CardArt::new("174a1d08-cd79-43d6-897f-3ee9a682d15e", "Wayne England"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Bird"], 2, 3)
        .with_abilities(&[abilities::flash(), abilities::flying()]),
);

// DKA 44 — Niblis of the Breath
// Audit: blocked — Needs a modal activated ability so tap versus untap is one activation choice rather than two separately identified abilities.

// DKA 45 — Relentless Skaabs
// Audit: blocked — Needs a nonmana additional casting cost that selects and exiles a creature card from your graveyard.

// DKA 46 — Saving Grasp
pub(in crate::card::sets) static SAVING_GRASP: CardRecord = CardRecord::new(
    cards::SAVING_GRASP,
    "Saving Grasp",
    CardArt::new("914837df-c255-4cea-9255-b05f218fd9f8", "Matt Stewart"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Return target creature you own to your hand.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
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
        ),
        abilities::flashback(mana_cost!("{W}")),
    ]),
);

// DKA 47 — Screeching Skaab
pub(in crate::card::sets) static SCREECHING_SKAAB: CardRecord = CardRecord::new(
    cards::SCREECHING_SKAAB,
    "Screeching Skaab",
    CardArt::new("3c40a2c7-df7a-41a6-a49e-5f7db808b810", "Clint Cearley"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Zombie"], 2, 1).with_ability(
        AbilityDef::triggered(
            "When this creature enters, mill two cards.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::Mill {
                player: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

// DKA 48 — Secrets of the Dead
// Audit: blocked — Needs SpellCast trigger capture to distinguish a spell cast from its controller's graveyard.

// DKA 49 — Shriekgeist
pub(in crate::card::sets) static SHRIEKGEIST: CardRecord = CardRecord::new(
    cards::SHRIEKGEIST,
    "Shriekgeist",
    CardArt::new("435c5218-46b3-456a-aedf-d9586a4bd0a3", "Raymond Swanland"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Spirit"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, that player mills two cards.",
            TriggerEventDef::CombatDamageDealtToPlayer {
                source: ObjectPredicateDef::Source,
            },
            EffectDef::Mill {
                player: EffectRecipientDef::EventPlayer,
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// DKA 50 — Soul Seizer
// Audit: blocked — Needs transforming a creature into an Aura, attaching the transformed permanent to the damaged player, and granting permanent control of that player's creature.

// DKA 51 — Stormbound Geist
// Audit: blocked — Needs a combat declaration restriction that permits this creature to block only creatures with flying.

// DKA 52 — Thought Scour
pub(in crate::card::sets) static THOUGHT_SCOUR: CardRecord = CardRecord::new(
    cards::THOUGHT_SCOUR,
    "Thought Scour",
    CardArt::new("88bf1ebb-9d85-4b9b-a614-c7f965c0893d", "David Rapoza"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target player mills two cards.\nDraw a card.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// DKA 53 — Tower Geist
pub(in crate::card::sets) static TOWER_GEIST: CardRecord = CardRecord::new(
    cards::TOWER_GEIST,
    "Tower Geist",
    CardArt::new("c9e9f552-34b6-43a5-8ef8-9d5208f4cae0", "Izzy"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Spirit"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "When this creature enters, look at the top two cards of your library. Put one of them into your hand and the other into your graveyard.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::LookAtTopAndSelect {
                player: EffectRecipientDef::Controller,
                selection: &TopCardSelectionDef {
                    count: ValueDef::Constant(2),
                    minimum: 1,
                    maximum: 1,
                    selected_zone: ZoneKind::Hand,
                    selected_placement: ZonePlacement::Top,
                    rest_zone: ZoneKind::Graveyard,
                    rest_placement: ZonePlacement::Top,
                    then: None,
                },
            },
        ),
    ]),
);

// DKA 54 — Black Cat
pub(in crate::card::sets) static BLACK_CAT: CardRecord = CardRecord::new(
    cards::BLACK_CAT,
    "Black Cat",
    CardArt::new("bb1c6379-69d5-48aa-8d06-257c0592794e", "David Palumbo"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Zombie", "Cat"], 1, 1).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature dies, target opponent discards a card at random.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: Some(ZoneKind::Battlefield),
                to: Some(ZoneKind::Graveyard),
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::Discard {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::Random,
            },
        ),
    ),
);

// DKA 55 — Chosen of Markov
// Audit: blocked — Needs an activated cost that taps a different untapped Vampire you control before transforming this permanent.

// DKA 56 — Curse of Misfortunes
// Audit: blocked — Needs player-attached Auras and a library search predicate that excludes Curse names already attached to that player.

// DKA 57 — Curse of Thirst
// Audit: blocked — Needs player-attached Auras and a count of Curses attached to the enchanted player.

// DKA 58 — Deadly Allure
// Audit: blocked — Needs a temporary combat requirement that makes the target creature be blocked this turn if able.

// DKA 59 — Death's Caress
// Audit: blocked — Needs the destroyed target's last-known toughness as a value, gated on that target having been a Human.

// DKA 60 — Falkenrath Torturer
// Audit: blocked — Needs an activated sacrifice cost to expose whether the chosen creature was Human so the conditional counter can follow the flying grant.

// DKA 61 — Farbog Boneflinger
pub(in crate::card::sets) static FARBOG_BONEFLINGER: CardRecord = CardRecord::new(
    cards::FARBOG_BONEFLINGER,
    "Farbog Boneflinger",
    CardArt::new("98d45316-b44a-4cf6-8cbe-b02fe6545141", "Tomasz Jedruszek"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Zombie"], 2, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature enters, target creature gets -2/-2 until end of turn.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(-2),
                    toughness: ValueDef::Constant(-2),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// DKA 62 — Fiend of the Shadows
// Audit: blocked — Needs play permission for a card exiled from an opponent's hand and a sacrifice-regeneration procedure that preserves source identity.

// DKA 63 — Geralf's Messenger
pub(in crate::card::sets) static GERALFS_MESSENGER: CardRecord = CardRecord::new(
    cards::GERALFS_MESSENGER,
    "Geralf's Messenger",
    CardArt::new("bffaad78-97ff-431f-bfb0-e96c7558f974", "Kev Walker"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{B}{B}{B}"), &["Zombie"], 3, 2).with_abilities(&[
        abilities::enters_tapped("This creature enters tapped."),
        AbilityDef::triggered_with_targets(
            "When this creature enters, target opponent loses 2 life.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ),
        abilities::undying(),
    ]),
);

// DKA 64 — Gravecrawler
// Audit: blocked — Needs conditional graveyard casting permission tied to controlling a Zombie and a static prohibition on blocking.

// DKA 65 — Gravepurge
// Audit: blocked — Needs an unbounded “any number” target group; the fixed-cardinality target representation cannot express every legal graveyard size.

// DKA 66 — Gruesome Discovery
// Audit: blocked — Needs the morbid replacement branch to reveal a hand and let the spell's controller choose two cards from it.

// DKA 67 — Harrowing Journey
pub(in crate::card::sets) static HARROWING_JOURNEY: CardRecord = CardRecord::new(
    cards::HARROWING_JOURNEY,
    "Harrowing Journey",
    CardArt::new("9cf96a6c-8481-4954-b149-7153b80480be", "James Paick"),
    CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{4}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target player draws three cards and loses 3 life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
        ]),
    )),
);

// DKA 68 — Highborn Ghoul
pub(in crate::card::sets) static HIGHBORN_GHOUL: CardRecord = CardRecord::new(
    cards::HIGHBORN_GHOUL,
    "Highborn Ghoul",
    CardArt::new("fbe999ed-b419-440c-9189-1046f43d7b87", "Volkan Baǵa"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{B}{B}"), &["Zombie"], 2, 1)
        .with_ability(abilities::intimidate()),
);

// DKA 69 — Increasing Ambition
// Audit: blocked — Needs a cast-from-graveyard condition to select two unrestricted library cards instead of one.

// DKA 70 — Mikaeus, the Unhallowed
// Audit: blocked — Needs a damage trigger keyed to Human sources plus a continuous effect that both excludes Humans and grants undying to other creatures.

// DKA 71 — Ravenous Demon
// Audit: blocked — Needs a transformed-face upkeep procedure that offers a Human sacrifice and otherwise transforms the source and makes its controller lose 9 life.

// DKA 72 — Reap the Seagraf
pub(in crate::card::sets) static REAP_THE_SEAGRAF: CardRecord = CardRecord::new(
    cards::REAP_THE_SEAGRAF,
    "Reap the Seagraf",
    CardArt::new("4defdead-19fa-4535-9f71-8808388b0332", "James Ryman"),
    CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{2}{B}")).with_abilities(&[
        AbilityDef::spell(
            "Create a 2/2 black Zombie creature token.",
            EffectDef::CreateToken {
                token: cards::ZOMBIE_TOKEN_2_2_BLACK,
                count: ValueDef::Constant(1),
            },
        ),
        abilities::flashback(mana_cost!("{4}{U}")),
    ]),
);

// DKA 73 — Sightless Ghoul
// Audit: blocked — Needs a static combat declaration prohibition for “This creature can't block.”

// DKA 74 — Skirsdag Flayer
pub(in crate::card::sets) static SKIRSDAG_FLAYER: CardRecord = CardRecord::new(
    cards::SKIRSDAG_FLAYER,
    "Skirsdag Flayer",
    CardArt::new("274976b0-2bb5-46e6-b62e-b50d80a77e28", "Austin Hsu"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Cleric"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{3}{B}, {T}, Sacrifice a Human: Destroy target creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{3}{B}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Human"),
                    ]),
                    controller: PlayerRelation::You,
                },
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
        ),
    ),
);

// DKA 75 — Spiteful Shadows
// Audit: blocked — Needs damage dealt to the attached creature to be re-dealt with that creature, rather than the Aura, as the damage source.

/// Morbid replaces the amount rather than adding a second effect, so both
/// printed clauses come down to which number this picks.
const TRAGIC_SLIP_AMOUNT: ValueDef = ValueDef::IfCreatureDiedThisTurn(&ConditionalValueDef {
    then: ValueDef::Constant(-13),
    otherwise: ValueDef::Constant(-1),
});

// DKA 76 — Tragic Slip
pub(in crate::card::sets) static TRAGIC_SLIP: CardRecord = CardRecord::new(
    cards::TRAGIC_SLIP,
    "Tragic Slip",
    CardArt::new("09666671-601e-4fca-bdfb-fb288bf2672c", "Christopher Moeller"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{B}")).with_abilities(&[
        AbilityDef::spell_with_targets("Target creature gets -1/-1 until end of turn.", &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )], EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: TRAGIC_SLIP_AMOUNT,
                    toughness: TRAGIC_SLIP_AMOUNT,
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            }),
        AbilityDef::static_ability(
            "Morbid — That creature gets -13/-13 until end of turn instead if a creature died this turn.",
            // The conditional value on the spell clause above already
            // carries this modifier; this clause has no second effect to run.
            EffectDef::None,
        )
        .with_coverage(AbilityCoverageDef::explained_complete(
            "The morbid amount is chosen by the value on the preceding clause.",
        )),
    ]),
);

// DKA 77 — Undying Evil
pub(in crate::card::sets) static UNDYING_EVIL: CardRecord = CardRecord::new(
    cards::UNDYING_EVIL,
    "Undying Evil",
    CardArt::new("325f2243-54fd-484b-a742-166cea7ec179", "Kev Walker"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gains undying until end of turn. (When it dies, if it had no +1/+1 counters on it, return it to the battlefield under its owner's control with a +1/+1 counter on it.)",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::GrantAbility(&abilities::undying()),
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// DKA 78 — Vengeful Vampire
pub(in crate::card::sets) static VENGEFUL_VAMPIRE: CardRecord = CardRecord::new(
    cards::VENGEFUL_VAMPIRE,
    "Vengeful Vampire",
    CardArt::new("d03c64a7-37d2-4d8f-bd7a-9435bc2f4101", "Lucas Graciano"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Vampire"], 3, 2)
        .with_abilities(&[abilities::flying(), abilities::undying()]),
);

// DKA 79 — Wakedancer
// Audit: blocked — Needs an intervening morbid condition so no trigger is created when a creature has not died this turn.

// DKA 80 — Zombie Apocalypse
// Audit: blocked — Needs a mass graveyard return that makes the returned Zombie cards enter the battlefield tapped before destroying all Humans.

// DKA 81 — Afflicted Deserter
// Audit: blocked — Needs a reusable complete double-faced Werewolf composition plus the back face's transform trigger that destroys an artifact and damages its controller.

// DKA 82 — Alpha Brawl
// Audit: blocked — Needs damage sourced by the targeted creature to every other creature its opponent controls, followed by reciprocal damage from each survivor.

// DKA 83 — Blood Feud
pub(in crate::card::sets) static BLOOD_FEUD: CardRecord = CardRecord::new(
    cards::BLOOD_FEUD,
    "Blood Feud",
    CardArt::new("634d59b8-6046-4796-95c5-eec75a239986", "Winona Nelson"),
    CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{4}{R}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Target creature fights another target creature. (Each deals damage equal to its power to the other.)",
            &[
                AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Creature,
                )),
                AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Creature,
                )),
            ],
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
        ),
    ),
);

// DKA 84 — Burning Oil
pub(in crate::card::sets) static BURNING_OIL: CardRecord = CardRecord::new(
    cards::BURNING_OIL,
    "Burning Oil",
    CardArt::new("47773da8-afe4-43e1-8355-6ab51451ee00", "Trevor Claxton"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Burning Oil deals 3 damage to target attacking or blocking creature.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AttackingOrBlocking,
                ]),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
        ),
        abilities::flashback(mana_cost!("{3}{W}")),
    ]),
);

// DKA 85 — Curse of Bloodletting
// Audit: blocked — Needs an Aura attached to a player so its damage-doubling replacement can be scoped to that enchanted player.

// DKA 86 — Erdwal Ripper
pub(in crate::card::sets) static ERDWAL_RIPPER: CardRecord = CardRecord::new(
    cards::ERDWAL_RIPPER,
    "Erdwal Ripper",
    CardArt::new("769ea5e9-6d05-4bc6-8f14-00eb2532c8b5", "Kev Walker"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Vampire"], 2, 1).with_abilities(&[
        abilities::haste(),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, put a +1/+1 counter on it.",
            TriggerEventDef::CombatDamageDealtToPlayer {
                source: ObjectPredicateDef::Source,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// DKA 87 — Faithless Looting
pub(in crate::card::sets) static FAITHLESS_LOOTING: CardRecord = CardRecord::new(
    cards::FAITHLESS_LOOTING,
    "Faithless Looting",
    CardArt::new("a1b0da17-d595-441d-811c-a2d28d2bb232", "Gabor Szikszai"),
    CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{R}")).with_abilities(&[
        AbilityDef::spell(
            "Draw two cards, then discard two cards.",
            EffectDef::Sequence(&[
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                    selection: DiscardSelectionDef::RecipientChooses,
                },
            ]),
        ),
        abilities::flashback(mana_cost!("{2}{R}")),
    ]),
);

// DKA 88 — Fires of Undeath
pub(in crate::card::sets) static FIRES_OF_UNDEATH: CardRecord = CardRecord::new(
    cards::FIRES_OF_UNDEATH,
    "Fires of Undeath",
    CardArt::new("6d94aaa4-c2fd-4714-9198-8415158b9c4d", "Jason Chan"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{2}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Fires of Undeath deals 2 damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ),
        abilities::flashback(mana_cost!("{5}{B}")),
    ]),
);

// DKA 89 — Flayer of the Hatebound
// Audit: blocked — Needs the triggering creature's power as a value and that creature, rather than this permanent, as the source of the damage.

// DKA 90 — Fling
// Audit: blocked — Needs a nonmana additional casting sacrifice whose last-known power is retained for the spell's damage amount.

// DKA 91 — Forge Devil
pub(in crate::card::sets) static FORGE_DEVIL: CardRecord = CardRecord::new(
    cards::FORGE_DEVIL,
    "Forge Devil",
    CardArt::new("63b565a5-d706-47b4-bfa2-deebcc0e2e60", "Austin Hsu"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{R}"), &["Devil"], 1, 1).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature enters, it deals 1 damage to target creature and 1 damage to you.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
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
    ),
);

// DKA 92 — Heckling Fiends
pub(in crate::card::sets) static HECKLING_FIENDS: CardRecord = CardRecord::new(
    cards::HECKLING_FIENDS,
    "Heckling Fiends",
    CardArt::new("e9fd8895-9282-44d3-969f-b0529eb3bc07", "Clint Cearley"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Devil"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{2}{R}: Target creature attacks this turn if able.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{R}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::GrantAbility(&abilities::attacks_each_combat_if_able(
                    "This creature attacks each combat if able.",
                )),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// DKA 93 — Hellrider
pub(in crate::card::sets) static HELLRIDER: CardRecord = CardRecord::new(
    cards::HELLRIDER,
    "Hellrider",
    CardArt::new("0ec8d800-7f06-44e0-b22d-cdff0a9b153d", "Svetlin Velinov"),
    CardSet::DarkAscension,
    CardRules::new_creature(
        mana_cost!("{2}{R}{R}"),
        &["Devil"],
        3,
        3,
    )
    .with_abilities(&[
        abilities::haste(),
        AbilityDef::triggered(
            "Whenever a creature you control attacks, this creature deals 1 damage to the player or planeswalker it's attacking.",
            TriggerEventDef::Attacks(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::DealDamage {
                // With no planeswalkers in the game, the player an attacker is
                // attacking is always the defending player.
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// DKA 94 — Hinterland Hermit
// Audit: blocked — Needs a complete transforming Werewolf composition plus the back face's restriction that it can't be blocked by more than one creature.

// DKA 95 — Increasing Vengeance
// Audit: blocked — Needs spell copies with new targets and a cast-from-graveyard branch that creates two copies instead of one.

// DKA 96 — Markov Blademaster
pub(in crate::card::sets) static MARKOV_BLADEMASTER: CardRecord = CardRecord::new(
    cards::MARKOV_BLADEMASTER,
    "Markov Blademaster",
    CardArt::new(
        "122163dd-e070-48af-8036-e9850541d138",
        "Jana Schirmer & Johannes Voss",
    ),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Vampire", "Warrior"], 1, 1)
        .with_abilities(&[
            abilities::double_strike(),
            AbilityDef::triggered(
                "Whenever this creature deals combat damage to a player, put a +1/+1 counter on it.",
                TriggerEventDef::CombatDamageDealtToPlayer {
                    source: ObjectPredicateDef::Source,
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// DKA 97 — Markov Warlord
// Audit: blocked — Needs a temporary “can't block” combat declaration restriction for up to two targets.

// DKA 98 — Mondronen Shaman
// Audit: blocked — Needs a complete transforming Werewolf composition plus a back-face trigger that damages an opponent whenever they cast a spell.

// DKA 99 — Moonveil Dragon
pub(in crate::card::sets) static MOONVEIL_DRAGON: CardRecord = CardRecord::new(
    cards::MOONVEIL_DRAGON,
    "Moonveil Dragon",
    CardArt::new("92503118-b37b-4c52-b40a-487f6ad695ef", "Todd Lockwood"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{3}{R}{R}{R}"), &["Dragon"], 5, 5).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{R}: Each creature you control gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{R}"))],
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
    ]),
);

// DKA 100 — Nearheath Stalker
pub(in crate::card::sets) static NEARHEATH_STALKER: CardRecord = CardRecord::new(
    cards::NEARHEATH_STALKER,
    "Nearheath Stalker",
    CardArt::new("7d4cdf4a-2d55-4769-8c51-bc86c13000ef", "Michael C. Hayes"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Vampire", "Rogue"], 4, 1)
        .with_ability(abilities::undying()),
);

// DKA 101 — Pyreheart Wolf
// Audit: blocked — Needs the menace blocking restriction and a temporary mass grant of it to creatures you control.

// DKA 102 — Russet Wolves
pub(in crate::card::sets) static RUSSET_WOLVES: CardRecord = CardRecord::new(
    cards::RUSSET_WOLVES,
    "Russet Wolves",
    CardArt::new(
        "b3c7c972-5a11-4709-b3ef-e2acb3b51dd9",
        "Christopher Moeller",
    ),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Wolf"], 3, 3),
);

// DKA 103 — Scorch the Fields
pub(in crate::card::sets) static SCORCH_THE_FIELDS: CardRecord = CardRecord::new(
    cards::SCORCH_THE_FIELDS,
    "Scorch the Fields",
    CardArt::new("05c4338d-e5c0-46b4-ab16-1f9aa97b4026", "Jaime Jones"),
    CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{4}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target land. Scorch the Fields deals 1 damage to each Human creature.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Land),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Human"),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::Any,
                },
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// DKA 104 — Shattered Perception
// Audit: blocked — Needs the pre-discard hand size captured for the later draw after the hand has become empty.

static TALONS_OF_FALKENRATH_PUMP: AbilityDef = AbilityDef::activated(
    "{1}{R}: This creature gets +2/+0 until end of turn.",
    &[AbilityCostDef::Mana(mana_cost!("{1}{R}"))],
    EffectDef::Apply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::ModifyPowerToughness {
            power: ValueDef::Constant(2),
            toughness: ValueDef::Constant(0),
        },
        duration: EffectDurationDef::UntilEndOfTurn,
    },
);

// DKA 105 — Talons of Falkenrath
pub(in crate::card::sets) static TALONS_OF_FALKENRATH: CardRecord = CardRecord::new(
    cards::TALONS_OF_FALKENRATH,
    "Talons of Falkenrath",
    CardArt::new("f8e38239-a9ec-4149-9c90-74dcd46ed95d", "Svetlin Velinov"),
    CardSet::DarkAscension,
    CardRules::new_enchantment(mana_cost!("{1}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::flash(),
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
                "Enchanted creature has \"{1}{R}: This creature gets +2/+0 until end of turn.\"",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::GrantAbility(&TALONS_OF_FALKENRATH_PUMP),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// DKA 106 — Torch Fiend
pub(in crate::card::sets) static TORCH_FIEND: CardRecord = CardRecord::new(
    cards::TORCH_FIEND,
    "Torch Fiend",
    CardArt::new("d596feee-6ccc-4648-884b-ed2eeb1cffc0", "Winona Nelson"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Devil"], 2, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{R}, Sacrifice this creature: Destroy target artifact.",
            &[
                AbilityCostDef::Mana(mana_cost!("{R}")),
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
    ),
);

// DKA 107 — Wrack with Madness
// Audit: blocked — Needs damage whose source is the targeted creature itself, including that source's damage abilities and prevention relations.

// DKA 108 — Briarpack Alpha
pub(in crate::card::sets) static BRIARPACK_ALPHA: CardRecord = CardRecord::new(
    cards::BRIARPACK_ALPHA,
    "Briarpack Alpha",
    CardArt::new("a052e945-7535-4b0a-b580-cf76377633f3", "Daarken"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Wolf"], 3, 3).with_abilities(&[
        abilities::flash(),
        AbilityDef::triggered_with_targets(
            "When this creature enters, target creature gets +2/+2 until end of turn.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
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

// DKA 109 — Clinging Mists
// Audit: blocked — Needs a controller-life threshold branch and a delayed next-untap restriction scoped to the attacking creatures tapped by that branch.

static CRUSHING_VINES_MODES: [AbilityDef; 2] = [
    AbilityDef::spell_with_targets(
        "Destroy target creature with flying",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Flying),
            ]),
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            can_regenerate: true,
        },
    )
    .with_coverage(AbilityCoverageDef::partial(
        "The flying target predicate does not yet account for flying granted or removed by static continuous effects.",
    )),
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
];

// DKA 110 — Crushing Vines
// Audit: partial — The creature mode's flying target predicate misses flying granted or removed by static continuous effects.
pub(in crate::card::sets) static CRUSHING_VINES: CardRecord = CardRecord::new(
    cards::CRUSHING_VINES,
    "Crushing Vines",
    CardArt::new("c59b3653-5a50-48f2-bcf1-ab305ef30902", "Scott Chou"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{2}{G}")).with_ability(AbilityDef::choose_one_spell(
        "Choose one —\n• Destroy target creature with flying.\n• Destroy target artifact.",
        &CRUSHING_VINES_MODES,
    )),
);

// DKA 111 — Dawntreader Elk
// Audit: blocked — SearchZone cannot make the selected basic land enter tapped.

// DKA 112 — Deranged Outcast
pub(in crate::card::sets) static DERANGED_OUTCAST: CardRecord = CardRecord::new(
    cards::DERANGED_OUTCAST,
    "Deranged Outcast",
    CardArt::new("e2b35fee-8e24-4d89-ad77-d55d06bb1d7f", "Nils Hamm"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Rogue"], 2, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{1}{G}, Sacrifice a Human: Put two +1/+1 counters on target creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{G}")),
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Human"),
                    ]),
                    controller: PlayerRelation::You,
                },
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

// DKA 113 — Favor of the Woods
// Audit: blocked — Trigger capture has no event for the attached creature becoming a blocker.

// DKA 114 — Feed the Pack
// Audit: blocked — The optional sacrifice continuation exposes power, not the sacrificed nontoken creature's toughness, for the Wolf-token count.

// DKA 115 — Ghoultree
pub(in crate::card::sets) static GHOULTREE: CardRecord = CardRecord::new(
    cards::GHOULTREE,
    "Ghoultree",
    CardArt::new("a413c65e-5965-429b-8c25-11f8b73cba03", "Volkan Baǵa"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{7}{G}"), &["Zombie", "Treefolk"], 10, 10).with_ability(
        AbilityDef::static_ability(
            "This spell costs {1} less to cast for each creature card in your graveyard.",
            EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(&ObjectQueryDef {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Graveyard],
                controller: PlayerRelation::You,
            })),
        )
        .with_source_zones(&[ZoneKind::Hand]),
    ),
);

// DKA 116 — Gravetiller Wurm
// Audit: blocked — Needs a battlefield-entry replacement condition keyed to whether a creature died this turn.

// DKA 117 — Grim Flowering
pub(in crate::card::sets) static GRIM_FLOWERING: CardRecord = CardRecord::new(
    cards::GRIM_FLOWERING,
    "Grim Flowering",
    CardArt::new("e5f3e2ad-7a04-4735-ba73-576e32249ba3", "Adam Paquette"),
    CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{5}{G}")).with_ability(AbilityDef::spell(
        "Draw a card for each creature card in your graveyard.",
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::CountMatchingObjects(&ObjectQueryDef {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Graveyard],
                controller: PlayerRelation::You,
            }),
        },
    )),
);

// DKA 118 — Hollowhenge Beast
pub(in crate::card::sets) static HOLLOWHENGE_BEAST: CardRecord = CardRecord::new(
    cards::HOLLOWHENGE_BEAST,
    "Hollowhenge Beast",
    CardArt::new("052ab91f-ac01-43f4-9276-9af35dbfbf71", "Dave Kendall"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Beast"], 5, 5),
);

const HUNGER_OF_THE_HOWLPACK_AMOUNT: ValueDef =
    ValueDef::IfCreatureDiedThisTurn(&ConditionalValueDef {
        then: ValueDef::Constant(3),
        otherwise: ValueDef::Constant(1),
    });

// DKA 119 — Hunger of the Howlpack
pub(in crate::card::sets) static HUNGER_OF_THE_HOWLPACK: CardRecord = CardRecord::new(
    cards::HUNGER_OF_THE_HOWLPACK,
    "Hunger of the Howlpack",
    CardArt::new("b38a0dbc-3ebd-4f87-a5fb-bc2ee8a48a8d", "Nils Hamm"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{G}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Put a +1/+1 counter on target creature.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: HUNGER_OF_THE_HOWLPACK_AMOUNT,
            },
        ),
        AbilityDef::static_ability(
            "Morbid — Put three +1/+1 counters on that creature instead if a creature died this turn.",
            EffectDef::None,
        )
        .with_coverage(AbilityCoverageDef::explained_complete(
            "The morbid replacement amount is selected by the preceding spell clause.",
        )),
    ]),
);

// DKA 120 — Increasing Savagery
// Audit: blocked — Needs a cast-from-graveyard condition to choose five versus ten +1/+1 counters.

// DKA 121 — Kessig Recluse
pub(in crate::card::sets) static KESSIG_RECLUSE: CardRecord = CardRecord::new(
    cards::KESSIG_RECLUSE,
    "Kessig Recluse",
    CardArt::new("695b8abe-796e-4d9b-aad3-4e03e925d2a7", "Vincent Proce"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Spider"], 2, 3)
        .with_abilities(&[abilities::reach(), abilities::deathtouch()]),
);

// DKA 122 — Lambholt Elder
// Audit: blocked — Needs a complete transforming Werewolf composition plus the back face's combat-damage card-draw trigger.

// DKA 123 — Lost in the Woods
// Audit: blocked — Needs a per-attacker top-card reveal, a Forest-card branch that removes that attacker from combat, and bottom placement.

// DKA 124 — Predator Ooze
pub(in crate::card::sets) static PREDATOR_OOZE: CardRecord = CardRecord::new(
    cards::PREDATOR_OOZE,
    "Predator Ooze",
    CardArt::new("73c71457-f7c9-4ab4-b89d-e235e3f15e16", "Ryan Yee"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{G}{G}{G}"), &["Ooze"], 1, 1).with_abilities(&[
        abilities::indestructible(),
        AbilityDef::triggered(
            "Whenever this creature attacks, put a +1/+1 counter on it.",
            TriggerEventDef::Attacks(ObjectPredicateDef::Source),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::triggered(
            "Whenever a creature dealt damage by this creature this turn dies, put a +1/+1 counter on this creature.",
            TriggerEventDef::DamagedCreatureDied,
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// DKA 125 — Scorned Villager
// Audit: blocked — Needs a complete transforming Werewolf composition whose two faces expose different mana abilities while sharing the upkeep transforms.

// DKA 126 — Somberwald Dryad
pub(in crate::card::sets) static SOMBERWALD_DRYAD: CardRecord = CardRecord::new(
    cards::SOMBERWALD_DRYAD,
    "Somberwald Dryad",
    CardArt::new("307edca0-769d-4071-9654-3537341e96bd", "Jaime Jones"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Dryad"], 2, 2)
        .with_ability(abilities::forestwalk()),
);

// DKA 127 — Strangleroot Geist
pub(in crate::card::sets) static STRANGLEROOT_GEIST: CardRecord = CardRecord::new(
    cards::STRANGLEROOT_GEIST,
    "Strangleroot Geist",
    CardArt::new("bf1fb137-205c-480f-b6dc-dfa137793ae3", "Jason Chan"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{G}{G}"), &["Spirit"], 2, 1)
        .with_abilities(&[abilities::haste(), abilities::undying()]),
);

// DKA 128 — Tracker's Instincts
// Audit: blocked — The top-card selection primitive cannot restrict the chosen card to a creature while moving all unchosen cards to the graveyard.

// DKA 129 — Ulvenwald Bear
// Audit: blocked — Needs an intervening morbid condition so the targeted trigger is not created when no creature died this turn.

// DKA 130 — Village Survivors
// Audit: blocked — Needs a controller-life threshold continuous condition for granting vigilance to other creatures.

// DKA 131 — Vorapede
pub(in crate::card::sets) static VORAPEDE: CardRecord = CardRecord::new(
    cards::VORAPEDE,
    "Vorapede",
    CardArt::new("1348aa65-85e7-4ac7-bcdb-a83f2c3aa1a6", "Slawomir Maniak"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{2}{G}{G}{G}"), &["Insect"], 5, 4).with_abilities(&[
        abilities::vigilance(),
        abilities::trample(),
        abilities::undying(),
    ]),
);

// DKA 132 — Wild Hunger
pub(in crate::card::sets) static WILD_HUNGER: CardRecord = CardRecord::new(
    cards::WILD_HUNGER,
    "Wild Hunger",
    CardArt::new("a564e8d4-4111-4d8e-897d-523bc4cfef94", "Karl Kopinski"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{2}{G}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature gets +3/+1 and gains trample until end of turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(3),
                        toughness: ValueDef::Constant(1),
                    },
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::GrantAbility(&abilities::trample()),
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
        abilities::flashback(mana_cost!("{3}{R}")),
    ]),
);

// DKA 133 — Wolfbitten Captive
// Audit: blocked — Needs a once-per-turn activation limit shared with each face's distinct self-pump ability in a transforming Werewolf composition.

// DKA 134 — Young Wolf
pub(in crate::card::sets) static YOUNG_WOLF: CardRecord = CardRecord::new(
    cards::YOUNG_WOLF,
    "Young Wolf",
    CardArt::new("0c39aa40-ef5f-40f1-a6dd-fbce91172c50", "Ryan Pancoast"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{G}"), &["Wolf"], 1, 1).with_ability(abilities::undying()),
);

// DKA 135 — Diregraf Captain
pub(in crate::card::sets) static DIREGRAF_CAPTAIN: CardRecord = CardRecord::new(
    cards::DIREGRAF_CAPTAIN,
    "Diregraf Captain",
    CardArt::new("0e5f41eb-609b-4882-af9e-904daa717484", "Slawomir Maniak"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{1}{U}{B}"), &["Zombie", "Soldier"], 2, 2).with_abilities(
        &[
            abilities::deathtouch(),
            AbilityDef::static_ability(
                "Other Zombie creatures you control get +1/+1.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::MatchingObjects {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype("Zombie"),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: PlayerRelation::You,
                    },
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(1),
                        toughness: ValueDef::Constant(1),
                    },
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
            AbilityDef::triggered_with_targets(
                "Whenever another Zombie you control dies, target opponent loses 1 life.",
                TriggerEventDef::ZoneChanged {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Zombie"),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    from: Some(ZoneKind::Battlefield),
                    to: Some(ZoneKind::Graveyard),
                },
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
            ),
        ],
    ),
);

// DKA 136 — Drogskol Captain
pub(in crate::card::sets) static DROGSKOL_CAPTAIN: CardRecord = CardRecord::new(
    cards::DROGSKOL_CAPTAIN,
    "Drogskol Captain",
    CardArt::new("b8238e36-625f-460d-9e39-fd501e65490c", "Peter Mohrbacher"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{1}{W}{U}"), &["Spirit", "Soldier"], 2, 2)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::static_ability(
                "Other Spirit creatures you control get +1/+1 and have hexproof. (They can't be the targets of spells or abilities your opponents control.)",
                EffectDef::Sequence(&[
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::MatchingObjects {
                            object: ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Subtype("Spirit"),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            zones: &[ZoneKind::Battlefield],
                            controller: PlayerRelation::You,
                        },
                        effect: AppliedEffectDef::ModifyPowerToughness {
                            power: ValueDef::Constant(1),
                            toughness: ValueDef::Constant(1),
                        },
                        duration: EffectDurationDef::WhileSourceRemainsInZone,
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::MatchingObjects {
                            object: ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Subtype("Spirit"),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            zones: &[ZoneKind::Battlefield],
                            controller: PlayerRelation::You,
                        },
                        effect: AppliedEffectDef::GrantAbility(&abilities::hexproof()),
                        duration: EffectDurationDef::WhileSourceRemainsInZone,
                    },
                ]),
            ),
        ]),
);

// DKA 137 — Drogskol Reaver
pub(in crate::card::sets) static DROGSKOL_REAVER: CardRecord = CardRecord::new(
    cards::DROGSKOL_REAVER,
    "Drogskol Reaver",
    CardArt::new("af2d9e0b-6433-40a2-9847-9fa4e3c008c4", "Vincent Proce"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{5}{W}{U}"), &["Spirit"], 3, 5).with_abilities(&[
        abilities::flying(),
        abilities::double_strike(),
        abilities::lifelink(),
        AbilityDef::triggered(
            "Whenever you gain life, draw a card.",
            TriggerEventDef::LifeGained(PlayerRelation::You),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// DKA 138 — Falkenrath Aristocrat
// Audit: blocked — Needs an activated sacrifice cost to expose whether the chosen creature was Human for the conditional +1/+1 counter.

// DKA 139 — Havengul Lich
// Audit: blocked — Needs temporary graveyard-casting permission for a targeted creature card and a later cast trigger that copies all of that card's activated abilities.

const fn huntmaster_front_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{2}{R}{G}"), &["Human", "Werewolf"], 2, 2)
        .with_abilities(&HUNTMASTER_FRONT_ABILITIES)
}

/// Entering and transforming into this face do the same thing, so the printed
/// sentence is two triggers watching two different events.
static HUNTMASTER_FRONT_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::triggered(
        "Whenever this creature enters, create a 2/2 green Wolf creature token and you gain 2 life.",
        TriggerEventDef::ZoneChanged {
            object: ObjectPredicateDef::Source,
            from: None,
            to: Some(ZoneKind::Battlefield),
        },
        HUNTMASTER_WOLF_AND_LIFE,
    ),
    AbilityDef::triggered(
        "Whenever this creature transforms into Huntmaster of the Fells, create a 2/2 green Wolf creature token and you gain 2 life.",
        TriggerEventDef::TransformsIntoThisFace,
        HUNTMASTER_WOLF_AND_LIFE,
    ),
    AbilityDef::triggered_if(
        "At the beginning of each upkeep, if no spells were cast last turn, transform this creature.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::Any,
        },
        &NO_SPELLS_LAST_TURN,
        EffectDef::Transform {
            object: EffectRecipientDef::Source,
        },
    ),
];

static HUNTMASTER_WOLF_AND_LIFE: EffectDef = EffectDef::Sequence(&[
    EffectDef::CreateToken {
        token: cards::WOLF_TOKEN_2_2_GREEN,
        count: ValueDef::Constant(1),
    },
    EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(2),
    },
]);

/// Nobody cast anything, so every player has to be at zero.
static NO_SPELLS_LAST_TURN: TriggerConditionDef = TriggerConditionDef::SpellsCastLastTurn {
    quantifier: QuantifierDef::Every,
    player: PlayerRelation::Any,
    comparison: ComparisonDef::LessOrEqual,
    amount: 0,
};

/// One player is enough, which is why this side turns back sooner than the
/// other side turns over.
static TWO_SPELLS_LAST_TURN: TriggerConditionDef = TriggerConditionDef::SpellsCastLastTurn {
    quantifier: QuantifierDef::Any,
    player: PlayerRelation::Any,
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 2,
};

static HUNTMASTER_BACK_ABILITIES: [AbilityDef; 3] = [
    abilities::trample(),
    AbilityDef::triggered_with_targets(
        "Whenever this creature transforms into Ravager of the Fells, it deals 2 damage to target opponent or planeswalker and 2 damage to up to one target creature that player or that planeswalker's controller controls.",
        TriggerEventDef::TransformsIntoThisFace,
        &RAVAGER_TARGETS,
        EffectDef::Sequence(&[
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex(1)),
                amount: ValueDef::Constant(2),
            },
        ]),
    ),
    AbilityDef::triggered_if(
        "At the beginning of each upkeep, if a player cast two or more spells last turn, transform this creature.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::Any,
        },
        &TWO_SPELLS_LAST_TURN,
        EffectDef::Transform {
            object: EffectRecipientDef::Source,
        },
    ),
];

/// The second slot reads the first: the creature has to belong to whoever the
/// damage was aimed at.
static RAVAGER_TARGETS: [AbilityTargetDef; 2] = [
    AbilityTargetDef::exactly_one(AbilityTargetPredicate::PlayerOrPlaneswalker(
        PlayerRelation::Opponent,
    )),
    AbilityTargetDef::up_to(
        AbilityTargetPredicate::ControlledByTargetOf {
            object: ObjectPredicateDef::HasType(CardType::Creature),
            slot: TargetIndex::PRIMARY,
        },
        1,
    ),
];

const fn huntmaster_back_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Werewolf"], 4, 4)
        .printed_colors(&[ManaColor::Red, ManaColor::Green])
        .with_abilities(&HUNTMASTER_BACK_ABILITIES)
}

fn huntmaster_composition() -> CardComposition {
    let front = huntmaster_front_rules();
    let back = huntmaster_back_rules();
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Huntmaster of the Fells", front),
            CardPart::new(CardPartId(1), "Ravager of the Fells", back),
        ],
        structure: CardStructure::DoubleFaced {
            front: CardPartId::PRIMARY,
            back: CardPartId(1),
            kind: DoubleFacedKind::Transforming,
        },
        play_options: vec![PlayOptionDef::cast(
            PlayOptionId::DEFAULT,
            "Huntmaster of the Fells",
            SpellForm::Part(CardPartId::PRIMARY),
            front
                .mana_cost()
                .expect("Huntmaster of the Fells has a printed mana cost"),
            CardEffectStatus::MetadataOnly,
        )],
    }
}

// DKA 140 — Huntmaster of the Fells
pub(in crate::card::sets) static HUNTMASTER_OF_THE_FELLS: CardRecord = CardRecord::new(
    cards::HUNTMASTER_OF_THE_FELLS,
    "Huntmaster of the Fells",
    CardArt::new("aae6fb12-b252-453b-bca7-1ea2a0d6c8dc", "Chris Rahn"),
    CardSet::DarkAscension,
    huntmaster_front_rules(),
)
.with_composition(huntmaster_composition);

// DKA 141 — Immerwolf
// Audit: blocked — Needs a continuous prohibition preventing non-Human Werewolves you control from transforming.

// DKA 142 — Sorin, Lord of Innistrad
// Audit: blocked — Needs the emblem/token identities and an ultimate continuation that returns only the permanents destroyed this way under your control.

// DKA 143 — Stromkirk Captain
pub(in crate::card::sets) static STROMKIRK_CAPTAIN: CardRecord = CardRecord::new(
    cards::STROMKIRK_CAPTAIN,
    "Stromkirk Captain",
    CardArt::new(
        "5bfcca87-04f8-480a-bae6-ae87f7afb7e1",
        "Jana Schirmer & Johannes Voss",
    ),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{1}{B}{R}"), &["Vampire", "Soldier"], 2, 2).with_abilities(
        &[
            abilities::first_strike(),
            AbilityDef::static_ability(
                "Other Vampire creatures you control get +1/+1 and have first strike.",
                EffectDef::Sequence(&[
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::MatchingObjects {
                            object: ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Subtype("Vampire"),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            zones: &[ZoneKind::Battlefield],
                            controller: PlayerRelation::You,
                        },
                        effect: AppliedEffectDef::ModifyPowerToughness {
                            power: ValueDef::Constant(1),
                            toughness: ValueDef::Constant(1),
                        },
                        duration: EffectDurationDef::WhileSourceRemainsInZone,
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::MatchingObjects {
                            object: ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Subtype("Vampire"),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            zones: &[ZoneKind::Battlefield],
                            controller: PlayerRelation::You,
                        },
                        effect: AppliedEffectDef::GrantAbility(&abilities::first_strike()),
                        duration: EffectDurationDef::WhileSourceRemainsInZone,
                    },
                ]),
            ),
        ],
    ),
);

// DKA 144 — Altar of the Lost
// Audit: blocked — Needs two-mana any-color combination choice plus spending provenance restricted to flashback spells cast from graveyards.

// DKA 145 — Avacyn's Collar
// Audit: blocked — Needs equipment attachment/equip actions and a death trigger granted to the equipped creature that tests whether it was Human.

// DKA 146 — Chalice of Life
// Audit: blocked — Needs an activation-resolution life-total threshold that transforms the source, plus the complete back-face mana ability.

// DKA 147 — Elbrus, the Binding Blade
// Audit: blocked — Needs equipment attachment/equip actions and a combat-damage trigger on the equipped creature that un attaches and transforms this permanent.

// DKA 148 — Executioner's Hood
// Audit: blocked — Needs equipment attachment/equip actions before intimidate can be granted to the equipped creature.

// DKA 149 — Grafdigger's Cage
// Audit: blocked — Needs zone-origin-sensitive casting prohibitions and a replacement that stops creature cards in graveyards or libraries entering the battlefield.

// DKA 150 — Heavy Mattock
// Audit: blocked — Needs equipment attachment/equip actions and a conditional extra +1/+1 while the equipped creature is Human.

// DKA 151 — Helvault
pub(in crate::card::sets) static HELVAULT: CardRecord = CardRecord::new(
    cards::HELVAULT,
    "Helvault",
    CardArt::new("16d2448c-1b2e-466a-a0ab-e20ba1de6bc9", "Jaime Jones"),
    CardSet::DarkAscension,
    CardRules::new_artifact(mana_cost!("{3}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_with_targets(
                "{1}, {T}: Exile target creature you control.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{1}")),
                    AbilityCostDef::TapSource,
                ],
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                })],
                EffectDef::ExileLinkedToSource {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::activated_with_targets(
                "{7}, {T}: Exile target creature you don't control.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{7}")),
                    AbilityCostDef::TapSource,
                ],
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::NotYou),
                    owner: None,
                })],
                EffectDef::ExileLinkedToSource {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::triggered(
                "When Helvault is put into a graveyard from the battlefield, return all cards exiled with it to the battlefield under their owners' control.",
                TriggerEventDef::ZoneChanged {
                    object: ObjectPredicateDef::Source,
                    from: Some(ZoneKind::Battlefield),
                    to: Some(ZoneKind::Graveyard),
                },
                EffectDef::ReturnLinkedExiles {
                    zone: ZoneKind::Battlefield,
                    grant: None,
                },
            ),
        ]),
);

// DKA 152 — Jar of Eyeballs
// Audit: blocked — Needs a “remove all” counter cost whose removed count is retained as X for a later top-card selection.

// DKA 153 — Warden of the Wall
// Audit: blocked — Needs a continuous animation active only during turns other than its controller's, while preserving the tapped entry and mana ability.

// DKA 154 — Wolfhunter's Quiver
// Audit: blocked — Needs equipment attachment/equip actions and two distinct tap abilities granted to the equipped creature.

// DKA 155 — Evolving Wilds
// Audit: blocked — SearchZone cannot make the selected basic land enter tapped.

// DKA 156 — Grim Backwoods
pub(in crate::card::sets) static GRIM_BACKWOODS: CardRecord = CardRecord::new(
    cards::GRIM_BACKWOODS,
    "Grim Backwoods",
    CardArt::new("045abeeb-f5e5-4f3f-9836-5b1553e03f11", "Vincent Proce"),
    CardSet::DarkAscension,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{2}{B}{G}, {T}, Sacrifice a creature: Draw a card.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{B}{G}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
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

// DKA 157 — Haunted Fengraf
// Audit: blocked — Needs a deterministic random choice among creature cards in the controller's graveyard.

// DKA 158 — Vault of the Archangel
pub(in crate::card::sets) static VAULT_OF_THE_ARCHANGEL: CardRecord = CardRecord::new(
    cards::VAULT_OF_THE_ARCHANGEL,
    "Vault of the Archangel",
    CardArt::new("35a65437-430a-42ef-854f-6e66f8e1a04a", "John Avon"),
    CardSet::DarkAscension,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{2}{W}{B}, {T}: Creatures you control gain deathtouch and lifelink until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{W}{B}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::MatchingObjects {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: PlayerRelation::You,
                    },
                    effect: AppliedEffectDef::GrantAbility(&abilities::deathtouch()),
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::MatchingObjects {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: PlayerRelation::You,
                    },
                    effect: AppliedEffectDef::GrantAbility(&abilities::lifelink()),
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ]),
);
pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &BAR_THE_DOOR,
    &BURDEN_OF_GUILT,
    &ELGAUD_INQUISITOR,
    &LINGERING_SOULS,
    &MIDNIGHT_GUARD,
    &NIBLIS_OF_THE_MIST,
    &NIBLIS_OF_THE_URN,
    &RAY_OF_REVELATION,
    &REQUIEM_ANGEL,
    &SANCTUARY_CAT,
    &SILVERCLAW_GRIFFIN,
    &SKILLFUL_LUNGE,
    &THRABEN_HERETIC,
    &ARTFUL_DODGE,
    &BONE_TO_ASH,
    &CHANT_OF_THE_SKIFSANG,
    &CHILL_OF_FOREBODING,
    &DIVINATION,
    &DUNGEON_GEISTS,
    &GERALFS_MINDCRUSHER,
    &GRIPTIDE,
    &HAVENGUL_RUNEBINDER,
    &MYSTIC_RETRIEVAL,
    &NEPHALIA_SEAKITE,
    &SAVING_GRASP,
    &SCREECHING_SKAAB,
    &SHRIEKGEIST,
    &THOUGHT_SCOUR,
    &TOWER_GEIST,
    &BLACK_CAT,
    &FARBOG_BONEFLINGER,
    &GERALFS_MESSENGER,
    &HARROWING_JOURNEY,
    &HIGHBORN_GHOUL,
    &REAP_THE_SEAGRAF,
    &SKIRSDAG_FLAYER,
    &TRAGIC_SLIP,
    &UNDYING_EVIL,
    &VENGEFUL_VAMPIRE,
    &BLOOD_FEUD,
    &BURNING_OIL,
    &ERDWAL_RIPPER,
    &FAITHLESS_LOOTING,
    &FIRES_OF_UNDEATH,
    &FORGE_DEVIL,
    &HECKLING_FIENDS,
    &HELLRIDER,
    &MARKOV_BLADEMASTER,
    &MOONVEIL_DRAGON,
    &NEARHEATH_STALKER,
    &RUSSET_WOLVES,
    &SCORCH_THE_FIELDS,
    &TALONS_OF_FALKENRATH,
    &TORCH_FIEND,
    &BRIARPACK_ALPHA,
    &CRUSHING_VINES,
    &DERANGED_OUTCAST,
    &GHOULTREE,
    &GRIM_FLOWERING,
    &HOLLOWHENGE_BEAST,
    &HUNGER_OF_THE_HOWLPACK,
    &KESSIG_RECLUSE,
    &PREDATOR_OOZE,
    &SOMBERWALD_DRYAD,
    &STRANGLEROOT_GEIST,
    &VORAPEDE,
    &WILD_HUNGER,
    &YOUNG_WOLF,
    &DIREGRAF_CAPTAIN,
    &DROGSKOL_CAPTAIN,
    &DROGSKOL_REAVER,
    &HUNTMASTER_OF_THE_FELLS,
    &STROMKIRK_CAPTAIN,
    &HELVAULT,
    &GRIM_BACKWOODS,
    &VAULT_OF_THE_ARCHANGEL,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
