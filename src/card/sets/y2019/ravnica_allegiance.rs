//! RNA card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AlternativeCastKindDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardChoiceSourceDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::CostQuantityDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::InstalledTriggerDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::PayOrDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::ScaledValueDef;
use crate::card::SubtypeDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

static SPHINX_OPENING_TRIGGER: AbilityDef = AbilityDef::triggered(
    "At the beginning of your first upkeep, scry 3.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    abilities::scry(ValueDef::Constant(3)),
);

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "RNA",
    slug: "ravnica-allegiance",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// RNA 3 — Archway Angel
pub(in crate::card::sets) static ARCHWAY_ANGEL: CardRecord = CardRecord::new(
    "Archway Angel",
    "b209d219-b946-4226-a8b4-65a5f3837fac",
    "Milivoj Ćeran",
    CardRules::new_creature(mana_cost!("{5}{W}"), &["Angel"], 3, 4).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, you gain 2 life for each Gate you \
             control.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Scaled(&ScaledValueDef {
                    value: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Gate")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    factor: 2,
                }),
            },
        ),
    ]),
);

// RNA 22 — Smothering Tithe
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SMOTHERING_TITHE_22: CardRecord = CardRecord::new(
    "Smothering Tithe",
    "7af082fa-86a3-4f7b-966d-2be1f1d0c0bc",
    "Mark Behm",
    crate::card::CardRules::unsupported(),
);

// RNA 40 — Gateway Sneak
pub(in crate::card::sets) static GATEWAY_SNEAK: CardRecord = CardRecord::new(
    "Gateway Sneak",
    "edc0229d-05e6-41b7-b7a9-2a8b2b258add",
    "Matt Stewart",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Vedalken", "Rogue"], 1, 3).with_abilities(&[
        AbilityDef::triggered(
            "Whenever a Gate you control enters, this creature can't be \
             blocked this turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Gate")),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, draw \
             a card.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

// RNA 55 — Sphinx of Foresight
pub(in crate::card::sets) static SPHINX_OF_FORESIGHT: CardRecord = CardRecord::new(
    "Sphinx of Foresight",
    "cf2386fd-edc0-4731-8f4e-7a7c45548bf3",
    "Titus Lunter",
CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Sphinx"], 4, 4).with_abilities(&[
        AbilityDef::opening_hand_reveal(
            "You may reveal this card from your opening hand. If you do, scry 3 at the beginning of your first upkeep.",
            EffectDef::InstallTrigger(InstalledTriggerDef::once(&SPHINX_OPENING_TRIGGER)),
        ),
        abilities::flying(),
        AbilityDef::triggered(
            "At the beginning of your upkeep, scry 1.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            abilities::scry(ValueDef::Constant(1)),
        ),
    ]),
);

// RNA 107 — Light Up the Stage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIGHT_UP_THE_STAGE_107: CardRecord = CardRecord::new(
    "Light Up the Stage",
    "9287b848-2aeb-4c70-ac4a-acafb871b7a4",
    "Dmitry Burmak",
    crate::card::CardRules::unsupported(),
);

// RNA 115 — Skewer the Critics
pub(in crate::card::sets) static SKEWER_THE_CRITICS: CardRecord = CardRecord::new(
    "Skewer the Critics",
    "97295660-6bea-46ae-9a3b-0fc6abba407f",
    "Heonhwa",
    // A one-mana Lava Spike in the deck that was already attacking, and a
    // dead card in the deck that was not. Nothing about the spell changes
    // when spectacle pays for it; only the price does.
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Skewer the Critics deals 3 damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(3),
            ),
        ),
        // Spectacle (CR 702.137a) is an alternative cost gated on a board
        // condition, which is the same shape Mogg Salvage's free cast has.
        // "Lost life", not "was dealt damage": a Thoughtseize or a painland
        // turns it on just as well as an attack.
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{R}"))],
            AlternativeCastKindDef::AlternativeCost,
            Some(
                "Spectacle {R} (You may cast this spell for its spectacle cost rather than its \
                 mana cost if an opponent lost life this turn.)",
            ),
            EffectDef::None,
        )
        .with_alternative_condition(&TriggerConditionDef::OpponentLostLifeThisTurn),
    ]),
);

