//! ORI card records required by supported formats.

use crate::card::AddManaEffectDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::CounterKind;
use crate::card::ObjectRefDef;
use crate::card::ReplacementEffectDef;
use crate::card::ZonePlacement;
use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AppliedEffectDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ComparisonDef;
use crate::card::CostDef;
use crate::card::CreateTokenDef;
use crate::card::DiscardSelectionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SubtypeDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "ORI",
    slug: "magic-origins",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// ORI 4 — Archangel of Tithes
// Audit: unsupported — Needs attack/block declaration taxes conditioned on this source being untapped or attacking, including the planeswalkers it protects; costed declarations lack this conditional source-state composition.
pub(in crate::card::sets) static ARCHANGEL_OF_TITHES: CardRecord = CardRecord::new(
    "Archangel of Tithes",
    "1af50bf1-c51e-4592-86bf-4197ec85a45d",
    "Cynthia Sheppard",
    CardRules::unsupported(),
);

// ORI 58 — Harbinger of the Tides
// Audit: unsupported — Needs a casting route that grants instant timing only when an optional additional mana payment is made; existing flash permissions do not carry a timing-specific surcharge.
pub(in crate::card::sets) static HARBINGER_OF_THE_TIDES: CardRecord = CardRecord::new(
    "Harbinger of the Tides",
    "94ca53de-cffb-4740-b318-a4ebfb3a31af",
    "Svetlin Velinov",
    CardRules::unsupported(),
);

// ORI 60 — Jace, Vryn's Prodigy // Jace, Telepath Unbound
pub(in crate::card::sets) static JACE_VRYN_S_PRODIGY: CardRecord = CardRecord::new_dfc(
    "Jace, Vryn's Prodigy // Jace, Telepath Unbound",
    "02d6d693-f1f3-4317-bcc0-c21fa8490d38",
    "Jaime Jones",
    &[
        (
            "Jace, Vryn's Prodigy",
            const {
                CardRules::new_creature(mana_cost!("{1}{U}"), &["Human", "Wizard"], 0, 2)
                .with_supertype(CardSupertype::Legendary)
                .with_abilities(&const { [AbilityDef::activated(
                    "{T}: Draw a card, then discard a card. If there are five or more cards in your graveyard, \
                     exile Jace, then return him to the battlefield transformed under his owner's control.",
                    &[CostDef::TapSource],
                    EffectDef::Sequence(&const { [
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
                            // Counted after the loot, so the card just discarded is one of the five --
                            // which is what makes the turn he arrives and the turn he flips so often
                            // the same turn.
                            condition: &TriggerConditionDef::ObjectCount {
                                query: ObjectQueryDef::matching(
                                    ObjectPredicateDef::Any,
                                    &[ZoneKind::Graveyard],
                                    PlayerRelation::You,
                                ),
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 5,
                            },
                            // The same exile-and-return every flip creature uses: one resolution, so he
                            // is gone and back before anything else happens, and what comes back is a
                            // new object with the loyalty the back face prints.
                            then: &EffectDef::Sequence(&[
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
                            ]),
                        },
                    ] }),
                )] })
            },
        ),
        (
            "Jace, Telepath Unbound",
            const {
                CardRules::new_planeswalker_without_mana_cost(&["Jace"])
                .with_supertype(CardSupertype::Legendary)
                .with_starting_loyalty(5)
                .printed_colors(&[crate::card::ManaColor::Blue])
                .with_abilities(&const { [
                    AbilityDef::activated_with_targets(
                        "+1: Up to one target creature gets -2/-0 until your next turn.",
                        &[CostDef::Loyalty(1)],
                        // "Up to one", so a Jace with nothing worth shrinking still ticks up.
                        &const { [AbilityTargetDef::up_to(
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::HasType(CardType::Creature),
                                zones: &[ZoneKind::Battlefield],
                                controller: None,
                                owner: None,
                            },
                            1,
                        )] },
                        // "Until your next turn" rather than until end of turn: the creature is
                        // smaller on their swing back as well, which is what makes the plus a
                        // defensive ability rather than a combat trick.
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(-2), ValueDef::Constant(0)),
                            duration: ResolvedEffectDurationDef::UntilYourNextTurn,
                        },
                    ),
                    // Written as the flashback his clause comes to: the cost is the card's
                    // own, the window is this turn, and the card is exiled rather than left
                    // in the graveyard. What differs from the printed wording is that the
                    // card is lent the keyword, so anything reading "has flashback" would
                    // see it.
                    AbilityDef::activated_with_targets(
                        "\u{2212}3: You may cast target instant or sorcery card from your graveyard this turn. \
                         If that spell would be put into your graveyard, exile it instead.",
                        &[CostDef::Loyalty(-3)],
                        &const { [AbilityTargetDef::exactly_one(
                                AbilityTargetPredicate::Object {
                                    object: ObjectPredicateDef::AnyOf(&[
                                        ObjectPredicateDef::HasType(CardType::Instant),
                                        ObjectPredicateDef::HasType(CardType::Sorcery),
                                    ]),
                                    zones: &[ZoneKind::Graveyard],
                                    controller: None,
                                    owner: Some(PlayerRelation::You),
                                },
                            )] },
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            effect: AppliedEffectDef::add_ability(&const {
                                abilities::flashback_for_card_mana_cost()
                            }),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    ),
                    AbilityDef::activated(
                        "\u{2212}9: You get an emblem with \"Whenever you cast a spell, target opponent mills \
                         five cards.\"",
                        &[CostDef::Loyalty(-9)],
                        EffectDef::create_emblem("Jace, Telepath Unbound emblem", &const { [AbilityDef::triggered_with_targets(
                            "Whenever you cast a spell, target opponent mills five cards.",
                            TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(PlayerRelation::You)),
                            &const { [AbilityTargetDef::exactly_one(
                                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                            )] },
                            EffectDef::Mill {
                                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                amount: ValueDef::Constant(5),
                            },
                        )] }),
                    ),
                ] })
            },
        ),
    ],
);

