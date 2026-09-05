//! The Brothers' War cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, CardArt, CardRules, CardSet, CardSupertype, CardType,
    CharacteristicOperationDef, CounterKind, CreatureTypeSetDef, EffectDef, EffectPaymentCostDef,
    EffectPaymentDef, EffectRecipientDef, ManaColor, ManaRestrictionDef, ObjectPredicateDef,
    PayOrDef, PlayerRefDef, PlayerRelation, PlayerSetDef, ResolvedEffectDurationDef,
    SetOperationDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::{TargetIndex, mana_cost};

// BRO 12 — Loran of the Third Path
pub(in crate::card::sets) static LORAN_OF_THE_THIRD_PATH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("59faa45d-868b-4bc7-934c-0e077642e129"),
    "Loran of the Third Path",
    CardArt::new("59faa45d-868b-4bc7-934c-0e077642e129", "Steven Belledin"),
    CardSet::TheBrothersWar,
    // Three mana for an answer to an artifact, a body that blocks, and a
    // symmetrical draw nobody else gets to use as often as you do.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Artificer"], 2, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::vigilance(),
            abilities::enters_trigger_with_targets(
                "When this creature enters, destroy up to one target artifact or enchantment.",
                // "Up to one target artifact or enchantment": an Loran with nothing worth
                // answering still arrives as a 2/1 that draws.
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    1,
                )],
                EffectDef::destroy_target(TargetIndex::PRIMARY, true),
            ),
            AbilityDef::activated_with_targets(
                "{T}: You and target opponent each draw a card.",
                &[AbilityCostDef::TapSource],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                // "You and target opponent each draw a card." Two draws rather than one
                // instruction naming both, because only one of them is targeted: the
                // opponent has to be a legal target and you never are.
                EffectDef::Sequence(&[
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
        ]),
);

// BRO 72 — Weakstone's Subjugation
static AN_ARTIFACT_OR_CREATURE: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasType(CardType::Artifact),
    ObjectPredicateDef::HasType(CardType::Creature),
]);

pub(in crate::card::sets) static WEAKSTONE_S_SUBJUGATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ef93ac79-8575-40f8-a222-63c2ffb30f60"),
    "Weakstone's Subjugation",
    CardArt::new("ef93ac79-8575-40f8-a222-63c2ffb30f60", "Igor Kieryluk"),
    CardSet::TheBrothersWar,
    // One mana to hold a permanent down permanently; the {3} is only for
    // catching one that is already untapped.
    CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Aura"])
        .enchanting(AN_ARTIFACT_OR_CREATURE)
        .with_abilities(&[
            abilities::aura_spell(
                "Enchant artifact or creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    AN_ARTIFACT_OR_CREATURE,
                )],
            ),
            abilities::enters_trigger(
                "When this Aura enters, you may pay {3}. If you do, tap enchanted permanent.",
                EffectDef::PayOr(PayOrDef::optional(
                    EffectPaymentDef {
                        payer: PlayerSetDef::One(PlayerRefDef::EffectController),
                        cost: EffectPaymentCostDef::GenericMana(ValueDef::Constant(3)),
                    },
                    &const {
                        EffectDef::Tap {
                            object: EffectRecipientDef::AttachedPermanent,
                        }
                    },
                )),
            ),
            AbilityDef::static_ability(
                "Enchanted permanent doesn't untap during its controller's untap step.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
                },
            ),
        ]),
);

