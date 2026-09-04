//! Magic 2010 card records.

use super::{CardRecord, PrintingRecord};
use crate::AbilityCostDef;
use crate::AbilityDef;
use crate::AbilityTargetDef;
use crate::AbilityTargetPredicate;
use crate::AddManaEffectDef;
use crate::AppliedEffectDef;
use crate::BasicLandType;
use crate::CardArt;
use crate::CardRules;
use crate::CardSet;
use crate::CardSupertype;
use crate::CardType;
use crate::ColorSet;
use crate::ControlDurationDef;
use crate::CreatureTypeSetDef;
use crate::DamageEventMatcherDef;
use crate::DamagePreventionDef;
use crate::EffectDef;
use crate::EffectRecipientDef;
use crate::ManaColor;
use crate::ObjectPredicateDef;
use crate::ObjectQueryDef;
use crate::PlayerRefDef;
use crate::PlayerRelation;
use crate::ResolvedEffectDurationDef;
use crate::TargetIndex;
use crate::TriggerEventDef;
use crate::ValueDef;
use crate::ZoneKind;
use crate::ZonePlacement;
use crate::card::abilities;

use crate::mana_cost;

// M10 2 — Angel's Mercy
pub(in crate::card::sets) static ANGELS_MERCY: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Angel's Mercy",
    "9b911124-3646-4014-b574-13fee44bfad5",
    "Andrew Robinson",
    CardRules::new_instant(mana_cost!("{2}{W}{W}")).with_ability(AbilityDef::spell(
        "You gain 7 life.",
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(7),
        },
    )),
);

// M10 6 — Captain of the Watch
pub(in crate::card::sets) static CAPTAIN_OF_THE_WATCH: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Captain of the Watch",
    "e6c26b98-790e-403b-b94b-261a4c92e721",
    "Greg Staples",
    CardRules::new_creature(mana_cost!("{4}{W}{W}"), &["Human", "Soldier"], 3, 3).with_abilities(
        &[
            abilities::vigilance(),
            AbilityDef::static_ability(
                "Other Soldier creatures you control get +1/+1 and have vigilance.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype("Soldier"),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::vigilance()),
                    ]),
                },
            ),
            abilities::enters_trigger(
                "When this creature enters, create three 1/1 white Soldier creature tokens.",
                EffectDef::create_creature_token(&["Soldier"], &[ManaColor::White], 1, 1)
                    .with_art(CardArt::new(
                        "86272c08-c5f2-413f-87ea-b135aca2d9c5",
                        "Greg Staples",
                    ))
                    .with_amount(3),
            ),
        ],
    ),
);

// M10 8 — Divine Verdict
pub(in crate::card::sets) static DIVINE_VERDICT: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Divine Verdict",
    "48444e14-c73b-47d1-9c55-0ff4dc3c6034",
    "Kev Walker",
    CardRules::new_instant(mana_cost!("{3}{W}")).with_ability(AbilityDef::destroy_target(
        "Destroy target attacking or blocking creature.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::AttackingOrBlocking,
        ])),
        true,
    )),
);

// M10 9 — Elite Vanguard
pub(in crate::card::sets) static ELITE_VANGUARD: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2010,
    "Elite Vanguard",
    "6bda0b4b-ab5a-4d91-9dd1-7a5a145b67f5",
    "Mark Tedin",
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 2, 1),
);

// M10 11 — Glorious Charge
pub(in crate::card::sets) static GLORIOUS_CHARGE: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Glorious Charge",
    "8c6dfcf8-a09b-4402-af80-90fe15b2ce0a",
    "Izzy",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell(
        "Creatures you control get +1/+1 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(1),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M10 12 — Griffin Sentinel
pub(in crate::card::sets) static GRIFFIN_SENTINEL: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Griffin Sentinel",
    "6784b663-b117-45a2-bde4-72e080058ea7",
    "Warren Mahy",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Griffin"], 1, 3)
        .with_abilities(&[abilities::flying(), abilities::vigilance()]),
);

