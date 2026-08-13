//! Avacyn Restored card records used by the built-in ISD–RTR Standard deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::sets::y1993::alpha;
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    AddManaEffectDef, AppliedEffectDef, CardArt, CardBehavior, CardRules, CardSet, CardSupertype,
    CardType, ComparisonDef, CounterKind, DiscardSelectionDef, DividedTotal, EffectDef,
    EffectDurationDef, EffectRecipientDef, ManaColor, ManaRestrictionDef, ManaSpendEffectDef,
    ObjectPredicateDef, ObjectQueryDef, PlayerRelation, TriggerConditionDef, TriggerEventDef,
    TurnStepDef, ValueDef, ZoneKind, ZonePlacement, abilities, cards,
};
use crate::{TargetIndex, mana_cost};

// AVR 1 — Angel of Glory's Rise
// Audit: blocked — Needs simultaneous batch movement for all Zombies and all returned Humans rather than processing each object as a separate zone change.

// AVR 2 — Angel of Jubilation
// Audit: blocked — Needs a static prohibition on paying life or sacrificing creatures specifically to cast spells and activate abilities.

// AVR 3 — Angel's Mercy
pub(in crate::card::sets) static ANGELS_MERCY: CardRecord = CardRecord::new(
    cards::ANGELS_MERCY,
    "Angel's Mercy",
    CardArt::new("7a437999-26ae-49fa-8647-c8c2b4640702", "Greg Staples"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{2}{W}{W}")).with_ability(AbilityDef::spell(
        "You gain 7 life.",
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(7),
        },
    )),
);

// AVR 4 — Angelic Wall
pub(in crate::card::sets) static ANGELIC_WALL: CardRecord = CardRecord::new(
    cards::ANGELIC_WALL,
    "Angelic Wall",
    CardArt::new("d7b2450d-87a7-46dc-b43a-2db2abeca44f", "Allen Williams"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Wall"], 0, 4)
        .with_abilities(&[abilities::defender(), abilities::flying()]),
);

// AVR 5 — Archangel
pub(in crate::card::sets) static ARCHANGEL: CardRecord = CardRecord::new(
    cards::ARCHANGEL,
    "Archangel",
    CardArt::new("3741b2a7-7bda-481a-b8f8-9b04c96035b0", "Cynthia Sheppard"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{5}{W}{W}"), &["Angel"], 5, 5)
        .with_abilities(&[abilities::flying(), abilities::vigilance()]),
);

// AVR 6 — Avacyn, Angel of Hope
pub(in crate::card::sets) static AVACYN_ANGEL_OF_HOPE: CardRecord = CardRecord::new(
    cards::AVACYN_ANGEL_OF_HOPE,
    "Avacyn, Angel of Hope",
    CardArt::new("ba149706-cd17-4da6-8403-ccfe2d6cb437", "Jason Chan"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{5}{W}{W}{W}"), &["Angel"], 8, 8)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            abilities::vigilance(),
            abilities::indestructible(),
            AbilityDef::static_ability(
                "Other permanents you control have indestructible.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::MatchingObjects {
                        object: ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        zones: &[ZoneKind::Battlefield],
                        controller: PlayerRelation::You,
                    },
                    effect: AppliedEffectDef::GrantAbility(&abilities::indestructible()),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// AVR 7 — Banishing Stroke
pub(in crate::card::sets) static BANISHING_STROKE: CardRecord = CardRecord::new(
    cards::BANISHING_STROKE,
    "Banishing Stroke",
    CardArt::new("238d8437-1abd-4bb7-8b5b-54f959bc2c79", "Igor Kieryluk"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{5}{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Put target artifact, creature, or enchantment on the bottom of its owner's library.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                ]),
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Library,
                placement: ZonePlacement::Bottom,
                controller: None,
            },
        ),
        abilities::miracle(mana_cost!("{W}")),
    ]),
);

// AVR 8 — Builder's Blessing
// Audit: blocked — Needs a continuous-effect recipient predicate for creatures that are currently untapped.

// AVR 9 — Call to Serve
// Audit: blocked — Needs an attachment-scoped effect that adds the Angel subtype without replacing the creature's existing types.

// AVR 10 — Cathars' Crusade
pub(in crate::card::sets) static CATHARS_CRUSADE: CardRecord = CardRecord::new(
    cards::CATHARS_CRUSADE,
    "Cathars' Crusade",
    CardArt::new("78154978-9e7d-44e9-a03f-c578072a8ff7", "Karl Kopinski"),
    CardSet::AvacynRestored,
    CardRules::new_enchantment(mana_cost!("{3}{W}{W}")).with_ability(AbilityDef::triggered(
        "Whenever a creature you control enters, put a +1/+1 counter on each creature you control.",
        TriggerEventDef::ZoneChanged {
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ]),
            from: None,
            to: Some(ZoneKind::Battlefield),
        },
        EffectDef::AddCounters {
            object: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::You,
            },
            kind: CounterKind::PlusOnePlusOne,
            amount: ValueDef::Constant(1),
        },
    )),
);

// AVR 11 — Cathedral Sanctifier
pub(in crate::card::sets) static CATHEDRAL_SANCTIFIER: CardRecord = CardRecord::new(
    cards::CATHEDRAL_SANCTIFIER,
    "Cathedral Sanctifier",
    CardArt::new("76cac47a-9e83-4039-8d80-fa9bdadb7527", "Michael C. Hayes"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Cleric"], 1, 1).with_ability(
        AbilityDef::triggered(
            "When this creature enters, you gain 3 life.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ),
    ),
);

// AVR 12 — Cloudshift
// Audit: blocked — Linked exile returns a blinked permanent under its owner rather than preserving the spell controller required for a stolen creature.

static COMMANDERS_AUTHORITY_UPKEEP: AbilityDef = AbilityDef::triggered(
    "At the beginning of your upkeep, create a 1/1 white Human creature token.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    EffectDef::CreateToken {
        token: cards::HUMAN_TOKEN_1_1_WHITE,
        count: ValueDef::Constant(1),
    },
);

// AVR 13 — Commander's Authority
pub(in crate::card::sets) static COMMANDERS_AUTHORITY: CardRecord = CardRecord::new(
    cards::COMMANDERS_AUTHORITY,
    "Commander's Authority",
    CardArt::new("08ef4383-11e7-4426-a04a-058570f46e47", "Johannes Voss"),
    CardSet::AvacynRestored,
    CardRules::new_enchantment(mana_cost!("{4}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature has \"At the beginning of your upkeep, create a 1/1 white Human creature token.\"",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::GrantAbility(&COMMANDERS_AUTHORITY_UPKEEP),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// AVR 14 — Cursebreak
pub(in crate::card::sets) static CURSEBREAK: CardRecord = CardRecord::new(
    cards::CURSEBREAK,
    "Cursebreak",
    CardArt::new("c71a0883-316c-4870-a029-25f16952fbc0", "Sam Wolfe Connelly"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target enchantment. You gain 2 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Enchantment),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ]),
    )),
);

// AVR 15 — Defang
// Audit: blocked — Needs a static prevention effect for all damage dealt by the enchanted creature, not merely combat damage for one turn.

// AVR 16 — Defy Death
// Audit: blocked — Needs a continuation that retains the moved graveyard target's new object identity for the Angel test and +1/+1 counters.

// AVR 17 — Devout Chaplain
// Audit: blocked — Needs an activation cost that taps two separately chosen untapped Humans you control.

// AVR 18 — Divine Deflection
// Audit: blocked — Needs a duration-scoped prevention shield that tracks the amount prevented and redirects exactly that amount to a chosen target.

// AVR 19 — Emancipation Angel
// Audit: blocked — Needs a resolving non-target choice of a permanent you control to return to its owner's hand.

// AVR 20 — Entreat the Angels
pub(in crate::card::sets) static ENTREAT_THE_ANGELS: CardRecord = CardRecord::new(
    cards::ENTREAT_THE_ANGELS,
    "Entreat the Angels",
    CardArt::new("31292616-70e6-4d19-a883-e63ad860f50c", "Todd Lockwood"),
    CardSet::AvacynRestored,
    CardRules::new_sorcery(mana_cost!("{X}{X}{W}{W}{W}")).with_abilities(&[
        AbilityDef::spell(
            "Create X 4/4 white Angel creature tokens with flying.",
            EffectDef::CreateToken {
                token: cards::ANGEL_TOKEN_4_4_WHITE,
                count: ValueDef::ChosenX,
            },
        ),
        abilities::miracle(mana_cost!("{X}{W}{W}")),
    ]),
);

// AVR 21 — Farbog Explorer
// Audit: blocked — Needs swampwalk and its defending-player land/blocking semantics.

// AVR 22 — Goldnight Commander
pub(in crate::card::sets) static GOLDNIGHT_COMMANDER: CardRecord = CardRecord::new(
    cards::GOLDNIGHT_COMMANDER,
    "Goldnight Commander",
    CardArt::new("c6ebec82-9d4a-4e78-b923-37c3a52133e7", "Chris Rahn"),
    CardSet::AvacynRestored,
    CardRules::new_creature(
        mana_cost!("{3}{W}"),
        &["Human", "Cleric", "Soldier"],
        2,
        2,
    )
    .with_ability(AbilityDef::triggered(
        "Whenever another creature you control enters, creatures you control get +1/+1 until end of turn.",
        TriggerEventDef::ZoneChanged {
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
            ]),
            from: None,
            to: Some(ZoneKind::Battlefield),
        },
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::You,
            },
            effect: AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Constant(1),
                toughness: ValueDef::Constant(1),
            },
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// AVR 23 — Goldnight Redeemer
// Audit: blocked — Needs multiplication of a matching-creature count by two for the life-gain amount.

// AVR 24 — Herald of War
// Audit: blocked — Needs a battlefield static cost reduction for other Angel and Human spells whose amount is the source's +1/+1-counter count.

static HOLY_JUSTICIAR_ZOMBIE: TriggerConditionDef = TriggerConditionDef::TargetMatches {
    slot: TargetIndex::PRIMARY,
    object: ObjectPredicateDef::Subtype("Zombie"),
};

// AVR 25 — Holy Justiciar
pub(in crate::card::sets) static HOLY_JUSTICIAR: CardRecord = CardRecord::new(
    cards::HOLY_JUSTICIAR,
    "Holy Justiciar",
    CardArt::new("640cad49-1db3-4611-a80d-7ce95f000fad", "David Rapoza"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Cleric"], 2, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{2}{W}, {T}: Tap target creature. If that creature is a Zombie, exile it.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{W}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::IfCondition {
                    condition: &HOLY_JUSTICIAR_ZOMBIE,
                    then: &EffectDef::MoveToZone {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        zone: ZoneKind::Exile,
                        placement: ZonePlacement::Top,
                        controller: None,
                    },
                },
            ]),
        ),
    ),
);

// AVR 26 — Leap of Faith
// Audit: blocked — Needs a duration-scoped prevention effect for all damage to the target, not only combat damage.

// AVR 27 — Midnight Duelist
// Audit: blocked — Needs protection from the Vampire creature subtype rather than from a color.

// AVR 28 — Midvast Protector
// Audit: blocked — Needs a resolving color choice and a duration-scoped protection ability parameterized by that choice.

