//! GPT card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostList, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, CardArt, CardRules, CardSet, CardSupertype, CardType,
    CostDef, EffectDef, EffectPaymentDef, EffectRecipientDef, ManaColor, ManaCost,
    ObjectPredicateDef, PayOrDef, PlayerRelation, PlayerSetDef, ReplacementEffectDef,
    ReplacementEventDef, ResolvedEffectDurationDef, TriggerConditionDef, TriggerEventDef,
    TurnStepDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::{TargetIndex, mana_cost};

// GPT 10 — Leyline of the Meek
pub(in crate::card::sets) static LEYLINE_OF_THE_MEEK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("efc58757-abcc-41c9-b4d2-e70e9f387cbb"),
    "Leyline of the Meek",
    CardArt::new("efc58757-abcc-41c9-b4d2-e70e9f387cbb", "Mark Zug"),
    CardSet::Guildpact,
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}")).with_abilities(&[
        abilities::begin_game_on_battlefield(),
        AbilityDef::static_ability(
            "Creature tokens get +1/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Token,
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
            },
        ),
    ]),
);

// GPT 29 — Leyline of Singularity
pub(in crate::card::sets) static LEYLINE_OF_SINGULARITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d40d7e5c-3b6d-4e42-b495-b3cd7ae0d808"),
    "Leyline of Singularity",
    CardArt::new(
        "d40d7e5c-3b6d-4e42-b495-b3cd7ae0d808",
        "Zoltan Boros & Gabor Szikszai",
    ),
    CardSet::Guildpact,
    CardRules::new_enchantment(mana_cost!("{2}{U}{U}")).with_abilities(&[
        abilities::begin_game_on_battlefield(),
        AbilityDef::static_ability(
            "All nonland permanents are legendary.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::add_supertype(CardSupertype::Legendary),
            },
        ),
    ]),
);

// GPT 52 — Leyline of the Void
pub(in crate::card::sets) static LEYLINE_OF_THE_VOID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("37dfe8b8-b39e-4e70-9e5b-be42c93b4f70"),
    "Leyline of the Void",
    CardArt::new("37dfe8b8-b39e-4e70-9e5b-be42c93b4f70", "Adam Rex"),
    CardSet::Guildpact,
    CardRules::new_enchantment(mana_cost!("{2}{B}{B}")).with_abilities(&[
        abilities::begin_game_on_battlefield(),
        AbilityDef::replacement_for(
            "If a card would be put into an opponent's graveyard from anywhere, exile it instead.",
            ReplacementEventDef::AnyObjectWouldMove {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::OwnedBy(PlayerRelation::Opponent),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                ]),
                to: ZoneKind::Graveyard,
            },
            ReplacementEffectDef::MoveToZone(ZoneKind::Exile),
        ),
    ]),
);

/// The Rusalka cycle's cost: one mana of the creature's own colour plus a
/// creature. Each Rusalka is itself a legal sacrifice for its own ability, so
/// the last body on the board can still pay.
///
/// A cost list rather than a slice, because the mana half is a parameter and
/// a slice holding it could not be given a `'static` lifetime.
const fn rusalka_sacrifice(mana: ManaCost) -> AbilityCostList {
    AbilityCostList::two(
        CostDef::Mana(mana),
        CostDef::SacrificePermanent {
            object: ObjectPredicateDef::HasType(CardType::Creature),
            controller: PlayerRelation::You,
        },
    )
}

// GPT 56 — Plagued Rusalka
pub(in crate::card::sets) static PLAGUED_RUSALKA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cd84bbb3-8b99-4e6d-b514-b094ec93eaa0"),
    "Plagued Rusalka",
    CardArt::new(
        "cd84bbb3-8b99-4e6d-b514-b094ec93eaa0",
        "Alex Horley-Orlandelli",
    ),
    CardSet::Guildpact,
    // A sacrifice outlet that also finishes off a one-toughness creature,
    // which is what makes feeding it a real line rather than a last resort.
    CardRules::new_creature(mana_cost!("{B}"), &["Spirit"], 1, 1).with_ability(
        AbilityDef::activated_with_cost_list_and_targets(
            "{B}, Sacrifice a creature: Target creature gets -1/-1 until end of turn.",
            rusalka_sacrifice(mana_cost!("{B}")),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(-1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// GPT 64 — Bloodscale Prowler
pub(in crate::card::sets) static BLOODSCALE_PROWLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8197cc43-c787-4372-81dc-759a9fe24708"),
    "Bloodscale Prowler",
    CardArt::new("8197cc43-c787-4372-81dc-759a9fe24708", "Lars Grant-West"),
    CardSet::Guildpact,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Lizard", "Warrior"], 3, 1)
        .with_ability(abilities::bloodthirst(1)),
);

// GPT 68 — Leyline of Lightning
pub(in crate::card::sets) static LEYLINE_OF_LIGHTNING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("23d09839-b41e-4aab-8913-40d63052dbf3"),
    "Leyline of Lightning",
    CardArt::new("23d09839-b41e-4aab-8913-40d63052dbf3", "Paolo Parente"),
    CardSet::Guildpact,
    CardRules::new_enchantment(mana_cost!("{2}{R}{R}")).with_abilities(&[
        abilities::begin_game_on_battlefield(),
        AbilityDef::triggered_with_targets(
            "Whenever you cast a spell, you may pay {1}. If you do, this enchantment deals 1 damage to target player or planeswalker.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(
                PlayerRelation::You,
            )),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )],
            EffectDef::PayOr(PayOrDef::optional(
                EffectPaymentDef::mana(
                    PlayerSetDef::Related(PlayerRelation::You),
                    mana_cost!("{1}"),
                ),
                &EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
            )),
        ),
    ]),
);

