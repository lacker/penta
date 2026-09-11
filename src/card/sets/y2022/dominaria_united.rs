//! Dominaria United cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::DrawEventMatcherDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::GraveyardPlayPermissionDef;
use crate::card::InstalledTriggerDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::PlayActionMatcherDef;
use crate::card::PlayRestrictionDef;
use crate::card::PlayerRelation;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "DMU",
    slug: "dominaria-united",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// DMU 2 — Anointed Peacekeeper
pub(in crate::card::sets) static ANOINTED_PEACEKEEPER: CardRecord = CardRecord::new(
    "Anointed Peacekeeper",
    "5b8127b5-3a65-411a-84bc-54e5c1be1477",
    "Tia Masic",
CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Cleric"], 3, 3).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::as_enters(
            "As this creature enters, look at an opponent's hand, then choose any card name.",
            crate::card::ReplacementEffectDef::Sequence(&[
                crate::card::ReplacementEffectDef::LookAtHand(PlayerRelation::Opponent),
                crate::card::ReplacementEffectDef::BindOutput {
                    binding: crate::Binding!("anointed_peacekeeper_name"),
                    effect: &abilities::choose_card_name_as_enters(
                        crate::card::CardNameSetDef::AllCardNames,
                    ),
                },
            ]),
        ),
        abilities::spell_cost_increase_for_name(
            "Spells your opponents cast with the chosen name cost {2} more to cast.",
            crate::card::CardNameDef::Binding(crate::Binding!("anointed_peacekeeper_name")),
            PlayerRelation::Opponent,
            mana_cost!("{2}"),
        ),
        abilities::ability_cost_increase_for_name(
            "Activated abilities of sources with the chosen name cost {2} more to activate unless they're mana abilities.",
            crate::card::CardNameDef::Binding(crate::Binding!("anointed_peacekeeper_name")),
            mana_cost!("{2}"),
        ),
    ]),
);

// DMU 24 — Leyline Binding
pub(in crate::card::sets) static LEYLINE_BINDING: CardRecord = CardRecord::new(
    "Leyline Binding",
    "3c3ac3dd-35db-447f-8674-37b4680a1ef7",
    "Cristi Balanescu",
// Six mana on paper and one in a deck with every basic land type, cast
    // at instant speed: the whole card is the mana base it asks for.
    CardRules::new_enchantment(mana_cost!("{5}{W}")).with_abilities(&[
        abilities::flash(),
        AbilityDef::static_ability(
            "Domain — This spell costs {1} less to cast for each basic land type among lands you \
             control.",
            EffectDef::ReduceGenericCostBy(ValueDef::BasicLandTypesControlled(PlayerRelation::You)),
        )
        .with_source_zones(&[ZoneKind::Hand]),
        abilities::enters_trigger_with_targets(
            "When this enchantment enters, exile target nonland permanent an opponent controls \
             until this enchantment leaves the battlefield.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::ExileLinkedToSource {
                    until_source_leaves: true,
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    face_down: false,
                    then: None,
                },
                // "Until this enchantment leaves the battlefield" is one printed clause, so
                // the return rides on a delayed trigger rather than appearing as a second
                // ability the card does not print.
                EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                    "When this enchantment leaves the battlefield, return the exiled card to the battlefield \
                     under its owner's control.",
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::Source,
                        Some(ZoneKind::Battlefield),
                        None,
                    ),
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

// DMU 28 — Prayer of Binding
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRAYER_OF_BINDING: CardRecord = CardRecord::new(
    "Prayer of Binding",
    "322f90b6-6b49-458d-9d5b-b601bfdd0af8",
    "Wylie Beckert",
    crate::card::CardRules::unsupported(),
);

// DMU 29 — Resolute Reinforcements
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RESOLUTE_REINFORCEMENTS: CardRecord = CardRecord::new(
    "Resolute Reinforcements",
    "3e11ad33-b9d7-43ef-840a-61955683b599",
    "Billy Christian",
    crate::card::CardRules::unsupported(),
);

// DMU 35 — Take Up the Shield
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TAKE_UP_THE_SHIELD: CardRecord = CardRecord::new(
    "Take Up the Shield",
    "851e842e-a497-4c36-90ee-8d64f806c378",
    "Manuel Castañón",
    crate::card::CardRules::unsupported(),
);

// DMU 57 — Micromancer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MICROMANCER: CardRecord = CardRecord::new(
    "Micromancer",
    "b21203c8-a935-4ce0-a742-148587e32145",
    "Ernanda Souza",
    crate::card::CardRules::unsupported(),
);

// DMU 64 — Shore Up
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHORE_UP: CardRecord = CardRecord::new(
    "Shore Up",
    "9d933bf1-14f0-4150-a0d2-6b845b9624cf",
    "Mark Behm",
    crate::card::CardRules::unsupported(),
);

// DMU 72 — Tolarian Terror
pub(in crate::card::sets) static TOLARIAN_TERROR: CardRecord = CardRecord::new(
    "Tolarian Terror",
    "42f01cba-43d4-46ad-b7a5-d7631b0e1347",
    "Vincent Christiaens",
// Seven mana on paper and two in practice, which is what makes ward the
    // relevant half: the deck that casts it cheaply is holding up counters.
    CardRules::new_creature(mana_cost!("{6}{U}"), &["Serpent"], 5, 5).with_abilities(&[
        AbilityDef::static_ability(
            "This spell costs {1} less to cast for each instant and sorcery card in your graveyard.",
            EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(
                &ObjectQueryDef::matching(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                ),
            )),
        )
        // Read from hand, where the cost is paid, rather than from the
        // battlefield the creature is heading to.
        .with_source_zones(&[ZoneKind::Hand]),
        abilities::ward(
            &[crate::CostDef::Mana(crate::ManaCost::new(2, 0))],
            "Ward {2} (Whenever this creature becomes the target of a spell or ability an opponent controls, counter it unless that player pays {2}.)",
        ),
    ]),
);

