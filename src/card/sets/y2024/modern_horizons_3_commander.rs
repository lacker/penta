//! Modern Horizons 3 Commander cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    AlternativeCastKindDef, AppliedEffectDef, CardArt, CardRules, CardSet, CardSupertype, CardType,
    ChoiceVisibilityDef, ChooseDef, EffectDef, EffectRecipientDef, ObjectChoiceBindingDef,
    ObjectPredicateDef, ObjectSetDef, PlayerRefDef, PlayerRelation, SpellAdditionalCostDef,
    SpendModeDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::ObjectSetBindingIndex;
use crate::{TargetIndex, mana_cost};

static ULALEK_ABILITIES: [AbilityDef; 2] = [
    abilities::devoid().with_coverage(AbilityCoverageDef::metadata_only(
        "Ulalek's colorlessness is represented directly in its printed color metadata.",
    )),
    AbilityDef::not_implemented(
        "Whenever you cast an Eldrazi spell, you may pay {C}{C}. If you do, copy all spells you control, then copy all other activated and triggered abilities you control. You may choose new targets for the copies. (Mana abilities can't be copied.)",
        "Copying every spell and nonmana ability one player controls, while preserving each copy's choices and allowing new targets, is not modeled.",
    ),
];

// M3C 4 — Ulalek, Fused Atrocity
// Audit: metadata-only — Its creature body and Devoid are catalog metadata; the mass spell-and-ability copy trigger is not executable.
pub(in crate::card::sets) static ULALEK_FUSED_ATROCITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fdad1b0e-d3cc-4d76-ae7e-fee12558cf2c"),
    "Ulalek, Fused Atrocity",
    CardArt::new("fdad1b0e-d3cc-4d76-ae7e-fee12558cf2c", "Alex Konstad"),
    CardSet::ModernHorizons3Commander,
    CardRules::new_creature(mana_cost!("{C/W}{C/U}{C/B}{C/R}{C/G}"), &["Eldrazi"], 2, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_metadata_only_creature_body()
        .printed_colors(&[])
        .with_abilities(&ULALEK_ABILITIES),
);

/// A Lhurgoyf you control -- this one included, which is what "this creature
/// or another" comes to.
static A_LHURGOYF_YOU_CONTROL: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::Subtype("Lhurgoyf"),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

static PYROGOYF_DAMAGE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::AnyTarget,
)];

static PYROGOYF_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::static_ability(
        "Pyrogoyf's power is equal to the number of card types among cards in all graveyards and its toughness is equal to that number plus 1.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Source,
            // The printed toughness carries the "plus 1", so the counted part
            // is the same number on both sides.
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::CardTypesAmongGraveyards(PlayerRelation::Any),
                ValueDef::CardTypesAmongGraveyards(PlayerRelation::Any),
            ),
        },
    )
    .with_coverage(AbilityCoverageDef::partial(
        "A characteristic-defining ability sets power and toughness in every zone. This is a \
         battlefield-only continuous effect, so the value is right wherever the card is played \
         and absent for anything reading it in another zone.",
    )),
    AbilityDef::triggered_with_targets(
        "Whenever this creature or another Lhurgoyf creature you control enters, that creature deals damage equal to its power to any target.",
        TriggerEventDef::zone_changed(A_LHURGOYF_YOU_CONTROL, None, Some(ZoneKind::Battlefield)),
        &PYROGOYF_DAMAGE_TARGET,
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::TriggeringObjectPower,
        },
    )
    .with_coverage(AbilityCoverageDef::partial(
        "The damage is dealt by the Lhurgoyf that entered. Its amount is read from that \
         creature, but the source recorded for the damage is Pyrogoyf, so protection from red \
         and redirection answer the wrong object when some other Lhurgoyf is the one entering. \
         No other Lhurgoyf is cataloged yet.",
    )),
];

/// "From among them" is what the mill just put there, not what the
/// graveyard already held -- and only a creature card among those.
static A_MILLED_CREATURE_CARD: ObjectSetDef = ObjectSetDef::MatchingBinding {
    binding: ObjectSetBindingIndex::PRIMARY,
    object: ObjectPredicateDef::HasType(CardType::Creature),
};

static BARROWGOYF_TAKES_ONE: EffectDef = EffectDef::MoveToZone {
    object: EffectRecipientDef::objects(ObjectSetDef::Binding(BARROWGOYF_TAKEN)),
    zone: ZoneKind::Hand,
    placement: ZonePlacement::Top,
    controller: None,
    arrival_effect: None,
    attachment: None,
};

/// Where the chosen card is saved, kept apart from the milled pile so that
/// "them" and "the one you took" are two different sets.
static BARROWGOYF_TAKEN: ObjectSetBindingIndex = ObjectSetBindingIndex::new(1);

