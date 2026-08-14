//! Visions cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, CardArt, CardRules, CardSet, CardType,
    EffectDef, EffectRecipientDef, ObjectPredicateDef, ReanimationAuraDef, TopCardSelectionDef,
    TriggerConditionDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement, cards,
};
use crate::mana_cost;

static IMPULSE_SELECTION: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::Constant(4),
    minimum: 1,
    maximum: 1,
    selected_zone: ZoneKind::Hand,
    selected_placement: ZonePlacement::Top,
    rest_zone: ZoneKind::Library,
    rest_placement: ZonePlacement::Bottom,
    then: None,
};

// VIS 34 — Impulse
pub(in crate::card::sets) static IMPULSE: CardRecord = CardRecord::new(
    cards::IMPULSE,
    "Impulse",
    CardArt::new("9d710a97-062f-4773-b6c6-8aeddeb3b6e8", "Bryan Talbot"),
    CardSet::Visions,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell(
        "Look at the top four cards of your library. Put one of them into your hand and the rest on the bottom of your library in any order.",
        EffectDef::LookAtTopAndSelect {
            player: EffectRecipientDef::Controller,
            selection: &IMPULSE_SELECTION,
        },
    )),
);

static NECROMANCY_CLEANUP_TRIGGER: AbilityDef = AbilityDef::triggered(
    "At the beginning of the next cleanup step, sacrifice the permanent this spell became.",
    TriggerEventDef::StepBegins {
        step: crate::card::TurnStepDef::Cleanup,
        player: crate::card::PlayerRelation::Any,
    },
    EffectDef::Sacrifice {
        object: EffectRecipientDef::LinkedPermanent,
    },
);

static NECROMANCY_LEAVE_TRIGGER: AbilityDef = AbilityDef::triggered(
    "When this enchantment leaves the battlefield, that creature's controller sacrifices it.",
    TriggerEventDef::ZoneChanged {
        object: ObjectPredicateDef::Source,
        from: Some(ZoneKind::Battlefield),
        to: None,
    },
    EffectDef::Sacrifice {
        object: EffectRecipientDef::LinkedPermanent,
    },
);

// VIS 64 — Necromancy
pub(in crate::card::sets) static NECROMANCY: CardRecord = CardRecord::new(
    cards::NECROMANCY,
    "Necromancy",
    CardArt::new("311a6257-dd77-4bb6-81cb-c8e7862350f3", "Pete Venters"),
    CardSet::Visions,
    CardRules::new_enchantment(mana_cost!("{2}{B}")).with_abilities(&[
        AbilityDef::static_ability(
            "You may cast this spell as though it had flash. If you cast it any time a sorcery couldn't have been cast, the controller of the permanent it becomes sacrifices it at the beginning of the next cleanup step.",
            EffectDef::FlashWithCleanupSacrifice {
                trigger: &NECROMANCY_CLEANUP_TRIGGER,
            },
        )
        .with_source_zones(&[ZoneKind::Hand]),
        AbilityDef::triggered_if_with_targets(
            "When this enchantment enters, if it's on the battlefield, it becomes an Aura with \"enchant creature put onto the battlefield with Necromancy.\" Put target creature card from a graveyard onto the battlefield under your control and attach this enchantment to it. When this enchantment leaves the battlefield, that creature's controller sacrifices it.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            &TriggerConditionDef::SourceOnBattlefield,
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: None,
            })],
            EffectDef::ReturnToBattlefieldAttached {
                card: EffectRecipientDef::Target(crate::TargetIndex::PRIMARY),
                aura: ReanimationAuraDef::AddAuraSubtype,
                leave: &NECROMANCY_LEAVE_TRIGGER,
            },
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&IMPULSE, &NECROMANCY];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
