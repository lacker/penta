//! Zendikar cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::ControlDurationDef;
use crate::CounterKind;
use crate::KeywordAbility;
use crate::PlayerRefDef;
use crate::ResolvedEffectDurationDef;
use crate::card::ColorChoiceOperationDef;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AdditionalCostValueDef, AppliedEffectDef, AppliedRuleDef, BasicLandType, CardRules, CardSet,
    CardType, ComparisonDef, EffectDef, EffectRecipientDef, ManaColor, ObjectPredicateDef,
    ObjectRefDef, PlayerRelation, TriggerConditionDef, TriggerEventDef, ValueComparisonDef,
    ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

/// The five allied fetchlands of Onslaught got an enemy-coloured cycle here,
/// with the same text. One helper states it once; only the two land types and
/// the order they are named in differ.
const fn fetch_land(text: &'static str, land_types: &'static [BasicLandType]) -> CardRules {
    CardRules::new_land(&[]).with_ability(abilities::fetch_land_ability(
        text,
        ObjectPredicateDef::HasAnyBasicLandType(land_types),
    ))
}

// ZEN 4 — Brave the Elements
pub(in crate::card::sets) static BRAVE_THE_ELEMENTS: CardRecord = CardRecord::new(
    CardSet::Zendikar,
    "Brave the Elements",
    "c14c492d-3fd7-4b2a-910e-bfcb33752eba",
    "Goran Josic",
    // One mana that makes a white board unblockable, or immune to a sweeper:
    // the group is settled first and the colour named afterwards.
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell(
        "Choose a color. White creatures you control gain protection from the chosen color until end of turn.",
        EffectDef::ChooseColor {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::White),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            operation: ColorChoiceOperationDef::ProtectionFromChosenColor,
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// ZEN 9 — Day of Judgment
pub(in crate::card::sets) static DAY_OF_JUDGMENT: CardRecord = CardRecord::new(
    crate::card::CardSet::Zendikar,
    "Day of Judgment",
    "2aa98fca-972b-46c2-bdec-6ace35c988d5",
    "Vincent Proce",
    CardRules::new_sorcery(mana_cost!("{2}{W}{W}")).with_ability(AbilityDef::spell(
        "Destroy all creatures.",
        EffectDef::Destroy {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            can_regenerate: true,
            then: None,
        },
    )),
);

// ZEN 14 — Journey to Nowhere
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JOURNEY_TO_NOWHERE: CardRecord = CardRecord::new(
    crate::card::CardSet::Zendikar,
    "Journey to Nowhere",
    "09cfe585-8a55-4b27-89e0-dfb6946fe1f3",
    "Warren Mahy",
    crate::card::CardRules::unsupported(),
);

// ZEN 23 — Kor Skyfisher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KOR_SKYFISHER: CardRecord = CardRecord::new(
    crate::card::CardSet::Zendikar,
    "Kor Skyfisher",
    "bb2e9465-f5ba-4c7b-9f03-d40dc8394acd",
    "Dan Murayama Scott",
    crate::card::CardRules::unsupported(),
);

// ZEN 31 — Pillarfield Ox
pub(in crate::card::sets) static PILLARFIELD_OX: CardRecord = CardRecord::new(
    CardSet::Zendikar,
    "Pillarfield Ox",
    "d70a8ff1-f0cf-4aef-ad90-06902f98d434",
    "Andrew Robinson",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Ox"], 2, 4),
);

// ZEN 48 — Into the Roil
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INTO_THE_ROIL: CardRecord = CardRecord::new(
    crate::card::CardSet::Zendikar,
    "Into the Roil",
    "5dba9972-dd8b-407b-9374-a8f0ed1a96db",
    "Kieran Yanner",
    crate::card::CardRules::unsupported(),
);

// ZEN 50 — Kraken Hatchling
pub(in crate::card::sets) static KRAKEN_HATCHLING: CardRecord = CardRecord::new(
    CardSet::Zendikar,
    "Kraken Hatchling",
    "45d100a3-93f2-428c-8f54-8807e71f2638",
    "Jason Felix",
    CardRules::new_creature(mana_cost!("{U}"), &["Kraken"], 0, 4),
);

// ZEN 58 — Paralyzing Grasp
pub(in crate::card::sets) static PARALYZING_GRASP: CardRecord = CardRecord::new(
    CardSet::Zendikar,
    "Paralyzing Grasp",
    "4af35801-9280-4ec1-9399-e34501919a8f",
    "Izzy",
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
                "Enchanted creature doesn't untap during its controller's untap step.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
                },
            ),
        ]),
);

