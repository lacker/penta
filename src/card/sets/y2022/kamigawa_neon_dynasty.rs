//! Kamigawa: Neon Dynasty attachment edge cases.

use crate::card::ActivationTimingDef;
use crate::card::ComparisonDef;
use crate::card::CopyStackObjectDef;
use crate::card::ExilePlayDurationDef;
use crate::card::PlayActionMatcherDef;
use crate::card::PlayRestrictionDef;
use crate::card::SubtypeDef;
use crate::card::TopOfLibraryCostDef;
use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BasicLandType;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::BattlefieldEntryScalarChoiceDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::ColorSet;
use crate::card::CopyAbilityDef;
use crate::card::CopyExceptionsDef;
use crate::card::CostAdjustmentDef;
use crate::card::CostAmountDef;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::CreatedTokensDef;
use crate::card::CreatureTypeSetDef;
use crate::card::DiscardSelectionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::InstalledTriggerDef;
use crate::card::ManaColor;
use crate::card::ManaTypeDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ReplacementChoiceDef;
use crate::card::ReplacementEffectDef;
use crate::card::ReplacementEventDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SpellCostConditionDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::card::tokens;
use crate::ids::ParentBinding;
use crate::ids::TargetIndex;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "NEO",
    slug: "kamigawa-neon-dynasty",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const SAMURAI_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Samurai"], &[ManaColor::White], 2, 2)
        .with_abilities(&[abilities::vigilance()])
        .with_art(CardArt::new(
            "f68e5337-6e44-4f8f-a102-2f97b433beea",
            "Gaboleps",
        ));

const TREASURE_TOKEN: TokenCharacteristics = tokens::treasure().with_art(CardArt::new(
    "6911181d-573b-41eb-96a4-799c96e008fc",
    "Yeong-Hao Han",
));

// NEO 17 — Imperial Oath
pub(in crate::card::sets) static IMPERIAL_OATH: CardRecord = CardRecord::new(
    "Imperial Oath",
    "3d6750dd-2303-493b-885d-1bfb5787b16c",
    "Nicholas Elias",
    // Six power that can attack and still block, plus three cards deep of
    // smoothing -- a limited finisher rather than anything a cube wants.
    CardRules::new_sorcery(mana_cost!("{5}{W}")).with_ability(AbilityDef::spell(
        "Create three 2/2 white Samurai creature tokens with vigilance. Scry 3.",
        EffectDef::Sequence(&[
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(SAMURAI_TOKEN)).with_amount(3),
            ),
            abilities::scry(ValueDef::Constant(3)),
        ]),
    )),
);

// NEO 26 — Lion Sash
pub(in crate::card::sets) static LION_SASH: CardRecord = CardRecord::new(
    "Lion Sash",
    "3e1766e9-2fa7-4446-a255-7beea1467ece",
    "Yongjae Choi",
    // Graveyard hate that grows into a threat, and reconfigure means the
    // two halves are the same card rather than a choice made on turn two.
    CardRules::new_artifact_creature(mana_cost!("{1}{W}"), &["Equipment", "Cat"], 1, 1)
        .with_abilities(&[
            AbilityDef::activated_with_targets(
                "{W}: Exile target card from a graveyard. If it was a permanent card, put a \
                 +1/+1 counter on this permanent.",
                &[CostDef::Mana(mana_cost!("{W}"))],
                // A card in anybody's graveyard, which is what "from a graveyard" means:
                // yours as readily as theirs.
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Any,
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: None,
                    },
                )],
                // The counter is decided before the card moves: a card in exile is no
                // longer where the target slot is looking.
                EffectDef::Sequence(&[
                    EffectDef::IfCondition {
                        // A permanent card is one of the types that can stay on the battlefield.
                        // Asked of the target while it is still in the graveyard, which is what
                        // "if it was" means once it has been exiled.
                        condition: &TriggerConditionDef::TargetMatches {
                            slot: TargetIndex::PRIMARY,
                            object: ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasType(CardType::Enchantment),
                                ObjectPredicateDef::HasType(CardType::Land),
                                ObjectPredicateDef::HasType(CardType::Planeswalker),
                            ]),
                        },
                        then: &EffectDef::AddCounters {
                            object: EffectRecipientDef::Source,
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(1),
                        },
                    },
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Exile,
                        ZonePlacement::Top,
                    ),
                ]),
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1 for each +1/+1 counter on this Equipment.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::CountersOnSource(CounterKind::PlusOnePlusOne),
                        ValueDef::CountersOnSource(CounterKind::PlusOnePlusOne),
                    ),
                },
            ),
            abilities::reconfigure(
                &[CostDef::Mana(mana_cost!("{2}"))],
                "Reconfigure {2} ({2}: Attach to target creature you control; or unattach from a \
                 creature. Reconfigure only as a sorcery. While attached, this isn't a creature.)",
            ),
        ]),
);

// NEO 40 — Touch the Spirit Realm
static AN_ARTIFACT_OR_CREATURE: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasType(CardType::Artifact),
    ObjectPredicateDef::HasType(CardType::Creature),
]);

pub(in crate::card::sets) static TOUCH_THE_SPIRIT_REALM: CardRecord = CardRecord::new(
    "Touch the Spirit Realm",
    "e16ab44e-4257-4c0c-b705-8ac1e9c1d835",
    "Marta Nael",
// Three mana to answer something for as long as the enchantment lives,
    // or two from hand to blink one of yours -- which is why it is never
    // quite dead.
    CardRules::new_enchantment(mana_cost!("{2}{W}")).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this enchantment enters, exile up to one target artifact or creature until this \
             enchantment leaves the battlefield.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: AN_ARTIFACT_OR_CREATURE,
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                1,
            )],
            EffectDef::Sequence(&[
                EffectDef::ExileLinkedToSource {
                    until_source_leaves: true,
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    face_down: false,
                    then: None,
                },
                // "Until this enchantment leaves the battlefield" is one printed clause, so
                // the return rides on a delayed trigger rather than appearing as a second
                // ability the card does not print.
                EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                    "When this enchantment leaves the battlefield, return the exiled card to the battlefield \
                     under its owner's control.",
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
                ))),
            ]),
        ),
        AbilityDef::activated_with_targets(
            "Channel — {1}{W}, Discard this card: Exile target artifact or creature. Return it to \
             the battlefield under its owner's control at the beginning of the next end step.",
            &[CostDef::Mana(mana_cost!("{1}{W}")), CostDef::DiscardSource],
            &[AbilityTargetDef::exactly_one_permanent(
                AN_ARTIFACT_OR_CREATURE,
            )],
            abilities::exile_until_next_end_step(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
        )
        .with_source_zones(&[ZoneKind::Hand]),
    ]),
);

