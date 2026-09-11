//! The Big Score cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::CopyExceptionsDef;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::mana_cost;

static ANY_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::AnyTarget,
)];

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "BIG",
    slug: "the-big-score",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// BIG 9 — Harvester of Misery
pub(in crate::card::sets) static HARVESTER_OF_MISERY: CardRecord = CardRecord::new(
    "Harvester of Misery",
    "a3012af9-621d-4fae-b00d-079a89ae35fe",
    "Jorge Jacinto",
    // Five mana for a board sweep on a hard-to-block body, or two mana from
    // the hand for one creature when the board does not need sweeping.
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Spirit"], 5, 4).with_abilities(&[
        abilities::menace(),
        abilities::enters_trigger(
            "When this creature enters, other creatures get -2/-2 until end of turn.",
            // "Other creatures": everyone's, and not the Spirit itself, which is what
            // lets a 5/4 sweep a board of two-toughness creatures and survive it.
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-2),
                    ValueDef::Constant(-2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated_with_targets(
            "{1}{B}, Discard this card: Target creature gets -2/-2 until end of turn.",
            &[CostDef::Mana(mana_cost!("{1}{B}")), CostDef::DiscardSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            // The same shrink, aimed at one creature. The card is discarded to pay for
            // it, so this is what the Spirit does on the turns five mana is too much.
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-2),
                    ValueDef::Constant(-2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .with_source_zones(&[ZoneKind::Hand]),
    ]),
);

// BIG 12 — Legion Extruder
pub(in crate::card::sets) static LEGION_EXTRUDER: CardRecord = CardRecord::new(
    "Legion Extruder",
    "5a077de0-1893-40d0-a499-ee2e6e2258f1",
    "Anton Solovianchyk",
    // Two mana that answers a creature on the way in and then turns every
    // spent artifact -- a cracked Lotus Petal, an emptied Bauble -- into a
    // 3/3, which is what the cube's artifact decks have lying around.
    CardRules::new_artifact(mana_cost!("{1}{R}")).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this artifact enters, it deals 2 damage to any target.",
            &ANY_TARGET,
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(2),
            ),
        ),
        AbilityDef::activated(
            "{2}, {T}, Sacrifice another artifact: Create a 3/3 colorless Golem artifact creature \
             token.",
            &[
                CostDef::Mana(mana_cost!("{2}")),
                CostDef::TapSource,
                CostDef::SacrificePermanent {
                    // Another one: the Extruder is an artifact itself and may not eat itself,
                    // which is what stops a two-mana artifact from being a Golem on its own.
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::create_artifact_creature_token(&["Golem"], &[], 3, 3).with_art(
                CardArt::new("406e2960-f560-48bb-b4a6-4bd35889a8f8", "Brian Valeza"),
            ),
        ),
    ]),
);

// BIG 21 — Loot, the Pathfinder
pub(in crate::card::sets) static LOOT_THE_PATHFINDER: CardRecord = CardRecord::new(
    "Loot, the Pathfinder",
    "fb169fa2-c92e-45f7-89a2-0ca0e3910a1c",
    "Rudy Siswanto",
// Five mana for a hasty double striker that also unloads three cards,
    // three mana, or three damage -- once each, and never twice, because
    // every one of them taps it.
    CardRules::new_creature(mana_cost!("{2}{G}{U}{R}"), &["Beast", "Noble"], 2, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::double_strike(),
            abilities::vigilance(),
            abilities::haste(),
            AbilityDef::activated_mana(
                "Exhaust — {G}, {T}: Add three mana of any one color. (Activate each exhaust ability \
                 only once.)",
                &[
                    CostDef::Mana(mana_cost!("{G}")),
                    CostDef::TapSource,
                ],
                EffectDef::AddMana(AddManaEffectDef::any_color().with_amount(3)),
            )
            .exhausting(),
            AbilityDef::activated(
                "Exhaust — {U}, {T}: Draw three cards.",
                &[
                    CostDef::Mana(mana_cost!("{U}")),
                    CostDef::TapSource,
                ],
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
            )
            .exhausting(),
            AbilityDef::activated_with_targets(
                "Exhaust — {R}, {T}: This creature deals 3 damage to any target.",
                &[
                    CostDef::Mana(mana_cost!("{R}")),
                    CostDef::TapSource,
                ],
                &ANY_TARGET,
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(3),
                ),
            )
            .exhausting(),
        ]),
);