// ZEN 67 — Spell Pierce
pub(in crate::card::sets) static SPELL_PIERCE: CardRecord = CardRecord::new(
    CardSet::Zendikar,
    "Spell Pierce",
    "cb3d3901-e4a6-45ab-a7b5-c65d91e1875e",
    "Vance Kovacs",
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target noncreature spell unless its controller pays {2}.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Spell,
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                ]),
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        abilities::counter_target_unless_paid(ValueDef::Constant(2)),
    )),
);

// ZEN 76 — Welkin Tern
pub(in crate::card::sets) static WELKIN_TERN: CardRecord = CardRecord::new(
    CardSet::Zendikar,
    "Welkin Tern",
    "357931d0-8ba6-4857-9db9-7f42d81514a5",
    "Austin Hsu",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Bird"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "This creature can block only creatures with flying.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::can_block_only(
                    ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                )),
            },
        ),
    ]),
);

// ZEN 80 — Blood Seeker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLOOD_SEEKER: CardRecord = CardRecord::new(
    crate::card::CardSet::Zendikar,
    "Blood Seeker",
    "d1abc9e8-9ecf-4665-9ea5-ee18ab83c148",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// ZEN 83 — Bloodghast
pub(in crate::card::sets) static BLOODGHAST: CardRecord = CardRecord::new(
    CardSet::Zendikar,
    "Bloodghast",
    "63870c81-63bf-4a9a-aeb5-74c6eaded9f1",
    "Daarken",
    CardRules::new_creature(mana_cost!("{B}{B}"), &["Vampire", "Spirit"], 2, 1)
        .with_abilities(&[
            AbilityDef::static_ability(
                "This creature can't block.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                },
            ),
            AbilityDef::static_ability(
                "This creature has haste as long as an opponent has 10 or less life.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                            left: ValueDef::LifeTotal(PlayerRelation::Opponent),
                            comparison: ComparisonDef::LessOrEqual,
                            right: ValueDef::Constant(10),
                        }),
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::add_ability(&abilities::haste()),
                    },
                },
            ),
            AbilityDef::triggered(
                "Landfall — Whenever a land you control enters, you may return this card from your graveyard to the battlefield.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::MoveToZone {
                        object: EffectRecipientDef::Source,
                        zone: ZoneKind::Battlefield,
                        placement: ZonePlacement::Top,
                    },
                },
            )
            .with_source_zones(&[ZoneKind::Graveyard]),
        ]),
);

// ZEN 87 — Disfigure
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISFIGURE: CardRecord = CardRecord::new(
    crate::card::CardSet::Zendikar,
    "Disfigure",
    "b3842ad2-a449-4963-8c96-276554125757",
    "Justin Sweet",
    crate::card::CardRules::unsupported(),
);

// ZEN 90 — Giant Scorpion
pub(in crate::card::sets) static GIANT_SCORPION: CardRecord = CardRecord::new(
    CardSet::Zendikar,
    "Giant Scorpion",
    "c27221df-ec7a-4c51-b3a8-34b65b236b49",
    "Raymond Swanland",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Scorpion"], 1, 3)
        .with_abilities(&[abilities::deathtouch()]),
);

// ZEN 111 — Sorin Markov
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SORIN_MARKOV: CardRecord = CardRecord::new(
    crate::card::CardSet::Zendikar,
    "Sorin Markov",
    "29606aca-f23f-4dfe-b685-2065193109c8",
    "Michael Komarck",
    crate::card::CardRules::unsupported(),
);

