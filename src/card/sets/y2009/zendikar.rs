//! Zendikar cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::ControlDurationDef;
use crate::CounterKind;
use crate::KeywordAbility;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AdditionalCostValueDef;
use crate::card::AlternativeCastKindDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BasicLandType;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::ColorChoiceOperationDef;
use crate::card::ComparisonDef;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SubtypeDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::ids::ParentBinding;
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

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "ZEN",
    slug: "zendikar",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// ZEN 4 — Brave the Elements
pub(in crate::card::sets) static BRAVE_THE_ELEMENTS: CardRecord = CardRecord::new(
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
            then: None,
        },
    )),
);

// ZEN 14 — Journey to Nowhere
pub(in crate::card::sets) static JOURNEY_TO_NOWHERE: CardRecord = CardRecord::new(
    "Journey to Nowhere",
    "09cfe585-8a55-4b27-89e0-dfb6946fe1f3",
    "Warren Mahy",
// Two printed abilities rather than the modern "until" wording, so the
    // return is its own leaves-the-battlefield trigger: answering the
    // enchantment in response to the exile leaves the creature where it was.
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this enchantment enters, exile target creature.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::ExileLinkedToSource {
                until_source_leaves: false,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                face_down: false,
                then: None,
            },
        ),
        AbilityDef::triggered(
            "When this enchantment leaves the battlefield, return the exiled card to the battlefield under its owner's control.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                None,
            ),
            EffectDef::ReturnLinkedExiles {
                object: ObjectPredicateDef::Any,
                counters: None,
                zone: ZoneKind::Battlefield,
                grant: None,
                controller: None,
                transformed: false,
            },
        ),
    ]),
);

// ZEN 23 — Kor Skyfisher
pub(in crate::card::sets) static KOR_SKYFISHER: CardRecord = CardRecord::new(
    "Kor Skyfisher",
    "bb2e9465-f5ba-4c7b-9f03-d40dc8394acd",
    "Dan Murayama Scott",
    // The bounce is a cost rather than a bonus, and with nothing else out it
    // has to pick itself up -- which is what pairs it with enters triggers
    // worth replaying rather than with an empty board.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Kor", "Soldier"], 2, 3).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, return a permanent you control to its owner's hand.",
            // Chosen as this resolves rather than targeted, the same way the
            // karoo lands pick the land they give back.
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::PermanentsControlledBy(PlayerRefDef::EffectController),
                exclude: None,
                minimum: 1,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::move_to_zone(
                    EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            }),
        ),
    ]),
);

// ZEN 31 — Pillarfield Ox
pub(in crate::card::sets) static PILLARFIELD_OX: CardRecord = CardRecord::new(
    "Pillarfield Ox",
    "d70a8ff1-f0cf-4aef-ad90-06902f98d434",
    "Andrew Robinson",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Ox"], 2, 4),
);

// ZEN 48 — Into the Roil
pub(in crate::card::sets) static INTO_THE_ROIL: CardRecord = CardRecord::new(
    "Into the Roil",
    "5dba9972-dd8b-407b-9374-a8f0ed1a96db",
    "Kieran Yanner",
// Two mana for tempo, or four for tempo that replaces itself, which is
    // what keeps it playable in a deck that is not otherwise bouncing things.
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{2}{U}{U}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {1}{U} (You may pay an additional {1}{U} as you cast this spell.)"),
            EffectDef::None,
        ),
        AbilityDef::spell_with_targets(
            "Return target nonland permanent to its owner's hand. If this spell was kicked, draw a card.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
            )],
            EffectDef::Sequence(&[
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
                // Asked as this resolves, so a kicked spell still draws even
                // when its target has already left the battlefield.
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceCastWith(
                        AlternativeCastKindDef::Kicked,
                    ),
                    then: &EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                },
            ]),
        ),
    ]),
);

// ZEN 50 — Kraken Hatchling
pub(in crate::card::sets) static KRAKEN_HATCHLING: CardRecord = CardRecord::new(
    "Kraken Hatchling",
    "45d100a3-93f2-428c-8f54-8807e71f2638",
    "Jason Felix",
    CardRules::new_creature(mana_cost!("{U}"), &["Kraken"], 0, 4),
);

