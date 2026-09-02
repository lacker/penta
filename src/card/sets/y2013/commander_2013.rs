//! Commander 2013 cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::CostQuantityDef;
use crate::card::{
    AbilityDef, AbilityTargetDef, AppliedEffectDef, CardArt, CardRules, CardSet, CardType,
    EffectDef, EffectRecipientDef, KeywordAbility, ObjectPredicateDef, PlayerRelation,
    ReplacementChoiceDef, ReplacementEffectDef, ResolvedEffectDurationDef, SpellAdditionalCostDef,
    ValueDef, ZoneKind,
};
use crate::{TargetIndex, mana_cost};

// C13 25 — Unexpectedly Absent
pub(in crate::card::sets) static UNEXPECTEDLY_ABSENT: CardRecord = CardRecord::new_with_legacy_id(
    2182,
    "Unexpectedly Absent",
    CardArt::new("6dff437b-ef68-48f7-afd3-3b72d3c56187", "Min Yum"),
    CardSet::Commander2013,
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
    PrintingAnchor::scryfall("e8c81cf6-e204-4fea-aaa1-4277366b31c7"),
    "True-Name Nemesis",
    CardArt::new("e8c81cf6-e204-4fea-aaa1-4277366b31c7", "Zack Stella"),
    CardSet::Commander2013,
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

pub(in crate::card::sets) static TOXIC_DELUGE: CardRecord = CardRecord::new_with_legacy_id(
    2164,
    "Toxic Deluge",
    CardArt::new("564caf57-4ba5-4993-a35e-945699c94eb7", "Svetlin Velinov"),
    CardSet::Commander2013,
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
        .with_spell_additional_cost(&SpellAdditionalCostDef::pay_life(
            CostQuantityDef::ChosenX,
        )),
    ),
);

// C13 279 — Boros Garrison
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOROS_GARRISON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7dfe3f03-078f-44fb-89cd-efa3ebfaf637"),
    "Boros Garrison",
    crate::card::CardArt::new("c468dd1c-6f0a-4679-9d33-17e17db8841d", "John Avon"),
    crate::card::CardSet::Commander2013,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &UNEXPECTEDLY_ABSENT,
    &TRUE_NAME_NEMESIS,
    &TOXIC_DELUGE,
    &BOROS_GARRISON,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
