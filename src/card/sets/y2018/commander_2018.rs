//! Commander 2018 cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AddManaEffectDef;
use crate::card::CardNameSetDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::ComparisonDef;
use crate::card::ControlDurationDef;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectSetCountConditionDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectSetFilterDef;
use crate::card::ObjectSetPredicateDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::mana_cost;

// C18 54 — Coveted Jewel
pub(in crate::card::sets) static COVETED_JEWEL: CardRecord = CardRecord::new(
    "Coveted Jewel",
    "f83ed433-fae3-4fa5-acad-bb8a5b535ce3",
    "Jason A. Engle",
// Six mana for three cards and a Gilded Lotus, held only as long as you
    // can stop them getting through -- and they untap it on the way out.
    CardRules::new_artifact(mana_cost!("{6}")).with_abilities(&[
        abilities::enters_trigger(
            "When this artifact enters, draw three cards.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add three mana of any one color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color().with_amount(3)),
        ),
        AbilityDef::triggered(
            "Whenever one or more creatures an opponent controls attack you and aren't blocked, that \
             player draws three cards and gains control of this artifact. Untap it.",
            // "One or more creatures an opponent controls attack you and aren't
            // blocked": one trigger for the whole unblocked crew, not one apiece, and
            // only for an attack aimed at you rather than at something you control.
            TriggerEventDef::UnblockedAttackersDeclared {
                attacker: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                ]),
                defender: PlayerRelation::You,
            },
            // The reward, the theft, and the untap, in printed order: the attacker has
            // the cards before the artifact changes hands, and it arrives ready to be
            // tapped again on their own turn.
            EffectDef::Sequence(&[
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(3),
                },
                EffectDef::gain_control(
                    EffectRecipientDef::Source,
                    PlayerRefDef::Opponent,
                    ControlDurationDef::Indefinitely,
                ),
                EffectDef::Untap {
                    object: EffectRecipientDef::Source,
                },
            ]),
        ),
    ]),
);

// C18 55 — Endless Atlas
pub(in crate::card::sets) static ENDLESS_ATLAS: CardRecord = CardRecord::new(
    "Endless Atlas",
    "d2547a42-b2b9-4887-8671-4cf63a7b0eff",
    "Titus Lunter",
CardRules::new_artifact(mana_cost!("{2}")).with_ability(
        AbilityDef::activated(
            "{2}, {T}: Draw a card. Activate only if you control three or more lands with the same name.",
            &[
                CostDef::Mana(mana_cost!("{2}")),
                CostDef::TapSource,
            ],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        )
        .with_activation_condition(&TriggerConditionDef::ObjectSetCount(
            &ObjectSetCountConditionDef {
                objects: &ObjectSetDef::Matching {
                    objects: &ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::NameIn(&
                        CardNameSetDef::NamesAppearingAtLeast {
                            objects: &ObjectSetDef::Query(ObjectQueryDef::matching(
                                ObjectPredicateDef::HasType(CardType::Land),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            )),
                            count: 3,
                        },
                    )),
                },
                predicate: ObjectSetPredicateDef {
                    filter: None,
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
            },
        )),
    ),
);

// C18 57 — Retrofitter Foundry
pub(in crate::card::sets) static RETROFITTER_FOUNDRY: CardRecord = CardRecord::new(
    "Retrofitter Foundry",
    "5da578b8-19e6-4068-9336-e7cd33c585f1",
    "Dmitry Burmak",
    // One mana on turn one and a mana sink for the rest of the game, which
    // is why it is played in decks with no other artifacts at all.
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        // No tap in the cost, so the untap is what makes every other ability
        // usable more than once a turn -- and usable on their turn.
        AbilityDef::activated(
            "{3}: Untap this artifact.",
            &[CostDef::Mana(mana_cost!("{3}"))],
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
        AbilityDef::activated(
            "{2}, {T}: Create a 1/1 colorless Servo artifact creature token.",
            // The line the card is played for: with four mana up on their turn it
            // untaps and makes a Servo, and the Servo becomes a Thopter and the Thopter
            // a 4/4, one tap at a time.
            &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
            EffectDef::create_artifact_creature_token(&["Servo"], &[], 1, 1),
        ),
        AbilityDef::activated(
            "{1}, {T}, Sacrifice a Servo: Create a 1/1 colorless Thopter artifact creature token \
             with flying.",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::TapSource,
                CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::Subtype("Servo"),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::create_artifact_creature_token(&["Thopter"], &[], 1, 1)
                .with_abilities(&[abilities::flying()]),
        ),
        AbilityDef::activated(
            "{T}, Sacrifice a Thopter: Create a 4/4 colorless Construct artifact creature token.",
            &[
                CostDef::TapSource,
                CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::Subtype("Thopter"),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::create_artifact_creature_token(&["Construct"], &[], 4, 4),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&COVETED_JEWEL, &ENDLESS_ATLAS, &RETROFITTER_FOUNDRY];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