// NEO 42 — The Wandering Emperor
pub(in crate::card::sets) static THE_WANDERING_EMPEROR: CardRecord = CardRecord::new(
    "The Wandering Emperor",
    "fab2d8a9-ab4c-4225-a570-22636293c17d",
    "Tommy Arnold",
// A planeswalker you cast on their turn: she answers an attacker, makes
    // a blocker, or wins a fight, and she does it before they can respond by
    // killing her.
    CardRules::new_planeswalker(mana_cost!("{2}{W}{W}"), &["The Wandering Emperor"], 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flash(),
            AbilityDef::static_ability(
                "As long as The Wandering Emperor entered this turn, you may activate her loyalty \
                 abilities any time you could cast an instant.",
                EffectDef::IfCondition {
                    // "As long as The Wandering Emperor entered this turn": the permission is
                    // hers for the turn she lands and no longer, which is what makes flashing
                    // her in at the end of a turn a plan rather than a waste.
                    condition: &TriggerConditionDef::SourceMatches {
                        object: ObjectPredicateDef::EnteredThisTurn,
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::MayActivateLoyaltyAnyTime),
                    },
                },
            ),
            AbilityDef::activated_with_targets(
                "+1: Put a +1/+1 counter on up to one target creature. It gains first strike until end of \
                 turn.",
                &[CostDef::Loyalty(1)],
                // "Up to one target creature", which is what keeps the plus activatable on
                // an empty board.
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    1,
                )],
                EffectDef::Sequence(&[
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            ),
            AbilityDef::activated(
                "−1: Create a 2/2 white Samurai creature token with vigilance.",
                &[CostDef::Loyalty(-1)],
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    SAMURAI_TOKEN,
                ))),
            ),
            AbilityDef::activated_with_targets(
                "−2: Exile target tapped creature. You gain 2 life.",
                &[CostDef::Loyalty(-2)],
                // A tapped creature: the minus answers an attacker that has already
                // committed, which is the half of removal flash was made for.
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Tapped,
                    ]),
                )],
                EffectDef::Sequence(&[
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Exile,
                        ZonePlacement::Top,
                    ),
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(2),
                    },
                ]),
            ),
        ]),
);

// NEO 61 — March of Swirling Mist
// Audit: unsupported — EffectDef::PhaseOut and X-bounded targets are available, but casting has
// no cost adjustment that can read the variable number of blue hand cards exiled as an additional
// cost. Cost reductions are calculated before the payment is committed, and CostAmountDef only
// accepts fixed mana or existing ValueDef values, none of which represents that selected payment.
pub(in crate::card::sets) static MARCH_OF_SWIRLING_MIST_61: CardRecord = CardRecord::new(
    "March of Swirling Mist",
    "100171d8-7436-44c8-b4cb-0101ffa05c25",
    "Iris Compiet",
    crate::card::CardRules::unsupported(),
);

// NEO 63 — Mirrorshell Crab
pub(in crate::card::sets) static MIRRORSHELL_CRAB: CardRecord = CardRecord::new(
    "Mirrorshell Crab",
    "0394c8df-2e8a-4477-93b7-569934d7b936",
    "Cristi Balanescu",
    // Seven mana is the price of the body nobody pays. The card is really a
    // three-mana soft counter that stops being dead in the games that go long
    // enough to cast it.
    CardRules::new_artifact_creature(mana_cost!("{5}{U}{U}"), &["Crab"], 5, 7).with_abilities(&[
        abilities::ward(
            &[CostDef::Mana(crate::ManaCost::new(3, 0))],
            "Ward {3} (Whenever this creature becomes the target of a spell or ability an \
             opponent controls, counter it unless that player pays {3}.)",
        ),
        AbilityDef::activated_with_targets(
            "Channel — {2}{U}, Discard this card: Counter target spell or ability unless its \
             controller pays {3}.",
            &[CostDef::Mana(mana_cost!("{2}{U}")), CostDef::DiscardSource],
            // "Spell or ability" is everything on the stack, so the predicate
            // names the zone and nothing else: a triggered ability is as legal
            // a target as a spell, and the Crab's own controller is too.
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            abilities::counter_target_unless_paid(&[CostDef::GenericMana(ValueDef::Constant(3))]),
        )
        .with_source_zones(&[ZoneKind::Hand]),
    ]),
);

// NEO 67 — Moon-Circuit Hacker
pub(in crate::card::sets) static MOON_CIRCUIT_HACKER: CardRecord = CardRecord::new(
    "Moon-Circuit Hacker",
    "c6e466d1-943d-41e6-a47d-c9d951ca4262",
    "Tia Masic",
    // One blue mana for a 2/1 that arrives attacking and draws a card. The
    // discard is what the turn it lands is exempt from, so the reward for
    // ninjutsu is a clean card and the reward for leaving it out is a loot.
    CardRules::new_enchantment_creature(mana_cost!("{1}{U}"), &["Human", "Ninja"], 2, 1)
        .with_abilities(&[
            abilities::ninjutsu!(
                "Ninjutsu {U} ({U}, Return an unblocked attacker you control to hand: Put this \
                 card onto the battlefield from your hand tapped and attacking.)",
                &[CostDef::Mana(mana_cost!("{U}"))],
            ),
            AbilityDef::triggered(
                "Whenever this creature deals combat damage to a player, you may draw a card. If \
                 you do, discard a card unless this creature entered this turn.",
                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::Sequence(&[
                        EffectDef::DrawCards {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(1),
                        },
                        EffectDef::IfCondition {
                            condition: &TriggerConditionDef::Not(
                                &TriggerConditionDef::SourceMatches {
                                    object: ObjectPredicateDef::EnteredThisTurn,
                                },
                            ),
                            then: &EffectDef::Discard {
                                recipient: EffectRecipientDef::Controller,
                                amount: ValueDef::Constant(1),
                                selection: DiscardSelectionDef::RecipientChooses,
                                then: None,
                            },
                        },
                    ]),
                },
            ),
        ]),
);

// NEO 69 — Moonsnare Prototype
// Audit: unsupported — Its mana activation requires a selected artifact-or-creature tap in addition to tapping the source, which the mana-ability planner rejects.
pub(in crate::card::sets) static MOONSNARE_PROTOTYPE_69: CardRecord = CardRecord::new(
    "Moonsnare Prototype",
    "9d8bc0e9-a536-4bca-92a6-8dca85e1e984",
    "Fariba Khamseh",
    crate::card::CardRules::unsupported(),
);

