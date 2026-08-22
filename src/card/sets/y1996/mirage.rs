//! Mirage cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AddManaEffectDef, AppliedEffectDef,
    AppliedRuleDef, CardArt, CardRules, CardSet, CardType, EffectDef, EffectPaymentCostDef,
    EffectPaymentDef, EffectRecipientDef, ManaColor, ObjectPredicateDef, ObjectSetDef, PayOrDef,
    PlayerRefDef, PlayerRelation, PlayerSetDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind,
    ZonePlacement, abilities,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

// MIR 14 — Enlightened Tutor
pub(in crate::card::sets) static ENLIGHTENED_TUTOR: CardRecord = CardRecord::new_with_legacy_id(
    313,
    "Enlightened Tutor",
    CardArt::new("cbac1d27-15e2-4e2f-82ab-625a16e096cb", "Dan Frazier"),
    CardSet::Mirage,
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell(
        "Search your library for an artifact or enchantment card, reveal it, then shuffle and put that card on top.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Enchantment),
            ]),
            minimum: 0,
            maximum: ValueDef::Constant(1),
            reveal: true,
            destination: ZoneKind::Library,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            binding: None,
            then: None,
        },
    )),
);

static GRAVE_HASTE: AbilityDef = abilities::haste();

/// The creature exiles itself rather than being named by a delayed trigger:
/// it is the object that arrived, and it carries the clause with it.
static GRAVE_EXILE_AT_END: AbilityDef = AbilityDef::triggered(
    "At the beginning of the next end step, exile this creature.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::End,
        player: PlayerRelation::Any,
    },
    EffectDef::MoveToZone {
        counters: None,
        object: EffectRecipientDef::Source,
        zone: ZoneKind::Exile,
        placement: ZonePlacement::Top,
        arrival_effect: None,
        attachment: None,
        controller: None,
    },
);

static GRAVE_ARRIVAL: AppliedEffectDef = AppliedEffectDef::Composite(&[
    AppliedEffectDef::add_ability(&GRAVE_HASTE),
    AppliedEffectDef::add_ability(&GRAVE_EXILE_AT_END),
]);

// MIR 80 — Mystical Tutor
pub(in crate::card::sets) static MYSTICAL_TUTOR: CardRecord = CardRecord::new_with_legacy_id(
    2107,
    "Mystical Tutor",
    CardArt::new("5d98101f-e32a-4a4a-a649-faa920d111ee", "David O'Connor"),
    CardSet::Mirage,
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell(
        "Search your library for an instant or sorcery card, reveal it, then shuffle and put that card on top.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Instant),
                ObjectPredicateDef::HasType(CardType::Sorcery),
            ]),
            minimum: 0,
            maximum: ValueDef::Constant(1),
            reveal: true,
            destination: ZoneKind::Library,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            binding: None,
            then: None,
        },
    )),
);

// MIR 141 — Shallow Grave
pub(in crate::card::sets) static SHALLOW_GRAVE: CardRecord = CardRecord::new_with_legacy_id(
    2072,
    "Shallow Grave",
    CardArt::new("8932e789-1d1c-4750-837e-e0b45a81c1c7", "John Coulthart"),
    CardSet::Mirage,
    // One turn with the creature, at instant speed, for two mana. The deck
    // that wants it is the one whose creature only has to attack once.
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(AbilityDef::spell(
        "Return the top creature card of your graveyard to the battlefield. That creature gains haste until end of turn. Exile it at the beginning of the next end step.",
        EffectDef::MoveToZone {
            counters: None,
            object: EffectRecipientDef::objects(ObjectSetDef::TopOfGraveyardMatching {
                player: PlayerRefDef::EffectController,
                object: ObjectPredicateDef::HasType(CardType::Creature),
            }),
            zone: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
            arrival_effect: Some(&GRAVE_ARRIVAL),
            attachment: None,
            controller: None,
        },
    )),
);

// MIR 180 — Goblin Tinkerer
pub(in crate::card::sets) static GOBLIN_TINKERER: CardRecord = CardRecord::new_with_legacy_id(
    2022,
    "Goblin Tinkerer",
    CardArt::new("e6529852-8b3e-4a70-a4a1-029e012231c6", "Hannibal King"),
    CardSet::Mirage,
    // The artifact hits back on the way out, which is why a 1/2 body
    // survives a Cursed Scroll and not much larger.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Artificer"], 1, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{R}, {T}: Destroy target artifact. That artifact deals damage equal to its mana value to this creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{R}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Artifact),
            )],
            EffectDef::Sequence(&GOBLIN_TINKERER_PROGRAM),
        ),
    ),
);

