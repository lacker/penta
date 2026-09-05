//! Kamigawa: Neon Dynasty attachment edge cases.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCostList, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    AddManaEffectDef, AppliedEffectDef, AppliedRuleDef, BasicLandType, CardArt, CardRules, CardSet,
    CardSupertype, CardType, ChoiceVisibilityDef, ChooseDef, CopyAbilityDef, CopyExceptionsDef,
    CostAdjustmentDef, CostAmountDef, CounterKind, CreatedTokensDef, EffectDef, EffectRecipientDef,
    InstalledTriggerDef, ManaColor, ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef,
    ObjectRefDef, ObjectSetDef, PlayerRefDef, PlayerRelation, PlayerSetDef,
    ResolvedEffectDurationDef, SpellCostConditionDef, TriggerConditionDef, TriggerEventDef,
    TurnStepDef, ValueDef, ZoneKind, ZonePlacement, abilities, tokens,
};
use crate::ids::{ParentBinding, TargetIndex};
use crate::mana_cost;

// NEO 17 — Imperial Oath
pub(in crate::card::sets) static IMPERIAL_OATH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3d6750dd-2303-493b-885d-1bfb5787b16c"),
    "Imperial Oath",
    CardArt::new("3d6750dd-2303-493b-885d-1bfb5787b16c", "Nicholas Elias"),
    CardSet::KamigawaNeonDynasty,
    // Six power that can attack and still block, plus three cards deep of
    // smoothing -- a limited finisher rather than anything a cube wants.
    CardRules::new_sorcery(mana_cost!("{5}{W}")).with_ability(AbilityDef::spell(
        "Create three 2/2 white Samurai creature tokens with vigilance. Scry 3.",
        EffectDef::Sequence(&[
            EffectDef::create_creature_token(&["Samurai"], &[ManaColor::White], 2, 2)
                .with_abilities(&[abilities::vigilance()])
                .with_amount(3),
            abilities::scry(ValueDef::Constant(3)),
        ]),
    )),
);

// NEO 26 — Lion Sash
pub(in crate::card::sets) static LION_SASH: CardRecord = CardRecord::new_with_legacy_id(
    2243,
    "Lion Sash",
    CardArt::new("3e1766e9-2fa7-4446-a255-7beea1467ece", "Yongjae Choi"),
    CardSet::KamigawaNeonDynasty,
    // Graveyard hate that grows into a threat, and reconfigure means the
    // two halves are the same card rather than a choice made on turn two.
    CardRules::new_artifact_creature(mana_cost!("{1}{W}"), &["Equipment", "Cat"], 1, 1)
        .with_abilities(&[
            AbilityDef::activated_with_targets(
                "{W}: Exile target card from a graveyard. If it was a permanent card, put a \
                 +1/+1 counter on this permanent.",
                &[AbilityCostDef::Mana(mana_cost!("{W}"))],
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
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        zone: ZoneKind::Exile,
                        placement: ZonePlacement::Top,
                    },
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
                mana_cost!("{2}"),
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
    PrintingAnchor::scryfall("e16ab44e-4257-4c0c-b705-8ac1e9c1d835"),
    "Touch the Spirit Realm",
    CardArt::new("e16ab44e-4257-4c0c-b705-8ac1e9c1d835", "Marta Nael"),
    CardSet::KamigawaNeonDynasty,
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
        AbilityDef::activated_with_cost_list_and_targets(
            "Channel — {1}{W}, Discard this card: Exile target artifact or creature. Return it to \
             the battlefield under its owner's control at the beginning of the next end step.",
            AbilityCostList::two(
                AbilityCostDef::Mana(mana_cost!("{1}{W}")),
                AbilityCostDef::DiscardSource,
            ),
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
    PrintingAnchor::scryfall("fab2d8a9-ab4c-4225-a570-22636293c17d"),
    "The Wandering Emperor",
    CardArt::new("fab2d8a9-ab4c-4225-a570-22636293c17d", "Tommy Arnold"),
    CardSet::KamigawaNeonDynasty,
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
                &[AbilityCostDef::Loyalty(1)],
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
                &[AbilityCostDef::Loyalty(-1)],
                EffectDef::create_creature_token(&["Samurai"], &[ManaColor::White], 2, 2)
                    .with_abilities(&[abilities::vigilance()])
                    .with_art(CardArt::new(
                        "f68e5337-6e44-4f8f-a102-2f97b433beea",
                        "Gaboleps",
                    )),
            ),
            AbilityDef::activated_with_targets(
                "−2: Exile target tapped creature. You gain 2 life.",
                &[AbilityCostDef::Loyalty(-2)],
                // A tapped creature: the minus answers an attacker that has already
                // committed, which is the half of removal flash was made for.
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Tapped,
                    ]),
                )],
                EffectDef::Sequence(&[
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        zone: ZoneKind::Exile,
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

// NEO 63 — Mirrorshell Crab
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIRRORSHELL_CRAB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0394c8df-2e8a-4477-93b7-569934d7b936"),
    "Mirrorshell Crab",
    crate::card::CardArt::new("0394c8df-2e8a-4477-93b7-569934d7b936", "Cristi Balanescu"),
    crate::card::CardSet::KamigawaNeonDynasty,
    crate::card::CardRules::unsupported(),
);

// NEO 67 — Moon-Circuit Hacker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOON_CIRCUIT_HACKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("75c43923-7280-4ccb-810b-e8c38dd8a26f"),
    "Moon-Circuit Hacker",
    crate::card::CardArt::new("c6e466d1-943d-41e6-a47d-c9d951ca4262", "Tia Masic"),
    crate::card::CardSet::KamigawaNeonDynasty,
    crate::card::CardRules::unsupported(),
);

