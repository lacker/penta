//! Khans of Tarkir cards cataloged as cross-format rules-engine test cases.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ComparisonDef;
use crate::card::CopyExceptionsDef;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::DiscardSelectionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::PlayerRelation;
use crate::card::ReplacementEffectDef;
use crate::card::ReplacementEventDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnKindDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZoneMoveCauseDef;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "KTK",
    slug: "khans-of-tarkir",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// KTK 3 — Ainok Bond-Kin
pub(in crate::card::sets) static AINOK_BOND_KIN: CardRecord = CardRecord::new(
    "Ainok Bond-Kin",
    "22d2a844-17fc-4628-9591-684555e98f7b",
    "Chris Rahn",
    // Outlast is slow enough that the anthem is the reason to play it: a
    // counters deck gets first strike on the whole board for free.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Dog", "Soldier"], 2, 1).with_abilities(&[
        AbilityDef::activated(
            "Outlast {1}{W} ({1}{W}, {T}: Put a +1/+1 counter on this creature. Outlast only as \
             a sorcery.)",
            &[CostDef::Mana(mana_cost!("{1}{W}")), CostDef::TapSource],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
        AbilityDef::static_ability(
            "Each creature you control with a +1/+1 counter on it has first strike.",
            EffectDef::StaticApply {
                // Itself included once it has outlasted, which is what makes
                // the slow ability worth activating at all.
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasCounter(CounterKind::PlusOnePlusOne),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
            },
        ),
    ]),
);

// KTK 22 — Seeker of the Way
pub(in crate::card::sets) static SEEKER_OF_THE_WAY: CardRecord = CardRecord::new(
    "Seeker of the Way",
    "3c17e350-44f7-4413-ad24-7c5d6616effd",
    "Craig J Spearing",
    // Prowess and lifelink on the same trigger is what turns one cheap spell
    // into a four-point life swing, which is why this ends races.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Warrior"], 2, 2).with_abilities(&[
        abilities::prowess(),
        // A second printed ability watching the same event, not a rider on
        // prowess: two spells in a turn grant lifelink twice, harmlessly,
        // and each grows the body separately.
        AbilityDef::triggered(
            "Whenever you cast a noncreature spell, this creature gains lifelink until end of \
             turn.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::NoncreatureSpell,
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::lifelink()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// KTK 34 — Clever Impersonator
pub(in crate::card::sets) static CLEVER_IMPERSONATOR_34: CardRecord = CardRecord::new(
    "Clever Impersonator",
    "cd8fffd3-81ad-47e3-a27b-d8059f2b506f",
    "Slawomir Maniak",
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Shapeshifter"], 0, 0).with_abilities(&[
AbilityDef::replacement("You may have this creature enter as a copy of any nonland permanent on the battlefield.", ReplacementEffectDef::CopyEntering { object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)), exceptions: CopyExceptionsDef::NONE })
]),
);

// KTK 37 — Disdainful Stroke
pub(in crate::card::sets) static DISDAINFUL_STROKE: CardRecord = CardRecord::new(
    "Disdainful Stroke",
    "180425c9-1898-48d4-9932-ddfb1a28e6b0",
    "Svetlin Velinov",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Counter target spell with mana value 4 or greater.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Spell,
                    ObjectPredicateDef::Not(&ObjectPredicateDef::ManaValueAtMost(3)),
                ]),
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::counter_target(TargetIndex::PRIMARY),
    )]),
);

// KTK 56 — Stubborn Denial
pub(in crate::card::sets) static STUBBORN_DENIAL_56: CardRecord = CardRecord::new(
    "Stubborn Denial",
    "6f8626c4-306f-4e9d-8840-2bb73fe87e87",
    "James Ryman",
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[
AbilityDef::spell_with_targets("Counter target noncreature spell unless its controller pays {1}.\nFerocious — If you control a creature with power 4 or greater, counter that spell instead.", &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::NoncreatureSpell, zones: &[ZoneKind::Stack], controller: None, owner: None })], EffectDef::IfElseCondition { condition: &TriggerConditionDef::ObjectCount { query: ObjectQueryDef::matching(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::PowerAtLeast(4)]), &[ZoneKind::Battlefield], PlayerRelation::You), comparison: ComparisonDef::GreaterOrEqual, amount: 1 }, then: &EffectDef::counter_target(TargetIndex::PRIMARY), otherwise: &abilities::counter_target_unless_paid(&[CostDef::Mana(mana_cost!("{1}"))]) })
]),
);

