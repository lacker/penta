//! Urza's Legacy cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, BattlefieldEntryChoiceDestinationDef,
    BattlefieldEntryScalarChoiceDef, CardArt, CardRules, CardSet, CardType, ChoiceVisibilityDef,
    ChooseDef, ColorChoiceOperationDef, DiscardSelectionDef, EffectDef, EffectRecipientDef,
    ManaColor, ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef, ObjectSetDef,
    PlayerRefDef, PlayerRelation, ReplacementChoiceDef, ReplacementEffectDef,
    ResolvedEffectDurationDef, SpellAdditionalCostCountDef, SpellAdditionalCostDef, SpendModeDef,
    TriggerEventDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::ObjectSetBindingIndex;
use crate::{TargetIndex, mana_cost};

/// Creatures of whatever type the Plague named. The chosen type lives on the
/// enchantment, so the predicate reads it from the ability's source rather
/// than naming a tribe the way a printed lord does.
static CREATURES_OF_THE_CHOSEN_TYPE: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::HasSourcesChosenScalar(BattlefieldEntryChoiceDestinationDef::CreatureType),
]);

/// Any lands, not only your own: the printed clause names no controller,
/// which is what lets it untap a land an opponent's effect left tapped.
static ANY_LANDS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Land),
    &[ZoneKind::Battlefield],
    PlayerRelation::Any,
);

/// The untap follows the discard rather than preceding it, which is the
/// printed order and the reason the card is free: the lands it untaps can
/// pay for the spell it just found.
static SEARCH_UNTAP: EffectDef = EffectDef::Untap {
    object: EffectRecipientDef::objects(ObjectSetDef::Binding(ObjectSetBindingIndex::PRIMARY)),
};

static SEARCH_DISCARD_THEN_UNTAP: EffectDef = EffectDef::Sequence(&[
    EffectDef::Discard {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(2),
        selection: DiscardSelectionDef::RecipientChooses,
        then: None,
    },
    EffectDef::Choose(ChooseDef {
        binding: ObjectChoiceBindingDef::Objects(ObjectSetBindingIndex::PRIMARY),
        unchosen: None,
        chooser: PlayerRefDef::EffectController,
        candidates: ObjectSetDef::Query(ANY_LANDS),
        exclude: None,
        minimum: 0,
        maximum: 3,
        visibility: ChoiceVisibilityDef::Public,
        then: &SEARCH_UNTAP,
    }),
]);

// ULG 14 — Mother of Runes
pub(in crate::card::sets) static MOTHER_OF_RUNES: CardRecord = CardRecord::new_with_legacy_id(
    2119,
    "Mother of Runes",
    CardArt::new("0b1a46ab-95cb-4c24-924f-fc2afd4fcac7", "Scott M. Fischer"),
    CardSet::UrzasLegacy,
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Cleric"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Target creature you control gains protection from the color of your choice until end of turn.",
            &[AbilityCostDef::TapSource],
            &MOTHER_OF_RUNES_TARGET,
            EffectDef::ChooseColor {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                operation: ColorChoiceOperationDef::ProtectionFromChosenColor,
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

static MOTHER_OF_RUNES_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    },
)];

// ULG 32 — Frantic Search
pub(in crate::card::sets) static FRANTIC_SEARCH: CardRecord = CardRecord::new_with_legacy_id(
    2078,
    "Frantic Search",
    CardArt::new("6cec132b-939d-4730-9bbd-2760c63c3cb4", "Jeff Miracola"),
    CardSet::UrzasLegacy,
    // Free if three of the lands paying for it untap again, which is why a
    // deck that wants to fill its graveyard plays it over a plain cantrip.
    CardRules::new_instant(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell(
        "Draw two then discard two cards. Untap up to three lands.",
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
            SEARCH_DISCARD_THEN_UNTAP,
        ]),
    )),
);

// ULG 36 — Miscalculation
pub(in crate::card::sets) static MISCALCULATION: CardRecord = CardRecord::new_with_legacy_id(
    2116,
    "Miscalculation",
    CardArt::new("4b4956a2-9a39-4152-9c98-70e4b2acfa26", "Jeff Laubenstein"),
    CardSet::UrzasLegacy,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Counter target spell unless its controller pays {2}.",
            &[AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Any)],
            abilities::counter_target_unless_paid(ValueDef::Constant(2)),
        ),
        abilities::cycling(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            mana_cost!("{2}"),
        ),
    ]),
);

