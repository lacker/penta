//! Dominaria cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AppliedEffectDef, CardArt,
    CardRules, CardSet, CardSupertype, CardType, ChoiceVisibilityDef, ChooseDef, CostAdjustmentDef,
    CostAmountDef, CounterKind, DrawEventMatcherDef, EffectDef, EffectRecipientDef,
    InstalledTriggerDef, ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef,
    ObjectSetDef, PlayerRefDef, PlayerRelation, PlayerSetDef, SpellCastQueryDef,
    SpellCostConditionDef, TokenCharacteristics, TopCardSelectionDef, TriggerEventDef, TurnStepDef,
    ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::{ObjectBindingIndex, ObjectSetBindingIndex, TargetIndex};
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
    select_one_of_each_type: false,
    reveal_inspected: false,
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
    object: EffectRecipientDef::object(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
    zone: ZoneKind::Hand,
    placement: ZonePlacement::Top,
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
static TEFERI_UNTAPS_THE_CHOSEN: EffectDef = EffectDef::Untap {
    object: EffectRecipientDef::objects(ObjectSetDef::Binding(ObjectSetBindingIndex::PRIMARY)),
};

/// Chosen as the delayed trigger resolves rather than targeted, and nothing
/// says whose lands they are -- the same shape Time Spiral's six use, with
/// "up to" meaning a minimum of none.
static TEFERI_UNTAPS_TWO_LANDS: EffectDef = EffectDef::Choose(ChooseDef {
    binding: ObjectChoiceBindingDef::Objects(ObjectSetBindingIndex::PRIMARY),
    unchosen: None,
    chooser: PlayerRefDef::EffectController,
    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
        ObjectPredicateDef::HasType(CardType::Land),
        &[ZoneKind::Battlefield],
        PlayerRelation::Any,
    )),
    exclude: None,
    minimum: 0,
    maximum: 2,
    visibility: ChoiceVisibilityDef::Public,
    then: &TEFERI_UNTAPS_THE_CHOSEN,
});

/// "The next end step" is whichever one comes first, which on Teferi's own
/// turn is his: the two lands come back before the other player untaps, and
/// that is the whole trick.
static TEFERI_END_STEP: AbilityDef = AbilityDef::triggered(
    "At the beginning of the next end step, untap up to two lands.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::End,
        player: PlayerRelation::Any,
    },
    TEFERI_UNTAPS_TWO_LANDS,
);

static TEFERI_DRAWS_THEN_UNTAPS: [EffectDef; 2] = [
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
    EffectDef::InstallTrigger(InstalledTriggerDef::once(&TEFERI_END_STEP)),
];

static TEFERI_TUCK_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
)];

/// Third from the top, so two cards have to be drawn before it comes back --
/// and unlike a bounce it answers a permanent that would rather be in a hand
/// or a graveyard.
static TEFERI_TUCKS_IT: EffectDef = EffectDef::MoveToZone {
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    zone: ZoneKind::Library,
    placement: ZonePlacement::FromTop(3),
};

static TEFERI_EMBLEM_EXILES_IT: EffectDef = EffectDef::MoveToZone {
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    zone: ZoneKind::Exile,
    placement: ZonePlacement::Top,
};

static TEFERI_EMBLEM_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
)];

/// One trigger per card drawn, which is what makes the emblem and the plus
/// the same card: every draw for the rest of the game eats a permanent.
static TEFERI_EMBLEM_ABILITIES: [AbilityDef; 1] = [AbilityDef::triggered_with_targets(
    "Whenever you draw a card, exile target permanent an opponent controls.",
    TriggerEventDef::DrewCard(DrawEventMatcherDef::any(PlayerRelation::You)),
    &TEFERI_EMBLEM_TARGET,
    TEFERI_EMBLEM_EXILES_IT,
)];

static TEFERI_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::activated(
        "+1: Draw a card. At the beginning of the next end step, untap up to two lands.",
        &[AbilityCostDef::Loyalty(1)],
        EffectDef::Sequence(&TEFERI_DRAWS_THEN_UNTAPS),
    ),
    AbilityDef::activated_with_targets(
        "\u{2212}3: Put target nonland permanent into its owner's library third from the top.",
        &[AbilityCostDef::Loyalty(-3)],
        &TEFERI_TUCK_TARGET,
        TEFERI_TUCKS_IT,
    ),
    AbilityDef::activated(
        "\u{2212}8: You get an emblem with \"Whenever you draw a card, exile target permanent an \
         opponent controls.\"",
        &[AbilityCostDef::Loyalty(-8)],
        EffectDef::create_emblem("Teferi, Hero of Dominaria emblem", &TEFERI_EMBLEM_ABILITIES),
    ),
];

pub(in crate::card::sets) static TEFERI_HERO_OF_DOMINARIA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5d10b752-d9cb-419d-a5c4-d4ee1acb655e"),
    "Teferi, Hero of Dominaria",
    crate::card::CardArt::new("5d10b752-d9cb-419d-a5c4-d4ee1acb655e", "Chris Rallis"),
    crate::card::CardSet::Dominaria,
    // Five mana that draws a card and leaves two lands up, so the turn he
    // lands is not the turn he costs you: the plus pays for the counterspell
    // held behind him.
    CardRules::new_planeswalker(mana_cost!("{3}{W}{U}"), &["Teferi"], 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&TEFERI_ABILITIES),
);

// DOM 213 — Damping Sphere
// Audit: partial — The spell tax is declarative. The mana-production clause needs a static replacement that changes a land ability producing two or more mana into exactly {C}.
static DAMPING_SPHERE_CASTS: SpellCastQueryDef = SpellCastQueryDef {
    player: PlayerRelation::You,
    spell: ObjectPredicateDef::Any,
};

static DAMPING_SPHERE_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::static_ability(
        "If a land is tapped for two or more mana, it produces {C} instead of any other type and amount.",
        EffectDef::None,
    )
    .with_coverage(AbilityCoverageDef::metadata_only(
        "Needs a static replacement that changes a land mana ability producing two or more mana into exactly one colorless mana.",
    )),
    abilities::spell_cost_adjustment(
        "Each spell a player casts costs {1} more to cast for each other spell that player has cast this turn.",
        ObjectPredicateDef::Any,
        PlayerRelation::Any,
        SpellCostConditionDef::Always,
        CostAdjustmentDef::Add(CostAmountDef::Generic(ValueDef::CountSpellsCastThisTurn(
            &DAMPING_SPHERE_CASTS,
        ))),
    ),
];

pub(in crate::card::sets) static DAMPING_SPHERE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a5c7d16b-8f4e-42b9-be24-3cb091932d7c"),
    "Damping Sphere",
    CardArt::new("a5c7d16b-8f4e-42b9-be24-3cb091932d7c", "Adam Paquette"),
    CardSet::Dominaria,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&DAMPING_SPHERE_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &KARN_SCION_OF_URZA,
    &TEFERI_HERO_OF_DOMINARIA,
    &DAMPING_SPHERE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