// AVR 29 — Moonlight Geist
pub(in crate::card::sets) static MOONLIGHT_GEIST: CardRecord = CardRecord::new(
    cards::MOONLIGHT_GEIST,
    "Moonlight Geist",
    CardArt::new("4cf4c4cf-df35-4725-81ca-d62b70b8d0dd", "Dan Murayama Scott"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Spirit"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{3}{W}: Prevent all combat damage that would be dealt to and dealt by this creature this turn.",
            &[AbilityCostDef::Mana(mana_cost!("{3}{W}"))],
            EffectDef::PreventCombatDamageThisTurn {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// AVR 30 — Moorland Inquisitor
pub(in crate::card::sets) static MOORLAND_INQUISITOR: CardRecord = CardRecord::new(
    cards::MOORLAND_INQUISITOR,
    "Moorland Inquisitor",
    CardArt::new("581dbbea-9995-4e4b-ba5c-d6d5597e4ace", "David Palumbo"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Soldier"], 2, 2).with_ability(
        AbilityDef::activated(
            "{2}{W}: This creature gains first strike until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::GrantAbility(&abilities::first_strike()),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// AVR 31 — Nearheath Pilgrim
// Audit: blocked — Needs soulbond pairing state, paired-object identity, and conditional ability grants to both paired creatures.

// AVR 32 — Restoration Angel
// Audit: partial — Linked exile returns a blinked permanent under its owner rather than this ability's controller when the target was stolen.
pub(in crate::card::sets) static RESTORATION_ANGEL: CardRecord = CardRecord::new(
    cards::RESTORATION_ANGEL,
    "Restoration Angel",
    CardArt::new("c2ad8639-e586-47f4-baca-2a1af5aa281b", "Johannes Voss"),
    CardSet::AvacynRestored,
    CardRules::new_creature(
        mana_cost!("{3}{W}"),
        &["Angel"],
        3,
        4,
    )
    .with_abilities(&[
        abilities::flash(),
        abilities::flying(),
        AbilityDef::triggered_with_targets("When this creature enters, you may exile target non-Angel creature you control, then return that card to the battlefield under your control.", TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            }, &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Angel")),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
        )], // The exile links the card to this Angel and the return drains
            // that link immediately, so the creature blinks within one
            // resolution. The card comes back under its owner's control,
            // which is the printed controller for every creature this can
            // legally target unless control of it was already stolen.
            EffectDef::May(&EffectDef::Sequence(&[
                EffectDef::ExileLinkedToSource {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::ReturnLinkedExiles {
                    zone: ZoneKind::Battlefield,
                    grant: None,
                },
            ])))
            .with_coverage(AbilityCoverageDef::partial(
                "Linked exile returns a stolen target under its owner rather than this ability's controller.",
            )),
    ]),
);

// AVR 33 — Riders of Gavony
// Audit: blocked — Needs protection from creatures of a dynamically chosen creature type.

// AVR 34 — Righteous Blow
pub(in crate::card::sets) static RIGHTEOUS_BLOW: CardRecord = CardRecord::new(
    cards::RIGHTEOUS_BLOW,
    "Righteous Blow",
    CardArt::new("9b640fdc-7a19-475e-858f-e159f61e154e", "Clint Cearley"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Righteous Blow deals 2 damage to target attacking or blocking creature.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::AttackingOrBlocking,
            ]),
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(2),
        },
    )),
);

// AVR 35 — Seraph of Dawn
pub(in crate::card::sets) static SERAPH_OF_DAWN: CardRecord = CardRecord::new(
    cards::SERAPH_OF_DAWN,
    "Seraph of Dawn",
    CardArt::new("5da345bd-8f2b-4966-97f5-c0e4c6cfe3b7", "Todd Lockwood"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Angel"], 2, 4)
        .with_abilities(&[abilities::flying(), abilities::lifelink()]),
);

// AVR 36 — Silverblade Paladin
// Audit: blocked — Needs soulbond pairing state, paired-object identity, and a conditional double-strike grant to both paired creatures.

// AVR 37 — Spectral Gateguards
// Audit: blocked — Needs soulbond pairing state, paired-object identity, and a conditional vigilance grant to both paired creatures.

// AVR 38 — Terminus
// Audit: partial — Permanents move to library one at a time, and the engine does not offer each owner the required ordering choice for multiple cards put on the bottom.
pub(in crate::card::sets) static TERMINUS: CardRecord = CardRecord::new(
    cards::TERMINUS,
    "Terminus",
    CardArt::new("0982ea7e-05a4-4e40-98ab-ea9aa6c7342e", "James Paick"),
    CardSet::AvacynRestored,
    CardRules::new_sorcery(mana_cost!("{4}{W}{W}")).with_abilities(&[
        AbilityDef::spell(
            "Put all creatures on the bottom of their owners' libraries.",
            EffectDef::MoveToZone {
                object: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::Any,
                },
                zone: ZoneKind::Library,
                controller: None,
                placement: ZonePlacement::Bottom,
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "Permanents move to library one at a time, and the engine does not offer each owner the required ordering choice for multiple cards put on the bottom.",
        )),
        abilities::miracle(mana_cost!("{W}")),
    ]),
);

// AVR 39 — Thraben Valiant
pub(in crate::card::sets) static THRABEN_VALIANT: CardRecord = CardRecord::new(
    cards::THRABEN_VALIANT,
    "Thraben Valiant",
    CardArt::new("20558f69-9240-49b9-9695-caf75ee2db1b", "Jason Chan"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Soldier"], 2, 1)
        .with_ability(abilities::vigilance()),
);

// AVR 40 — Voice of the Provinces
pub(in crate::card::sets) static VOICE_OF_THE_PROVINCES: CardRecord = CardRecord::new(
    cards::VOICE_OF_THE_PROVINCES,
    "Voice of the Provinces",
    CardArt::new("b785276b-3778-49f3-b46f-a1f3d91db097", "Igor Kieryluk"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{4}{W}{W}"), &["Angel"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "When this creature enters, create a 1/1 white Human creature token.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::CreateToken {
                token: cards::HUMAN_TOKEN_1_1_WHITE,
                count: ValueDef::Constant(1),
            },
        ),
    ]),
);

// AVR 41 — Zealous Strike
pub(in crate::card::sets) static ZEALOUS_STRIKE: CardRecord = CardRecord::new(
    cards::ZEALOUS_STRIKE,
    "Zealous Strike",
    CardArt::new("ae8a01fb-dd47-44de-b528-8b7ca4b3388b", "Bud Cook"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +2/+2 and gains first strike until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(2),
                    toughness: ValueDef::Constant(2),
                },
                AppliedEffectDef::GrantAbility(&abilities::first_strike()),
            ]),
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// AVR 42 — Alchemist's Apprentice
pub(in crate::card::sets) static ALCHEMISTS_APPRENTICE: CardRecord = CardRecord::new(
    cards::ALCHEMISTS_APPRENTICE,
    "Alchemist's Apprentice",
    CardArt::new("31abba67-1241-4fb3-88b5-4c4668ec5f25", "David Palumbo"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Human", "Wizard"], 1, 1).with_ability(
        AbilityDef::activated(
            "Sacrifice this creature: Draw a card.",
            &[AbilityCostDef::SacrificeSource],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// AVR 43 — Amass the Components
// Audit: blocked — Needs a resolving hand-card choice after drawing and a continuation that puts the chosen card on the bottom of its owner's library.

// AVR 44 — Arcane Melee
// Audit: blocked — Needs a battlefield-wide generic cost reduction for instant and sorcery spells cast by every player.

// AVR 45 — Captain of the Mists
// Audit: blocked — Needs a tap-or-untap choice on a single activated ability; the shared modal vocabulary currently covers spells only.

// AVR 46 — Crippling Chill
// Audit: blocked — Needs a duration tied to the targeted creature's controller's next untap step.

// AVR 47 — Deadeye Navigator
// Audit: blocked — Needs soulbond pairing state, paired-object identity, and an activated blink ability granted to both paired creatures.

// AVR 48 — Devastation Tide
// Audit: blocked — Needs simultaneous batch movement for all nonland permanents rather than processing each battlefield exit separately.

static DREADWATERS_LANDS: ObjectQueryDef = ObjectQueryDef {
    object: ObjectPredicateDef::HasType(CardType::Land),
    zones: &[ZoneKind::Battlefield],
    controller: PlayerRelation::You,
};

// AVR 49 — Dreadwaters
pub(in crate::card::sets) static DREADWATERS: CardRecord = CardRecord::new(
    cards::DREADWATERS,
    "Dreadwaters",
    CardArt::new("88245a41-d4d5-46bf-969f-48d4dd540e2c", "Cliff Childs"),
    CardSet::AvacynRestored,
    CardRules::new_sorcery(mana_cost!("{3}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target player mills X cards, where X is the number of lands you control.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Mill {
            player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::CountMatchingObjects(&DREADWATERS_LANDS),
        },
    )),
);

// AVR 50 — Elgaud Shieldmate
// Audit: blocked — Needs soulbond pairing state, paired-object identity, and a conditional hexproof grant to both paired creatures.

// AVR 51 — Favorable Winds
// Audit: blocked — The flying predicate used by static effects ignores flying granted or removed by other static continuous effects.

// AVR 52 — Fettergeist
// Audit: blocked — Needs an unless-payment whose mana amount is the dynamic count of other creatures you control.

// AVR 53 — Fleeting Distraction
pub(in crate::card::sets) static FLEETING_DISTRACTION: CardRecord = CardRecord::new(
    cards::FLEETING_DISTRACTION,
    "Fleeting Distraction",
    CardArt::new("1ba49d16-e3e4-470a-8ca2-a93a5b358f6e", "Ryan Yee"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets -1/-0 until end of turn. Draw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(-1),
                    toughness: ValueDef::Constant(0),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// AVR 54 — Galvanic Alchemist
// Audit: blocked — Needs soulbond pairing state, paired-object identity, and an activated untap ability granted to both paired creatures.

// AVR 55 — Geist Snatch
pub(in crate::card::sets) static GEIST_SNATCH: CardRecord = CardRecord::new(
    cards::GEIST_SNATCH,
    "Geist Snatch",
    CardArt::new("b6dac5db-ef96-4bd5-aabc-e5ae2b95c8c3", "Dan Murayama Scott"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{2}{U}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target creature spell. Create a 1/1 blue Spirit creature token with flying.",
        &[AbilityTargetDef::exactly_one_spell(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Graveyard,
            },
            EffectDef::CreateToken {
                token: cards::SPIRIT_TOKEN_1_1_BLUE,
                count: ValueDef::Constant(1),
            },
        ]),
    )),
);

// AVR 56 — Ghostform
pub(in crate::card::sets) static GHOSTFORM: CardRecord = CardRecord::new(
    cards::GHOSTFORM,
    "Ghostform",
    CardArt::new("1f6a20ba-6691-4844-9685-dfcd4184224e", "Scott Chou"),
    CardSet::AvacynRestored,
    CardRules::new_sorcery(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Up to two target creatures can't be blocked this turn.",
        &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
            2,
        )],
        EffectDef::MakeUnblockableThisTurn {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    )),
);

