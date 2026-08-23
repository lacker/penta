//! Visions cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AlternativeCastKindDef,
    ArrivalAttachmentDef, BasicLandType, CardArt, CardRules, CardSet, CardType, EffectDef,
    EffectRecipientDef, InstalledTriggerDef, ManaColor, ObjectPredicateDef, PlayerRefDef,
    PlayerRelation, SpellAdditionalCostCountDef, SpellAdditionalCostDef, SpendModeDef,
    TopCardSelectionDef, TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind,
    ZonePlacement, abilities,
};
use crate::card::{
    AppliedEffectDef, AppliedRuleDef, AttackDefenderScopeDef, AttackRestrictionDef, CounterKind,
    EffectPaymentDef, PayOrDef, PlayerSetDef,
};
use crate::{TargetIndex, mana_cost};

static IMPULSE_SELECTION: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::Constant(4),
    object: None,
    minimum: 1,
    maximum: 1,
    select_all_matching: false,
    reveal_selected: false,
    selected_zone: ZoneKind::Hand,
    selected_placement: ZonePlacement::Top,
    rest_zone: ZoneKind::Library,
    rest_placement: ZonePlacement::Bottom,
    rest_random_order: false,
    selected_order_follows_choice: false,
    then: None,
    selected_face_down: None,
};

// VIS 34 — Impulse
pub(in crate::card::sets) static IMPULSE: CardRecord = CardRecord::new_with_legacy_id(
    310,
    "Impulse",
    CardArt::new("9d710a97-062f-4773-b6c6-8aeddeb3b6e8", "Bryan Talbot"),
    CardSet::Visions,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell(
        "Look at the top four cards of your library. Put one of them into your hand and the rest on the bottom of your library in any order.",
        EffectDef::LookAtTopAndSelect {
            player: EffectRecipientDef::Controller,
            looker: EffectRecipientDef::Controller,
            selection: &IMPULSE_SELECTION,
        },
    )),
);

/// Two Mountains off the battlefield, which is why the card is a finisher
/// rather than a burn spell: it is cast from an empty board on the turn the
/// lands stop mattering.
static SACRIFICE_TWO_MOUNTAINS: SpellAdditionalCostDef = SpellAdditionalCostDef {
    or_life: None,
    object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Mountain]),
    zone: ZoneKind::Battlefield,
    count: 2,
    counted: SpellAdditionalCostCountDef::Printed,
    spend: SpendModeDef::ByZone,
    or: None,
};

static FIREBLAST_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::AnyTarget,
)];

static VISION_CHARM_PLAYER: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Any),
)];

static VISION_CHARM_ARTIFACT: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Artifact),
)];

/// The printed first choice is "a land type", which includes the nonbasic
/// ones. Nothing in this card pool carries a nonbasic land subtype, so the
/// choice offered is over the basic types alone.
static VISION_CHARM_MODES: [AbilityDef; 3] = [
    AbilityDef::spell_with_targets(
        "Target player mills four cards.",
        &VISION_CHARM_PLAYER,
        EffectDef::Mill {
            player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(4),
            binding: None,
            then: None,
        },
    ),
    AbilityDef::spell(
        "Choose a land type and a basic land type. Each land of the first chosen type becomes the second chosen type until end of turn.",
        EffectDef::SubstituteBasicLandTypeUntilEndOfTurn {
            chooser: PlayerRefDef::EffectController,
        },
    ),
    AbilityDef::spell_with_targets(
        "Target artifact phases out.",
        &VISION_CHARM_ARTIFACT,
        EffectDef::PhaseOut {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    ),
];

// VIS 49 — Vision Charm
pub(in crate::card::sets) static VISION_CHARM: CardRecord = CardRecord::new_with_legacy_id(
    2090,
    "Vision Charm",
    CardArt::new("0efaa72c-8f65-4488-ad66-80dc877166cc", "Greg Spalenka"),
    CardSet::Visions,
    // One blue for whichever of three the turn calls for. The deck wants the
    // land mode to strand an opponent's colours, and the phase-out to answer
    // an artifact at instant speed.
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::modal_spell(
        "Choose one —\n• Target player mills four cards.\n• Choose a land type and a basic land type. Each land of the first chosen type becomes the second chosen type until end of turn.\n• Target artifact phases out.",
        &VISION_CHARM_MODES,
        1,
        1,
        false,
    )),
);

/// Any graveyard, not only your own: the card is a reanimation spell for
/// whatever died, whoever owned it.
static NECROMANCY_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Graveyard],
        controller: None,
        owner: None,
    },
)];

/// "The controller of the permanent it becomes sacrifices it at the
/// beginning of the next cleanup step" -- the price of casting it at
/// instant speed, and nothing at all when it was cast on your own turn.
static NECROMANCY_SACRIFICES_ITSELF: AbilityDef = AbilityDef::triggered(
    "At the beginning of the next cleanup step, sacrifice this enchantment.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Cleanup,
        player: PlayerRelation::Any,
    },
    EffectDef::Sacrifice {
        object: EffectRecipientDef::Source,
    },
);