// ZEN 114 — Vampire Hexmage
pub(in crate::card::sets) static VAMPIRE_HEXMAGE: CardRecord = CardRecord::new(
    CardSet::Zendikar,
    "Vampire Hexmage",
    "93d2c4d1-6205-404a-b03d-995b90a3a33a",
    "Eric Deschamps",
    // A two-mana first striker that is never a dead card: it answers a
    // planeswalker outright, and everything else it might name is a bonus.
    CardRules::new_creature(mana_cost!("{B}{B}"), &["Vampire", "Shaman"], 2, 1).with_abilities(&[
        abilities::first_strike(),
        AbilityDef::activated_with_targets(
            "Sacrifice this creature: Remove all counters from target permanent.",
            &[AbilityCostDef::SacrificeSource],
            // Any permanent, which is the point: what it takes off a planeswalker is
            // the loyalty, and a planeswalker with no loyalty is put into a graveyard
            // by the ordinary state-based action.
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::RemoveAllCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: None,
            },
        ),
    ]),
);

// ZEN 115 — Vampire Lacerator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VAMPIRE_LACERATOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Zendikar,
    "Vampire Lacerator",
    "114eca6c-76de-4b87-8174-78e2d17ad0e3",
    "Steve Argyle",
    crate::card::CardRules::unsupported(),
);

// ZEN 116 — Vampire Nighthawk
pub(in crate::card::sets) static VAMPIRE_NIGHTHAWK: CardRecord = CardRecord::new(
    CardSet::Zendikar,
    "Vampire Nighthawk",
    "44f19fe3-7a17-4c45-adfa-590f73dfebfa",
    "Jason Chan",
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Vampire", "Shaman"], 2, 3).with_abilities(
        &[
            abilities::flying(),
            abilities::deathtouch(),
            abilities::lifelink(),
        ],
    ),
);

// ZEN 118 — Bladetusk Boar
pub(in crate::card::sets) static BLADETUSK_BOAR: CardRecord = CardRecord::new(
    CardSet::Zendikar,
    "Bladetusk Boar",
    "1558dfaf-15ed-4220-9051-bf0bf442b2e9",
    "Paul Bonner",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Boar"], 3, 2)
        .with_abilities(&[abilities::intimidate()]),
);

// ZEN 119 — Burst Lightning
pub(in crate::card::sets) static BURST_LIGHTNING: CardRecord = CardRecord::new(
    CardSet::Zendikar,
    "Burst Lightning",
    "2dc16614-5cf8-444d-a5ae-cac25018af68",
    "Vance Kovacs",
    // One mana to answer what a one-drop deck leads with, and five to point
    // the same card at anything later.
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[
        abilities::kicker(mana_cost!("{4}")),
        AbilityDef::spell_with_targets(
            "Burst Lightning deals 2 damage to any target. If this spell was kicked, it deals 4 damage instead.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::IfAdditionalCostPaid(&AdditionalCostValueDef::new(
                    crate::AdditionalCostIndex::PRIMARY,
                    ValueDef::Constant(4),
                    ValueDef::Constant(2),
                )),
            },
        ),
    ]),
);

// ZEN 125 — Goblin Bushwhacker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_BUSHWHACKER: CardRecord = CardRecord::new(
    crate::card::CardSet::Zendikar,
    "Goblin Bushwhacker",
    "4085a5bf-a71b-4c73-9b39-0dcc328fe11b",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ZEN 128 — Goblin Shortcutter
pub(in crate::card::sets) static GOBLIN_SHORTCUTTER: CardRecord = CardRecord::new(
    CardSet::Zendikar,
    "Goblin Shortcutter",
    "5daeaa2e-68e5-4f49-9220-58c0c9b1a3d0",
    "Jesper Ejsing",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Scout"], 2, 1).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, target creature can't block this turn.",
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

// ZEN 129 — Goblin War Paint
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_WAR_PAINT: CardRecord = CardRecord::new(
    crate::card::CardSet::Zendikar,
    "Goblin War Paint",
    "4388e57e-0c87-4d66-a862-58261d76c5ac",
    "Austin Hsu",
    crate::card::CardRules::unsupported(),
);

// ZEN 137 — Mark of Mutiny
pub(in crate::card::sets) static MARK_OF_MUTINY: CardRecord = CardRecord::new(
    CardSet::Zendikar,
    "Mark of Mutiny",
    "58a0a019-239d-428e-85a2-e19cae8f4b58",
    "Mike Bierek",
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Gain control of target creature until end of turn. Put a +1/+1 counter on it and untap it. That creature gains haste until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::GainControl {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                duration: ControlDurationDef::UntilEndOfTurn,
                controller: PlayerRefDef::EffectController,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
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
    )),
);

