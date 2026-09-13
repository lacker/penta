//! Warhammer 40,000 Commander card records required by the cEDH corpus.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::CreateTokenDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectSetFilterDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::ids::ParentBinding;
use crate::mana_cost;

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "40K",
    slug: "warhammer-40-000-commander",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// 40K 8 — Marneus Calgar
// Audit: unsupported — The battlefield event stream is per object and cannot raise one entry trigger per simultaneous batch of tokens.
pub(in crate::card::sets) static MARNEUS_CALGAR_8: CardRecord = CardRecord::new(
    "Marneus Calgar",
    "e7517e8e-b424-4731-ba9d-6132bdefa6bf",
    "Slawomir Maniak",
    crate::card::CardRules::unsupported(),
);

// 40K 17 — Triumph of Saint Katherine
// Audit: unsupported — Needs a face-down pile combining this exact graveyard object and the library top, followed by a hidden shuffle and ordered library return.
pub(in crate::card::sets) static TRIUMPH_OF_SAINT_KATHERINE: CardRecord = CardRecord::new(
    "Triumph of Saint Katherine",
    "cc5338e1-26a6-466e-9393-788f69370e15",
    "David Astruga",
    CardRules::unsupported(),
);

// 40K 51★ — Psychomancer
pub(in crate::card::sets) static PSYCHOMANCER_51_: CardRecord = CardRecord::new(
    "Psychomancer",
    "32c7cab2-4fc9-4d53-ba67-902f72799d20",
    "Alex Konstad",
    CardRules::new_artifact_creature(mana_cost!("{1}{B}"), &["Necron", "Wizard"], 1, 1).with_abilities(&[
abilities::flying(),
AbilityDef::triggered_with_targets("Harbinger of Despair — Whenever this creature or another nontoken artifact you control is put into a graveyard from the battlefield or is put into exile from the battlefield, target opponent loses 1 life and you gain 1 life.", TriggerEventDef::AnyOf(&[TriggerEventDef::zone_changed(ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::Source, ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::Not(&ObjectPredicateDef::Token), ObjectPredicateDef::ControlledBy(PlayerRelation::You)])]), Some(ZoneKind::Battlefield), Some(ZoneKind::Graveyard)), TriggerEventDef::zone_changed(ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::Source, ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::Not(&ObjectPredicateDef::Token), ObjectPredicateDef::ControlledBy(PlayerRelation::You)])]), Some(ZoneKind::Battlefield), Some(ZoneKind::Exile))]), &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Opponent))], EffectDef::Sequence(&[EffectDef::LoseLife { recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY), amount: ValueDef::Constant(1) }, EffectDef::GainLife { recipient: EffectRecipientDef::Controller, amount: ValueDef::Constant(1) }]))
]),
);

// 40K 110 — Chaos Defiler
pub(in crate::card::sets) static CHAOS_DEFILER: CardRecord = CardRecord::new(
    "Chaos Defiler",
    "1c0f2873-5849-4cab-855a-2fbbc41dbb6c",
    "Games Workshop",
    CardRules::new_artifact_creature(mana_cost!("{3}{B}{R}"), &["Demon", "Construct"], 5, 4)
        .with_abilities(&[
            abilities::trample(),
            AbilityDef::triggered(
                "Battle Cannon — When this creature enters or dies, for each opponent, choose \
                 a nonland permanent that player controls. Destroy one of them chosen at random.",
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::Source,
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::Source,
                        Some(ZoneKind::Battlefield),
                        Some(ZoneKind::Graveyard),
                    ),
                ]),
                // In the two-player engine, the chosen set contains at most one
                // permanent, so its random member is that same permanent.
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Opponent,
                    )),
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::Destroy {
                        object: EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                        then: None,
                    },
                }),
            ),
        ]),
);

// 40K 150 — Canoptek Scarab Swarm
pub(in crate::card::sets) static CANOPTEK_SCARAB_SWARM: CardRecord = CardRecord::new(
    "Canoptek Scarab Swarm",
    "3ce2021c-1422-4901-a554-2d4fab72c8e4",
    "Alexey Kruglov",
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Insect"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "Feeder Mandibles — When this creature enters, exile target player's \
             graveyard. For each artifact or land card exiled this way, create a 1/1 \
             colorless Insect artifact creature token with flying.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::WithZoneMoveResult {
                binding: Binding!("exiled"),
                effect: &EffectDef::move_to_zone(
                    EffectRecipientDef::cards_owned_by_target(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Graveyard],
                        TargetIndex::PRIMARY,
                    ),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
                then: &EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(
                        TokenCharacteristics::artifact_creature(&["Insect"], &[], 1, 1)
                            .with_abilities(&[abilities::flying()])
                            .with_art(crate::card::CardArt::new(
                                "9af00a97-9938-4ab2-938a-669cdfe4332a",
                                "Bartek Fedyczak",
                            )),
                    ))
                    .with_count(ValueDef::CountObjects(
                        &ObjectSetDef::Matching {
                            objects: &ObjectSetDef::ZoneChangeSuccessorsOfBinding(Binding!(
                                "exiled"
                            )),
                            object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Land),
                            ])),
                        },
                    )),
                ),
            },
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &MARNEUS_CALGAR_8,
    &TRIUMPH_OF_SAINT_KATHERINE,
    &PSYCHOMANCER_51_,
    &CHAOS_DEFILER,
    &CANOPTEK_SCARAB_SWARM,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
