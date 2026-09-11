//! Commander 2013 cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AppliedEffectDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::CostQuantityDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::KeywordAbility;
use crate::card::ObjectPredicateDef;
use crate::card::PlayerRelation;
use crate::card::ReplacementChoiceDef;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "C13",
    slug: "commander-2013",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// C13 25 — Unexpectedly Absent
pub(in crate::card::sets) static UNEXPECTEDLY_ABSENT: CardRecord = CardRecord::new(
    "Unexpectedly Absent",
    "6dff437b-ef68-48f7-afd3-3b72d3c56187",
    "Min Yum",
// X=0 is the mode that matters: two mana puts anything on top of its
    // owner's library, which answers a permanent nothing else can touch and
    // costs its controller their draw step.
    CardRules::new_instant(mana_cost!("{X}{W}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Put target nonland permanent into its owner's library just beneath the top X cards of that library.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
        )],
        EffectDef::PutIntoLibraryBeneathTop {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            depth: ValueDef::ChosenX,
        },
    )),
);

// C13 63 — True-Name Nemesis
pub(in crate::card::sets) static TRUE_NAME_NEMESIS: CardRecord = CardRecord::new(
    "True-Name Nemesis",
    "e8c81cf6-e204-4fea-aaa1-4277366b31c7",
    "Zack Stella",
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Merfolk", "Rogue"], 3, 1).with_abilities(
        &[
            AbilityDef::replacement(
                "As this creature enters, choose a player.",
                ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                    crate::card::BattlefieldEntryScalarChoiceDef::PLAYER,
                )),
            ),
            AbilityDef::keyword(
                "This creature has protection from the chosen player.",
                KeywordAbility::ProtectionFrom(&ObjectPredicateDef::ControlledBy(
                    PlayerRelation::ChosenPlayer,
                )),
            ),
        ],
    ),
);

// C13 96 — Toxic Deluge
/// Every creature, whoever controls it, and the amount is the life its caster
/// was willing to spend. Held behind a reference because a negated value is
/// one word wider than the value it negates.
static TOXIC_DELUGE_AMOUNT: ValueDef = ValueDef::Negate(&ValueDef::ChosenX);

pub(in crate::card::sets) static TOXIC_DELUGE: CardRecord = CardRecord::new(
    "Toxic Deluge",
    "564caf57-4ba5-4993-a35e-945699c94eb7",
    "Svetlin Velinov",
CardRules::new_sorcery(mana_cost!("{2}{B}")).with_ability(
        AbilityDef::spell(
            "As an additional cost to cast this spell, pay X life.\nAll creatures get -X/-X until end of turn.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    TOXIC_DELUGE_AMOUNT,
                    TOXIC_DELUGE_AMOUNT,
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .with_spell_additional_cost(&CostDef::pay_life(
            CostQuantityDef::ChosenX,
        )),
    ),
);

// C13 279 — Boros Garrison (reprint)
const BOROS_GARRISON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2005::ravnica_city_of_guilds::BOROS_GARRISON,
    "c468dd1c-6f0a-4679-9d33-17e17db8841d",
    "John Avon",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&UNEXPECTEDLY_ABSENT, &TRUE_NAME_NEMESIS, &TOXIC_DELUGE];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] =
    &[BOROS_GARRISON_REPRINT];