// AVR 57 — Ghostly Flicker
// Audit: blocked — Linked exile returns stolen permanents under their owners rather than the spell controller, and the effect must preserve that controller for both targets.

// AVR 58 — Ghostly Touch
// Audit: blocked — Needs a tap-or-untap choice inside the triggered ability granted by an Aura.

// AVR 59 — Gryff Vanguard
pub(in crate::card::sets) static GRYFF_VANGUARD: CardRecord = CardRecord::new(
    cards::GRYFF_VANGUARD,
    "Gryff Vanguard",
    CardArt::new("b7238136-c8de-4949-9b54-ff75094e0569", "Jason Chan"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Human", "Knight"], 3, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "When this creature enters, draw a card.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// AVR 60 — Havengul Skaab
// Audit: blocked — Needs a resolving non-target choice of another creature you control to return to its owner's hand.

// AVR 61 — Infinite Reflection
// Audit: blocked — Needs attachment-derived copy effects for existing creatures and an entry replacement that copies the currently enchanted creature.

// AVR 62 — Into the Void
pub(in crate::card::sets) static INTO_THE_VOID: CardRecord = CardRecord::new(
    cards::INTO_THE_VOID,
    "Into the Void",
    CardArt::new("5ddd1050-8abd-4dfe-9e52-5b56af358653", "Daarken"),
    CardSet::AvacynRestored,
    CardRules::new_sorcery(mana_cost!("{3}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return up to two target creatures to their owners' hands.",
        &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
            2,
        )],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            controller: None,
        },
    )),
);

// AVR 63 — Latch Seeker
pub(in crate::card::sets) static LATCH_SEEKER: CardRecord = CardRecord::new(
    cards::LATCH_SEEKER,
    "Latch Seeker",
    CardArt::new("3e4e7589-9cee-4d57-8648-ce733781bfb2", "Vincent Proce"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Spirit"], 3, 1).with_ability(
        AbilityDef::static_ability(
            "This creature can't be blocked.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::CannotBeBlockedBy(ObjectPredicateDef::Any),
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
    ),
);

// AVR 64 — Lone Revenant
// Audit: blocked — Needs ordered bottom-of-library placement for the unchosen cards after the conditional top-four selection.

// AVR 65 — Lunar Mystic
pub(in crate::card::sets) static LUNAR_MYSTIC: CardRecord = CardRecord::new(
    cards::LUNAR_MYSTIC,
    "Lunar Mystic",
    CardArt::new("f346d236-528c-4164-9995-74cdc56597a9", "Wesley Burt"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Human", "Wizard"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever you cast an instant spell, you may pay {1}. If you do, draw a card.",
            TriggerEventDef::SpellCast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Instant),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::OptionalPayment {
                payment: crate::card::PaymentDef::new(
                    PlayerRelation::You,
                    &[AbilityCostDef::Mana(mana_cost!("{1}"))],
                ),
                if_paid: &EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            },
        ),
    ),
);

static MASS_APPEAL_HUMANS: ObjectQueryDef = ObjectQueryDef {
    object: ObjectPredicateDef::Subtype("Human"),
    zones: &[ZoneKind::Battlefield],
    controller: PlayerRelation::You,
};

// AVR 66 — Mass Appeal
pub(in crate::card::sets) static MASS_APPEAL: CardRecord = CardRecord::new(
    cards::MASS_APPEAL,
    "Mass Appeal",
    CardArt::new(
        "dfe9ae51-fd2b-45ca-a780-725f51f897b2",
        "Christopher Moeller",
    ),
    CardSet::AvacynRestored,
    CardRules::new_sorcery(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell(
        "Draw a card for each Human you control.",
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::CountMatchingObjects(&MASS_APPEAL_HUMANS),
        },
    )),
);

// AVR 67 — Mist Raven
pub(in crate::card::sets) static MIST_RAVEN: CardRecord = CardRecord::new(
    cards::MIST_RAVEN,
    "Mist Raven",
    CardArt::new("0d98f0c4-021a-407a-8b0c-5500d804f959", "John Avon"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Bird"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "When this creature enters, return target creature to its owner's hand.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
                controller: None,
            },
        ),
    ]),
);

// AVR 68 — Misthollow Griffin
// Audit: blocked — Needs a cast permission and play-option source zone for casting this card from exile.

// AVR 69 — Nephalia Smuggler
// Audit: blocked — Linked exile returns a stolen target under its owner rather than the ability controller required by the card.

// AVR 70 — Outwit
// Audit: blocked — Needs a stack-object predicate for a spell that currently targets a player.

// AVR 71 — Peel from Reality
pub(in crate::card::sets) static PEEL_FROM_REALITY: CardRecord = CardRecord::new(
    cards::PEEL_FROM_REALITY,
    "Peel from Reality",
    CardArt::new("7f41285b-5961-4653-96a0-fb6d27111390", "Jason Felix"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(
        AbilityDef::spell_with_targets(
            "Return target creature you control and target creature you don't control to their owners' hands.",
            &[
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                }),
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                }),
            ],
            EffectDef::Sequence(&[
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    controller: None,
                },
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex(1)),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    controller: None,
                },
            ]),
        ),
    ),
);

// AVR 72 — Rotcrown Ghoul
pub(in crate::card::sets) static ROTCROWN_GHOUL: CardRecord = CardRecord::new(
    cards::ROTCROWN_GHOUL,
    "Rotcrown Ghoul",
    CardArt::new("f13b5ba6-0de1-4f5c-867b-57e2c10bde8e", "Dave Kendall"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Zombie"], 3, 3).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature dies, target player mills five cards.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: Some(ZoneKind::Battlefield),
                to: Some(ZoneKind::Graveyard),
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(5),
            },
        ),
    ),
);

// AVR 73 — Scrapskin Drake
// Audit: blocked — Needs a combat declaration restriction allowing this creature to block only creatures with flying.

// AVR 74 — Second Guess
// Audit: blocked — Needs a target predicate or casting-history relation for the second spell cast during the current turn.

// AVR 75 — Spectral Prison
// Audit: blocked — Needs an event for the enchanted creature becoming the target of a spell and an attachment-derived event subject.

// AVR 76 — Spirit Away
// Audit: blocked — Needs an attachment-scoped continuous control-changing effect.

// AVR 77 — Stern Mentor
// Audit: blocked — Needs soulbond pairing state, paired-object identity, and a targeted mill ability granted to both paired creatures.

// AVR 78 — Stolen Goods
// Audit: blocked — Needs repeat-until library exile plus temporary permission to cast the resulting card without paying its mana cost.

// AVR 79 — Tamiyo, the Moon Sage
// Audit: blocked — Needs next-untap-step duration, a tapped-creature count, maximum-hand-size modification, and graveyard-entry triggers from every zone.

// AVR 80 — Tandem Lookout
// Audit: blocked — Needs soulbond pairing state, paired-object identity, and a damage trigger granted to both paired creatures.

// AVR 81 — Temporal Mastery
// Audit: blocked — Needs declarative extra-turn scheduling.

// AVR 82 — Vanishment
pub(in crate::card::sets) static VANISHMENT: CardRecord = CardRecord::new(
    cards::VANISHMENT,
    "Vanishment",
    CardArt::new("dece40c1-790c-4471-a790-1d356b345603", "Daarken"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{4}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Put target nonland permanent on top of its owner's library.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Library,
                placement: ZonePlacement::Top,
                controller: None,
            },
        ),
        abilities::miracle(mana_cost!("{U}")),
    ]),
);

// AVR 83 — Wingcrafter
// Audit: blocked — Needs soulbond pairing state, paired-object identity, and a conditional flying grant to both paired creatures.

// AVR 84 — Appetite for Brains
// Audit: blocked — Needs a hidden-hand card choice constrained by mana value, followed by exile of the chosen card.

// AVR 85 — Barter in Blood
// Audit: blocked — Needs each player to make one resolving choice of two creatures, or a continuation across two sacrifice choices.

// AVR 86 — Blood Artist
pub(in crate::card::sets) static BLOOD_ARTIST: CardRecord = CardRecord::new(
    cards::BLOOD_ARTIST,
    "Blood Artist",
    CardArt::new("2e1fb442-68ff-4249-8e44-87edf6fae211", "Johannes Voss"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Vampire"], 0, 1).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever this creature or another creature dies, target player loses 1 life and you gain 1 life.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                from: Some(ZoneKind::Battlefield),
                to: Some(ZoneKind::Graveyard),
            },
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                PlayerRelation::Any,
            ))],
            EffectDef::Sequence(&[
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ),
);

// AVR 87 — Bloodflow Connoisseur
pub(in crate::card::sets) static BLOODFLOW_CONNOISSEUR: CardRecord = CardRecord::new(
    cards::BLOODFLOW_CONNOISSEUR,
    "Bloodflow Connoisseur",
    CardArt::new("97485dbf-2f31-4ed2-a6cd-529ca22c9ac5", "Slawomir Maniak"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Vampire"], 1, 1).with_ability(
        AbilityDef::activated(
            "Sacrifice a creature: Put a +1/+1 counter on this creature.",
            &[AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::You,
            }],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// AVR 88 — Bone Splinters
// Audit: blocked — Needs a nonmana additional spell cost that sacrifices a chosen creature.

// AVR 89 — Butcher Ghoul
pub(in crate::card::sets) static BUTCHER_GHOUL: CardRecord = CardRecord::new(
    cards::BUTCHER_GHOUL,
    "Butcher Ghoul",
    CardArt::new(
        "44a91e62-e946-4101-8cef-d1c147caebf2",
        "Christopher Moeller",
    ),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Zombie"], 1, 1)
        .with_ability(abilities::undying()),
);

// AVR 90 — Corpse Traders
// Audit: blocked — Needs a hidden-hand card choice and an activation timing restriction of sorcery speed.

// AVR 91 — Crypt Creeper
pub(in crate::card::sets) static CRYPT_CREEPER: CardRecord = CardRecord::new(
    cards::CRYPT_CREEPER,
    "Crypt Creeper",
    CardArt::new("0382cb94-0836-4e23-99b7-034faa363203", "Scott Chou"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Zombie"], 2, 1).with_ability(
        AbilityDef::activated_with_targets(
            "Sacrifice this creature: Exile target card from a graveyard.",
            &[AbilityCostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
                controller: None,
            },
        ),
    ),
);

// AVR 92 — Dark Impostor
// Audit: blocked — Needs the source to acquire every activated ability of the creature cards it exiled.

// AVR 93 — Death Wind
pub(in crate::card::sets) static DEATH_WIND: CardRecord = CardRecord::new(
    cards::DEATH_WIND,
    "Death Wind",
    CardArt::new("462a0961-cca5-4d63-867f-7426dbef8639", "Tomasz Jedruszek"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{X}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets -X/-X until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Negate(&ValueDef::ChosenX),
                toughness: ValueDef::Negate(&ValueDef::ChosenX),
            },
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )),
);

/// The printed intervening-if condition is checked both as the end step begins
/// and again when the trigger resolves.
static EXACTLY_ONE_CREATURE: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: PlayerRelation::You,
    },
    comparison: ComparisonDef::Equal,
    amount: 1,
};

