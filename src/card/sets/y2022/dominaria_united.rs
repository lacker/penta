//! Dominaria United cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef, AppliedRuleDef,
    CardRules, CardSet, CardSupertype, CardType, DrawEventMatcherDef, EffectDef,
    EffectRecipientDef, GraveyardPlayPermissionDef, InstalledTriggerDef, ObjectPredicateDef,
    PlayActionMatcherDef, PlayRestrictionDef, PlayerRelation, TriggerEventDef, ValueDef, ZoneKind,
    ZonePlacement, abilities,
};
use crate::{TargetIndex, mana_cost};

// DMU 2 — Anointed Peacekeeper
pub(in crate::card::sets) static ANOINTED_PEACEKEEPER: CardRecord = CardRecord::new(
    crate::card::CardSet::DominariaUnited,
    "Anointed Peacekeeper",
    "5b8127b5-3a65-411a-84bc-54e5c1be1477",
    "Tia Masic",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Cleric"], 3, 3).with_abilities(&[
        abilities::vigilance(),
        abilities::look_at_opponent_hand_then_choose_card_name_as_enters(
            "As this creature enters, look at an opponent's hand, then choose any card name.",
        ),
        abilities::chosen_name_spell_cost_increase(
            "Spells your opponents cast with the chosen name cost {2} more to cast.",
            PlayerRelation::Opponent,
            mana_cost!("{2}"),
        ),
        abilities::chosen_name_ability_cost_increase(
            "Activated abilities of sources with the chosen name cost {2} more to activate unless they're mana abilities.",
            mana_cost!("{2}"),
        ),
    ]),
);

// DMU 24 — Leyline Binding
pub(in crate::card::sets) static LEYLINE_BINDING: CardRecord = CardRecord::new(
    CardSet::DominariaUnited,
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

// DMU 72 — Tolarian Terror
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOLARIAN_TERROR: CardRecord = CardRecord::new(
    crate::card::CardSet::DominariaUnited,
    "Tolarian Terror",
    "42f01cba-43d4-46ad-b7a5-d7631b0e1347",
    "Vincent Christiaens",
    crate::card::CardRules::unsupported(),
);

// DMU 89 — Cut Down
pub(in crate::card::sets) static CUT_DOWN: CardRecord = CardRecord::new(
    CardSet::DominariaUnited,
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
            can_regenerate: true,
            then: None,
        },
    )),
);

// DMU 107 — Sheoldred, the Apocalypse
pub(in crate::card::sets) static SHEOLDRED_THE_APOCALYPSE: CardRecord = CardRecord::new(
    CardSet::DominariaUnited,
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

// DMU 183 — Tear Asunder
pub(in crate::card::sets) static TEAR_ASUNDER: CardRecord = CardRecord::new(
    CardSet::DominariaUnited,
    "Tear Asunder",
    "629aa907-9533-4681-9bf2-9e56450a4cc2",
    "Dave Kendall",
    // Two mana for the artifact or enchantment the deck was worried about,
    // or four for anything at all -- and exile rather than destruction,
    // which is what the extra mana is really paying for.
    CardRules::new_instant(mana_cost!("{1}{G}")).with_abilities(&[
        abilities::kicker(mana_cost!("{1}{B}")),
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
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// DMU 339 — Ertai Resurrected
pub(in crate::card::sets) static ERTAI_RESURRECTED: CardRecord = CardRecord::new(
    CardSet::DominariaUnited,
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
                            can_regenerate: true,
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
    crate::card::CardSet::DominariaUnited,
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
                                    EffectDef::MoveToZone {
                                        object: EffectRecipientDef::TriggeringZoneChangeResult,
                                        zone: ZoneKind::Exile,
                                        placement: ZonePlacement::Top,
                                    },
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
    &TOLARIAN_TERROR,
    &CUT_DOWN,
    &SHEOLDRED_THE_APOCALYPSE,
    &TEAR_ASUNDER,
    &ERTAI_RESURRECTED,
    &SERRA_PARAGON,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] =
    &[LIGHTNING_STRIKE_REPRINT, LEYLINE_BINDING_ALTERNATE_1];