// KTK 59 — Treasure Cruise
pub(in crate::card::sets) static TREASURE_CRUISE: CardRecord = CardRecord::new(
    "Treasure Cruise",
    "7a59d4b1-6cf4-44ec-8a96-1bb7094fea21",
    "Cynthia Sheppard",
    CardRules::new_sorcery(mana_cost!("{7}{U}")).with_abilities(&[
        abilities::delve(),
        AbilityDef::spell(
            "Draw three cards.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ),
    ]),
);

// KTK 78 — Mardu Skullhunter
pub(in crate::card::sets) static MARDU_SKULLHUNTER: CardRecord = CardRecord::new(
    "Mardu Skullhunter",
    "dd3ca5e7-96f3-4326-9315-34bb396a054c",
    "Jason Rainville",
    // The discard is the whole card; entering tapped is what it costs, since a
    // 2/1 that cannot block the turn it lands trades a tempo point for the
    // hand it stripped.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Warrior"], 2, 1)
        .with_ability(abilities::enters_tapped(CardType::Creature))
        .with_ability(abilities::enters_trigger_with_targets(
            "When this creature enters, target opponent discards a card.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::Discard {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        )),
);

// KTK 111 — Hordeling Outburst
pub(in crate::card::sets) static HORDELING_OUTBURST: CardRecord = CardRecord::new(
    "Hordeling Outburst",
    "a5c1bf52-2737-423a-b340-07448afcaea6",
    "Zoltan Boros",
    // Three bodies from one card is what a go-wide deck is buying; the
    // sorcery speed is the price for not paying one mana each.
    CardRules::new_sorcery(mana_cost!("{1}{R}{R}")).with_ability(AbilityDef::spell(
        "Create three 1/1 red Goblin creature tokens.",
        EffectDef::CreateToken(
            CreateTokenDef::new(TokenDef::Literal(TokenCharacteristics::creature(
                &["Goblin"],
                &[ManaColor::Red],
                1,
                1,
            )))
            .with_amount(3),
        ),
    )),
);

// KTK 118 — Monastery Swiftspear
pub(in crate::card::sets) static MONASTERY_SWIFTSPEAR: CardRecord = CardRecord::new(
    "Monastery Swiftspear",
    "b81c6c8b-a9cf-4866-89ba-7f8ad077b836",
    "Steve Argyle",
    // Haste is what makes prowess pay on the turn it lands rather than the
    // turn after, which is the whole card.
    CardRules::new_creature(mana_cost!("{R}"), &["Human", "Monk"], 1, 2)
        .with_abilities(&[abilities::haste(), abilities::prowess()]),
);

// KTK 123 — Tormenting Voice
pub(in crate::card::sets) static TORMENTING_VOICE_123: CardRecord = CardRecord::new(
    "Tormenting Voice",
    "25af9ac1-a03b-4be7-b726-fb66427b1caa",
    "Volkan Baǵa",
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::spell(
        "Draw two cards.",
        abilities::draw_cards(ValueDef::Constant(2)),
    )
    .with_spell_additional_cost(&CostDef::discard(ObjectPredicateDef::Any))]),
);

// KTK 133 — Hardened Scales
// Audit: unsupported — ReplacementEventDef has no counter-placement event to modify the amount of +1/+1 counters put on other controlled creatures.
pub(in crate::card::sets) static HARDENED_SCALES_133: CardRecord = CardRecord::new(
    "Hardened Scales",
    "7dcdf1db-bfaf-4160-8003-1fa2e56b00dc",
    "Mark Winters",
    crate::card::CardRules::unsupported(),
);

// KTK 137 — Hooting Mandrills
pub(in crate::card::sets) static HOOTING_MANDRILLS: CardRecord = CardRecord::new(
    "Hooting Mandrills",
    "090d678c-f0e4-4757-8900-93dfe67aefe9",
    "Mike Bierek",
    // Trample is what separates this from the other delve fatties: a
    // graveyard deck casts it early, when nothing on the far side blocks it
    // profitably anyway.
    CardRules::new_creature(mana_cost!("{5}{G}"), &["Ape"], 4, 4)
        .with_abilities(&[abilities::delve(), abilities::trample()]),
);

// KTK 207 — Temur Ascendancy
pub(in crate::card::sets) static TEMUR_ASCENDANCY_207: CardRecord = CardRecord::new(
    "Temur Ascendancy",
    "11746bf1-d813-4ade-8ce4-9935cebef856",
    "Jaime Jones",
    CardRules::new_enchantment(mana_cost!("{G}{U}{R}")).with_abilities(&[
        AbilityDef::static_ability(
            "Creatures you control have haste.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::haste()),
            },
        ),
        AbilityDef::triggered(
            "Whenever a creature you control with power 4 or greater enters, you may draw a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::PowerAtLeast(4),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &abilities::draw_cards(ValueDef::Constant(1)),
            },
        ),
    ]),
);