// ORI 62 — Jhessian Thief
pub(in crate::card::sets) static JHESSIAN_THIEF: CardRecord = CardRecord::new(
    "Jhessian Thief",
    "33b8553d-d326-4280-bc3a-2fffdd377cd2",
    "Miles Johnston",
    // A 1/3 that gets through on its own rarely, so prowess is what turns a
    // spell-heavy turn into both a bigger body and a card.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Rogue"], 1, 3).with_abilities(&[
        abilities::prowess(),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, draw a card.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// ORI 90 — Dark Petition
pub(in crate::card::sets) static DARK_PETITION_90: CardRecord = CardRecord::new(
    "Dark Petition",
    "e9df9c5e-7087-42b2-9001-c89d40a66c68",
    "Igor Kieryluk",
    CardRules::new_sorcery(mana_cost!("{3}{B}{B}")).with_abilities(&[
AbilityDef::spell("Search your library for a card, put that card into your hand, then shuffle.\nSpell mastery — If there are two or more instant and/or sorcery cards in your graveyard, add {B}{B}{B}.", EffectDef::Sequence(&[EffectDef::SearchZone { player: EffectRecipientDef::Controller, source: ZoneKind::Library, object: ObjectPredicateDef::Any, minimum: 0, maximum: ValueDef::Constant(1), reveal: false, destination: ZoneKind::Hand, placement: ZonePlacement::Top, shuffle: true, enters_tapped: false, attachment: None, binding: None, then: None }, EffectDef::IfCondition { condition: &TriggerConditionDef::ObjectCount { query: ObjectQueryDef::matching(ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Instant), ObjectPredicateDef::HasType(CardType::Sorcery)]), &[ZoneKind::Graveyard], PlayerRelation::You), comparison: ComparisonDef::GreaterOrEqual, amount: 2 }, then: &EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black).with_amount(3)) }]))
]),
);

// ORI 92 — Demonic Pact
// Audit: unsupported — Needs per-incarnation history of previously selected upkeep modes, excluding them from later choices; ordinary modal triggers have no persistent used-mode set.
pub(in crate::card::sets) static DEMONIC_PACT: CardRecord = CardRecord::new(
    "Demonic Pact",
    "82c04014-91f9-4197-b4b4-f62c4739a5c2",
    "Aleksi Briclot",
    CardRules::unsupported(),
);

