//! Foundations Jumpstart cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ComparisonDef;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::DamageEventMatcherDef;
use crate::card::DamageKindDef;
use crate::card::DamageRecipientMatcherDef;
use crate::card::DamageSourceMatcherDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::SubtypeDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::card::sets::y2005::ravnica_city_of_guilds as catalog_rav;
use crate::card::tokens;
use crate::ids::TargetIndex;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "J25",
    slug: "foundations-jumpstart",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const BLOOD_TOKEN: TokenCharacteristics = tokens::blood().with_art(CardArt::new(
    "a6f374bc-cd29-469f-808a-6a6c004ee8aa",
    "Miranda Meeks",
));

// J25 1 — Dawnwing Marshal
pub(in crate::card::sets) static DAWNWING_MARSHAL: CardRecord = CardRecord::new(
    "Dawnwing Marshal",
    "51258ab9-25f6-4617-9499-b17cf7a8db06",
    "Aldo Domínguez",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Cat", "Soldier"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{4}{W}: Creatures you control get +1/+1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{4}{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// J25 5 — Hinterland Sanctifier
pub(in crate::card::sets) static HINTERLAND_SANCTIFIER: CardRecord = CardRecord::new(
    "Hinterland Sanctifier",
    "7bd187b7-5001-4fc3-8c1e-7093827027ee",
    "Justine Cruz",
    CardRules::new_creature(mana_cost!("{W}"), &["Rabbit", "Cleric"], 1, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever another creature you control enters, you gain 1 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// J25 9 — Starlight Snare
pub(in crate::card::sets) static STARLIGHT_SNARE: CardRecord = CardRecord::new(
    "Starlight Snare",
    "80ab3040-fec2-4a65-8825-e6a1132601d1",
    "Borja Pindado",
    CardRules::new_enchantment(mana_cost!("{2}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            abilities::enters_trigger(
                "When this Aura enters, tap enchanted creature.",
                EffectDef::Tap {
                    object: EffectRecipientDef::AttachedPermanent,
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature doesn't untap during its controller's \
                 untap step.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
                },
            ),
        ]),
);

// J25 13 — Dropkick Bomber
pub(in crate::card::sets) static DROPKICK_BOMBER: CardRecord = CardRecord::new(
    "Dropkick Bomber",
    "a5f9a7bb-4ace-4720-8651-08428494223f",
    "Quintin Gleim",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Warrior"], 2, 3).with_abilities(&[
        AbilityDef::static_ability(
            "Other Goblins you control get +1/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Goblin")),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
            },
        ),
        AbilityDef::activated_with_targets(
            "{R}: Until end of turn, another target Goblin you control \
             gains flying and \"When this creature deals combat damage, \
             sacrifice it.\"",
            &[CostDef::Mana(mana_cost!("{R}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Goblin")),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_ability(&abilities::flying()),
                    AppliedEffectDef::add_ability(&AbilityDef::triggered(
                        "When this creature deals combat damage, sacrifice it.",
                        TriggerEventDef::DamageDealt(DamageEventMatcherDef {
                            kind: DamageKindDef::Combat,
                            source: DamageSourceMatcherDef::Object(ObjectRefDef::Source),
                            recipient: DamageRecipientMatcherDef::Any,
                        }),
                        EffectDef::sacrifice(EffectRecipientDef::Source),
                    )),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// J25 14 — Firespitter Whelp
pub(in crate::card::sets) static FIRESPITTER_WHELP: CardRecord = CardRecord::new(
    "Firespitter Whelp",
    "2f7cff11-c8c9-4ab8-af08-05be72c37cbb",
    "David Álvarez",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Dragon"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever you cast a noncreature or Dragon spell, this \
             creature deals 1 damage to each opponent.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dragon")),
                ]),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(1)),
        ),
    ]),
);