// ZEN 149 — Slaughter Cry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLAUGHTER_CRY: CardRecord = CardRecord::new(
    crate::card::CardSet::Zendikar,
    "Slaughter Cry",
    "c93b0eda-693e-4a17-be1d-1df162702146",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// ZEN 168 — Lotus Cobra
pub(in crate::card::sets) static LOTUS_COBRA: CardRecord = CardRecord::new(
    CardSet::Zendikar,
    "Lotus Cobra",
    "19adde22-e5eb-4815-beb6-c520b3274cc9",
    "Chippy",
    // Two mana that turns every land after it into a Lotus Petal, which is
    // what makes a fetchland a ritual.
    // Not a mana ability: it triggers off a land entering rather than off mana
    // being made (CR 605.1b), so it uses the stack, and the colour is named as
    // it resolves rather than when it triggers.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Snake"], 2, 1).with_ability(
        AbilityDef::triggered(
            "Landfall — Whenever a land you control enters, add one mana of any color.",
            // A land you control, not any land: their fetchland does nothing for her.
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Blue,
                ManaColor::Black,
                ManaColor::Red,
                ManaColor::Green,
            ])),
        ),
    ),
);

// ZEN 192 — Vastwood Gorger
pub(in crate::card::sets) static VASTWOOD_GORGER: CardRecord = CardRecord::new(
    CardSet::Zendikar,
    "Vastwood Gorger",
    "bc5daf96-ceae-4c9a-95cd-f6d706e9b1fa",
    "Kieran Yanner",
    CardRules::new_creature(mana_cost!("{5}{G}"), &["Wurm"], 5, 6),
);

// ZEN 193 — Vines of Vastwood
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VINES_OF_VASTWOOD: CardRecord = CardRecord::new(
    crate::card::CardSet::Zendikar,
    "Vines of Vastwood",
    "e8bd8b10-de86-4bb6-b49f-6ccb5297c81c",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// ZEN 197 — Blazing Torch
pub(in crate::card::sets) static BLAZING_TORCH: CardRecord = CardRecord::new(
    CardSet::Zendikar,
    "Blazing Torch",
    "1e9d1ff2-9ce3-4737-af1d-9fc82e4dffe6",
    "Vance Kovacs",
    CardRules::new_artifact(mana_cost!("{1}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature can't be blocked by Vampires or Zombies.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::Subtype("Vampire"),
                            ObjectPredicateDef::Subtype("Zombie"),
                        ]),
                    )),
                },
            ),
            AbilityDef::static_ability(
                "Equipped creature has \"{T}, Sacrifice Blazing Torch: Blazing Torch deals 2 damage to any target.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&AbilityDef::activated_with_targets(
                        "{T}, Sacrifice Blazing Torch: Blazing Torch deals 2 damage to any target.",
                        &[
                            AbilityCostDef::TapSource,
                            AbilityCostDef::SacrificeObject(ObjectRefDef::AbilityGrantSource),
                        ],
                        &[AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::AnyTarget,
                        )],
                        EffectDef::DealDamageFrom {
                            source: ObjectRefDef::AbilityGrantSource,
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            amount: ValueDef::Constant(2),
                        },
                    )),
                },
            ),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{1}"))],
                "Equip {1} ({1}: Attach to target creature you control. Equip only as a sorcery.)",
            ),
        ]),
);

// ZEN 201 — Expedition Map
pub(in crate::card::sets) static EXPEDITION_MAP: CardRecord = CardRecord::new(
    CardSet::Zendikar,
    "Expedition Map",
    "c55bee97-593f-441f-b96c-a998d5212a55",
    "Franz Vohwinkel",
    // Three mana over two turns for any land in the deck, which is a
    // terrible rate and exactly what a deck built around one land wants.
    CardRules::new_artifact(mana_cost!("{1}")).with_ability(AbilityDef::activated(
        "{2}, {T}, Sacrifice this artifact: Search your library for a land card, reveal it, put \
         it into your hand, then shuffle.",
        &[
            AbilityCostDef::Mana(mana_cost!("{2}")),
            AbilityCostDef::TapSource,
            AbilityCostDef::SacrificeSource,
        ],
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::HasType(CardType::Land),
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
    )),
);

