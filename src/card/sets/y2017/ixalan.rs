//! Ixalan cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::CounterKind;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::InstalledTriggerDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectRefDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::SubtypeDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::ids::ParentBinding;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "XLN",
    slug: "ixalan",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// XLN 6 — Bishop's Soldier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BISHOP_S_SOLDIER: CardRecord = CardRecord::new(
    "Bishop's Soldier",
    "d954677c-2de6-440a-90d0-bab2e0c8b4af",
    "Scott Murphy",
    crate::card::CardRules::unsupported(),
);

// XLN 34 — Settle the Wreckage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SETTLE_THE_WRECKAGE: CardRecord = CardRecord::new(
    "Settle the Wreckage",
    "9cbd346e-098a-4cf6-a72f-468376fd2e8f",
    "Dimitar Marinski",
    crate::card::CardRules::unsupported(),
);

// XLN 41 — Territorial Hammerskull
pub(in crate::card::sets) static TERRITORIAL_HAMMERSKULL: CardRecord = CardRecord::new(
    "Territorial Hammerskull",
    "af5a237a-31e7-43ee-8d47-3eb12dd1a60c",
    "Lars Grant-West",
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

// XLN 48 — Chart a Course
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHART_A_COURSE: CardRecord = CardRecord::new(
    "Chart a Course",
    "98291778-2ec2-47e2-ac99-5f8cfbb3cf24",
    "James Ryman",
    crate::card::CardRules::unsupported(),
);

// XLN 53 — Dive Down
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIVE_DOWN: CardRecord = CardRecord::new(
    "Dive Down",
    "b33e493e-1aef-43b3-9716-52158b002430",
    "Magali Villeneuve",
    crate::card::CardRules::unsupported(),
);

// XLN 71 — River's Rebuke
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIVER_S_REBUKE: CardRecord = CardRecord::new(
    "River's Rebuke",
    "fda8ef30-bbfa-4857-9750-0dd0def8b13f",
    "Raymond Swanland",
    crate::card::CardRules::unsupported(),
);

// XLN 84 — Storm Fleet Spy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORM_FLEET_SPY: CardRecord = CardRecord::new(
    "Storm Fleet Spy",
    "f7c33ef4-60bb-4f95-92a5-7abedaac6767",
    "Scott Murphy",
    crate::card::CardRules::unsupported(),
);

// XLN 110 — Kitesail Freebooter
pub(in crate::card::sets) static KITESAIL_FREEBOOTER: CardRecord = CardRecord::new(
    "Kitesail Freebooter",
    "f62fd592-4910-417d-a500-e7029f3d119f",
    "Dan Murayama Scott",
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

// XLN 123 — Skulduggery
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKULDUGGERY: CardRecord = CardRecord::new(
    "Skulduggery",
    "ba30343b-1637-490f-810e-d614219789e3",
    "Deruchenko Alexander",
    crate::card::CardRules::unsupported(),
);

// XLN 191 — Growing Rites of Itlimoc // Itlimoc, Cradle of the Sun
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GROWING_RITES_OF_ITLIMOC: CardRecord = CardRecord::new(
    "Growing Rites of Itlimoc // Itlimoc, Cradle of the Sun",
    "b3b87bfc-f97f-4734-94f6-e3e2f335fc4d",
    "Grzegorz Rutkowski",
    crate::card::CardRules::unsupported(),
);

// XLN 194 — Jade Guardian
pub(in crate::card::sets) static JADE_GUARDIAN: CardRecord = CardRecord::new(
    "Jade Guardian",
    "aca83e48-6e32-477f-8714-6103e77c06df",
    "Chris Seaman",
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
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Merfolk")),
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

// XLN 198 — New Horizons
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NEW_HORIZONS: CardRecord = CardRecord::new(
    "New Horizons",
    "15b12c75-1248-4c81-90cf-28e341a885cf",
    "Noah Bradley",
    crate::card::CardRules::unsupported(),
);

// XLN 222 — Gishath, Sun's Avatar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GISHATH_SUN_S_AVATAR: CardRecord = CardRecord::new(
    "Gishath, Sun's Avatar",
    "7335e500-342d-476d-975c-817512e6e3d6",
    "Zack Stella",
    crate::card::CardRules::unsupported(),
);

// XLN 242 — Pirate's Cutlass
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PIRATE_S_CUTLASS: CardRecord = CardRecord::new(
    "Pirate's Cutlass",
    "b3e7e871-19cf-486d-bacb-1499fe066974",
    "John Stanko",
    crate::card::CardRules::unsupported(),
);

// XLN 248 — Sorcerous Spyglass
pub(in crate::card::sets) static SORCEROUS_SPYGLASS: CardRecord = CardRecord::new(
    "Sorcerous Spyglass",
    "85506a24-8d60-475c-9f43-65994caca7d4",
    "Kieran Yanner",
CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::as_enters(
            "As this artifact enters, look at an opponent's hand, then choose any card name.",
            crate::card::ReplacementEffectDef::Sequence(&[
                crate::card::ReplacementEffectDef::LookAtHand(PlayerRelation::Opponent),
                crate::card::ReplacementEffectDef::BindOutput {
                    binding: crate::Binding!("sorcerous_spyglass_name"),
                    effect: &abilities::choose_card_name_as_enters(
                        crate::card::CardNameSetDef::AllCardNames,
                    ),
                },
            ]),
        ),
        abilities::cannot_activate_nonmana_abilities_with_name(
            "Activated abilities of sources with the chosen name can't be activated unless they're mana abilities.",
            crate::card::CardNameDef::Binding(crate::Binding!("sorcerous_spyglass_name")),
        ),
    ]),
);

// XLN 250 — Treasure Map // Treasure Cove
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TREASURE_MAP: CardRecord = CardRecord::new(
    "Treasure Map // Treasure Cove",
    "c0f9c733-0818-4a03-8f0c-a163d09e0fff",
    "Cliff Childs",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &BISHOP_S_SOLDIER,
    &SETTLE_THE_WRECKAGE,
    &TERRITORIAL_HAMMERSKULL,
    &CHART_A_COURSE,
    &DIVE_DOWN,
    &RIVER_S_REBUKE,
    &STORM_FLEET_SPY,
    &KITESAIL_FREEBOOTER,
    &SKULDUGGERY,
    &GROWING_RITES_OF_ITLIMOC,
    &JADE_GUARDIAN,
    &NEW_HORIZONS,
    &GISHATH_SUN_S_AVATAR,
    &PIRATE_S_CUTLASS,
    &SORCEROUS_SPYGLASS,
    &TREASURE_MAP,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
