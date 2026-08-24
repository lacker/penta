//! Edge of Eternities cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AlternativeCastKindDef, CardArt, CardRules, CardSet, CardType, CounterKind,
    EffectDef, EffectRecipientDef, ObjectPredicateDef, ObjectQueryDef, ObjectSetDef,
    PlayerRelation, ReplacementAbilityDef, ReplacementConditionDef, ReplacementEffectDef,
    ReplacementEventDef, TopCardSelectionDef, TriggerConditionDef, TriggerEventDef, TurnStepDef,
    ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::mana_cost;

// EOE 2 — Tezzeret, Cruel Captain
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TEZZERET_CRUEL_CAPTAIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("02e8e540-8aa3-4e6a-9a11-c3949cab5f0f"),
    "Tezzeret, Cruel Captain",
    crate::card::CardArt::new("02e8e540-8aa3-4e6a-9a11-c3949cab5f0f", "Chris Rahn"),
    crate::card::CardSet::EdgeOfEternities,
    crate::card::CardRules::unsupported(),
);

// EOE 9 — Cosmogrand Zenith
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static COSMOGRAND_ZENITH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b3c1e5e3-4e6b-456a-958c-7a75c38f8183"),
    "Cosmogrand Zenith",
    crate::card::CardArt::new("b3c1e5e3-4e6b-456a-958c-7a75c38f8183", "Anna Steinbauer"),
    crate::card::CardSet::EdgeOfEternities,
    crate::card::CardRules::unsupported(),
);

// EOE 18 — Focus Fire
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FOCUS_FIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a9ddfcbc-0f84-4315-aaa3-ca54ff64d7de"),
    "Focus Fire",
    crate::card::CardArt::new("a9ddfcbc-0f84-4315-aaa3-ca54ff64d7de", "Borja Pindado"),
    crate::card::CardSet::EdgeOfEternities,
    crate::card::CardRules::unsupported(),
);

// EOE 51 — Consult the Star Charts
/// "Where X is the number of lands you control", which is the whole reason
/// the card is playable: it looks at more the longer the game goes.
static LANDS_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Land),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

/// One selection differs from the other only in how many it keeps, so the
/// two are the same shape twice rather than a count the spell could carry:
/// how many cards a look takes is printed on it, and this card prints two
/// numbers.
const fn consult_selection(cards: u8) -> TopCardSelectionDef {
    TopCardSelectionDef {
        count: ValueDef::CountMatchingObjects(&LANDS_YOU_CONTROL),
        object: None,
        minimum: cards,
        maximum: cards,
        select_all_matching: false,
        reveal_selected: false,
        selected_zone: ZoneKind::Hand,
        selected_placement: ZonePlacement::Top,
        selected_hidden: false,
        selected_linked_to_source: false,
        selected_face_down: None,
        rest_zone: ZoneKind::Library,
        rest_placement: ZonePlacement::Bottom,
        rest_random_order: true,
        rest_counters: None,
        selected_order_follows_choice: false,
        then: None,
    }
}

static CONSULT_ONE: TopCardSelectionDef = consult_selection(1);

static CONSULT_TWO: TopCardSelectionDef = consult_selection(2);

static CONSULT_WAS_KICKED: TriggerConditionDef =
    TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked);

static CONSULT_NOT_KICKED: TriggerConditionDef = TriggerConditionDef::Not(&CONSULT_WAS_KICKED);

static CONSULT_LOOK_ONE: EffectDef = EffectDef::LookAtTopAndSelect {
    player: EffectRecipientDef::Controller,
    looker: EffectRecipientDef::Controller,
    selection: &CONSULT_ONE,
};

static CONSULT_LOOK_TWO: EffectDef = EffectDef::LookAtTopAndSelect {
    player: EffectRecipientDef::Controller,
    looker: EffectRecipientDef::Controller,
    selection: &CONSULT_TWO,
};

/// The two halves are complementary conditions on one fact rather than an
/// effect with a branch, so each reads the way its own printed clause does.
static CONSULT_EFFECT: [EffectDef; 2] = [
    EffectDef::IfCondition {
        condition: &CONSULT_NOT_KICKED,
        then: &CONSULT_LOOK_ONE,
    },
    EffectDef::IfCondition {
        condition: &CONSULT_WAS_KICKED,
        then: &CONSULT_LOOK_TWO,
    },
];