// BRO 98 — Gixian Infiltrator
pub(in crate::card::sets) static GIXIAN_INFILTRATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c94a3317-7d1f-4f29-8353-180f1ab48d18"),
    "Gixian Infiltrator",
    CardArt::new("c94a3317-7d1f-4f29-8353-180f1ab48d18", "Peter Polach"),
    CardSet::TheBrothersWar,
    // Any permanent, not just a creature, which is what makes it a payoff
    // for the artifact deck this set is built around.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Phyrexian", "Human"], 2, 1).with_ability(
        AbilityDef::triggered(
            "Whenever you sacrifice another permanent, put a +1/+1 counter on this creature.",
            TriggerEventDef::Sacrificed {
                // "Another" excludes this creature, so sacrificing it to its
                // own outlet never grows a body that has already left.
                object: ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                player: PlayerRelation::You,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// BRO 164 — Scrapwork Mutt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCRAPWORK_MUTT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4742800a-4872-4c2d-b884-01e0ba16950c"),
    "Scrapwork Mutt",
    crate::card::CardArt::new(
        "4742800a-4872-4c2d-b884-01e0ba16950c",
        "Sidharth Chaturvedi",
    ),
    crate::card::CardSet::TheBrothersWar,
    crate::card::CardRules::unsupported(),
);

// BRO 199 — Haywire Mite
pub(in crate::card::sets) static HAYWIRE_MITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("847a175e-ead1-4596-baf3-5f7f57859e0b"),
    "Haywire Mite",
    CardArt::new("847a175e-ead1-4596-baf3-5f7f57859e0b", "Izzy"),
    CardSet::TheBrothersWar,
    // One mana for a body that is never dead: it answers whichever artifact
    // or enchantment the format is afraid of this week, and every deck can
    // cast it whether or not it can pay the green.
    CardRules::new_artifact_creature(mana_cost!("{1}"), &["Insect"], 1, 1).with_abilities(&[
        abilities::dies_trigger(
            "When this creature dies, you gain 2 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
        AbilityDef::activated_with_targets(
            "{G}, Sacrifice this creature: Exile target noncreature artifact or noncreature \
             enchantment.",
            &[
                AbilityCostDef::Mana(mana_cost!("{G}")),
                AbilityCostDef::SacrificeSource,
            ],
            // "Noncreature artifact or noncreature enchantment." The two types are
            // alternatives and the exclusion applies to both, so it sits outside the
            // choice rather than inside it -- which is what leaves a creature that
            // happens to be an artifact alone.
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                        ]),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// BRO 223 — Third Path Iconoclast
pub(in crate::card::sets) static THIRD_PATH_ICONOCLAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f1a21287-e244-4960-84fb-c4f6e5c346d9"),
    "Third Path Iconoclast",
    CardArt::new("f1a21287-e244-4960-84fb-c4f6e5c346d9", "Manuel Castañón"),
    CardSet::TheBrothersWar,
    // Two mana for a body that turns every cantrip into an artifact
    // creature, which is what the deck around it is counting.
    CardRules::new_creature(mana_cost!("{U}{R}"), &["Human", "Monk"], 2, 1).with_ability(
        AbilityDef::triggered(
            "Whenever you cast a noncreature spell, create a 1/1 colorless Soldier artifact \
             creature token.",
            // A noncreature spell of your own. What it does is no part of the trigger:
            // the Soldier arrives whether the spell resolves, is countered, or is
            // answered on the stack.
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
            ])),
            EffectDef::create_artifact_creature_token(&["Soldier"], &[], 1, 1),
        ),
    ),
);

// BRO 238 — The Mightstone and Weakstone
pub(in crate::card::sets) static THE_MIGHTSTONE_AND_WEAKSTONE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("02aea379-b444-46a3-82f4-3038f698d4f4"),
    "The Mightstone and Weakstone",
    CardArt::new("02aea379-b444-46a3-82f4-3038f698d4f4", "Ryan Pancoast"),
    CardSet::TheBrothersWar,
    // Five mana for two cards or a dead creature, and two mana a turn
    // afterwards. The meld is Urza's ability rather than this card's: the
    // parenthesis here only says which card it pairs with.
    CardRules::new_artifact(mana_cost!("{5}"))
        .with_supertype(CardSupertype::Legendary)
        .with_subtypes(&["Powerstone"])
        .with_abilities(&[
            AbilityDef::modal_triggered(
                "When this artifact enters, choose one —\n• Draw two cards.\n• Target creature \
                 gets -5/-5 until end of turn.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                &[
                    AbilityDef::spell(
                        "Draw two cards.",
                        EffectDef::DrawCards {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(2),
                        },
                    ),
                    AbilityDef::spell_with_targets(
                        "Target creature gets -5/-5 until end of turn.",
                        &[AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::HasType(CardType::Creature),
                        )],
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            effect: AppliedEffectDef::modify_power_toughness(
                                ValueDef::Constant(-5),
                                ValueDef::Constant(-5),
                            ),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    ),
                ],
            ),
            AbilityDef::activated_mana(
                "{T}: Add {C}{C}. This mana can't be spent to cast nonartifact spells.",
                &[AbilityCostDef::TapSource],
                EffectDef::AddMana(
                    AddManaEffectDef::one(ManaColor::Colorless)
                        .with_amount(2)
                        // A Powerstone's restriction is a prohibition rather than a permission:
                        // this mana activates abilities and pays for artifact spells, and the one
                        // thing it cannot do is cast a spell that is not an artifact.
                        .with_restrictions(&[ManaRestrictionDef::CannotCastSpell(
                            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                                CardType::Artifact,
                            )),
                        )]),
                ),
            ),
        ]),
);

