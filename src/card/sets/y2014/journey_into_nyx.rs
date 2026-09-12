//! Journey into Nyx cards cataloged for the Vintage Cube pool.

use crate::TargetIndex;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BindObjectsDef;
use crate::card::CopyAbilityDef;
use crate::card::CopyExceptionsDef;
use crate::card::CostQuantityDef;
use crate::card::CreatedTokensDef;
use crate::card::InstalledTriggerDef;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PlayActionMatcherDef;
use crate::card::PlayRestrictionDef;
use crate::card::PlayerRuleDef;
use crate::card::TokenCopyDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
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

// JOU 1 — Aegis of the Gods
pub(in crate::card::sets) static AEGIS_OF_THE_GODS_1: CardRecord = CardRecord::new(
    "Aegis of the Gods",
    "f2b2f381-86a2-42ac-b694-dcde437d574f",
    "Yefim Kligerman",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Soldier"], 2, 1).with_abilities(&[
AbilityDef::static_ability("You have hexproof. (You can't be the target of spells or abilities your opponents control.)", EffectDef::StaticApply { recipient: EffectRecipientDef::Controller, effect: AppliedEffectDef::Rule(AppliedRuleDef::PlayerRule(PlayerRuleDef::Hexproof)) })
])
.with_type(crate::card::CardType::Enchantment),
);

// JOU 5 — Banishing Light
// Audit: unsupported — Needs exile-until-source-leaves with immediate return when the duration ends (CR 610.3); an ordinary leaves trigger returns through the stack too late.
pub(in crate::card::sets) static BANISHING_LIGHT: CardRecord = CardRecord::new(
    "Banishing Light",
    "fbaa4800-30cc-4a80-a6cc-9a24ada9eb40",
    "Willian Murai",
    CardRules::unsupported(),
);

// JOU 10 — Eidolon of Rhetoric
pub(in crate::card::sets) static EIDOLON_OF_RHETORIC_10: CardRecord = CardRecord::new(
    "Eidolon of Rhetoric",
    "c3bc8b9e-4d22-41ba-b593-d383fd301ef9",
    "Ryan Yee",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Spirit"], 1, 4)
        .with_abilities(&[AbilityDef::static_ability(
            "Each player can't cast more than one spell each turn.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::EachPlayer,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(
                    PlayRestrictionDef::new(
                        PlayActionMatcherDef::CastSpell,
                        ObjectPredicateDef::Any,
                    )
                    .after_spells_cast(1),
                )),
            },
        )])
        .with_type(crate::card::CardType::Enchantment),
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

// JOU 94 — Eidolon of the Great Revel
pub(in crate::card::sets) static EIDOLON_OF_THE_GREAT_REVEL_94: CardRecord = CardRecord::new(
    "Eidolon of the Great Revel",
    "a6c10816-e825-452a-90b0-80eb9f20bd6d",
    "Cyril Van Der Haegen",
    CardRules::new_creature(mana_cost!("{R}{R}"), &["Spirit"], 2, 2).with_abilities(&[
AbilityDef::triggered("Whenever a player casts a spell with mana value 3 or less, this creature deals 2 damage to that player.", TriggerEventDef::spell_cast(ObjectPredicateDef::ManaValueAtMost(3)), EffectDef::damage(EffectRecipientDef::ControllerOfTriggeringObject, ValueDef::Constant(2)))
])
.with_type(crate::card::CardType::Enchantment),
);

// JOU 115 — Twinflame
pub(in crate::card::sets) static TWINFLAME_115: CardRecord = CardRecord::new(
    "Twinflame",
    "207128b3-2de3-495a-bf29-eec50c3bd752",
    "Chase Stone",
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_abilities(&[
AbilityDef::spell_with_additional_cost("Strive — This spell costs {2}{R} more to cast for each target beyond the first.\nChoose any number of target creatures you control. For each of them, create a token that's a copy of that creature, except it has haste. Exile those tokens at the beginning of the next end step.", &[AbilityTargetDef::any_number(AbilityTargetPredicate::Object { object: ObjectPredicateDef::HasType(CardType::Creature), zones: &[ZoneKind::Battlefield], controller: Some(PlayerRelation::You), owner: None })], CostDef::ManaTimes { cost: mana_cost!("{2}{R}"), quantity: CostQuantityDef::Subtract(&CostQuantityDef::TargetCount, &CostQuantityDef::Fixed(1)) }, EffectDef::BindObjects(BindObjectsDef { source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::LegalTargets(TargetIndex::PRIMARY)), binding: Binding!("twinflame_targets"), then: &EffectDef::ForEachInBinding { objects: Binding!("twinflame_targets"), binding: Binding!("twinflame_original"), effect: &EffectDef::create_token_from_copy(&TokenCopyDef { object: &EffectRecipientDef::object(ObjectRefDef::Binding(Binding!("twinflame_original"))), exceptions: CopyExceptionsDef::NONE.with_abilities(&[CopyAbilityDef::Ability(&abilities::haste())]) }).with_created_tokens(CreatedTokensDef { binding: Binding!("twinflame_token"), then: &EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered("At the beginning of the next end step, exile those tokens.", TriggerEventDef::StepBegins { step: TurnStepDef::End, player: PlayerRelation::Any }, EffectDef::move_to_zone(EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("twinflame_token"))), ZoneKind::Exile, ZonePlacement::Top)))) }) } }))
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
    &AEGIS_OF_THE_GODS_1,
    &BANISHING_LIGHT,
    &EIDOLON_OF_RHETORIC_10,
    &DICTATE_OF_KRUPHIX,
    &EIDOLON_OF_THE_GREAT_REVEL_94,
    &TWINFLAME_115,
    &HEROES_BANE,
    &MANA_CONFLUENCE,
    &TEMPLE_OF_EPIPHANY,
    &TEMPLE_OF_MALADY,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