// KTK 216 — Altar of the Brood
pub(in crate::card::sets) static ALTAR_OF_THE_BROOD_216: CardRecord = CardRecord::new(
    "Altar of the Brood",
    "8d59d264-87ee-4305-bffb-110549331a82",
    "Erica Yang",
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[AbilityDef::triggered(
        "Whenever another permanent you control enters, each opponent mills a card.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ]),
            None,
            Some(ZoneKind::Battlefield),
        ),
        EffectDef::Mill {
            player: EffectRecipientDef::Opponent,
            amount: ValueDef::Constant(1),
        },
    )]),
);

// KTK 217 — Briber's Purse
pub(in crate::card::sets) static BRIBER_S_PURSE_217: CardRecord = CardRecord::new(
    "Briber's Purse",
    "7f9951f1-ca51-44a2-8480-602df466f0ab",
    "Steve Argyle",
    CardRules::new_artifact(mana_cost!("{X}")).with_abilities(&[
AbilityDef::as_enters("This artifact enters with X gem counters on it.", ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::AddCastXCounters { kind: CounterKind::named("gem") })),
AbilityDef::activated_with_targets("{1}, {T}, Remove a gem counter from this artifact: Target creature can't attack or block this turn.", &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource, CostDef::RemoveCountersFromSource { kind: CounterKind::named("gem"), amount: 1 }], &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Creature))], EffectDef::Apply { recipient: EffectRecipientDef::target_objects(TargetIndex::PRIMARY), effect: AppliedEffectDef::Composite(&[AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK), AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK)]), duration: ResolvedEffectDurationDef::UntilEndOfTurn })
]),
);

// KTK 227 — Ugin's Nexus
pub(in crate::card::sets) static UGINS_NEXUS: CardRecord = CardRecord::new(
    "Ugin's Nexus",
    "94002868-a48a-4ea8-bfce-17257078f5db",
    "Sam Burley",
CardRules::new_artifact(mana_cost!("{5}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::replacement_for(
                "If a player would begin an extra turn, that player skips that turn instead.",
                ReplacementEventDef::WouldBeginTurn {
                    player: PlayerRelation::Any,
                    kind: TurnKindDef::Extra,
                },
                ReplacementEffectDef::ReplaceEventWithNothing,
            ),
            AbilityDef::replacement_for(
                "If Ugin's Nexus would be put into a graveyard from the battlefield, instead exile it and take an extra turn after this one.",
                ReplacementEventDef::WouldMove {
                    from: Some(ZoneKind::Battlefield),
                    to: ZoneKind::Graveyard,
                    cause: ZoneMoveCauseDef::Any,
                },
                ReplacementEffectDef::Sequence(&[
                    ReplacementEffectDef::MoveToZone(ZoneKind::Exile),
                    ReplacementEffectDef::Perform(&EffectDef::TakeExtraTurn {
                        player: EffectRecipientDef::Controller,
                    }),
                ]),
            ),
        ]),
);