// M10 16 — Honor of the Pure
pub(in crate::card::sets) static HONOR_OF_THE_PURE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2010,
    "Honor of the Pure",
    "35a40f09-d16a-43c7-b4fd-244f45883a47",
    "Greg Staples",
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_ability(AbilityDef::static_ability(
        "White creatures you control get +1/+1.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::White),
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

// M10 17 — Indestructibility
pub(in crate::card::sets) static INDESTRUCTIBILITY: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Indestructibility",
    "ef8765a0-c2ae-4b1a-a3f5-0243b43e6da0",
    "Darrell Riche",
    CardRules::new_enchantment(mana_cost!("{3}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant permanent",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::Any,
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted permanent has indestructible.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                },
            ),
        ]),
);

// M10 18 — Lifelink
pub(in crate::card::sets) static LIFELINK: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2010,
    "Lifelink",
    "f0d881c1-24e7-4ce7-8ab1-474cb040ddd7",
    "Terese Nielsen",
    CardRules::new_enchantment(mana_cost!("{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature has lifelink.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::lifelink()),
                },
            ),
        ]),
);

// M10 24 — Planar Cleansing
pub(in crate::card::sets) static PLANAR_CLEANSING: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Planar Cleansing",
    "30ee0d57-e404-4599-9b6e-f8ab8a95f9fa",
    "Michael Komarck",
    CardRules::new_sorcery(mana_cost!("{3}{W}{W}{W}")).with_ability(AbilityDef::spell(
        "Destroy all nonland permanents.",
        EffectDef::Destroy {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            can_regenerate: true,
            then: None,
        },
    )),
);

// M10 28 — Safe Passage
pub(in crate::card::sets) static SAFE_PASSAGE: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Safe Passage",
    "4d8528ef-5e7d-46da-a454-395cd38c213f",
    "Christopher Moeller",
    CardRules::new_instant(mana_cost!("{2}{W}")).with_ability(AbilityDef::spell(
        "Prevent all damage that would be dealt to you and creatures you control this turn.",
        EffectDef::PreventDamage {
            prevention: DamagePreventionDef::unlimited(
                DamageEventMatcherDef::to_player_and_creatures_controlled_by(
                    PlayerRefDef::EffectController,
                ),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M10 30 — Siege Mastodon
pub(in crate::card::sets) static SIEGE_MASTODON: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Siege Mastodon",
    "287d4c56-1b75-4ac4-8be8-333b1aba982a",
    "Matt Cavotta",
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Elephant"], 3, 5),
);

// M10 31 — Silence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SILENCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2010,
    "Silence",
    "1559d660-8a9d-422b-95d3-710a046583dd",
    "Wayne Reynolds",
    crate::card::CardRules::unsupported(),
);

// M10 32 — Silvercoat Lion
pub(in crate::card::sets) static SILVERCOAT_LION: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Silvercoat Lion",
    "ea82996f-a05f-4831-9bbd-3281ebca9a61",
    "Terese Nielsen",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Cat"], 2, 2),
);

// M10 33 — Solemn Offering
pub(in crate::card::sets) static SOLEMN_OFFERING: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Solemn Offering",
    "67aafbcc-113e-4816-95d2-a192f32ea9ea",
    "Sam Wood",
    CardRules::new_sorcery(mana_cost!("{2}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target artifact or enchantment. You gain 4 life.",
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
                amount: ValueDef::Constant(4),
            },
        ]),
    )),
);

// M10 35 — Stormfront Pegasus
pub(in crate::card::sets) static STORMFRONT_PEGASUS: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2010,
    "Stormfront Pegasus",
    "d2429a15-ccbe-463c-9218-968709d9e878",
    "rk post",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Pegasus"], 2, 1)
        .with_abilities(&[abilities::flying()]),
);

// M10 43 — Alluring Siren
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALLURING_SIREN: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2010,
    "Alluring Siren",
    "df4e1cc3-4e47-4eff-9047-c6d1cc84d635",
    "Chippy",
    crate::card::CardRules::unsupported(),
);

// M10 49 — Divination
pub(in crate::card::sets) static DIVINATION: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Divination",
    "3102cec9-1cdc-4946-a2dd-caf04eaa8b97",
    "Howard Lyon",
    CardRules::new_sorcery(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell(
        "Draw two cards.",
        abilities::draw_cards(ValueDef::Constant(2)),
    )),
);