// RNA 123 — Biogenic Upgrade
// Audit: unsupported — Needs counter-placement resolution to read each target's assigned share from a divided target slot; DividedAmongTargets is implemented by damage resolution but not AddCounters.
pub(in crate::card::sets) static BIOGENIC_UPGRADE: CardRecord = CardRecord::new(
    "Biogenic Upgrade",
    "0dd73fb2-453f-40b9-8beb-dfa99e6a706e",
    "Tomasz Jedruszek",
    CardRules::unsupported(),
);

// RNA 131 — Incubation Druid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INCUBATION_DRUID_131: CardRecord = CardRecord::new(
    "Incubation Druid",
    "075bbe5d-d0f3-4be3-a3a6-072d5d3d614c",
    "Daniel Ljunggren",
    crate::card::CardRules::unsupported(),
);

// RNA 139 — Saruli Caretaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SARULI_CARETAKER_139: CardRecord = CardRecord::new(
    "Saruli Caretaker",
    "ef3358cb-714c-49bf-b7e9-a69d02d7799e",
    "Howard Lyon",
    crate::card::CardRules::unsupported(),
);

// RNA 158 — Biomancer's Familiar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BIOMANCER_S_FAMILIAR_158: CardRecord = CardRecord::new(
    "Biomancer's Familiar",
    "d38c9891-36d1-4565-9c4a-1cd9dbf8c048",
    "Izzy",
    crate::card::CardRules::unsupported(),
);

// RNA 161 — Cindervines
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CINDERVINES_161: CardRecord = CardRecord::new(
    "Cindervines",
    "9f970f79-3051-4ba1-badb-697ef321cbb3",
    "Mark Behm",
    crate::card::CardRules::unsupported(),
);

// RNA 171 — Final Payment
pub(in crate::card::sets) static FINAL_PAYMENT: CardRecord = CardRecord::new(
    "Final Payment",
    "49a21a8f-9c7b-4ae8-8635-f2ee2151c8de",
    "Victor Adame Minguez",
    CardRules::new_instant(mana_cost!("{W}{B}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, pay 5 life or sacrifice a creature or \
             enchantment.\nDestroy target creature.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            CostDef::choice(&[
                CostDef::pay_life(CostQuantityDef::Fixed(5)),
                CostDef::sacrifice(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                    ]),
                    CostQuantityDef::Fixed(1),
                ),
            ]),
            EffectDef::destroy_target(crate::TargetIndex::PRIMARY),
        ),
    ),
);

// RNA 172 — Fireblade Artist
pub(in crate::card::sets) static FIREBLADE_ARTIST: CardRecord = CardRecord::new(
    "Fireblade Artist",
    "21e1161f-bd2c-45a7-a86b-3b2e5210f148",
    "Steve Argyle",
    // Two damage every upkeep for a spare creature, and haste means the
    // Artist itself can be the first thing fed to it after it attacks.
    CardRules::new_creature(mana_cost!("{B}{R}"), &["Human", "Shaman"], 2, 2).with_abilities(&[
        abilities::haste(),
        AbilityDef::triggered(
            "At the beginning of your upkeep, you may sacrifice a creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::sacrifice_permanent(ObjectPredicateDef::HasType(
                    CardType::Creature,
                ))],
                &EffectDef::None,
            )),
        ),
        AbilityDef::triggered_with_targets(
            "When you do, this creature deals 2 damage to target opponent or planeswalker.",
            // The reflexive half: it goes on the stack by itself once the
            // sacrifice is taken, and names its target only then.
            TriggerEventDef::OptionalEffectTaken(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Opponent),
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(2),
            ),
        ),
    ]),
);

