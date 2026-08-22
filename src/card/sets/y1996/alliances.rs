//! Alliances cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AlternativeCastKindDef, CardArt, CardRules, CardSet, CardSupertype, CardType, DividedTotal,
    EffectDef, EffectRecipientDef, InstalledTriggerDef, ManaColor, ObjectPredicateDef,
    ObjectRefDef, PlayerRefDef, PlayerRelation, SpellAdditionalCostDef, SpendModeDef,
    TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::{TargetIndex, mana_cost};

/// Four damage split however the caster likes. There is no printed ceiling on
/// the number of creatures, but the division supplies one anyway: every target
/// must be assigned at least one damage, so four is the most it can ever
/// reach.
static PYROKINESIS_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef {
    predicate: AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[crate::card::ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
    minimum: 1,
    maximum: AbilityTargetDef::UNLIMITED,
    divided_total: Some(DividedTotal::Fixed(4)),
    another: false,
}];

/// Exiled from hand rather than discarded: the card is spent without ever
/// becoming a graveyard card, which is what "exile a red card" means.
static EXILE_A_RED_CARD: SpellAdditionalCostDef =
    SpellAdditionalCostDef::new(ObjectPredicateDef::Color(ManaColor::Red), ZoneKind::Hand, 1)
        .spent(SpendModeDef::Exile);

/// "Up to two" is two questions rather than one number: take the first card,
/// then decide about the second. The reachable answers -- none, one, or both
/// -- are the ones the printed card offers.
static DENIED_CONTROLLER: EffectRecipientDef = EffectRecipientDef::player(
    PlayerRefDef::ControllerOf(ObjectRefDef::Target(TargetIndex::PRIMARY)),
);

static DENIAL_SECOND_DRAW: EffectDef = EffectDef::May {
    player: DENIED_CONTROLLER,
    effect: &EffectDef::DrawCards {
        recipient: DENIED_CONTROLLER,
        amount: ValueDef::Constant(1),
    },
};

static DENIAL_FIRST_DRAW: EffectDef = EffectDef::May {
    player: DENIED_CONTROLLER,
    effect: &EffectDef::Sequence(&[
        EffectDef::DrawCards {
            recipient: DENIED_CONTROLLER,
            amount: ValueDef::Constant(1),
        },
        DENIAL_SECOND_DRAW,
    ]),
};

/// Both draws are delayed to the next upkeep, which is what makes the card a
/// real counterspell rather than a gift: the two cards arrive a turn later,
/// and by then the spell it answered is long gone.
static DENIAL_DRAWS: EffectDef = EffectDef::Sequence(&[
    EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
        "At the beginning of the next turn's upkeep, that spell's controller may draw up to two cards.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::Any,
        },
        DENIAL_FIRST_DRAW,
    ))),
    EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
        "At the beginning of the next turn's upkeep, draw a card.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::Any,
        },
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    ))),
]);

// ALL 22a — Arcane Denial
pub(in crate::card::sets) static ARCANE_DENIAL: CardRecord = CardRecord::new_with_legacy_id(
    2061,
    "Arcane Denial",
    CardArt::new("b0c5728e-4a52-4d2f-9b04-3c1c7d3f5e6a", "Richard Kane Ferguson"),
    CardSet::Alliances,
    // Two mana to answer anything, and the cards it gives back arrive a turn
    // too late to matter in a deck that is about to lock the game up.
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell. Its controller may draw up to two cards at the beginning of the next turn's upkeep.\nYou draw a card at the beginning of the next turn's upkeep.",
        &DENIAL_TARGET,
        EffectDef::Sequence(&[
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Graveyard,
            },
            DENIAL_DRAWS,
        ]),
    )),
);

static DENIAL_TARGET: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Any)];

static EXILE_A_BLUE_CARD: SpellAdditionalCostDef = SpellAdditionalCostDef::new(
    ObjectPredicateDef::Color(ManaColor::Blue),
    ZoneKind::Hand,
    1,
)
.spent(SpendModeDef::Exile);

static FORCE_OF_WILL_TARGET: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Any)];