// ZEN 58 — Paralyzing Grasp
pub(in crate::card::sets) static PARALYZING_GRASP: CardRecord = CardRecord::new(
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

// ZEN 61 — Rite of Replication
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RITE_OF_REPLICATION: CardRecord = CardRecord::new(
    "Rite of Replication",
    "4530fe45-8a3d-48e9-a7a5-abf8fb1485e3",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// ZEN 67 — Spell Pierce
pub(in crate::card::sets) static SPELL_PIERCE: CardRecord = CardRecord::new(
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
        abilities::counter_target_unless_paid(&[CostDef::GenericMana(ValueDef::Constant(2))]),
    )),
);

// ZEN 76 — Welkin Tern
pub(in crate::card::sets) static WELKIN_TERN: CardRecord = CardRecord::new(
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
pub(in crate::card::sets) static BLOOD_SEEKER: CardRecord = CardRecord::new(
    "Blood Seeker",
    "d1abc9e8-9ecf-4665-9ea5-ee18ab83c148",
    "Greg Staples",
CardRules::new_creature(mana_cost!("{1}{B}"), &["Vampire", "Shaman"], 1, 1).with_ability(
        AbilityDef::triggered(
            "Whenever a creature an opponent controls enters, you may have that player lose 1 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::LoseLife {
                    recipient: EffectRecipientDef::ControllerOfTriggeringObject,
                    amount: ValueDef::Constant(1),
                },
            },
        ),
    ),
);

// ZEN 83 — Bloodghast
pub(in crate::card::sets) static BLOODGHAST: CardRecord = CardRecord::new(
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
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::Source,
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                },
            )
            .with_source_zones(&[ZoneKind::Graveyard]),
        ]),
);

// ZEN 87 — Disfigure
pub(in crate::card::sets) static DISFIGURE: CardRecord = CardRecord::new(
    "Disfigure",
    "b3842ad2-a449-4963-8c96-276554125757",
    "Justin Sweet",
    // A one-mana answer that shrinks rather than destroys, so it also wins a
    // combat outright instead of only trading after damage.
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets -2/-2 until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-2),
                ValueDef::Constant(-2),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// ZEN 89 — Gatekeeper of Malakir
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GATEKEEPER_OF_MALAKIR: CardRecord = CardRecord::new(
    "Gatekeeper of Malakir",
    "71db3698-a45c-4eaf-87e6-30502c0c10f4",
    "Karl Kopinski",
    crate::card::CardRules::unsupported(),
);

// ZEN 90 — Giant Scorpion
pub(in crate::card::sets) static GIANT_SCORPION: CardRecord = CardRecord::new(
    "Giant Scorpion",
    "c27221df-ec7a-4c51-b3a8-34b65b236b49",
    "Raymond Swanland",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Scorpion"], 1, 3)
        .with_abilities(&[abilities::deathtouch()]),
);

// ZEN 111 — Sorin Markov
// Audit: unsupported — Needs control of another player during that player's next turn.
pub(in crate::card::sets) static SORIN_MARKOV: CardRecord = CardRecord::new(
    "Sorin Markov",
    "29606aca-f23f-4dfe-b685-2065193109c8",
    "Michael Komarck",
    crate::card::CardRules::unsupported(),
);

// ZEN 114 — Vampire Hexmage
pub(in crate::card::sets) static VAMPIRE_HEXMAGE: CardRecord = CardRecord::new(
    "Vampire Hexmage",
    "93d2c4d1-6205-404a-b03d-995b90a3a33a",
    "Eric Deschamps",
    // A two-mana first striker that is never a dead card: it answers a
    // planeswalker outright, and everything else it might name is a bonus.
    CardRules::new_creature(mana_cost!("{B}{B}"), &["Vampire", "Shaman"], 2, 1).with_abilities(&[
        abilities::first_strike(),
        AbilityDef::activated_with_targets(
            "Sacrifice this creature: Remove all counters from target permanent.",
            &[CostDef::SacrificeSource],
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
pub(in crate::card::sets) static VAMPIRE_LACERATOR: CardRecord = CardRecord::new(
    "Vampire Lacerator",
    "114eca6c-76de-4b87-8174-78e2d17ad0e3",
    "Steve Argyle",
    // A 2/2 for one that bills you a life a turn until the race is won,
    // which is a cost an aggressive deck expects to stop paying.
    CardRules::new_creature(mana_cost!("{B}"), &["Vampire", "Warrior"], 2, 2).with_ability(
        AbilityDef::triggered(
            "At the beginning of your upkeep, you lose 1 life unless an opponent has 10 or less \
             life.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            // The trigger always goes on the stack; the "unless" is read as
            // it resolves, so a life total that changed in response counts.
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::LifeTotal(PlayerRelation::Opponent),
                    comparison: ComparisonDef::Greater,
                    right: ValueDef::Constant(10),
                }),
                then: &EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            },
        ),
    ),
);

// ZEN 116 — Vampire Nighthawk
pub(in crate::card::sets) static VAMPIRE_NIGHTHAWK: CardRecord = CardRecord::new(
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
    "Bladetusk Boar",
    "1558dfaf-15ed-4220-9051-bf0bf442b2e9",
    "Paul Bonner",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Boar"], 3, 2)
        .with_abilities(&[abilities::intimidate()]),
);

// ZEN 119 — Burst Lightning
pub(in crate::card::sets) static BURST_LIGHTNING: CardRecord = CardRecord::new(
    "Burst Lightning",
    "2dc16614-5cf8-444d-a5ae-cac25018af68",
    "Vance Kovacs",
// One mana to answer what a one-drop deck leads with, and five to point
    // the same card at anything later.
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[
        abilities::kicker(
            &[CostDef::Mana(mana_cost!("{4}"))],
        ),
        AbilityDef::spell_with_targets(
            "Burst Lightning deals 2 damage to any target. If this spell was kicked, it deals 4 damage instead.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::IfAdditionalCostPaid(&AdditionalCostValueDef::new(
                    crate::AdditionalCostIndex::PRIMARY,
                    ValueDef::Constant(4),
                    ValueDef::Constant(2),
                )),
            ),
        ),
    ]),
);

