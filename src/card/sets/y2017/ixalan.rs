//! Ixalan cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, CardArt, CardRules, CardSet, CardType,
    CounterKind, EffectDef, EffectRecipientDef, InstalledTriggerDef, ObjectPredicateDef,
    ObjectRefDef, PlayerRefDef, PlayerRelation, TriggerEventDef, ValueDef, ZoneKind, abilities,
};
use crate::ids::ParentBinding;
use crate::{TargetIndex, mana_cost};

// XLN 41 — Territorial Hammerskull
pub(in crate::card::sets) static TERRITORIAL_HAMMERSKULL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("af5a237a-31e7-43ee-8d47-3eb12dd1a60c"),
    "Territorial Hammerskull",
    CardArt::new("af5a237a-31e7-43ee-8d47-3eb12dd1a60c", "Lars Grant-West"),
    CardSet::Ixalan,
    // The tap happens on the declaration, so it clears a blocker before
    // blockers are chosen: a 2/3 that attacks as if it were much larger.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Dinosaur"], 2, 3).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, tap target creature an opponent controls.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ),
);

// XLN 110 — Kitesail Freebooter
pub(in crate::card::sets) static KITESAIL_FREEBOOTER: CardRecord = CardRecord::new_with_legacy_id(
    2149,
    "Kitesail Freebooter",
    CardArt::new("f62fd592-4910-417d-a500-e7029f3d119f", "Dan Murayama Scott"),
    CardSet::Ixalan,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Pirate"], 1, 2)
        .with_abilities(&[
            abilities::flying(),
            abilities::enters_trigger_with_targets(
                "When this creature enters, target opponent reveals their hand. You choose a noncreature, nonland card from it. Exile that card until this creature leaves the battlefield.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::Sequence(&abilities::reveal_hand_and_choose_card(
                    PlayerRefDef::Target(TargetIndex::PRIMARY),
                    // Neither a creature nor a land: the Freebooter takes the answer, not the
                    // threat, which is what separates it from the Sculler.
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    ]),
                    &EffectDef::Sequence(&[
                        EffectDef::ExileLinkedToSource {
                            until_source_leaves: true,
                            object: EffectRecipientDef::object(ObjectRefDef::Binding(ParentBinding)),
                            face_down: false,
                            then: None,
                        },
                        // "Until this creature leaves the battlefield" is one printed ability, so
                        // the return is a delayed trigger installed by the same resolution rather
                        // than a second clause the card does not print.
                        EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                            "When this creature leaves the battlefield, return the exiled card to its owner's hand.",
                            TriggerEventDef::zone_changed(
                                ObjectPredicateDef::Source,
                                Some(ZoneKind::Battlefield),
                                None,
                            ),
                            EffectDef::ReturnLinkedExiles {
                                object: ObjectPredicateDef::Any,
                                counters: None,
                                zone: ZoneKind::Hand,
                                grant: None,
                                controller: None,
                                transformed: false,
                            },
                        ))),
                    ]),
                )),
            ),
        ]),
);

// XLN 194 — Jade Guardian
pub(in crate::card::sets) static JADE_GUARDIAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aca83e48-6e32-477f-8714-6103e77c06df"),
    "Jade Guardian",
    CardArt::new("aca83e48-6e32-477f-8714-6103e77c06df", "Chris Seaman"),
    CardSet::Ixalan,
    // Hexproof is what makes the counter safe to spend on itself: a 3/3 the
    // opponent cannot answer with a spell.
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Merfolk", "Shaman"], 2, 2).with_abilities(&[
        abilities::hexproof(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, put a +1/+1 counter on target Merfolk you control.",
            // It is itself a Merfolk, so a board with no other one still has
            // a legal target.
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Subtype("Merfolk"),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// XLN 248 — Sorcerous Spyglass
pub(in crate::card::sets) static SORCEROUS_SPYGLASS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("85506a24-8d60-475c-9f43-65994caca7d4"),
    "Sorcerous Spyglass",
    crate::card::CardArt::new("85506a24-8d60-475c-9f43-65994caca7d4", "Kieran Yanner"),
    crate::card::CardSet::Ixalan,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        abilities::look_at_opponent_hand_then_choose_card_name_as_enters(
            "As this artifact enters, look at an opponent's hand, then choose any card name.",
        ),
        abilities::cannot_activate_nonmana_abilities_with_chosen_name(
            "Activated abilities of sources with the chosen name can't be activated unless they're mana abilities.",
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &TERRITORIAL_HAMMERSKULL,
    &KITESAIL_FREEBOOTER,
    &JADE_GUARDIAN,
    &SORCEROUS_SPYGLASS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
