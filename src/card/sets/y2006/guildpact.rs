//! GPT card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::CastTimingPermissionDef;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::PayOrDef;
use crate::card::PlayerRelation;
use crate::card::ReplacementEffectDef;
use crate::card::ReplacementEventDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new("GPT", "guildpact");

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// GPT 10 — Leyline of the Meek
pub(in crate::card::sets) static LEYLINE_OF_THE_MEEK: CardRecord = CardRecord::new(
    "Leyline of the Meek",
    "efc58757-abcc-41c9-b4d2-e70e9f387cbb",
    "Mark Zug",
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
    "Leyline of Singularity",
    "d40d7e5c-3b6d-4e42-b495-b3cd7ae0d808",
    "Zoltan Boros & Gabor Szikszai",
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

// GPT 31 — Quicken
pub(in crate::card::sets) static QUICKEN: CardRecord = CardRecord::new(
    "Quicken",
    "7a276b12-4647-4223-b89e-f55d72feb2d0",
    "Aleksi Briclot",
// One spell ability per part, so the card's two sentences are one clause
    // with a sequence rather than two spell clauses.
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell(
        "The next sorcery spell you cast this turn can be cast as though it had flash. (It can be cast any time you could cast an instant.)\nDraw a card.",
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayCastAsThoughItHadFlash(
                    CastTimingPermissionDef::new(ObjectPredicateDef::HasType(CardType::Sorcery)),
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn
                    .or(ResolvedEffectDurationDef::UntilNextMatchingCast),
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// GPT 32 — Repeal
pub(in crate::card::sets) static REPEAL: CardRecord = CardRecord::new(
    "Repeal",
    "9e7dd929-4bba-46a6-86c9-b8ed853eb721",
    "Dan Murayama Scott",
    // X is paid to match what it answers rather than to make it bigger, so
    // the cantrip is what keeps a one-mana mode from being a wasted card.
    CardRules::new_instant(mana_cost!("{X}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return target nonland permanent with mana value X to its owner's hand. Draw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                ObjectPredicateDef::ManaValueEqualTo(ValueDef::ChosenX),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
            // The draw is unconditional: it still happens when the target
            // has left before this resolves.
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// GPT 52 — Leyline of the Void
pub(in crate::card::sets) static LEYLINE_OF_THE_VOID: CardRecord = CardRecord::new(
    "Leyline of the Void",
    "37dfe8b8-b39e-4e70-9e5b-be42c93b4f70",
    "Adam Rex",
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

// GPT 56 — Plagued Rusalka
pub(in crate::card::sets) static PLAGUED_RUSALKA: CardRecord = CardRecord::new(
    "Plagued Rusalka",
    "cd84bbb3-8b99-4e6d-b514-b094ec93eaa0",
    "Alex Horley-Orlandelli",
    // A sacrifice outlet that also finishes off a one-toughness creature,
    // which is what makes feeding it a real line rather than a last resort.
    CardRules::new_creature(mana_cost!("{B}"), &["Spirit"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{B}, Sacrifice a creature: Target creature gets -1/-1 until end of turn.",
            &[
                CostDef::Mana(mana_cost!("{B}")),
                CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    controller: PlayerRelation::You,
                },
            ],
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
    "Bloodscale Prowler",
    "8197cc43-c787-4372-81dc-759a9fe24708",
    "Lars Grant-West",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Lizard", "Warrior"], 3, 1)
        .with_ability(abilities::bloodthirst(1)),
);

// GPT 68 — Leyline of Lightning
pub(in crate::card::sets) static LEYLINE_OF_LIGHTNING: CardRecord = CardRecord::new(
    "Leyline of Lightning",
    "23d09839-b41e-4aab-8913-40d63052dbf3",
    "Paolo Parente",
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
                &[CostDef::Mana(mana_cost!("{1}"))],
                &EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(1),
                ),
            )),
        ),
    ]),
);

// GPT 74 — Scorched Rusalka
pub(in crate::card::sets) static SCORCHED_RUSALKA: CardRecord = CardRecord::new(
    "Scorched Rusalka",
    "9f955164-ddb8-484c-a063-967621abce87",
    "Luca Zontini",
    // A sacrifice outlet that turns every dying creature into reach, which
    // is what an aggressive deck wants from a one-drop.
    CardRules::new_creature(mana_cost!("{R}"), &["Spirit"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{R}, Sacrifice a creature: This creature deals 1 damage to target player or \
             planeswalker.",
            &[
                CostDef::Mana(mana_cost!("{R}")),
                CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    controller: PlayerRelation::You,
                },
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
        ),
    ),
);

// GPT 77 — Skarrgan Firebird
pub(in crate::card::sets) static SKARRGAN_FIREBIRD: CardRecord = CardRecord::new(
    "Skarrgan Firebird",
    "61c51e46-3236-41ee-913e-f253f218067c",
    "Kev Walker",
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
    "Gristleback",
    "b82f763a-c960-4b59-8c77-f3bea7bd8c8b",
    "Lars Grant-West",
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
    "Leyline of Lifeforce",
    "f7caffa7-29bd-455c-9770-94a0ad7ef5e3",
    "Kev Walker",
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

// GPT 112 — Feral Animist
pub(in crate::card::sets) static FERAL_ANIMIST: CardRecord = CardRecord::new(
    "Feral Animist",
    "ad65c721-b601-46f9-ba0b-ac4d96567cee",
    "Ron Spears",
    CardRules::new_creature(mana_cost!("{1}{R}{G}"), &["Goblin", "Shaman"], 2, 1).with_ability(
        abilities::apply_to_self_until_end_of_turn(
            "{3}: This creature gets +X/+0 until end of turn, where X is its power.",
            &[CostDef::Mana(mana_cost!("{3}"))],
            AppliedEffectDef::modify_power_toughness(ValueDef::SourcePower, ValueDef::Constant(0)),
        ),
    ),
);

// GPT 125 — Pillory of the Sleepless
pub(in crate::card::sets) static PILLORY_OF_THE_SLEEPLESS: CardRecord = CardRecord::new(
    "Pillory of the Sleepless",
    "36964bbd-f068-4a69-8d6b-7e4e97938b98",
    "Mark Romanoski",
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

// GPT 157 — Godless Shrine
pub(in crate::card::sets) static GODLESS_SHRINE: CardRecord = CardRecord::new(
    "Godless Shrine",
    "be010c2f-06db-47e3-80bd-df3f2a21ca34",
    "Rob Alexander",
    CardRules::new_land(&["Plains", "Swamp"]).with_ability(abilities::shock_land_enters()),
);

// GPT 158 — Gruul Turf
pub(in crate::card::sets) static GRUUL_TURF: CardRecord = CardRecord::new(
    "Gruul Turf",
    "550b70e0-ebd5-49de-b62c-5224b8bf8e98",
    "John Avon",
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

// GPT 159 — Izzet Boilerworks
pub(in crate::card::sets) static IZZET_BOILERWORKS: CardRecord = CardRecord::new(
    "Izzet Boilerworks",
    "666f455e-3a3d-475d-b67a-a1fdd74820eb",
    "John Avon",
    // The last of the ten karoos; only the two colours below are its own.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::karoo_bounce(),
        AbilityDef::activated_mana(
            "{T}: Add {U}{R}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one_of_each(
                ManaColor::Blue,
                ManaColor::Red,
            )),
        ),
    ]),
);

// GPT 161 — Orzhov Basilica
pub(in crate::card::sets) static ORZHOV_BASILICA: CardRecord = CardRecord::new(
    "Orzhov Basilica",
    "f9154d2a-3fc5-4fd6-9885-a810cb6b542a",
    "John Avon",
    // The white-black karoo; only the two colours below are its own.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::karoo_bounce(),
        AbilityDef::activated_mana(
            "{T}: Add {W}{B}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one_of_each(
                ManaColor::White,
                ManaColor::Black,
            )),
        ),
    ]),
);