pub(in crate::card::sets) static CONSULT_THE_STAR_CHARTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a16a6555-2e3a-4587-aacd-0307d696b26c"),
    "Consult the Star Charts",
    CardArt::new(
        "a16a6555-2e3a-4587-aacd-0307d696b26c",
        "Antonio José Manzanedo",
    ),
    CardSet::EdgeOfEternities,
    // Two mana to dig as deep as your mana base, and four to keep twice as
    // much of what it finds.
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::alternative_cast(
            mana_cost!("{2}{U}{U}"),
            AlternativeCastKindDef::Kicked,
            Some("Kicker {1}{U} (You may pay an additional {1}{U} as you cast this spell.)"),
            EffectDef::None,
        ),
        AbilityDef::spell(
            "Look at the top X cards of your library, where X is the number of lands you \
             control. Put one of those cards into your hand. If this spell was kicked, put two \
             of those cards into your hand instead. Put the rest on the bottom of your library \
             in a random order.",
            EffectDef::Sequence(&CONSULT_EFFECT),
        ),
    ]),
);

// EOE 52 — Cryogen Relic
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CRYOGEN_RELIC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7bfb33b6-e2bf-498f-8c58-ae21a840cf75"),
    "Cryogen Relic",
    crate::card::CardArt::new("7bfb33b6-e2bf-498f-8c58-ae21a840cf75", "Eelis Kyttanen"),
    crate::card::CardSet::EdgeOfEternities,
    crate::card::CardRules::unsupported(),
);

// EOE 53 — Cryoshatter
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CRYOSHATTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7b62b1e2-9e43-4a66-a647-7e5de2871f2a"),
    "Cryoshatter",
    crate::card::CardArt::new("7b62b1e2-9e43-4a66-a647-7e5de2871f2a", "Jeremy Wilson"),
    crate::card::CardSet::EdgeOfEternities,
    crate::card::CardRules::unsupported(),
);

// EOE 66 — Mechanozoa
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MECHANOZOA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0cb8d8ce-329a-4a97-b3d8-796703ebcb37"),
    "Mechanozoa",
    crate::card::CardArt::new("0cb8d8ce-329a-4a97-b3d8-796703ebcb37", "Daarken"),
    crate::card::CardSet::EdgeOfEternities,
    crate::card::CardRules::unsupported(),
);

// EOE 72 — Quantum Riddler
/// "As long as you have one or fewer cards in hand, if you would draw one
/// or more cards, you draw that many cards plus one instead." One
/// replacement of the whole instruction: a draw of three becomes a draw of
/// four rather than a draw of six.
static RIDDLER_EXTRA_CARD: ReplacementAbilityDef = ReplacementAbilityDef::new()
    .with_event(ReplacementEventDef::WouldDraw {
        player: PlayerRelation::You,
        during_own_draw_step: false,
    })
    .with_condition(ReplacementConditionDef::ControllerHandAtMost(1));

pub(in crate::card::sets) static QUANTUM_RIDDLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("120be808-ff3b-4fca-96a1-4db6b9825856"),
    "Quantum Riddler",
    CardArt::new("120be808-ff3b-4fca-96a1-4db6b9825856", "Izzy"),
    CardSet::EdgeOfEternities,
    // Five mana for a 4/6 flier that draws a card, or two mana for the same
    // body until the end of turn and the card it comes back with later.
    CardRules::new_creature(mana_cost!("{3}{U}{U}"), &["Sphinx"], 4, 6).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::defined_replacement(
            "As long as you have one or fewer cards in hand, if you would draw one or more \
             cards, you draw that many cards plus one instead.",
            RIDDLER_EXTRA_CARD,
            ReplacementEffectDef::AddToEventAmount(1),
        ),
        abilities::warp(
            mana_cost!("{1}{U}"),
            "Warp {1}{U} (You may cast this card from your hand for its warp cost. Exile it at \
             the beginning of the next end step, then you may cast it from exile on a later \
             turn.)",
        ),
        abilities::warped_exile(),
    ]),
);