// ZEN 125 — Goblin Bushwhacker
pub(in crate::card::sets) static GOBLIN_BUSHWHACKER: CardRecord = CardRecord::new(
    "Goblin Bushwhacker",
    "4085a5bf-a71b-4c73-9b39-0dcc328fe11b",
    "Mark Tedin",
// Unkicked it is a one-mana body; kicked it is the second half of an
    // Empty the Warrens turn, which is the only reason the card sees play.
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin", "Warrior"], 1, 1).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{R}{R}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {R} (You may pay an additional {R} as you cast this spell.)"),
            EffectDef::None,
        ),
        AbilityDef::triggered_if(
            "When this creature enters, if it was kicked, creatures you control get +1/+0 and gain haste until end of turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
            // The Bushwhacker pumps itself too, and the haste is what lets
            // the whole board attack the turn it lands.
            EffectDef::Sequence(&[
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
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::add_ability(&abilities::haste()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ]),
);

// ZEN 128 — Goblin Shortcutter
pub(in crate::card::sets) static GOBLIN_SHORTCUTTER: CardRecord = CardRecord::new(
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
pub(in crate::card::sets) static GOBLIN_WAR_PAINT: CardRecord = CardRecord::new(
    "Goblin War Paint",
    "4388e57e-0c87-4d66-a862-58261d76c5ac",
    "Austin Hsu",
    // Haste on an Aura, which only reads as a bonus on the turn the
    // creature arrived.
    CardRules::new_enchantment(mana_cost!("{1}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+2 and has haste.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&abilities::haste()),
                    ]),
                },
            ),
        ]),
);

