//! The Brothers' War cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, CardRules, CardSet, CardSupertype, CardType, CharacteristicOperationDef,
    CreatureTypeSetDef, EffectDef, EffectRecipientDef, ManaColor, ManaRestrictionDef,
    ObjectPredicateDef, PlayerRelation, ResolvedEffectDurationDef, SetOperationDef,
    TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::{TargetIndex, mana_cost};

// BRO 12 — Loran of the Third Path
pub(in crate::card::sets) static LORAN_OF_THE_THIRD_PATH: CardRecord = CardRecord::new(
    CardSet::TheBrothersWar,
    "Loran of the Third Path",
    "59faa45d-868b-4bc7-934c-0e077642e129",
    "Steven Belledin",
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WEAKSTONE_S_SUBJUGATION: CardRecord = CardRecord::new(
    crate::card::CardSet::TheBrothersWar,
    "Weakstone's Subjugation",
    "ef93ac79-8575-40f8-a222-63c2ffb30f60",
    "Igor Kieryluk",
    crate::card::CardRules::unsupported(),
);

// BRO 98 — Gixian Infiltrator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GIXIAN_INFILTRATOR: CardRecord = CardRecord::new(
    crate::card::CardSet::TheBrothersWar,
    "Gixian Infiltrator",
    "c94a3317-7d1f-4f29-8353-180f1ab48d18",
    "Peter Polach",
    crate::card::CardRules::unsupported(),
);

// BRO 164 — Scrapwork Mutt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCRAPWORK_MUTT: CardRecord = CardRecord::new(
    crate::card::CardSet::TheBrothersWar,
    "Scrapwork Mutt",
    "4742800a-4872-4c2d-b884-01e0ba16950c",
    "Sidharth Chaturvedi",
    crate::card::CardRules::unsupported(),
);

// BRO 174 — Bushwhack
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BUSHWHACK: CardRecord = CardRecord::new(
    crate::card::CardSet::TheBrothersWar,
    "Bushwhack",
    "712a0640-d9c8-46fc-b38b-bf20a40fa902",
    "Artur Nakhodkin",
    crate::card::CardRules::unsupported(),
);

// BRO 199 — Haywire Mite
pub(in crate::card::sets) static HAYWIRE_MITE: CardRecord = CardRecord::new(
    CardSet::TheBrothersWar,
    "Haywire Mite",
    "847a175e-ead1-4596-baf3-5f7f57859e0b",
    "Izzy",
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
    CardSet::TheBrothersWar,
    "Third Path Iconoclast",
    "f1a21287-e244-4960-84fb-c4f6e5c346d9",
    "Manuel Castañón",
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
    CardSet::TheBrothersWar,
    "The Mightstone and Weakstone",
    "02aea379-b444-46a3-82f4-3038f698d4f4",
    "Ryan Pancoast",
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
    CardSet::TheBrothersWar,
    "Portal to Phyrexia",
    "5f608efc-0dbc-4cc3-aadd-ed473bfc29ab",
    "Svetlin Velinov",
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOCASIA_S_DIG_SITE: CardRecord = CardRecord::new(
    crate::card::CardSet::TheBrothersWar,
    "Tocasia's Dig Site",
    "23d4b90c-95b1-4828-bc08-7067da0d5364",
    "Nadia Hurianova",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &LORAN_OF_THE_THIRD_PATH,
    &WEAKSTONE_S_SUBJUGATION,
    &GIXIAN_INFILTRATOR,
    &SCRAPWORK_MUTT,
    &BUSHWHACK,
    &HAYWIRE_MITE,
    &THIRD_PATH_ICONOCLAST,
    &THE_MIGHTSTONE_AND_WEAKSTONE,
    &PORTAL_TO_PHYREXIA,
    &TOCASIA_S_DIG_SITE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