// NEO 82 — Tameshi, Reality Architect
// Audit: unsupported — The zone-change stream cannot aggregate simultaneous returns to hand into one event, and the activated-cost planner cannot return a selected land as this activation's cost.
pub(in crate::card::sets) static TAMESHI_REALITY_ARCHITECT_82: CardRecord = CardRecord::new(
    "Tameshi, Reality Architect",
    "26594b52-3e9c-4cde-88df-1f4e9e16676e",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// NEO 91 — Clawing Torment
pub(in crate::card::sets) static CLAWING_TORMENT: CardRecord = CardRecord::new(
    "Clawing Torment",
    "621fce96-5933-4e2b-98ec-2589940e24cb",
    "Rovina Cai",
    // One mana that shrinks a creature or just drains, and either way it
    // closes the game a life at a time.
    CardRules::new_enchantment(mana_cost!("{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell(
                "Enchant artifact or creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                )],
            ),
            AbilityDef::static_ability(
                "As long as enchanted permanent is a creature, it gets -1/-1 and can't block.",
                // Written as the printed conditional rather than applied
                // unconditionally: this can land on an artifact, and the
                // clause only starts once that artifact is also a creature.
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::AttachedPermanentMatches {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::modify_power_toughness(
                                ValueDef::Constant(-1),
                                ValueDef::Constant(-1),
                            ),
                            AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                        ]),
                    },
                },
            ),
            AbilityDef::static_ability(
                "Enchanted permanent has \"At the beginning of your upkeep, you lose 1 life.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    // Granted to the permanent, so "your" upkeep is its
                    // controller's -- this drains whoever it landed on.
                    effect: AppliedEffectDef::add_ability(&AbilityDef::triggered(
                        "At the beginning of your upkeep, you lose 1 life.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::Upkeep,
                            player: PlayerRelation::You,
                        },
                        EffectDef::LoseLife {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(1),
                        },
                    )),
                },
            ),
        ]),
);

// NEO 117 — Okiba Reckoner Raid // Nezumi Road Captain
// Audit: unsupported — Needs a Saga clause shared across chapters. Every piece is present -- saga_chapter, exile_and_return_transformed, and the transforming two-face record -- but "I, II" is one printed clause on two chapters, and a chapter is one ability per lore counter. Repeating the text prints the line twice and leaving the second empty is rejected outright.
pub(in crate::card::sets) static OKIBA_RECKONER_RAID: CardRecord = CardRecord::new(
    "Okiba Reckoner Raid",
    "4f0582b4-d951-4450-b158-4a34109e48cd",
    "Victor Adame Minguez",
    crate::card::CardRules::unsupported(),
);

// NEO 128 — Virus Beetle
pub(in crate::card::sets) static VIRUS_BEETLE: CardRecord = CardRecord::new(
    "Virus Beetle",
    "488ee202-0d28-4cc0-8a7d-644d9878e952",
    "Dan Murayama Scott",
    CardRules::new_artifact_creature(mana_cost!("{1}{B}"), &["Insect"], 1, 1).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, each opponent discards a card.",
            EffectDef::Discard {
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ),
    ]),
);

// NEO 136 — Crackling Emergence
static EMERGENCE_REPLACEMENT: AbilityDef = AbilityDef::replacement_for(
    "If enchanted land would be destroyed, instead sacrifice this \
     Aura and that land gains indestructible until end of turn.",
    ReplacementEventDef::WouldBeDestroyed {
        object: ObjectPredicateDef::AttachedToSource,
    },
    ReplacementEffectDef::Sequence(&[
        ReplacementEffectDef::ReplaceEventWithNothing,
        ReplacementEffectDef::Perform(&EffectDef::Sequence(&[
            EffectDef::sacrifice_yours(EffectRecipientDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::TriggeringObject,
                effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ])),
    ]),
);

pub(in crate::card::sets) static CRACKLING_EMERGENCE: CardRecord = CardRecord::new(
    "Crackling Emergence",
    "6f77f987-ebb7-4105-a67c-0987f50fc676",
    "Jason Kang",
    CardRules::new_enchantment(mana_cost!("{1}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_land_you_control(),
            AbilityDef::static_ability(
                "Enchanted land is a 3/3 red Spirit creature with haste. It's still a land.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                        AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&[
                            "Spirit",
                        ])),
                        AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Red])),
                        AppliedEffectDef::set_base_power_toughness(
                            ValueDef::Constant(3),
                            ValueDef::Constant(3),
                        ),
                        AppliedEffectDef::add_ability(&abilities::haste()),
                    ]),
                },
            ),
            EMERGENCE_REPLACEMENT,
        ]),
);

// NEO 138 — Experimental Synthesizer
pub(in crate::card::sets) static EXPERIMENTAL_SYNTHESIZER_138: CardRecord = CardRecord::new(
    "Experimental Synthesizer",
    "c47931c9-685d-4b83-8299-bc347224b4e8",
    "Yeong-Hao Han",
    CardRules::new_artifact(mana_cost!("{R}")).with_abilities(&[
AbilityDef::triggered("When this artifact enters or leaves the battlefield, exile the top card of your library. Until end of turn, you may play that card.", TriggerEventDef::AnyOf(&[TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)), TriggerEventDef::zone_changed(ObjectPredicateDef::Source, Some(ZoneKind::Battlefield), None)]), EffectDef::ExileTopOfLibraryToPlay { player: EffectRecipientDef::Controller, amount: ValueDef::Constant(1), free: false, face_down: false, duration: ExilePlayDurationDef::ThisTurn, spend_any_color: false, play_condition: None, cast_only: false }),
AbilityDef::activated("{2}{R}, Sacrifice this artifact: Create a 2/2 white Samurai creature token with vigilance. Activate only as a sorcery.", &[CostDef::Mana(mana_cost!("{2}{R}")), CostDef::SacrificeSource], EffectDef::create_creature_token(&["Samurai"], &[ManaColor::White], 2, 2).with_abilities(&[abilities::vigilance()])).with_activation_timing(ActivationTimingDef::SorcerySpeed)
]),
);

// NEO 145 — Goro-Goro, Disciple of Ryusei
pub(in crate::card::sets) static GORO_GORO_DISCIPLE_OF_RYUSEI_145: CardRecord = CardRecord::new(
    "Goro-Goro, Disciple of Ryusei",
    "1ca736c7-35a9-48c7-b5a9-69b2a6e33ad0",
    "Mike Jordana",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Samurai"], 2, 2).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::activated("{R}: Creatures you control gain haste until end of turn.", &[CostDef::Mana(mana_cost!("{R}"))], EffectDef::Apply { recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::You), effect: AppliedEffectDef::add_ability(&abilities::haste()), duration: ResolvedEffectDurationDef::UntilEndOfTurn }),
AbilityDef::activated("{3}{R}{R}: Create a 5/5 red Dragon Spirit creature token with flying. Activate only if you control an attacking modified creature. (Equipment, Auras you control, and counters are modifications.)", &[CostDef::Mana(mana_cost!("{3}{R}{R}"))], EffectDef::create_creature_token(&["Dragon", "Spirit"], &[ManaColor::Red], 5, 5).with_abilities(&[abilities::flying()])).with_activation_condition(&TriggerConditionDef::AnyOf(&[TriggerConditionDef::ObjectCount { query: ObjectQueryDef::matching(ObjectPredicateDef::All(&[ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Attacking, ObjectPredicateDef::ControlledBy(PlayerRelation::You)]), ObjectPredicateDef::HasAnyCounter]), &[ZoneKind::Battlefield], PlayerRelation::You), comparison: ComparisonDef::GreaterOrEqual, amount: 1 }, TriggerConditionDef::ObjectCount { query: ObjectQueryDef::matching(ObjectPredicateDef::All(&[ObjectPredicateDef::Subtype(SubtypeDef::Literal("Aura")), ObjectPredicateDef::AttachedTo(&ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Attacking, ObjectPredicateDef::ControlledBy(PlayerRelation::You)]))]), &[ZoneKind::Battlefield], PlayerRelation::You), comparison: ComparisonDef::GreaterOrEqual, amount: 1 }, TriggerConditionDef::ObjectCount { query: ObjectQueryDef::matching(ObjectPredicateDef::All(&[ObjectPredicateDef::Subtype(SubtypeDef::Literal("Equipment")), ObjectPredicateDef::AttachedTo(&ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Attacking, ObjectPredicateDef::ControlledBy(PlayerRelation::You)]))]), &[ZoneKind::Battlefield], PlayerRelation::Any), comparison: ComparisonDef::GreaterOrEqual, amount: 1 }]))
]),
);

