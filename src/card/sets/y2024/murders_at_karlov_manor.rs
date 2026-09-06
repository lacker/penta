//! Murders at Karlov Manor cards cataloged for the Vintage Cube.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, ActivationTimingDef, AppliedEffectDef,
    AppliedRuleDef, BasicLandType, CardArt, CardRules, CardSet, CardSupertype, CardType, ColorSet,
    ComparisonDef, CostDef, CostModificationDef, CounterKind, EffectDef, EffectRecipientDef,
    ManaColor, ObjectPredicateDef, PlayerRelation, PlayerSetDef, ResolvedEffectDurationDef,
    SumValueDef, TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueComparisonDef, ValueDef,
    ZoneKind, ZonePlacement, abilities, tokens,
};
use crate::{TargetIndex, mana_cost};

static SURVEIL_LAND_ABILITIES: [AbilityDef; 2] = [
    abilities::enters_tapped(CardType::Land),
    abilities::enters_trigger(
        "When this land enters, surveil 1. (Look at the top card of your library. You may put it \
         into your graveyard.)",
        abilities::surveil(ValueDef::Constant(1)),
    ),
];

/// The surveil-land cycle: two basic types, tapped on the way in, and one
/// look at the top of your library to pay for it. The mana abilities come
/// from the types rather than from a printed clause.
const fn surveil_land(types: &'static [&'static str]) -> CardRules {
    CardRules::new_land(types).with_abilities(&SURVEIL_LAND_ABILITIES)
}

// MKM 29 — Novice Inspector
pub(in crate::card::sets) static NOVICE_INSPECTOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0ad38866-fc5f-4f62-89c1-afc0f50765aa"),
    "Novice Inspector",
    CardArt::new("0ad38866-fc5f-4f62-89c1-afc0f50765aa", "Fajareka Setiawan"),
    CardSet::MurdersAtKarlovManor,
    // One mana for a blocker and half a card, which is the floor a white
    // one-drop has to clear to be playable at all.
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Detective"], 1, 2).with_ability(
        abilities::enters_trigger(
            "When this creature enters, investigate. (Create a Clue token. It's an artifact with \"{2}, Sacrifice this token: Draw a card.\")",
            EffectDef::create_token(tokens::clue()).with_art(CardArt::new(
                "ef607895-d6d2-44ab-a6b4-84af55fce593",
                "Daneen Wilkerson",
            )),
        ),
    ),
);

// MKM 57 — Forensic Gadgeteer
pub(in crate::card::sets) static FORENSIC_GADGETEER: CardRecord = CardRecord::new_with_legacy_id(
    2206,
    "Forensic Gadgeteer",
    CardArt::new("97d08a15-e61c-4421-a541-c68a4f87cb74", "Volkan Baǵa"),
    CardSet::MurdersAtKarlovManor,
    // Every artifact you cast is a card later, and every artifact you
    // already have is cheaper to use -- including the Clues it just made.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Vedalken", "Artificer", "Detective"], 2, 3)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever you cast an artifact spell, investigate. (Create a Clue token. It's an artifact with \"{2}, Sacrifice this token: Draw a card.\")",
                // An artifact spell you cast, which is the whole of the trigger: what it
                // does is not part of the condition.
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                EffectDef::create_token(tokens::clue()).with_art(CardArt::new(
                    "ef607895-d6d2-44ab-a6b4-84af55fce593",
                    "Daneen Wilkerson",
                )),
            ),
            AbilityDef::static_ability(
                "Activated abilities of artifacts you control cost {1} less to activate. This effect can't reduce the mana in that cost to less than one mana.",
                EffectDef::ModifyCost(CostModificationDef::AbilityReduction {
                    permanent: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    amount: ValueDef::Constant(1),
                    minimum: 1,
                }),
            ),
        ]),
);

// MKM 105 — Snarling Gorehound
pub(in crate::card::sets) static SNARLING_GOREHOUND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("93ab3e11-8584-406f-b9ae-9e1df4396cbc"),
    "Snarling Gorehound",
    CardArt::new("93ab3e11-8584-406f-b9ae-9e1df4396cbc", "John Tedrick"),
    CardSet::MurdersAtKarlovManor,
    // A one-drop that keeps paying in a deck full of other one-drops, which
    // is exactly the deck that wants a menace body this cheap.
    CardRules::new_creature(mana_cost!("{B}"), &["Dog"], 1, 1).with_abilities(&[
        abilities::menace(),
        AbilityDef::triggered(
            "Whenever another creature you control with power 2 or less enters, surveil 1. (Look \
             at the top card of your library. You may put it into your graveyard.)",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    // "Power 2 or less" has to be a strict comparison because
                    // power only reads upward here.
                    ObjectPredicateDef::PowerLessThan(ValueDef::Constant(3)),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            abilities::surveil(ValueDef::Constant(1)),
        ),
    ]),
);