// J25 19 — Scholar of Combustion
// Audit: unsupported — Needs an effect that exiles a targeted card and grants a timed permission to cast it. PermitCastFromGraveyardThisTurn leaves the card in the graveyard and lasts one turn; the exile-to-play effects read the top of a library rather than a target.
pub(in crate::card::sets) static SCHOLAR_OF_COMBUSTION: CardRecord = CardRecord::new(
    "Scholar of Combustion",
    "23660e44-8546-438d-a2c4-e1cef6e50855",
    "Nereida",
    crate::card::CardRules::unsupported(),
);

// J25 24 — Scythecat Cub
pub(in crate::card::sets) static SCYTHECAT_CUB: CardRecord = CardRecord::new(
    "Scythecat Cub",
    "b3dd3c7d-4685-4579-b483-14ddaaaddf5b",
    "Gabor Szikszai",
    // Two mana that turns a land drop into a counter and the second land of
    // the turn into all of them at once -- and trample, so what it grows
    // into does not stop at a blocker.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Cat"], 2, 2).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered_with_targets(
            "Landfall \u{2014} Whenever a land you control enters, put a +1/+1 counter on target \
             creature you control. If this is the second time this ability has resolved this \
             turn, double the number of +1/+1 counters on that creature instead.",
            // A land arriving under your control, which is what landfall watches: a
            // land put onto the battlefield by a search counts exactly as one played
            // from hand does.
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::IfElseCondition {
                // The count includes the resolution asking, so the second land of the turn
                // reads two. A third reads three and takes the other branch.
                condition: &TriggerConditionDef::SourceResolutionsThisTurn {
                    comparison: ComparisonDef::Equal,
                    amount: 2,
                },
                // "Double the number of +1/+1 counters on that creature": what it has, not
                // what this ability put there, so a creature somebody else grew doubles
                // just as readily.
                then: &EffectDef::DoubleCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                },
                otherwise: &EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            },
        ),
    ]),
);

// J25 28 — Shardless Outlander
pub(in crate::card::sets) static SHARDLESS_OUTLANDER: CardRecord = CardRecord::new(
    "Shardless Outlander",
    "fccb51a4-cb78-4437-b9ab-cc77736af561",
    "Leon Tukker",
    // Seven mana buys a 6/5 trampler almost nobody casts. The cycling half is
    // what earns the slot: a two-mana land fixer early, and a real threat in
    // the games that go long enough to want one.
    CardRules::new_artifact_creature(mana_cost!("{7}"), &["Construct", "Scout"], 6, 5)
        .with_abilities(&[
            abilities::trample(),
            abilities::typecycling!(
                "Basic landcycling {2} ({2}, Discard this card: Search your library for a basic \
                land card, reveal it, put it into your hand, then shuffle.)",
                &[CostDef::Mana(mana_cost!("{2}"))],
                // "Basic land card" is the conjunction, not the Basic supertype
                // alone: a basic Snow-Covered land qualifies and a legendary
                // land does not.
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Supertype(CardSupertype::Basic),
                    ObjectPredicateDef::HasType(CardType::Land),
                ]),
            ),
        ]),
);

