//! Stronghold cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::sets::PrintingAnchor;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AddManaEffectDef, AppliedEffectDef,
    AppliedRuleDef, AttackDefenderScopeDef, AttackRestrictionDef, CardArt, CardRules, CardSet,
    CardSupertype, CardType, DamageEventMatcherDef, DamagePreventionDef, EffectDef,
    EffectPaymentCostDef, EffectPaymentDef, EffectRecipientDef, ObjectPredicateDef, PlayerRelation,
    PlayerSetDef, ReplacementEffectDef, ResolvedEffectDurationDef, SpellAdditionalCostDef,
    ValueDef, ZoneKind, abilities,
};
use crate::mana_cost;

// STH 36 — Mana Leak
pub(in crate::card::sets) static MANA_LEAK: CardRecord = CardRecord::new_with_legacy_id(
    272,
    "Mana Leak",
    CardArt::new("abcaf16d-aa02-43e2-aa38-bb1835d47a05", "Christopher Rush"),
    CardSet::Stronghold,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell unless its controller pays {3}.",
        &[AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Any)],
        abilities::counter_target_unless_paid(ValueDef::Constant(3)),
    )),
);

static SACRIFICE_A_LAND: SpellAdditionalCostDef = SpellAdditionalCostDef::new(
    ObjectPredicateDef::HasType(CardType::Land),
    ZoneKind::Battlefield,
    1,
);

// STH 74 — Tortured Existence
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TORTURED_EXISTENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1754b92b-d6f9-4503-af01-dee03f72a048"),
    "Tortured Existence",
    crate::card::CardArt::new("1754b92b-d6f9-4503-af01-dee03f72a048", "Keith Parkinson"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 104 — Constant Mists
pub(in crate::card::sets) static CONSTANT_MISTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("97a8a5fe-0391-489b-9556-0a1bf7e1900d"),
    "Constant Mists",
    CardArt::new("97a8a5fe-0391-489b-9556-0a1bf7e1900d", "Dermot Power"),
    CardSet::Stronghold,
    CardRules::new_instant(mana_cost!("{1}{G}")).with_abilities(&[
        abilities::buyback_with_additional_cost(
            "Buyback—Sacrifice a land. (You may sacrifice a land in addition to any other costs as you cast this spell. If you do, put this card into your hand as it resolves.)",
            &SACRIFICE_A_LAND,
        ),
        AbilityDef::spell(
            "Prevent all combat damage that would be dealt this turn.",
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::COMBAT),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

/// A land card from hand, which is the whole cost. A hand with none cannot
/// pay at all, and the Mox goes straight to the graveyard.
static A_LAND_CARD: ObjectPredicateDef = ObjectPredicateDef::HasType(CardType::Land);

static MOX_DIAMOND_ENTRY: ReplacementEffectDef = ReplacementEffectDef::PayOr {
    payment: EffectPaymentDef {
        payer: PlayerSetDef::Related(PlayerRelation::You),
        cost: EffectPaymentCostDef::DiscardMatching(A_LAND_CARD),
    },
    // Paying changes nothing about the entry: the Mox arrives as it was
    // going to. Declining is what redirects it.
    if_paid: &[],
    if_declined: &[ReplacementEffectDef::MoveToZone(ZoneKind::Graveyard)],
};

/// Basic lands only, which is why the Druid empties a library that holds
/// none: what it does not find, it passes over into the graveyard.
static A_BASIC_LAND_CARD: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::Supertype(CardSupertype::Basic),
    ObjectPredicateDef::HasType(CardType::Land),
]);

// STH 108 — Hermit Druid
pub(in crate::card::sets) static HERMIT_DRUID: CardRecord = CardRecord::new_with_legacy_id(
    2070,
    "Hermit Druid",
    CardArt::new("a912f57d-9622-453d-826d-ef3d83644850", "Heather Hudson"),
    CardSet::Stronghold,
    // Printed as land smoothing. A deck with no basic lands at all reads the
    // same ability as "put your library into your graveyard", which is the
    // only reason anyone plays it.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Druid"], 1, 1).with_ability(
        AbilityDef::activated(
            "{G}, {T}: Reveal cards from the top of your library until you reveal a basic land card. Put that card into your hand and all other cards revealed this way into your graveyard.",
            &[
                AbilityCostDef::Mana(mana_cost!("{G}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::MillUntil {
                player: EffectRecipientDef::Controller,
                object: A_BASIC_LAND_CARD,
                matched_zone: ZoneKind::Hand,
                binding: None,
                then: None,
            },
        ),
    ),
);

static ENSNARING_BRIDGE_HAND_SIZE: ValueDef = ValueDef::CardsInHandAbove {
    player: PlayerRelation::You,
    threshold: 0,
};

// STH 133 — Ensnaring Bridge
pub(in crate::card::sets) static ENSNARING_BRIDGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("27d838a1-2739-45f7-a856-6202334fa76a"),
    "Ensnaring Bridge",
    CardArt::new("27d838a1-2739-45f7-a856-6202334fa76a", "Pete Venters"),
    CardSet::Stronghold,
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::static_ability(
        "Creatures with power greater than the number of cards in your hand can't attack.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::EachPlayer,
            effect: AppliedEffectDef::Rule(AppliedRuleDef::AttackRestriction(
                AttackRestrictionDef::prohibit(
                    ObjectPredicateDef::PowerGreaterThan(ENSNARING_BRIDGE_HAND_SIZE),
                    AttackDefenderScopeDef::AffectedPlayerOrPlaneswalker,
                ),
            )),
        },
    )),
);

// STH 138 — Mox Diamond
pub(in crate::card::sets) static MOX_DIAMOND: CardRecord = CardRecord::new_with_legacy_id(
    2052,
    "Mox Diamond",
    CardArt::new("28028830-83ed-45e2-b495-3b9ad9d3e988", "Dan Frazier"),
    CardSet::Stronghold,
    // Free mana that costs a land: the deck playing one is trading a card for
    // the turn it comes down.
    CardRules::new_artifact(mana_cost!("{0}")).with_abilities(&[
        AbilityDef::replacement(
            "If this artifact would enter, you may discard a land card instead. If you do, put this artifact onto the battlefield. If you don't, put it into its owner's graveyard.",
            MOX_DIAMOND_ENTRY,
        ),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &MANA_LEAK,
    &TORTURED_EXISTENCE,
    &CONSTANT_MISTS,
    &HERMIT_DRUID,
    &ENSNARING_BRIDGE,
    &MOX_DIAMOND,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