// RNA 178 — Growth Spiral
pub(in crate::card::sets) static GROWTH_SPIRAL: CardRecord = CardRecord::new(
    "Growth Spiral",
    "7c77a6b1-ef06-4da5-8e86-a5204216cb77",
    "Seb McKinnon",
    // Ramping at instant speed is the point: the land drop it hands out is
    // extra, so this is a cantrip on a turn where the land would rot in hand.
    CardRules::new_instant(mana_cost!("{G}{U}")).with_ability(AbilityDef::spell(
        "Draw a card. You may put a land card from your hand onto the battlefield.",
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
            // "You may put": the card drawn can itself be the land, and
            // declining matters when the only land in hand is one you would
            // rather keep for a real land drop.
            EffectDef::ChooseCards {
                player: EffectRecipientDef::Controller,
                sources: &[CardChoiceSourceDef::Zone(ZoneKind::Hand)],
                object: ObjectPredicateDef::HasType(CardType::Land),
                minimum: 0,
                maximum: 1,
                reveal: false,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
            },
        ]),
    )),
);

// RNA 189 — Lavinia, Azorius Renegade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAVINIA_AZORIUS_RENEGADE_189: CardRecord = CardRecord::new(
    "Lavinia, Azorius Renegade",
    "c497d496-1232-4614-93b0-9864fa93c29f",
    "Steven Belledin",
    crate::card::CardRules::unsupported(),
);

// RNA 195 — Prime Speaker Vannifar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRIME_SPEAKER_VANNIFAR_195: CardRecord = CardRecord::new(
    "Prime Speaker Vannifar",
    "84abfc59-10a7-4cb5-9cdd-81797116c810",
    "Kieran Yanner",
    crate::card::CardRules::unsupported(),
);

// RNA 201 — Rhythm of the Wild
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RHYTHM_OF_THE_WILD_201: CardRecord = CardRecord::new(
    "Rhythm of the Wild",
    "84062ce2-fea2-4e06-b83b-7cc597fb2a1b",
    "Tomasz Jedruszek",
    crate::card::CardRules::unsupported(),
);

// RNA 232 — Gate Colossus
pub(in crate::card::sets) static GATE_COLOSSUS: CardRecord = CardRecord::new(
    "Gate Colossus",
    "99767e2f-a558-4d63-b9b6-923d15b433e1",
    "Izzy",
    CardRules::new_artifact_creature(mana_cost!("{8}"), &["Construct"], 8, 8).with_abilities(&[
        AbilityDef::static_ability(
            "Affinity for Gates (This spell costs {1} less to cast for \
             each Gate you control.)",
            EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(
                &ObjectQueryDef::matching(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Gate")),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
            )),
        )
        .with_source_zones(&[ZoneKind::Hand]),
        AbilityDef::static_ability(
            "This creature can't be blocked by creatures with power 2 or less.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::PowerLessThan(ValueDef::Constant(3)),
                )),
            },
        ),
        AbilityDef::triggered(
            "Whenever a Gate you control enters, you may put this card \
             from your graveyard on top of your library.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Gate")),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::move_to_zone(
                    EffectRecipientDef::Source,
                    ZoneKind::Library,
                    ZonePlacement::Top,
                ),
            },
        )
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ARCHWAY_ANGEL,
    &SMOTHERING_TITHE_22,
    &GATEWAY_SNEAK,
    &SPHINX_OF_FORESIGHT,
    &LIGHT_UP_THE_STAGE_107,
    &SKEWER_THE_CRITICS,
    &BIOGENIC_UPGRADE,
    &INCUBATION_DRUID_131,
    &SARULI_CARETAKER_139,
    &BIOMANCER_S_FAMILIAR_158,
    &CINDERVINES_161,
    &FINAL_PAYMENT,
    &FIREBLADE_ARTIST,
    &GROWTH_SPIRAL,
    &LAVINIA_AZORIUS_RENEGADE_189,
    &PRIME_SPEAKER_VANNIFAR_195,
    &RHYTHM_OF_THE_WILD_201,
    &GATE_COLOSSUS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