// ULG 51 — Engineered Plague
pub(in crate::card::sets) static ENGINEERED_PLAGUE: CardRecord = CardRecord::new_with_legacy_id(
    2048,
    "Engineered Plague",
    CardArt::new("27e158d5-efb2-4f90-8898-60ede98f7d29", "Michael Sutfin"),
    CardSet::UrzasLegacy,
    CardRules::new_enchantment(mana_cost!("{2}{B}")).with_abilities(&[
        AbilityDef::replacement(
            "As this enchantment enters, choose a creature type.",
            ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                BattlefieldEntryScalarChoiceDef::CREATURE_TYPE,
            )),
        ),
        // Both players' creatures, which is what makes it a sideboard card
        // rather than a lord: it shrinks the mirror as readily as the matchup
        // it was brought in for.
        AbilityDef::static_ability(
            "All creatures of the chosen type get -1/-1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    CREATURES_OF_THE_CHOSEN_TYPE,
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(-1),
                ),
            },
        ),
    ]),
);

static RANCOR_GRANT: AbilityDef = abilities::trample();

/// Sacrificing a land is what makes this an instant-speed tutor rather than a
/// ramp spell: the land you give up pays for the one you go and get, so the
/// board count never moves.
static SACRIFICE_A_LAND: SpellAdditionalCostDef = SpellAdditionalCostDef {
    object: ObjectPredicateDef::HasType(CardType::Land),
    zone: ZoneKind::Battlefield,
    count: 1,
    counted: SpellAdditionalCostCountDef::Printed,
    spend: SpendModeDef::ByZone,
    or: None,
};

// ULG 98 — Crop Rotation
pub(in crate::card::sets) static CROP_ROTATION: CardRecord = CardRecord::new_with_legacy_id(
    2143,
    "Crop Rotation",
    CardArt::new("6563f790-862c-465a-b963-7a61f2385516", "DiTerlizzi"),
    CardSet::UrzasLegacy,
    CardRules::new_instant(mana_cost!("{G}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, sacrifice a land.\nSearch your library for a land card, put that card onto the battlefield, then shuffle.",
            &[],
            SACRIFICE_A_LAND,
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::HasType(CardType::Land),
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

// ULG 110 — Rancor
pub(in crate::card::sets) static RANCOR: CardRecord = CardRecord::new_with_legacy_id(
    2124,
    "Rancor",
    CardArt::new("59e256c2-38df-4012-9308-ce17dd889e5f", "Kev Walker"),
    CardSet::UrzasLegacy,
    CardRules::new_enchantment(mana_cost!("{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+0 and has trample.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&RANCOR_BONUS),
                },
            ),
            // An Aura that dies with its host still goes to the graveyard, so
            // this fires whether the creature was answered or the Aura was.
            // It is the same trigger either way, and the card that comes back
            // is the one already in the graveyard.
            AbilityDef::triggered(
                "When this Aura is put into a graveyard from the battlefield, return it to its owner's hand.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::TriggeringObject,
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    arrival_effect: None,
                    attachment: None,
                    controller: None,
                },
            ),
        ]),
);

static RANCOR_BONUS: [AppliedEffectDef; 2] = [
    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(0)),
    AppliedEffectDef::add_ability(&RANCOR_GRANT),
];

// ULG 125 — Defense Grid
pub(in crate::card::sets) static DEFENSE_GRID: CardRecord = CardRecord::new_with_legacy_id(
    2065,
    "Defense Grid",
    CardArt::new("5c2592c9-3f8c-4b7e-9e0a-4a6f2c1d8b3e", "Mark Tedin"),
    CardSet::UrzasLegacy,
    // "Except during its controller's turn" is the nonactive player: the tax
    // lands on the instant held up and not on the sorcery cast on time.
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::static_ability(
        "Each spell costs {3} more to cast except during its controller's turn.",
        EffectDef::IncreaseMatchingSpellCostBy {
            spell: ObjectPredicateDef::Any,
            caster: PlayerRelation::NonactivePlayer,
            amount: mana_cost!("{3}"),
        },
    )),
);

// ULG 126 — Grim Monolith
pub(in crate::card::sets) static GRIM_MONOLITH: CardRecord = CardRecord::new_with_legacy_id(
    2118,
    "Grim Monolith",
    CardArt::new("9ddc9fe1-17c8-4e1d-aeb8-c4214e881280", "Chippy"),
    CardSet::UrzasLegacy,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::static_ability(
            "This artifact doesn't untap during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {C}{C}{C}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_amount(3)),
        ),
        AbilityDef::activated(
            "{4}: Untap this artifact.",
            &[AbilityCostDef::Mana(mana_cost!("{4}"))],
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &MOTHER_OF_RUNES,
    &FRANTIC_SEARCH,
    &MISCALCULATION,
    &ENGINEERED_PLAGUE,
    &CROP_ROTATION,
    &RANCOR,
    &DEFENSE_GRID,
    &GRIM_MONOLITH,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
