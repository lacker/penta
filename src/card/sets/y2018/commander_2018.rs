//! Commander 2018 cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AddManaEffectDef, CardArt, CardRules, CardSet, CardType,
    ControlDurationDef, EffectDef, EffectRecipientDef, ObjectPredicateDef, PlayerRefDef,
    PlayerRelation, TriggerEventDef, ValueDef, abilities,
};
use crate::mana_cost;

// C18 54 — Coveted Jewel
pub(in crate::card::sets) static COVETED_JEWEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f83ed433-fae3-4fa5-acad-bb8a5b535ce3"),
    "Coveted Jewel",
    CardArt::new("f83ed433-fae3-4fa5-acad-bb8a5b535ce3", "Jason A. Engle"),
    CardSet::Commander2018,
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
            &[AbilityCostDef::TapSource],
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
                EffectDef::GainControl {
                    object: EffectRecipientDef::Source,
                    controller: PlayerRefDef::Opponent,
                    duration: ControlDurationDef::Indefinitely,
                },
                EffectDef::Untap {
                    object: EffectRecipientDef::Source,
                },
            ]),
        ),
    ]),
);

// C18 57 — Retrofitter Foundry
pub(in crate::card::sets) static RETROFITTER_FOUNDRY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5da578b8-19e6-4068-9336-e7cd33c585f1"),
    "Retrofitter Foundry",
    crate::card::CardArt::new("5da578b8-19e6-4068-9336-e7cd33c585f1", "Dmitry Burmak"),
    crate::card::CardSet::Commander2018,
    // One mana on turn one and a mana sink for the rest of the game, which
    // is why it is played in decks with no other artifacts at all.
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        // No tap in the cost, so the untap is what makes every other ability
        // usable more than once a turn -- and usable on their turn.
        AbilityDef::activated(
            "{3}: Untap this artifact.",
            &[AbilityCostDef::Mana(mana_cost!("{3}"))],
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
        AbilityDef::activated(
            "{2}, {T}: Create a 1/1 colorless Servo artifact creature token.",
            // The line the card is played for: with four mana up on their turn it
            // untaps and makes a Servo, and the Servo becomes a Thopter and the Thopter
            // a 4/4, one tap at a time.
            &[
                AbilityCostDef::Mana(mana_cost!("{2}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::create_artifact_creature_token(&["Servo"], &[], 1, 1),
        ),
        AbilityDef::activated(
            "{1}, {T}, Sacrifice a Servo: Create a 1/1 colorless Thopter artifact creature token \
             with flying.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificePermanent {
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
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::Subtype("Thopter"),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::create_artifact_creature_token(&["Construct"], &[], 4, 4),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&COVETED_JEWEL, &RETROFITTER_FOUNDRY];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
