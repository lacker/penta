//! Theros cards cataloged as cross-format rules-engine test cases.

use crate::card::CardTypeSet;
use crate::card::CharacteristicOperationDef;
use crate::card::PlayerRefDef;
use crate::card::SetOperationDef;
use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::BasicLandType;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ColorChoiceOperationDef;
use crate::card::ComparisonDef;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectCounterValueDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::ids::TargetIndex;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "THS",
    slug: "theros",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// THS 16 — Gods Willing
pub(in crate::card::sets) static GODS_WILLING: CardRecord = CardRecord::new(
    "Gods Willing",
    "abafabb3-b2e7-4d78-b4b7-d8f701d3ee8b",
    "Mark Winters",
    // One mana that beats a removal spell and pushes damage through, and the
    // scry is what keeps it from being a dead card when neither is needed.
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature you control gains protection from the color of your choice until end of \
         turn. (It can't be blocked, targeted, dealt damage, enchanted, or equipped by anything \
         of that color.)\nScry 1.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::ChooseColor {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                operation: ColorChoiceOperationDef::ProtectionFromChosenColor,
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            // The scry happens even when the protection did nothing, so it
            // is sequenced after rather than made conditional.
            abilities::scry(ValueDef::Constant(1)),
        ]),
    )),
);

// THS 65 — Swan Song
pub(in crate::card::sets) static SWAN_SONG_65: CardRecord = CardRecord::new(
    "Swan Song",
    "efd26041-059b-4a1e-9ce8-c3cfd69a3721",
    "Peter Mohrbacher",
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[
AbilityDef::spell_with_targets("Counter target enchantment, instant, or sorcery spell. Its controller creates a 2/2 blue Bird creature token with flying.", &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::All(&[ObjectPredicateDef::Spell, ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Enchantment), ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Instant), ObjectPredicateDef::HasType(CardType::Sorcery)])])]), zones: &[ZoneKind::Stack], controller: None, owner: None })], EffectDef::Sequence(&[EffectDef::counter_target(TargetIndex::PRIMARY), EffectDef::CreateToken(crate::card::CreateTokenDef::new(crate::card::TokenDef::Literal(crate::card::TokenCharacteristics::creature(&["Bird"], &[ManaColor::Blue], 2, 2).with_abilities(&[abilities::flying()]))).with_controller(PlayerRefDef::ControllerOf(ObjectRefDef::Target(TargetIndex::PRIMARY))))]))
]),
);

// THS 89 — Gray Merchant of Asphodel
pub(in crate::card::sets) static GRAY_MERCHANT_OF_ASPHODEL: CardRecord = CardRecord::new(
    "Gray Merchant of Asphodel",
    "b06078ce-f534-4e16-9a70-d51620a33eb2",
    "Robbie Trevino",
// Its own two black pips count, so the Merchant is never worth less than
    // two even on an otherwise empty board.
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Zombie"], 2, 4).with_ability(
        abilities::enters_trigger(
            "When this creature enters, each opponent loses X life, where X is your devotion to black. You gain life equal to the life lost this way.",
            // Devotion is counted once for the whole resolution, so both
            // halves read the same number and the gain always matches the
            // loss.
            EffectDef::Sequence(&[
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::players(PlayerSetDef::Related(
                        PlayerRelation::Opponent,
                    )),
                    amount: ValueDef::DevotionTo(ManaColor::Black),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::DevotionTo(ManaColor::Black),
                },
            ]),
        ),
    ),
);

// THS 90 — Hero's Downfall
pub(in crate::card::sets) static HERO_S_DOWNFALL: CardRecord = CardRecord::new(
    "Hero's Downfall",
    "596822f6-dbd4-4cc8-aa50-9331ff42544e",
    "Ryan Pancoast",
    CardRules::new_instant(mana_cost!("{1}{B}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Destroy target creature or planeswalker.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                ]),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ]),
);

// THS 119 — Dragon Mantle
pub(in crate::card::sets) static DRAGON_MANTLE_119: CardRecord = CardRecord::new(
    "Dragon Mantle",
    "d97b1080-9001-4751-b2f5-7f56d9f58dff",
    "Anthony Palumbo",
    CardRules::new_enchantment(mana_cost!("{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            abilities::enters_trigger(
                "When this Aura enters, draw a card.",
                abilities::draw_cards(ValueDef::Constant(1)),
            ),
            AbilityDef::static_ability(
                "Enchanted creature has \"{R}: This creature gets +1/+0 until end of turn.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::object(ObjectRefDef::AttachedToSource),
                    effect: AppliedEffectDef::add_ability(&AbilityDef::activated(
                        "{R}: This creature gets +1/+0 until end of turn.",
                        &[CostDef::Mana(mana_cost!("{R}"))],
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::Source,
                            effect: AppliedEffectDef::modify_power_toughness(
                                ValueDef::Constant(1),
                                ValueDef::Constant(0),
                            ),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    )),
                },
            ),
        ]),
);