// KTK 229 — Bloodfell Caves
pub(in crate::card::sets) static BLOODFELL_CAVES: CardRecord = CardRecord::new(
    "Bloodfell Caves",
    "15a7b30a-c59f-4a87-9e8a-b29daea27422",
    "Adam Paquette",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, you gain 1 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {B} or {R}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Red,
            ])),
        ),
    ]),
);

// KTK 231 — Blossoming Sands
pub(in crate::card::sets) static BLOSSOMING_SANDS: CardRecord = CardRecord::new(
    "Blossoming Sands",
    "a32a1c0b-f6ea-475a-aa01-3618ea7d8647",
    "Sam Burley",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, you gain 1 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {G} or {W}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Green,
                ManaColor::White,
            ])),
        ),
    ]),
);

// KTK 232 — Dismal Backwater
pub(in crate::card::sets) static DISMAL_BACKWATER: CardRecord = CardRecord::new(
    "Dismal Backwater",
    "63742780-47ee-4a66-993a-69e06c14967d",
    "Sam Burley",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, you gain 1 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {U} or {B}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Blue,
                ManaColor::Black,
            ])),
        ),
    ]),
);

// KTK 234 — Frontier Bivouac
pub(in crate::card::sets) static FRONTIER_BIVOUAC: CardRecord = CardRecord::new(
    "Frontier Bivouac",
    "e4335951-e73e-45cb-b2a5-6e9d14ba87ee",
    "Titus Lunter",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {G}, {U}, or {R}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Green,
                ManaColor::Blue,
                ManaColor::Red,
            ])),
        ),
    ]),
);

// KTK 235 — Jungle Hollow
pub(in crate::card::sets) static JUNGLE_HOLLOW: CardRecord = CardRecord::new(
    "Jungle Hollow",
    "fea27aa7-7fcf-4198-b03a-5034a03ba81f",
    "Eytan Zana",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, you gain 1 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {B} or {G}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Green,
            ])),
        ),
    ]),
);

// KTK 236 — Mystic Monastery
pub(in crate::card::sets) static MYSTIC_MONASTERY: CardRecord = CardRecord::new(
    "Mystic Monastery",
    "bae51d77-e06b-4e5a-9543-a17dd0b2a333",
    "Florian de Gesincourt",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {U}, {R}, or {W}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Blue,
                ManaColor::Red,
                ManaColor::White,
            ])),
        ),
    ]),
);

// KTK 237 — Nomad Outpost
pub(in crate::card::sets) static NOMAD_OUTPOST: CardRecord = CardRecord::new(
    "Nomad Outpost",
    "fb6ae4a5-227d-465b-9e99-bae158b7d410",
    "Noah Bradley",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {R}, {W}, or {B}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Red,
                ManaColor::White,
                ManaColor::Black,
            ])),
        ),
    ]),
);

// KTK 238 — Opulent Palace
pub(in crate::card::sets) static OPULENT_PALACE: CardRecord = CardRecord::new(
    "Opulent Palace",
    "21326575-80b9-4a4e-a93c-6880ec6575d5",
    "Adam Paquette",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {B}, {G}, or {U}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Green,
                ManaColor::Blue,
            ])),
        ),
    ]),
);

// KTK 240 — Rugged Highlands
pub(in crate::card::sets) static RUGGED_HIGHLANDS: CardRecord = CardRecord::new(
    "Rugged Highlands",
    "501ce6cb-0324-4cca-bc79-903cefe1ac1f",
    "Eytan Zana",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, you gain 1 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {R} or {G}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Red,
                ManaColor::Green,
            ])),
        ),
    ]),
);