// M10 50 — Djinn of Wishes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DJINN_OF_WISHES: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2010,
    "Djinn of Wishes",
    "3e3b0949-17e1-4f12-8999-d4638d32dd3e",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// M10 51 — Essence Scatter
pub(in crate::card::sets) static ESSENCE_SCATTER: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Essence Scatter",
    "c231101e-6620-46fc-a0ad-a53291d12dc2",
    "Jon Foster",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::counter_target(
        "Counter target creature spell.",
        &AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::Spell,
                ObjectPredicateDef::HasType(CardType::Creature),
            ]),
            zones: &[ZoneKind::Stack],
            controller: None,
            owner: None,
        }),
    )),
);

// M10 56 — Ice Cage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ICE_CAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2010,
    "Ice Cage",
    "4d18c4d7-c779-473b-9b41-f22b439bb501",
    "Mike Bierek",
    crate::card::CardRules::unsupported(),
);

// M10 63 — Mind Control
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIND_CONTROL: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2010,
    "Mind Control",
    "37151305-e489-4df1-9b0a-c5e11c77d2f1",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// M10 71 — Sleep
/// Both clauses reach the same set, so the skip lands on exactly the
/// creatures the tap found rather than on whatever is tapped later.
static SLEEP_THEIR_CREATURES: EffectRecipientDef = EffectRecipientDef::objects_controlled_by_target(
    ObjectPredicateDef::HasType(CardType::Creature),
    TargetIndex::PRIMARY,
);

pub(in crate::card::sets) static SLEEP: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Sleep",
    "133be4f4-1daa-41b6-b509-9e64c6b00059",
    "Chris Rahn",
    CardRules::new_sorcery(mana_cost!("{2}{U}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Tap all creatures target player controls. Those creatures don't untap during that \
         player's next untap step.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::Tap {
                object: SLEEP_THEIR_CREATURES,
            },
            EffectDef::SkipNextUntapSteps {
                object: SLEEP_THEIR_CREATURES,
                count: 1,
            },
        ]),
    )),
);

// M10 76 — Tome Scour
pub(in crate::card::sets) static TOME_SCOUR: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Tome Scour",
    "fdbdf96b-e7c5-42e5-9a16-03daafde40af",
    "Steven Belledin",
    CardRules::new_sorcery(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target player mills five cards.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Mill {
            player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(5),
        },
    )),
);

// M10 80 — Wall of Frost
pub(in crate::card::sets) static WALL_OF_FROST: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Wall of Frost",
    "17bc35a7-e38b-4c15-889a-d58c8b360315",
    "Mike Bierek",
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Wall"], 0, 7).with_abilities(&[
        abilities::defender(),
        // The blocked creature is the trigger's own object, so the skip
        // lands on it rather than on whatever else is in the combat.
        AbilityDef::triggered(
            "Whenever this creature blocks a creature, that creature doesn't untap during its \
             controller's next untap step.",
            TriggerEventDef::Blocks {
                blocked: ObjectPredicateDef::Any,
            },
            EffectDef::SkipNextUntapSteps {
                object: EffectRecipientDef::TriggeringObject,
                count: 1,
            },
        ),
    ]),
);

// M10 87 — Cemetery Reaper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CEMETERY_REAPER: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2010,
    "Cemetery Reaper",
    "639b48f0-3426-46cf-b857-4611f7de4826",
    "Dave Allsop",
    crate::card::CardRules::unsupported(),
);

// M10 88 — Child of Night
pub(in crate::card::sets) static CHILD_OF_NIGHT: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Child of Night",
    "e1f7a9a7-3679-4a18-a52a-e3a8ab16ad32",
    "Ash Wood",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Vampire"], 2, 1)
        .with_abilities(&[abilities::lifelink()]),
);

// M10 92 — Disentomb
pub(in crate::card::sets) static DISENTOMB: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Disentomb",
    "99a329a0-a14a-49b9-adcd-397b566211ee",
    "Alex Horley-Orlandelli",
    CardRules::new_sorcery(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
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
        },
    )),
);

// M10 93 — Doom Blade
pub(in crate::card::sets) static DOOM_BLADE: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Doom Blade",
    "6e19acff-f3dd-417a-a9ab-ea3e36c1ba61",
    "Chippy",
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(AbilityDef::destroy_target(
        "Destroy target nonblack creature.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::Black)),
        ])),
        true,
    )),
);