// J25 37 — Plagon, Lord of the Beach
pub(in crate::card::sets) static PLAGON_LORD_OF_THE_BEACH: CardRecord = CardRecord::new(
    "Plagon, Lord of the Beach",
    "7f8a6bfe-6033-4f6b-ab45-6b553f8b51a1",
    "GOSSAN",
    // A 0/3 that pays for itself in a deck of walls and then turns them into
    // an offense: the numbers stay what they are, and only the combat
    // assignment reads the other one.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Starfish", "Wizard"], 0, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When Plagon enters, draw a card for each creature you control with toughness \
                 greater than its power.",
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    // "Each creature you control with toughness greater than its power": the
                    // comparison is between one creature's own two numbers, which is what makes
                    // a board of defensive bodies into a handful of cards.
                    amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::ToughnessGreaterThanItsPower,
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                },
            ),
            AbilityDef::activated_with_targets(
                "{W/U}: Target creature you control assigns combat damage equal to its toughness \
                 rather than its power this turn.",
                &[CostDef::Mana(mana_cost!("{W/U}"))],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Rule(
                        AppliedRuleDef::AssignsCombatDamageEqualToToughness,
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// J25 50 — Ivora, Insatiable Heir
pub(in crate::card::sets) static IVORA_INSATIABLE_HEIR: CardRecord = CardRecord::new(
    "Ivora, Insatiable Heir",
    "2ba70366-b6ae-423a-a8d8-29d2b8afd939",
    "Canata Katana",
CardRules::new_creature(mana_cost!("{1}{R}"), &["Vampire", "Warrior"], 1, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::trample(),
            AbilityDef::triggered(
                "When Ivora enters and whenever it deals combat damage to a player, create a Blood token.",
                // One printed ability with two ways in, which is what "when it enters and
                // whenever it deals combat damage" says. Splitting it would make her two
                // triggered abilities where the card has one.
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::Source,
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                    TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                ]),
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    BLOOD_TOKEN,
                ))),
            ),
            // Any discard, including one paid as a cost -- which is how her own Blood
            // token feeds her.
            AbilityDef::triggered(
                "Whenever you discard a card, put a +1/+1 counter on Ivora.",
                TriggerEventDef::Discarded(PlayerRelation::You),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// J25 114 — Dark Confidant (reprint)
const DARK_CONFIDANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rav::DARK_CONFIDANT,
    "c74e9388-460d-4dbf-934e-f3ecb48af6e8",
    "Victor Adame Minguez",
);

// J25 212 — Inspiring Overseer (reprint)
const INSPIRING_OVERSEER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2022::streets_of_new_capenna::INSPIRING_OVERSEER,
    "be1c0c41-cd92-49b2-be07-0c44219bcb6a",
    "Irina Nordsol",
);

// J25 343 — Pestermite (reprint)
const PESTERMITE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2007::lorwyn::PESTERMITE,
    "4c8b4f64-244c-4944-b23f-c383039d9767",
    "Christopher Moeller",
);

// J25 349 — Remand (reprint)
const REMAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2005::ravnica_city_of_guilds::REMAND,
    "36de9999-8d0a-4174-8e38-549bacdc128b",
    "Mark A. Nelson",
);

// J25 641 — Bushwhack (reprint)
const BUSHWHACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2022::the_brothers_war::BUSHWHACK,
    "f6b92766-1ab8-462d-bd45-ccd6f55cbe14",
    "Artur Nakhodkin",
);

// J25 684 — Llanowar Visionary (reprint)
const LLANOWAR_VISIONARY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2020::core_set_2021::LLANOWAR_VISIONARY,
    "c2635b0c-c990-4cce-9ac4-97602a757cf0",
    "Cristi Balanescu",
);

// J25 753 — Guardian Idol (reprint)
const GUARDIAN_IDOL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2004::fifth_dawn::GUARDIAN_IDOL,
    "1537f377-64c3-4c3b-a276-28d8234c029b",
    "Igor Kieryluk",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &DAWNWING_MARSHAL,
    &HINTERLAND_SANCTIFIER,
    &STARLIGHT_SNARE,
    &DROPKICK_BOMBER,
    &FIRESPITTER_WHELP,
    &SCHOLAR_OF_COMBUSTION,
    &SCYTHECAT_CUB,
    &SHARDLESS_OUTLANDER,
    &PLAGON_LORD_OF_THE_BEACH,
    &IVORA_INSATIABLE_HEIR,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    DARK_CONFIDANT_REPRINT,
    INSPIRING_OVERSEER_REPRINT,
    PESTERMITE_REPRINT,
    REMAND_REPRINT,
    BUSHWHACK_REPRINT,
    LLANOWAR_VISIONARY_REPRINT,
    GUARDIAN_IDOL_REPRINT,
];