static NECROMANCY_CAST_AT_INSTANT_SPEED: TriggerConditionDef =
    TriggerConditionDef::SourceCastAtInstantSpeed;

/// The reanimation and the attachment are one step: what arrives is a new
/// object, so a following effect would have nothing left to name.
static NECROMANCY_REANIMATES: [EffectDef; 2] = [
    EffectDef::MoveToZone {
        counters: None,
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        zone: ZoneKind::Battlefield,
        placement: ZonePlacement::Top,
        controller: Some(PlayerRelation::You),
        arrival_effect: None,
        attachment: Some(ArrivalAttachmentDef::SourceToArrival),
    },
    EffectDef::IfCondition {
        condition: &NECROMANCY_CAST_AT_INSTANT_SPEED,
        then: &EffectDef::InstallTrigger(InstalledTriggerDef::once(&NECROMANCY_SACRIFICES_ITSELF)),
    },
];

// VIS 55 — Crypt Rats
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CRYPT_RATS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("736455f6-c1b3-4a5a-a91f-a0cd3986ed53"),
    "Crypt Rats",
    crate::card::CardArt::new("736455f6-c1b3-4a5a-a91f-a0cd3986ed53", "Paul Lee"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

// VIS 64 — Necromancy
pub(in crate::card::sets) static NECROMANCY: CardRecord = CardRecord::new_with_legacy_id(
    2202,
    "Necromancy",
    CardArt::new("311a6257-dd77-4bb6-81cb-c8e7862350f3", "Pete Venters"),
    CardSet::Visions,
    // Three mana for anything in any graveyard, at instant speed if you are
    // willing to give it back at cleanup. It is typed an Aura from the
    // start rather than becoming one as it enters: the difference is only
    // visible while the spell is on the stack, and nothing there reads it.
    CardRules::new_enchantment(mana_cost!("{2}{B}"))
        .with_subtypes(&["Aura"])
        // "Enchant creature put onto the battlefield with Necromancy" is
        // narrower than this, but the card guarantees the narrowing itself:
        // it only ever attaches to the creature it just reanimated.
        .enchanting(ObjectPredicateDef::HasType(CardType::Creature))
        .with_abilities(&[
            // "As though it had flash" and having flash differ only in what
            // reads the keyword, and nothing in the pool reads an
            // enchantment's.
            abilities::flash(),
            AbilityDef::triggered_with_targets(
                "When this enchantment enters, if it's on the battlefield, it becomes an Aura with \"enchant creature put onto the battlefield with Necromancy.\" Put target creature card from a graveyard onto the battlefield under your control and attach this enchantment to it.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                &NECROMANCY_TARGET,
                EffectDef::Sequence(&NECROMANCY_REANIMATES),
            ),
            AbilityDef::triggered(
                "When this enchantment leaves the battlefield, that creature's controller sacrifices it.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    None,
                ),
                EffectDef::Sacrifice {
                    object: EffectRecipientDef::AttachedPermanent,
                },
            ),
        ]),
);

// VIS 72 — Vampiric Tutor
pub(in crate::card::sets) static VAMPIRIC_TUTOR: CardRecord = CardRecord::new_with_legacy_id(
    2108,
    "Vampiric Tutor",
    CardArt::new("0a07cba3-2e8d-48ec-a6f8-4d2edfcd833d", "Gary Leach"),
    CardSet::Visions,
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell(
        "Search your library for a card, then shuffle and put that card on top. You lose 2 life.",
        EffectDef::Sequence(&VAMPIRIC_TUTOR_EFFECT),
    )),
);

static VAMPIRIC_TUTOR_EFFECT: [EffectDef; 2] = [
    EffectDef::SearchZone {
        player: EffectRecipientDef::Controller,
        source: ZoneKind::Library,
        object: ObjectPredicateDef::Any,
        minimum: 0,
        maximum: ValueDef::Constant(1),
        reveal: false,
        destination: ZoneKind::Library,
        placement: ZonePlacement::Top,
        shuffle: true,
        enters_tapped: false,
        binding: None,
        then: None,
    },
    EffectDef::LoseLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(2),
    },
];

// VIS 79 — Fireblast
pub(in crate::card::sets) static FIREBLAST: CardRecord = CardRecord::new_with_legacy_id(
    2035,
    "Fireblast",
    CardArt::new("b1eb5b2c-1f02-48a6-a287-88eb189d6780", "Michael Danza"),
    CardSet::Visions,
    CardRules::new_instant(mana_cost!("{4}{R}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Fireblast deals 4 damage to any target.",
            &FIREBLAST_TARGET,
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(4),
            },
        ),
        AbilityDef::alternative_cast(
            crate::mana_cost!("{0}"),
            AlternativeCastKindDef::AlternativeCost,
            Some("You may sacrifice two Mountains rather than pay this spell's mana cost."),
            EffectDef::None,
        )
        .with_alternative_additional_cost(&SACRIFICE_TWO_MOUNTAINS),
    ]),
);