// KTK 241 — Sandsteppe Citadel
pub(in crate::card::sets) static SANDSTEPPE_CITADEL: CardRecord = CardRecord::new(
    "Sandsteppe Citadel",
    "2dd40d90-c939-458a-9a98-27d10da6ff2f",
    "Sam Burley",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {W}, {B}, or {G}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Black,
                ManaColor::Green,
            ])),
        ),
    ]),
);

// KTK 242 — Scoured Barrens
pub(in crate::card::sets) static SCOURED_BARRENS: CardRecord = CardRecord::new(
    "Scoured Barrens",
    "0824a960-dd89-45c5-90f0-3ec9eb47d9ce",
    "Eytan Zana",
    // A tapped dual with a life attached: the life is what a limited deck
    // is paid for the turn it loses.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, you gain 1 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {W} or {B}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Black,
            ])),
        ),
    ]),
);

// KTK 243 — Swiftwater Cliffs
pub(in crate::card::sets) static SWIFTWATER_CLIFFS: CardRecord = CardRecord::new(
    "Swiftwater Cliffs",
    "e782d005-a563-4738-978a-73a3465de78f",
    "Eytan Zana",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, you gain 1 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {U} or {R}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[ManaColor::Blue, ManaColor::Red])),
        ),
    ]),
);

// KTK 244 — Thornwood Falls
pub(in crate::card::sets) static THORNWOOD_FALLS: CardRecord = CardRecord::new(
    "Thornwood Falls",
    "9e57abd9-e864-4047-a3c8-618952071858",
    "Eytan Zana",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, you gain 1 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {G} or {U}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Green,
                ManaColor::Blue,
            ])),
        ),
    ]),
);

// KTK 246 — Tranquil Cove
pub(in crate::card::sets) static TRANQUIL_COVE: CardRecord = CardRecord::new(
    "Tranquil Cove",
    "0f840bd2-c4f5-4ac4-918c-91b4feeb8783",
    "John Avon",
    // A gain land: the tempo is the whole cost, and the life is what makes
    // the tapped land bearable in a slow deck.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, you gain 1 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {W} or {U}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Blue,
            ])),
        ),
    ]),
);

// KTK 247 — Wind-Scarred Crag
pub(in crate::card::sets) static WIND_SCARRED_CRAG: CardRecord = CardRecord::new(
    "Wind-Scarred Crag",
    "3b296781-78ac-411f-88fc-2d924ad22986",
    "Eytan Zana",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, you gain 1 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {R} or {W}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Red,
                ManaColor::White,
            ])),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AINOK_BOND_KIN,
    &SEEKER_OF_THE_WAY,
    &CLEVER_IMPERSONATOR_34,
    &DISDAINFUL_STROKE,
    &STUBBORN_DENIAL_56,
    &TREASURE_CRUISE,
    &MARDU_SKULLHUNTER,
    &HORDELING_OUTBURST,
    &MONASTERY_SWIFTSPEAR,
    &TORMENTING_VOICE_123,
    &HARDENED_SCALES_133,
    &HOOTING_MANDRILLS,
    &TEMUR_ASCENDANCY_207,
    &ALTAR_OF_THE_BROOD_216,
    &BRIBER_S_PURSE_217,
    &UGINS_NEXUS,
    &BLOODFELL_CAVES,
    &BLOSSOMING_SANDS,
    &DISMAL_BACKWATER,
    &FRONTIER_BIVOUAC,
    &JUNGLE_HOLLOW,
    &MYSTIC_MONASTERY,
    &NOMAD_OUTPOST,
    &OPULENT_PALACE,
    &RUGGED_HIGHLANDS,
    &SANDSTEPPE_CITADEL,
    &SCOURED_BARRENS,
    &SWIFTWATER_CLIFFS,
    &THORNWOOD_FALLS,
    &TRANQUIL_COVE,
    &WIND_SCARRED_CRAG,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