// ZEN 211 — Arid Mesa
pub(in crate::card::sets) static ARID_MESA: CardRecord = CardRecord::new(
    CardSet::Zendikar,
    "Arid Mesa",
    "16c8d2fa-54a7-46e8-980c-905258497c90",
    "Raymond Swanland",
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for a Mountain or Plains card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Mountain, BasicLandType::Plains],
    ),
);

// ZEN 219 — Marsh Flats
pub(in crate::card::sets) static MARSH_FLATS: CardRecord = CardRecord::new(
    CardSet::Zendikar,
    "Marsh Flats",
    "45026d57-0324-4312-8b86-2e7d4f581ee9",
    "Izzy",
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for a Plains or Swamp card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Plains, BasicLandType::Swamp],
    ),
);

// ZEN 220 — Misty Rainforest
pub(in crate::card::sets) static MISTY_RAINFOREST: CardRecord = CardRecord::new(
    CardSet::Zendikar,
    "Misty Rainforest",
    "24a5cc2c-0fbf-4a5f-b175-6e0ffd0d0787",
    "Shelly Wan",
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for a Forest or Island card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Forest, BasicLandType::Island],
    ),
);

// ZEN 223 — Scalding Tarn
pub(in crate::card::sets) static SCALDING_TARN: CardRecord = CardRecord::new(
    CardSet::Zendikar,
    "Scalding Tarn",
    "327cf118-cc92-4073-85d0-94d2a0a6989a",
    "Philip Straub",
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for an Island or Mountain card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Island, BasicLandType::Mountain],
    ),
);

// ZEN 226 — Teetering Peaks
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEETERING_PEAKS: CardRecord = CardRecord::new(
    crate::card::CardSet::Zendikar,
    "Teetering Peaks",
    "e56aca36-bb51-45e3-9ef9-9f9f2aa1e088",
    "Fred Fields",
    crate::card::CardRules::unsupported(),
);

// ZEN 229 — Verdant Catacombs
pub(in crate::card::sets) static VERDANT_CATACOMBS: CardRecord = CardRecord::new(
    CardSet::Zendikar,
    "Verdant Catacombs",
    "7abd2723-2851-4f1a-b2d0-dfcb526472c3",
    "Vance Kovacs",
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for a Swamp or Forest card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Swamp, BasicLandType::Forest],
    ),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &BRAVE_THE_ELEMENTS,
    &DAY_OF_JUDGMENT,
    &JOURNEY_TO_NOWHERE,
    &KOR_SKYFISHER,
    &PILLARFIELD_OX,
    &INTO_THE_ROIL,
    &KRAKEN_HATCHLING,
    &PARALYZING_GRASP,
    &SPELL_PIERCE,
    &WELKIN_TERN,
    &BLOOD_SEEKER,
    &BLOODGHAST,
    &DISFIGURE,
    &GIANT_SCORPION,
    &SORIN_MARKOV,
    &VAMPIRE_HEXMAGE,
    &VAMPIRE_LACERATOR,
    &VAMPIRE_NIGHTHAWK,
    &BLADETUSK_BOAR,
    &BURST_LIGHTNING,
    &GOBLIN_BUSHWHACKER,
    &GOBLIN_SHORTCUTTER,
    &GOBLIN_WAR_PAINT,
    &MARK_OF_MUTINY,
    &SLAUGHTER_CRY,
    &LOTUS_COBRA,
    &VASTWOOD_GORGER,
    &VINES_OF_VASTWOOD,
    &BLAZING_TORCH,
    &EXPEDITION_MAP,
    &ARID_MESA,
    &MARSH_FLATS,
    &MISTY_RAINFOREST,
    &SCALDING_TARN,
    &TEETERING_PEAKS,
    &VERDANT_CATACOMBS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