// AVR 94 — Demonic Rising
pub(in crate::card::sets) static DEMONIC_RISING: CardRecord = CardRecord::new(
    cards::DEMONIC_RISING,
    "Demonic Rising",
    CardArt::new("a2136a82-b535-47f6-9eee-5b7585ac5cf1", "Trevor Claxton"),
    CardSet::AvacynRestored,
    CardRules::new_enchantment(mana_cost!("{3}{B}{B}")).with_ability(
        AbilityDef::triggered_if(
            "At the beginning of your end step, if you control exactly one creature, create a 5/5 black Demon creature token with flying.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            &EXACTLY_ONE_CREATURE,
            EffectDef::CreateToken {
                token: cards::DEMON_TOKEN_5_5_BLACK,
                count: ValueDef::Constant(1),
            },
        ),
    ),
);

// AVR 95 — Demonic Taskmaster
pub(in crate::card::sets) static DEMONIC_TASKMASTER: CardRecord = CardRecord::new(
    cards::DEMONIC_TASKMASTER,
    "Demonic Taskmaster",
    CardArt::new("fb5d6266-30a7-4360-84bc-22b52fb782b3", "Chris Rahn"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Demon"], 4, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "At the beginning of your upkeep, sacrifice a creature other than this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::SacrificeOfChoice {
                player: EffectRecipientDef::Controller,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                then: None,
                optional: false,
            },
        ),
    ]),
);

// AVR 96 — Demonlord of Ashmouth
// Audit: blocked — Needs an enters-the-battlefield sacrifice choice whose no-sacrifice branch exiles the source.

// AVR 97 — Descent into Madness
// Audit: blocked — Needs each player to choose a counter-derived number of permanents and/or hand cards to exile in one resolving choice.

// AVR 98 — Dread Slaver
// Audit: blocked — The damaged-creature death event and reanimation are available, but adding black and Zombie to the returned creature's characteristics is not.

// AVR 99 — Driver of the Dead
pub(in crate::card::sets) static DRIVER_OF_THE_DEAD: CardRecord = CardRecord::new(
    cards::DRIVER_OF_THE_DEAD,
    "Driver of the Dead",
    CardArt::new("56113cde-4210-46be-bd53-8966c36ef2a3", "James Ryman"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Vampire"], 3, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature dies, return target creature card with mana value 2 or less from your graveyard to the battlefield.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: Some(ZoneKind::Battlefield),
                to: Some(ZoneKind::Graveyard),
            },
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ManaValueAtMost(2),
                ]),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            })],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                controller: None,
            },
        ),
    ),
);

// AVR 100 — Essence Harvest
// Audit: blocked — Needs a dynamic value equal to the greatest power among creatures you control.

// AVR 101 — Evernight Shade
pub(in crate::card::sets) static EVERNIGHT_SHADE: CardRecord = CardRecord::new(
    cards::EVERNIGHT_SHADE,
    "Evernight Shade",
    CardArt::new("1091fadf-97c4-4f87-8466-6a1246a72226", "Nic Klein"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Shade"], 1, 1).with_abilities(&[
        AbilityDef::activated(
            "{B}: This creature gets +1/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(1),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::undying(),
    ]),
);

// AVR 102 — Exquisite Blood
// Audit: blocked — Needs a committed life-loss event that also captures nondamage life loss and its amount.

// AVR 103 — Ghoulflesh
// Audit: blocked — The power/toughness modifier is available, but adding black and Zombie to the enchanted creature's characteristics is not.

// AVR 104 — Gloom Surgeon
// Audit: blocked — Needs a combat-damage replacement that prevents the event and exiles exactly that many cards from the top of your library.

// AVR 105 — Grave Exchange
// Audit: blocked — Its sacrifice choice would be nested after another zone move, and the current resolving-decision continuation cannot resume that sequence.

// AVR 106 — Griselbrand
pub(in crate::card::sets) static GRISELBRAND: CardRecord = CardRecord::new(
    cards::GRISELBRAND,
    "Griselbrand",
    CardArt::new("b51666ae-2aef-4cb1-9cd4-44aec81530f8", "Igor Kieryluk"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{4}{B}{B}{B}{B}"), &["Demon"], 7, 7)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            abilities::lifelink(),
            AbilityDef::activated(
                "Pay 7 life: Draw seven cards.",
                &[AbilityCostDef::PayLife(7)],
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(7),
                },
            ),
        ]),
);

// AVR 107 — Harvester of Souls
// Audit: blocked — Needs a token-status object predicate so the death trigger can exclude token creatures exactly.

// AVR 108 — Homicidal Seclusion
// Audit: blocked — Needs an exactly-one-creature condition that controls both the affected recipient and a lifelink grant in the static ability layer.

// AVR 109 — Human Frailty
pub(in crate::card::sets) static HUMAN_FRAILTY: CardRecord = CardRecord::new(
    cards::HUMAN_FRAILTY,
    "Human Frailty",
    CardArt::new("1d1de712-86ac-4c03-be86-2403cd121f66", "David Palumbo"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::destroy_target(
        "Destroy target Human creature.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Subtype("Human"),
        ])),
        true,
    )),
);

// AVR 110 — Hunted Ghoul
// Audit: blocked — Needs a combat declaration restriction that prevents this creature from blocking Humans.

// AVR 111 — Killing Wave
// Audit: blocked — Needs a separate pay-X-life-or-sacrifice choice for the controller of every creature.

// AVR 112 — Maalfeld Twins
pub(in crate::card::sets) static MAALFELD_TWINS: CardRecord = CardRecord::new(
    cards::MAALFELD_TWINS,
    "Maalfeld Twins",
    CardArt::new("c63dd203-bce9-4ab7-8a0c-059d19d384e9", "Mike Sass"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{5}{B}"), &["Zombie"], 4, 4).with_ability(
        AbilityDef::triggered(
            "When this creature dies, create two 2/2 black Zombie creature tokens.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: Some(ZoneKind::Battlefield),
                to: Some(ZoneKind::Graveyard),
            },
            EffectDef::CreateToken {
                token: cards::ZOMBIE_TOKEN_2_2_BLACK,
                count: ValueDef::Constant(2),
            },
        ),
    ),
);

// AVR 113 — Marrow Bats
// Audit: blocked — Needs regeneration shields and their destroy-event replacement procedure.

// AVR 114 — Mental Agony
// Audit: blocked — Needs a continuation that waits for the targeted player's discard choice before applying the printed life loss.

// AVR 115 — Necrobite
// Audit: blocked — Needs regeneration shields and their destroy-event replacement procedure.

// AVR 116 — Polluted Dead
pub(in crate::card::sets) static POLLUTED_DEAD: CardRecord = CardRecord::new(
    cards::POLLUTED_DEAD,
    "Polluted Dead",
    CardArt::new("036c1954-37d3-4787-8df8-f2d0dd39058a", "Jason A. Engle"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Zombie"], 3, 3).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature dies, destroy target land.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: Some(ZoneKind::Battlefield),
                to: Some(ZoneKind::Graveyard),
            },
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Land),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
        ),
    ),
);

// AVR 117 — Predator's Gambit
// Audit: blocked — Needs a no-other-creatures condition that controls a static intimidate grant to the enchanted creature.

// AVR 118 — Renegade Demon
pub(in crate::card::sets) static RENEGADE_DEMON: CardRecord = CardRecord::new(
    cards::RENEGADE_DEMON,
    "Renegade Demon",
    CardArt::new("395696f8-9be2-4925-852f-b783850e1ca2", "Tomasz Jedruszek"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Demon"], 5, 3),
);

// AVR 119 — Searchlight Geist
pub(in crate::card::sets) static SEARCHLIGHT_GEIST: CardRecord = CardRecord::new(
    cards::SEARCHLIGHT_GEIST,
    "Searchlight Geist",
    CardArt::new("b0dc1a94-0193-464e-a481-730b34b57db5", "Steven Belledin"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Spirit"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{3}{B}: This creature gains deathtouch until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{3}{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::GrantAbility(&abilities::deathtouch()),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// AVR 120 — Soulcage Fiend
pub(in crate::card::sets) static SOULCAGE_FIEND: CardRecord = CardRecord::new(
    cards::SOULCAGE_FIEND,
    "Soulcage Fiend",
    CardArt::new("dce1b1d3-9602-42bf-b341-d96976ff1e60", "Jason A. Engle"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Demon"], 3, 2).with_ability(
        AbilityDef::triggered(
            "When this creature dies, each player loses 3 life.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: Some(ZoneKind::Battlefield),
                to: Some(ZoneKind::Graveyard),
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::EachPlayer,
                amount: ValueDef::Constant(3),
            },
        ),
    ),
);

// AVR 121 — Treacherous Pit-Dweller
// Audit: blocked — Needs a graveyard-to-battlefield trigger and a permanent control change to a targeted opponent.

// AVR 122 — Triumph of Cruelty
// Audit: blocked — Needs a comparison against the greatest creature power on the battlefield, including ties.

// AVR 123 — Undead Executioner
pub(in crate::card::sets) static UNDEAD_EXECUTIONER: CardRecord = CardRecord::new(
    cards::UNDEAD_EXECUTIONER,
    "Undead Executioner",
    CardArt::new("8d330058-16af-4486-aa89-b6be759e35d4", "Dave Kendall"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Zombie"], 2, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature dies, you may have target creature get -2/-2 until end of turn.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: Some(ZoneKind::Battlefield),
                to: Some(ZoneKind::Graveyard),
            },
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::May(&EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(-2),
                    toughness: ValueDef::Constant(-2),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            }),
        ),
    ),
);

// AVR 124 — Unhallowed Pact
// Audit: blocked — Needs a zone-change trigger whose subject is the permanent currently attached to this Aura.

// AVR 125 — Aggravate
// Audit: blocked — Needs to grant the attack requirement only to creatures actually dealt damage after prevention and replacement effects.

// AVR 126 — Archwing Dragon
pub(in crate::card::sets) static ARCHWING_DRAGON: CardRecord = CardRecord::new(
    cards::ARCHWING_DRAGON,
    "Archwing Dragon",
    CardArt::new("6c6f1a8b-329e-4094-8141-6bc88311a08c", "Daarken"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Dragon"], 4, 4).with_abilities(&[
        abilities::flying(),
        abilities::haste(),
        AbilityDef::triggered(
            "At the beginning of the end step, return this creature to its owner's hand.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::Any,
            },
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Source,
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
                controller: None,
            },
        ),
    ]),
);

// AVR 127 — Banners Raised
pub(in crate::card::sets) static BANNERS_RAISED: CardRecord = CardRecord::new(
    cards::BANNERS_RAISED,
    "Banners Raised",
    CardArt::new("a7792df3-e2ab-4e60-abee-f24b72807107", "Mike Bierek"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::spell(
        "Creatures you control get +1/+0 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::You,
            },
            effect: AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Constant(1),
                toughness: ValueDef::Constant(0),
            },
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )),
);

static BATTLE_HYMN_CREATURES: ObjectQueryDef = ObjectQueryDef {
    object: ObjectPredicateDef::HasType(CardType::Creature),
    zones: &[ZoneKind::Battlefield],
    controller: PlayerRelation::You,
};

