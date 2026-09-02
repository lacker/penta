//! Duskmourn: House of Horror Commander cards cataloged for the Vintage Cube.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef, CardArt, CardRules,
    CardSet, CardType, CounterKind, EffectDef, EffectRecipientDef, ObjectPredicateDef,
    PlayerRelation, ResolvedEffectDurationDef, TokenCountersDef, TriggerEventDef, TurnStepDef,
    ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::{TargetIndex, mana_cost};

// DSC 21 — Metamorphosis Fanatic
pub(in crate::card::sets) static METAMORPHOSIS_FANATIC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("16448d95-ee21-4def-b880-26f6f159c213"),
    "Metamorphosis Fanatic",
    CardArt::new("16448d95-ee21-4def-b880-26f6f159c213", "Andreas Zafiratos"),
    CardSet::DuskmournHouseOfHorrorCommander,
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
pub(in crate::card::sets) static URSINE_MONSTROSITY: CardRecord = CardRecord::new_with_legacy_id(
    2195,
    "Ursine Monstrosity",
    CardArt::new("73cc6df4-3564-4ace-bf8a-eac3e62d725a", "Carlos Palma Cruchaga"),
    CardSet::DuskmournHouseOfHorrorCommander,
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

// DSC 88 — Growth Spiral
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GROWTH_SPIRAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("288ed3e9-4485-44ad-8561-efa09ed96f34"),
    "Growth Spiral",
    crate::card::CardArt::new("1e10e2b4-9639-41ae-8b8e-253224d3d513", "Nicholas Gregory"),
    crate::card::CardSet::DuskmournHouseOfHorrorCommander,
    crate::card::CardRules::unsupported(),
);

// DSC 270 — Dimir Aqueduct
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIMIR_AQUEDUCT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("df3c3d56-8291-407e-87a1-94b7d12811fd"),
    "Dimir Aqueduct",
    crate::card::CardArt::new("84bf9d60-64b8-4209-acfe-e07eefc6bf1f", "John Avon"),
    crate::card::CardSet::DuskmournHouseOfHorrorCommander,
    crate::card::CardRules::unsupported(),
);

// DSC 279 — Golgari Rot Farm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOLGARI_ROT_FARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("104364d5-ede8-4ac5-900f-19947f51bbc1"),
    "Golgari Rot Farm",
    crate::card::CardArt::new("725fab98-558b-4b0c-a0a4-ef0eec92eebb", "John Avon"),
    crate::card::CardSet::DuskmournHouseOfHorrorCommander,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &METAMORPHOSIS_FANATIC,
    &URSINE_MONSTROSITY,
    &GROWTH_SPIRAL,
    &DIMIR_AQUEDUCT,
    &GOLGARI_ROT_FARM,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
