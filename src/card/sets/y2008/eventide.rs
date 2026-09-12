//! Eventide cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::ManaColor;
use crate::ZoneKind;
use crate::ZonePlacement;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AlternativeCastKindDef;
use crate::card::AppliedEffectDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::ChoiceVisibilityDef;
use crate::card::CostDef;
use crate::card::CreateTokenDef;
use crate::card::CreatureTypeSetDef;
use crate::card::DiscardSelectionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SubtypeDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::abilities;
use crate::ids::TargetIndex;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "EVE",
    slug: "eventide",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// EVE 1 — Archon of Justice
pub(in crate::card::sets) static ARCHON_OF_JUSTICE: CardRecord = CardRecord::new(
    "Archon of Justice",
    "ab707e7f-8ab5-43f1-9428-6a17c1b672fa",
    "Jason Chan",
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Archon"], 4, 4).with_abilities(&[
        abilities::flying(),
        abilities::dies_trigger_with_targets(
            "When this creature dies, exile target permanent.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Any,
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Exile,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// EVE 6 — Flickerwisp
pub(in crate::card::sets) static FLICKERWISP: CardRecord = CardRecord::new(
    "Flickerwisp",
    "5bb3cb5c-8d66-4f5e-a9a9-917e6045f024",
    "Jeremy Enecio",
    // Three mana for a 3/1 flier that also answers something for a turn:
    // an attacker, a blocker, a land on the turn it matters, or one of your
    // own permanents that would rather enter again.
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Elemental"], 3, 1).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, exile another target permanent. Return that card to the \
             battlefield under its owner's control at the beginning of the next end step.",
            // "Another target permanent": his own arrival cannot answer itself, and
            // nothing else is out of reach -- a land is as blinkable as a creature,
            // which is what separates him from every other flicker in white.
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
            )],
            abilities::exile_until_next_end_step(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
        ),
    ]),
);

// EVE 22 — Glen Elendra Archmage
pub(in crate::card::sets) static GLEN_ELENDRA_ARCHMAGE_22: CardRecord = CardRecord::new(
    "Glen Elendra Archmage",
    "09516d3d-e6c2-4359-af2a-a4aa244ca033",
    "Warren Mahy",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Faerie", "Wizard"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_with_targets(
            "{U}, Sacrifice this creature: Counter target noncreature spell.",
            &[CostDef::Mana(mana_cost!("{U}")), CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::NoncreatureSpell,
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::counter_target(TargetIndex::PRIMARY),
        ),
        abilities::persist(),
    ]),
);

// EVE 37 — Merrow Bonegnawer
pub(in crate::card::sets) static MERROW_BONEGNAWER_37: CardRecord = CardRecord::new(
    "Merrow Bonegnawer",
    "09e49aa4-ac23-49b1-b9b7-49d45b56b21d",
    "Jim Nelson",
    CardRules::new_creature(mana_cost!("{B}"), &["Merfolk", "Rogue"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: Target player exiles a card from their graveyard.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::ChooseExact(crate::card::ChooseExactDef {
                binding: Binding!("graveyard_exile"),
                chooser: crate::card::PlayerRefDef::Target(TargetIndex::PRIMARY),
                candidates: ObjectSetDef::Query(crate::card::ObjectQueryDef::owned_by(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Graveyard],
                    crate::card::PlayerSetDef::One(crate::card::PlayerRefDef::Target(
                        TargetIndex::PRIMARY,
                    )),
                )),
                exclude: None,
                amount: ValueDef::Constant(1),
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::move_to_zone(
                    EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("graveyard_exile"))),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
            }),
        ),
        AbilityDef::triggered(
            "Whenever you cast a black spell, you may untap this creature.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ObjectPredicateDef::Color(ManaColor::Black),
            ])),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Untap {
                    object: EffectRecipientDef::Source,
                },
            },
        ),
    ]),
);

// EVE 41 — Raven's Crime
pub(in crate::card::sets) static RAVEN_S_CRIME: CardRecord = CardRecord::new(
    "Raven's Crime",
    "7ced5797-5de0-43ca-9dc9-e48912333a70",
    "Warren Mahy",
    // Retrace turns every excess land into another discard, which is why a
    // land-heavy deck treats this one card as an engine.
    CardRules::new_sorcery(mana_cost!("{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target player discards a card.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Discard {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ),
        AbilityDef::alternative_cast(
            &[
                CostDef::ManaCostOf(crate::ObjectRefDef::Source),
                const { CostDef::discard(ObjectPredicateDef::HasType(CardType::Land)) },
            ],
            AlternativeCastKindDef::Retrace,
            Some(
                "Retrace (You may cast this card from your graveyard by discarding a land card \
                 in addition to paying its other costs.)",
            ),
            EffectDef::None,
        ), // Retrace's own cost: the card's mana cost again, plus a land out of
           // hand. Discarding is what an ordinary hand cost does, so nothing
           // else has to be said about how the land is spent.
    ]),
);