// NEO 148 — Ironhoof Boar
pub(in crate::card::sets) static IRONHOOF_BOAR: CardRecord = CardRecord::new(
    "Ironhoof Boar",
    "73abe574-6fb8-4809-9c18-0cf989f986f5",
    "Antonio José Manzanedo",
    // Six mana for the body or two for a trick: channel is what keeps a
    // top-heavy creature from being a dead card in the early turns.
    CardRules::new_artifact_creature(mana_cost!("{5}{R}"), &["Boar"], 5, 4).with_abilities(&[
        abilities::trample(),
        abilities::haste(),
        AbilityDef::activated_with_targets(
            "Channel — {1}{R}, Discard this card: Target creature gets +3/+1 and gains trample \
             until end of turn.",
            &[CostDef::Mana(mana_cost!("{1}{R}")), CostDef::DiscardSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(1),
                    ),
                    AppliedEffectDef::add_ability(&abilities::trample()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        // Activated from hand, which is the only place a card can be
        // discarded from.
        .with_source_zones(&[ZoneKind::Hand]),
    ]),
);

// NEO 154 — March of Reckless Joy
// Audit: unsupported — The casting planner cannot select an arbitrary red-card exile group and reduce this spell's generic cost by two per card. The exile-play permission also cannot limit the group to two plays.
pub(in crate::card::sets) static MARCH_OF_RECKLESS_JOY_154: CardRecord = CardRecord::new(
    "March of Reckless Joy",
    "780e1bf1-e392-40f2-9e84-764dedc5fcd4",
    "Fiona Hsieh",
    crate::card::CardRules::unsupported(),
);

// NEO 157 — Rabbit Battery
pub(in crate::card::sets) static RABBIT_BATTERY: CardRecord = CardRecord::new(
    "Rabbit Battery",
    "5d33a5b7-797b-4079-8d62-edd124c0fb5a",
    "Justyna Dura",
CardRules::new_artifact_creature(mana_cost!("{R}"), &["Equipment", "Rabbit"], 1, 1)
        .with_abilities(&[
            abilities::haste(),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1 and has haste.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::haste()),
                    ]),
                },
            ),
            abilities::reconfigure(
                &[CostDef::Mana(mana_cost!("{R}"))],
                "Reconfigure {R} ({R}: Attach to target creature you control; or unattach from a creature. Reconfigure only as a sorcery. While attached, this isn't a creature.)",
            ),
        ]),
);

// NEO 168 — Twinshot Sniper
pub(in crate::card::sets) static TWINSHOT_SNIPER_168: CardRecord = CardRecord::new(
    "Twinshot Sniper",
    "08a86009-4637-4b6c-9d36-367151583668",
    "Brent Hollowell",
    CardRules::new_artifact_creature(mana_cost!("{3}{R}"), &["Goblin", "Archer"], 2, 3)
        .with_abilities(&[
            abilities::reach(),
            abilities::enters_trigger_with_targets(
                "When this creature enters, it deals 2 damage to any target.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::AnyTarget,
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(2),
                ),
            ),
            AbilityDef::activated_with_targets(
                "Channel — {1}{R}, Discard this card: It deals 2 damage to any target.",
                &[CostDef::Mana(mana_cost!("{1}{R}")), CostDef::DiscardSource],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::AnyTarget,
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(2),
                ),
            )
            .with_source_zones(&[ZoneKind::Hand]),
        ]),
);

// NEO 189 — Greater Tanuki
pub(in crate::card::sets) static GREATER_TANUKI: CardRecord = CardRecord::new(
    "Greater Tanuki",
    "b4fbaee3-a10f-4b2d-b07e-d041a96a7e27",
    "Ilse Gort",
// Six mana for the body or three for a land: channel is what makes a
    // top-heavy creature a reasonable card to draw on turn three.
    CardRules::new_enchantment_creature(mana_cost!("{4}{G}{G}"), &["Dog"], 6, 5).with_abilities(&[
        abilities::trample(),
        AbilityDef::activated(
            "Channel — {2}{G}, Discard this card: Search your library for a basic land card, put it onto the battlefield tapped, then shuffle.",
            &[
                CostDef::Mana(mana_cost!("{2}{G}")),
                CostDef::DiscardSource,
            ],
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Supertype(CardSupertype::Basic),
                ]),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: false,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: true,
                attachment: None,
                binding: None,
                then: None,
            },
        )
        // Activated from hand, which is the only place a card can be
        // discarded from.
        .with_source_zones(&[ZoneKind::Hand]),
    ]),
);

// NEO 190 — Harmonious Emergence
pub(in crate::card::sets) static HARMONIOUS_EMERGENCE: CardRecord = CardRecord::new(
    "Harmonious Emergence",
    "c92ff968-b436-4313-8375-8a3bb41f9892",
    "Simon Dominic",
CardRules::new_enchantment(mana_cost!("{3}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_land_you_control(),
            AbilityDef::static_ability(
                "Enchanted land is a 4/5 green Spirit creature with vigilance and haste. It's still a land.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                        AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&[
                            "Spirit",
                        ])),
                        AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Green])),
                        AppliedEffectDef::set_base_power_toughness(
                            ValueDef::Constant(4),
                            ValueDef::Constant(5),
                        ),
                        AppliedEffectDef::add_ability(&abilities::vigilance()),
                        AppliedEffectDef::add_ability(&abilities::haste()),
                    ]),
                },
            ),
            EMERGENCE_REPLACEMENT,
        ]),
);

// NEO 211 — Tamiyo's Safekeeping
pub(in crate::card::sets) static TAMIYO_S_SAFEKEEPING: CardRecord = CardRecord::new(
    "Tamiyo's Safekeeping",
    "fd4b7ee2-de65-4288-872d-486065a4f226",
    "Aurore Folny",
    // One mana that answers removal and damage alike, and the two life is
    // what keeps it from being dead when neither is coming.
    CardRules::new_instant(mana_cost!("{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target permanent you control gains hexproof and indestructible until end of turn. You \
         gain 2 life. (A permanent with hexproof and indestructible can't be the target of \
         spells or abilities your opponents control. Damage and effects that say \"destroy\" \
         don't destroy it.)",
        // Any permanent, not just a creature: it protects a Vehicle or an
        // enchantment the deck cares about just as well.
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Any,
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            // The life is unconditional: it still arrives when the target
            // has already left in response.
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ]),
    )),
);

