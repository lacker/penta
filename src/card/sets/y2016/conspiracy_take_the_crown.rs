//! Conspiracy: Take the Crown cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AppliedEffectDef, AppliedRuleDef, CardArt, CardRules, CardSet,
    CardSupertype, CardType, EffectDef, EffectRecipientDef, InstalledTriggerDef,
    ObjectPredicateDef, PlayerRefDef, PlayerRelation, StackTargetAggregationDef,
    StackTargetFilterDef, TriggerEventDef, ValueDef, ZoneKind, abilities,
};
use crate::{TargetIndex, mana_cost};

// CN2 18 — Palace Jailer
pub(in crate::card::sets) static PALACE_JAILER: CardRecord = CardRecord::new_with_legacy_id(
    2171,
    "Palace Jailer",
    CardArt::new("78cef262-c753-4658-b3ec-fec8db47f944", "David Palumbo"),
    CardSet::ConspiracyTakeTheCrown,
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
pub(in crate::card::sets) static PALACE_SENTINELS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3e002a99-eb2b-4cc3-992e-f3ee42245dba"),
    "Palace Sentinels",
    CardArt::new("3e002a99-eb2b-4cc3-992e-f3ee42245dba", "Aaron Miller"),
    CardSet::ConspiracyTakeTheCrown,
    // A 2/4 wall attached to a card every turn, which is why the crown is
    // worth four mana on a body that does nothing else.
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Soldier"], 2, 4).with_ability(
        abilities::enters_trigger(
            "When this creature enters, you become the monarch.",
            EffectDef::BecomeMonarch {
                player: PlayerRefDef::EffectController,
            },
        ),
    ),
);

// CN2 48 — Thorn of the Black Rose
pub(in crate::card::sets) static THORN_OF_THE_BLACK_ROSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2e4829c6-50d4-4602-af78-59249486a97c"),
    "Thorn of the Black Rose",
    CardArt::new("2e4829c6-50d4-4602-af78-59249486a97c", "David Gaillet"),
    CardSet::ConspiracyTakeTheCrown,
    // Deathtouch is what defends the crown: nothing profitably attacks
    // through it, which is the whole reason the body is worth four mana.
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Human", "Assassin"], 1, 3).with_abilities(&[
        abilities::deathtouch(),
        abilities::enters_trigger(
            "When this creature enters, you become the monarch.",
            EffectDef::BecomeMonarch {
                player: PlayerRefDef::EffectController,
            },
        ),
    ]),
);

// CN2 64 — Entourage of Trest
// Audit: unsupported — Needs a "you are the monarch" state condition. Becoming the monarch and blocking an additional creature both exist (EffectDef::BecomeMonarch, AppliedRuleDef::MayBlockAdditionalCreatures), but nothing can ask whether the ability's controller currently holds the crown, and granting the extra block unconditionally would keep it after the crown is taken away.
pub(in crate::card::sets) static ENTOURAGE_OF_TREST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3d7ee777-6113-43f8-883e-c7569eefb872"),
    "Entourage of Trest",
    crate::card::CardArt::new("3d7ee777-6113-43f8-883e-c7569eefb872", "Anthony Palumbo"),
    crate::card::CardSet::ConspiracyTakeTheCrown,
    crate::card::CardRules::unsupported(),
);

// CN2 77 — Leovold, Emissary of Trest
pub(in crate::card::sets) static LEOVOLD_EMISSARY_OF_TREST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("49bb0ad3-1082-41f1-82a4-52a4006cc9b6"),
    "Leovold, Emissary of Trest",
    CardArt::new("49bb0ad3-1082-41f1-82a4-52a4006cc9b6", "Magali Villeneuve"),
    CardSet::ConspiracyTakeTheCrown,
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
                TriggerEventDef::targets_selected(
                    ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                    StackTargetFilterDef::AnyOf(&[
                        StackTargetFilterDef::Player(PlayerRelation::You),
                        StackTargetFilterDef::Permanent(ObjectPredicateDef::ControlledBy(
                            PlayerRelation::You,
                        )),
                    ]),
                    StackTargetAggregationDef::EachMatchingTarget,
                ),
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