// BIG 41 — Generous Plunderer
pub(in crate::card::sets) static GENEROUS_PLUNDERER: CardRecord = CardRecord::new(
    "Generous Plunderer",
    "351eea06-f5be-4044-b3b3-cc6bf805abb1",
    "Josiah \"Jo\" Cameron",
// Two mana for a 2/2 that hands the other player a Treasure every
    // upkeep and then bills them for it on the attack.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Rogue"], 2, 2)
        .with_abilities(&[
            abilities::menace(),
            // Nobody is named here: "you may create a Treasure token" is all this
            // half does, and declining it ends the matter.
            AbilityDef::triggered(
                "At the beginning of your upkeep, you may create a Treasure token.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    // Yours is untapped, so the Treasure you keep is usable this turn.
                    effect: &EffectDef::create_token(crate::card::tokens::treasure()),
                },
            ),
            // "When you do": a reflexive trigger, which is why the opponent is named
            // only once the Treasure exists, and why either player may respond to
            // the gift without touching the Treasure that prompted it.
            AbilityDef::triggered_with_targets(
                "When you do, target opponent creates a tapped Treasure token.",
                TriggerEventDef::OptionalEffectTaken(ObjectPredicateDef::Source),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::create_token(crate::card::tokens::treasure())
                    .with_controller(PlayerRefDef::Target(TargetIndex::PRIMARY))
                    .entering_tapped(),
            ),
            // "Defending player" is the opponent in a two-player game, whether the
            // attack is aimed at them or at something they control.
            AbilityDef::triggered(
                "Whenever this creature attacks, it deals damage to defending player equal to the number \
                 of artifacts they control.",
                TriggerEventDef::attack_declared(ObjectPredicateDef::Source, 1, None),
                EffectDef::damage(
                    EffectRecipientDef::Opponent, // Artifacts they control as the trigger resolves, which is what makes the
                    // Treasure handed over on the upkeep into damage on the attack.
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Opponent,
                    )),
                ),
            ),
        ]),
);

// BIG 85 — Vaultborn Tyrant
pub(in crate::card::sets) static VAULTBORN_TYRANT: CardRecord = CardRecord::new(
    "Vaultborn Tyrant",
    "07ca436a-e992-40a9-978a-501a82e443ed",
    "Loïc Canavaggia",
// Seven mana that draws a card the moment it lands, and killing it hands
    // the same body back once.
    CardRules::new_creature(mana_cost!("{5}{G}{G}"), &["Dinosaur"], 6, 6)
        .with_abilities(&[
            abilities::trample(),
            AbilityDef::triggered(
                "Whenever this creature or another creature you control with power 4 or greater enters, \
                 you gain 3 life and draw a card.",
                TriggerEventDef::zone_changed(
                    // "This creature or another creature you control with power 4 or greater":
                    // one predicate covers both halves, because the Tyrant is a 6/6 and so
                    // matches the size clause itself.
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ObjectPredicateDef::PowerAtLeast(4),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::Sequence(&[
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(3),
                    },
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
            AbilityDef::triggered_if(
                "When this creature dies, if it's not a token, create a token that's a copy of it, \
                 except it's an artifact in addition to its other types.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                // "If it's not a token", read off the creature that died rather than off
                // the card in the graveyard: without it every copy would make another copy
                // and the Tyrant would never stay dead.
                &TriggerConditionDef::SourceMatches {
                    object: ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                },
                // The copy is of the creature as it last existed on the battlefield
                // (CR 608.2h), which is why a Tyrant that grew before it died comes back
                // the size it was.
                EffectDef::create_token_from_copy(&crate::card::TokenCopyDef {
                        object: &EffectRecipientDef::Source,
                        exceptions: CopyExceptionsDef::NONE
                            .with_added_types(CardTypeSet::single(CardType::Artifact)),
                    }),
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &HARVESTER_OF_MISERY,
    &LEGION_EXTRUDER,
    &LOOT_THE_PATHFINDER,
    &GENEROUS_PLUNDERER,
    &VAULTBORN_TYRANT,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