// THS 127 — Lightning Strike
pub(in crate::card::sets) static LIGHTNING_STRIKE: CardRecord = CardRecord::new(
    "Lightning Strike",
    "bbb03f2e-2b92-4aa1-afae-301ed5d151d3",
    "Adam Paquette",
    // Lightning Bolt at two mana, which is the rate every later red burn
    // spell is measured against.
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Lightning Strike deals 3 damage to any target.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::damage(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ValueDef::Constant(3),
        ),
    )),
);

// THS 135 — Purphoros, God of the Forge
pub(in crate::card::sets) static PURPHOROS_GOD_OF_THE_FORGE_135: CardRecord = CardRecord::new(
    "Purphoros, God of the Forge",
    "7bf6baf2-d20b-467d-8929-abefcf7dfa99",
    "Eric Deschamps",
    CardRules::new_enchantment_creature(mana_cost!("{3}{R}"), &["God"], 6, 5).with_supertype(CardSupertype::Legendary).with_abilities(&[
abilities::indestructible(),
AbilityDef::static_ability("As long as your devotion to red is less than five, Purphoros isn't a creature.", EffectDef::IfCondition { condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef { left: ValueDef::DevotionTo(ManaColor::Red), comparison: ComparisonDef::Less, right: ValueDef::Constant(5) }), then: &EffectDef::StaticApply { recipient: EffectRecipientDef::Source, effect: AppliedEffectDef::Characteristic(CharacteristicOperationDef::CardTypes(SetOperationDef::Remove(CardTypeSet::single(CardType::Creature)))) } }),
AbilityDef::triggered("Whenever another creature you control enters, Purphoros deals 2 damage to each opponent.", TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::ControlledBy(PlayerRelation::You), ObjectPredicateDef::Not(&ObjectPredicateDef::Source)]), None, Some(ZoneKind::Battlefield)), EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(2))),
AbilityDef::activated("{2}{R}: Creatures you control get +1/+0 until end of turn.", &[CostDef::Mana(mana_cost!("{2}{R}"))], EffectDef::Apply { recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::You), effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(0)), duration: ResolvedEffectDurationDef::UntilEndOfTurn })
]),
);

// THS 169 — Nylea's Presence
pub(in crate::card::sets) static NYLEAS_PRESENCE: CardRecord = CardRecord::new(
    "Nylea's Presence",
    "e68f1fd4-1a2f-405b-a592-6c4af6214eae",
    "Ralph Horsley",
    CardRules::new_enchantment(mana_cost!("{1}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant land",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Land),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            abilities::enters_trigger(
                "When Nylea's Presence enters, draw a card.",
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted land is every basic land type in addition to its other types.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_basic_land_types(&BasicLandType::ALL),
                },
            ),
        ]),
);

// THS 170 — Ordeal of Nylea
pub(in crate::card::sets) static ORDEAL_OF_NYLEA: CardRecord = CardRecord::new(
    "Ordeal of Nylea",
    "e5c48950-c246-47ad-94e1-bf42a62c2fe7",
    "David Palumbo",
    CardRules::new_enchantment(mana_cost!("{1}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::triggered(
                "Whenever enchanted creature attacks, put a +1/+1 counter on \
                 it. Then if it has three or more +1/+1 counters on it, \
                 sacrifice this Aura.",
                TriggerEventDef::attacks(ObjectPredicateDef::AttachedToSource),
                EffectDef::Sequence(&[
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::AttachedPermanent,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                            left: ValueDef::CountersOnObject(&ObjectCounterValueDef {
                                object: ObjectRefDef::AttachedToSource,
                                kind: CounterKind::PlusOnePlusOne,
                            }),
                            comparison: ComparisonDef::GreaterOrEqual,
                            right: ValueDef::Constant(3),
                        }),
                        then: &EffectDef::sacrifice(EffectRecipientDef::Source),
                    },
                ]),
            ),
            AbilityDef::triggered(
                "When you sacrifice this Aura, search your library for up to \
                 two basic land cards, put them onto the battlefield tapped, \
                 then shuffle.",
                TriggerEventDef::Sacrificed {
                    object: ObjectPredicateDef::Source,
                    player: PlayerRelation::You,
                },
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::Constant(2),
                    reveal: true,
                    destination: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: true,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            ),
        ]),
);