// AVR 128 — Battle Hymn
pub(in crate::card::sets) static BATTLE_HYMN: CardRecord = CardRecord::new(
    cards::BATTLE_HYMN,
    "Battle Hymn",
    CardArt::new("43b5d46e-7054-44f8-9a14-b412f2f0ab86", "Nils Hamm"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell(
        "Add {R} for each creature you control.",
        EffectDef::AddManaEqualTo {
            color: ManaColor::Red,
            amount: ValueDef::CountMatchingObjects(&BATTLE_HYMN_CREATURES),
        },
    )),
);

// AVR 129 — Bonfire of the Damned
pub(in crate::card::sets) static BONFIRE_OF_THE_DAMNED: CardRecord = CardRecord::new(
    cards::BONFIRE_OF_THE_DAMNED,
    "Bonfire of the Damned",
    CardArt::new("e60610fe-891d-46de-b556-d03b637dccec", "James Paick"),
    CardSet::AvacynRestored,
    CardRules::new_sorcery(mana_cost!("{X}{X}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Bonfire of the Damned deals X damage to target player or planeswalker and each creature that player or that planeswalker's controller controls.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )],
            EffectDef::Sequence(&[
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::ChosenX,
                },
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::ObjectsControlledByTarget {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        slot: TargetIndex::PRIMARY,
                    },
                    amount: ValueDef::ChosenX,
                },
            ]),
        ),
        abilities::miracle(mana_cost!("{X}{R}")),
    ]),
);

// AVR 130 — Burn at the Stake
// Audit: blocked — Needs a spell additional cost that taps any number of chosen untapped creatures and retains that count for a three-times damage value.

// AVR 131 — Dangerous Wager
// Audit: blocked — Needs a dynamic whole-hand discard amount before the draw.

// AVR 132 — Demolish
pub(in crate::card::sets) static DEMOLISH: CardRecord = CardRecord::new(
    cards::DEMOLISH,
    "Demolish",
    CardArt::new("4657aa15-8274-4bd7-afe4-504693064373", "Raymond Swanland"),
    CardSet::AvacynRestored,
    CardRules::new_sorcery(mana_cost!("{3}{R}")).with_ability(AbilityDef::destroy_target(
        "Destroy target artifact or land.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Artifact),
            ObjectPredicateDef::HasType(CardType::Land),
        ])),
        true,
    )),
);

// AVR 133 — Dual Casting
// Audit: blocked — Needs a stack-spell copy effect with optional new targets.

// AVR 134 — Falkenrath Exterminator
pub(in crate::card::sets) static FALKENRATH_EXTERMINATOR: CardRecord = CardRecord::new(
    cards::FALKENRATH_EXTERMINATOR,
    "Falkenrath Exterminator",
    CardArt::new("40e23909-7e08-4686-ae59-e18e7d4cfd3c", "Winona Nelson"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Vampire", "Archer"], 1, 1).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, put a +1/+1 counter on it.",
            TriggerEventDef::CombatDamageDealtToPlayer {
                source: ObjectPredicateDef::Source,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_with_targets(
            "{2}{R}: This creature deals damage to target creature equal to the number of +1/+1 counters on this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{R}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::CountersOnSource(CounterKind::PlusOnePlusOne),
            },
        ),
    ]),
);

// AVR 135 — Fervent Cathar
// Audit: blocked — Needs a duration-scoped restriction preventing the targeted creature from blocking this turn.

// AVR 136 — Gang of Devils
pub(in crate::card::sets) static GANG_OF_DEVILS: CardRecord = CardRecord::new(
    cards::GANG_OF_DEVILS,
    "Gang of Devils",
    CardArt::new("0430b9fa-3bc6-4183-ad5b-d70ad401fa97", "Erica Yang"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{5}{R}"), &["Devil"], 3, 3).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature dies, it deals 3 damage divided as you choose among one, two, or three targets.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: Some(ZoneKind::Battlefield),
                to: Some(ZoneKind::Graveyard),
            },
            &[AbilityTargetDef {
                predicate: AbilityTargetPredicate::AnyTarget,
                minimum: 1,
                maximum: 3,
                divided_total: Some(DividedTotal::Fixed(3)),
            }],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::DividedAmongTargets,
            },
        ),
    ),
);

// AVR 137 — Guise of Fire
pub(in crate::card::sets) static GUISE_OF_FIRE: CardRecord = CardRecord::new(
    cards::GUISE_OF_FIRE,
    "Guise of Fire",
    CardArt::new("beb10d42-fa19-400c-bad8-ec3827f077bc", "Dave Kendall"),
    CardSet::AvacynRestored,
    CardRules::new_enchantment(mana_cost!("{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/-1 and attacks each combat if able.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::ModifyPowerToughness {
                            power: ValueDef::Constant(1),
                            toughness: ValueDef::Constant(-1),
                        },
                        AppliedEffectDef::GrantAbility(&abilities::attacks_each_combat_if_able(
                            "This creature attacks each combat if able.",
                        )),
                    ]),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// AVR 138 — Hanweir Lancer
// Audit: blocked — Needs soulbond pairing state, paired-object identity, and a conditional first-strike grant to both paired creatures.