// NEO 91 — Clawing Torment
pub(in crate::card::sets) static CLAWING_TORMENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("621fce96-5933-4e2b-98ec-2589940e24cb"),
    "Clawing Torment",
    CardArt::new("621fce96-5933-4e2b-98ec-2589940e24cb", "Rovina Cai"),
    CardSet::KamigawaNeonDynasty,
    // One mana that shrinks a creature or just drains, and either way it
    // closes the game a life at a time.
    CardRules::new_enchantment(mana_cost!("{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell(
                "Enchant artifact or creature",
                &const {
                    [AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Creature),
                        ]),
                    )]
                },
            ),
            AbilityDef::static_ability(
                "As long as enchanted permanent is a creature, it gets -1/-1 and can't block.",
                // Written as the printed conditional rather than applied
                // unconditionally: this can land on an artifact, and the
                // clause only starts once that artifact is also a creature.
                EffectDef::IfCondition {
                    condition: &const {
                        TriggerConditionDef::AttachedPermanentMatches {
                            object: ObjectPredicateDef::HasType(CardType::Creature),
                        }
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
                    effect: AppliedEffectDef::add_ability(
                        &const {
                            AbilityDef::triggered(
                                "At the beginning of your upkeep, you lose 1 life.",
                                TriggerEventDef::StepBegins {
                                    step: TurnStepDef::Upkeep,
                                    player: PlayerRelation::You,
                                },
                                EffectDef::LoseLife {
                                    recipient: EffectRecipientDef::Controller,
                                    amount: ValueDef::Constant(1),
                                },
                            )
                        },
                    ),
                },
            ),
        ]),
);

// NEO 117 — Okiba Reckoner Raid // Nezumi Road Captain
// Audit: unsupported — Needs a Saga clause shared across chapters. Every piece is present -- saga_chapter, exile_and_return_transformed, and the transforming two-face record -- but "I, II" is one printed clause on two chapters, and a chapter is one ability per lore counter. Repeating the text prints the line twice and leaving the second empty is rejected outright.
pub(in crate::card::sets) static OKIBA_RECKONER_RAID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4f0582b4-d951-4450-b158-4a34109e48cd"),
    "Okiba Reckoner Raid",
    crate::card::CardArt::new(
        "4f0582b4-d951-4450-b158-4a34109e48cd",
        "Victor Adame Minguez",
    ),
    crate::card::CardSet::KamigawaNeonDynasty,
    crate::card::CardRules::unsupported(),
);