// DMU 89 — Cut Down
pub(in crate::card::sets) static CUT_DOWN: CardRecord = CardRecord::new(
    "Cut Down",
    "753db072-5d6a-4f37-8f7d-255572ecd3bd",
    "Dominik Mayer",
    // One black mana answers most of what an aggressive deck plays and
    // nothing of what a big one does, which is the whole design.
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target creature with total power and toughness 5 or less.",
        // "Total power and toughness 5 or less" is read live, so a creature that
        // was in range stops being a legal target the moment anything pumps it.
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::TotalPowerAndToughnessAtMost(5),
            ]),
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            then: None,
        },
    )),
);

// DMU 102 — Pilfer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PILFER: CardRecord = CardRecord::new(
    "Pilfer",
    "6d872c10-4126-4130-a74a-1331ed418ca8",
    "Pauline Voss",
    crate::card::CardRules::unsupported(),
);

// DMU 107 — Sheoldred, the Apocalypse
pub(in crate::card::sets) static SHEOLDRED_THE_APOCALYPSE: CardRecord = CardRecord::new(
    "Sheoldred, the Apocalypse",
    "d67be074-cdd4-41d9-ac89-0a0456c4e4b2",
    "Chris Rahn",
    // A four-mana 4/5 deathtouch would be playable on its own. The draw
    // clauses are what make it unanswerable: the opponent's own draw step
    // pays for it, every turn it survives.
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Phyrexian", "Praetor"], 4, 5)
        .with_supertype(CardSupertype::Legendary)
        // Two clauses rather than one symmetrical one, because they are not
        // symmetrical: yours gains and theirs loses, and a card that made both
        // players lose would read very differently.
        .with_abilities(&[
            abilities::deathtouch(),
            AbilityDef::triggered(
                "Whenever you draw a card, you gain 2 life.",
                TriggerEventDef::DrewCard(DrawEventMatcherDef::any(PlayerRelation::You)),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ),
            AbilityDef::triggered(
                "Whenever an opponent draws a card, they lose 2 life.",
                TriggerEventDef::DrewCard(DrawEventMatcherDef::any(PlayerRelation::Opponent)),
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::EventPlayer,
                    amount: ValueDef::Constant(2),
                },
            ),
        ]),
);

// DMU 137 — Lightning Strike (reprint)
const LIGHTNING_STRIKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2013::theros::LIGHTNING_STRIKE,
    "7d541125-bfb8-4f88-8bf3-ad7b6af7ad1d",
    "Marta Nael",
);

// DMU 155 — Bite Down
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BITE_DOWN: CardRecord = CardRecord::new(
    "Bite Down",
    "0eacd3de-b803-4322-8d88-d533761aa748",
    "Kitt Lapeña",
    crate::card::CardRules::unsupported(),
);

