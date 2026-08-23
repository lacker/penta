//! Urza's Destiny cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AppliedEffectDef, CardArt, CardRules, CardSet, CardType,
    CardTypeSet, CharacteristicOperationDef, CounterKind, EffectDef, EffectRecipientDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectSetDef, PlayerRelation, PowerToughnessOperationDef,
    SetOperationDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement,
};
use crate::mana_cost;

/// Every enchantment card the graveyard holds, all at once. The printed
/// reminder about Auras is the ordinary rule for an Aura arriving with
/// nothing to enchant, not a clause of its own.
static ENCHANTMENTS_IN_YOUR_GRAVEYARD: EffectRecipientDef = EffectRecipientDef::matching_objects(
    ObjectPredicateDef::HasType(CardType::Enchantment),
    &[ZoneKind::Graveyard],
    PlayerRelation::You,
);

/// Every other non-Aura enchantment. An Aura is left alone because a
/// creature Aura would fall off whatever it was attached to, and the
/// enchantment doing the animating is not one of the things it animates.
static OTHER_NON_AURA_ENCHANTMENTS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Enchantment),
        ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Aura")),
        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
    ]),
    &[ZoneKind::Battlefield],
    PlayerRelation::Any,
);

/// A creature in addition to its other types, with a body its own cost
/// decides: the number is read off each affected enchantment rather than off
/// the Opalescence.
static ANIMATE_AS_ITS_OWN_COST: AppliedEffectDef = AppliedEffectDef::Composite(&[
    AppliedEffectDef::Characteristic(CharacteristicOperationDef::CardTypes(SetOperationDef::Add(
        CardTypeSet::single(CardType::Creature),
    ))),
    AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
        PowerToughnessOperationDef::SetBase {
            power: ValueDef::AffectedManaValue,
            toughness: ValueDef::AffectedManaValue,
        },
    )),
]);

// UDS 13 — Opalescence
pub(in crate::card::sets) static OPALESCENCE: CardRecord = CardRecord::new_with_legacy_id(
    2080,
    "Opalescence",
    CardArt::new("c8b66a4d-4ee1-40ba-993a-a56a5cbd2c3c", "John Avon"),
    CardSet::UrzasDestiny,
    // The deck's whole win condition: the enchantments it already wanted to
    // resolve stand up and attack.
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}")).with_ability(AbilityDef::static_ability(
        "Each other non-Aura enchantment is a creature in addition to its other types and has base power and base toughness each equal to its mana value.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                OTHER_NON_AURA_ENCHANTMENTS,
            )),
            effect: ANIMATE_AS_ITS_OWN_COST,
        },
    )),
);

// UDS 15 — Replenish
pub(in crate::card::sets) static REPLENISH: CardRecord = CardRecord::new_with_legacy_id(
    2077,
    "Replenish",
    CardArt::new("c922d401-7916-42d3-9185-9de6219f9c38", "Jim Nelson"),
    CardSet::UrzasDestiny,
    // The deck is built to fill its own graveyard first, so this is not
    // recursion so much as the whole board arriving on one turn.
    CardRules::new_sorcery(mana_cost!("{3}{W}")).with_ability(AbilityDef::spell(
        "Return all enchantment cards from your graveyard to the battlefield.",
        EffectDef::MoveToZone {
            counters: None,
            object: ENCHANTMENTS_IN_YOUR_GRAVEYARD,
            zone: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
            arrival_effect: None,
            attachment: None,
            controller: None,
        },
    )),
);

/// Everything the fuse counters name. A Keg with no counters on it destroys
/// every nothing-cost permanent, which is the mode that answers a board of
/// tokens.
static MATCHING_ARTIFACTS_AND_CREATURES: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::AnyOf(&[
        ObjectPredicateDef::HasType(CardType::Artifact),
        ObjectPredicateDef::HasType(CardType::Creature),
    ]),
    ObjectPredicateDef::ManaValueEqualTo(ValueDef::CountersOnSource(CounterKind::Fuse)),
]);

static KEG_DETONATION: EffectDef = EffectDef::Destroy {
    object: EffectRecipientDef::matching_objects(
        MATCHING_ARTIFACTS_AND_CREATURES,
        &[ZoneKind::Battlefield],
        PlayerRelation::Any,
    ),
    can_regenerate: true,
};

/// The counter is optional, so the Keg can be held at whatever size the board
/// calls for rather than ticking past it.
static KEG_FUSE: EffectDef = EffectDef::May {
    player: EffectRecipientDef::Controller,
    effect: &EffectDef::AddCounters {
        object: EffectRecipientDef::Source,
        kind: CounterKind::Fuse,
        amount: ValueDef::Constant(1),
    },
};

// UDS 124 — Yavimaya Elder
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static YAVIMAYA_ELDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("325d9372-01c9-4e99-a966-13c8f8566e2e"),
    "Yavimaya Elder",
    crate::card::CardArt::new("325d9372-01c9-4e99-a966-13c8f8566e2e", "Ray Lago"),
    crate::card::CardSet::UrzasDestiny,
    crate::card::CardRules::unsupported(),
);

// UDS 136 — Powder Keg
pub(in crate::card::sets) static POWDER_KEG: CardRecord = CardRecord::new_with_legacy_id(
    2053,
    "Powder Keg",
    CardArt::new("4d9715c2-9036-4ae2-a5b4-1b190d50c963", "Dan Frazier"),
    CardSet::UrzasDestiny,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, you may put a fuse counter on this artifact.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            KEG_FUSE,
        ),
        AbilityDef::activated(
            "{T}, Sacrifice this artifact: Destroy each artifact and creature with mana value equal to the number of fuse counters on this artifact.",
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
            KEG_DETONATION,
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&OPALESCENCE, &REPLENISH, &YAVIMAYA_ELDER, &POWDER_KEG];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