// GPT 164 — Steam Vents
pub(in crate::card::sets) static STEAM_VENTS: CardRecord = CardRecord::new(
    "Steam Vents",
    "054f2276-2dd5-43da-bb26-c57c560861fe",
    "Rob Alexander",
    CardRules::new_land(&["Island", "Mountain"]).with_ability(abilities::shock_land_enters()),
);

// GPT 165 — Stomping Ground
pub(in crate::card::sets) static STOMPING_GROUND: CardRecord = CardRecord::new(
    "Stomping Ground",
    "a2773d8f-f906-475d-aaff-b7ca3b01f188",
    "Rob Alexander",
    CardRules::new_land(&["Mountain", "Forest"]).with_ability(abilities::shock_land_enters()),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &LEYLINE_OF_THE_MEEK,
    &LEYLINE_OF_SINGULARITY,
    &QUICKEN,
    &REPEAL,
    &LEYLINE_OF_THE_VOID,
    &PLAGUED_RUSALKA,
    &BLOODSCALE_PROWLER,
    &LEYLINE_OF_LIGHTNING,
    &SCORCHED_RUSALKA,
    &SKARRGAN_FIREBIRD,
    &GRISTLEBACK,
    &LEYLINE_OF_LIFEFORCE,
    &FERAL_ANIMIST,
    &PILLORY_OF_THE_SLEEPLESS,
    &GODLESS_SHRINE,
    &GRUUL_TURF,
    &IZZET_BOILERWORKS,
    &ORZHOV_BASILICA,
    &STEAM_VENTS,
    &STOMPING_GROUND,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