/// The damage is read after the destruction, from the target slot's own
/// last-known information: the artifact is already in a graveyard by then,
/// which is the only time the reading is interesting.
static GOBLIN_TINKERER_PROGRAM: [EffectDef; 2] = [
    EffectDef::Destroy {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        can_regenerate: true,
    },
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Source,
        amount: ValueDef::TargetManaValue(TargetIndex::PRIMARY),
    },
];

// MIR 245 — Tranquil Domain
pub(in crate::card::sets) static TRANQUIL_DOMAIN: CardRecord = CardRecord::new_with_legacy_id(
    285,
    "Tranquil Domain",
    CardArt::new(
        "801f34a6-9f22-43c2-b1e5-194395cc7da1",
        "D. Alexander Gregory",
    ),
    CardSet::Mirage,
    CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell(
        "Destroy all non-Aura enchantments.",
        EffectDef::Destroy {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Aura")),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            can_regenerate: true,
        },
    )),
);

// MIR 255 — Worldly Tutor
pub(in crate::card::sets) static WORLDLY_TUTOR: CardRecord = CardRecord::new_with_legacy_id(
    314,
    "Worldly Tutor",
    CardArt::new("f00115bc-b551-4bf5-a121-bebb37201575", "David O'Connor"),
    CardSet::Mirage,
    CardRules::new_instant(mana_cost!("{G}")).with_ability(AbilityDef::spell(
        "Search your library for a creature card, reveal it, then shuffle and put the card on top.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::HasType(CardType::Creature),
            minimum: 0,
            maximum: ValueDef::Constant(1),
            reveal: true,
            destination: ZoneKind::Library,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            binding: None,
            then: None,
        },
    )),
);

// MIR 258 — Cadaverous Bloom
pub(in crate::card::sets) static CADAVEROUS_BLOOM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c9bef70b-61c7-4df5-b4df-09cd6ab2015c"),
    "Cadaverous Bloom",
    CardArt::new("c9bef70b-61c7-4df5-b4df-09cd6ab2015c", "Alan Rabinowitz"),
    CardSet::Mirage,
    CardRules::new_enchantment(mana_cost!("{3}{B}{G}")).with_ability(AbilityDef::activated_mana(
        "Exile a card from your hand: Add {B}{B} or {G}{G}.",
        &[AbilityCostDef::ExileCardFromHand(ObjectPredicateDef::Any)],
        EffectDef::AddMana(
            AddManaEffectDef::choice(&[ManaColor::Black, ManaColor::Green]).with_amount(2),
        ),
    )),
);

// MIR 299 — Cursed Totem
pub(in crate::card::sets) static CURSED_TOTEM: CardRecord = CardRecord::new_with_legacy_id(
    2039,
    "Cursed Totem",
    CardArt::new(
        "cc99ee76-45b6-4f1d-b0b0-7da8775ca90c",
        "D. Alexander Gregory",
    ),
    CardSet::Mirage,
    // Symmetrical and unconditional: it shuts off every creature on the
    // table, including the ones that make mana.
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::static_ability(
        "Activated abilities of creatures can't be activated.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotActivateAbilities),
        },
    )),
);

/// Twelve power, paid in creatures. A board that cannot reach it is never
/// asked, which is the ordinary case: the deck plays this to be answered by
/// its own Stifle, not to be paid for.
static DREADNOUGHT_COST: PayOrDef = PayOrDef::unless(
    EffectPaymentDef {
        payer: PlayerSetDef::One(PlayerRefDef::EffectController),
        cost: EffectPaymentCostDef::SacrificeCreaturesWithTotalPower(12),
    },
    &EffectDef::Sacrifice {
        object: EffectRecipientDef::Source,
    },
);

// MIR 315 — Phyrexian Dreadnought
pub(in crate::card::sets) static PHYREXIAN_DREADNOUGHT: CardRecord = CardRecord::new_with_legacy_id(
    2085,
    "Phyrexian Dreadnought",
    CardArt::new("57fc0c2b-42b6-4d89-845c-6c08587f330e", "Pete Venters"),
    CardSet::Mirage,
    // A 12/12 for one mana whose drawback nobody intends to pay: the deck
    // answers its own trigger and keeps the body.
    CardRules::new_artifact_creature(mana_cost!("{1}"), &["Phyrexian", "Dreadnought"], 12, 12)
        .with_abilities(&[
            abilities::trample(),
            AbilityDef::triggered(
                "When this creature enters, sacrifice it unless you sacrifice any number of creatures with total power 12 or greater.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::PayOr(DREADNOUGHT_COST),
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ENLIGHTENED_TUTOR,
    &MYSTICAL_TUTOR,
    &SHALLOW_GRAVE,
    &GOBLIN_TINKERER,
    &TRANQUIL_DOMAIN,
    &WORLDLY_TUTOR,
    &CADAVEROUS_BLOOM,
    &CURSED_TOTEM,
    &PHYREXIAN_DREADNOUGHT,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