// MKM 174 — Rubblebelt Maverick
pub(in crate::card::sets) static RUBBLEBELT_MAVERICK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("81c7ff67-b9e1-4d2e-b1ae-da9b946da00b"),
    "Rubblebelt Maverick",
    CardArt::new("81c7ff67-b9e1-4d2e-b1ae-da9b946da00b", "Carissa Susilo"),
    CardSet::MurdersAtKarlovManor,
    // A one-drop that fills the graveyard on the way in and cashes itself
    // out of it later, so trading it away costs the deck almost nothing.
    CardRules::new_creature(mana_cost!("{G}"), &["Human", "Detective"], 1, 1).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, surveil 2. (Look at the top two cards of your library, \
             then put any number of them into your graveyard and the rest on top of your library \
             in any order.)",
            abilities::surveil(ValueDef::Constant(2)),
        ),
        AbilityDef::activated_with_targets(
            "{G}, Exile this card from your graveyard: Put a +1/+1 counter on target creature. \
             Activate only as a sorcery.",
            &[CostDef::Mana(mana_cost!("{G}")), CostDef::ExileSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        )
        // Activated from the graveyard rather than the battlefield, and at
        // sorcery speed.
        .with_source_zones(&[ZoneKind::Graveyard])
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// MKM 197 — Dog Walker
// Audit: unsupported — Needs a turned-face-up trigger. Disguise itself has a cast kind and face-down characteristics, but no event fires when a permanent is turned face up, which is when this card does everything it does.
pub(in crate::card::sets) static DOG_WALKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a6e0adb7-a030-4dcc-9284-cd91c7598a22"),
    "Dog Walker",
    crate::card::CardArt::new("a6e0adb7-a030-4dcc-9284-cd91c7598a22", "Milivoj Ćeran"),
    crate::card::CardSet::MurdersAtKarlovManor,
    crate::card::CardRules::unsupported(),
);

// MKM 217 — Leyline of the Guildpact
pub(in crate::card::sets) static LEYLINE_OF_THE_GUILDPACT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bf6e59be-f959-4f4a-8c2d-b7c441e88135"),
    "Leyline of the Guildpact",
    CardArt::new("bf6e59be-f959-4f4a-8c2d-b7c441e88135", "Daarken"),
    CardSet::MurdersAtKarlovManor,
    CardRules::new_enchantment(mana_cost!("{G/W}{G/U}{B/G}{R/G}")).with_abilities(&[
        abilities::begin_game_on_battlefield(),
        AbilityDef::static_ability(
            "Each nonland permanent you control is all colors.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_colors(ColorSet::from_colors(&[
                    ManaColor::White,
                    ManaColor::Blue,
                    ManaColor::Black,
                    ManaColor::Red,
                    ManaColor::Green,
                ])),
            },
        ),
        AbilityDef::static_ability(
            "Lands you control are every basic land type in addition to their other types.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Land),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_basic_land_types(&[
                    BasicLandType::Plains,
                    BasicLandType::Island,
                    BasicLandType::Swamp,
                    BasicLandType::Mountain,
                    BasicLandType::Forest,
                ]),
            },
        ),
    ]),
);

// MKM 221 — No More Lies
pub(in crate::card::sets) static NO_MORE_LIES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1e0c695d-62f9-4805-9e2f-7032e8464136"),
    "No More Lies",
    CardArt::new("1e0c695d-62f9-4805-9e2f-7032e8464136", "Liiga Smilshkalne"),
    CardSet::MurdersAtKarlovManor,
    // Mana Leak that eats what it catches: the exile is what makes it worth
    // a second color, since nothing gets the spell back afterwards.
    CardRules::new_instant(mana_cost!("{W}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell unless its controller pays {3}. If that spell is countered this \
         way, exile it instead of putting it into its owner's graveyard.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        abilities::counter_target_to_exile_unless_paid(ValueDef::Constant(3)),
    )),
);

