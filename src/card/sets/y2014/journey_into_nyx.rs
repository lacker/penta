//! Journey into Nyx cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AddManaEffectDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::PlayerRelation;
use crate::card::ReplacementEffectDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "JOU",
    slug: "journey-into-nyx",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// JOU 5 — Banishing Light
// Audit: unsupported — Needs an exile-until-source-leaves duration with immediate return when that duration ends (CR 610.3); an ordinary leaves trigger returns the card later through the stack.
pub(in crate::card::sets) static BANISHING_LIGHT: CardRecord = CardRecord::new(
    "Banishing Light",
    "fbaa4800-30cc-4a80-a6cc-9a24ada9eb40",
    "Willian Murai",
    CardRules::unsupported(),
);

// JOU 37 — Dictate of Kruphix
pub(in crate::card::sets) static DICTATE_OF_KRUPHIX: CardRecord = CardRecord::new(
    "Dictate of Kruphix",
    "e8e7916c-f39a-48a0-a47d-7e83ebf028fa",
    "Daarken",
    CardRules::new_enchantment(mana_cost!("{1}{U}{U}")).with_abilities(&[
        abilities::flash(),
        AbilityDef::triggered(
            "At the beginning of each player's draw step, that player \
             draws an additional card.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Draw,
                player: PlayerRelation::Any,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::EventPlayer,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// JOU 126 — Heroes' Bane
pub(in crate::card::sets) static HEROES_BANE: CardRecord = CardRecord::new(
    "Heroes' Bane",
    "380e82f6-30ee-49c5-b5a2-85c5cf3e9bb1",
    "Raymond Swanland",
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Hydra"], 0, 0).with_abilities(&[
        AbilityDef::as_enters(
            "This permanent enters with counters.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 4,
                },
            ),
        ),
        AbilityDef::activated(
            "{2}{G}{G}: Put X +1/+1 counters on this creature, where X is \
             its power.",
            &[CostDef::Mana(mana_cost!("{2}{G}{G}"))],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::SourcePower,
            },
        ),
    ]),
);

// JOU 163 — Mana Confluence
pub(in crate::card::sets) static MANA_CONFLUENCE: CardRecord = CardRecord::new(
    "Mana Confluence",
    "504a69eb-3c2d-4bb1-b117-252b15acf0c2",
    "Richard Wright",
    // City of Brass charges its life when it becomes tapped, by anyone and
    // for any reason. This charges it as a cost of its own ability, so a land
    // tapped by someone else costs nothing and an activation with no life to
    // spare is simply not offered.
    CardRules::new_land(&[]).with_ability(AbilityDef::activated_mana(
        "{T}, Pay 1 life: Add one mana of any color.",
        &[CostDef::TapSource, CostDef::PayLife(1)],
        EffectDef::AddMana(AddManaEffectDef::any_color()),
    )),
);

// JOU 164 — Temple of Epiphany
pub(in crate::card::sets) static TEMPLE_OF_EPIPHANY: CardRecord = CardRecord::new(
    "Temple of Epiphany",
    "b882c6bf-b795-49fe-8242-a928aadb6f13",
    "Noah Bradley",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, scry 1. (Look at the top card of your \
             library. You may put that card on the bottom.)",
            abilities::scry(ValueDef::Constant(1)),
        ),
        AbilityDef::activated_mana(
            "{T}: Add {U} or {R}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[ManaColor::Blue, ManaColor::Red])),
        ),
    ]),
);

// JOU 165 — Temple of Malady
pub(in crate::card::sets) static TEMPLE_OF_MALADY: CardRecord = CardRecord::new(
    "Temple of Malady",
    "f30220f1-1992-4b5c-9e13-1762fb673155",
    "James Paick",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, scry 1. (Look at the top card of your \
             library. You may put that card on the bottom.)",
            abilities::scry(ValueDef::Constant(1)),
        ),
        AbilityDef::activated_mana(
            "{T}: Add {B} or {G}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Green,
            ])),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &BANISHING_LIGHT,
    &DICTATE_OF_KRUPHIX,
    &HEROES_BANE,
    &MANA_CONFLUENCE,
    &TEMPLE_OF_EPIPHANY,
    &TEMPLE_OF_MALADY,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