// AVR 139 — Havengul Vampire
pub(in crate::card::sets) static HAVENGUL_VAMPIRE: CardRecord = CardRecord::new(
    cards::HAVENGUL_VAMPIRE,
    "Havengul Vampire",
    CardArt::new("cbc09839-1463-40b8-86bd-fb96797b2633", "James Ryman"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Vampire"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, put a +1/+1 counter on it.",
            TriggerEventDef::CombatDamageDealtToPlayer {
                source: ObjectPredicateDef::Source,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::triggered(
            "Whenever another creature dies, put a +1/+1 counter on this creature.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                from: Some(ZoneKind::Battlefield),
                to: Some(ZoneKind::Graveyard),
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// AVR 140 — Heirs of Stromkirk
pub(in crate::card::sets) static HEIRS_OF_STROMKIRK: CardRecord = CardRecord::new(
    cards::HEIRS_OF_STROMKIRK,
    "Heirs of Stromkirk",
    CardArt::new("ff89ad3b-b154-49e2-a0fd-135279512250", "Winona Nelson"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Vampire"], 2, 2).with_abilities(&[
        abilities::intimidate(),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, put a +1/+1 counter on it.",
            TriggerEventDef::CombatDamageDealtToPlayer {
                source: ObjectPredicateDef::Source,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// AVR 141 — Hound of Griselbrand
pub(in crate::card::sets) static HOUND_OF_GRISELBRAND: CardRecord = CardRecord::new(
    cards::HOUND_OF_GRISELBRAND,
    "Hound of Griselbrand",
    CardArt::new("0fe68bce-6207-4fd1-9e82-a18fd2d6ddca", "Svetlin Velinov"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Elemental", "Dog"], 2, 2)
        .with_abilities(&[abilities::double_strike(), abilities::undying()]),
);

static KESSIG_MALCONTENTS_HUMANS: ObjectQueryDef = ObjectQueryDef {
    object: ObjectPredicateDef::Subtype("Human"),
    zones: &[ZoneKind::Battlefield],
    controller: PlayerRelation::You,
};

// AVR 142 — Kessig Malcontents
pub(in crate::card::sets) static KESSIG_MALCONTENTS: CardRecord = CardRecord::new(
    cards::KESSIG_MALCONTENTS,
    "Kessig Malcontents",
    CardArt::new("dce9a30f-a850-4826-a255-ce511d567b60", "John Stanko"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Warrior"], 3, 1).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature enters, it deals damage to target player or planeswalker equal to the number of Humans you control.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::CountMatchingObjects(&KESSIG_MALCONTENTS_HUMANS),
            },
        ),
    ),
);

// AVR 143 — Kruin Striker
pub(in crate::card::sets) static KRUIN_STRIKER: CardRecord = CardRecord::new(
    cards::KRUIN_STRIKER,
    "Kruin Striker",
    CardArt::new("73e72249-84ea-4e9c-9f64-b67b02ffdf3a", "Christopher Moeller"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Warrior"], 2, 1).with_ability(
        AbilityDef::triggered(
            "Whenever another creature you control enters, this creature gets +1/+0 and gains trample until end of turn.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(1),
                        toughness: ValueDef::Constant(0),
                    },
                    AppliedEffectDef::GrantAbility(&abilities::trample()),
                ]),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// AVR 144 — Lightning Mauler
// Audit: blocked — Needs soulbond pairing state, paired-object identity, and a conditional haste grant to both paired creatures.

static LIGHTNING_PROWESS_PING: AbilityDef = AbilityDef::activated_with_targets(
    "{T}: This creature deals 1 damage to any target.",
    &[AbilityCostDef::TapSource],
    &[AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::AnyTarget,
    )],
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: ValueDef::Constant(1),
    },
);

// AVR 145 — Lightning Prowess
pub(in crate::card::sets) static LIGHTNING_PROWESS: CardRecord = CardRecord::new(
    cards::LIGHTNING_PROWESS,
    "Lightning Prowess",
    CardArt::new("5578e3e2-2460-4dfb-9016-527463f2d918", "David Rapoza"),
    CardSet::AvacynRestored,
    CardRules::new_enchantment(mana_cost!("{2}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature has haste and \"{T}: This creature deals 1 damage to any target.\"",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::GrantAbility(&abilities::haste()),
                        AppliedEffectDef::GrantAbility(&LIGHTNING_PROWESS_PING),
                    ]),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// AVR 146 — Mad Prophet
// Audit: blocked — Needs discarding a chosen card as an activated-ability cost.

// AVR 147 — Malicious Intent
// Audit: blocked — Needs a duration-scoped restriction that prevents the targeted creature from blocking this turn.

// AVR 148 — Malignus
// Audit: blocked — Needs a characteristic-defining half-highest-opponent-life value and a damage-prevention prohibition for the source.

// AVR 149 — Pillar of Flame
pub(in crate::card::sets) static PILLAR_OF_FLAME: CardRecord = CardRecord::new(
    cards::PILLAR_OF_FLAME,
    "Pillar of Flame",
    CardArt::new("c983e879-d9d2-47cc-9958-506711ca80cd", "Karl Kopinski"),
    CardSet::AvacynRestored,
    CardRules::new_sorcery(mana_cost!("{R}")).with_ability(
        AbilityDef::custom_full(
            "Pillar of Flame deals 2 damage to any target. If a creature dealt damage this way would die this turn, exile it instead.",
            CardBehavior::PillarOfFlame,
            "Implemented by the named card-local special behavior.",
        ),
    ),
);

// AVR 150 — Raging Poltergeist
pub(in crate::card::sets) static RAGING_POLTERGEIST: CardRecord = CardRecord::new(
    cards::RAGING_POLTERGEIST,
    "Raging Poltergeist",
    CardArt::new("78833788-ffb2-43fc-9345-975f1cd46f38", "Slawomir Maniak"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Spirit"], 6, 1),
);

// AVR 151 — Reforge the Soul
// Audit: blocked — Needs a dynamic whole-hand discard for every player before the seven-card draw.

// AVR 152 — Riot Ringleader
pub(in crate::card::sets) static RIOT_RINGLEADER: CardRecord = CardRecord::new(
    cards::RIOT_RINGLEADER,
    "Riot Ringleader",
    CardArt::new("c043f30b-548f-4c31-a415-0e59c2841dcf", "Gabor Szikszai"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Warrior"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever this creature attacks, Human creatures you control get +1/+0 until end of turn.",
            TriggerEventDef::Attacks(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Human"),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::You,
                },
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(0),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// AVR 153 — Rite of Ruin
// Audit: blocked — Needs a chosen ordering of three permanent types and six sequential per-player sacrifice choices with retained mode order.

// AVR 154 — Rush of Blood
pub(in crate::card::sets) static RUSH_OF_BLOOD: CardRecord = CardRecord::new(
    cards::RUSH_OF_BLOOD,
    "Rush of Blood",
    CardArt::new("a2884824-d138-47f2-913b-32cd475e9584", "Cynthia Sheppard"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{2}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +X/+0 until end of turn, where X is its power.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::TargetPower(TargetIndex::PRIMARY),
                toughness: ValueDef::Constant(0),
            },
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// AVR 155 — Scalding Devil
pub(in crate::card::sets) static SCALDING_DEVIL: CardRecord = CardRecord::new(
    cards::SCALDING_DEVIL,
    "Scalding Devil",
    CardArt::new("bbe49a97-dac8-4273-b4dc-45cdf8f5a6e0", "Erica Yang"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Devil"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{2}{R}: This creature deals 1 damage to target player or planeswalker.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{R}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// AVR 156 — Somberwald Vigilante
// Audit: blocked — Needs a becomes-blocked event carrying the individual blocking creature.

// AVR 157 — Stonewright
// Audit: blocked — Needs soulbond pairing state, paired-object identity, and an activated pump ability granted to both paired creatures.

// AVR 158 — Thatcher Revolt
// Audit: blocked — Needs identity links from the three created tokens to the delayed sacrifice so it does not sacrifice unrelated Human tokens.

// AVR 159 — Thunderbolt
// Audit: blocked — The flying target predicate does not see flying granted or removed by static continuous effects.

// AVR 160 — Thunderous Wrath
pub(in crate::card::sets) static THUNDEROUS_WRATH: CardRecord = CardRecord::new(
    cards::THUNDEROUS_WRATH,
    "Thunderous Wrath",
    CardArt::new("daa39826-7f89-41cb-a7fe-7f7be817d5cd", "Adam Paquette"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{4}{R}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Thunderous Wrath deals 5 damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(5),
            },
        ),
        abilities::miracle(mana_cost!("{R}")),
    ]),
);

// AVR 161 — Tibalt, the Fiend-Blooded
// Audit: blocked — Needs a target player's hand-size value; its other loyalty effects do not make the whole planeswalker exact without that value.

// AVR 162 — Tyrant of Discord
// Audit: blocked — Needs an opponent's random permanent choice and repeat-until-land sacrifice loop.

// AVR 163 — Uncanny Speed
pub(in crate::card::sets) static UNCANNY_SPEED: CardRecord = CardRecord::new(
    cards::UNCANNY_SPEED,
    "Uncanny Speed",
    CardArt::new("1d7b747e-446a-4c25-9834-0be8476dc22d", "Raymond Swanland"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +3/+0 and gains haste until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(3),
                    toughness: ValueDef::Constant(0),
                },
                AppliedEffectDef::GrantAbility(&abilities::haste()),
            ]),
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// AVR 164 — Vexing Devil
// Audit: blocked — Needs an opponent choice on resolution, including which opponent in multiplayer, with a sacrifice branch only when one accepts the damage.

// AVR 165 — Vigilante Justice
pub(in crate::card::sets) static VIGILANTE_JUSTICE: CardRecord = CardRecord::new(
    cards::VIGILANTE_JUSTICE,
    "Vigilante Justice",
    CardArt::new("a9db329b-6248-4082-bfc8-5d2c0db43338", "Steve Prescott"),
    CardSet::AvacynRestored,
    CardRules::new_enchantment(mana_cost!("{3}{R}")).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever a Human you control enters, this enchantment deals 1 damage to any target.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype("Human"),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

/// Haste matters here because the permanent has not been under its new
/// controller's control since the turn began.
static HASTE_GRANT: AbilityDef = abilities::haste();

// AVR 166 — Zealous Conscripts
pub(in crate::card::sets) static ZEALOUS_CONSCRIPTS: CardRecord = CardRecord::new(
    cards::ZEALOUS_CONSCRIPTS,
    "Zealous Conscripts",
    CardArt::new("fc027b11-1ecc-430d-a862-586a14bb23c3", "Steve Prescott"),
    CardSet::AvacynRestored,
    CardRules::new_creature(
        mana_cost!("{4}{R}"),
        &["Human", "Warrior"],
        3,
        3,
    )
    .with_abilities(&[
        abilities::haste(),
        AbilityDef::triggered_with_targets("When this creature enters, gain control of target permanent until end of turn. Untap that permanent. It gains haste until end of turn.", TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            }, &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::Any,
        )], // Control first: the untap and the haste are worth having only
            // on a permanent that is already yours to use.
            EffectDef::Sequence(&[
                EffectDef::GainControlThisTurn {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::GrantAbility(&HASTE_GRANT),
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
            ])),
    ]),
);

static ABUNDANT_GROWTH_MANA: AbilityDef = AbilityDef::activated_mana(
    "{T}: Add one mana of any color.",
    &[AbilityCostDef::TapSource],
    EffectDef::AddMana(AddManaEffectDef::any_color()),
);

// AVR 167 — Abundant Growth
pub(in crate::card::sets) static ABUNDANT_GROWTH: CardRecord = CardRecord::new(
    cards::ABUNDANT_GROWTH,
    "Abundant Growth",
    CardArt::new("afbc8fd0-dc15-4ac9-b97b-173f7fb66ed7", "Vincent Proce"),
    CardSet::AvacynRestored,
    CardRules::new_enchantment(mana_cost!("{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant land",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Land),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::triggered(
                "When this Aura enters, draw a card.",
                TriggerEventDef::ZoneChanged {
                    object: ObjectPredicateDef::Source,
                    from: None,
                    to: Some(ZoneKind::Battlefield),
                },
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted land has \"{T}: Add one mana of any color.\"",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::GrantAbility(&ABUNDANT_GROWTH_MANA),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// AVR 168 — Blessings of Nature
// Audit: blocked — Divided target shares are currently implemented only for damage; using them for counters resolves every counter amount as zero.

// AVR 169 — Borderland Ranger
pub(in crate::card::sets) static BORDERLAND_RANGER: CardRecord = CardRecord::new(
    cards::BORDERLAND_RANGER,
    "Borderland Ranger",
    CardArt::new("8f067c26-c51d-44d0-a0af-106b5778f06a", "Zoltan Boros"),
    CardSet::AvacynRestored,
    CardRules::new_creature(
        mana_cost!("{2}{G}"),
        &["Human", "Scout", "Ranger"],
        2,
        2,
    )
    .with_ability(AbilityDef::triggered(
        "When this creature enters, you may search your library for a basic land card, reveal it, put it into your hand, then shuffle.",
        TriggerEventDef::ZoneChanged {
            object: ObjectPredicateDef::Source,
            from: None,
            to: Some(ZoneKind::Battlefield),
        },
        EffectDef::SearchLibrary {
            player: EffectRecipientDef::Controller,
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Land),
                ObjectPredicateDef::Supertype(CardSupertype::Basic),
            ]),
            destination: ZoneKind::Hand,
        },
    )),
);

// AVR 170 — Bower Passage
// Audit: blocked — The blocker flying predicate ignores flying granted or removed by static continuous effects.

// AVR 171 — Champion of Lambholt
// Audit: blocked — Needs a blocking predicate that dynamically compares each prospective blocker's power with this creature's current power.

static CRATERHOOF_CREATURES: ObjectQueryDef = ObjectQueryDef {
    object: ObjectPredicateDef::HasType(CardType::Creature),
    zones: &[ZoneKind::Battlefield],
    controller: PlayerRelation::You,
};

// AVR 172 — Craterhoof Behemoth
pub(in crate::card::sets) static CRATERHOOF_BEHEMOTH: CardRecord = CardRecord::new(
    cards::CRATERHOOF_BEHEMOTH,
    "Craterhoof Behemoth",
    CardArt::new("a249be17-73ed-4108-89c0-f7e87939beb8", "Chris Rahn"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{5}{G}{G}{G}"), &["Beast"], 5, 5).with_abilities(&[
        abilities::haste(),
        AbilityDef::triggered(
            "When this creature enters, creatures you control gain trample and get +X/+X until end of turn, where X is the number of creatures you control.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::You,
                },
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::CountMatchingObjects(&CRATERHOOF_CREATURES),
                        toughness: ValueDef::CountMatchingObjects(&CRATERHOOF_CREATURES),
                    },
                    AppliedEffectDef::GrantAbility(&abilities::trample()),
                ]),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// AVR 173 — Descendants' Path
// Audit: blocked — Needs a top-card reveal, shared-creature-type test, free-cast permission, and bottom placement when the card is not cast.

// AVR 174 — Diregraf Escort
// Audit: blocked — Needs soulbond pairing state, paired-object identity, and protection from Zombies granted to both paired creatures.

// AVR 175 — Druid's Familiar
// Audit: blocked — Needs soulbond pairing state, paired-object identity, and a conditional +2/+2 bonus to both paired creatures.

// AVR 176 — Druids' Repository
pub(in crate::card::sets) static DRUIDS_REPOSITORY: CardRecord = CardRecord::new(
    cards::DRUIDS_REPOSITORY,
    "Druids' Repository",
    CardArt::new("57e6fb62-7ee3-444d-8fd4-c1f44014a05c", "Daarken"),
    CardSet::AvacynRestored,
    CardRules::new_enchantment(mana_cost!("{1}{G}{G}")).with_abilities(&[
        AbilityDef::triggered(
            "Whenever a creature you control attacks, put a charge counter on this enchantment.",
            TriggerEventDef::Attacks(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::Charge,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "Remove a charge counter from this enchantment: Add one mana of any color.",
            &[AbilityCostDef::RemoveCountersFromSource {
                kind: CounterKind::Charge,
                amount: 1,
            }],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// AVR 177 — Eaten by Spiders
// Audit: blocked — Needs an attachment relation that finds and destroys every Equipment attached to the targeted creature.

// AVR 178 — Flowering Lumberknot
// Audit: blocked — Needs soulbond pairing state plus attack and block legality tied to being paired with a soulbond creature.

// AVR 179 — Geist Trappers
// Audit: blocked — Needs soulbond pairing state, paired-object identity, and a conditional reach grant to both paired creatures.

// AVR 180 — Gloomwidow
// Audit: blocked — Needs a combat declaration restriction allowing this creature to block only creatures with flying.

// AVR 181 — Grounded
pub(in crate::card::sets) static GROUNDED: CardRecord = CardRecord::new(
    cards::GROUNDED,
    "Grounded",
    CardArt::new("dc4982f0-0ede-4846-82c8-bcf7ad63d099", "Greg Staples"),
    CardSet::AvacynRestored,
    CardRules::new_enchantment(mana_cost!("{1}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature loses flying.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::RemoveAbilities(
                        crate::card::AbilityPredicateDef::Keyword(
                            crate::card::KeywordAbility::Flying,
                        ),
                    ),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// AVR 182 — Howlgeist
// Audit: blocked — Needs a blocking predicate that dynamically compares each prospective blocker's power with this creature's current power.

// AVR 183 — Joint Assault
// Audit: blocked — Needs soulbond pairing state and the identity of the creature paired with the target.

// AVR 184 — Lair Delve
// Audit: blocked — Needs a mandatory characteristic-filtered top-two split and player-chosen ordering for the cards put on the bottom.

// AVR 185 — Natural End
pub(in crate::card::sets) static NATURAL_END: CardRecord = CardRecord::new(
    cards::NATURAL_END,
    "Natural End",
    CardArt::new("95d25235-de1c-4b67-9712-24f0564bd2bf", "Scott Chou"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{2}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target artifact or enchantment. You gain 3 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Enchantment),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ]),
    )),
);

// AVR 186 — Nettle Swine
pub(in crate::card::sets) static NETTLE_SWINE: CardRecord = CardRecord::new(
    cards::NETTLE_SWINE,
    "Nettle Swine",
    CardArt::new(
        "75935f0e-9086-485b-b3e6-1a958fd0f2af",
        "Christopher Moeller",
    ),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Boar"], 4, 3),
);

// AVR 187 — Nightshade Peddler
// Audit: blocked — Needs soulbond pairing state, paired-object identity, and a conditional deathtouch grant to both paired creatures.

// AVR 188 — Pathbreaker Wurm
// Audit: blocked — Needs soulbond pairing state, paired-object identity, and a conditional trample grant to both paired creatures.

// AVR 189 — Primal Surge
// Audit: blocked — Needs a repeatable top-card exile procedure with a permanent-card branch and a new optional decision on every iteration.

// AVR 190 — Rain of Thorns
pub(in crate::card::sets) static RAIN_OF_THORNS: CardRecord = CardRecord::new(
    cards::RAIN_OF_THORNS,
    "Rain of Thorns",
    CardArt::new("fd1cb530-b9d5-4386-b89e-2acecc8294c8", "Sam Burley"),
    CardSet::AvacynRestored,
    CardRules::new_sorcery(mana_cost!("{4}{G}{G}")).with_ability(AbilityDef::modal_spell(
        "Choose one or more —\n• Destroy target artifact.\n• Destroy target enchantment.\n• Destroy target land.",
        &[
            AbilityDef::spell_with_targets(
                "Destroy target artifact",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                },
            ),
            AbilityDef::spell_with_targets(
                "Destroy target enchantment",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                },
            ),
            AbilityDef::spell_with_targets(
                "Destroy target land",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Land),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                },
            ),
        ],
        1,
        3,
        false,
    )),
);

// AVR 191 — Revenge of the Hunted
// Audit: blocked — Needs a turn-scoped requirement that every creature able to block the target does so.

// AVR 192 — Sheltering Word
// Audit: blocked — Needs a value for the targeted creature's toughness after the hexproof grant resolves.

// AVR 193 — Snare the Skies
pub(in crate::card::sets) static SNARE_THE_SKIES: CardRecord = CardRecord::new(
    cards::SNARE_THE_SKIES,
    "Snare the Skies",
    CardArt::new("28f75827-a144-4fe2-a713-4439ae7567eb", "Ryan Yee"),
    CardSet::AvacynRestored,
    CardRules::new_instant(mana_cost!("{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +1/+1 and gains reach until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(1),
                },
                AppliedEffectDef::GrantAbility(&abilities::reach()),
            ]),
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )),
);

static SOMBERWALD_SAGE_RESTRICTIONS: [ManaRestrictionDef; 1] = [ManaRestrictionDef::CastSpell(
    ObjectPredicateDef::HasType(CardType::Creature),
)];

// AVR 194 — Somberwald Sage
pub(in crate::card::sets) static SOMBERWALD_SAGE: CardRecord = CardRecord::new(
    cards::SOMBERWALD_SAGE,
    "Somberwald Sage",
    CardArt::new("409c0272-7a43-4a6c-ab3f-740397b1f5c8", "Steve Argyle"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Druid"], 0, 1).with_ability(
        AbilityDef::activated_mana(
            "{T}: Add three mana of any one color. Spend this mana only to cast creature spells.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(
                AddManaEffectDef::any_color()
                    .with_amount(3)
                    .with_restrictions(&SOMBERWALD_SAGE_RESTRICTIONS),
            ),
        ),
    ),
);

