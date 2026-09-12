//! Commander 2014 cards cataloged for the Vintage Cube pool.

use crate::card::AppliedRuleDef;
use crate::card::BindObjectsDef;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::CopyStackObjectDef;
use crate::card::DeckConstructionDef;
use crate::card::EmblemCharacteristics;
use crate::card::GameActionDef;
use crate::card::InstalledTriggerDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectRefDef;
use crate::card::PlayerRefDef;
use crate::card::TurnStepDef;
use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AppliedEffectDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::CreateTokenDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRelation;
use crate::card::ReplacementEffectDef;
use crate::card::ReplacementEventDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "C14",
    slug: "commander-2014",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// C14 5 — Containment Priest
pub(in crate::card::sets) static CONTAINMENT_PRIEST: CardRecord = CardRecord::new(
    "Containment Priest",
    "c2c794b9-09da-49be-b258-b0e21f1663e3",
    "John Stanko",
    // Flash is half the card: it is held up like a counterspell and answers
    // the reanimation on the stack rather than the creature on the board.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Cleric"], 2, 2).with_abilities(&[
        abilities::flash(),
        AbilityDef::replacement_for(
            "If a nontoken creature would enter and it wasn't cast, exile it instead.",
            ReplacementEventDef::ObjectEntersBattlefield {
                // A nontoken creature that was not cast. Tokens are exempt because the card
                // says so; everything else that arrives without going through the stack --
                // reanimation, Show and Tell, a fetched Natural Order target -- is not.
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                ]),
                controller: PlayerRelation::Any,
                cast: Some(false),
            },
            ReplacementEffectDef::MoveToZone(ZoneKind::Exile),
        ),
    ]),
);

// C14 9 — Jazal Goldmane
pub(in crate::card::sets) static JAZAL_GOLDMANE: CardRecord = CardRecord::new(
    "Jazal Goldmane",
    "c410d530-e9fc-4dc0-a4bd-70bd70aaf0c7",
    "Aaron Miller",
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Cat", "Warrior"], 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::first_strike(),
            AbilityDef::activated(
                "{3}{W}{W}: Attacking creatures you control get +X/+X until \
                 end of turn, where X is the number of attacking creatures.",
                &[CostDef::Mana(mana_cost!("{3}{W}{W}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Attacking,
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Attacking,
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        )),
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Attacking,
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        )),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// C14 19 — Teferi, Temporal Archmage
pub(in crate::card::sets) static TEFERI_TEMPORAL_ARCHMAGE_19: CardRecord = CardRecord::new(
    "Teferi, Temporal Archmage",
    "80eec0bd-9eb4-4360-b97b-9c7985a63343",
    "Tyler Jacobson",
    CardRules::new_planeswalker(mana_cost!("{4}{U}{U}"), &["Teferi"], 5).with_supertype(CardSupertype::Legendary).with_abilities(&[AbilityDef::activated("+1: Look at the top two cards of your library. Put one of them into your hand and the other on the bottom of your library.", &[CostDef::Loyalty(1)], abilities::look_at_top_cards_choose_to_hand_rest_bottom(ValueDef::Constant(2), ObjectPredicateDef::Any, 1, 1)), AbilityDef::activated_with_targets("−1: Untap up to four target permanents.", &[CostDef::Loyalty(-1)], &[AbilityTargetDef::up_to(AbilityTargetPredicate::Object { object: ObjectPredicateDef::Any, zones: &[ZoneKind::Battlefield], controller: None, owner: None }, 4)], EffectDef::Untap { object: EffectRecipientDef::Target(TargetIndex::PRIMARY) }), AbilityDef::activated("−10: You get an emblem with \"You may activate loyalty abilities of planeswalkers you control on any player's turn any time you could cast an instant.\"", &[CostDef::Loyalty(-10)], EffectDef::CreateEmblem { emblem: EmblemCharacteristics::new("Teferi, Temporal Archmage emblem", &[AbilityDef::static_ability("You may activate loyalty abilities of planeswalkers you control on any player's turn any time you could cast an instant.", EffectDef::StaticApply { recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Planeswalker), &[ZoneKind::Battlefield], PlayerRelation::You), effect: AppliedEffectDef::Rule(AppliedRuleDef::MayActivateLoyaltyAnyTime) })]) }), AbilityDef::deck_construction("Teferi, Temporal Archmage can be your commander.", DeckConstructionDef::MayBeCommander, "Commander designation is chosen before the game.")]),
);