// NEO 216 — Colossal Skyturtle
pub(in crate::card::sets) static COLOSSAL_SKYTURTLE_216: CardRecord = CardRecord::new(
    "Colossal Skyturtle",
    "f40bd797-4d12-4098-a1a8-d7e5b7b82ac9",
    "Nicholas Gregory",
    CardRules::new_enchantment_creature(mana_cost!("{4}{G}{G}{U}"), &["Turtle"], 6, 5).with_abilities(&[
abilities::flying(),
abilities::ward(&[CostDef::Mana(mana_cost!("{2}"))], "Ward {2}"),
AbilityDef::activated_with_targets("Channel — {2}{G}, Discard this card: Return target card from your graveyard to your hand.", &[CostDef::Mana(mana_cost!("{2}{G}")), CostDef::DiscardSource], &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::Any, zones: &[ZoneKind::Graveyard], controller: None, owner: Some(PlayerRelation::You) })], EffectDef::move_to_zone(EffectRecipientDef::Target(TargetIndex::PRIMARY), ZoneKind::Hand, ZonePlacement::Top)).with_source_zones(&[ZoneKind::Hand]),
AbilityDef::activated_with_targets("Channel — {1}{U}, Discard this card: Return target creature to its owner's hand.", &[CostDef::Mana(mana_cost!("{1}{U}")), CostDef::DiscardSource], &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Creature))], EffectDef::move_to_zone(EffectRecipientDef::Target(TargetIndex::PRIMARY), ZoneKind::Hand, ZonePlacement::Top)).with_source_zones(&[ZoneKind::Hand])
]),
);

// NEO 218 — Enthusiastic Mechanaut
pub(in crate::card::sets) static ENTHUSIASTIC_MECHANAUT_218: CardRecord = CardRecord::new(
    "Enthusiastic Mechanaut",
    "ac00521f-1b7d-478d-afe8-6761ea512d8d",
    "Anna Steinbauer",
    CardRules::new_artifact_creature(mana_cost!("{U}{R}"), &["Goblin", "Artificer"], 2, 2)
        .with_abilities(&[
            abilities::flying(),
            abilities::spell_cost_adjustment(
                "Artifact spells you cast cost {1} less to cast.",
                ObjectPredicateDef::HasType(CardType::Artifact),
                PlayerRelation::You,
                SpellCostConditionDef::Always,
                CostAdjustmentDef::Subtract(CostAmountDef::Mana(mana_cost!("{1}"))),
            ),
        ]),
);

// NEO 222 — Hinata, Dawn-Crowned
pub(in crate::card::sets) static HINATA_DAWN_CROWNED: CardRecord = CardRecord::new(
    "Hinata, Dawn-Crowned",
    "f25aff90-56fd-4f70-bb3b-cabf2900c391",
    "Alexander Mokhov",
    CardRules::new_creature(mana_cost!("{1}{U}{R}{W}"), &["Kirin", "Spirit"], 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            abilities::trample(),
            abilities::spell_cost_adjustment(
                "Spells you cast cost {1} less to cast for each target.",
                ObjectPredicateDef::Any,
                PlayerRelation::You,
                SpellCostConditionDef::Always,
                CostAdjustmentDef::Subtract(CostAmountDef::Generic(ValueDef::DistinctTargets)),
            ),
            abilities::spell_cost_adjustment(
                "Spells your opponents cast cost {1} more to cast for each target.",
                ObjectPredicateDef::Any,
                PlayerRelation::Opponent,
                SpellCostConditionDef::Always,
                CostAdjustmentDef::Add(CostAmountDef::Generic(ValueDef::DistinctTargets)),
            ),
        ]),
);

// NEO 238 — Tamiyo, Compleated Sage
// Audit: unsupported — Needs variable loyalty costs and arbitrary graveyard-card copy tokens using last-known information.
pub(in crate::card::sets) static TAMIYO_COMPLEATED_SAGE: CardRecord = CardRecord::new(
    "Tamiyo, Compleated Sage",
    "222a736e-d819-452d-aeda-eb848c4b2302",
    "Chris Rahn",
    CardRules::unsupported(),
);

// NEO 243 — Containment Construct
// Audit: unsupported — The discard event also covers replacement destinations. An arbitrary triggering-card reference cannot be restricted to its graveyard before ExileGrantingControllerPlayThisTurn, so a replaced discard would incorrectly grant play permission.
pub(in crate::card::sets) static CONTAINMENT_CONSTRUCT_243: CardRecord = CardRecord::new(
    "Containment Construct",
    "520e5505-429b-4da0-b25e-14b8d4e81ce3",
    "Julian Kok Joon Wen",
    crate::card::CardRules::unsupported(),
);

// NEO 247 — High-Speed Hoverbike
pub(in crate::card::sets) static HIGH_SPEED_HOVERBIKE_247: CardRecord = CardRecord::new(
    "High-Speed Hoverbike",
    "7c619116-1eae-439d-9b1f-639643458a23",
    "Julian Kok Joon Wen",
    CardRules::new_artifact(mana_cost!("{2}")).with_subtypes(&["Vehicle"]).with_abilities(&[
        abilities::flash(),
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this Vehicle enters, tap up to one target creature.",
            &[AbilityTargetDef::up_to(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature), zones: &[ZoneKind::Battlefield], controller: None, owner: None,
            }, 1)],
            EffectDef::Tap { object: EffectRecipientDef::Target(TargetIndex::PRIMARY) },
        ),
        abilities::crew("Crew 1 (Tap any number of creatures you control with total power 1 or more: This Vehicle becomes an artifact creature until end of turn.)", 1),
    ]),
);

// NEO 248 — Iron Apprentice
// Audit: unsupported — Needs a kind-agnostic counter transfer. "Put those counters on target creature" moves every kind the dying creature had, in the amounts it had, but AddCounters always names one kind and only removal has a kind-agnostic form. Narrowing it to +1/+1 would silently drop any other counter that reached this creature.
pub(in crate::card::sets) static IRON_APPRENTICE: CardRecord = CardRecord::new(
    "Iron Apprentice",
    "13d6d9fc-509b-42db-8ac1-85066eb6e9c4",
    "Kekai Kotaki",
    crate::card::CardRules::unsupported(),
);

// NEO 250 — Mirror Box
// Audit: unsupported — Needs a static value evaluated relative to each affected creature so it can count other creatures sharing that creature's name.
pub(in crate::card::sets) static MIRROR_BOX: CardRecord = CardRecord::new(
    "Mirror Box",
    "d507daa3-3f16-4ab1-81ea-794e5bb488fc",
    "Néstor Ossandón Leal",
    CardRules::unsupported(),
);

