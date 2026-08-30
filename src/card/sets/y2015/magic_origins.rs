//! ORI card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef,
    CardArt, CardRules, CardSet, CardSupertype, CardType, ComparisonDef, DiscardSelectionDef,
    EffectDef, EffectRecipientDef, ObjectPredicateDef, ObjectQueryDef, PlayerRelation,
    ResolvedEffectDurationDef, TriggerConditionDef, TriggerEventDef, ValueDef, ZoneKind, abilities,
};
use crate::{TargetIndex, mana_cost};

// ORI 60 — Jace, Vryn's Prodigy // Jace, Telepath Unbound
/// Counted after the loot, so the card just discarded is one of the five --
/// which is what makes the turn he arrives and the turn he flips so often
/// the same turn.
static FIVE_IN_YOUR_GRAVEYARD: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef::matching(
        ObjectPredicateDef::Any,
        &[ZoneKind::Graveyard],
        PlayerRelation::You,
    ),
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 5,
};

/// The same exile-and-return every flip creature uses: one resolution, so he
/// is gone and back before anything else happens, and what comes back is a
/// new object with the loyalty the back face prints.
static JACE_TURNS_OVER: [EffectDef; 2] = [
    EffectDef::ExileLinkedToSource {
        until_source_leaves: false,
        object: EffectRecipientDef::Source,
        face_down: false,
        then: None,
    },
    EffectDef::ReturnLinkedExiles {
        object: ObjectPredicateDef::Any,
        counters: None,
        zone: ZoneKind::Battlefield,
        grant: None,
        controller: None,
        transformed: true,
    },
];

static JACE_TURNS_OVER_SEQUENCE: EffectDef = EffectDef::Sequence(&JACE_TURNS_OVER);

static JACE_LOOTS: [EffectDef; 3] = [
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
    EffectDef::Discard {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
        selection: DiscardSelectionDef::RecipientChooses,
        then: None,
    },
    EffectDef::IfCondition {
        condition: &FIVE_IN_YOUR_GRAVEYARD,
        then: &JACE_TURNS_OVER_SEQUENCE,
    },
];

static JACE_PRODIGY_ABILITIES: [AbilityDef; 1] = [AbilityDef::activated(
    "{T}: Draw a card, then discard a card. If there are five or more cards in your graveyard, \
     exile Jace, then return him to the battlefield transformed under his owner's control.",
    &[AbilityCostDef::TapSource],
    EffectDef::Sequence(&JACE_LOOTS),
)];

/// "Up to one", so a Jace with nothing worth shrinking still ticks up.
static UP_TO_ONE_CREATURE: [AbilityTargetDef; 1] = [AbilityTargetDef::up_to(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
    1,
)];

/// "Until your next turn" rather than until end of turn: the creature is
/// smaller on their swing back as well, which is what makes the plus a
/// defensive ability rather than a combat trick.
static JACE_SHRINKS_IT: EffectDef = EffectDef::Apply {
    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(-2), ValueDef::Constant(0)),
    duration: ResolvedEffectDurationDef::UntilYourNextTurn,
};

static AN_INSTANT_OR_SORCERY_IN_YOUR_GRAVEYARD: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Instant),
                ObjectPredicateDef::HasType(CardType::Sorcery),
            ]),
            zones: &[ZoneKind::Graveyard],
            controller: None,
            owner: Some(PlayerRelation::You),
        },
    )];

static JACE_EMBLEM_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
)];

static JACE_EMBLEM_ABILITIES: [AbilityDef; 1] = [AbilityDef::triggered_with_targets(
    "Whenever you cast a spell, target opponent mills five cards.",
    TriggerEventDef::SpellCast(ObjectPredicateDef::ControlledBy(PlayerRelation::You)),
    &JACE_EMBLEM_TARGET,
    EffectDef::Mill {
        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: ValueDef::Constant(5),
        binding: None,
        then: None,
    },
)];

static JACE_TELEPATH_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::activated_with_targets(
        "+1: Up to one target creature gets -2/-0 until your next turn.",
        &[AbilityCostDef::Loyalty(1)],
        &UP_TO_ONE_CREATURE,
        JACE_SHRINKS_IT,
    ),
    // Written as the flashback his clause comes to: the cost is the card's
    // own, the window is this turn, and the card is exiled rather than left
    // in the graveyard. What differs from the printed wording is that the
    // card is lent the keyword, so anything reading "has flashback" would
    // see it.
    AbilityDef::activated_with_targets(
        "\u{2212}3: You may cast target instant or sorcery card from your graveyard this turn. \
         If that spell would be put into your graveyard, exile it instead.",
        &[AbilityCostDef::Loyalty(-3)],
        &AN_INSTANT_OR_SORCERY_IN_YOUR_GRAVEYARD,
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::add_ability(&abilities::flashback_for_card_mana_cost()),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    ),
    AbilityDef::activated(
        "\u{2212}9: You get an emblem with \"Whenever you cast a spell, target opponent mills \
         five cards.\"",
        &[AbilityCostDef::Loyalty(-9)],
        EffectDef::create_emblem("Jace, Telepath Unbound emblem", &JACE_EMBLEM_ABILITIES),
    ),
];

const fn jace_vryns_prodigy_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Human", "Wizard"], 0, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&JACE_PRODIGY_ABILITIES)
}

const fn jace_telepath_unbound_rules() -> CardRules {
    CardRules::new_planeswalker_without_mana_cost(&["Jace"])
        .with_supertype(CardSupertype::Legendary)
        .with_starting_loyalty(5)
        .printed_colors(&[crate::card::ManaColor::Blue])
        .with_abilities(&JACE_TELEPATH_ABILITIES)
}

pub(in crate::card::sets) static JACE_VRYN_S_PRODIGY: CardRecord = CardRecord::new_dfc(
    PrintingAnchor::scryfall("e7b5705f-dc56-41af-a781-8a41aaa7c5b8"),
    "Jace, Vryn's Prodigy // Jace, Telepath Unbound",
    CardArt::new("02d6d693-f1f3-4317-bcc0-c21fa8490d38", "Jaime Jones"),
    CardSet::MagicOrigins,
    &[
        ("Jace, Vryn's Prodigy", jace_vryns_prodigy_rules()),
        ("Jace, Telepath Unbound", jace_telepath_unbound_rules()),
    ],
);

// ORI 62 — Jhessian Thief
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static JHESSIAN_THIEF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("33b8553d-d326-4280-bc3a-2fffdd377cd2"),
    "Jhessian Thief",
    crate::card::CardArt::new("33b8553d-d326-4280-bc3a-2fffdd377cd2", "Miles Johnston"),
    crate::card::CardSet::MagicOrigins,
    crate::card::CardRules::unsupported(),
);

// ORI 171 — Conclave Naturalists
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CONCLAVE_NATURALISTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3759fc28-9adb-41ed-851c-566a3a424e09"),
    "Conclave Naturalists",
    crate::card::CardArt::new("3759fc28-9adb-41ed-851c-566a3a424e09", "Howard Lyon"),
    crate::card::CardSet::MagicOrigins,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&JACE_VRYN_S_PRODIGY, &JHESSIAN_THIEF, &CONCLAVE_NATURALISTS];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
