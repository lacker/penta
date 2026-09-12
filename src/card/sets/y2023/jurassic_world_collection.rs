//! Jurassic World Collection card records required by the cEDH corpus.

use super::{CardRecord, PrintingRecord};
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectSetDef;
use crate::card::PlayActionMatcherDef;
use crate::card::PlayRestrictionDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SubtypeDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::card::{
    ChoiceVisibilityDef, ChooseDef, ObjectChoiceBindingDef, ObjectQueryDef, PlayerRefDef,
    PlayerRelation,
};
use crate::mana_cost;

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "REX",
    slug: "jurassic-world-collection",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());









// REX 7 — Welcome to . . . // Jurassic Park
// Audit: unsupported — Resolved durations can follow the source remaining on the battlefield, but cannot end when its controller changes. The first chapter's animation must end as soon as you no longer control this Saga, even if the Saga remains and later returns to your control.
pub(in crate::card::sets) static WELCOME_TO_JURASSIC_PARK_7: CardRecord = CardRecord::new(
    "Welcome to . . . // Jurassic Park",
    "6d84e2d4-38bf-4d46-99a6-37c2dda66b16",
    "Villarrte",
    crate::card::CardRules::unsupported(),
);

// REX 30 — Hunting Velociraptor
// Audit: unsupported — The cast planner has no granted prowl alternative cost conditioned on combat damage by a creature sharing any of the candidate spell's creature types.
pub(in crate::card::sets) static HUNTING_VELOCIRAPTOR_30: CardRecord = CardRecord::new(
    "Hunting Velociraptor",
    "419ef4a1-20d2-4770-99a5-7673518d0b86",
    "Caio Monteiro",
    crate::card::CardRules::unsupported(),
);

// REX 32 — Savage Order
pub(in crate::card::sets) static SAVAGE_ORDER_32: CardRecord = CardRecord::new(
    "Savage Order",
    "e01e2ded-f3ed-41c2-b200-6573d2c15611",
    "Jesper Ejsing",CardRules::new_sorcery(mana_cost!("{2}{G}{G}")).with_ability(AbilityDef::spell_with_additional_cost("As an additional cost to cast this spell, sacrifice a creature with power 4 or greater.\nSearch your library for a Dinosaur creature card, put it onto the battlefield, then shuffle. It gains indestructible until your next turn.", &[], CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::PowerAtLeast(4)])), EffectDef::Choose(ChooseDef { chooser: PlayerRefDef::EffectController, candidates: ObjectSetDef::Query(ObjectQueryDef::matching(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dinosaur"))]), &[ZoneKind::Library], PlayerRelation::You)), exclude: None, minimum: 0, maximum: 1, binding: ObjectChoiceBindingDef::Objects(Binding!("ordered_dinosaur")), unchosen: None, visibility: ChoiceVisibilityDef::Private, then: &EffectDef::WithZoneMoveResult { effect: &EffectDef::move_to_zone(EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("ordered_dinosaur"))), ZoneKind::Battlefield, ZonePlacement::Top), binding: Binding!("ordered_arrival"), then: &EffectDef::Sequence(&[EffectDef::ShuffleLibrary { player: EffectRecipientDef::Controller }, EffectDef::Apply { recipient: EffectRecipientDef::binding_zone_change_successors(Binding!("ordered_arrival")), effect: AppliedEffectDef::add_ability(&abilities::indestructible()), duration: ResolvedEffectDurationDef::UntilYourNextTurn }]) } }))),
);

// REX 42 — Permission Denied
pub(in crate::card::sets) static PERMISSION_DENIED_42: CardRecord = CardRecord::new(
    "Permission Denied",
    "678ad86d-6b2b-4e66-a535-9fc696cfa4ab",
    "Leonardo Santanna",
    CardRules::new_instant(mana_cost!("{W}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Counter target noncreature spell. Your opponents can't cast noncreature spells this turn.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Spell,
                    ObjectPredicateDef::NoncreatureSpell,
                ]),
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::counter_target(TargetIndex::PRIMARY),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Opponent,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(
                    PlayRestrictionDef::new(
                        PlayActionMatcherDef::CastSpell,
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                    ),
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &WELCOME_TO_JURASSIC_PARK_7,
    &HUNTING_VELOCIRAPTOR_30,
    &SAVAGE_ORDER_32,
    &PERMISSION_DENIED_42,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