// NEO 271 — Otawara, Soaring City
/// The discount, which is what makes the land a spell: a legendary board
/// takes the channel cost down toward the {U} that cannot be reduced away.
static LEGENDARY_CREATURES_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::Supertype(CardSupertype::Legendary),
        ObjectPredicateDef::HasType(CardType::Creature),
    ]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

pub(in crate::card::sets) static OTAWARA_SOARING_CITY: CardRecord = CardRecord::new(
    "Otawara, Soaring City",
    "486d7edc-d983-41f0-8b78-c99aecd72996",
    "Alayna Danner",
    // A land that costs nothing to play and is never a dead draw, which is
    // the whole of why the cycle is in the cube.
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_mana(
                "{T}: Add {U}.",
                &[CostDef::TapSource],
                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue)),
            ),
            AbilityDef::activated_with_targets(
                "Channel — {3}{U}, Discard this card: Return target artifact, creature, \
                 enchantment, or planeswalker to its owner\'s hand. This ability costs {1} less \
                 to activate for each legendary creature you control.",
                &[CostDef::Mana(mana_cost!("{3}{U}")), CostDef::DiscardSource],
                &[AbilityTargetDef::exactly_one_permanent(
                    // Everything a bounce spell would want and nothing else: a land answers a
                    // creature, but not another land.
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]),
                )],
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            )
            .with_source_zones(&[ZoneKind::Hand])
            .with_activation_cost_reduction(
                ValueDef::CountMatchingObjects(&LEGENDARY_CREATURES_YOU_CONTROL),
                0,
            ),
        ]),
);

// NEO 275 — Secluded Courtyard
// Audit: unsupported — Needs a mana restriction selecting both creature spells and creature-source activated abilities by the chosen creature type, with the choice retained in each produced mana unit.
pub(in crate::card::sets) static SECLUDED_COURTYARD: CardRecord = CardRecord::new(
    "Secluded Courtyard",
    "0539b1a5-8704-476f-ba1f-2fe01190e157",
    "Sam Burley",
    CardRules::unsupported(),
);

// NEO 281 — Uncharted Haven
pub(in crate::card::sets) static UNCHARTED_HAVEN: CardRecord = CardRecord::new(
    "Uncharted Haven",
    "1d4ad89a-3a00-4bf4-a357-4a8a089d4a82",
    "Lorenzo Lanfranconi",
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::replacement(
            "This land enters tapped. As it enters, choose a color.",
            ReplacementEffectDef::Sequence(&[
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::Tapped,
                ),
                ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                    BattlefieldEntryScalarChoiceDef::COLOR,
                )),
            ]),
        ),
        AbilityDef::activated_mana(
            "{T}: Add one mana of the chosen color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one_of_type(ManaTypeDef::ChosenColor)),
        ),
    ]),
);

// NEO 357 — Fable of the Mirror-Breaker // Reflection of Kiki-Jiki
pub(in crate::card::sets) static FABLE_OF_THE_MIRROR_BREAKER: CardRecord = CardRecord::new_dfc(
    "Fable of the Mirror-Breaker // Reflection of Kiki-Jiki",
    "0b696cd1-0d72-4df5-bacc-dc77e62f9a13",
    "akio",
    // Three mana that pays for itself twice over: a body, a loot, and then
    // the half nobody reads the Saga for.
    &[
        (
            "Fable of the Mirror-Breaker",
            const {
                CardRules::new_enchantment(mana_cost!("{2}{R}"))
                .with_subtypes(&["Saga"])
                .with_abilities(&const { [
                    abilities::saga_chapter(
                        1,
                        "I — Create a 2/2 red Goblin Shaman creature token with \"Whenever this token attacks, \
                         create a Treasure token.\"",
                        EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                            TokenCharacteristics::creature(
                                &["Goblin", "Shaman"],
                                &[ManaColor::Red],
                                2,
                                2,
                            )
                            // The Goblin's own clause, printed on the token rather than on the Saga.
                            .with_abilities(
                                &const {
                                    [AbilityDef::triggered(
                                        "Whenever this token attacks, create a Treasure token.",
                                        TriggerEventDef::attacks(ObjectPredicateDef::Source),
                                        EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))),
                                    )]
                                },
                            ),
                        ))),
                    ),
                    abilities::saga_chapter(
                        2,
                        "II — You may discard up to two cards. If you do, draw that many cards.",
                        EffectDef::Choose(ChooseDef {
                            binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                            unchosen: None,
                            chooser: PlayerRefDef::EffectController,
                            candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
                                ObjectPredicateDef::Any,
                                &[ZoneKind::Hand],
                                PlayerSetDef::One(PlayerRefDef::EffectController),
                            )),
                            exclude: None,
                            minimum: 0,
                            maximum: 2,
                            visibility: ChoiceVisibilityDef::Private,
                            // "Discard up to two cards. If you do, draw that many." The size is the
                            // player's to choose, so the discard is a choice with a floor of none and
                            // what is drawn is however many that turned out to be.
                            then: &EffectDef::Sequence(&const { [
                                EffectDef::discard_cards(EffectRecipientDef::objects(
                                        ObjectSetDef::Binding(ParentBinding),
                                    )),
                                EffectDef::DrawCards {
                                    recipient: EffectRecipientDef::Controller,
                                    amount: ValueDef::BoundObjectCount(ParentBinding),
                                },
                            ] }),
                        }),
                    ),
                    abilities::saga_chapter(
                        3,
                        "III — Exile this Saga, then return it to the battlefield transformed under your control.",
                        abilities::exile_and_return_transformed(EffectRecipientDef::Source),
                    ),
                ] })
            },
        ),
        (
            "Reflection of Kiki-Jiki",
            const {
                CardRules::new_creature_without_mana_cost(&["Goblin", "Shaman"], 2, 2)
                .with_type(CardType::Enchantment)
                .printed_colors(&[ManaColor::Red])
                .with_abilities(&const { [AbilityDef::activated_with_targets(
                    "{1}, {T}: Create a token that's a copy of another target nonlegendary creature you control, \
                     except it has haste. Sacrifice it at the beginning of the next end step.",
                    &[
                        CostDef::Mana(mana_cost!("{1}")),
                        CostDef::TapSource,
                    ],
                    // "Another target nonlegendary creature you control": the Reflection may
                    // not copy itself, and a legendary copy would be put into a graveyard by
                    // the legend rule the moment it arrived.
                    &const { [
                        AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(CardSupertype::Legendary)),
                            ]),
                            zones: &[ZoneKind::Battlefield],
                            controller: Some(PlayerRelation::You),
                            owner: None,
                        })
                        .excluding_source(),
                    ] },
                    EffectDef::CreateToken(
                        CreateTokenDef::new(TokenDef::Copy(
                            &const {
                                crate::card::TokenCopyDef {
                                    object: &EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                    exceptions: CopyExceptionsDef::NONE
                                        .with_abilities(&const { [CopyAbilityDef::Ability(&abilities::haste())] }),
                                }
                            },
                        ))
                        .with_created_tokens(CreatedTokensDef {
                            binding: ParentBinding,
                            then: &const {
                                EffectDef::InstallTrigger(InstalledTriggerDef::once(
                                    &const {
                                        AbilityDef::triggered(
                                            "Sacrifice it at the beginning of the next end step.",
                                            TriggerEventDef::StepBegins {
                                                step: TurnStepDef::End,
                                                player: PlayerRelation::Any,
                                            },
                                            EffectDef::sacrifice(EffectRecipientDef::objects(ObjectSetDef::Binding(
                                                ParentBinding,
                                            ))),
                                        )
                                    },
                                ))
                            },
                        }),
                    ),
                )] })
            },
        ),
    ],
);