// C14 33 — Daretti, Scrap Savant
pub(in crate::card::sets) static DARETTI_SCRAP_SAVANT_33: CardRecord = CardRecord::new(
    "Daretti, Scrap Savant",
    "cda40e31-ba99-4565-ad92-7c687ff44bd9",
    "Dan Murayama Scott",
    CardRules::new_planeswalker(mana_cost!("{3}{R}"), &["Daretti"], 3).with_supertype(CardSupertype::Legendary).with_abilities(&[AbilityDef::activated("+2: Discard up to two cards, then draw that many cards.", &[CostDef::Loyalty(2)], EffectDef::Choose(ChooseDef { binding: ObjectChoiceBindingDef::Objects(Binding!("daretti_discards")), unchosen: None, chooser: PlayerRefDef::EffectController, candidates: ObjectSetDef::Query(ObjectQueryDef::matching(ObjectPredicateDef::Any, &[ZoneKind::Hand], PlayerRelation::You)), exclude: None, minimum: 0, maximum: 2, visibility: ChoiceVisibilityDef::Private, then: &EffectDef::Sequence(&[EffectDef::Perform(GameActionDef::DiscardCards { object: EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("daretti_discards"))) }), abilities::draw_cards(ValueDef::BoundObjectCount(Binding!("daretti_discards")))]) })), AbilityDef::activated_with_targets("−2: Sacrifice an artifact. If you do, return target artifact card from your graveyard to the battlefield.", &[CostDef::Loyalty(-2)], &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::HasType(CardType::Artifact), zones: &[ZoneKind::Graveyard], controller: None, owner: Some(PlayerRelation::You) })], EffectDef::SacrificeOfChoice { player: EffectRecipientDef::Controller, object: ObjectPredicateDef::HasType(CardType::Artifact), count: ValueDef::Constant(1), then: Some(&EffectDef::move_to_zone(EffectRecipientDef::Target(TargetIndex::PRIMARY), ZoneKind::Battlefield, ZonePlacement::Top)), amount: crate::card::SacrificedAmountDef::Power, otherwise: None, optional: false }), AbilityDef::activated("−10: You get an emblem with \"Whenever an artifact is put into your graveyard from the battlefield, return that card to the battlefield at the beginning of the next end step.\"", &[CostDef::Loyalty(-10)], EffectDef::CreateEmblem { emblem: EmblemCharacteristics::new("Daretti, Scrap Savant emblem", &[AbilityDef::triggered("Whenever an artifact is put into your graveyard from the battlefield, return that card to the battlefield at the beginning of the next end step.", TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::OwnedBy(PlayerRelation::You)]), Some(ZoneKind::Battlefield), Some(ZoneKind::Graveyard)), EffectDef::BindObjects(BindObjectsDef { source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::One(ObjectRefDef::ZoneChangeResultOfTriggeringObject)), binding: Binding!("daretti_return"), then: &EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered("At the beginning of the next end step, return that card to the battlefield.", TriggerEventDef::StepBegins { step: TurnStepDef::End, player: PlayerRelation::Any }, EffectDef::move_to_zone(EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("daretti_return"))), ZoneKind::Battlefield, ZonePlacement::Top)))) }))]) }), AbilityDef::deck_construction("Daretti, Scrap Savant can be your commander.", DeckConstructionDef::MayBeCommander, "Commander designation is chosen before the game.")]),
);

// C14 34 — Dualcaster Mage
pub(in crate::card::sets) static DUALCASTER_MAGE_34: CardRecord = CardRecord::new(
    "Dualcaster Mage",
    "0b80c8a0-0870-4836-bee1-f4a805d119d6",
    "Matt Stewart",
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Human", "Wizard"], 2, 2).with_abilities(&[
abilities::flash(),
abilities::enters_trigger_with_targets("When this creature enters, copy target instant or sorcery spell. You may choose new targets for the copy.", &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::All(&[ObjectPredicateDef::Spell, ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Instant), ObjectPredicateDef::HasType(CardType::Sorcery)])]), zones: &[ZoneKind::Stack], controller: None, owner: None })], EffectDef::CopyStackObject(&CopyStackObjectDef { object: EffectRecipientDef::Target(TargetIndex::PRIMARY), controller: PlayerRefDef::EffectController, count: ValueDef::Constant(1), retarget: true, colors: None }))
]),
);

// C14 50 — Titania, Protector of Argoth
pub(in crate::card::sets) static TITANIA_PROTECTOR_OF_ARGOTH: CardRecord =
    CardRecord::new(
    "Titania, Protector of Argoth",
    "224d904a-5972-4152-878a-9a922e7a55b6",
    "Magali Villeneuve",
// Five mana that gives a land back on the way in and then turns every
        // fetchland the deck was already playing into five power.
        CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Elemental"], 5, 3)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&[
                abilities::enters_trigger_with_targets(
                    "When Titania enters, return target land card from your graveyard to the battlefield.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::HasType(CardType::Land),
                            zones: &[ZoneKind::Graveyard],
                            controller: None,
                            owner: Some(PlayerRelation::You),
                        },
                    )],
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                ),
                AbilityDef::triggered(
                    "Whenever a land you control is put into a graveyard from the battlefield, create a 5/3 \
                     green Elemental creature token.",
                    TriggerEventDef::zone_changed(
                        // "A land you control", read as it leaves: the trigger is captured from
                        // the battlefield as it was a moment before, which is the only place a
                        // land that is now in a graveyard was ever controlled by anyone.
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ]),
                        Some(ZoneKind::Battlefield),
                        Some(ZoneKind::Graveyard),
                    ),
                    EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                        TokenCharacteristics::creature(&["Elemental"], &[ManaColor::Green], 5, 3).with_art(
                            CardArt::new("27440269-3b09-4010-8401-f159dc49a4cd", "Nils Hamm"),
                        ),
                    ))),
                ),
            ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &CONTAINMENT_PRIEST,
    &JAZAL_GOLDMANE,
    &TEFERI_TEMPORAL_ARCHMAGE_19,
    &DARETTI_SCRAP_SAVANT_33,
    &DUALCASTER_MAGE_34,
    &TITANIA_PROTECTOR_OF_ARGOTH,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