/// A minimum of zero is the second "you may": milling and taking nothing is
/// a legal answer, and a pile with no creature in it never asks.
static BARROWGOYF_CHOOSES: EffectDef = EffectDef::Choose(ChooseDef {
    binding: ObjectChoiceBindingDef::Objects(BARROWGOYF_TAKEN),
    unchosen: None,
    chooser: PlayerRefDef::EffectController,
    candidates: A_MILLED_CREATURE_CARD,
    exclude: None,
    minimum: 0,
    maximum: 1,
    visibility: ChoiceVisibilityDef::Public,
    then: &BARROWGOYF_TAKES_ONE,
});

static BARROWGOYF_MILLS: EffectDef = EffectDef::Mill {
    player: EffectRecipientDef::Controller,
    amount: ValueDef::TriggerEventAmount,
    binding: Some(ObjectSetBindingIndex::PRIMARY),
    then: Some(&BARROWGOYF_CHOOSES),
};

// M3C 50 — Barrowgoyf
pub(in crate::card::sets) static BARROWGOYF: CardRecord = CardRecord::new_with_legacy_id(
    2213,
    "Barrowgoyf",
    CardArt::new("f979fc86-2c7e-49b3-965e-607a203cbfb1", "Igor Kieryluk"),
    CardSet::ModernHorizons3Commander,
    // Deathtouch and lifelink on a body that grows with every graveyard,
    // and every hit digs for the next one.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Lhurgoyf"], 0, 1).with_abilities(&[
        abilities::deathtouch(),
        abilities::lifelink(),
        AbilityDef::static_ability(
            "Barrowgoyf's power is equal to the number of card types among cards in all graveyards and its toughness is equal to that number plus 1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                // The printed toughness carries the "plus 1", so the counted
                // part is the same number on both sides.
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::CardTypesAmongGraveyards(PlayerRelation::Any),
                    ValueDef::CardTypesAmongGraveyards(PlayerRelation::Any),
                ),
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "A characteristic-defining ability sets power and toughness in every zone. This is a \
             battlefield-only continuous effect, so the value is right wherever the card is \
             played and absent for anything reading it in another zone.",
        )),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, you may mill that many cards. If you do, you may put a creature card from among them into your hand.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &BARROWGOYF_MILLS,
            },
        ),
    ]),
);

// M3C 59 — Pyrogoyf
pub(in crate::card::sets) static PYROGOYF: CardRecord = CardRecord::new_with_legacy_id(
    2141,
    "Pyrogoyf",
    CardArt::new("f60be310-4461-4b84-95f0-b2095108bd79", "Xabi Gaztelua"),
    CardSet::ModernHorizons3Commander,
    // The printed body is 0/1: the counted part supplies the rest, and the
    // "plus 1" is the toughness this starts from.
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Lhurgoyf"], 0, 1)
        .with_abilities(&PYROGOYF_ABILITIES),
);

/// Three cards out of your own graveyard, exiled to pay. The card being cast
/// is on the stack by the time costs are paid, so "other" takes care of
/// itself: it is not there to be chosen.
static EXILE_THREE_OTHER_CARDS: SpellAdditionalCostDef =
    SpellAdditionalCostDef::new(ObjectPredicateDef::Any, ZoneKind::Graveyard, 3)
        .spent(SpendModeDef::Exile);

static BLOODBRAID_CHALLENGER_ABILITIES: [AbilityDef; 3] = [
    abilities::cascade(),
    abilities::haste(),
    AbilityDef::alternative_cast(
        mana_cost!("{3}{R}{G}"),
        AlternativeCastKindDef::Escape,
        Some(
            "Escape—{3}{R}{G}, Exile three other cards from your graveyard. (You may cast this \
             card from your graveyard for its escape cost.)",
        ),
        EffectDef::None,
    )
    .with_alternative_additional_cost(&EXILE_THREE_OTHER_CARDS),
];

// M3C 70 — Bloodbraid Challenger
pub(in crate::card::sets) static BLOODBRAID_CHALLENGER: CardRecord = CardRecord::new_with_legacy_id(
    2255,
    "Bloodbraid Challenger",
    CardArt::new("4b39d43d-2a02-4edb-915a-6a7c002c945f", "Lie Setiawan"),
    CardSet::ModernHorizons3Commander,
    // Five mana for a hasty 4/3 and a free spell, and the graveyard keeps
    // handing it back for five more.
    CardRules::new_creature(mana_cost!("{3}{R}{G}"), &["Elf", "Berserker"], 4, 3)
        .with_abilities(&BLOODBRAID_CHALLENGER_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ULALEK_FUSED_ATROCITY,
    &BARROWGOYF,
    &PYROGOYF,
    &BLOODBRAID_CHALLENGER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
