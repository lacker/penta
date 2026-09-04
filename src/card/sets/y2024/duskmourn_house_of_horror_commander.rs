//! Duskmourn: House of Horror Commander cards cataloged for the Vintage Cube.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef, CardRules, CardSet,
    CardType, CounterKind, EffectDef, EffectRecipientDef, ObjectPredicateDef, PlayerRelation,
    ResolvedEffectDurationDef, TokenCountersDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind,
    ZonePlacement, abilities,
};
use crate::{TargetIndex, mana_cost};

// DSC 21 — Metamorphosis Fanatic
pub(in crate::card::sets) static METAMORPHOSIS_FANATIC: CardRecord = CardRecord::new(
    CardSet::DuskmournHouseOfHorrorCommander,
    "Metamorphosis Fanatic",
    "16448d95-ee21-4def-b880-26f6f159c213",
    "Andreas Zafiratos",
    // Six mana for a 4/4 that reanimates is a fair rate and nothing more.
    // Two mana for it off the top of your library is what puts the card in
    // a cube -- and the body it brings back is the half that wins games.
    CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Human", "Cleric"], 4, 4).with_abilities(&[
        abilities::lifelink(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, return up to one target creature card from your graveyard \
             to the battlefield with a lifelink counter on it.",
            // "Up to one target creature card from your graveyard." Your own, so this
            // never reaches across the table the way an opponent-facing reanimator
            // would, and choosing none is already a legal declaration.
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
                1,
            )],
            EffectDef::WithBattlefieldArrival {
                effect: &EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                },
                arrival: crate::card::BattlefieldArrivalDef {
                    // A counter rather than a granted keyword (CR 122.1b): what comes back has
                    // lifelink for exactly as long as the counter is on it, which outlives
                    // every duration a spell could have named.
                    counters: Some(TokenCountersDef {
                        kind: CounterKind::Lifelink,
                        amount: ValueDef::Constant(1),
                    }),
                    ..crate::card::BattlefieldArrivalDef::DEFAULT
                },
            },
        ),
        abilities::miracle(mana_cost!("{1}{B}")),
    ]),
);

// DSC 36 — Ursine Monstrosity
pub(in crate::card::sets) static URSINE_MONSTROSITY: CardRecord = CardRecord::new(
    CardSet::DuskmournHouseOfHorrorCommander,
    "Ursine Monstrosity",
    "73cc6df4-3564-4ace-bf8a-eac3e62d725a",
    "Carlos Palma Cruchaga",
    // The bear feeds itself: every combat mills one more card, and every
    // card type that turns up is another point in both directions.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Bear", "Mutant"], 3, 3).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered(
            "At the beginning of combat on your turn, mill a card and choose an opponent at random. This creature attacks that player this combat if able. Until end of turn, this creature gains indestructible and gets +1/+1 for each card type among cards in your graveyard.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            EffectDef::Sequence(&[
                EffectDef::Mill {
                    player: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_ability(&abilities::indestructible()),
                        // "This creature attacks that player this combat if able." In a two-player
                        // game the chosen opponent is the only player there is to attack, so what
                        // the requirement names is that seat; a planeswalker they control does not
                        // satisfy it, which is what separates this from the plain "attacks each
                        // combat if able". It is granted for the turn rather than printed, and the
                        // trigger renews it at the beginning of every combat.
                        AppliedEffectDef::add_ability(&abilities::attacks_player_each_combat_if_able(
                            "This creature attacks that player this combat if able.",
                        )),
                        // Read as the trigger resolves, which is after the mill: the card it
                        // just put there counts toward its own bonus.
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::CardTypesAmongGraveyards(PlayerRelation::You),
                            ValueDef::CardTypesAmongGraveyards(PlayerRelation::You),
                        ),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ]),
);

// DSC 88 — Growth Spiral (reprint)
const GROWTH_SPIRAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2019::ravnica_allegiance::GROWTH_SPIRAL,
    "1e10e2b4-9639-41ae-8b8e-253224d3d513",
    "Nicholas Gregory",
);

// DSC 270 — Dimir Aqueduct (reprint)
const DIMIR_AQUEDUCT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2005::ravnica_city_of_guilds::DIMIR_AQUEDUCT,
    "84bf9d60-64b8-4209-acfe-e07eefc6bf1f",
    "John Avon",
);

// DSC 279 — Golgari Rot Farm (reprint)
const GOLGARI_ROT_FARM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2005::ravnica_city_of_guilds::GOLGARI_ROT_FARM,
    "725fab98-558b-4b0c-a0a4-ef0eec92eebb",
    "John Avon",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&METAMORPHOSIS_FANATIC, &URSINE_MONSTROSITY];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    GROWTH_SPIRAL_REPRINT,
    DIMIR_AQUEDUCT_REPRINT,
    GOLGARI_ROT_FARM_REPRINT,
];