// AVR 195 — Soul of the Harvest
// Audit: blocked — Needs a token-status object predicate so the entry trigger can exclude token creatures exactly.

// AVR 196 — Terrifying Presence
// Audit: blocked — Needs a recipient set for every creature other than the targeted creature while retaining that target identity through prevention.

// AVR 197 — Timberland Guide
pub(in crate::card::sets) static TIMBERLAND_GUIDE: CardRecord = CardRecord::new(
    cards::TIMBERLAND_GUIDE,
    "Timberland Guide",
    CardArt::new("ae80fefb-af78-4f98-8058-71b61e91842f", "Zoltan Boros"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Scout"], 1, 1).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature enters, put a +1/+1 counter on target creature.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// AVR 198 — Triumph of Ferocity
// Audit: blocked — Needs a comparison against the greatest creature power on the battlefield, including ties.

// AVR 199 — Trusted Forcemage
// Audit: blocked — Needs soulbond pairing state, paired-object identity, and a conditional +1/+1 bonus to both paired creatures.

// AVR 200 — Ulvenwald Tracker
// Audit: blocked — Needs the simultaneous fight damage procedure and its two-creature target relation.

// AVR 201 — Vorstclaw
pub(in crate::card::sets) static VORSTCLAW: CardRecord = CardRecord::new(
    cards::VORSTCLAW,
    "Vorstclaw",
    CardArt::new("7591ee4f-9bfe-4419-84df-abf35d85bb94", "Lucas Graciano"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Elemental", "Horror"], 7, 7),
);

// AVR 202 — Wandering Wolf
// Audit: blocked — Needs a blocking predicate that dynamically compares each prospective blocker's power with this creature's current power.

// AVR 203 — Wild Defiance
// Audit: blocked — Needs an event for a creature becoming the target of an instant or sorcery spell, carrying that creature as the effect recipient.

// AVR 204 — Wildwood Geist
// Audit: blocked — Needs an active-player condition usable by a continuous power/toughness effect.

// AVR 205 — Wolfir Avenger
// Audit: blocked — Needs regeneration shields and their destroy-event replacement procedure.

// AVR 206 — Wolfir Silverheart
// Audit: blocked — Needs soulbond pairing state, paired-object identity, and a conditional +4/+4 bonus to both paired creatures.

// AVR 207 — Yew Spirit
pub(in crate::card::sets) static YEW_SPIRIT: CardRecord = CardRecord::new(
    cards::YEW_SPIRIT,
    "Yew Spirit",
    CardArt::new("b9320432-4f89-4363-91e6-2e740535cc2e", "Dan Murayama Scott"),
    CardSet::AvacynRestored,
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Spirit", "Treefolk"], 3, 3).with_ability(
        AbilityDef::activated(
            "{2}{G}{G}: This creature gets +X/+X until end of turn, where X is its power.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{G}{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::SourcePower,
                    toughness: ValueDef::SourcePower,
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// AVR 208 — Bruna, Light of Alabaster
// Audit: blocked — Needs a blocking trigger and resolving choices of any number of legal Auras across the battlefield, hand, and graveyard to attach to the source.

// AVR 209 — Gisela, Blade of Goldnight
// Audit: blocked — Needs global damage-event replacements that double opposing damage and prevent half of incoming damage with rounding.

// AVR 210 — Sigarda, Host of Herons
pub(in crate::card::sets) static SIGARDA_HOST_OF_HERONS: CardRecord = CardRecord::new(
    cards::SIGARDA_HOST_OF_HERONS,
    "Sigarda, Host of Herons",
    CardArt::new("feccd0e2-fae6-4ced-acdf-4252ed5c56e7", "Chris Rahn"),
    CardSet::AvacynRestored,
    CardRules::new_creature(
        mana_cost!("{2}{G}{W}{W}"),
        &["Angel"],
        5,
        5,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::flying(),
        abilities::hexproof(),
        AbilityDef::static_ability(
            "Spells and abilities your opponents control can't cause you to sacrifice permanents.",
            EffectDef::CannotBeForcedToSacrifice,
        ),
    ]),
);

// AVR 211 — Angel's Tomb
// Audit: blocked — Resolving animation cannot currently set an artifact creature's white color exactly; the animation payload only applies colors when replacing printed characteristics.

// AVR 212 — Angelic Armaments
// Audit: blocked — Needs Equipment attach actions and attachment-scoped color and subtype changes.

// AVR 213 — Bladed Bracers
// Audit: blocked — Needs Equipment attach actions and a Human-or-Angel condition for the vigilance grant.

// AVR 214 — Conjurer's Closet
// Audit: blocked — Linked exile returns a stolen target under its owner rather than the ability controller required by the card.

// AVR 215 — Gallows at Willow Hill
// Audit: blocked — Needs an activation cost that taps three separately chosen untapped Humans you control.

// AVR 216 — Haunted Guardian
pub(in crate::card::sets) static HAUNTED_GUARDIAN: CardRecord = CardRecord::new(
    cards::HAUNTED_GUARDIAN,
    "Haunted Guardian",
    CardArt::new("7d97f8b8-bdb0-4d4b-b077-9affe2f9cd91", "Daniel Ljunggren"),
    CardSet::AvacynRestored,
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Construct"], 2, 1)
        .with_abilities(&[abilities::defender(), abilities::first_strike()]),
);

// AVR 217 — Moonsilver Spear
// Audit: blocked — Needs Equipment attach actions and a trigger whose subject is the currently equipped creature.

// AVR 218 — Narstad Scrapper
pub(in crate::card::sets) static NARSTAD_SCRAPPER: CardRecord = CardRecord::new(
    cards::NARSTAD_SCRAPPER,
    "Narstad Scrapper",
    CardArt::new("f808ed9b-95ac-4069-bdca-b100bc816b5b", "Steven Belledin"),
    CardSet::AvacynRestored,
    CardRules::new_artifact_creature(mana_cost!("{5}"), &["Construct"], 3, 3).with_ability(
        AbilityDef::activated(
            "{2}: This creature gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{2}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(0),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// AVR 219 — Otherworld Atlas
pub(in crate::card::sets) static OTHERWORLD_ATLAS: CardRecord = CardRecord::new(
    cards::OTHERWORLD_ATLAS,
    "Otherworld Atlas",
    CardArt::new("46e4aa67-4643-42ff-8172-200498686494", "Sam Wolfe Connelly"),
    CardSet::AvacynRestored,
    CardRules::new_artifact(mana_cost!("{4}"))
        .with_subtypes(&["Book"])
        .with_abilities(&[
            AbilityDef::activated(
                "{T}: Put a charge counter on this artifact.",
                &[AbilityCostDef::TapSource],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::Charge,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::activated(
                "{T}: Each player draws a card for each charge counter on this artifact.",
                &[AbilityCostDef::TapSource],
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::EachPlayer,
                    amount: ValueDef::CountersOnSource(CounterKind::Charge),
                },
            ),
        ]),
);

static CONTROLS_AN_ANGEL: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef {
        object: ObjectPredicateDef::Subtype("Angel"),
        zones: &[ZoneKind::Battlefield],
        controller: PlayerRelation::You,
    },
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 1,
};

// AVR 220 — Scroll of Avacyn
pub(in crate::card::sets) static SCROLL_OF_AVACYN: CardRecord = CardRecord::new(
    cards::SCROLL_OF_AVACYN,
    "Scroll of Avacyn",
    CardArt::new("871e6e2a-7e45-446b-b964-94377eb6ca92", "Cliff Childs"),
    CardSet::AvacynRestored,
    CardRules::new_artifact(mana_cost!("{1}")).with_ability(AbilityDef::activated(
        "{1}, Sacrifice this artifact: Draw a card. If you control an Angel, you gain 5 life.",
        &[
            AbilityCostDef::Mana(mana_cost!("{1}")),
            AbilityCostDef::SacrificeSource,
        ],
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
            EffectDef::IfCondition {
                condition: &CONTROLS_AN_ANGEL,
                then: &EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(5),
                },
            },
        ]),
    )),
);

// AVR 221 — Scroll of Griselbrand
// Audit: blocked — Needs a continuation that waits for the opponent's discard choice before checking for a Demon and applying the printed life loss.

// AVR 222 — Tormentor's Trident
// Audit: blocked — Needs Equipment attach actions and an attack requirement granted to the equipped creature.

// AVR 223 — Vanguard's Shield
// Audit: blocked — Needs Equipment attach actions and support for blocking one additional creature each combat.

// AVR 224 — Vessel of Endless Rest
pub(in crate::card::sets) static VESSEL_OF_ENDLESS_REST: CardRecord = CardRecord::new(
    cards::VESSEL_OF_ENDLESS_REST,
    "Vessel of Endless Rest",
    CardArt::new("ec733373-3f68-47ad-ac35-6f39092f1e26", "John Avon"),
    CardSet::AvacynRestored,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "When this artifact enters, put target card from a graveyard on the bottom of its owner's library.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Any,
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: None,
            })],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Library,
                placement: ZonePlacement::Bottom,
                controller: None,
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// AVR 225 — Alchemist's Refuge
// Audit: blocked — Needs a turn-scoped permission allowing every spell you cast to be cast as though it had flash.

static CAVERN_COLORED_MANA_RESTRICTIONS: [ManaRestrictionDef; 1] =
    [ManaRestrictionDef::CastCreatureSpellOfChosenType];

static CAVERN_COLORED_MANA_SPEND_EFFECTS: [ManaSpendEffectDef; 1] =
    [ManaSpendEffectDef::ApplyToPaidSpell(
        AppliedEffectDef::CannotBeCountered,
    )];

// AVR 226 — Cavern of Souls
pub(in crate::card::sets) static CAVERN_OF_SOULS: CardRecord = CardRecord::new(
    cards::CAVERN_OF_SOULS,
    "Cavern of Souls",
    CardArt::new("1381c8f1-a292-4bdf-b20c-a5c2a169ee84", "Cliff Childs"),
    CardSet::AvacynRestored,
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::replacement(
            "As this land enters, choose a creature type.",
            EffectDef::ChooseCreatureType {
                object: EffectRecipientDef::Source,
            },
        ),
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color. Spend this mana only to cast a creature spell of the chosen type, and that spell can't be countered.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(
                AddManaEffectDef::any_color()
                .with_restrictions(&CAVERN_COLORED_MANA_RESTRICTIONS)
                .with_spend_effects(&CAVERN_COLORED_MANA_SPEND_EFFECTS),
            ),
        ),
    ]),
);