// M10 109 — Rise from the Grave
pub(in crate::card::sets) static RISE_FROM_THE_GRAVE: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Rise from the Grave",
    "cb9dd0d9-8e35-4a8c-af6f-83c7d2a3ea7d",
    "Vance Kovacs",
    // Any graveyard, so it steals as readily as it recurs.
    CardRules::new_sorcery(mana_cost!("{4}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Put target creature card from a graveyard onto the battlefield under your control. That creature is a black Zombie in addition to its other colors and types.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::WithZoneMoveResult {
            effect: &EffectDef::WithBattlefieldArrival {
                effect: &EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
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
                // than set.
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_colors(ColorSet::from_colors(&[ManaColor::Black])),
                    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Zombie"])),
                ]),
                duration: ResolvedEffectDurationDef::Permanent,
            },
        },
    )),
);

// M10 111 — Sanguine Bond
pub(in crate::card::sets) static SANGUINE_BOND: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Sanguine Bond",
    "445d6a68-ed53-4c96-973a-c29282514f41",
    "Jaime Jones",
    CardRules::new_enchantment(mana_cost!("{3}{B}{B}")).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever you gain life, target opponent loses that much life.",
            TriggerEventDef::LifeGained(PlayerRelation::You),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::TriggerEventAmount,
            },
        ),
    ),
);

// M10 112 — Sign in Blood
pub(in crate::card::sets) static SIGN_IN_BLOOD: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Sign in Blood",
    "1975ed97-acb8-4bb6-804a-e5da725d876e",
    "Howard Lyon",
    CardRules::new_sorcery(mana_cost!("{B}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target player draws two cards and loses 2 life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ]),
    )]),
);

// M10 118 — Vampire Nocturnus
// Audit: unsupported — PlaysWithTopOfLibraryRevealed exists, but static conditions cannot inspect the top card's color for the Vampire mass bonus and flying grant.
pub(in crate::card::sets) static VAMPIRE_NOCTURNUS: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2010,
    "Vampire Nocturnus",
    "9df4f1ea-dbaa-456c-884c-97f03b64fa17",
    "Raymond Swanland",
    crate::card::CardRules::unsupported(),
);

// M10 120 — Warpath Ghoul
pub(in crate::card::sets) static WARPATH_GHOUL: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2010,
    "Warpath Ghoul",
    "2c6cc262-ba0c-4cca-ae9c-24a1824753e4",
    "rk post",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Zombie"], 3, 2),
);

// M10 123 — Zombie Goliath
pub(in crate::card::sets) static ZOMBIE_GOLIATH: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Zombie Goliath",
    "cc295834-af33-45ae-be4d-7a1987f85561",
    "E. M. Gist",
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Zombie", "Giant"], 4, 3),
);

// M10 124 — Act of Treason
pub(in crate::card::sets) static ACT_OF_TREASON: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Act of Treason",
    "8b63bee5-d8e5-4c2f-8514-8c86d025f7c9",
    "Eric Deschamps",
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

// M10 135 — Fiery Hellhound
pub(in crate::card::sets) static FIERY_HELLHOUND: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2010,
    "Fiery Hellhound",
    "6d6b2c8a-8019-4e4b-8f4e-058ab5284153",
    "Ted Galaday",
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Elemental", "Dog"], 2, 2).with_ability(
        AbilityDef::activated(
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
        ),
    ),
);

// M10 139 — Goblin Chieftain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_CHIEFTAIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2010,
    "Goblin Chieftain",
    "f5c8a4a4-1611-4188-9c59-8aefb016b5ad",
    "Sam Wood",
    crate::card::CardRules::unsupported(),
);

// M10 165 — Acidic Slime
pub(in crate::card::sets) static ACIDIC_SLIME: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Acidic Slime",
    "f1377f45-edee-4922-825b-6f22163ff63d",
    "Karl Kopinski",
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Ooze"], 2, 2).with_abilities(&[
        abilities::deathtouch(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, destroy target artifact, enchantment, or land.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::HasType(CardType::Land),
                ]),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
        ),
    ]),
);