// ALL 28 — Force of Will
pub(in crate::card::sets) static FORCE_OF_WILL: CardRecord = CardRecord::new_with_legacy_id(
    2174,
    "Force of Will",
    CardArt::new("9a879b60-4381-447d-8a5a-8e0b6a1d49ca", "Terese Nielsen"),
    CardSet::Alliances,
    // Answering a spell for no mana is what makes an entire format possible:
    // a deck can tap out and still not be dead to the one card that would
    // have beaten it.
    CardRules::new_instant(mana_cost!("{3}{U}{U}")).with_abilities(&[
        AbilityDef::alternative_cast(
            mana_cost!("{0}"),
            AlternativeCastKindDef::AlternativeCost,
            Some(
                "You may pay 1 life and exile a blue card from your hand rather than pay this \
                 spell's mana cost.",
            ),
            EffectDef::None,
        )
        .with_alternative_additional_cost(&EXILE_A_BLUE_CARD)
        .with_alternative_life(1),
        AbilityDef::spell_with_targets(
            "Counter target spell.",
            &FORCE_OF_WILL_TARGET,
            EffectDef::counter_target(TargetIndex::PRIMARY),
        ),
    ]),
);

// ALL 78 — Pyrokinesis
pub(in crate::card::sets) static PYROKINESIS: CardRecord = CardRecord::new_with_legacy_id(
    2031,
    "Pyrokinesis",
    CardArt::new("db2a5e85-6cbc-43c1-9362-4056ad017ef0", "Ron Spencer"),
    CardSet::Alliances,
    // The free cast is what the card is played for -- a blowout from an empty
    // board -- so the printed cost alone understates it considerably.
    CardRules::new_instant(mana_cost!("{4}{R}{R}")).with_abilities(&[
        AbilityDef::alternative_cast(
            mana_cost!("{0}"),
            AlternativeCastKindDef::AlternativeCost,
            Some("You may exile a red card from your hand rather than pay this spell's mana cost."),
            EffectDef::None,
        )
        .with_alternative_additional_cost(&EXILE_A_RED_CARD),
        AbilityDef::spell_with_targets(
            "Pyrokinesis deals 4 damage divided as you choose among any number of target creatures.",
            &PYROKINESIS_TARGETS,
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::DividedAmongTargets,
            },
        ),
    ]),
);

// ALL 89 — Elvish Spirit Guide
pub(in crate::card::sets) static ELVISH_SPIRIT_GUIDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5b94f37f-ebdf-4b79-a615-58331d27cf4e"),
    "Elvish Spirit Guide",
    CardArt::new("5b94f37f-ebdf-4b79-a615-58331d27cf4e", "Julie Baroh"),
    CardSet::Alliances,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf", "Spirit"], 2, 2).with_ability(
        AbilityDef::activated_mana(
            "Exile this card from your hand: Add {G}.",
            &[AbilityCostDef::ExileSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
        )
        .with_source_zones(&[ZoneKind::Hand]),
    ),
);

/// The land fetches, and then leaves: the return is a delayed trigger so
/// that the land is available to tap again next turn rather than staying to
/// be tapped twice in one.
static GLACIERS_RETURN: EffectDef =
    EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
        "At the beginning of the next cleanup step, return this land to its owner's hand.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Cleanup,
            player: PlayerRelation::Any,
        },
        EffectDef::MoveToZone {
            counters: None,
            object: EffectRecipientDef::Source,
            zone: ZoneKind::Hand,
            controller: None,
            placement: ZonePlacement::Top,
            arrival_effect: None,
            attachment: None,
        },
    )));

static GLACIERS_FETCH: EffectDef = EffectDef::Sequence(&[
    EffectDef::SearchZone {
        player: EffectRecipientDef::Controller,
        source: ZoneKind::Library,
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Land),
            ObjectPredicateDef::Supertype(CardSupertype::Basic),
        ]),
        minimum: 0,
        maximum: ValueDef::Constant(1),
        reveal: false,
        destination: ZoneKind::Battlefield,
        placement: ZonePlacement::Top,
        shuffle: true,
        enters_tapped: true,
        binding: None,
        then: None,
    },
    GLACIERS_RETURN,
]);

// ALL 144 — Thawing Glaciers
pub(in crate::card::sets) static THAWING_GLACIERS: CardRecord = CardRecord::new_with_legacy_id(
    2057,
    "Thawing Glaciers",
    CardArt::new("6411a8c6-010f-4863-a0fa-bbebe09d5c34", "Jeff A. Menges"),
    CardSet::Alliances,
    // One basic a turn, forever: slow enough that only a deck with nothing
    // better to do at end of turn wants it, which is exactly Landstill.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::activated(
            "{1}, {T}: Search your library for a basic land card, put that card onto the battlefield tapped, then shuffle. Return this land to its owner's hand at the beginning of the next cleanup step.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
            ],
            GLACIERS_FETCH,
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ARCANE_DENIAL,
    &FORCE_OF_WILL,
    &PYROKINESIS,
    &ELVISH_SPIRIT_GUIDE,
    &THAWING_GLACIERS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