// MKM 259 — Commercial District
pub(in crate::card::sets) static COMMERCIAL_DISTRICT: CardRecord = CardRecord::new_with_legacy_id(
    2275,
    "Commercial District",
    CardArt::new(
        "bf220c06-3cce-4bdd-aa58-83940c223e9c",
        "Julian Kok Joon Wen",
    ),
    CardSet::MurdersAtKarlovManor,
    // The red-green half, which wants the graveyard less than the others and
    // plays it anyway because a tapped dual is what the mana costs.
    surveil_land(&["Mountain", "Forest"]),
);

// MKM 261 — Escape Tunnel
pub(in crate::card::sets) static ESCAPE_TUNNEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("93ddde4f-d35e-4128-8f43-d0eadbd715de"),
    "Escape Tunnel",
    CardArt::new(
        "93ddde4f-d35e-4128-8f43-d0eadbd715de",
        "Carlos Palma Cruchaga",
    ),
    CardSet::MurdersAtKarlovManor,
    // A land that taps for nothing: both halves spend the land itself, so
    // playing it is a decision about which one the deck wants later.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated(
            "{T}, Sacrifice this land: Search your library for a basic land card, put it onto the battlefield tapped, then shuffle.",
            &[CostDef::TapSource, CostDef::SacrificeSource],
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Supertype(CardSupertype::Basic),
                ]),
                // A qualified library search may legally fail to find.
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
        ),
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice this land: Target creature with power 2 or less can't be blocked this turn.",
            &[CostDef::TapSource, CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    // The predicate reads power only upwards, so the cap is
                    // the complement of three or more.
                    ObjectPredicateDef::Not(&ObjectPredicateDef::PowerAtLeast(3)),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// MKM 262 — Hedge Maze
pub(in crate::card::sets) static HEDGE_MAZE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5260f8ae-805b-4eae-badf-62de0f768867"),
    "Hedge Maze",
    CardArt::new("5260f8ae-805b-4eae-badf-62de0f768867", "Andrew Mar"),
    CardSet::MurdersAtKarlovManor,
    // The green-blue half of the cycle, and the one whose deck is usually
    // happiest to see the surveil: the graveyard is where half its cards
    // want to be anyway.
    surveil_land(&["Forest", "Island"]),
);

// MKM 263 — Lush Portico
pub(in crate::card::sets) static LUSH_PORTICO: CardRecord = CardRecord::new_with_legacy_id(
    2248,
    "Lush Portico",
    CardArt::new("c17816e8-28b1-4295-a637-efb0e5c18873", "Kamila Szutenberg"),
    CardSet::MurdersAtKarlovManor,
    // The green-white half of the cycle, which the decks that want it are
    // playing for the fixing rather than for the graveyard.
    surveil_land(&["Forest", "Plains"]),
);

// MKM 264 — Meticulous Archive
pub(in crate::card::sets) static METICULOUS_ARCHIVE: CardRecord = CardRecord::new_with_legacy_id(
    2303,
    "Meticulous Archive",
    CardArt::new("652236c2-84ef-45e4-b5fc-ed6170bc3d6c", "Sam Burley"),
    CardSet::MurdersAtKarlovManor,
    // The white-blue half, which wants the graveyard least of the cycle and
    // is played for the dual land the tempo decks cannot otherwise have.
    surveil_land(&["Plains", "Island"]),
);

// MKM 269 — Thundering Falls
pub(in crate::card::sets) static THUNDERING_FALLS: CardRecord = CardRecord::new_with_legacy_id(
    2226,
    "Thundering Falls",
    CardArt::new("17260fff-b239-4af4-9306-3236ae3fa5a5", "Grady Frederick"),
    CardSet::MurdersAtKarlovManor,
    // A dual that costs you the turn it lands and pays a little of it back by
    // filling the graveyard the decks that want it are built around.
    surveil_land(&["Island", "Mountain"]),
);

// MKM 270 — Undercity Sewers
pub(in crate::card::sets) static UNDERCITY_SEWERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2b5801fb-2026-4f25-98bc-ebb2f99684b9"),
    "Undercity Sewers",
    CardArt::new("2b5801fb-2026-4f25-98bc-ebb2f99684b9", "Yeong-Hao Han"),
    CardSet::MurdersAtKarlovManor,
    // The blue-black half, and the one the cycle was designed for: the deck
    // playing it is already trying to fill a graveyard, so the look costs it
    // nothing it was not going to spend.
    surveil_land(&["Island", "Swamp"]),
);