// DMU 172 — Magnigoth Sentry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGNIGOTH_SENTRY: CardRecord = CardRecord::new(
    "Magnigoth Sentry",
    "d939d4bc-b7e8-4ee8-b904-68f0bff0fde1",
    "Dave Kendall",
    crate::card::CardRules::unsupported(),
);

// DMU 183 — Tear Asunder
pub(in crate::card::sets) static TEAR_ASUNDER: CardRecord = CardRecord::new(
    "Tear Asunder",
    "629aa907-9533-4681-9bf2-9e56450a4cc2",
    "Dave Kendall",
// Two mana for the artifact or enchantment the deck was worried about,
    // or four for anything at all -- and exile rather than destruction,
    // which is what the extra mana is really paying for.
    CardRules::new_instant(mana_cost!("{1}{G}")).with_abilities(&[
        abilities::kicker(
            &[crate::CostDef::Mana(mana_cost!("{1}{B}"))],
        ),
        AbilityDef::spell_with_targets(
            "Exile target artifact or enchantment. If this spell was kicked, exile target nonland permanent instead.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::IfAdditionalCostPaid {
                    cost: crate::AdditionalCostIndex::PRIMARY,
                    // What four buys instead. "Instead" widens the one target.
                    if_paid: &AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                            CardType::Land,
                        )),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    // What two mana buys, on either side of the board.
                    otherwise: &AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                },
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Exile,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// DMU 196 — Balmor, Battlemage Captain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BALMOR_BATTLEMAGE_CAPTAIN: CardRecord = CardRecord::new(
    "Balmor, Battlemage Captain",
    "959ba62e-bb3a-49ad-8b1b-e787e413e5d4",
    "Bram Sels",
    crate::card::CardRules::unsupported(),
);

// DMU 200 — Garna, Bloodfist of Keld
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GARNA_BLOODFIST_OF_KELD: CardRecord = CardRecord::new(
    "Garna, Bloodfist of Keld",
    "294c5f08-08e7-458f-8838-ff321dc5d9f2",
    "Andrey Kuzinskiy",
    crate::card::CardRules::unsupported(),
);

// DMU 246 — Crystal Grotto
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRYSTAL_GROTTO: CardRecord = CardRecord::new(
    "Crystal Grotto",
    "bd250c9d-c65f-4293-a6b0-007fac634d3d",
    "Piotr Dura",
    crate::card::CardRules::unsupported(),
);

// DMU 282 — Serra Redeemer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SERRA_REDEEMER: CardRecord = CardRecord::new(
    "Serra Redeemer",
    "a8b9cb5c-29f2-46ed-803e-c2170955217c",
    "Joshua Raphael",
    crate::card::CardRules::unsupported(),
);

// DMU 339 — Ertai Resurrected
pub(in crate::card::sets) static ERTAI_RESURRECTED: CardRecord = CardRecord::new(
    "Ertai Resurrected",
    "2c46a2ca-27fd-44d4-80d0-7c83ed0a564e",
    "Justin Hernandez & Alexis Hernandez",
// A flash body that answers something on the way in, and pays for the
    // privilege with the card its victim's controller draws.
    CardRules::new_creature(
        mana_cost!("{2}{U}{B}"),
        &["Phyrexian", "Human", "Wizard"],
        3,
        2,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::flash(),
        AbilityDef::modal_triggered_up_to_one(
            "When this creature enters, choose up to one —\n• Counter target spell, activated \
             ability, or triggered ability. Its controller draws a card.\n• Destroy another \
             target creature or planeswalker. Its controller draws a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            // Both modes pay the same compensation, and both read it off the target
            // after that target is gone: the countered or destroyed object is retired
            // with its controller recorded, which is what "its controller" wants.
            &[
                AbilityDef::spell_with_targets(
                    "Counter target spell, activated ability, or triggered ability. Its controller draws a \
                     card.",
                    // "Spell, activated ability, or triggered ability" is every stack object
                    // there is: mana abilities never use the stack, so the wider slot needs no
                    // clause excluding them.
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::Any,
                            zones: &[ZoneKind::Stack],
                            controller: None,
                            owner: None,
                        },
                    )],
                    EffectDef::Sequence(&[
                        EffectDef::Counter {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            zone: ZoneKind::Graveyard,
                            placement: ZonePlacement::Top,
                        },
                        EffectDef::DrawCards {
                            recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                            amount: ValueDef::Constant(1),
                        },
                    ]),
                ),
                AbilityDef::spell_with_targets(
                    "Destroy another target creature or planeswalker. Its controller draws a card.",
                    // "Another" is the exclusion; Ertai himself has just arrived, so without it
                    // he would be a legal answer to his own trigger.
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::All(&[
                                ObjectPredicateDef::AnyOf(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                                ]),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            zones: &[ZoneKind::Battlefield],
                            controller: None,
                            owner: None,
                        },
                    )],
                    EffectDef::Sequence(&[
                        EffectDef::Destroy {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            then: None,
                        },
                        EffectDef::DrawCards {
                            recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                            amount: ValueDef::Constant(1),
                        },
                    ]),
                ),
            ],
        ),
    ]),
);