// ORI 137 — Chandra's Ignition
pub(in crate::card::sets) static CHANDRA_S_IGNITION_137: CardRecord = CardRecord::new(
    "Chandra's Ignition",
    "7d4c90de-49aa-43ed-a18a-f7f96268e5eb",
    "Eric Deschamps",
    CardRules::new_sorcery(mana_cost!("{3}{R}{R}")).with_abilities(&[
AbilityDef::spell_with_targets("Target creature you control deals damage equal to its power to each other creature and each opponent.", &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::HasType(CardType::Creature), zones: &[ZoneKind::Battlefield], controller: Some(PlayerRelation::You), owner: None })], EffectDef::damage_simultaneously(&[
 crate::card::DamageAssignmentDef::from(ObjectRefDef::Target(TargetIndex::PRIMARY), EffectRecipientDef::objects(crate::card::ObjectSetDef::Query(ObjectQueryDef::matching(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::Any).excluding_target(TargetIndex::PRIMARY))), ValueDef::TargetPower(TargetIndex::PRIMARY)),
 crate::card::DamageAssignmentDef::from(ObjectRefDef::Target(TargetIndex::PRIMARY), EffectRecipientDef::Opponent, ValueDef::TargetPower(TargetIndex::PRIMARY)),
]))]),
);

// ORI 155 — Magmatic Insight
pub(in crate::card::sets) static MAGMATIC_INSIGHT_155: CardRecord = CardRecord::new(
    "Magmatic Insight",
    "f00192e0-439d-43b2-882c-90a2d52103f8",
    "Ryan Barger",
    CardRules::new_sorcery(mana_cost!("{R}")).with_abilities(&[
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, discard a land card.\nDraw two cards.",
            &[],
            CostDef::discard(ObjectPredicateDef::HasType(CardType::Land)),
            abilities::draw_cards(ValueDef::Constant(2)),
        ),
    ]),
);

// ORI 162 — Skyraker Giant
pub(in crate::card::sets) static SKYRAKER_GIANT: CardRecord = CardRecord::new(
    "Skyraker Giant",
    "c5f0d87a-8f37-4598-9106-c3545dadf6fd",
    "Anastasia Ovchinnikova",
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Giant"], 4, 3)
        .with_abilities(&[abilities::reach()]),
);

// ORI 171 — Conclave Naturalists
pub(in crate::card::sets) static CONCLAVE_NATURALISTS: CardRecord = CardRecord::new(
    "Conclave Naturalists",
    "3759fc28-9adb-41ed-851c-566a3a424e09",
    "Howard Lyon",
    // A 4/4 body that carries its own answer, so the trigger is optional
    // rather than a liability when the opponent has nothing worth breaking.
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Dryad"], 4, 4).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, you may destroy target artifact or enchantment.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            },
        ),
    ),
);

// ORI 172 — Dwynen, Gilt-Leaf Daen
pub(in crate::card::sets) static DWYNEN_GILT_LEAF_DAEN: CardRecord = CardRecord::new(
    "Dwynen, Gilt-Leaf Daen",
    "91c143a9-c642-425a-a469-a9d158e43c21",
    "Johannes Voss",
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Elf", "Warrior"], 3, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::reach(),
            AbilityDef::static_ability(
                "Other Elf creatures you control get +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elf")),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
            AbilityDef::triggered(
                "Whenever Dwynen attacks, you gain 1 life for each attacking \
                 Elf you control.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elf")),
                            ObjectPredicateDef::Attacking,
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                },
            ),
        ]),
);

// ORI 173 — Dwynen's Elite
pub(in crate::card::sets) static DWYNEN_S_ELITE: CardRecord = CardRecord::new(
    "Dwynen's Elite",
    "c203722b-3f16-4b6c-9b2e-18169d3f80c9",
    "Lius Lasahido",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Elf", "Warrior"], 2, 2).with_abilities(&[
        AbilityDef::triggered_if(
            "When this creature enters, if you control another Elf, create \
             a 1/1 green Elf Warrior creature token.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elf")),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Elf", "Warrior"], &[ManaColor::Green], 1, 1),
            ))),
        ),
    ]),
);

// ORI 174 — Elemental Bond
pub(in crate::card::sets) static ELEMENTAL_BOND_174: CardRecord = CardRecord::new(
    "Elemental Bond",
    "554a8769-c840-4c9d-9959-b075c174457b",
    "David Gaillet",
    CardRules::new_enchantment(mana_cost!("{2}{G}")).with_abilities(&[AbilityDef::triggered(
        "Whenever a creature you control with power 3 or greater enters, draw a card.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ObjectPredicateDef::PowerGreaterThan(ValueDef::Constant(2)),
            ]),
            None,
            Some(ZoneKind::Battlefield),
        ),
        abilities::draw_cards(ValueDef::Constant(1)),
    )]),
);