// THS 180 — Sylvan Caryatid
pub(in crate::card::sets) static SYLVAN_CARYATID: CardRecord = CardRecord::new(
    "Sylvan Caryatid",
    "d40b65c1-b24d-492d-81b9-d8474ebdc08c",
    "Chase Stone",
    // Hexproof is what separates it from every other two-mana accelerant: the
    // removal that answers a mana creature cannot be pointed at this one, and
    // a 0/3 wall survives most of what is left.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Plant"], 0, 3).with_abilities(&[
        abilities::defender(),
        abilities::hexproof(),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// THS 213 — Burnished Hart
pub(in crate::card::sets) static BURNISHED_HART: CardRecord = CardRecord::new(
    "Burnished Hart",
    "772cbcba-9efa-4894-9b57-e73fd296333d",
    "Yeong-Hao Han",
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Elk"], 2, 2).with_abilities(&[
        AbilityDef::activated(
            "{3}, Sacrifice this creature: Search your library for up to \
             two basic land cards, put them onto the battlefield tapped, \
             then shuffle.",
            &[CostDef::Mana(mana_cost!("{3}")), CostDef::SacrificeSource],
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Supertype(CardSupertype::Basic),
                ]),
                minimum: 0,
                maximum: ValueDef::Constant(2),
                reveal: true,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: true,
                attachment: None,
                binding: None,
                then: None,
            },
        ),
    ]),
);

// THS 220 — Pyxis of Pandemonium
// Audit: unsupported — Revealing exiled cards emits an information event but does not turn face-down exile objects face up. Nonpermanent cards must remain face up in exile after this activation, which has no shared operation.
pub(in crate::card::sets) static PYXIS_OF_PANDEMONIUM_220: CardRecord = CardRecord::new(
    "Pyxis of Pandemonium",
    "dbc6a246-f32a-4dc0-9785-4038804f372f",
    "David Palumbo",
    crate::card::CardRules::unsupported(),
);

// THS 223 — Nykthos, Shrine to Nyx
// Audit: unsupported — Mana production can read devotion to a fixed color, but cannot bind the chosen output color and evaluate devotion to that same choice.
pub(in crate::card::sets) static NYKTHOS_SHRINE_TO_NYX_223: CardRecord = CardRecord::new(
    "Nykthos, Shrine to Nyx",
    "834b27a0-dfd7-4f96-8cde-cacac4b24acc",
    "Jung Park",
    crate::card::CardRules::unsupported(),
);

// THS 224 — Temple of Abandon
pub(in crate::card::sets) static TEMPLE_OF_ABANDON: CardRecord = CardRecord::new(
    "Temple of Abandon",
    "46febc5d-1625-4e48-bb3f-31ee06fc13dd",
    "Mike Bierek",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, scry 1. (Look at the top card of your \
             library. You may put that card on the bottom.)",
            abilities::scry(ValueDef::Constant(1)),
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

// THS 225 — Temple of Deceit
pub(in crate::card::sets) static TEMPLE_OF_DECEIT: CardRecord = CardRecord::new(
    "Temple of Deceit",
    "686559d7-8ac1-496b-a5a6-1467bf8fc7c5",
    "Raymond Swanland",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, scry 1. (Look at the top card of your \
             library. You may put that card on the bottom.)",
            abilities::scry(ValueDef::Constant(1)),
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

// THS 226 — Temple of Mystery
pub(in crate::card::sets) static TEMPLE_OF_MYSTERY: CardRecord = CardRecord::new(
    "Temple of Mystery",
    "2f66945b-3e64-498a-9478-5f96a61d4ec7",
    "Noah Bradley",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, scry 1. (Look at the top card of your \
             library. You may put that card on the bottom.)",
            abilities::scry(ValueDef::Constant(1)),
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

// THS 227 — Temple of Silence
pub(in crate::card::sets) static TEMPLE_OF_SILENCE: CardRecord = CardRecord::new(
    "Temple of Silence",
    "0f14b6b3-5f40-4328-a3be-28fe32dd7cb1",
    "Karl Kopinski",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, scry 1. (Look at the top card of your \
             library. You may put that card on the bottom.)",
            abilities::scry(ValueDef::Constant(1)),
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

// THS 228 — Temple of Triumph
pub(in crate::card::sets) static TEMPLE_OF_TRIUMPH: CardRecord = CardRecord::new(
    "Temple of Triumph",
    "4f53049c-2491-4d20-aa19-00eb5c55b438",
    "Jason Felix",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, scry 1. (Look at the top card of your \
             library. You may put that card on the bottom.)",
            abilities::scry(ValueDef::Constant(1)),
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
    &GODS_WILLING,
    &SWAN_SONG_65,
    &GRAY_MERCHANT_OF_ASPHODEL,
    &HERO_S_DOWNFALL,
    &DRAGON_MANTLE_119,
    &LIGHTNING_STRIKE,
    &PURPHOROS_GOD_OF_THE_FORGE_135,
    &NYLEAS_PRESENCE,
    &ORDEAL_OF_NYLEA,
    &SYLVAN_CARYATID,
    &BURNISHED_HART,
    &PYXIS_OF_PANDEMONIUM_220,
    &NYKTHOS_SHRINE_TO_NYX_223,
    &TEMPLE_OF_ABANDON,
    &TEMPLE_OF_DECEIT,
    &TEMPLE_OF_MYSTERY,
    &TEMPLE_OF_SILENCE,
    &TEMPLE_OF_TRIUMPH,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
