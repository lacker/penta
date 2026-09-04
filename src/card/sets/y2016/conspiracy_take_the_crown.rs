//! Conspiracy: Take the Crown cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AppliedEffectDef, AppliedRuleDef, CardRules, CardSet,
    CardSupertype, CardType, EffectDef, EffectRecipientDef, InstalledTriggerDef,
    ObjectPredicateDef, PlayerRefDef, PlayerRelation, TriggerEventDef, ValueDef, ZoneKind,
    abilities,
};
use crate::{TargetIndex, mana_cost};

// CN2 18 — Palace Jailer
pub(in crate::card::sets) static PALACE_JAILER: CardRecord = CardRecord::new(
    CardSet::ConspiracyTakeTheCrown,
    "Palace Jailer",
    "78cef262-c753-4658-b3ec-fec8db47f944",
    "David Palumbo",
    // The crown is the card: a removal spell that also draws every turn, for
    // as long as nobody can get through to take it back.
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Human", "Soldier"], 2, 2)
        .with_abilities(&[
            abilities::enters_trigger(
                "When this creature enters, you become the monarch.",
                EffectDef::BecomeMonarch {
                    player: PlayerRefDef::EffectController,
                },
            ),
            abilities::enters_trigger_with_targets(
                "When this creature enters, exile target creature an opponent controls until an opponent becomes the monarch.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                    ]),
                )],
                // Exiling and arming the release are one clause: the card is linked to the
                // Jailer, and the delayed trigger is what "until" means.
                EffectDef::Sequence(&[
                    EffectDef::ExileLinkedToSource {
                        until_source_leaves: false,
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        face_down: false,
                        then: None,
                    },
                    // The release. It listens from outside every zone, so a Jailer that has
                    // already died still gives the creature back the moment the crown changes
                    // hands -- and if it never does, the creature never comes back.
                    EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                        "When an opponent becomes the monarch, return the exiled card to the battlefield.",
                        TriggerEventDef::BecomesMonarch(PlayerRelation::Opponent),
                        EffectDef::ReturnLinkedExiles {
                            object: ObjectPredicateDef::Any,
                            counters: None,
                            zone: ZoneKind::Battlefield,
                            grant: None,
                            controller: None,
                            transformed: false,
                        },
                    ))),
                ]),
            ),
        ]),
);

// CN2 19 — Palace Sentinels
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PALACE_SENTINELS: CardRecord = CardRecord::new(
    crate::card::CardSet::ConspiracyTakeTheCrown,
    "Palace Sentinels",
    "3e002a99-eb2b-4cc3-992e-f3ee42245dba",
    "Aaron Miller",
    crate::card::CardRules::unsupported(),
);

// CN2 48 — Thorn of the Black Rose
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THORN_OF_THE_BLACK_ROSE: CardRecord = CardRecord::new(
    crate::card::CardSet::ConspiracyTakeTheCrown,
    "Thorn of the Black Rose",
    "2e4829c6-50d4-4602-af78-59249486a97c",
    "David Gaillet",
    crate::card::CardRules::unsupported(),
);

// CN2 64 — Entourage of Trest
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENTOURAGE_OF_TREST: CardRecord = CardRecord::new(
    crate::card::CardSet::ConspiracyTakeTheCrown,
    "Entourage of Trest",
    "3d7ee777-6113-43f8-883e-c7569eefb872",
    "Anthony Palumbo",
    crate::card::CardRules::unsupported(),
);

// CN2 77 — Leovold, Emissary of Trest
pub(in crate::card::sets) static LEOVOLD_EMISSARY_OF_TREST: CardRecord = CardRecord::new(
    CardSet::ConspiracyTakeTheCrown,
    "Leovold, Emissary of Trest",
    "49bb0ad3-1082-41f1-82a4-52a4006cc9b6",
    "Magali Villeneuve",
    // Three mana that turns every draw spell they have into one card and
    // every removal spell they point at you into a replacement.
    CardRules::new_creature(mana_cost!("{B}{G}{U}"), &["Elf", "Advisor"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Each opponent can't draw more than one card each turn.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Opponent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotDrawMoreThanEachTurn(1)),
                },
            ),
            // The turn-based draw for the turn is one of the two: an opponent who
            // has already drawn a card off something of their own draws nothing in
            // their draw step, and their second Brainstorm card never arrives.
            AbilityDef::triggered(
                "Whenever you or a permanent you control becomes the target of a spell or ability an \
                 opponent controls, you may draw a card.",
                TriggerEventDef::YouOrYourPermanentBecomesTarget(ObjectPredicateDef::ControlledBy(
                    PlayerRelation::Opponent,
                )),
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                },
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &PALACE_JAILER,
    &PALACE_SENTINELS,
    &THORN_OF_THE_BLACK_ROSE,
    &ENTOURAGE_OF_TREST,
    &LEOVOLD_EMISSARY_OF_TREST,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