// EVE 66 — Bloom Tender
// Audit: unsupported — Needs conditional immediate mana production once per represented color in a single mana ability; nonmana IfCondition composition cannot resolve inside an immediate mana activation.
pub(in crate::card::sets) static BLOOM_TENDER: CardRecord = CardRecord::new(
    "Bloom Tender",
    "d7cc2828-dfe7-410b-9735-10bb7211f0f5",
    "Chippy",
    CardRules::unsupported(),
);

// EVE 67 — Duskdale Wurm
pub(in crate::card::sets) static DUSKDALE_WURM: CardRecord = CardRecord::new(
    "Duskdale Wurm",
    "8d10736d-047b-423f-9017-f59732d446bf",
    "Dan Dos Santos",
    CardRules::new_creature(mana_cost!("{5}{G}{G}"), &["Wurm"], 7, 7)
        .with_abilities(&[abilities::trample()]),
);

// EVE 82 — Beckon Apparition
pub(in crate::card::sets) static BECKON_APPARITION: CardRecord = CardRecord::new(
    "Beckon Apparition",
    "3bae1a3b-881b-4b10-ac5f-822c809edc36",
    "Larry MacDougall",
CardRules::new_instant(mana_cost!("{W/B}")).with_ability(
        AbilityDef::spell_with_targets(
            "Exile target card from a graveyard. Create a 1/1 white and black Spirit creature token with flying.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Any,
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: None,
            })],
            EffectDef::Sequence(&[
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::creature(&["Spirit"], &[ManaColor::White, ManaColor::Black], 1, 1)
                        .with_abilities(&[abilities::flying()])
                        .with_art(CardArt::new(
                            "91f3a4b0-0992-4245-b245-033ad1083a93",
                            "Cliff Childs",
                        )),
                ))),
            ]),
        ),
    ),
);

// EVE 94 — Restless Apparition
pub(in crate::card::sets) static RESTLESS_APPARITION_94: CardRecord = CardRecord::new(
    "Restless Apparition",
    "dc6480d0-17c5-4ac2-afb2-4d44f089de22",
    "Jeff Easley",
    CardRules::new_creature(mana_cost!("{W/B}{W/B}{W/B}"), &["Spirit"], 2, 2).with_abilities(&[
        AbilityDef::activated(
            "{W/B}{W/B}{W/B}: This creature gets +3/+3 until end of turn.",
            &[CostDef::Mana(mana_cost!("{W/B}{W/B}{W/B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(3),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::persist(),
    ]),
);

// EVE 119 — Desecrator Hag
// Audit: unsupported — Needs a value that aggregates over a query to be readable inside a query filter. "The creature card with the greatest power" is expressible as a choice among the cards nothing beats, but the maximum is an AggregateObjectValues over the graveyard and a predicate cannot evaluate one, so the comparison silently fails and its negation admits every creature card.
pub(in crate::card::sets) static DESECRATOR_HAG: CardRecord = CardRecord::new(
    "Desecrator Hag",
    "74d2e092-c805-447c-b784-1896b69524e0",
    "Fred Harper",
    crate::card::CardRules::unsupported(),
);

// EVE 139 — Figure of Destiny
pub(in crate::card::sets) static FIGURE_OF_DESTINY: CardRecord = CardRecord::new(
    "Figure of Destiny",
    "0da69523-cece-425a-b08a-fb27fac29374",
    "Scott M. Fischer",
// A one-drop that is never a dead draw: it is a 1/1 on turn one and an
    // 8/8 flier on turn six, and every point of mana in between goes into it.
    CardRules::new_creature(mana_cost!("{R/W}"), &["Kithkin"], 1, 1)
        .with_abilities(&[
            AbilityDef::activated(
                "{R/W}: This creature becomes a Kithkin Spirit with base power and toughness 2/2.",
                &[CostDef::Mana(mana_cost!("{R/W}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    // Each step repaints the whole creature-type line rather than adding to it,
                    // which is what "becomes a Kithkin Spirit Warrior" says: the types it lists
                    // are the types it has. None of them ends, so every one is permanent and
                    // the next step reads the one before it off the board.
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Kithkin", "Spirit"])),
                        AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
                    ]),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
            ),
            AbilityDef::activated(
                "{R/W}{R/W}{R/W}: If this creature is a Spirit, it becomes a Kithkin Spirit Warrior with \
                 base power and toughness 4/4.",
                &[CostDef::Mana(mana_cost!("{R/W}{R/W}{R/W}"))],
                EffectDef::IfCondition {
                    // "If this creature is a Spirit" is read as the ability resolves, so a
                    // Figure that was answered in response is a 1/1 again and the second
                    // activation does nothing.
                    condition: &TriggerConditionDef::SourceMatches {
                        object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spirit")),
                    },
                    then: &EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&[
                                "Kithkin", "Spirit", "Warrior",
                            ])),
                            AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(4), ValueDef::Constant(4)),
                        ]),
                        duration: ResolvedEffectDurationDef::Permanent,
                    },
                },
            ),
            AbilityDef::activated(
                "{R/W}{R/W}{R/W}{R/W}{R/W}{R/W}: If this creature is a Warrior, it becomes a Kithkin \
                 Spirit Warrior Avatar with base power and toughness 8/8, flying, and first strike.",
                &[CostDef::Mana(mana_cost!(
                    "{R/W}{R/W}{R/W}{R/W}{R/W}{R/W}"
                ))],
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceMatches {
                        object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Warrior")),
                    },
                    then: &EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&[
                                "Kithkin", "Spirit", "Warrior", "Avatar",
                            ])),
                            AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(8), ValueDef::Constant(8)),
                            AppliedEffectDef::add_ability(&abilities::flying()),
                            AppliedEffectDef::add_ability(&abilities::first_strike()),
                        ]),
                        duration: ResolvedEffectDurationDef::Permanent,
                    },
                },
            ),
        ]),
);