// NEO 148 — Ironhoof Boar
pub(in crate::card::sets) static IRONHOOF_BOAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("73abe574-6fb8-4809-9c18-0cf989f986f5"),
    "Ironhoof Boar",
    CardArt::new(
        "73abe574-6fb8-4809-9c18-0cf989f986f5",
        "Antonio José Manzanedo",
    ),
    CardSet::KamigawaNeonDynasty,
    // Six mana for the body or two for a trick: channel is what keeps a
    // top-heavy creature from being a dead card in the early turns.
    CardRules::new_artifact_creature(mana_cost!("{5}{R}"), &["Boar"], 5, 4).with_abilities(&[
        abilities::trample(),
        abilities::haste(),
        AbilityDef::activated_with_targets(
            "Channel — {1}{R}, Discard this card: Target creature gets +3/+1 and gains trample \
             until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{R}")),
                AbilityCostDef::DiscardSource,
            ],
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

// NEO 157 — Rabbit Battery
pub(in crate::card::sets) static RABBIT_BATTERY: CardRecord = CardRecord::new_with_legacy_id(
    1706,
    "Rabbit Battery",
    CardArt::new("5d33a5b7-797b-4079-8d62-edd124c0fb5a", "Justyna Dura"),
    CardSet::KamigawaNeonDynasty,
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
                mana_cost!("{R}"),
                "Reconfigure {R} ({R}: Attach to target creature you control; or unattach from a creature. Reconfigure only as a sorcery. While attached, this isn't a creature.)",
            ),
        ]),
);

// NEO 189 — Greater Tanuki
pub(in crate::card::sets) static GREATER_TANUKI: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b4fbaee3-a10f-4b2d-b07e-d041a96a7e27"),
    "Greater Tanuki",
    CardArt::new("b4fbaee3-a10f-4b2d-b07e-d041a96a7e27", "Ilse Gort"),
    CardSet::KamigawaNeonDynasty,
    // Six mana for the body or three for a land: channel is what makes a
    // top-heavy creature a reasonable card to draw on turn three.
    CardRules::new_enchantment_creature(mana_cost!("{4}{G}{G}"), &["Dog"], 6, 5).with_abilities(&[
        abilities::trample(),
        AbilityDef::activated(
            "Channel — {2}{G}, Discard this card: Search your library for a basic land card, put it onto the battlefield tapped, then shuffle.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{G}")),
                AbilityCostDef::DiscardSource,
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

// NEO 211 — Tamiyo's Safekeeping
pub(in crate::card::sets) static TAMIYO_S_SAFEKEEPING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fd4b7ee2-de65-4288-872d-486065a4f226"),
    "Tamiyo's Safekeeping",
    CardArt::new("fd4b7ee2-de65-4288-872d-486065a4f226", "Aurore Folny"),
    CardSet::KamigawaNeonDynasty,
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

// NEO 222 — Hinata, Dawn-Crowned
pub(in crate::card::sets) static HINATA_DAWN_CROWNED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f25aff90-56fd-4f70-bb3b-cabf2900c391"),
    "Hinata, Dawn-Crowned",
    CardArt::new("f25aff90-56fd-4f70-bb3b-cabf2900c391", "Alexander Mokhov"),
    CardSet::KamigawaNeonDynasty,
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
    PrintingAnchor::scryfall("222a736e-d819-452d-aeda-eb848c4b2302"),
    "Tamiyo, Compleated Sage",
    CardArt::new("222a736e-d819-452d-aeda-eb848c4b2302", "Chris Rahn"),
    CardSet::KamigawaNeonDynasty,
    CardRules::unsupported(),
);