// MKM 329 — Raucous Theater
pub(in crate::card::sets) static RAUCOUS_THEATER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2faf0337-c7a3-45a0-bb14-c431526da2cd"),
    "Raucous Theater",
    CardArt::new("2faf0337-c7a3-45a0-bb14-c431526da2cd", "Sergey Glushakov"),
    CardSet::MurdersAtKarlovManor,
    // The black-red half, which wants the graveyard for what it can cast out
    // of it rather than for a count: the look is a discard the deck was glad
    // to make.
    surveil_land(&["Swamp", "Mountain"]),
);

// MKM 330 — Shadowy Backstreet
pub(in crate::card::sets) static SHADOWY_BACKSTREET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("27eae4ce-e0b3-482b-9136-6fc17333877e"),
    "Shadowy Backstreet",
    CardArt::new("27eae4ce-e0b3-482b-9136-6fc17333877e", "Sergey Glushakov"),
    CardSet::MurdersAtKarlovManor,
    // The white-black half. Its deck is the one least pleased to be given a
    // card it has to bin, which is why the look is worth reading twice.
    surveil_land(&["Plains", "Swamp"]),
);

// MKM 333 — Underground Mortuary
pub(in crate::card::sets) static UNDERGROUND_MORTUARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0d8938e4-bfa5-47e1-8c71-9c6583346300"),
    "Underground Mortuary",
    CardArt::new("0d8938e4-bfa5-47e1-8c71-9c6583346300", "Sergey Glushakov"),
    CardSet::MurdersAtKarlovManor,
    // The black-green half, whose deck is usually pleased to bin whatever
    // the look turns up: half of what it wants is already in the graveyard.
    surveil_land(&["Swamp", "Forest"]),
);

// MKM 396 — Proft's Eidetic Memory
pub(in crate::card::sets) static PROFT_S_EIDETIC_MEMORY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a3472756-0305-4567-b425-f7dbf9b3cc7f"),
    "Proft's Eidetic Memory",
    CardArt::new("a3472756-0305-4567-b425-f7dbf9b3cc7f", "Julie Dillon"),
    CardSet::MurdersAtKarlovManor,
    // Two mana that replaces itself and then turns every spare cantrip into
    // permanent power, as long as there is a creature to put it on.
    CardRules::new_enchantment(mana_cost!("{1}{U}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When this enchantment enters, draw a card.",
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::static_ability(
                "You have no maximum hand size.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::You)),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::PlayerRule(
                        crate::card::PlayerRuleDef::NoMaximumHandSize,
                    )),
                },
            ),
            AbilityDef::triggered_if_with_targets(
                "At the beginning of combat on your turn, if you've drawn more than one card this turn, \
                 put X +1/+1 counters on target creature you control, where X is the number of cards \
                 you've drawn this turn minus one.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::BeginningOfCombat,
                    player: PlayerRelation::You,
                },
                &// The card it draws on the way in is the first of the turn, so anything at
                    // all afterwards -- a cantrip, a fetchland cracked on their turn is not it,
                    // but a second draw on yours -- turns the trigger on.
                    TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CardsDrawnThisTurn(PlayerRelation::You),
                        comparison: ComparisonDef::Greater,
                        right: ValueDef::Constant(1),
                    }),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    // "Minus one", which is why the card it draws itself is free rather than
                    // the first counter: the draw that turns the ability on is the one it does
                    // not pay for.
                    amount: ValueDef::Sum(&SumValueDef::new(
                        ValueDef::CardsDrawnThisTurn(PlayerRelation::You),
                        ValueDef::Constant(-1),
                    )),
                },
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &NOVICE_INSPECTOR,
    &FORENSIC_GADGETEER,
    &SNARLING_GOREHOUND,
    &RUBBLEBELT_MAVERICK,
    &DOG_WALKER,
    &LEYLINE_OF_THE_GUILDPACT,
    &NO_MORE_LIES,
    &COMMERCIAL_DISTRICT,
    &ESCAPE_TUNNEL,
    &HEDGE_MAZE,
    &LUSH_PORTICO,
    &METICULOUS_ARCHIVE,
    &THUNDERING_FALLS,
    &UNDERCITY_SEWERS,
    &RAUCOUS_THEATER,
    &SHADOWY_BACKSTREET,
    &UNDERGROUND_MORTUARY,
    &PROFT_S_EIDETIC_MEMORY,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