// EVE 156 — Murkfiend Liege
// Audit: unsupported — The untap-step rule cannot continuously add only matching green/blue controlled creatures to another player’s normal untap turn-based action.
pub(in crate::card::sets) static MURKFIEND_LIEGE_156: CardRecord = CardRecord::new(
    "Murkfiend Liege",
    "8d8250af-696f-4e28-86ba-29e316d01e56",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// EVE 175 — Cascade Bluffs
// Audit: unsupported — Mana-ability payment rejects hybrid symbols in activation costs; it cannot choose which half of the filter cost to pay.
pub(in crate::card::sets) static CASCADE_BLUFFS_175: CardRecord = CardRecord::new(
    "Cascade Bluffs",
    "c3eede44-270a-481d-850b-b4862b9685ea",
    "Brandon Kitkouski",
    crate::card::CardRules::unsupported(),
);

// EVE 176 — Fetid Heath
// Audit: unsupported — Mana-ability payment rejects hybrid symbols in activation costs; it cannot choose which half of the filter cost to pay.
pub(in crate::card::sets) static FETID_HEATH_176: CardRecord = CardRecord::new(
    "Fetid Heath",
    "0fbb9790-3744-4dcb-881a-452573298822",
    "Daarken",
    crate::card::CardRules::unsupported(),
);

// EVE 178 — Rugged Prairie
// Audit: unsupported — Mana-ability payment rejects hybrid symbols in activation costs; it cannot choose which half of the filter cost to pay.
pub(in crate::card::sets) static RUGGED_PRAIRIE_178: CardRecord = CardRecord::new(
    "Rugged Prairie",
    "e31f8b2a-acf4-423c-bc99-8cf44f3c018a",
    "Fred Fields",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ARCHON_OF_JUSTICE,
    &FLICKERWISP,
    &GLEN_ELENDRA_ARCHMAGE_22,
    &MERROW_BONEGNAWER_37,
    &RAVEN_S_CRIME,
    &BLOOM_TENDER,
    &DUSKDALE_WURM,
    &BECKON_APPARITION,
    &RESTLESS_APPARITION_94,
    &DESECRATOR_HAG,
    &FIGURE_OF_DESTINY,
    &MURKFIEND_LIEGE_156,
    &CASCADE_BLUFFS_175,
    &FETID_HEATH_176,
    &RUGGED_PRAIRIE_178,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