// NEO 248 — Iron Apprentice
// Audit: unsupported — Needs a kind-agnostic counter transfer. "Put those counters on target creature" moves every kind the dying creature had, in the amounts it had, but AddCounters always names one kind and only removal has a kind-agnostic form. Narrowing it to +1/+1 would silently drop any other counter that reached this creature.
pub(in crate::card::sets) static IRON_APPRENTICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("13d6d9fc-509b-42db-8ac1-85066eb6e9c4"),
    "Iron Apprentice",
    crate::card::CardArt::new("13d6d9fc-509b-42db-8ac1-85066eb6e9c4", "Kekai Kotaki"),
    crate::card::CardSet::KamigawaNeonDynasty,
    crate::card::CardRules::unsupported(),
);

// NEO 250 — Mirror Box
// Audit: unsupported — Needs a static value evaluated relative to each affected creature so it can count other creatures sharing that creature's name.
pub(in crate::card::sets) static MIRROR_BOX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d507daa3-3f16-4ab1-81ea-794e5bb488fc"),
    "Mirror Box",
    CardArt::new(
        "d507daa3-3f16-4ab1-81ea-794e5bb488fc",
        "Néstor Ossandón Leal",
    ),
    CardSet::KamigawaNeonDynasty,
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
    PrintingAnchor::scryfall("486d7edc-d983-41f0-8b78-c99aecd72996"),
    "Otawara, Soaring City",
    CardArt::new("486d7edc-d983-41f0-8b78-c99aecd72996", "Alayna Danner"),
    CardSet::KamigawaNeonDynasty,
    // A land that costs nothing to play and is never a dead draw, which is
    // the whole of why the cycle is in the cube.
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_mana(
                "{T}: Add {U}.",
                &[AbilityCostDef::TapSource],
                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue)),
            ),
            AbilityDef::activated_with_cost_list_and_targets(
                "Channel — {3}{U}, Discard this card: Return target artifact, creature, \
                 enchantment, or planeswalker to its owner\'s hand. This ability costs {1} less \
                 to activate for each legendary creature you control.",
                AbilityCostList::two(
                    AbilityCostDef::Mana(mana_cost!("{3}{U}")),
                    AbilityCostDef::DiscardSource,
                ),
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
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
            )
            .with_source_zones(&[ZoneKind::Hand])
            .with_activation_cost_reduction(
                ValueDef::CountMatchingObjects(&LEGENDARY_CREATURES_YOU_CONTROL),
                0,
            ),
        ]),
);