// AVR 227 — Desolate Lighthouse
pub(in crate::card::sets) static DESOLATE_LIGHTHOUSE: CardRecord = CardRecord::new(
    cards::DESOLATE_LIGHTHOUSE,
    "Desolate Lighthouse",
    CardArt::new("16fb45bc-6152-4b01-9831-a8e80b1c1852", "Scott Chou"),
    CardSet::AvacynRestored,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{1}{U}{R}, {T}: Draw a card, then discard a card.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{U}{R}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::Sequence(&[
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                },
            ]),
        ),
    ]),
);

// AVR 228 — Seraph Sanctuary
pub(in crate::card::sets) static SERAPH_SANCTUARY: CardRecord = CardRecord::new(
    cards::SERAPH_SANCTUARY,
    "Seraph Sanctuary",
    CardArt::new("f903b04a-2733-4ce7-9d83-9db8d5e1e10d", "David Palumbo"),
    CardSet::AvacynRestored,
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::triggered(
            "When this land enters, you gain 1 life.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::triggered(
            "Whenever an Angel you control enters, you gain 1 life.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype("Angel"),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::tap_for(ManaColor::Colorless),
    ]),
);

// AVR 229 — Slayers' Stronghold
pub(in crate::card::sets) static SLAYERS_STRONGHOLD: CardRecord = CardRecord::new(
    cards::SLAYERS_STRONGHOLD,
    "Slayers' Stronghold",
    CardArt::new("939a4351-3ec7-4e6c-8cdd-766bfd670391", "Karl Kopinski"),
    CardSet::AvacynRestored,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{R}{W}, {T}: Target creature gets +2/+0 and gains vigilance and haste until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{R}{W}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(2),
                        toughness: ValueDef::Constant(0),
                    },
                    AppliedEffectDef::GrantAbility(&abilities::vigilance()),
                    AppliedEffectDef::GrantAbility(&abilities::haste()),
                ]),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANGELS_MERCY,
    &ANGELIC_WALL,
    &ARCHANGEL,
    &AVACYN_ANGEL_OF_HOPE,
    &BANISHING_STROKE,
    &CATHARS_CRUSADE,
    &CATHEDRAL_SANCTIFIER,
    &COMMANDERS_AUTHORITY,
    &CURSEBREAK,
    &ENTREAT_THE_ANGELS,
    &GOLDNIGHT_COMMANDER,
    &HOLY_JUSTICIAR,
    &MOONLIGHT_GEIST,
    &MOORLAND_INQUISITOR,
    &RESTORATION_ANGEL,
    &RIGHTEOUS_BLOW,
    &SERAPH_OF_DAWN,
    &TERMINUS,
    &THRABEN_VALIANT,
    &VOICE_OF_THE_PROVINCES,
    &ZEALOUS_STRIKE,
    &ALCHEMISTS_APPRENTICE,
    &DREADWATERS,
    &FLEETING_DISTRACTION,
    &GEIST_SNATCH,
    &GHOSTFORM,
    &GRYFF_VANGUARD,
    &INTO_THE_VOID,
    &LATCH_SEEKER,
    &LUNAR_MYSTIC,
    &MASS_APPEAL,
    &MIST_RAVEN,
    &PEEL_FROM_REALITY,
    &ROTCROWN_GHOUL,
    &VANISHMENT,
    &BLOOD_ARTIST,
    &BLOODFLOW_CONNOISSEUR,
    &BUTCHER_GHOUL,
    &CRYPT_CREEPER,
    &DEATH_WIND,
    &DEMONIC_RISING,
    &DEMONIC_TASKMASTER,
    &DRIVER_OF_THE_DEAD,
    &EVERNIGHT_SHADE,
    &GRISELBRAND,
    &HUMAN_FRAILTY,
    &MAALFELD_TWINS,
    &POLLUTED_DEAD,
    &RENEGADE_DEMON,
    &SEARCHLIGHT_GEIST,
    &SOULCAGE_FIEND,
    &UNDEAD_EXECUTIONER,
    &ARCHWING_DRAGON,
    &BANNERS_RAISED,
    &BATTLE_HYMN,
    &BONFIRE_OF_THE_DAMNED,
    &DEMOLISH,
    &FALKENRATH_EXTERMINATOR,
    &GANG_OF_DEVILS,
    &GUISE_OF_FIRE,
    &HAVENGUL_VAMPIRE,
    &HEIRS_OF_STROMKIRK,
    &HOUND_OF_GRISELBRAND,
    &KESSIG_MALCONTENTS,
    &KRUIN_STRIKER,
    &LIGHTNING_PROWESS,
    &PILLAR_OF_FLAME,
    &RAGING_POLTERGEIST,
    &RIOT_RINGLEADER,
    &RUSH_OF_BLOOD,
    &SCALDING_DEVIL,
    &THUNDEROUS_WRATH,
    &UNCANNY_SPEED,
    &VIGILANTE_JUSTICE,
    &ZEALOUS_CONSCRIPTS,
    &ABUNDANT_GROWTH,
    &BORDERLAND_RANGER,
    &CRATERHOOF_BEHEMOTH,
    &DRUIDS_REPOSITORY,
    &GROUNDED,
    &NATURAL_END,
    &NETTLE_SWINE,
    &RAIN_OF_THORNS,
    &SNARE_THE_SKIES,
    &SOMBERWALD_SAGE,
    &TIMBERLAND_GUIDE,
    &VORSTCLAW,
    &YEW_SPIRIT,
    &SIGARDA_HOST_OF_HERONS,
    &HAUNTED_GUARDIAN,
    &NARSTAD_SCRAPPER,
    &OTHERWORLD_ATLAS,
    &SCROLL_OF_AVACYN,
    &VESSEL_OF_ENDLESS_REST,
    &CAVERN_OF_SOULS,
    &DESOLATE_LIGHTHOUSE,
    &SERAPH_SANCTUARY,
    &SLAYERS_STRONGHOLD,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&alpha::PLAINS),        // AVR 230
    PrintingRecord::alternate(&alpha::PLAINS, 1),   // AVR 231
    PrintingRecord::alternate(&alpha::PLAINS, 2),   // AVR 232
    PrintingRecord::reprint(&alpha::ISLAND),        // AVR 233
    PrintingRecord::alternate(&alpha::ISLAND, 1),   // AVR 234
    PrintingRecord::alternate(&alpha::ISLAND, 2),   // AVR 235
    PrintingRecord::reprint(&alpha::SWAMP),         // AVR 236
    PrintingRecord::alternate(&alpha::SWAMP, 1),    // AVR 237
    PrintingRecord::alternate(&alpha::SWAMP, 2),    // AVR 238
    PrintingRecord::reprint(&alpha::MOUNTAIN),      // AVR 239
    PrintingRecord::alternate(&alpha::MOUNTAIN, 1), // AVR 240
    PrintingRecord::alternate(&alpha::MOUNTAIN, 2), // AVR 241
    PrintingRecord::reprint(&alpha::FOREST),        // AVR 242
    PrintingRecord::alternate(&alpha::FOREST, 1),   // AVR 243
    PrintingRecord::alternate(&alpha::FOREST, 2),   // AVR 244
];