// NEO 371 — Jin-Gitaxias, Progress Tyrant
pub(in crate::card::sets) static JIN_GITAXIAS_PROGRESS_TYRANT_371: CardRecord = CardRecord::new(
    "Jin-Gitaxias, Progress Tyrant",
    "01985566-275b-4bf0-8667-c81eb95ad70c",
    "Ai Nanahira",
    CardRules::new_creature(mana_cost!("{5}{U}{U}"), &["Phyrexian", "Praetor"], 5, 5).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::triggered("Whenever you cast an artifact, instant, or sorcery spell, copy that spell. You may choose new targets for the copy. This ability triggers only once each turn. (A copy of a permanent spell becomes a token.)", TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Instant), ObjectPredicateDef::HasType(CardType::Sorcery)])]), ObjectPredicateDef::ControlledBy(PlayerRelation::You)])), EffectDef::CopyStackObject(&CopyStackObjectDef { object: EffectRecipientDef::TriggeringObject, controller: PlayerRefDef::EffectController, count: ValueDef::Constant(1), retarget: true, colors: None })).triggering_at_most(1),
AbilityDef::triggered("Whenever an opponent casts an artifact, instant, or sorcery spell, counter that spell. This ability triggers only once each turn.", TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Instant), ObjectPredicateDef::HasType(CardType::Sorcery)])]), ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent)])), EffectDef::Counter { object: EffectRecipientDef::TriggeringObject, zone: ZoneKind::Graveyard, placement: ZonePlacement::Top }).triggering_at_most(1)
]),
);

// NEO 412 — Boseiju, Who Endures
pub(in crate::card::sets) static BOSEIJU_WHO_ENDURES: CardRecord = CardRecord::new(
    "Boseiju, Who Endures",
    "0055ea30-20fb-4324-a632-8fed87628f05",
    "Esuthio",
    // A Forest that answers the one artifact the deck could not otherwise
    // beat, and costs nothing to play when it does not have to.
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_mana(
                "{T}: Add {G}.",
                &[CostDef::TapSource],
                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
            ),
            AbilityDef::activated_with_targets(
                "Channel — {1}{G}, Discard this card: Destroy target artifact, enchantment, or \
                 nonbasic land an opponent controls. That player may search their library for a \
                 land card with a basic land type, put it onto the battlefield, then shuffle. \
                 This ability costs {1} less to activate for each legendary creature you control.",
                &[CostDef::Mana(mana_cost!("{1}{G}")), CostDef::DiscardSource],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        // "Nonbasic" is the whole reason the land half is in the target list: every
                        // land worth answering is one, and a basic is never worth the card.
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Land),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(
                                    CardSupertype::Basic,
                                )),
                            ]),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::Opponent),
                        owner: None,
                    },
                )],
                EffectDef::Sequence(&[
                    EffectDef::Destroy {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        then: None,
                    },
                    // Their search, not yours: the player whose permanent was destroyed is the
                    // one who may go looking, and the land arrives untapped.
                    EffectDef::May {
                        player: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                            ObjectRefDef::Target(TargetIndex::PRIMARY),
                        )),
                        effect: &EffectDef::SearchZone {
                            player: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                                ObjectRefDef::Target(TargetIndex::PRIMARY),
                            )),
                            source: ZoneKind::Library,
                            // "A land card with a basic land type", which is what makes the
                            // compensation a fixing land rather than a card: a dual with a basic type
                            // counts and a Wasteland does not.
                            object: ObjectPredicateDef::HasAnyBasicLandType(&BasicLandType::ALL),
                            minimum: 0,
                            maximum: ValueDef::Constant(1),
                            reveal: false,
                            destination: ZoneKind::Battlefield,
                            placement: ZonePlacement::Top,
                            shuffle: true,
                            enters_tapped: false,
                            attachment: None,
                            binding: None,
                            then: None,
                        },
                    },
                ]),
            )
            .with_source_zones(&[ZoneKind::Hand])
            .with_activation_cost_reduction(
                ValueDef::CountMatchingObjects(&LEGENDARY_CREATURES_YOU_CONTROL),
                0,
            ),
        ]),
);

// NEO 413 — Eiganjo, Seat of the Empire
pub(in crate::card::sets) static EIGANJO_SEAT_OF_THE_EMPIRE_413: CardRecord = CardRecord::new(
    "Eiganjo, Seat of the Empire",
    "7c31c48f-6275-4430-8dc9-05d70c332b7a",
    "ZOUNOSE",
    CardRules::new_land(&[]).with_supertype(CardSupertype::Legendary).with_abilities(&[
abilities::tap_for(ManaColor::White),
AbilityDef::activated_with_targets("Channel — {2}{W}, Discard this card: It deals 4 damage to target attacking or blocking creature. This ability costs {1} less to activate for each legendary creature you control.", &[CostDef::Mana(mana_cost!("{2}{W}")), CostDef::DiscardSource], &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::Attacking, ObjectPredicateDef::Blocking])]))], EffectDef::damage(EffectRecipientDef::Target(TargetIndex::PRIMARY), ValueDef::Constant(4))).with_source_zones(&[ZoneKind::Hand]).with_activation_cost_reduction(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Supertype(CardSupertype::Legendary)]), &[ZoneKind::Battlefield], PlayerRelation::You)), 0)
]),
);

// NEO 415 — Sokenzan, Crucible of Defiance
pub(in crate::card::sets) static SOKENZAN_CRUCIBLE_OF_DEFIANCE_415: CardRecord = CardRecord::new(
    "Sokenzan, Crucible of Defiance",
    "327333cc-2cc9-44ba-a0e6-d01329c416a3",
    "Nao Miyoshi",
    CardRules::new_land(&[]).with_supertype(CardSupertype::Legendary).with_abilities(&[
abilities::tap_for(ManaColor::Red),
AbilityDef::activated("Channel — {3}{R}, Discard this card: Create two 1/1 colorless Spirit creature tokens. They gain haste until end of turn. This ability costs {1} less to activate for each legendary creature you control.", &[CostDef::Mana(mana_cost!("{3}{R}")), CostDef::DiscardSource], EffectDef::create_creature_token(&["Spirit"], &[], 1, 1).with_count(ValueDef::Constant(2)).with_created_tokens(crate::card::CreatedTokensDef { binding: ParentBinding, then: &EffectDef::Apply { recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)), effect: AppliedEffectDef::add_ability(&abilities::haste()), duration: ResolvedEffectDurationDef::UntilEndOfTurn } })).with_source_zones(&[ZoneKind::Hand]).with_activation_cost_reduction(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Supertype(CardSupertype::Legendary)]), &[ZoneKind::Battlefield], PlayerRelation::You)), 0)
]),
);

