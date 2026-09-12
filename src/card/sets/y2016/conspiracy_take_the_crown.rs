//! Conspiracy: Take the Crown cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::InstalledTriggerDef;
use crate::card::ObjectPredicateDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::StackTargetAggregationDef;
use crate::card::StackTargetFilterDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "CN2",
    slug: "conspiracy-take-the-crown",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// CN2 18 — Palace Jailer
pub(in crate::card::sets) static PALACE_JAILER: CardRecord = CardRecord::new(
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
pub(in crate::card::sets) static PALACE_SENTINELS: CardRecord = CardRecord::new(
    "Palace Sentinels",
    "3e002a99-eb2b-4cc3-992e-f3ee42245dba",
    "Aaron Miller",
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

// CN2 22 — Recruiter of the Guard
pub(in crate::card::sets) static RECRUITER_OF_THE_GUARD_22: CardRecord = CardRecord::new(
    "Recruiter of the Guard",
    "bb9ad57f-cca2-4717-a951-cbe3c7782efe",
    "Jason Rainville",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 1, 1).with_abilities(&[
abilities::enters_trigger("When this creature enters, you may search your library for a creature card with toughness 2 or less, reveal it, put it into your hand, then shuffle.", EffectDef::SearchZone { player: EffectRecipientDef::Controller, source: ZoneKind::Library, object: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::ToughnessLessThan(ValueDef::Constant(3))]), minimum: 0, maximum: ValueDef::Constant(1), reveal: true, destination: ZoneKind::Hand, placement: ZonePlacement::Top, shuffle: true, enters_tapped: false, attachment: None, binding: None, then: None })
]),
);

// CN2 23 — Sanctum Prelate
// Audit: unsupported — Entry replacements cannot record an arbitrary chosen number for a continuous mana-value cast prohibition.
pub(in crate::card::sets) static SANCTUM_PRELATE_23: CardRecord = CardRecord::new(
    "Sanctum Prelate",
    "1d95a7dd-2803-4164-8979-d7e8e8085ca2",
    "Winona Nelson",
    crate::card::CardRules::unsupported(),
);

// CN2 30 — Expropriate
// Audit: unsupported — The engine has no ordered player voting procedure with separate per-voter consequences.
pub(in crate::card::sets) static EXPROPRIATE_30: CardRecord = CardRecord::new(
    "Expropriate",
    "9c8a2a5a-cb9b-4582-a453-085da78584f9",
    "Zack Stella",
    crate::card::CardRules::unsupported(),
);

// CN2 48 — Thorn of the Black Rose
pub(in crate::card::sets) static THORN_OF_THE_BLACK_ROSE: CardRecord = CardRecord::new(
    "Thorn of the Black Rose",
    "2e4829c6-50d4-4602-af78-59249486a97c",
    "David Gaillet",
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
    "Entourage of Trest",
    "3d7ee777-6113-43f8-883e-c7569eefb872",
    "Anthony Palumbo",
    crate::card::CardRules::unsupported(),
);

// CN2 69 — Regal Behemoth
// Audit: unsupported — The monarch designation and its transfer rules exist, but no condition can ask whether this ability's controller is currently the monarch before triggering the additional mana.
pub(in crate::card::sets) static REGAL_BEHEMOTH_69: CardRecord = CardRecord::new(
    "Regal Behemoth",
    "644e5b24-394a-43f1-8384-186d6108f540",
    "Jakub Kasper",
    crate::card::CardRules::unsupported(),
);

// CN2 70 — Selvala, Heart of the Wilds
// Audit: unsupported — The resolving draw comparison is expressible, but mana_ability_value delegates aggregate power to cost_reduction_value, which returns zero for AggregateObjectValues. The greatest-power mana amount is therefore not executable by the current mana-activation planner.
pub(in crate::card::sets) static SELVALA_HEART_OF_THE_WILDS_70: CardRecord = CardRecord::new(
    "Selvala, Heart of the Wilds",
    "99a3e619-fbdb-406c-9f21-eb582cb878c0",
    "Tyler Jacobson",
    crate::card::CardRules::unsupported(),
);

// CN2 77 — Leovold, Emissary of Trest
pub(in crate::card::sets) static LEOVOLD_EMISSARY_OF_TREST: CardRecord = CardRecord::new(
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
    &RECRUITER_OF_THE_GUARD_22,
    &SANCTUM_PRELATE_23,
    &EXPROPRIATE_30,
    &THORN_OF_THE_BLACK_ROSE,
    &ENTOURAGE_OF_TREST,
    &REGAL_BEHEMOTH_69,
    &SELVALA_HEART_OF_THE_WILDS_70,
    &LEOVOLD_EMISSARY_OF_TREST,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