// DMU 387 — Leyline Binding (alternate printing)
const LEYLINE_BINDING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LEYLINE_BINDING,
    1,
    "32da8479-7d0e-4eb1-b18c-66eb170e31a5",
    "Cristi Balanescu",
);

// DMU 388 — Serra Paragon
pub(in crate::card::sets) static SERRA_PARAGON: CardRecord = CardRecord::new(
    "Serra Paragon",
    "69284b53-f712-418c-94a0-4e5638117256",
    "Heonhwa",
// Four mana for a 3/4 flier that buys back a land or a cheap permanent
    // every turn it lives, and pays two life for each one on its way out.
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Angel"], 3, 4)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::static_ability(
                "Once during each of your turns, you may play a land from your graveyard or cast a \
                 permanent spell with mana value 3 or less from your graveyard. If you do, it gains \
                 \"When this permanent is put into a graveyard from the battlefield, exile it and you \
                 gain 2 life.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Controller,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromGraveyard(
                        // "A land ... or a permanent spell with mana value 3 or less": one
                        // permission rather than two, because the once-each-turn bound is on the
                        // pair. Any play action, since which one it is follows from the card --
                        // nothing but a land is ever played as a land, and nothing but a spell is
                        // ever cast.
                        GraveyardPlayPermissionDef::once_each_of_your_turns(PlayRestrictionDef::new(
                            PlayActionMatcherDef::Any,
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Land),
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::Not(&ObjectPredicateDef::AnyOf(&[
                                        ObjectPredicateDef::HasType(CardType::Instant),
                                        ObjectPredicateDef::HasType(CardType::Sorcery),
                                    ])),
                                    ObjectPredicateDef::ManaValueAtMost(3),
                                ]),
                            ]),
                        ))
                            // What the permanent gains, and what makes the Paragon a value engine
                            // rather than a recursion loop: the card leaves for good, and the two life
                            // are the consolation.
                            .granting(&AppliedEffectDef::add_ability(&abilities::dies_trigger(
                                "When this permanent is put into a graveyard from the battlefield, exile it and you gain 2 \
                                 life.",
                                EffectDef::Sequence(&[
                                    EffectDef::move_to_zone(
                                        EffectRecipientDef::TriggeringZoneChangeResult,
                                        ZoneKind::Exile,
                                        ZonePlacement::Top,
                                    ),
                                    EffectDef::GainLife {
                                        recipient: EffectRecipientDef::Controller,
                                        amount: ValueDef::Constant(2),
                                    },
                                ]),
                            ))),
                    )),
                },
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANOINTED_PEACEKEEPER,
    &LEYLINE_BINDING,
    &PRAYER_OF_BINDING,
    &RESOLUTE_REINFORCEMENTS,
    &TAKE_UP_THE_SHIELD,
    &MICROMANCER,
    &SHORE_UP,
    &TOLARIAN_TERROR,
    &CUT_DOWN,
    &PILFER,
    &SHEOLDRED_THE_APOCALYPSE,
    &BITE_DOWN,
    &MAGNIGOTH_SENTRY,
    &TEAR_ASUNDER,
    &BALMOR_BATTLEMAGE_CAPTAIN,
    &GARNA_BLOODFIST_OF_KELD,
    &CRYSTAL_GROTTO,
    &SERRA_REDEEMER,
    &ERTAI_RESURRECTED,
    &SERRA_PARAGON,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] =
    &[LIGHTNING_STRIKE_REPRINT, LEYLINE_BINDING_ALTERNATE_1];