// NEO 418 — The Wandering Emperor (alternate printing)
const THE_WANDERING_EMPEROR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_WANDERING_EMPEROR,
    1,
    "22edc832-993b-432a-8435-8d8a72799122",
    "Hisashi Momose",
);

// NEO 436 — Farewell
pub(in crate::card::sets) static FAREWELL_436: CardRecord = CardRecord::new(
    "Farewell",
    "0050b693-7bad-4c0c-baca-0186d153ce2e",
    "Seb McKinnon",
    CardRules::new_sorcery(mana_cost!("{4}{W}{W}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one or more —",
        &[
            AbilityDef::spell(
                "Exile all artifacts.",
                EffectDef::move_to_zone(
                    EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
            ),
            AbilityDef::spell(
                "Exile all creatures.",
                EffectDef::move_to_zone(
                    EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
            ),
            AbilityDef::spell(
                "Exile all enchantments.",
                EffectDef::move_to_zone(
                    EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
            ),
            AbilityDef::spell(
                "Exile all graveyards.",
                EffectDef::move_to_zone(
                    EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Graveyard],
                        PlayerRelation::Any,
                    ),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
            ),
        ],
    )
    .with_mode_selection(1, 4, false)]),
);

// NEO 449 — The Reality Chip
pub(in crate::card::sets) static THE_REALITY_CHIP_449: CardRecord = CardRecord::new(
    "The Reality Chip",
    "9797bb82-24f6-4dd5-8f5d-b3ea45bb65b8",
    "Campbell White",
    CardRules::new_artifact_creature(mana_cost!("{1}{U}"), &["Equipment", "Jellyfish"], 0, 4).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::static_ability("You may look at the top card of your library any time.", EffectDef::StaticApply { recipient: EffectRecipientDef::Controller, effect: AppliedEffectDef::Rule(AppliedRuleDef::MayLookAtTopOfLibrary) }),
AbilityDef::static_ability("As long as The Reality Chip is attached to a creature, you may play lands and cast spells from the top of your library.", EffectDef::IfCondition { condition: &TriggerConditionDef::AttachedPermanentMatches { object: ObjectPredicateDef::HasType(CardType::Creature) }, then: &EffectDef::StaticApply { recipient: EffectRecipientDef::Controller, effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromTopOfLibrary { restriction: PlayRestrictionDef::new(PlayActionMatcherDef::Any, ObjectPredicateDef::Any), cost: TopOfLibraryCostDef::Printed }) } }),
abilities::reconfigure(&[CostDef::Mana(mana_cost!("{2}{U}"))], "Reconfigure {2}{U} ({2}{U}: Attach to target creature you control; or unattach from a creature. Reconfigure only as a sorcery. While attached, this isn't a creature.)")
]),
);

// NEO 505 — Takenuma, Abandoned Mire
pub(in crate::card::sets) static TAKENUMA_ABANDONED_MIRE_505: CardRecord = CardRecord::new(
    "Takenuma, Abandoned Mire",
    "13410bd5-acee-4cc9-90e3-dbcf8415bcaf",
    "Sam Burley",
    CardRules::new_land(&[]).with_supertype(CardSupertype::Legendary).with_abilities(&[
abilities::tap_for(ManaColor::Black),
AbilityDef::activated("Channel — {3}{B}, Discard this card: Mill three cards, then return a creature or planeswalker card from your graveyard to your hand. This ability costs {1} less to activate for each legendary creature you control.", &[CostDef::Mana(mana_cost!("{3}{B}")), CostDef::DiscardSource], EffectDef::Sequence(&[EffectDef::Mill { player: EffectRecipientDef::Controller, amount: ValueDef::Constant(3) }, EffectDef::Choose(ChooseDef { chooser: PlayerRefDef::EffectController, candidates: ObjectSetDef::Query(ObjectQueryDef::matching(ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::HasType(CardType::Planeswalker)]), &[ZoneKind::Graveyard], PlayerRelation::You)), exclude: None, minimum: 1, maximum: 1, binding: ObjectChoiceBindingDef::Objects(Binding!("takenuma_return")), unchosen: None, visibility: ChoiceVisibilityDef::Public, then: &EffectDef::move_to_zone(EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("takenuma_return"))), ZoneKind::Hand, ZonePlacement::Top) })])).with_source_zones(&[ZoneKind::Hand]).with_activation_cost_reduction(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Supertype(CardSupertype::Legendary)]), &[ZoneKind::Battlefield], PlayerRelation::You)), 0)
]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &IMPERIAL_OATH,
    &LION_SASH,
    &TOUCH_THE_SPIRIT_REALM,
    &THE_WANDERING_EMPEROR,
    &MARCH_OF_SWIRLING_MIST_61,
    &MIRRORSHELL_CRAB,
    &MOON_CIRCUIT_HACKER,
    &MOONSNARE_PROTOTYPE_69,
    &TAMESHI_REALITY_ARCHITECT_82,
    &CLAWING_TORMENT,
    &OKIBA_RECKONER_RAID,
    &VIRUS_BEETLE,
    &CRACKLING_EMERGENCE,
    &EXPERIMENTAL_SYNTHESIZER_138,
    &GORO_GORO_DISCIPLE_OF_RYUSEI_145,
    &IRONHOOF_BOAR,
    &MARCH_OF_RECKLESS_JOY_154,
    &RABBIT_BATTERY,
    &TWINSHOT_SNIPER_168,
    &GREATER_TANUKI,
    &HARMONIOUS_EMERGENCE,
    &TAMIYO_S_SAFEKEEPING,
    &COLOSSAL_SKYTURTLE_216,
    &ENTHUSIASTIC_MECHANAUT_218,
    &HINATA_DAWN_CROWNED,
    &TAMIYO_COMPLEATED_SAGE,
    &CONTAINMENT_CONSTRUCT_243,
    &HIGH_SPEED_HOVERBIKE_247,
    &IRON_APPRENTICE,
    &MIRROR_BOX,
    &OTAWARA_SOARING_CITY,
    &SECLUDED_COURTYARD,
    &UNCHARTED_HAVEN,
    &FABLE_OF_THE_MIRROR_BREAKER,
    &JIN_GITAXIAS_PROGRESS_TYRANT_371,
    &BOSEIJU_WHO_ENDURES,
    &EIGANJO_SEAT_OF_THE_EMPIRE_413,
    &SOKENZAN_CRUCIBLE_OF_DEFIANCE_415,
    &FAREWELL_436,
    &THE_REALITY_CHIP_449,
    &TAKENUMA_ABANDONED_MIRE_505,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] =
    &[THE_WANDERING_EMPEROR_ALTERNATE_1];