// ORI 183 — Joraga Invocation
// Audit: unsupported — Needs each affected creature to be blocked by at least one creature if able; MustBeBlockedBy requires every matching creature to block, which is a different requirement.
pub(in crate::card::sets) static JORAGA_INVOCATION: CardRecord = CardRecord::new(
    "Joraga Invocation",
    "65c89431-0881-4aa6-ac15-d4c13b075273",
    "Kieran Yanner",
    CardRules::unsupported(),
);

// ORI 207 — Woodland Bellower
pub(in crate::card::sets) static WOODLAND_BELLOWER_207: CardRecord = CardRecord::new(
    "Woodland Bellower",
    "a706d4bb-0b44-4e43-b340-7de799c086b8",
    "Jasper Sandner",
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Beast"], 6, 5).with_abilities(&[
abilities::enters_trigger("When this creature enters, you may search your library for a nonlegendary green creature card with mana value 3 or less, put it onto the battlefield, then shuffle.", EffectDef::SearchZone { player: EffectRecipientDef::Controller, source: ZoneKind::Library, object: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(CardSupertype::Legendary)), ObjectPredicateDef::Color(ManaColor::Green), ObjectPredicateDef::ManaValueAtMost(3)]), minimum: 0, maximum: ValueDef::Constant(1), reveal: false, destination: ZoneKind::Battlefield, placement: ZonePlacement::Top, shuffle: true, enters_tapped: false, attachment: None, binding: None, then: None })
]),
);

// ORI 229 — Hangarback Walker
pub(in crate::card::sets) static HANGARBACK_WALKER_229: CardRecord = CardRecord::new(
    "Hangarback Walker",
    "791c21fb-fc78-4106-9a42-abc73f41ab8b",
    "Daarken",
    CardRules::new_artifact_creature(mana_cost!("{X}{X}"), &["Construct"], 0, 0).with_abilities(&[
AbilityDef::as_enters("This creature enters with X +1/+1 counters on it.", ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::AddCastXCounters { kind: CounterKind::PlusOnePlusOne })),
abilities::dies_trigger("When this creature dies, create a 1/1 colorless Thopter artifact creature token with flying for each +1/+1 counter on this creature.", EffectDef::CreateToken(crate::card::CreateTokenDef::new(crate::card::TokenDef::Literal(crate::card::TokenCharacteristics::artifact_creature(&["Thopter"], &[], 1, 1).with_abilities(&[abilities::flying()]))).with_count(ValueDef::CountersOnSource(CounterKind::PlusOnePlusOne)))),
AbilityDef::activated("{1}, {T}: Put a +1/+1 counter on this creature.", &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource], EffectDef::AddCounters { object: EffectRecipientDef::Source, kind: CounterKind::PlusOnePlusOne, amount: ValueDef::Constant(1) })
]),
);

// ORI 236 — Pyromancer's Goggles
// Audit: unsupported — Needs a delayed trigger tied to a particular produced mana unit being spent to cast a red instant or sorcery, retaining the cast spell for a copy with optional new targets.
pub(in crate::card::sets) static PYROMANCER_S_GOGGLES: CardRecord = CardRecord::new(
    "Pyromancer's Goggles",
    "1163ce9f-cf22-422e-a4b5-0240b88e2816",
    "James Paick",
    CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ARCHANGEL_OF_TITHES,
    &HARBINGER_OF_THE_TIDES,
    &JACE_VRYN_S_PRODIGY,
    &JHESSIAN_THIEF,
    &DARK_PETITION_90,
    &DEMONIC_PACT,
    &CHANDRA_S_IGNITION_137,
    &MAGMATIC_INSIGHT_155,
    &SKYRAKER_GIANT,
    &CONCLAVE_NATURALISTS,
    &DWYNEN_GILT_LEAF_DAEN,
    &DWYNEN_S_ELITE,
    &ELEMENTAL_BOND_174,
    &JORAGA_INVOCATION,
    &WOODLAND_BELLOWER_207,
    &HANGARBACK_WALKER_229,
    &PYROMANCER_S_GOGGLES,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
