//! Amonkhet cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, ActivationTimingDef,
    CardArt, CardRules, CardSet, CardType, ConditionalValueDef, EffectDef, EffectRecipientDef,
    ManaColor, ObjectPredicateDef, PlayerRelation, TriggerEventDef, ValueDef, ZoneKind, abilities,
};
use crate::{TargetIndex, mana_cost};

// AKH 75 — Vizier of Tumbling Sands
pub(in crate::card::sets) static VIZIER_OF_TUMBLING_SANDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ce4ff0f5-abee-4f3e-89ae-1b7ee771ec68"),
    "Vizier of Tumbling Sands",
    CardArt::new("ce4ff0f5-abee-4f3e-89ae-1b7ee771ec68", "Josu Hernaiz"),
    CardSet::Amonkhet,
    // Two ways to untap something out of one card: the body unlocks a land
    // every turn, and cycling unlocks one the turn you gave up on the body.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Cleric"], 1, 3).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: Untap another target permanent.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
            )],
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
        abilities::cycling(
            "Cycling {1}{U} ({1}{U}, Discard this card: Draw a card.)",
            mana_cost!("{1}{U}"),
        ),
        AbilityDef::triggered_with_targets(
            "When you cycle this card, untap target permanent.",
            TriggerEventDef::Cycled,
            // No "another" here: the Vizier is in the graveyard by now, so
            // any permanent is a legal choice.
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Any,
            )],
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// AKH 81 — Bone Picker
pub(in crate::card::sets) static BONE_PICKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bdc6a825-43f7-40a4-95f0-335dc538b6cd"),
    "Bone Picker",
    CardArt::new("bdc6a825-43f7-40a4-95f0-335dc538b6cd", "Yeong-Hao Han"),
    CardSet::Amonkhet,
    // A one-mana flying deathtouch blocker after any trade, which is why the
    // full four mana is a price the card almost never pays.
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Bird"], 3, 2).with_abilities(&[
        AbilityDef::static_ability(
            "This spell costs {3} less to cast if a creature died this turn.",
            EffectDef::ReduceGenericCostBy(ValueDef::IfCreatureDiedThisTurn(
                &const {
                    ConditionalValueDef {
                        then: ValueDef::Constant(3),
                        otherwise: ValueDef::Constant(0),
                    }
                },
            )),
        )
        // Read while the card is in hand: this prices the spell, so it has
        // to apply from the zone the spell is cast out of.
        .with_source_zones(&[ZoneKind::Hand]),
        abilities::flying(),
        abilities::deathtouch(),
    ]),
);

// AKH 134 — Glorybringer
pub(in crate::card::sets) static GLORYBRINGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3277ad99-5682-4baa-b106-de15721876a6"),
    "Glorybringer",
    CardArt::new("3277ad99-5682-4baa-b106-de15721876a6", "Sam Burley"),
    CardSet::Amonkhet,
    // Five mana that attacks the turn it lands for four in the air and kills
    // something on the way in. What exerting costs is the next attack, which
    // is the only thing keeping it honest.
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Dragon"], 4, 4).with_abilities(&[
        abilities::flying(),
        abilities::haste(),
        AbilityDef::triggered_with_targets(
            "You may exert this creature as it attacks. When you do, it deals 4 damage to target \
             non-Dragon creature an opponent controls.",
            TriggerEventDef::Exerted(ObjectPredicateDef::Source),
            // "Target non-Dragon creature an opponent controls." The exclusion is why
            // the card does not simply answer another Glorybringer, which is the whole
            // reason it is printed that way.
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Dragon")),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(4),
            },
        ),
    ]),
);

// AKH 241 — Cradle of the Accursed
pub(in crate::card::sets) static CRADLE_OF_THE_ACCURSED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("41713e82-c3d3-4c2f-b075-f684cbd68ce8"),
    "Cradle of the Accursed",
    CardArt::new("41713e82-c3d3-4c2f-b075-f684cbd68ce8", "Noah Bradley"),
    CardSet::Amonkhet,
    // Untapped and colourless, so the body it eventually becomes costs the
    // deck nothing but the land slot.
    CardRules::new_land(&["Desert"]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{3}, {T}, Sacrifice this land: Create a 2/2 black Zombie creature token. Activate only as a sorcery.",
            &[
                AbilityCostDef::Mana(mana_cost!("{3}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
            ],
            EffectDef::create_creature_token(&["Zombie"], &[ManaColor::Black], 2, 2),
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &VIZIER_OF_TUMBLING_SANDS,
    &BONE_PICKER,
    &GLORYBRINGER,
    &CRADLE_OF_THE_ACCURSED,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