// GPT 74 — Scorched Rusalka
pub(in crate::card::sets) static SCORCHED_RUSALKA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9f955164-ddb8-484c-a063-967621abce87"),
    "Scorched Rusalka",
    CardArt::new("9f955164-ddb8-484c-a063-967621abce87", "Luca Zontini"),
    CardSet::Guildpact,
    // A sacrifice outlet that turns every dying creature into reach, which
    // is what an aggressive deck wants from a one-drop.
    CardRules::new_creature(mana_cost!("{R}"), &["Spirit"], 1, 1).with_ability(
        AbilityDef::activated_with_cost_list_and_targets(
            "{R}, Sacrifice a creature: This creature deals 1 damage to target player or \
             planeswalker.",
            rusalka_sacrifice(mana_cost!("{R}")),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// GPT 77 — Skarrgan Firebird
pub(in crate::card::sets) static SKARRGAN_FIREBIRD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("61c51e46-3236-41ee-913e-f253f218067c"),
    "Skarrgan Firebird",
    CardArt::new("61c51e46-3236-41ee-913e-f253f218067c", "Kev Walker"),
    CardSet::Guildpact,
    CardRules::new_creature(mana_cost!("{4}{R}{R}"), &["Phoenix"], 3, 3).with_abilities(&[
        abilities::bloodthirst(3),
        abilities::flying(),
        AbilityDef::activated(
            "{R}{R}{R}: Return this card from your graveyard to your hand. Activate only if an opponent was dealt damage this turn.",
            &[CostDef::Mana(mana_cost!("{R}{R}{R}"))],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Source,
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        )
        .with_activation_condition(&TriggerConditionDef::OpponentWasDealtDamageThisTurn)
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// GPT 87 — Gristleback
pub(in crate::card::sets) static GRISTLEBACK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b82f763a-c960-4b59-8c77-f3bea7bd8c8b"),
    "Gristleback",
    CardArt::new("b82f763a-c960-4b59-8c77-f3bea7bd8c8b", "Lars Grant-West"),
    CardSet::Guildpact,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Boar", "Beast"], 2, 2).with_abilities(&[
        abilities::bloodthirst(1),
        AbilityDef::activated(
            "Sacrifice this creature: You gain life equal to its power.",
            &[CostDef::SacrificeSource],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::SourcePower,
            },
        ),
    ]),
);

// GPT 90 — Leyline of Lifeforce
pub(in crate::card::sets) static LEYLINE_OF_LIFEFORCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f7caffa7-29bd-455c-9770-94a0ad7ef5e3"),
    "Leyline of Lifeforce",
    CardArt::new("f7caffa7-29bd-455c-9770-94a0ad7ef5e3", "Kev Walker"),
    CardSet::Guildpact,
    CardRules::new_enchantment(mana_cost!("{2}{G}{G}")).with_abilities(&[
        abilities::begin_game_on_battlefield(),
        AbilityDef::static_ability(
            "Creature spells can't be countered.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Stack],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered),
            },
        ),
    ]),
);

// GPT 125 — Pillory of the Sleepless
pub(in crate::card::sets) static PILLORY_OF_THE_SLEEPLESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("36964bbd-f068-4a69-8d6b-7e4e97938b98"),
    "Pillory of the Sleepless",
    CardArt::new("36964bbd-f068-4a69-8d6b-7e4e97938b98", "Mark Romanoski"),
    CardSet::Guildpact,
    // A Pacifism that also closes the game, which is what the second colour
    // and the extra mana are buying.
    CardRules::new_enchantment(mana_cost!("{1}{W}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            abilities::enchanted_creature_pacified(),
            AbilityDef::static_ability(
                "Enchanted creature has \"At the beginning of your upkeep, you lose 1 life.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    // Granted to the creature rather than kept on the Aura,
                    // which is what the printed quotation marks mean: "your"
                    // upkeep is the creature's controller's, and an effect
                    // that strips the creature's abilities turns this off.
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

// GPT 158 — Gruul Turf
pub(in crate::card::sets) static GRUUL_TURF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("550b70e0-ebd5-49de-b62c-5224b8bf8e98"),
    "Gruul Turf",
    CardArt::new("550b70e0-ebd5-49de-b62c-5224b8bf8e98", "John Avon"),
    CardSet::Guildpact,
    // The red-green karoo; only the two colours below are its own.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::karoo_bounce(),
        AbilityDef::activated_mana(
            "{T}: Add {R}{G}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one_of_each(
                ManaColor::Red,
                ManaColor::Green,
            )),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &LEYLINE_OF_THE_MEEK,
    &LEYLINE_OF_SINGULARITY,
    &LEYLINE_OF_THE_VOID,
    &PLAGUED_RUSALKA,
    &BLOODSCALE_PROWLER,
    &LEYLINE_OF_LIGHTNING,
    &SCORCHED_RUSALKA,
    &SKARRGAN_FIREBIRD,
    &GRISTLEBACK,
    &LEYLINE_OF_LIFEFORCE,
    &PILLORY_OF_THE_SLEEPLESS,
    &GRUUL_TURF,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