// NEO 357 — Fable of the Mirror-Breaker // Reflection of Kiki-Jiki
pub(in crate::card::sets) static FABLE_OF_THE_MIRROR_BREAKER: CardRecord = CardRecord::new_dfc(
    PrintingAnchor::scryfall("0b696cd1-0d72-4df5-bacc-dc77e62f9a13"),
    "Fable of the Mirror-Breaker // Reflection of Kiki-Jiki",
    CardArt::new("0b696cd1-0d72-4df5-bacc-dc77e62f9a13", "akio"),
    CardSet::KamigawaNeonDynasty,
    // Three mana that pays for itself twice over: a body, a loot, and then
    // the half nobody reads the Saga for.
    &[
        (
            "Fable of the Mirror-Breaker",
            const {
                CardRules::new_enchantment(mana_cost!("{2}{R}"))
                .with_subtypes(&const { ["Saga"] })
                .with_abilities(&const { [
                    abilities::saga_chapter(
                        1,
                        "I — Create a 2/2 red Goblin Shaman creature token with \"Whenever this token attacks, \
                         create a Treasure token.\"",
                        EffectDef::create_creature_token(&const { ["Goblin", "Shaman"] }, &const { [ManaColor::Red] }, 2, 2)
                            // The Goblin's own clause, printed on the token rather than on the Saga.
                            .with_abilities(&const { [AbilityDef::triggered(
                                "Whenever this token attacks, create a Treasure token.",
                                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                                EffectDef::create_token(tokens::treasure()),
                            )] }),
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
                                &const { [ZoneKind::Hand] },
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
                                EffectDef::DiscardCards {
                                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                                },
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
                CardRules::new_creature_without_mana_cost(&const { ["Goblin", "Shaman"] }, 2, 2)
                .with_type(CardType::Enchantment)
                .printed_colors(&const { [ManaColor::Red] })
                .with_abilities(&const { [AbilityDef::activated_with_targets(
                    "{1}, {T}: Create a token that's a copy of another target nonlegendary creature you control, \
                     except it has haste. Sacrifice it at the beginning of the next end step.",
                    &const { [
                        AbilityCostDef::Mana(mana_cost!("{1}")),
                        AbilityCostDef::TapSource,
                    ] },
                    // "Another target nonlegendary creature you control": the Reflection may
                    // not copy itself, and a legendary copy would be put into a graveyard by
                    // the legend rule the moment it arrived.
                    &const { [
                        AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::All(&const { [
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(CardSupertype::Legendary)),
                            ] }),
                            zones: &const { [ZoneKind::Battlefield] },
                            controller: Some(PlayerRelation::You),
                            owner: None,
                        })
                        .excluding_source(),
                    ] },
                    EffectDef::create_token_from_copy(&const { crate::card::TokenCopyDef {
                        object: &EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        exceptions: CopyExceptionsDef::NONE
                            .with_abilities(&const { [CopyAbilityDef::Ability(&abilities::haste())] }),
                    } })
                    .with_created_tokens(CreatedTokensDef {
                        binding: ParentBinding,
                        then: &const {
                            EffectDef::InstallTrigger(InstalledTriggerDef::once(&const {
                                AbilityDef::triggered(
                                    "Sacrifice it at the beginning of the next end step.",
                                    TriggerEventDef::StepBegins {
                                        step: TurnStepDef::End,
                                        player: PlayerRelation::Any,
                                    },
                                    EffectDef::Sacrifice {
                                        object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                            ParentBinding,
                                        )),
                                    },
                                )
                            }))
                        },
                    }),
                )] })
            },
        ),
    ],
);

// NEO 412 — Boseiju, Who Endures
pub(in crate::card::sets) static BOSEIJU_WHO_ENDURES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0055ea30-20fb-4324-a632-8fed87628f05"),
    "Boseiju, Who Endures",
    CardArt::new("0055ea30-20fb-4324-a632-8fed87628f05", "Esuthio"),
    CardSet::KamigawaNeonDynasty,
    // A Forest that answers the one artifact the deck could not otherwise
    // beat, and costs nothing to play when it does not have to.
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_mana(
                "{T}: Add {G}.",
                &[AbilityCostDef::TapSource],
                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
            ),
            AbilityDef::activated_with_cost_list_and_targets(
                "Channel — {1}{G}, Discard this card: Destroy target artifact, enchantment, or \
                 nonbasic land an opponent controls. That player may search their library for a \
                 land card with a basic land type, put it onto the battlefield, then shuffle. \
                 This ability costs {1} less to activate for each legendary creature you control.",
                AbilityCostList::two(
                    AbilityCostDef::Mana(mana_cost!("{1}{G}")),
                    AbilityCostDef::DiscardSource,
                ),
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
                        can_regenerate: true,
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

// NEO 418 — The Wandering Emperor (alternate printing)

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &IMPERIAL_OATH,
    &LION_SASH,
    &TOUCH_THE_SPIRIT_REALM,
    &THE_WANDERING_EMPEROR,
    &MIRRORSHELL_CRAB,
    &MOON_CIRCUIT_HACKER,
    &CLAWING_TORMENT,
    &OKIBA_RECKONER_RAID,
    &IRONHOOF_BOAR,
    &RABBIT_BATTERY,
    &GREATER_TANUKI,
    &TAMIYO_S_SAFEKEEPING,
    &HINATA_DAWN_CROWNED,
    &TAMIYO_COMPLEATED_SAGE,
    &IRON_APPRENTICE,
    &MIRROR_BOX,
    &OTAWARA_SOARING_CITY,
    &FABLE_OF_THE_MIRROR_BREAKER,
    &BOSEIJU_WHO_ENDURES,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::alternate(&THE_WANDERING_EMPEROR, 1), // NEO 418
];