// M10 169 — Borderland Ranger
pub(in crate::card::sets) static BORDERLAND_RANGER: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Borderland Ranger",
    "bdd0f8c8-1a1f-4d9b-a6e1-3654f3995012",
    "Jesper Ejsing",
    CardRules::new_creature(
        mana_cost!("{2}{G}"),
        &["Human", "Scout", "Ranger"],
        2,
        2,
    )
    .with_ability(abilities::enters_trigger("When this creature enters, you may search your library for a basic land card, reveal it, put it into your hand, then shuffle.", EffectDef::May {
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
                attachment: None,
                binding: None,
                then: None,
            },
        })),
);

// M10 170 — Bountiful Harvest
pub(in crate::card::sets) static BOUNTIFUL_HARVEST: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Bountiful Harvest",
    "d225382c-cc6f-4224-82b5-4309b72feb0b",
    "Jason Chan",
    CardRules::new_sorcery(mana_cost!("{4}{G}")).with_ability(AbilityDef::spell(
        "You gain 1 life for each land you control.",
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Land),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            )),
        },
    )),
);

// M10 172 — Centaur Courser
pub(in crate::card::sets) static CENTAUR_COURSER: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Centaur Courser",
    "03354b67-7df2-4b4b-a996-a37550e58561",
    "Vance Kovacs",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Centaur", "Warrior"], 3, 3),
);

// M10 174 — Cudgel Troll
pub(in crate::card::sets) static CUDGEL_TROLL: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2010,
    "Cudgel Troll",
    "d779b14c-a100-4382-9e7c-0969efda73ec",
    "Jesper Ejsing",
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Troll"], 4, 3).with_ability(
        abilities::regenerate_self(
            "{G}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{G}"))],
        ),
    ),
);

// M10 175 — Deadly Recluse
pub(in crate::card::sets) static DEADLY_RECLUSE: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Deadly Recluse",
    "6ab810f1-21d6-4a98-b77a-e455370aa6cc",
    "Warren Mahy",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Spider"], 1, 2)
        .with_abilities(&[abilities::reach(), abilities::deathtouch()]),
);

// M10 176 — Elvish Archdruid
pub(in crate::card::sets) static ELVISH_ARCHDRUID: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Elvish Archdruid",
    "544bc214-e32d-4370-a15f-62812a0420be",
    "Karl Kopinski",
    // The count includes the Archdruid itself, which is an Elf: a lone one
    // taps for a single green rather than none.
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Elf", "Druid"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "Other Elf creatures you control get +1/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Elf"),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {G} for each Elf you control.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddManaEqualTo {
                color: ManaColor::Green,
                amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::Subtype("Elf"),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
            },
        ),
    ]),
);

// M10 203 — Runeclaw Bear
pub(in crate::card::sets) static RUNECLAW_BEAR: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2010,
    "Runeclaw Bear",
    "268bd9d5-4da1-4cbf-83f9-47f7aac1cfc3",
    "Jesper Ejsing",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Bear"], 2, 2),
);

// M10 204 — Stampeding Rhino
pub(in crate::card::sets) static STAMPEDING_RHINO: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2010,
    "Stampeding Rhino",
    "f5a33394-d26c-4dcd-948c-e7d370059b11",
    "Steven Belledin",
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Rhino"], 4, 4)
        .with_abilities(&[abilities::trample()]),
);

// M10 205 — Windstorm
pub(in crate::card::sets) static WINDSTORM: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Windstorm",
    "ee3768ec-bb3b-44dc-9fa3-7cb3d3ee9f8c",
    "Rob Alexander",
    CardRules::new_instant(mana_cost!("{X}{G}")).with_ability(AbilityDef::spell(
        "Windstorm deals X damage to each creature with flying.",
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Flying),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            amount: ValueDef::ChosenX,
        },
    )),
);

// M10 223 — Dragonskull Summit
pub(in crate::card::sets) static DRAGONSKULL_SUMMIT: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Dragonskull Summit",
    "2d4998db-13c0-412f-b02c-9f041cc45c7e",
    "Jon Foster",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::check_land_enters(
            "This land enters tapped unless you control a Swamp or a Mountain.",
            &[BasicLandType::Swamp, BasicLandType::Mountain],
        ),
        AbilityDef::activated_mana(
            "{T}: Add {B} or {R}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Red,
            ])),
        ),
    ]),
);