static ELEPHANT_GRASS_BLACK_CREATURES: ObjectPredicateDef =
    ObjectPredicateDef::Color(ManaColor::Black);
static ELEPHANT_GRASS_NONBLACK_CREATURES: ObjectPredicateDef =
    ObjectPredicateDef::Not(&ELEPHANT_GRASS_BLACK_CREATURES);

static ELEPHANT_GRASS_SACRIFICE: EffectDef = EffectDef::Sacrifice {
    object: EffectRecipientDef::Source,
};

static ELEPHANT_GRASS_UPKEEP_STEPS: [EffectDef; 2] = [
    EffectDef::AddCounters {
        object: EffectRecipientDef::Source,
        kind: CounterKind::Age,
        amount: ValueDef::Constant(1),
    },
    EffectDef::PayOr(PayOrDef::unless(
        EffectPaymentDef::generic_mana(
            PlayerSetDef::One(PlayerRefDef::EffectController),
            ValueDef::CountersOnSource(CounterKind::Age),
        ),
        &ELEPHANT_GRASS_SACRIFICE,
    )),
];

static ELEPHANT_GRASS_UPKEEP: EffectDef = EffectDef::IfCondition {
    condition: &TriggerConditionDef::SourceOnBattlefield,
    then: &EffectDef::Sequence(&ELEPHANT_GRASS_UPKEEP_STEPS),
};

// VIS 104 — Elephant Grass
pub(in crate::card::sets) static ELEPHANT_GRASS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f4c1f5a7-0d28-43ab-9b66-937e963f42cd"),
    "Elephant Grass",
    CardArt::new("f4c1f5a7-0d28-43ab-9b66-937e963f42cd", "Tony Roberts"),
    CardSet::Visions,
    CardRules::new_enchantment(mana_cost!("{G}")).with_abilities(&[
        AbilityDef::triggered(
            "Cumulative upkeep {1} (At the beginning of your upkeep, put an age counter on this permanent, then sacrifice it unless you pay its upkeep cost for each age counter on it.)",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            ELEPHANT_GRASS_UPKEEP,
        ),
        AbilityDef::static_ability(
            "Black creatures can't attack you.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::AttackRestriction(
                    AttackRestrictionDef::prohibit(
                        ELEPHANT_GRASS_BLACK_CREATURES,
                        AttackDefenderScopeDef::AffectedPlayer,
                    ),
                )),
            },
        ),
        AbilityDef::static_ability(
            "Nonblack creatures can't attack you unless their controller pays {2} for each creature they control that's attacking you.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::AttackRestriction(
                    AttackRestrictionDef::unless_paid(
                        ELEPHANT_GRASS_NONBLACK_CREATURES,
                        AttackDefenderScopeDef::AffectedPlayer,
                        mana_cost!("{2}"),
                    ),
                )),
            },
        ),
    ]),
);

/// A green creature, wherever the card is looking for one. The sacrifice and
/// the search name the same thing, which is what makes this a trade rather
/// than a tutor.
static A_GREEN_CREATURE: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::Color(ManaColor::Green),
]);

/// Paid as the spell is cast, so a board with nothing green on it cannot
/// cast this at all.
static SACRIFICE_A_GREEN_CREATURE: SpellAdditionalCostDef = SpellAdditionalCostDef {
    or_life: None,
    object: A_GREEN_CREATURE,
    zone: ZoneKind::Battlefield,
    count: 1,
    counted: SpellAdditionalCostCountDef::Printed,
    spend: SpendModeDef::ByZone,
    or: None,
};

// VIS 114 — Natural Order
pub(in crate::card::sets) static NATURAL_ORDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0845f0b0-9413-4ddd-861d-9607636bebc6"),
    "Natural Order",
    CardArt::new("0845f0b0-9413-4ddd-861d-9607636bebc6", "Terese Nielsen"),
    CardSet::Visions,
    // Four mana and a Llanowar Elves for whatever the deck is built around.
    // The search is mandatory and the sacrifice is a cost, so the card is a
    // dead draw exactly when the board is empty.
    CardRules::new_sorcery(mana_cost!("{2}{G}{G}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, sacrifice a green creature.\nSearch your \
             library for a green creature card, put it onto the battlefield, then shuffle.",
            &[],
            SACRIFICE_A_GREEN_CREATURE,
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: A_GREEN_CREATURE,
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: false,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: false,
                binding: None,
                then: None,
            },
        ),
    ),
);

// VIS 118 — River Boa
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RIVER_BOA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2e9d5aaf-b7e8-4676-aec8-7d29a0169a2c"),
    "River Boa",
    crate::card::CardArt::new("2e9d5aaf-b7e8-4676-aec8-7d29a0169a2c", "Steve White"),
    crate::card::CardSet::Visions,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &IMPULSE,
    &VISION_CHARM,
    &CRYPT_RATS,
    &NECROMANCY,
    &VAMPIRIC_TUTOR,
    &FIREBLAST,
    &ELEPHANT_GRASS,
    &NATURAL_ORDER,
    &RIVER_BOA,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
