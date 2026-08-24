//! Dominaria cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AppliedEffectDef, CardArt, CardRules, CardSet, CardSupertype,
    CardType, ChoiceVisibilityDef, ChooseDef, CounterKind, EffectDef, EffectRecipientDef,
    ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef,
    PlayerRefDef, PlayerRelation, PlayerSetDef, TokenCharacteristics, TopCardSelectionDef,
    ValueDef, ZoneKind, ZonePlacement,
};
use crate::ids::ObjectBindingIndex;
use crate::mana_cost;

// DOM 1 — Karn, Scion of Urza
/// The opponent chooses which of the two you keep, so what Karn draws is
/// always the worse half -- and the better one waits in exile for his minus.
static KARN_REVEALS_TWO: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::Constant(2),
    object: None,
    minimum: 1,
    maximum: 1,
    select_all_matching: false,
    reveal_selected: true,
    counted: None,
    selected_zone: ZoneKind::Hand,
    selected_placement: ZonePlacement::Top,
    selected_hidden: false,
    selected_linked_to_source: false,
    selected_face_down: None,
    rest_zone: ZoneKind::Exile,
    rest_placement: ZonePlacement::Top,
    rest_random_order: false,
    rest_counters: Some((CounterKind::named("silver"), 1)),
    selected_order_follows_choice: false,
    then: None,
};

/// "A card you own with a silver counter on it from exile": the counter is
/// what makes the pile nameable at all, since exile holds everything anybody
/// has ever put there.
static YOUR_SILVER_CARDS: ObjectQueryDef = ObjectQueryDef::owned_by(
    ObjectPredicateDef::HasCounter(CounterKind::named("silver")),
    &[ZoneKind::Exile],
    PlayerSetDef::Related(PlayerRelation::You),
);

static KARN_RETURNS_IT: EffectDef = EffectDef::MoveToZone {
    counters: None,
    object: EffectRecipientDef::object(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
    from: None,
    zone: ZoneKind::Hand,
    placement: ZonePlacement::Top,
    controller: None,
    arrival_effect: None,
    attachment: None,
    tapped: false,
};

/// "This token gets +1/+1 for each artifact you control", which counts the
/// token itself: a lone Construct is a 1/1, and every artifact beside it is
/// another point in both directions.
static ARTIFACTS_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Artifact),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

static KARN_CONSTRUCT_ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
    "This token gets +1/+1 for each artifact you control.",
    EffectDef::StaticApply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::modify_power_toughness(
            ValueDef::CountMatchingObjects(&ARTIFACTS_YOU_CONTROL),
            ValueDef::CountMatchingObjects(&ARTIFACTS_YOU_CONTROL),
        ),
    },
)];

static KARN_CONSTRUCT: TokenCharacteristics =
    TokenCharacteristics::artifact_creature(&["Construct"], &[], 0, 0)
        .with_abilities(&KARN_CONSTRUCT_ABILITIES);

static KARN_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::activated(
        "+1: Reveal the top two cards of your library. An opponent chooses one of them. Put that \
         card into your hand and exile the other with a silver counter on it.",
        &[AbilityCostDef::Loyalty(1)],
        EffectDef::LookAtTopAndSelect {
            player: EffectRecipientDef::Controller,
            looker: EffectRecipientDef::Opponent,
            selection: &KARN_REVEALS_TWO,
        },
    ),
    AbilityDef::activated(
        "\u{2212}1: Put a card you own with a silver counter on it from exile into your hand.",
        &[AbilityCostDef::Loyalty(-1)],
        EffectDef::Choose(ChooseDef {
            binding: ObjectChoiceBindingDef::Object(ObjectBindingIndex::PRIMARY),
            unchosen: None,
            chooser: PlayerRefDef::EffectController,
            candidates: ObjectSetDef::Query(YOUR_SILVER_CARDS),
            exclude: None,
            minimum: 1,
            maximum: 1,
            visibility: ChoiceVisibilityDef::Public,
            then: &KARN_RETURNS_IT,
        }),
    ),
    AbilityDef::activated(
        "\u{2212}2: Create a 0/0 colorless Construct artifact creature token with \"This token \
         gets +1/+1 for each artifact you control.\"",
        &[AbilityCostDef::Loyalty(-2)],
        EffectDef::create_token(KARN_CONSTRUCT),
    ),
];

pub(in crate::card::sets) static KARN_SCION_OF_URZA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("07a3d9e8-8597-498b-869c-cff79e0df516"),
    "Karn, Scion of Urza",
    CardArt::new("07a3d9e8-8597-498b-869c-cff79e0df516", "Chase Stone"),
    CardSet::Dominaria,
    // Colorless, so every deck can play him: a card every turn that the
    // other player picks, the pile of leftovers he can cash in later, and a
    // body that grows with the artifacts the deck is made of.
    CardRules::new_planeswalker(mana_cost!("{4}"), &["Karn"], 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&KARN_ABILITIES),
);

// DOM 207 — Teferi, Hero of Dominaria
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TEFERI_HERO_OF_DOMINARIA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5d10b752-d9cb-419d-a5c4-d4ee1acb655e"),
    "Teferi, Hero of Dominaria",
    crate::card::CardArt::new("5d10b752-d9cb-419d-a5c4-d4ee1acb655e", "Chris Rallis"),
    crate::card::CardSet::Dominaria,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&KARN_SCION_OF_URZA, &TEFERI_HERO_OF_DOMINARIA];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