// M10 224 — Drowned Catacomb
pub(in crate::card::sets) static DROWNED_CATACOMB: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Drowned Catacomb",
    "bdc6e950-38dd-46da-a1c3-58dc7495a9f9",
    "Dave Kendall",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::check_land_enters(
            "This land enters tapped unless you control an Island or a Swamp.",
            &[BasicLandType::Island, BasicLandType::Swamp],
        ),
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

// M10 226 — Glacial Fortress
pub(in crate::card::sets) static GLACIAL_FORTRESS: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Glacial Fortress",
    "ee834e27-595d-4d12-8e69-e94e84ef337a",
    "Franz Vohwinkel",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::check_land_enters(
            "This land enters tapped unless you control a Plains or an Island.",
            &[BasicLandType::Plains, BasicLandType::Island],
        ),
        AbilityDef::activated_mana(
            "{T}: Add {W} or {U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Blue,
            ])),
        ),
    ]),
);

// M10 227 — Rootbound Crag
pub(in crate::card::sets) static ROOTBOUND_CRAG: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Rootbound Crag",
    "5433b11b-efe9-4d94-8f71-6bf7c403494d",
    "Matt Stewart",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::check_land_enters(
            "This land enters tapped unless you control a Mountain or a Forest.",
            &[BasicLandType::Mountain, BasicLandType::Forest],
        ),
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

// M10 228 — Sunpetal Grove
pub(in crate::card::sets) static SUNPETAL_GROVE: CardRecord = CardRecord::new(
    CardSet::Magic2010,
    "Sunpetal Grove",
    "9f4e8ead-8c82-4258-bc59-551f7a74e042",
    "Jason Chan",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::check_land_enters(
            "This land enters tapped unless you control a Forest or a Plains.",
            &[BasicLandType::Forest, BasicLandType::Plains],
        ),
        AbilityDef::activated_mana(
            "{T}: Add {G} or {W}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Green,
                ManaColor::White,
            ])),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANGELS_MERCY,
    &CAPTAIN_OF_THE_WATCH,
    &DIVINE_VERDICT,
    &ELITE_VANGUARD,
    &GLORIOUS_CHARGE,
    &GRIFFIN_SENTINEL,
    &HONOR_OF_THE_PURE,
    &INDESTRUCTIBILITY,
    &LIFELINK,
    &PLANAR_CLEANSING,
    &SAFE_PASSAGE,
    &SIEGE_MASTODON,
    &SILENCE,
    &SILVERCOAT_LION,
    &SOLEMN_OFFERING,
    &STORMFRONT_PEGASUS,
    &ALLURING_SIREN,
    &DIVINATION,
    &DJINN_OF_WISHES,
    &ESSENCE_SCATTER,
    &ICE_CAGE,
    &MIND_CONTROL,
    &SLEEP,
    &TOME_SCOUR,
    &WALL_OF_FROST,
    &CEMETERY_REAPER,
    &CHILD_OF_NIGHT,
    &DISENTOMB,
    &DOOM_BLADE,
    &RISE_FROM_THE_GRAVE,
    &SANGUINE_BOND,
    &SIGN_IN_BLOOD,
    &VAMPIRE_NOCTURNUS,
    &WARPATH_GHOUL,
    &ZOMBIE_GOLIATH,
    &ACT_OF_TREASON,
    &FIERY_HELLHOUND,
    &GOBLIN_CHIEFTAIN,
    &ACIDIC_SLIME,
    &BORDERLAND_RANGER,
    &BOUNTIFUL_HARVEST,
    &CENTAUR_COURSER,
    &CUDGEL_TROLL,
    &DEADLY_RECLUSE,
    &ELVISH_ARCHDRUID,
    &RUNECLAW_BEAR,
    &STAMPEDING_RHINO,
    &WINDSTORM,
    &DRAGONSKULL_SUMMIT,
    &DROWNED_CATACOMB,
    &GLACIAL_FORTRESS,
    &ROOTBOUND_CRAG,
    &SUNPETAL_GROVE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