// ZEN 137 — Mark of Mutiny
pub(in crate::card::sets) static MARK_OF_MUTINY: CardRecord = CardRecord::new(
    "Mark of Mutiny",
    "58a0a019-239d-428e-85a2-e19cae8f4b58",
    "Mike Bierek",
CardRules::new_sorcery(mana_cost!("{2}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Gain control of target creature until end of turn. Put a +1/+1 counter on it and untap it. That creature gains haste until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::gain_control(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                PlayerRefDef::EffectController,
                ControlDurationDef::UntilEndOfTurn,
            ),
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
pub(in crate::card::sets) static SLAUGHTER_CRY: CardRecord = CardRecord::new(
    "Slaughter Cry",
    "c93b0eda-693e-4a17-be1d-1df162702146",
    "Matt Cavotta",
    CardRules::new_instant(mana_cost!("{2}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +3/+0 and gains first strike until end of turn.",
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
                AppliedEffectDef::add_ability(&abilities::first_strike()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// ZEN 168 — Lotus Cobra
pub(in crate::card::sets) static LOTUS_COBRA: CardRecord = CardRecord::new(
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

// ZEN 178 — Rampaging Baloths
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAMPAGING_BALOTHS: CardRecord = CardRecord::new(
    "Rampaging Baloths",
    "66ae703d-b133-4749-9d38-216abe6c6647",
    "Steve Prescott",
    crate::card::CardRules::unsupported(),
);

// ZEN 192 — Vastwood Gorger
pub(in crate::card::sets) static VASTWOOD_GORGER: CardRecord = CardRecord::new(
    "Vastwood Gorger",
    "bc5daf96-ceae-4c9a-95cd-f6d706e9b1fa",
    "Kieran Yanner",
    CardRules::new_creature(mana_cost!("{5}{G}"), &["Wurm"], 5, 6),
);

// ZEN 193 — Vines of Vastwood
pub(in crate::card::sets) static VINES_OF_VASTWOOD: CardRecord = CardRecord::new(
    "Vines of Vastwood",
    "e8bd8b10-de86-4bb6-b49f-6ccb5297c81c",
    "Christopher Moeller",
    // One mana answers a removal spell and two mana ends a race, which is
    // why the same card is live at both ends of the game.
    CardRules::new_instant(mana_cost!("{G}")).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{G}{G}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {G} (You may pay an additional {G} as you cast this spell.)"),
            EffectDef::None,
        ),
        AbilityDef::spell_with_targets(
            "Target creature can't be the target of spells or abilities your opponents control \
             this turn. If this spell was kicked, that creature gets +4/+4 until end of turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                // "Can't be the target of spells or abilities your opponents
                // control" is hexproof, so the shielded half is granted
                // whether or not the kicker was paid.
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
                    then: &EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(4),
                            ValueDef::Constant(4),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                },
            ]),
        ),
    ]),
);

// ZEN 195 — Adventuring Gear
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ADVENTURING_GEAR: CardRecord = CardRecord::new(
    "Adventuring Gear",
    "3aa395f2-656e-4bf3-bd9b-6240bd3e2774",
    "Howard Lyon",
    crate::card::CardRules::unsupported(),
);

// ZEN 197 — Blazing Torch
pub(in crate::card::sets) static BLAZING_TORCH: CardRecord = CardRecord::new(
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
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vampire")),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Zombie")),
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
                            CostDef::TapSource,
                            CostDef::SacrificeObject(ObjectRefDef::AbilityGrantSource),
                        ],
                        &[AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::AnyTarget,
                        )],
                        EffectDef::damage_from(
                            ObjectRefDef::AbilityGrantSource,
                            EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            ValueDef::Constant(2),
                        ),
                    )),
                },
            ),
            abilities::equip(
                &[CostDef::Mana(mana_cost!("{1}"))],
                "Equip {1} ({1}: Attach to target creature you control. Equip only as a sorcery.)",
            ),
        ]),
);

// ZEN 201 — Expedition Map
pub(in crate::card::sets) static EXPEDITION_MAP: CardRecord = CardRecord::new(
    "Expedition Map",
    "c55bee97-593f-441f-b96c-a998d5212a55",
    "Franz Vohwinkel",
    // Three mana over two turns for any land in the deck, which is a
    // terrible rate and exactly what a deck built around one land wants.
    CardRules::new_artifact(mana_cost!("{1}")).with_ability(AbilityDef::activated(
        "{2}, {T}, Sacrifice this artifact: Search your library for a land card, reveal it, put \
         it into your hand, then shuffle.",
        &[
            CostDef::Mana(mana_cost!("{2}")),
            CostDef::TapSource,
            CostDef::SacrificeSource,
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
    "Scalding Tarn",
    "327cf118-cc92-4073-85d0-94d2a0a6989a",
    "Philip Straub",
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for an Island or Mountain card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Island, BasicLandType::Mountain],
    ),
);

// ZEN 226 — Teetering Peaks
pub(in crate::card::sets) static TEETERING_PEAKS: CardRecord = CardRecord::new(
    "Teetering Peaks",
    "e56aca36-bb51-45e3-9ef9-9f9f2aa1e088",
    "Fred Fields",
    // Coming in tapped is the whole cost, and an aggressive deck pays it
    // gladly: the land is a burn spell that also casts spells later.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped.",
            ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
        ),
        abilities::enters_trigger_with_targets(
            "When this land enters, target creature gets +2/+0 until end of turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {R}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
        ),
    ]),
);

// ZEN 229 — Verdant Catacombs
pub(in crate::card::sets) static VERDANT_CATACOMBS: CardRecord = CardRecord::new(
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
    &RITE_OF_REPLICATION,
    &SPELL_PIERCE,
    &WELKIN_TERN,
    &BLOOD_SEEKER,
    &BLOODGHAST,
    &DISFIGURE,
    &GATEKEEPER_OF_MALAKIR,
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
    &RAMPAGING_BALOTHS,
    &VASTWOOD_GORGER,
    &VINES_OF_VASTWOOD,
    &ADVENTURING_GEAR,
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