// BRO 240 — Portal to Phyrexia
pub(in crate::card::sets) static PORTAL_TO_PHYREXIA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5f608efc-0dbc-4cc3-aadd-ed473bfc29ab"),
    "Portal to Phyrexia",
    CardArt::new("5f608efc-0dbc-4cc3-aadd-ed473bfc29ab", "Svetlin Velinov"),
    CardSet::TheBrothersWar,
    // Nine mana, and the game is over: three of their creatures die on the
    // way in and one comes back for you every upkeep afterwards.
    CardRules::new_artifact(mana_cost!("{9}")).with_abilities(&[
        abilities::enters_trigger(
            "When this artifact enters, each opponent sacrifices three creatures of their choice.",
            EffectDef::SacrificeOfChoice {
                player: EffectRecipientDef::Opponent,
                object: ObjectPredicateDef::HasType(CardType::Creature),
                count: ValueDef::Constant(3),
                then: None,
                amount: crate::card::SacrificedAmountDef::Power,
                otherwise: None,
                optional: false,
            },
        ),
        AbilityDef::triggered_with_targets(
            "At the beginning of your upkeep, put target creature card from a graveyard onto the \
             battlefield under your control. It's a Phyrexian in addition to its other types.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            // Any graveyard, not only yours: the Portal is as happy to take back what
            // it made an opponent sacrifice as anything of your own.
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
                    // "It's a Phyrexian in addition to its other types." Added rather than set:
                    // what comes back through the Portal keeps whatever it already was, and is
                    // a Phyrexian as well.
                    effect: AppliedEffectDef::Characteristic(
                        CharacteristicOperationDef::CreatureTypes(SetOperationDef::Add(
                            CreatureTypeSetDef::named(&["Phyrexian"]),
                        )),
                    ),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
            },
        ),
    ]),
);

// BRO 266 — Tocasia's Dig Site
pub(in crate::card::sets) static TOCASIA_S_DIG_SITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("23d4b90c-95b1-4828-bc08-7067da0d5364"),
    "Tocasia's Dig Site",
    CardArt::new("23d4b90c-95b1-4828-bc08-7067da0d5364", "Nadia Hurianova"),
    CardSet::TheBrothersWar,
    // Colourless and untapped, so the sink costs nothing to include: it is
    // a Wastes that does something on the turns nothing else does.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{3}, {T}: Surveil 1.",
            &[
                AbilityCostDef::Mana(mana_cost!("{3}")),
                AbilityCostDef::TapSource,
            ],
            abilities::surveil(ValueDef::Constant(1)),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &LORAN_OF_THE_THIRD_PATH,
    &WEAKSTONE_S_SUBJUGATION,
    &GIXIAN_INFILTRATOR,
    &SCRAPWORK_MUTT,
    &HAYWIRE_MITE,
    &THIRD_PATH_ICONOCLAST,
    &THE_MIGHTSTONE_AND_WEAKSTONE,
    &PORTAL_TO_PHYREXIA,
    &TOCASIA_S_DIG_SITE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
