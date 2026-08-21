//! Aetherdrift cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AddManaEffectDef, BasicLandType, CardArt, CardRules, CardSet,
    CardType, ComparisonDef, EffectDef, EffectRecipientDef, ManaColor, ObjectPredicateDef,
    ObjectQueryDef, PlayerRelation, TopCardSelectionDef, TriggerConditionDef, TriggerEventDef,
    ValueDef, ZoneKind, ZonePlacement, abilities, cards,
};
use crate::mana_cost;

/// Impulse's shape, one card deeper and one card wider. The rest going to
/// the bottom rather than the graveyard is what keeps it from being a
/// self-mill, which matters to the decks that play it.
static STOCK_UP_SELECTION: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::Constant(5),
    object: None,
    minimum: 2,
    maximum: 2,
    select_all_matching: false,
    reveal_selected: false,
    selected_zone: ZoneKind::Hand,
    selected_placement: ZonePlacement::Top,
    rest_zone: ZoneKind::Library,
    rest_placement: ZonePlacement::Bottom,
    selected_order_follows_choice: false,
    then: None,
    selected_face_down: None,
};

// DFT 67 — Stock Up
pub(in crate::card::sets) static STOCK_UP: CardRecord = CardRecord::new(
    cards::STOCK_UP,
    "Stock Up",
    CardArt::new("0a786855-6eb4-42c0-a528-4842db46809d", "Izzy"),
    CardSet::Aetherdrift,
    // Two cards for three mana at sorcery speed is unremarkable; seeing five
    // to find them is what puts it in a deck built around one or two cards.
    CardRules::new_sorcery(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell(
        "Look at the top five cards of your library. Put two of them into your hand and the rest on the bottom of your library in any order.",
        EffectDef::LookAtTopAndSelect {
            player: EffectRecipientDef::Controller,
            looker: EffectRecipientDef::Controller,
            selection: &STOCK_UP_SELECTION,
        },
    )),
);

/// "Artifact, creature, and/or enchantment cards with mana value 1 or less."
/// The three types are alternatives and the mana value applies to all of
/// them, so the bound is outside the choice rather than inside it.
static A_CHEAP_PERMANENT_CARD: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::AnyOf(&[
        ObjectPredicateDef::HasType(CardType::Artifact),
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::HasType(CardType::Enchantment),
    ]),
    ObjectPredicateDef::ManaValueAtMost(1),
]);

/// "Up to two" and revealed: a minimum of none, and everything taken is
/// shown, which is what stops the search being private information.
static GEARHULK_SEARCH: EffectDef = EffectDef::SearchZone {
    player: EffectRecipientDef::Controller,
    source: ZoneKind::Library,
    object: A_CHEAP_PERMANENT_CARD,
    minimum: 0,
    maximum: ValueDef::Constant(2),
    reveal: true,
    destination: ZoneKind::Hand,
    placement: ZonePlacement::Top,
    shuffle: true,
    enters_tapped: false,
    binding: None,
    then: None,
};

static BRIGHTGLASS_GEARHULK_ABILITIES: [AbilityDef; 3] = [
    abilities::first_strike(),
    abilities::trample(),
    // "You may" on top of a search that already allows none: declining and
    // finding nothing look the same from the outside, and the card offers
    // both because a library nobody wants to shuffle is a real answer.
    AbilityDef::triggered(
        "When this creature enters, you may search your library for up to two artifact, creature, \
         and/or enchantment cards with mana value 1 or less, reveal them, put them into your \
         hand, then shuffle.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            None,
            Some(ZoneKind::Battlefield),
        ),
        EffectDef::May {
            player: EffectRecipientDef::Controller,
            effect: &GEARHULK_SEARCH,
        },
    ),
];

// DFT 191 — Brightglass Gearhulk
pub(in crate::card::sets) static BRIGHTGLASS_GEARHULK: CardRecord = CardRecord::new(
    cards::BRIGHTGLASS_GEARHULK,
    "Brightglass Gearhulk",
    CardArt::new("3dea5b45-925c-4732-8e9d-fa8232792736", "José Parodi"),
    CardSet::Aetherdrift,
    // A 4/4 first striker with trample that also finds the two one-drops the
    // deck is built around, which is what four coloured pips buy.
    CardRules::new_artifact_creature(mana_cost!("{G}{G}{W}{W}"), &["Construct"], 4, 4)
        .with_abilities(&BRIGHTGLASS_GEARHULK_ABILITIES),
);

/// The verge condition: any land you control with either type answers it,
/// so a Bayou is both halves at once and a land whose types were changed
/// counts for what it is now rather than what it was printed as.
static A_SWAMP_OR_A_FOREST_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Swamp, BasicLandType::Forest]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

static VERGE_HAS_ITS_LAND: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: A_SWAMP_OR_A_FOREST_YOU_CONTROL,
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 1,
};

// DFT 268 — Wastewood Verge
pub(in crate::card::sets) static WASTEWOOD_VERGE: CardRecord = CardRecord::new(
    cards::WASTEWOOD_VERGE,
    "Wastewood Verge",
    CardArt::new("5ceacc7d-d407-4f82-af58-9bdf8426924e", "Bartek Fedyczak"),
    CardSet::Aetherdrift,
    // Untapped and free either way: the green is unconditional, and the
    // black is what the second land in the deck is for.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {G}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
        ),
        AbilityDef::activated_mana_if(
            "{T}: Add {B}. Activate only if you control a Swamp or a Forest.",
            &[AbilityCostDef::TapSource],
            &VERGE_HAS_ITS_LAND,
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black)),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&STOCK_UP, &BRIGHTGLASS_GEARHULK, &WASTEWOOD_VERGE];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