// EOE 77 — Starbreach Whale
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STARBREACH_WHALE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8a1a0476-7145-4493-97e5-4fc05c85e476"),
    "Starbreach Whale",
    crate::card::CardArt::new("8a1a0476-7145-4493-97e5-4fc05c85e476", "Sam Burley"),
    crate::card::CardSet::EdgeOfEternities,
    crate::card::CardRules::unsupported(),
);

// EOE 152 — Plasma Bolt
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PLASMA_BOLT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a1a1834b-76c2-4496-b8c5-18b69ab34c4c"),
    "Plasma Bolt",
    crate::card::CardArt::new("a1a1834b-76c2-4496-b8c5-18b69ab34c4c", "Viko Menezes"),
    crate::card::CardSet::EdgeOfEternities,
    crate::card::CardRules::unsupported(),
);

// EOE 201 — Ouroboroid
/// "Each creature you control" includes the Wurm itself, so the counters it
/// hands out make the next round of them bigger.
static CREATURES_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

pub(in crate::card::sets) static OUROBOROID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("209c591a-4ab2-4e89-9523-a7b766cf4e51"),
    "Ouroboroid",
    CardArt::new("209c591a-4ab2-4e89-9523-a7b766cf4e51", "Samuel Perin"),
    CardSet::EdgeOfEternities,
    // A 1/3 that doubles itself every combat and takes the rest of the board
    // with it: one counter each the first turn, two the next, four after
    // that.
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Plant", "Wurm"], 1, 3).with_ability(
        AbilityDef::triggered(
            "At the beginning of combat on your turn, put X +1/+1 counters on each creature you \
             control, where X is this creature's power.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            // X is read once, as the ability resolves, and every creature
            // gets that many -- including the Wurm, whose own growth does
            // not raise the number partway through.
            EffectDef::AddCounters {
                object: EffectRecipientDef::objects(ObjectSetDef::Query(CREATURES_YOU_CONTROL)),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::SourcePower,
            },
        ),
    ),
);

// EOE 244 — Pinnacle Kill-Ship
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PINNACLE_KILL_SHIP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bf784de8-5ae2-4c07-92bb-a5b7f593b773"),
    "Pinnacle Kill-Ship",
    crate::card::CardArt::new("bf784de8-5ae2-4c07-92bb-a5b7f593b773", "Alexandre Honoré"),
    crate::card::CardSet::EdgeOfEternities,
    crate::card::CardRules::unsupported(),
);

// EOE 297 — Mightform Harmonizer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MIGHTFORM_HARMONIZER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("29bc9be4-4fc3-440a-a851-0c7f8989c9b5"),
    "Mightform Harmonizer",
    crate::card::CardArt::new("29bc9be4-4fc3-440a-a851-0c7f8989c9b5", "Jessica Fong"),
    crate::card::CardSet::EdgeOfEternities,
    crate::card::CardRules::unsupported(),
);

// EOE 362 — Icetill Explorer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ICETILL_EXPLORER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("895e5e9b-84dd-4741-8a2c-442165ea9b15"),
    "Icetill Explorer",
    crate::card::CardArt::new("895e5e9b-84dd-4741-8a2c-442165ea9b15", "Raimaru"),
    crate::card::CardSet::EdgeOfEternities,
    crate::card::CardRules::unsupported(),
);

// EOE 391 — The Endstone
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THE_ENDSTONE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1227eb7f-c2a5-4112-98d0-70275a63c26a"),
    "The Endstone",
    crate::card::CardArt::new("1227eb7f-c2a5-4112-98d0-70275a63c26a", "Hidetaka Tenjin"),
    crate::card::CardSet::EdgeOfEternities,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &TEZZERET_CRUEL_CAPTAIN,
    &COSMOGRAND_ZENITH,
    &FOCUS_FIRE,
    &CONSULT_THE_STAR_CHARTS,
    &CRYOGEN_RELIC,
    &CRYOSHATTER,
    &MECHANOZOA,
    &QUANTUM_RIDDLER,
    &STARBREACH_WHALE,
    &PLASMA_BOLT,
    &OUROBOROID,
    &PINNACLE_KILL_SHIP,
    &MIGHTFORM_HARMONIZER,
    &ICETILL_EXPLORER,
    &THE_ENDSTONE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
