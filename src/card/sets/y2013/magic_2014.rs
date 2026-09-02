//! Magic 2014 card records used by the built-in ISD–M14 Standard decks.

use super::{CardRecord, PrintingAnchor, PrintingRecord, gatecrash};
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1998::urzas_saga as catalog_usg;
use crate::card::sets::y2002::torment as catalog_tor;
use crate::card::sets::y2007::lorwyn as catalog_lrw;
use crate::card::sets::y2010::magic_2011 as catalog_m11;
use crate::card::sets::y2010::rise_of_the_eldrazi as catalog_roe;
use crate::card::sets::y2011::innistrad as catalog_isd;
use crate::card::sets::y2012::magic_2013 as catalog_m13;
use crate::card::sets::y2013::gatecrash as catalog_gtc;
use crate::card::sets::{
    y1993::alpha,
    y1994::antiquities,
    y2002::onslaught,
    y2004::darksteel,
    y2011::innistrad,
    y2012::{avacyn_restored, dark_ascension, magic_2013, return_to_ravnica},
};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, BasicLandType, BattlefieldEntryModificationDef, CardArt,
    CardRules, CardSet, CardSupertype, CardType, CardTypeSet, CastTimingPermissionDef,
    ChoiceVisibilityDef, ChooseDef, ClassifyObjectsDef, ColorChoiceOperationDef, ColorSet,
    ComparisonDef, CounterKind, CreatureTypeSetDef, DamageEventMatcherDef, DiscardSelectionDef,
    EffectDef, EffectRecipientDef, HalvedValueDef, ManaColor, MoveObjectsDef,
    ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef, ObjectSetDef, PlayerRefDef,
    PlayerRelation, ReplacementEffectDef, ReplacementEventDef, ResolvedEffectDurationDef,
    RoundingDef, SacrificedAmountDef, ScaledValueDef, TargetConditionDef, TriggerConditionDef,
    TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement, abilities, tokens,
};
use crate::ids::{Binding, ParentBinding, TargetIndex};
use crate::mana_cost;

static TAPPED_ZOMBIE: EffectDef =
    EffectDef::create_creature_token(&["Zombie"], &[ManaColor::Black], 2, 2)
        .with_art(CardArt::new(
            "07d82a8d-4c57-401f-92c3-8fd9ba20174a",
            "Lucas Graciano",
        ))
        .entering_tapped();

// M14 1 — Ajani, Caller of the Pride (reprint)

// M14 2 — Ajani's Chosen
// Audit: metadata-only — CreateToken cannot continue by conditionally attaching the triggering Aura to the newly created token.
pub(in crate::card::sets) static AJANI_S_CHOSEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("583bfbc1-638b-4de5-b865-0b00a69dd073"),
    "Ajani's Chosen",
    crate::card::CardArt::new("583bfbc1-638b-4de5-b865-0b00a69dd073", "Wayne Reynolds"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 3 — Angelic Accord
// Audit: metadata-only — Trigger conditions cannot track life gained during the turn or compare that total with four.
pub(in crate::card::sets) static ANGELIC_ACCORD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("03f26bc2-53d7-4448-8021-de35aa82fcc6"),
    "Angelic Accord",
    crate::card::CardArt::new("03f26bc2-53d7-4448-8021-de35aa82fcc6", "Michael C. Hayes"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 4 — Angelic Wall (reprint)

// M14 5 — Archangel of Thune
pub(in crate::card::sets) static ARCHANGEL_OF_THUNE: CardRecord = CardRecord::new_with_legacy_id(
    133,
    "Archangel of Thune",
    CardArt::new("531cba81-afd7-4be4-adec-87edb77ba2a9", "James Ryman"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Angel"], 3, 4).with_abilities(&[
        abilities::flying(),
        abilities::lifelink(),
        AbilityDef::triggered(
            "Whenever you gain life, put a +1/+1 counter on each creature you control.",
            TriggerEventDef::LifeGained(PlayerRelation::You),
            EffectDef::AddCounters {
                kind: CounterKind::PlusOnePlusOne,
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                // One counter however much life arrived, and one trigger for
                // each separate gain.
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// M14 6 — Auramancer
pub(in crate::card::sets) static AURAMANCER: CardRecord = CardRecord::new_with_legacy_id(
    1144,
    "Auramancer",
    CardArt::new("0a3dc4ab-1c45-4495-91b6-27d62087380c", "Rebecca Guay"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Wizard"], 2, 2).with_ability(
        abilities::enters_trigger_with_targets("When this creature enters, you may return target enchantment card from your graveyard to your hand.", &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Enchantment),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            })], EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
},
            }),
    ),
);

// M14 7 — Banisher Priest
// Audit: metadata-only — Linked exile cannot express Banisher Priest's one-shot duration, which must do nothing if the source left before the enter trigger resolved.
pub(in crate::card::sets) static BANISHER_PRIEST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("06823bf8-2fca-49e1-ba40-9b61c9ae55b3"),
    "Banisher Priest",
    crate::card::CardArt::new("06823bf8-2fca-49e1-ba40-9b61c9ae55b3", "Willian Murai"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 8 — Blessing (reprint)

// M14 9 — Bonescythe Sliver
pub(in crate::card::sets) static BONESCYTHE_SLIVER: CardRecord = CardRecord::new_with_legacy_id(
    1145,
    "Bonescythe Sliver",
    CardArt::new("a26bb68b-1830-470a-8cea-91edc7db0c57", "Trevor Claxton"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Sliver"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "Sliver creatures you control have double strike.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Sliver"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::double_strike()),
            },
        ),
    ),
);

// M14 10 — Brave the Elements
pub(in crate::card::sets) static BRAVE_THE_ELEMENTS: CardRecord = CardRecord::new_with_legacy_id(
    1996,
    "Brave the Elements",
    CardArt::new("097d7838-ae58-4306-ba0f-e914601b31b6", "Goran Josic"),
    CardSet::Magic2014,
    // One mana that makes a white board unblockable, or immune to a sweeper:
    // the group is settled first and the colour named afterwards.
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell(
        "Choose a color. White creatures you control gain protection from the chosen color until end of turn.",
        EffectDef::ChooseColor {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::White),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            operation: ColorChoiceOperationDef::ProtectionFromChosenColor,
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M14 11 — Capashen Knight
pub(in crate::card::sets) static CAPASHEN_KNIGHT: CardRecord = CardRecord::new_with_legacy_id(
    1146,
    "Capashen Knight",
    CardArt::new("78802af4-46b5-4bac-8cdf-5b77d0b19895", "Jasper Sandner"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Knight"], 1, 1).with_abilities(&[
        abilities::first_strike(),
        AbilityDef::activated(
            "{1}{W}: This creature gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M14 12 — Celestial Flare
pub(in crate::card::sets) static CELESTIAL_FLARE: CardRecord = CardRecord::new_with_legacy_id(
    148,
    "Celestial Flare",
    CardArt::new("6c8d1320-0f1a-4c66-86c9-9f8da0f1d9ef", "Clint Cearley"),
    CardSet::Magic2014,
    CardRules::new_instant(mana_cost!("{W}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target player sacrifices an attacking or blocking creature of their choice.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::SacrificeOfChoice {
            count: ValueDef::Constant(1),
            player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::AttackingOrBlocking,
            ]),
            then: None,
            amount: SacrificedAmountDef::Power,
            otherwise: None,
            optional: false,
        },
    )),
);

// M14 13 — Charging Griffin
pub(in crate::card::sets) static CHARGING_GRIFFIN: CardRecord = CardRecord::new_with_legacy_id(
    1147,
    "Charging Griffin",
    CardArt::new("88637cc0-3b2a-402c-b491-26fcc2d21fb8", "Erica Yang"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Griffin"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever this creature attacks, it gets +1/+1 until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M14 14 — Congregate
pub(in crate::card::sets) static CONGREGATE: CardRecord = CardRecord::new_with_legacy_id(
    1861,
    "Congregate",
    CardArt::new("b792574a-4d8f-4c80-a958-7c0edbe391fc", "Mark Zug"),
    CardSet::Magic2014,
    CardRules::new_instant(mana_cost!("{3}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target player gains 2 life for each creature on the battlefield.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Scaled(
                &// Every creature on the battlefield, both sides included: the card says "on
                // the battlefield" rather than "you control".
                ScaledValueDef::new(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                )), 2),
            ),
        },
    )),
);

// M14 15 — Dawnstrike Paladin
pub(in crate::card::sets) static DAWNSTRIKE_PALADIN: CardRecord = CardRecord::new_with_legacy_id(
    1148,
    "Dawnstrike Paladin",
    CardArt::new("93cf5fb3-bb41-4efa-9721-2c2d169b05cd", "Tyler Jacobson"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Human", "Knight"], 2, 4)
        .with_abilities(&[abilities::vigilance(), abilities::lifelink()]),
);

// M14 16 — Devout Invocation
// Audit: metadata-only — Spell costs cannot tap a freely chosen number of creatures and carry that paid count into token creation.
pub(in crate::card::sets) static DEVOUT_INVOCATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8a286954-fb40-4440-9f0e-a28367c6823c"),
    "Devout Invocation",
    crate::card::CardArt::new("8a286954-fb40-4440-9f0e-a28367c6823c", "David Palumbo"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 17 — Divine Favor (reprint)

// M14 18 — Fiendslayer Paladin
// Audit: metadata-only — Targeting restrictions cannot filter opposing spell sources by black or red color while leaving abilities unaffected.
pub(in crate::card::sets) static FIENDSLAYER_PALADIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5cfb0f4a-e273-4ffb-91cd-dd1a7b6f6a8f"),
    "Fiendslayer Paladin",
    crate::card::CardArt::new("5cfb0f4a-e273-4ffb-91cd-dd1a7b6f6a8f", "Wesley Burt"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 19 — Fortify
pub(in crate::card::sets) static FORTIFY: CardRecord = CardRecord::new_with_legacy_id(
    1149,
    "Fortify",
    CardArt::new(
        "1eff4028-d4f9-4822-81d6-9f5e5e6f3011",
        "Christopher Moeller",
    ),
    CardSet::Magic2014,
    CardRules::new_instant(mana_cost!("{2}{W}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "Creatures you control get +2/+0 until end of turn.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell(
                "Creatures you control get +0/+2 until end of turn.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(0),
                        ValueDef::Constant(2),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    )),
);

// M14 20 — Griffin Sentinel
pub(in crate::card::sets) static GRIFFIN_SENTINEL: CardRecord = CardRecord::new_with_legacy_id(
    1150,
    "Griffin Sentinel",
    CardArt::new("b40d6626-a85f-4116-9721-19e39b83cba0", "Warren Mahy"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Griffin"], 1, 3)
        .with_abilities(&[abilities::flying(), abilities::vigilance()]),
);

// M14 21 — Hive Stirrings
pub(in crate::card::sets) static HIVE_STIRRINGS: CardRecord = CardRecord::new_with_legacy_id(
    1151,
    "Hive Stirrings",
    CardArt::new("e4399e19-d05d-4bb3-9aff-c4133ddd2850", "Maciej Kuciara"),
    CardSet::Magic2014,
    CardRules::new_sorcery(mana_cost!("{2}{W}")).with_ability(AbilityDef::spell(
        "Create two 1/1 colorless Sliver creature tokens.",
        EffectDef::create_creature_token(&["Sliver"], &[], 1, 1)
            .with_art(CardArt::new(
                "68353af0-9cd0-43c0-9b39-8f904c618e3a",
                "Igor Kieryluk",
            ))
            .with_amount(2),
    )),
);

// M14 22 — Imposing Sovereign
pub(in crate::card::sets) static IMPOSING_SOVEREIGN: CardRecord = CardRecord::new_with_legacy_id(
    1152,
    "Imposing Sovereign",
    CardArt::new("0f672328-3361-498e-a9f4-2d8e69a8b072", "Scott M. Fischer"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Noble"], 2, 1).with_ability(
        AbilityDef::replacement_for(
            "Creatures your opponents control enter tapped.",
            ReplacementEventDef::ObjectEntersBattlefield {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::Opponent,
                cast: None,
            },
            ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
        ),
    ),
);

// M14 23 — Indestructibility
pub(in crate::card::sets) static INDESTRUCTIBILITY: CardRecord = CardRecord::new_with_legacy_id(
    1153,
    "Indestructibility",
    CardArt::new("e086a062-d39b-4e2a-bde0-f4d6d1797a5f", "Darrell Riche"),
    CardSet::Magic2014,
    CardRules::new_enchantment(mana_cost!("{3}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant permanent",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::Any,
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted permanent has indestructible.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                },
            ),
        ]),
);

// M14 24 — Master of Diversion
// Audit: metadata-only — Attack triggers cannot restrict a target to the creature controlled by that attack's defending player.
pub(in crate::card::sets) static MASTER_OF_DIVERSION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2bec89b3-640e-4093-a6e9-5639610769b9"),
    "Master of Diversion",
    crate::card::CardArt::new("2bec89b3-640e-4093-a6e9-5639610769b9", "Michael Komarck"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 25 — Pacifism (reprint)

// M14 26 — Path of Bravery
// Audit: metadata-only — Continuous effects cannot compare current and starting life, and attack events cannot aggregate one combat's attackers into one trigger amount.
pub(in crate::card::sets) static PATH_OF_BRAVERY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5222e200-df1b-46c6-a194-c341e8c1d516"),
    "Path of Bravery",
    crate::card::CardArt::new("5222e200-df1b-46c6-a194-c341e8c1d516", "Chris Rahn"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 27 — Pay No Heed (reprint)

// M14 28 — Pillarfield Ox (reprint)

// M14 29 — Planar Cleansing (reprint)

// M14 30 — Sentinel Sliver
pub(in crate::card::sets) static SENTINEL_SLIVER: CardRecord = CardRecord::new_with_legacy_id(
    1154,
    "Sentinel Sliver",
    CardArt::new("74c28560-e6ac-4be9-a253-22c4613b0d90", "Maciej Kuciara"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Sliver"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "Sliver creatures you control have vigilance.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Sliver"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::vigilance()),
            },
        ),
    ),
);

// M14 31 — Seraph of the Sword
pub(in crate::card::sets) static SERAPH_OF_THE_SWORD: CardRecord = CardRecord::new_with_legacy_id(
    1745,
    "Seraph of the Sword",
    CardArt::new("6caa91aa-f175-40cd-b984-f37cb2cae7db", "Jaime Jones"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Angel"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "Prevent all combat damage that would be dealt to this creature.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::PreventDamage(
                    DamageEventMatcherDef::COMBAT_TO_AFFECTED,
                )),
            },
        ),
    ]),
);

// M14 32 — Serra Angel (reprint)

// M14 33 — Show of Valor (reprint)

// M14 34 — Siege Mastodon
pub(in crate::card::sets) static SIEGE_MASTODON: CardRecord = CardRecord::new_with_legacy_id(
    1155,
    "Siege Mastodon",
    CardArt::new("40e7a30f-bb29-4c6b-bf70-53e9e4292814", "Matt Cavotta"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Elephant"], 3, 5),
);

// M14 35 — Silence (reprint)

// M14 36 — Solemn Offering
pub(in crate::card::sets) static SOLEMN_OFFERING: CardRecord = CardRecord::new_with_legacy_id(
    1156,
    "Solemn Offering",
    CardArt::new("9ca09fed-f9b3-49ee-be89-404581a4cbd2", "Sam Wood"),
    CardSet::Magic2014,
    CardRules::new_sorcery(mana_cost!("{2}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target artifact or enchantment. You gain 4 life.",
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
                then: None,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(4),
            },
        ]),
    )),
);

// M14 37 — Soulmender
pub(in crate::card::sets) static SOULMENDER: CardRecord = CardRecord::new_with_legacy_id(
    1157,
    "Soulmender",
    CardArt::new("37f45133-6134-4664-9952-67c03d60f9a0", "James Ryman"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Cleric"], 1, 1).with_ability(
        AbilityDef::activated(
            "{T}: You gain 1 life.",
            &[AbilityCostDef::TapSource],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// M14 38 — Steelform Sliver
pub(in crate::card::sets) static STEELFORM_SLIVER: CardRecord = CardRecord::new_with_legacy_id(
    1158,
    "Steelform Sliver",
    CardArt::new("c15d6329-ffb1-43fd-8558-60c8315f5b91", "Chase Stone"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Sliver"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "Sliver creatures you control get +0/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Sliver"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(1),
                ),
            },
        ),
    ),
);

// M14 39 — Stonehorn Chanter
pub(in crate::card::sets) static STONEHORN_CHANTER: CardRecord = CardRecord::new_with_legacy_id(
    1159,
    "Stonehorn Chanter",
    CardArt::new("cd6ec61b-c039-4526-a359-a7947eeba5c3", "Raymond Swanland"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{5}{W}"), &["Rhino", "Cleric"], 4, 4).with_ability(
        AbilityDef::activated(
            "{5}{W}: This creature gains vigilance and lifelink until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{5}{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_ability(&abilities::vigilance()),
                    AppliedEffectDef::add_ability(&abilities::lifelink()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .once_each_turn(),
    ),
);

// M14 40 — Suntail Hawk
pub(in crate::card::sets) static SUNTAIL_HAWK: CardRecord = CardRecord::new_with_legacy_id(
    1160,
    "Suntail Hawk",
    CardArt::new("28a1f83c-a9ef-463e-97b5-2ca3b7232f82", "Heather Hudson"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{W}"), &["Bird"], 1, 1)
        .with_abilities(&[abilities::flying()]),
);

// M14 41 — Wall of Swords (reprint)

// M14 42 — Air Servant
pub(in crate::card::sets) static AIR_SERVANT: CardRecord = CardRecord::new_with_legacy_id(
    1161,
    "Air Servant",
    CardArt::new("0cbc279d-952a-4b8d-b6ff-37166daa2dd5", "Lars Grant-West"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Elemental"], 4, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_with_targets(
            "{2}{U}: Tap target creature with flying.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{U}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Flying),
                ]),
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// M14 43 — Archaeomancer (reprint)

// M14 44 — Armored Cancrix
pub(in crate::card::sets) static ARMORED_CANCRIX: CardRecord = CardRecord::new_with_legacy_id(
    1162,
    "Armored Cancrix",
    CardArt::new("3b455b0f-a69c-43b4-bbf5-605ed41f10e0", "Tomasz Jedruszek"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Crab"], 2, 5),
);

// M14 45 — Cancel (reprint)

// M14 46 — Claustrophobia (reprint)

// M14 47 — Clone (reprint)

// M14 48 — Colossal Whale
// Audit: metadata-only — Islandwalk and defending-player targeting are unsupported, and linked exile cannot express the required one-shot duration if the source leaves early.
pub(in crate::card::sets) static COLOSSAL_WHALE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("20968c42-d63b-4b2a-ae47-f986f3c35fdc"),
    "Colossal Whale",
    crate::card::CardArt::new("f7f7caca-14ee-4d6a-97c3-e19898f86635", "Adam Paquette"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 49 — Coral Merfolk
pub(in crate::card::sets) static CORAL_MERFOLK: CardRecord = CardRecord::new_with_legacy_id(
    1163,
    "Coral Merfolk",
    CardArt::new("09ef366b-26f5-473a-ab96-e668ed54d691", "rk post"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Merfolk"], 2, 1),
);

// M14 50 — Dismiss into Dream
// Audit: metadata-only — Static effects cannot add a creature subtype, and no trigger event observes a permanent becoming a target.
pub(in crate::card::sets) static DISMISS_INTO_DREAM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("af4cd7fe-639c-45a5-97af-9529904e3975"),
    "Dismiss into Dream",
    crate::card::CardArt::new("af4cd7fe-639c-45a5-97af-9529904e3975", "Sam Wolfe Connelly"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 51 — Disperse
pub(in crate::card::sets) static DISPERSE: CardRecord = CardRecord::new_with_legacy_id(
    1164,
    "Disperse",
    CardArt::new("e6b415d2-53fe-4540-aea6-9cd2c498134c", "Steve Ellis"),
    CardSet::Magic2014,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return target nonland permanent to its owner's hand.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
        )],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
        },
    )),
);

// M14 52 — Divination (reprint)

// M14 53 — Domestication (reprint)

// M14 54 — Elite Arcanist
// Audit: metadata-only — Imprint cannot retain a chosen hand card for a later X cost, spell copy, and free-cast permission.
pub(in crate::card::sets) static ELITE_ARCANIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("99b225fe-c07d-4d8a-bf2b-c1777bd29061"),
    "Elite Arcanist",
    crate::card::CardArt::new("99b225fe-c07d-4d8a-bf2b-c1777bd29061", "James Zapata"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 55 — Essence Scatter (reprint)

// M14 56 — Frost Breath
pub(in crate::card::sets) static FROST_BREATH: CardRecord = CardRecord::new_with_legacy_id(
    1850,
    "Frost Breath",
    CardArt::new("85d3f777-7660-48ae-8c32-6777ec8427d4", "Mike Bierek"),
    CardSet::Magic2014,
    CardRules::new_instant(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Tap up to two target creatures. Those creatures don't untap during their controller's \
         next untap step.",
        // "Up to two", so nothing at all is a legal declaration, and the skip is
        // counted on each creature separately -- the two may belong to different
        // players, who do not reach their untap steps together.
        &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
            2,
        )],
        EffectDef::Sequence(&[
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
            EffectDef::SkipNextUntapSteps {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                count: 1,
            },
        ]),
    )),
);

// M14 57 — Galerider Sliver
pub(in crate::card::sets) static GALERIDER_SLIVER: CardRecord = CardRecord::new_with_legacy_id(
    1165,
    "Galerider Sliver",
    CardArt::new("425f5d1b-9989-4fd1-88e2-6c3108aefa0b", "James Zapata"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{U}"), &["Sliver"], 1, 1).with_ability(
        AbilityDef::static_ability(
            "Sliver creatures you control have flying.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Sliver"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::flying()),
            },
        ),
    ),
);

// M14 58 — Glimpse the Future
pub(in crate::card::sets) static GLIMPSE_THE_FUTURE: CardRecord = CardRecord::new_with_legacy_id(
    1166,
    "Glimpse the Future",
    CardArt::new("f4d875e9-713d-4ddb-ae0a-db8483366319", "Andrew Robinson"),
    CardSet::Magic2014,
    CardRules::new_sorcery(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell(
        "Look at the top three cards of your library. Put one of them into your hand and the rest into your graveyard.",
        abilities::look_at_top_cards_choose_to_hand_rest_graveyard(
            ValueDef::Constant(3),
            ObjectPredicateDef::Any,
            1,
            1,
        ),
    )),
);

// M14 59 — Illusionary Armor
// Audit: metadata-only — No trigger event observes the enchanted creature becoming a target.
pub(in crate::card::sets) static ILLUSIONARY_ARMOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d09346ff-6e63-499b-9265-c15a7b2cdece"),
    "Illusionary Armor",
    crate::card::CardArt::new("d09346ff-6e63-499b-9265-c15a7b2cdece", "Mathias Kollros"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 60 — Jace, Memory Adept (reprint)

// M14 61 — Jace's Mindseeker
// Audit: metadata-only — Mill cannot retain the exact milled batch for a filtered optional free-cast choice.
pub(in crate::card::sets) static JACE_S_MINDSEEKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f67852a6-ae75-44e7-9e2d-d458c7b9d869"),
    "Jace's Mindseeker",
    crate::card::CardArt::new("f67852a6-ae75-44e7-9e2d-d458c7b9d869", "Greg Staples"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 62 — Merfolk Spy (reprint)

// M14 63 — Messenger Drake
pub(in crate::card::sets) static MESSENGER_DRAKE: CardRecord = CardRecord::new_with_legacy_id(
    1167,
    "Messenger Drake",
    CardArt::new("13dd3172-0b45-4dc8-adc6-9e0ba112e664", "Yeong-Hao Han"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{3}{U}{U}"), &["Drake"], 3, 3).with_abilities(&[
        abilities::flying(),
        abilities::dies_trigger(
            "When this creature dies, draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// M14 64 — Negate (reprint)

// M14 65 — Nephalia Seakite (reprint)

// M14 66 — Opportunity
pub(in crate::card::sets) static OPPORTUNITY: CardRecord = CardRecord::new_with_legacy_id(
    1168,
    "Opportunity",
    CardArt::new("e1b242f3-9398-4d65-a2c7-4de56ee58933", "Allen Williams"),
    CardSet::Magic2014,
    CardRules::new_instant(mana_cost!("{4}{U}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target player draws four cards.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(4),
        },
    )),
);

// M14 67 — Phantom Warrior
pub(in crate::card::sets) static PHANTOM_WARRIOR: CardRecord = CardRecord::new_with_legacy_id(
    1169,
    "Phantom Warrior",
    CardArt::new("e12a1a64-5b32-4b85-8fae-c407d7926547", "Greg Staples"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Illusion", "Warrior"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "This creature can't be blocked.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::Any,
                )),
            },
        ),
    ),
);

// M14 68 — Quicken
pub(in crate::card::sets) static QUICKEN: CardRecord = CardRecord::new_with_legacy_id(
    199,
    "Quicken",
    CardArt::new("066bef3d-c785-4b25-9b91-8f676aa9906f", "Aleksi Briclot"),
    CardSet::Magic2014,
    // One spell ability per part, so the card's two sentences are one clause
    // with a sequence rather than two spell clauses.
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell(
        "The next sorcery spell you cast this turn can be cast as though it had flash. (It can be cast any time you could cast an instant.)\nDraw a card.",
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayCastAsThoughItHadFlash(
                    CastTimingPermissionDef::new(ObjectPredicateDef::HasType(CardType::Sorcery)),
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn
                    .or(ResolvedEffectDurationDef::UntilNextMatchingCast),
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// M14 69 — Scroll Thief (reprint)

// M14 70 — Seacoast Drake
pub(in crate::card::sets) static SEACOAST_DRAKE: CardRecord = CardRecord::new_with_legacy_id(
    1170,
    "Seacoast Drake",
    CardArt::new("5333de10-a6d4-47ff-ab57-4edb49535739", "Scott Chou"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Drake"], 1, 3)
        .with_abilities(&[abilities::flying()]),
);

// M14 71 — Sensory Deprivation (reprint)

// M14 72 — Spell Blast (reprint)

// M14 73 — Tidebinder Mage
// Audit: metadata-only — Effect durations cannot end when the source changes controller while remaining on the battlefield.
pub(in crate::card::sets) static TIDEBINDER_MAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e032d1dd-6efc-4f6c-ad3b-30fe74845edf"),
    "Tidebinder Mage",
    crate::card::CardArt::new(
        "e032d1dd-6efc-4f6c-ad3b-30fe74845edf",
        "John Severin Brassell",
    ),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 74 — Time Ebb
pub(in crate::card::sets) static TIME_EBB: CardRecord = CardRecord::new_with_legacy_id(
    1171,
    "Time Ebb",
    CardArt::new("bf0c48f6-8b2e-4eff-aa1e-10e6ccae426a", "Alan Rabinowitz"),
    CardSet::Magic2014,
    CardRules::new_sorcery(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Put target creature on top of its owner's library.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Library,
            placement: ZonePlacement::Top,
        },
    )),
);

// M14 75 — Tome Scour
pub(in crate::card::sets) static TOME_SCOUR: CardRecord = CardRecord::new_with_legacy_id(
    1172,
    "Tome Scour",
    CardArt::new("aed4cfec-5cea-4987-890e-825b2802e9f9", "Steven Belledin"),
    CardSet::Magic2014,
    CardRules::new_sorcery(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target player mills five cards.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Mill {
            player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(5),
        },
    )),
);

// M14 76 — Trained Condor
pub(in crate::card::sets) static TRAINED_CONDOR: CardRecord = CardRecord::new_with_legacy_id(
    1173,
    "Trained Condor",
    CardArt::new("6e1eaa5a-3f9d-4166-b418-fd82fff86c73", "Alex Horley-Orlandelli"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Bird"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, another target creature you control gains flying until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
            ]))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::flying()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M14 77 — Traumatize
pub(in crate::card::sets) static TRAUMATIZE: CardRecord = CardRecord::new_with_legacy_id(
    2008,
    "Traumatize",
    CardArt::new("9b8784dd-83f9-41f8-aedc-f0f81073ffcb", "Greg Staples"),
    CardSet::Magic2014,
    CardRules::new_sorcery(mana_cost!("{3}{U}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target player mills half their library, rounded down.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Mill {
            player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            // Half of what the library holds when the spell resolves, rounded down.
            // Reading it from the target rather than from a fixed count is the whole
            // clause: a Traumatize into an empty library mills nothing.
            amount: ValueDef::Halved(&HalvedValueDef::new(
                ValueDef::TargetLibrarySize(TargetIndex::PRIMARY),
                RoundingDef::Down,
            )),
        },
    )),
);

// M14 78 — Wall of Frost
pub(in crate::card::sets) static WALL_OF_FROST: CardRecord = CardRecord::new_with_legacy_id(
    1862,
    "Wall of Frost",
    CardArt::new("d4000b46-7843-4c07-8332-a10f207e2cdc", "Mike Bierek"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Wall"], 0, 7).with_abilities(&[
        abilities::defender(),
        // The blocked creature is the trigger's own object, so the skip
        // lands on it rather than on whatever else is in the combat.
        AbilityDef::triggered(
            "Whenever this creature blocks a creature, that creature doesn't untap during its \
             controller's next untap step.",
            TriggerEventDef::Blocks {
                blocked: ObjectPredicateDef::Any,
            },
            EffectDef::SkipNextUntapSteps {
                object: EffectRecipientDef::TriggeringObject,
                count: 1,
            },
        ),
    ]),
);

// M14 79 — Warden of Evos Isle
// Audit: metadata-only — Generic-cost reduction cannot be filtered to creature spells with effective flying.
pub(in crate::card::sets) static WARDEN_OF_EVOS_ISLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2316d281-21a4-460d-9062-f0737249484e"),
    "Warden of Evos Isle",
    crate::card::CardArt::new("2316d281-21a4-460d-9062-f0737249484e", "Nils Hamm"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 80 — Water Servant
pub(in crate::card::sets) static WATER_SERVANT: CardRecord = CardRecord::new_with_legacy_id(
    1174,
    "Water Servant",
    CardArt::new("a2c7562e-3e25-447d-b9f4-eb96960511b8", "Igor Kieryluk"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Elemental"], 3, 4).with_abilities(&[
        AbilityDef::activated(
            "{U}: This creature gets +1/-1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(-1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated(
            "{U}: This creature gets -1/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M14 81 — Windreader Sphinx
pub(in crate::card::sets) static WINDREADER_SPHINX: CardRecord = CardRecord::new_with_legacy_id(
    1175,
    "Windreader Sphinx",
    CardArt::new("f566741d-a847-4f24-b6fc-7873f0797d59", "Min Yum"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{5}{U}{U}"), &["Sphinx"], 3, 7).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever a creature with flying attacks, you may draw a card.",
            TriggerEventDef::attacks(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Flying),
            ])),
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

// M14 82 — Zephyr Charge
pub(in crate::card::sets) static ZEPHYR_CHARGE: CardRecord = CardRecord::new_with_legacy_id(
    1176,
    "Zephyr Charge",
    CardArt::new("f9ea2808-0dde-4065-ae7d-905aae98703f", "Steve Prescott"),
    CardSet::Magic2014,
    CardRules::new_enchantment(mana_cost!("{1}{U}")).with_ability(
        AbilityDef::activated_with_targets(
            "{1}{U}: Target creature gains flying until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{U}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::flying()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// M14 83 — Accursed Spirit
pub(in crate::card::sets) static ACCURSED_SPIRIT: CardRecord = CardRecord::new_with_legacy_id(
    1177,
    "Accursed Spirit",
    CardArt::new("cf08313b-14c9-4e0b-aad7-05cbd90b1ed8", "Kev Walker"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Spirit"], 3, 2)
        .with_abilities(&[abilities::intimidate()]),
);

// M14 84 — Altar's Reap (reprint)

// M14 85 — Artificer's Hex
// Audit: metadata-only — Conditions and recipients cannot follow an Aura to an Equipment and then to the creature that Equipment is attached to.
pub(in crate::card::sets) static ARTIFICER_S_HEX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3a5cd9a1-da2e-44ef-9f2e-352dc9f92c50"),
    "Artificer's Hex",
    crate::card::CardArt::new("3a5cd9a1-da2e-44ef-9f2e-352dc9f92c50", "Franz Vohwinkel"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 86 — Blightcaster
pub(in crate::card::sets) static BLIGHTCASTER: CardRecord = CardRecord::new_with_legacy_id(
    1178,
    "Blightcaster",
    CardArt::new("61752b13-255a-44d0-9fb0-5ed5680b954e", "Winona Nelson"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Human", "Wizard"], 2, 3).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever you cast an enchantment spell, you may have target creature get -2/-2 until end of turn.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Enchantment),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(-2), ValueDef::Constant(-2)),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            },
        ),
    ),
);

// M14 87 — Blood Bairn
pub(in crate::card::sets) static BLOOD_BAIRN: CardRecord = CardRecord::new_with_legacy_id(
    1179,
    "Blood Bairn",
    CardArt::new("a3fcbbd1-ee51-42a3-ad11-2fd41728c35d", "Ryan Yee"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Vampire"], 2, 2).with_ability(
        AbilityDef::activated(
            "Sacrifice another creature: This creature gets +2/+2 until end of turn.",
            &[AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                controller: PlayerRelation::You,
            }],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// M14 88 — Bogbrew Witch
// Audit: metadata-only — SearchZone cannot match either exact printed name or make the selected permanent enter tapped.
pub(in crate::card::sets) static BOGBREW_WITCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7559cf3e-7fad-4bcf-8551-045f9150e014"),
    "Bogbrew Witch",
    crate::card::CardArt::new("7559cf3e-7fad-4bcf-8551-045f9150e014", "Eric Deschamps"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 89 — Child of Night
pub(in crate::card::sets) static CHILD_OF_NIGHT: CardRecord = CardRecord::new_with_legacy_id(
    1180,
    "Child of Night",
    CardArt::new("c21b5476-5f5f-46b5-b627-398e9fcd04aa", "Ash Wood"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Vampire"], 2, 1)
        .with_abilities(&[abilities::lifelink()]),
);

// M14 90 — Corpse Hauler
pub(in crate::card::sets) static CORPSE_HAULER: CardRecord = CardRecord::new_with_legacy_id(
    1181,
    "Corpse Hauler",
    CardArt::new("ca6adc5e-9221-4a18-8d41-4675797e5d46", "Jesper Ejsing"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Rogue"], 2, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{2}{B}, Sacrifice this creature: Return another target creature card from your graveyard to your hand.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{B}")),
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            })],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
},
        ),
    ),
);

// M14 91 — Corrupt (reprint)

// M14 92 — Dark Favor (reprint)

// M14 93 — Dark Prophecy
pub(in crate::card::sets) static DARK_PROPHECY: CardRecord = CardRecord::new_with_legacy_id(
    1182,
    "Dark Prophecy",
    CardArt::new("ecf82c3b-7a35-43dd-8bf3-ebc68dc1b8fc", "Scott Chou"),
    CardSet::Magic2014,
    CardRules::new_enchantment(mana_cost!("{B}{B}{B}")).with_ability(AbilityDef::triggered(
        "Whenever a creature you control dies, you draw a card and you lose 1 life.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ]),
            Some(ZoneKind::Battlefield),
            Some(ZoneKind::Graveyard),
        ),
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// M14 94 — Deathgaze Cockatrice
pub(in crate::card::sets) static DEATHGAZE_COCKATRICE: CardRecord = CardRecord::new_with_legacy_id(
    1183,
    "Deathgaze Cockatrice",
    CardArt::new("9f17b58c-9738-4cdb-a408-e1595c384b92", "Kev Walker"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Cockatrice"], 2, 2)
        .with_abilities(&[abilities::flying(), abilities::deathtouch()]),
);

// M14 95 — Diabolic Tutor
pub(in crate::card::sets) static DIABOLIC_TUTOR: CardRecord = CardRecord::new_with_legacy_id(
    1184,
    "Diabolic Tutor",
    CardArt::new("d75a7c8b-f29f-4574-96c0-daac17fc75bb", "Greg Staples"),
    CardSet::Magic2014,
    CardRules::new_sorcery(mana_cost!("{2}{B}{B}")).with_ability(AbilityDef::spell(
        "Search your library for a card, put that card into your hand, then shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::Any,
            minimum: 1,
            maximum: ValueDef::Constant(1),
            reveal: false,
            destination: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            attachment: None,
            binding: None,
            then: None,
        },
    )),
);

// M14 96 — Doom Blade
pub(in crate::card::sets) static DOOM_BLADE: CardRecord = CardRecord::new_with_legacy_id(
    158,
    "Doom Blade",
    CardArt::new("75d96a37-bdbe-46ae-926f-8742699a0b20", "Chippy"),
    CardSet::Magic2014,
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(AbilityDef::destroy_target(
        "Destroy target nonblack creature.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::Black)),
        ])),
        true,
    )),
);

// M14 97 — Duress (reprint)

// M14 98 — Festering Newt
// Audit: metadata-only — Object predicates cannot test for a different exact card name to choose between two effect amounts.
pub(in crate::card::sets) static FESTERING_NEWT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eaee5261-416c-41e9-9ad7-bf7bd169aa08"),
    "Festering Newt",
    crate::card::CardArt::new("eaee5261-416c-41e9-9ad7-bf7bd169aa08", "Eric Deschamps"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 99 — Gnawing Zombie
pub(in crate::card::sets) static GNAWING_ZOMBIE: CardRecord = CardRecord::new_with_legacy_id(
    1185,
    "Gnawing Zombie",
    CardArt::new("56653d9e-0c29-440b-8724-cae746abb1a9", "Greg Staples"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Zombie"], 1, 3).with_ability(
        AbilityDef::activated_with_targets(
            "{1}{B}, Sacrifice a creature: Target player loses 1 life and you gain 1 life.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{B}")),
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    controller: PlayerRelation::You,
                },
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
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

// M14 100 — Grim Return
// Audit: metadata-only — Graveyard target predicates cannot inspect whether a card moved there from the battlefield this turn.
pub(in crate::card::sets) static GRIM_RETURN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("15b69f74-3b54-4db4-abf3-b71db8cc9562"),
    "Grim Return",
    crate::card::CardArt::new("15b69f74-3b54-4db4-abf3-b71db8cc9562", "Seb McKinnon"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 101 — Lifebane Zombie
pub(in crate::card::sets) static LIFEBANE_ZOMBIE: CardRecord = CardRecord::new_with_legacy_id(
    183,
    "Lifebane Zombie",
    CardArt::new("98370735-5303-40d4-9e80-cdb40dee18e2", "Min Yum"),
    CardSet::Magic2014,
    CardRules::new_creature(
        mana_cost!("{1}{B}{B}"),
        &["Zombie", "Warrior"],
        3,
        1,
    )
    .with_abilities(&[
        abilities::intimidate(),
        abilities::enters_trigger_with_targets("When this creature enters, target opponent reveals their hand. You choose a green or white creature card from it and exile that card.", &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Opponent),
        )], EffectDef::Sequence(&abilities::reveal_hand_and_exile_chosen_card(
            PlayerRefDef::Target(TargetIndex::PRIMARY),
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Color(ManaColor::Green),
                    ObjectPredicateDef::Color(ManaColor::White),
                ]),
            ]),
        ))),
    ]),
);

// M14 102 — Liliana of the Dark Realms (reprint)

// M14 103 — Liliana's Reaver
pub(in crate::card::sets) static LILIANAS_REAVER: CardRecord = CardRecord::new_with_legacy_id(
    1631,
    "Liliana's Reaver",
    CardArt::new("a734c33c-4fa0-4f7a-943c-14a8aecea1a6", "Karl Kopinski"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Zombie"], 4, 3).with_abilities(&[
        abilities::deathtouch(),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, that player discards a \
             card and you create a tapped 2/2 black Zombie creature token.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::Sequence(&[
                EffectDef::Discard {
                    recipient: EffectRecipientDef::EventPlayer,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
                TAPPED_ZOMBIE,
            ]),
        ),
    ]),
);

// M14 104 — Liturgy of Blood
pub(in crate::card::sets) static LITURGY_OF_BLOOD: CardRecord = CardRecord::new_with_legacy_id(
    1186,
    "Liturgy of Blood",
    CardArt::new("3532105d-c550-4c20-8465-a6a19169efbd", "Zack Stella"),
    CardSet::Magic2014,
    CardRules::new_sorcery(mana_cost!("{3}{B}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target creature. Add {B}{B}{B}.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black).with_amount(3)),
        ]),
    )),
);

// M14 105 — Mark of the Vampire (reprint)

// M14 106 — Mind Rot (reprint)

// M14 107 — Minotaur Abomination
pub(in crate::card::sets) static MINOTAUR_ABOMINATION: CardRecord = CardRecord::new_with_legacy_id(
    1187,
    "Minotaur Abomination",
    CardArt::new("9dca75a1-443d-4f8e-b12b-2aada3a8e3e4", "Karl Kopinski"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Zombie", "Minotaur"], 4, 6),
);

// M14 108 — Nightmare (reprint)

// M14 109 — Nightwing Shade
pub(in crate::card::sets) static NIGHTWING_SHADE: CardRecord = CardRecord::new_with_legacy_id(
    1188,
    "Nightwing Shade",
    CardArt::new("a3112a8a-dc80-4099-966c-8fa1807a189b", "Lucas Graciano"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Shade"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{1}{B}: This creature gets +1/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M14 110 — Quag Sickness
static QUAG_SICKNESS_PENALTY: ValueDef = ValueDef::Scaled(&ScaledValueDef::new(
    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
        ObjectPredicateDef::Subtype("Swamp"),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    )),
    -1,
));

pub(in crate::card::sets) static QUAG_SICKNESS: CardRecord = CardRecord::new_with_legacy_id(
    1189,
    "Quag Sickness",
    CardArt::new("a759dcd2-ca07-4428-a3ea-b2e829b1fcb4", "Martina Pilcerova"),
    CardSet::Magic2014,
    CardRules::new_enchantment(mana_cost!("{2}{B}"))
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
                "Enchanted creature gets -1/-1 for each Swamp you control.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        QUAG_SICKNESS_PENALTY,
                        QUAG_SICKNESS_PENALTY,
                    ),
                },
            ),
        ]),
);

// M14 111 — Rise of the Dark Realms
// Audit: metadata-only — MoveToZone cannot sweep matching cards from every graveyard into one player's control.
pub(in crate::card::sets) static RISE_OF_THE_DARK_REALMS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("073f81e8-8c0c-4430-bd3e-95ed3625340f"),
    "Rise of the Dark Realms",
    crate::card::CardArt::new("073f81e8-8c0c-4430-bd3e-95ed3625340f", "Michael Komarck"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 112 — Sanguine Bond
pub(in crate::card::sets) static SANGUINE_BOND: CardRecord = CardRecord::new_with_legacy_id(
    1190,
    "Sanguine Bond",
    CardArt::new("e50e807d-b2eb-4b62-8663-8ad17eed2a39", "Jaime Jones"),
    CardSet::Magic2014,
    CardRules::new_enchantment(mana_cost!("{3}{B}{B}")).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever you gain life, target opponent loses that much life.",
            TriggerEventDef::LifeGained(PlayerRelation::You),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::TriggerEventAmount,
            },
        ),
    ),
);

// M14 113 — Sengir Vampire (reprint)

// M14 114 — Shadowborn Apostle
// Audit: metadata-only — Deck construction has no any-number exception, and activated costs cannot choose and sacrifice six matching permanents as one payment.
pub(in crate::card::sets) static SHADOWBORN_APOSTLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("202c2323-6589-457a-af51-5528a98e7b30"),
    "Shadowborn Apostle",
    crate::card::CardArt::new("202c2323-6589-457a-af51-5528a98e7b30", "Lucas Graciano"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 115 — Shadowborn Demon
pub(in crate::card::sets) static SHADOWBORN_DEMON: CardRecord = CardRecord::new_with_legacy_id(
    211,
    "Shadowborn Demon",
    CardArt::new("3884c05b-c10e-4f1d-a8bd-8b5118657972", "Lucas Graciano"),
    CardSet::Magic2014,
    CardRules::new_creature(
        mana_cost!("{3}{B}{B}"),
        &["Demon"],
        5,
        6,
    )
    .with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets("When this creature enters, destroy target non-Demon creature.", &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Demon")),
            ]),
        )], EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            }),
        AbilityDef::triggered_if(
            "At the beginning of your upkeep, if there are fewer than six creature cards in your graveyard, sacrifice a creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            // Fewer than six is at most five. The count is of creature cards in your
            // own graveyard, which the Demon feeds on and which is why it stops eating
            // your board once the graveyard is full enough.
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::LessOrEqual,
                amount: 5,
            },
            EffectDef::SacrificeOfChoice {
                count: ValueDef::Constant(1),
                player: EffectRecipientDef::Controller,
                object: ObjectPredicateDef::HasType(CardType::Creature),
                then: None,
                amount: SacrificedAmountDef::Power,
                otherwise: None,
                optional: false,
            },
        ),
    ]),
);

// M14 116 — Shrivel
pub(in crate::card::sets) static SHRIVEL: CardRecord = CardRecord::new_with_legacy_id(
    1191,
    "Shrivel",
    CardArt::new("47b2ffdd-f8a4-49e4-aab1-a8096ba2b7cb", "Jung Park"),
    CardSet::Magic2014,
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_ability(AbilityDef::spell(
        "All creatures get -1/-1 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-1),
                ValueDef::Constant(-1),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M14 117 — Syphon Sliver
pub(in crate::card::sets) static SYPHON_SLIVER: CardRecord = CardRecord::new_with_legacy_id(
    1192,
    "Syphon Sliver",
    CardArt::new("85cb40e3-c3ed-4b3f-88ad-6f1305297c6f", "Tyler Jacobson"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Sliver"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "Sliver creatures you control have lifelink.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Sliver"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::lifelink()),
            },
        ),
    ),
);

// M14 118 — Tenacious Dead
// Audit: metadata-only — The effect vocabulary cannot make the returned source enter tapped after a death trigger.
pub(in crate::card::sets) static TENACIOUS_DEAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5b96fed2-0be9-4181-94ae-10f031e2aeb2"),
    "Tenacious Dead",
    crate::card::CardArt::new("5b96fed2-0be9-4181-94ae-10f031e2aeb2", "John Stanko"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 119 — Undead Minotaur
pub(in crate::card::sets) static UNDEAD_MINOTAUR: CardRecord = CardRecord::new_with_legacy_id(
    1193,
    "Undead Minotaur",
    CardArt::new("5e5ae910-ee1d-4958-92d9-0b06872913c6", "Karl Kopinski"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Zombie", "Minotaur"], 2, 3),
);

// M14 120 — Vampire Warlord
pub(in crate::card::sets) static VAMPIRE_WARLORD: CardRecord = CardRecord::new_with_legacy_id(
    1484,
    "Vampire Warlord",
    CardArt::new("7e07929b-450c-45b0-85e6-512ad280a122", "Wesley Burt"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Vampire", "Warrior"], 4, 2).with_ability(
        abilities::regenerate_self(
            "Sacrifice another creature: Regenerate this creature.",
            &[AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                controller: PlayerRelation::You,
            }],
        ),
    ),
);

// M14 121 — Vile Rebirth (reprint)

// M14 122 — Wring Flesh
pub(in crate::card::sets) static WRING_FLESH: CardRecord = CardRecord::new_with_legacy_id(
    1194,
    "Wring Flesh",
    CardArt::new("d6b77692-08aa-40b6-b21b-c29a2dc87709", "Izzy"),
    CardSet::Magic2014,
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets -3/-1 until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-3),
                ValueDef::Constant(-1),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M14 123 — Xathrid Necromancer
pub(in crate::card::sets) static XATHRID_NECROMANCER: CardRecord = CardRecord::new_with_legacy_id(
    1632,
    "Xathrid Necromancer",
    CardArt::new("26494f96-1d97-4435-a116-3ade1becaab4", "Maciej Kuciara"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Human", "Wizard"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever this creature or another Human creature you control dies, create a \
             tapped 2/2 black Zombie creature token.",
            TriggerEventDef::zone_changed(
                // "This creature or another Human creature you control" is every Human
                // creature its controller controls, since the Necromancer is one itself.
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Subtype("Human"),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            TAPPED_ZOMBIE,
        ),
    ),
);

// M14 124 — Academy Raider
// Audit: metadata-only — The optional discard cost needs a continuation that draws only when a card was actually discarded.
pub(in crate::card::sets) static ACADEMY_RAIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6652ed29-ee90-4abc-a6cf-6b18a6cbae86"),
    "Academy Raider",
    crate::card::CardArt::new("6652ed29-ee90-4abc-a6cf-6b18a6cbae86", "Karl Kopinski"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 125 — Act of Treason (reprint)

// M14 126 — Awaken the Ancient
pub(in crate::card::sets) static AWAKEN_THE_ANCIENT: CardRecord = CardRecord::new_with_legacy_id(
    1195,
    "Awaken the Ancient",
    CardArt::new("e4125304-fd68-4051-96d5-625ffa9b0d3c", "Jaime Jones"),
    CardSet::Magic2014,
    CardRules::new_enchantment(mana_cost!("{1}{R}{R}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant Mountain",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Mountain]),
                    ]),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted Mountain is a 7/7 red Giant creature with haste. It's still a land.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                        AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Giant"])),
                        AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Red])),
                        AppliedEffectDef::set_base_power_toughness(
                            ValueDef::Constant(7),
                            ValueDef::Constant(7),
                        ),
                        AppliedEffectDef::add_ability(&abilities::haste()),
                    ]),
                },
            ),
        ]),
);

// M14 127 — Barrage of Expendables
pub(in crate::card::sets) static BARRAGE_OF_EXPENDABLES: CardRecord =
    CardRecord::new_with_legacy_id(
        1196,
        "Barrage of Expendables",
        CardArt::new("b9e0912d-b4b9-497c-bce7-ed80b79bab32", "Trevor Claxton"),
        CardSet::Magic2014,
        CardRules::new_enchantment(mana_cost!("{R}")).with_ability(
            AbilityDef::activated_with_targets(
                "{R}, Sacrifice a creature: This enchantment deals 1 damage to any target.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{R}")),
                    AbilityCostDef::SacrificePermanent {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        controller: PlayerRelation::You,
                    },
                ],
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

// M14 128 — Battle Sliver
pub(in crate::card::sets) static BATTLE_SLIVER: CardRecord = CardRecord::new_with_legacy_id(
    1197,
    "Battle Sliver",
    CardArt::new("68490b8c-e9d1-4f5c-9001-750be0e0569f", "Slawomir Maniak"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Sliver"], 3, 3).with_ability(
        AbilityDef::static_ability(
            "Sliver creatures you control get +2/+0.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Sliver"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
            },
        ),
    ),
);

// M14 129 — Blur Sliver
pub(in crate::card::sets) static BLUR_SLIVER: CardRecord = CardRecord::new_with_legacy_id(
    1198,
    "Blur Sliver",
    CardArt::new("63227937-86cc-45e0-9e9e-8c7ab80cbaef", "Daarken"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Sliver"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "Sliver creatures you control have haste.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Sliver"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::haste()),
            },
        ),
    ),
);

// M14 130 — Burning Earth
pub(in crate::card::sets) static BURNING_EARTH: CardRecord = CardRecord::new_with_legacy_id(
    146,
    "Burning Earth",
    CardArt::new("1df3a7c9-5c8d-438c-a5ad-3c9754c6ea5d", "rk post"),
    CardSet::Magic2014,
    CardRules::new_enchantment(mana_cost!("{3}{R}")).with_ability(
        AbilityDef::triggered(
            "Whenever a player taps a nonbasic land for mana, this enchantment deals 1 damage to that player.",
            TriggerEventDef::tapped_for_mana(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Land),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(CardSupertype::Basic)),
            ])),
            EffectDef::DealDamage {
                // Whoever tapped it, which includes this enchantment's own
                // controller.
                recipient: EffectRecipientDef::ControllerOfTriggeringObject,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// M14 131 — Canyon Minotaur (reprint)

// M14 132 — Chandra, Pyromaster
// Audit: metadata-only — Planeswalker support lacks a turn-long cannot-block effect, top-card exile/play permission, and the ultimate's repeatable spell-copy procedure.
pub(in crate::card::sets) static CHANDRA_PYROMASTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("53ce8381-4cbb-4bf9-bdac-3b2375a46340"),
    "Chandra, Pyromaster",
    crate::card::CardArt::new("bcb4f983-a4b4-46df-830d-ab3d892c93bb", "Winona Nelson"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 133 — Chandra's Outrage
pub(in crate::card::sets) static CHANDRAS_OUTRAGE: CardRecord = CardRecord::new_with_legacy_id(
    1199,
    "Chandra's Outrage",
    CardArt::new(
        "65d1b479-f6f6-4fec-a5a6-1a74d426fb13",
        "Christopher Moeller",
    ),
    CardSet::Magic2014,
    CardRules::new_instant(mana_cost!("{2}{R}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Chandra's Outrage deals 4 damage to target creature and 2 damage to that creature's controller.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(4),
                },
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(2),
                },
            ]),
        ),
    ),
);

// M14 134 — Chandra's Phoenix
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CHANDRA_S_PHOENIX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c7c319cd-74ed-4ad8-ac0f-efb932bf5813"),
    "Chandra's Phoenix",
    crate::card::CardArt::new("ca08d3ce-a3a7-49ca-aa2f-4dcdacbf923d", "Aleksi Briclot"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 135 — Cyclops Tyrant
pub(in crate::card::sets) static CYCLOPS_TYRANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f0b8e733-22a7-4696-83b3-297cbe75dadc"),
    "Cyclops Tyrant",
    crate::card::CardArt::new("f0b8e733-22a7-4696-83b3-297cbe75dadc", "Zack Stella"),
    crate::card::CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{5}{R}"), &["Cyclops"], 3, 4).with_abilities(&[
        abilities::intimidate(),
        AbilityDef::static_ability(
            "This creature can't block creatures with power 2 or less.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::can_block_only(
                    ObjectPredicateDef::PowerAtLeast(3),
                )),
            },
        ),
    ]),
);

// M14 136 — Demolish (reprint)

// M14 137 — Dragon Egg
pub(in crate::card::sets) static DRAGON_EGG: CardRecord = CardRecord::new_with_legacy_id(
    1200,
    "Dragon Egg",
    CardArt::new("dc2048f7-0c68-4142-9aad-de9b91fe5958", "Jack Wang"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Dragon", "Egg"], 0, 2).with_abilities(&[
        abilities::defender(),
        abilities::dies_trigger("When this creature dies, create a 2/2 red Dragon creature token with flying and \"{R}: This token gets +1/+0 until end of turn.\"", EffectDef::create_creature_token(&["Dragon"], &[ManaColor::Red], 2, 2)
                .with_abilities(&[abilities::flying(), tokens::dragon_pump()])
                .with_art(CardArt::new(
                    "0efaa5b5-984d-4eff-81b6-9b4989f149eb",
                    "Jack Wang",
                ))),
    ]),
);

// M14 138 — Dragon Hatchling (reprint)

// M14 139 — Flames of the Firebrand (reprint)

// M14 140 — Fleshpulper Giant
pub(in crate::card::sets) static FLESHPULPER_GIANT: CardRecord = CardRecord::new_with_legacy_id(
    1201,
    "Fleshpulper Giant",
    CardArt::new(
        "f2726d3c-c182-4d8a-a723-0de2c5c4b152",
        "Alex Horley-Orlandelli",
    ),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{5}{R}{R}"), &["Giant"], 4, 4).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, you may destroy target creature with toughness 2 or less.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ToughnessLessThan(ValueDef::Constant(3)),
                ]),
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                    then: None,
                },
            },
        ),
    ),
);

// M14 141 — Goblin Diplomats
pub(in crate::card::sets) static GOBLIN_DIPLOMATS: CardRecord = CardRecord::new_with_legacy_id(
    1202,
    "Goblin Diplomats",
    CardArt::new("4620c581-fef7-45e8-ba20-d00903c2f4c5", "Izzy"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin"], 2, 1).with_ability(
        AbilityDef::activated(
            "{T}: Each creature attacks this turn if able.",
            &[AbilityCostDef::TapSource],
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::attacks_each_combat_if_able(
                    "This creature attacks this turn if able.",
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// M14 142 — Goblin Shortcutter
pub(in crate::card::sets) static GOBLIN_SHORTCUTTER: CardRecord = CardRecord::new_with_legacy_id(
    1747,
    "Goblin Shortcutter",
    CardArt::new("71bccbec-6e1e-43d5-b0dc-eddf942fa798", "Jesper Ejsing"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Scout"], 2, 1).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, target creature can't block this turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// M14 143 — Lava Axe
pub(in crate::card::sets) static LAVA_AXE: CardRecord = CardRecord::new_with_legacy_id(
    1203,
    "Lava Axe",
    CardArt::new("1c4f1041-8bbe-46fa-bbe4-40cd993f53a2", "Brian Snõddy"),
    CardSet::Magic2014,
    CardRules::new_sorcery(mana_cost!("{4}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Lava Axe deals 5 damage to target player or planeswalker.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(5),
        },
    )),
);

// M14 144 — Lightning Talons
pub(in crate::card::sets) static LIGHTNING_TALONS: CardRecord = CardRecord::new_with_legacy_id(
    1204,
    "Lightning Talons",
    CardArt::new("87186a8a-45da-4cde-a167-c16a6abc4d24", "Johann Bodin"),
    CardSet::Magic2014,
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
                "Enchanted creature gets +3/+0 and has first strike.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(3),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::add_ability(&abilities::first_strike()),
                    ]),
                },
            ),
        ]),
);

// M14 145 — Marauding Maulhorn
// Audit: metadata-only — Attack requirements cannot be conditional on controlling a permanent with a different exact name.
pub(in crate::card::sets) static MARAUDING_MAULHORN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b7d5e3dc-f307-4f91-a5ee-e7c5d03d8102"),
    "Marauding Maulhorn",
    crate::card::CardArt::new("b7d5e3dc-f307-4f91-a5ee-e7c5d03d8102", "Jesper Ejsing"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 146 — Mindsparker
pub(in crate::card::sets) static MINDSPARKER: CardRecord = CardRecord::new_with_legacy_id(
    1205,
    "Mindsparker",
    CardArt::new("a94295dc-d078-4f3f-9856-bd0a1899a9ca", "Wayne Reynolds"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Elemental"], 3, 2).with_abilities(&[
        abilities::first_strike(),
        AbilityDef::triggered(
            "Whenever an opponent casts a white or blue instant or sorcery spell, this creature deals 2 damage to that player.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Color(ManaColor::White),
                    ObjectPredicateDef::Color(ManaColor::Blue),
                ]),
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
            ])),
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::EventPlayer,
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// M14 147 — Molten Birth
// Audit: metadata-only — There is no coin-flip decision or result-conditioned self-return effect.
pub(in crate::card::sets) static MOLTEN_BIRTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0cd182be-1604-47e1-858f-3c304fd0ee63"),
    "Molten Birth",
    crate::card::CardArt::new("0cd182be-1604-47e1-858f-3c304fd0ee63", "Jaime Jones"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 148 — Ogre Battledriver
pub(in crate::card::sets) static OGRE_BATTLEDRIVER: CardRecord = CardRecord::new_with_legacy_id(
    1206,
    "Ogre Battledriver",
    CardArt::new("bff2d740-22cc-4719-ac58-28621951e68d", "Greg Staples"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Ogre", "Warrior"], 3, 3).with_ability(
        AbilityDef::triggered(
            "Whenever another creature you control enters, that creature gets +2/+0 and gains haste until end of turn.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]), None, Some(ZoneKind::Battlefield)),
            EffectDef::Apply {
                recipient: EffectRecipientDef::TriggeringObject,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(0)),
                    AppliedEffectDef::add_ability(&abilities::haste()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// M14 149 — Pitchburn Devils (reprint)

// M14 150 — Regathan Firecat
pub(in crate::card::sets) static REGATHAN_FIRECAT: CardRecord = CardRecord::new_with_legacy_id(
    1207,
    "Regathan Firecat",
    CardArt::new("4b4df1dd-886d-4fe7-b3f7-2dca044de41c", "Eric Velhagen"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Elemental", "Cat"], 4, 1),
);

// M14 151 — Scourge of Valkas
pub(in crate::card::sets) static SCOURGE_OF_VALKAS: CardRecord = CardRecord::new_with_legacy_id(
    1208,
    "Scourge of Valkas",
    CardArt::new("27ce2b55-45bf-4852-a74a-d0b17c6c9c3f", "Lucas Graciano"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{2}{R}{R}{R}"), &["Dragon"], 4, 4).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "Whenever this creature or another Dragon you control enters, it deals X damage to any target, where X is the number of Dragons you control.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Subtype("Dragon"),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]), None, Some(ZoneKind::Battlefield)),
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Dragon"),
                    ]), &[ZoneKind::Battlefield], PlayerRelation::You)),
            },
        ),
        AbilityDef::activated(
            "{R}: This creature gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(0)),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M14 152 — Seismic Stomp
pub(in crate::card::sets) static SEISMIC_STOMP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f55a02a3-8b65-44a7-82ef-2d3dc05d00ab"),
    "Seismic Stomp",
    crate::card::CardArt::new("f55a02a3-8b65-44a7-82ef-2d3dc05d00ab", "Chase Stone"),
    crate::card::CardSet::Magic2014,
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell(
        "Creatures without flying can't block this turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                        crate::card::KeywordAbility::Flying,
                    )),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M14 153 — Shiv's Embrace (reprint)

// M14 154 — Shivan Dragon (reprint)

// M14 155 — Shock
pub(in crate::card::sets) static SHOCK: CardRecord = CardRecord::new_with_legacy_id(
    1209,
    "Shock",
    CardArt::new("2fbec2ea-7b60-4c51-9782-52ccdd96c4b7", "Jon Foster"),
    CardSet::Magic2014,
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::spell_with_targets(
        "Shock deals 2 damage to any target.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(2),
        },
    )),
);

// M14 156 — Smelt (reprint)

// M14 157 — Striking Sliver
pub(in crate::card::sets) static STRIKING_SLIVER: CardRecord = CardRecord::new_with_legacy_id(
    1210,
    "Striking Sliver",
    CardArt::new("4ee9254b-3d98-4477-a82e-1450cf3ee96e", "Maciej Kuciara"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{R}"), &["Sliver"], 1, 1).with_ability(
        AbilityDef::static_ability(
            "Sliver creatures you control have first strike.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Sliver"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
            },
        ),
    ),
);

// M14 158 — Thorncaster Sliver
pub(in crate::card::sets) static THORNCASTER_SLIVER: CardRecord = CardRecord::new_with_legacy_id(
    1211,
    "Thorncaster Sliver",
    CardArt::new("3655d837-945f-4ff5-8952-cff5f7b2d18f", "Trevor Claxton"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Sliver"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "Sliver creatures you control have \"Whenever this creature attacks, it deals 1 damage to any target.\"",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Sliver"),
                    ]), &[ZoneKind::Battlefield], PlayerRelation::You),
                effect: AppliedEffectDef::add_ability(&AbilityDef::triggered_with_targets(
                    "Whenever this creature attacks, it deals 1 damage to any target.",
                    TriggerEventDef::attacks(ObjectPredicateDef::Source),
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::AnyTarget,
                    )],
                    EffectDef::DealDamage {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(1),
                    },
                )),
            },
        ),
    ),
);

// M14 159 — Thunder Strike
pub(in crate::card::sets) static THUNDER_STRIKE: CardRecord = CardRecord::new_with_legacy_id(
    1212,
    "Thunder Strike",
    CardArt::new("61aa445d-d734-4e4f-800d-fe7bea86eb70", "Wayne Reynolds"),
    CardSet::Magic2014,
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +2/+0 and gains first strike until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                AppliedEffectDef::add_ability(&abilities::first_strike()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M14 160 — Volcanic Geyser (reprint)

// M14 161 — Wild Guess (reprint)

// M14 162 — Wild Ricochet (reprint)

// M14 163 — Young Pyromancer
pub(in crate::card::sets) static YOUNG_PYROMANCER: CardRecord = CardRecord::new_with_legacy_id(
    1213,
    "Young Pyromancer",
    CardArt::new("e349c204-3a93-4bf7-b79a-5f5f261ea2d3", "Cynthia Sheppard"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Shaman"], 2, 1).with_ability(
        AbilityDef::triggered(
            "Whenever you cast an instant or sorcery spell, create a 1/1 red Elemental creature token.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
            ])),
            EffectDef::create_creature_token(&["Elemental"], &[ManaColor::Red], 1, 1).with_art(CardArt::new("fc7315d5-26d9-4ecc-bca2-b75c6fb12597", "Winona Nelson")),
        ),
    ),
);

// M14 164 — Advocate of the Beast
pub(in crate::card::sets) static ADVOCATE_OF_THE_BEAST: CardRecord = CardRecord::new_with_legacy_id(
    1214,
    "Advocate of the Beast",
    CardArt::new("b1320400-5aa8-48d6-be84-197b4559456f", "Jesper Ejsing"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf", "Shaman"], 2, 3).with_ability(
        AbilityDef::triggered_with_targets(
            "At the beginning of your end step, put a +1/+1 counter on target Beast creature you control.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Subtype("Beast"),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ]))],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// M14 165 — Bramblecrush (reprint)

// M14 166 — Briarpack Alpha (reprint)

// M14 167 — Brindle Boar
pub(in crate::card::sets) static BRINDLE_BOAR: CardRecord = CardRecord::new_with_legacy_id(
    1215,
    "Brindle Boar",
    CardArt::new("a30b4a78-afdd-4067-810e-1fa0ddf8fb0e", "Dave Allsop"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Boar"], 2, 2).with_ability(
        AbilityDef::activated(
            "Sacrifice this creature: You gain 4 life.",
            &[AbilityCostDef::SacrificeSource],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(4),
            },
        ),
    ),
);

// M14 168 — Deadly Recluse (reprint)

// M14 169 — Elvish Mystic
pub(in crate::card::sets) static ELVISH_MYSTIC: CardRecord = CardRecord::new_with_legacy_id(
    160,
    "Elvish Mystic",
    CardArt::new("60d0e6a6-629a-45a7-bfcb-25ba7156788b", "Wesley Burt"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Druid"], 1, 1)
        .with_abilities(&[abilities::tap_for(ManaColor::Green)]),
);

// M14 170 — Enlarge
pub(in crate::card::sets) static ENLARGE: CardRecord = CardRecord::new_with_legacy_id(
    1740,
    "Enlarge",
    CardArt::new("ec40df78-b1ca-4300-8a47-4d5b0ae3499e", "Michael Komarck"),
    CardSet::Magic2014,
    CardRules::new_sorcery(mana_cost!("{3}{G}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +7/+7 and gains trample until end of turn. It must be blocked this \
         turn if able.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            // Trample and the requirement work together: something has to block it, and
            // blocking barely slows seven extra power down.
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(7),
                    ValueDef::Constant(7),
                ),
                AppliedEffectDef::add_ability(&abilities::trample()),
                AppliedEffectDef::Rule(AppliedRuleDef::MustBeBlockedBy(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M14 171 — Fog (reprint)

// M14 172 — Garruk, Caller of Beasts
// Audit: metadata-only — The planeswalker modes need filtered multi-card top selection, a hand-to-battlefield choice, and an emblem carrying an optional creature tutor trigger.
pub(in crate::card::sets) static GARRUK_CALLER_OF_BEASTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d61f670e-8e4c-493f-bef0-c4c7b0bcd288"),
    "Garruk, Caller of Beasts",
    crate::card::CardArt::new("a96d0c67-e9f4-46d9-bd74-13a8606fdfe3", "Karl Kopinski"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 173 — Garruk's Horde
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GARRUK_S_HORDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3313f4ea-1275-4835-b4ff-73d3601c04e1"),
    "Garruk's Horde",
    crate::card::CardArt::new("88b24651-1814-440e-a415-a96c03e51544", "Steve Prescott"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 174 — Giant Growth (reprint)

// M14 175 — Giant Spider (reprint)

// M14 176 — Gladecover Scout
pub(in crate::card::sets) static GLADECOVER_SCOUT: CardRecord = CardRecord::new_with_legacy_id(
    1216,
    "Gladecover Scout",
    CardArt::new("e112d77d-f019-4709-b31a-b02952df0e35", "Allen Williams"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Scout"], 1, 1)
        .with_abilities(&[abilities::hexproof()]),
);

// M14 177 — Groundshaker Sliver
pub(in crate::card::sets) static GROUNDSHAKER_SLIVER: CardRecord = CardRecord::new_with_legacy_id(
    1217,
    "Groundshaker Sliver",
    CardArt::new("712f0ce4-9189-4c75-9c2b-d370bce89052", "Chase Stone"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{6}{G}"), &["Sliver"], 5, 5).with_ability(
        AbilityDef::static_ability(
            "Sliver creatures you control have trample.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Sliver"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::trample()),
            },
        ),
    ),
);

// M14 178 — Howl of the Night Pack
pub(in crate::card::sets) static HOWL_OF_THE_NIGHT_PACK: CardRecord =
    CardRecord::new_with_legacy_id(
        1218,
        "Howl of the Night Pack",
        CardArt::new("20fc5ff1-b8bd-44d5-a659-17eeae06736a", "Lars Grant-West"),
        CardSet::Magic2014,
        CardRules::new_sorcery(mana_cost!("{6}{G}")).with_ability(AbilityDef::spell(
            "Create a 2/2 green Wolf creature token for each Forest you control.",
            EffectDef::create_creature_token(&["Wolf"], &[ManaColor::Green], 2, 2)
                .with_art(CardArt::new(
                    "309f1bd4-78af-4722-9d45-b5f40b001570",
                    "Lars Grant-West",
                ))
                .with_count(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ))),
        )),
    );

// M14 179 — Hunt the Weak
// Audit: metadata-only — DealDamage can only attribute damage to the resolving spell, not to each fighting creature.
pub(in crate::card::sets) static HUNT_THE_WEAK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8f7a6df7-acfc-4047-b119-505f4277225c"),
    "Hunt the Weak",
    crate::card::CardArt::new("8f7a6df7-acfc-4047-b119-505f4277225c", "Raoul Vitale"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 180 — Into the Wilds
/// Only a land may be taken, and taking it is optional. Whatever is not
/// taken remains on top, so a nonland card is still the next draw.
const WILDS_LAND: Binding = Binding!("wilds_land");
pub(in crate::card::sets) static INTO_THE_WILDS: CardRecord = CardRecord::new_with_legacy_id(
    2007,
    "Into the Wilds",
    CardArt::new("ecfa6c8d-b5b5-4b68-9ad4-c9d8169659d6", "Véronique Meignaud"),
    CardSet::Magic2014,
    // A free land every upkeep the top card cooperates, and the land it puts
    // down does not spend the turn's land drop.
    CardRules::new_enchantment(mana_cost!("{3}{G}")).with_ability(AbilityDef::triggered(
        "At the beginning of your upkeep, look at the top card of your library. If it's a land card, you may put it onto the battlefield.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::You,
        },
        abilities::bind_top_cards_then(
            PlayerRefDef::EffectController,
            ValueDef::Constant(1),
            &const { EffectDef::ClassifyObjects(ClassifyObjectsDef {
                input: ObjectSetDef::Binding(ParentBinding),
                object: ObjectPredicateDef::HasType(CardType::Land),
                matching: WILDS_LAND,
                remainder: Binding!("wilderness_remainder"),
                then: &EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Binding(WILDS_LAND),
                    exclude: None,
                    minimum: 0,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Private,
                    then: &EffectDef::MoveObjects(MoveObjectsDef {
                        input: ObjectSetDef::Binding(ParentBinding),
                        from: Some(ZoneKind::Library),
                        zone: ZoneKind::Battlefield,
                        placement: ZonePlacement::Top,
                        moved: None,
                        then: &EffectDef::None,
                    }),
                }),
            }) },
        ),
    )),
);

// M14 181 — Kalonian Hydra
// Audit: metadata-only — Counter effects cannot double each creature's existing +1/+1 counter count.
pub(in crate::card::sets) static KALONIAN_HYDRA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("438bd3c1-98f2-4fcc-8521-995c6c5c1a79"),
    "Kalonian Hydra",
    crate::card::CardArt::new("438bd3c1-98f2-4fcc-8521-995c6c5c1a79", "Chris Rahn"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 182 — Kalonian Tusker
pub(in crate::card::sets) static KALONIAN_TUSKER: CardRecord = CardRecord::new_with_legacy_id(
    1219,
    "Kalonian Tusker",
    CardArt::new("135946fc-fe67-401f-821d-d7145c63f030", "Svetlin Velinov"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{G}{G}"), &["Beast"], 3, 3),
);

// M14 183 — Lay of the Land
pub(in crate::card::sets) static LAY_OF_THE_LAND: CardRecord = CardRecord::new_with_legacy_id(
    1220,
    "Lay of the Land",
    CardArt::new("3bb3410b-d6c3-4e42-b3c9-fb557f9a16f0", "Chuck Lukacs"),
    CardSet::Magic2014,
    CardRules::new_sorcery(mana_cost!("{G}")).with_ability(AbilityDef::spell(
        "Search your library for a basic land card, reveal it, put it into your hand, then shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Land),
                ObjectPredicateDef::Supertype(CardSupertype::Basic),
            ]),
            minimum: 0,
            maximum: ValueDef::Constant(1),
            reveal: true,
            destination: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            attachment: None,
            binding: None,
            then: None,
        },
    )),
);

// M14 184 — Manaweft Sliver
pub(in crate::card::sets) static MANAWEFT_SLIVER: CardRecord = CardRecord::new_with_legacy_id(
    1221,
    "Manaweft Sliver",
    CardArt::new("fe45433b-e124-44d7-9463-dada39310148", "Trevor Claxton"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Sliver"], 1, 1).with_ability(
        AbilityDef::static_ability(
            "Sliver creatures you control have \"{T}: Add one mana of any color.\"",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Sliver"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&AbilityDef::activated_mana(
                    "{T}: Add one mana of any color.",
                    &[AbilityCostDef::TapSource],
                    EffectDef::AddMana(AddManaEffectDef::any_color()),
                )),
            },
        ),
    ),
);

// M14 185 — Megantic Sliver
pub(in crate::card::sets) static MEGANTIC_SLIVER: CardRecord = CardRecord::new_with_legacy_id(
    1222,
    "Megantic Sliver",
    CardArt::new("7745f6a9-400c-4200-9732-86c54247de46", "Ryan Barger"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{5}{G}"), &["Sliver"], 3, 3).with_ability(
        AbilityDef::static_ability(
            "Sliver creatures you control get +3/+3.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Sliver"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(3),
                ),
            },
        ),
    ),
);

// M14 186 — Naturalize (reprint)

// M14 187 — Oath of the Ancient Wood
pub(in crate::card::sets) static OATH_OF_THE_ANCIENT_WOOD: CardRecord = CardRecord::new_with_legacy_id(
    1223,
    "Oath of the Ancient Wood",
    CardArt::new("9bc42032-8727-4f78-b369-ba103d965b73", "Dan Murayama Scott"),
    CardSet::Magic2014,
    CardRules::new_enchantment(mana_cost!("{2}{G}")).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever this enchantment or another enchantment you control enters, you may put a +1/+1 counter on target creature.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]), None, Some(ZoneKind::Battlefield)),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            },
        ),
    ),
);

// M14 188 — Plummet (reprint)

// M14 189 — Predatory Sliver
pub(in crate::card::sets) static PREDATORY_SLIVER: CardRecord = CardRecord::new_with_legacy_id(
    1224,
    "Predatory Sliver",
    CardArt::new("a2e37de8-66a1-4afa-aa6f-1151f849dfa8", "Mathias Kollros"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Sliver"], 1, 1).with_ability(
        AbilityDef::static_ability(
            "Sliver creatures you control get +1/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Sliver"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
            },
        ),
    ),
);

// M14 190 — Primeval Bounty
pub(in crate::card::sets) static PRIMEVAL_BOUNTY: CardRecord = CardRecord::new_with_legacy_id(
    197,
    "Primeval Bounty",
    CardArt::new("e750d55d-d5e8-4abe-99cf-f6b8ba86cf16", "Christine Choi"),
    CardSet::Magic2014,
    CardRules::new_enchantment(mana_cost!("{5}{G}")).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you cast a creature spell, create a 3/3 green Beast creature token.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::create_creature_token(&["Beast"], &[ManaColor::Green], 3, 3).with_art(CardArt::new("a8fc2dc9-40df-46d8-98c0-ca4919bd5524", "John Donahue")),
        ),
        AbilityDef::triggered_with_targets("Whenever you cast a noncreature spell, put three +1/+1 counters on target creature you control.", TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::NoncreatureSpell,
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])), &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ]),
        )], EffectDef::AddCounters {
                kind: CounterKind::PlusOnePlusOne,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            }),
        AbilityDef::triggered(
            "Landfall — Whenever a land you control enters, you gain 3 life.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]), None, Some(ZoneKind::Battlefield)),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ),
    ]),
);

// M14 191 — Ranger's Guile (reprint)

// M14 192 — Rootwalla
pub(in crate::card::sets) static ROOTWALLA: CardRecord = CardRecord::new_with_legacy_id(
    1902,
    "Rootwalla",
    CardArt::new("2b84b6dc-d78d-4d6a-9e9a-2b40854a102b", "Roger Raupp"),
    CardSet::Magic2014,
    // The quota is per turn and per permanent, so a second Rootwalla still
    // has its own.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Lizard"], 2, 2).with_ability(
        AbilityDef::activated(
            "{1}{G}: This creature gets +2/+2 until end of turn. Activate only once each turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .once_each_turn(),
    ),
);

// M14 193 — Rumbling Baloth
pub(in crate::card::sets) static RUMBLING_BALOTH: CardRecord = CardRecord::new_with_legacy_id(
    1225,
    "Rumbling Baloth",
    CardArt::new("d8610ff1-064b-4c75-a8df-d3b076370d1e", "Jesper Ejsing"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Beast"], 4, 4),
);

// M14 194 — Savage Summoning
// Audit: metadata-only — No continuation can tag the next creature spell with flash, uncounterability, and an entry counter while also making this spell uncounterable.
pub(in crate::card::sets) static SAVAGE_SUMMONING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b5346ed7-2e17-4d8c-9c4b-b5efdd26380d"),
    "Savage Summoning",
    crate::card::CardArt::new("b5346ed7-2e17-4d8c-9c4b-b5efdd26380d", "Johann Bodin"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 195 — Scavenging Ooze
/// One when the exiled card was a creature, nothing otherwise.
static EXILED_A_CREATURE: TargetConditionDef = TargetConditionDef {
    slot: TargetIndex::PRIMARY,
    object: ObjectPredicateDef::HasType(CardType::Creature),
    then: ValueDef::Constant(1),
    otherwise: ValueDef::Constant(0),
};

pub(in crate::card::sets) static SCAVENGING_OOZE: CardRecord = CardRecord::new_with_legacy_id(
    208,
    "Scavenging Ooze",
    CardArt::new("ec30153a-36b5-42f8-beed-9efab09f1051", "Austin Hsu"),
    CardSet::Magic2014,
    CardRules::new_creature(
        mana_cost!("{1}{G}"),
        &["Ooze"],
        2,
        2,
    )
    .with_ability(
        AbilityDef::activated_with_targets("{G}: Exile target card from a graveyard. If it was a creature card, put a +1/+1 counter on this creature and you gain 1 life.", &[AbilityCostDef::Mana(mana_cost!("{G}"))], &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Any,
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: None,
            },
        )], // The counter and the life come first so the card is still in the
            // graveyard to be asked what it was. Exiling it first would leave
            // nothing to look at, and nothing here can observe the order.
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::IfTargetMatches(&EXILED_A_CREATURE),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::IfTargetMatches(&EXILED_A_CREATURE),
                },
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Exile,
                    placement: ZonePlacement::Top,
},
            ])),
    ),
);

// M14 196 — Sporemound
pub(in crate::card::sets) static SPOREMOUND: CardRecord = CardRecord::new_with_legacy_id(
    1226,
    "Sporemound",
    CardArt::new("2d256cd0-6fe9-4905-9886-fb1457292db5", "Svetlin Velinov"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Fungus"], 3, 3).with_ability(
        AbilityDef::triggered(
            "Landfall — Whenever a land you control enters, create a 1/1 green Saproling creature token.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]), None, Some(ZoneKind::Battlefield)),
            EffectDef::create_creature_token(&["Saproling"], &[ManaColor::Green], 1, 1).with_art(CardArt::new("afd66b96-eccb-44ce-9125-063d34af2ff8", "Brad Rigney")),
        ),
    ),
);

// M14 197 — Trollhide
pub(in crate::card::sets) static TROLLHIDE: CardRecord = CardRecord::new_with_legacy_id(
    1485,
    "Trollhide",
    CardArt::new("08b9c400-dc8f-4fe6-a868-fdf0d247086a", "Steven Belledin"),
    CardSet::Magic2014,
    CardRules::new_enchantment(mana_cost!("{2}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+2 and has \"{1}{G}: Regenerate this creature.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&abilities::regenerate_self(
                            "{1}{G}: Regenerate this creature.",
                            &[AbilityCostDef::Mana(mana_cost!("{1}{G}"))],
                        )),
                    ]),
                },
            ),
        ]),
);

// M14 198 — Vastwood Hydra
// Audit: metadata-only — Entry replacements cannot add chosen-X counters, and counter distribution cannot read the dead source's last-known counter count.
pub(in crate::card::sets) static VASTWOOD_HYDRA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9e635174-7f7d-4c04-a6aa-8674da6863ff"),
    "Vastwood Hydra",
    crate::card::CardArt::new("9e635174-7f7d-4c04-a6aa-8674da6863ff", "Slawomir Maniak"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 199 — Verdant Haven (reprint)

// M14 200 — Voracious Wurm
// Audit: metadata-only — Entry replacements cannot derive a counter amount from life gained during the turn.
pub(in crate::card::sets) static VORACIOUS_WURM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("da15100b-2934-438c-9917-84ad8bdc4181"),
    "Voracious Wurm",
    crate::card::CardArt::new("da15100b-2934-438c-9917-84ad8bdc4181", "Igor Kieryluk"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 201 — Windstorm
pub(in crate::card::sets) static WINDSTORM: CardRecord = CardRecord::new_with_legacy_id(
    1227,
    "Windstorm",
    CardArt::new("3cb7d122-34e8-48e1-a978-831c78a37d0c", "Rob Alexander"),
    CardSet::Magic2014,
    CardRules::new_instant(mana_cost!("{X}{G}")).with_ability(AbilityDef::spell(
        "Windstorm deals X damage to each creature with flying.",
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Flying),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            amount: ValueDef::ChosenX,
        },
    )),
);

// M14 202 — Witchstalker
// Audit: metadata-only — A non-intervening spell-cast trigger cannot capture “during your turn” without incorrectly rechecking that restriction on resolution.
pub(in crate::card::sets) static WITCHSTALKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5a5ce47d-ea4f-4e15-adb6-5bb66981ed24"),
    "Witchstalker",
    crate::card::CardArt::new(
        "5a5ce47d-ea4f-4e15-adb6-5bb66981ed24",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 203 — Woodborn Behemoth
pub(in crate::card::sets) static WOODBORN_BEHEMOTH: CardRecord = CardRecord::new_with_legacy_id(
    1919,
    "Woodborn Behemoth",
    CardArt::new("8c73dbf3-e68e-4f21-b6ca-94302bf5574c", "Matt Stewart"),
    CardSet::Magic2014,
    // Both halves are behind the same threshold, so losing the eighth land
    // takes the trample away with the size.
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Elemental"], 4, 4).with_ability(
        AbilityDef::static_ability(
            "As long as you control eight or more lands, this creature gets +4/+4 and has \
             trample.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 8,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(4),
                            ValueDef::Constant(4),
                        ),
                        AppliedEffectDef::add_ability(&abilities::trample()),
                    ]),
                },
            },
        ),
    ),
);

// M14 204 — Accorder's Shield
pub(in crate::card::sets) static ACCORDERS_SHIELD: CardRecord = CardRecord::new_with_legacy_id(
    1624,
    "Accorder's Shield",
    CardArt::new("c5a4c2ab-c5bc-4e07-8671-a688ebd5471c", "Alan Pollack"),
    CardSet::Magic2014,
    CardRules::new_artifact(mana_cost!("{0}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +0/+3 and has vigilance.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(0),
                            ValueDef::Constant(3),
                        ),
                        AppliedEffectDef::add_ability(&abilities::vigilance()),
                    ]),
                },
            ),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{3}"))],
                "Equip {3} ({3}: Attach to target creature you control. Equip only as a \
                 sorcery.)",
            ),
        ]),
);

// M14 205 — Bubbling Cauldron
// Audit: metadata-only — Costs cannot select an exact named sacrifice, and effects cannot total life actually lost by all opponents for the linked gain.
pub(in crate::card::sets) static BUBBLING_CAULDRON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9af87c24-a534-462b-968b-dccf6ac63299"),
    "Bubbling Cauldron",
    crate::card::CardArt::new("9af87c24-a534-462b-968b-dccf6ac63299", "Eric Deschamps"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 206 — Darksteel Forge
pub(in crate::card::sets) static DARKSTEEL_FORGE: CardRecord = CardRecord::new_with_legacy_id(
    1228,
    "Darksteel Forge",
    CardArt::new("2c95a0a1-9c2c-44df-b0fe-c22efb6d87ee", "Martina Pilcerova"),
    CardSet::Magic2014,
    CardRules::new_artifact(mana_cost!("{9}")).with_ability(AbilityDef::static_ability(
        "Artifacts you control have indestructible.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Artifact),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
        },
    )),
);

// M14 207 — Darksteel Ingot (reprint)

// M14 208 — Door of Destinies
// Audit: metadata-only — Predicates cannot consume a stored creature-type choice for both spell triggers and a counter-scaled continuous bonus.
pub(in crate::card::sets) static DOOR_OF_DESTINIES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d4ab817e-11d4-4444-b9e1-322624501619"),
    "Door of Destinies",
    crate::card::CardArt::new("68a6bf1a-7152-496f-a4c7-e720ef4294d8", "Larry MacDougall"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 209 — Elixir of Immortality (reprint)

// M14 210 — Fireshrieker
pub(in crate::card::sets) static FIRESHRIEKER: CardRecord = CardRecord::new_with_legacy_id(
    1625,
    "Fireshrieker",
    CardArt::new(
        "9f653742-b92a-4cfa-b3b5-8d20aabdb5dd",
        "Christopher Moeller",
    ),
    CardSet::Magic2014,
    CardRules::new_artifact(mana_cost!("{3}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature has double strike.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::double_strike()),
                },
            ),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{2}"))],
                "Equip {2} ({2}: Attach to target creature you control. Equip only as a \
                 sorcery.)",
            ),
        ]),
);

// M14 211 — Guardian of the Ages
// Audit: metadata-only — Attack events cannot match attacks at you or your planeswalker, and abilities cannot permanently remove defender from the source after that trigger.
pub(in crate::card::sets) static GUARDIAN_OF_THE_AGES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c825c138-97de-44b9-8aec-70608ae035b6"),
    "Guardian of the Ages",
    crate::card::CardArt::new("c825c138-97de-44b9-8aec-70608ae035b6", "Ryan Pancoast"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 212 — Haunted Plate Mail
pub(in crate::card::sets) static HAUNTED_PLATE_MAIL: CardRecord = CardRecord::new_with_legacy_id(
    2312,
    "Haunted Plate Mail",
    CardArt::new("e2dc1e07-7894-4f22-936d-bf5df3f8d5a5", "Izzy"),
    CardSet::Magic2014,
    CardRules::new_artifact(mana_cost!("{4}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +4/+4.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(4),
                        ValueDef::Constant(4),
                    ),
                },
            ),
            AbilityDef::activated(
                "{0}: Until end of turn, this permanent becomes a 4/4 Spirit artifact creature that's no longer an Equipment. Activate only if you control no creatures.",
                &[],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::set_card_types(
                            CardTypeSet::single(CardType::Artifact).with(CardType::Creature),
                        ),
                        AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Spirit"])),
                        AppliedEffectDef::remove_subtypes(&["Equipment"]),
                        AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(4), ValueDef::Constant(4)),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            )
            .with_activation_condition(&TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::Equal,
                amount: 0,
            }),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{4}"))],
                "Equip {4} ({4}: Attach to target creature you control. Equip only as a sorcery.)",
            ),
        ]),
);

// M14 213 — Millstone (reprint)

// M14 214 — Pyromancer's Gauntlet
// Audit: metadata-only — Damage replacement cannot filter red instant, sorcery, or planeswalker sources and add a fixed amount to the event.
pub(in crate::card::sets) static PYROMANCER_S_GAUNTLET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9bde6763-2102-4adb-8048-fc9fe921205b"),
    "Pyromancer's Gauntlet",
    crate::card::CardArt::new("9bde6763-2102-4adb-8048-fc9fe921205b", "Christine Choi"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 215 — Ratchet Bomb
pub(in crate::card::sets) static RATCHET_BOMB: CardRecord = CardRecord::new_with_legacy_id(
    200,
    "Ratchet Bomb",
    CardArt::new("3e9045df-3eff-4236-9bbb-77537b302e27", "Austin Hsu"),
    CardSet::Magic2014,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::activated(
            "{T}: Put a charge counter on this artifact.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("charge"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "{T}, Sacrifice this artifact: Destroy each nonland permanent with mana value equal to the number of charge counters on this artifact.",
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
            EffectDef::Destroy {
                object: EffectRecipientDef::matching_objects(ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        // The Bomb is already gone by the time this resolves,
                        // so the count comes from last-known information.
                        ObjectPredicateDef::ManaValueEqualTo(ValueDef::CountersOnSource(
                            CounterKind::named("charge"),
                        )),
                    ]), &[ZoneKind::Battlefield], PlayerRelation::Any),
                can_regenerate: true,
                then: None,
            },
        ),
    ]),
);

// M14 216 — Ring of Three Wishes
// Audit: metadata-only — CounterKind has no wish counter, so the entry counters and removal cost cannot share the printed counter identity.
pub(in crate::card::sets) static RING_OF_THREE_WISHES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("219ab03a-2b3b-4eef-8a42-2cbe793d2f33"),
    "Ring of Three Wishes",
    crate::card::CardArt::new("219ab03a-2b3b-4eef-8a42-2cbe793d2f33", "Mark Winters"),
    crate::card::CardSet::Magic2014,
    crate::card::CardRules::unsupported(),
);

// M14 217 — Rod of Ruin (reprint)

// M14 218 — Sliver Construct
pub(in crate::card::sets) static SLIVER_CONSTRUCT: CardRecord = CardRecord::new_with_legacy_id(
    1229,
    "Sliver Construct",
    CardArt::new("3129645a-221c-4eb5-88fd-12cc742a1dfe", "Mathias Kollros"),
    CardSet::Magic2014,
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Sliver", "Construct"], 2, 2),
);

// M14 219 — Staff of the Death Magus
pub(in crate::card::sets) static STAFF_OF_THE_DEATH_MAGUS: CardRecord =
    CardRecord::new_with_legacy_id(
        1230,
        "Staff of the Death Magus",
        CardArt::new("624fe171-8bd8-4156-b40e-74e2a847d380", "Daniel Ljunggren"),
        CardSet::Magic2014,
        CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
            AbilityDef::triggered(
                "Whenever you cast a black spell, you gain 1 life.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Color(ManaColor::Black),
                ])),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::triggered(
                "Whenever a Swamp you control enters, you gain 1 life.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Swamp]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
    );

// M14 220 — Staff of the Flame Magus
pub(in crate::card::sets) static STAFF_OF_THE_FLAME_MAGUS: CardRecord =
    CardRecord::new_with_legacy_id(
        1231,
        "Staff of the Flame Magus",
        CardArt::new("6d6befbd-4fe3-4338-b8ea-13b8b70a7664", "Daniel Ljunggren"),
        CardSet::Magic2014,
        CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
            AbilityDef::triggered(
                "Whenever you cast a red spell, you gain 1 life.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Color(ManaColor::Red),
                ])),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::triggered(
                "Whenever a Mountain you control enters, you gain 1 life.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Mountain]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
    );

// M14 221 — Staff of the Mind Magus
pub(in crate::card::sets) static STAFF_OF_THE_MIND_MAGUS: CardRecord =
    CardRecord::new_with_legacy_id(
        1232,
        "Staff of the Mind Magus",
        CardArt::new("f86bf36b-b83f-4451-8cdc-2a4ccffb93c7", "Daniel Ljunggren"),
        CardSet::Magic2014,
        CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
            AbilityDef::triggered(
                "Whenever you cast a blue spell, you gain 1 life.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Color(ManaColor::Blue),
                ])),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::triggered(
                "Whenever an Island you control enters, you gain 1 life.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
    );

// M14 222 — Staff of the Sun Magus
pub(in crate::card::sets) static STAFF_OF_THE_SUN_MAGUS: CardRecord =
    CardRecord::new_with_legacy_id(
        1233,
        "Staff of the Sun Magus",
        CardArt::new("90a1f830-d19a-4ebf-9573-09b677693dd6", "Daniel Ljunggren"),
        CardSet::Magic2014,
        CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
            AbilityDef::triggered(
                "Whenever you cast a white spell, you gain 1 life.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Color(ManaColor::White),
                ])),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::triggered(
                "Whenever a Plains you control enters, you gain 1 life.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Plains]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
    );

// M14 223 — Staff of the Wild Magus
pub(in crate::card::sets) static STAFF_OF_THE_WILD_MAGUS: CardRecord =
    CardRecord::new_with_legacy_id(
        1234,
        "Staff of the Wild Magus",
        CardArt::new("d207f03d-4c7b-444f-bf95-e63f7004d525", "Daniel Ljunggren"),
        CardSet::Magic2014,
        CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
            AbilityDef::triggered(
                "Whenever you cast a green spell, you gain 1 life.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Color(ManaColor::Green),
                ])),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::triggered(
                "Whenever a Forest you control enters, you gain 1 life.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
    );

// M14 224 — Strionic Resonator
pub(in crate::card::sets) static STRIONIC_RESONATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("94d1fc0f-5c8b-4e47-aaf8-8888c025f70f"),
    "Strionic Resonator",
    crate::card::CardArt::new("94d1fc0f-5c8b-4e47-aaf8-8888c025f70f", "Noah Bradley"),
    crate::card::CardSet::Magic2014,
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(
        AbilityDef::activated_with_targets(
            "{2}, {T}: Copy target triggered ability you control. You may choose new targets for the copy. (A triggered ability uses the words \"when,\" \"whenever,\" or \"at.\")",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::TriggeredAbility,
                    zones: &[ZoneKind::Stack],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::CopyStackObject(&crate::card::CopyStackObjectDef {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                controller: PlayerRefDef::EffectController,
                count: ValueDef::Constant(1),
                retarget: true,
                colors: None,
            }),
        ),
    ),
);

// M14 225 — Trading Post (reprint)

// M14 226 — Vial of Poison
pub(in crate::card::sets) static VIAL_OF_POISON: CardRecord = CardRecord::new_with_legacy_id(
    1235,
    "Vial of Poison",
    CardArt::new("7769159b-5a6a-45e5-b69b-8db2a6ef5418", "Franz Vohwinkel"),
    CardSet::Magic2014,
    CardRules::new_artifact(mana_cost!("{1}")).with_ability(AbilityDef::activated_with_targets(
        "{1}, Sacrifice this artifact: Target creature gains deathtouch until end of turn.",
        &[
            AbilityCostDef::Mana(mana_cost!("{1}")),
            AbilityCostDef::SacrificeSource,
        ],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::add_ability(&abilities::deathtouch()),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M14 227 — Encroaching Wastes
pub(in crate::card::sets) static ENCROACHING_WASTES: CardRecord = CardRecord::new_with_legacy_id(
    161,
    "Encroaching Wastes",
    CardArt::new("1ad5a84b-ae9b-4ed1-a4de-b91bbf8ed0a5", "Noah Bradley"),
    CardSet::Magic2014,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{4}, {T}, Sacrifice this land: Destroy target nonbasic land.",
            &[
                AbilityCostDef::Mana(mana_cost!("{4}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(CardSupertype::Basic)),
                ]),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
        ),
    ]),
);

// M14 228 — Mutavault
pub(in crate::card::sets) static MUTAVAULT: CardRecord = CardRecord::new_with_legacy_id(
    189,
    "Mutavault",
    CardArt::new("927ed667-c228-4b96-a9f6-7cbadade8134", "Fred Fields"),
    CardSet::Magic2014,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{1}: This land becomes a 2/2 creature with all creature types until end of turn. It's still a land.",
            &[AbilityCostDef::Mana(mana_cost!("{1}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                // The animation keeps the land types Mutavault is printed with, so the
                // creature types are added rather than replacing anything.
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::ALL),
                    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M14 229 — Shimmering Grotto (reprint)

// M14 230 — Plains (reprint)

// M14 231 — Plains (alternate printing)

// M14 232 — Plains (alternate printing)

// M14 233 — Plains (alternate printing)

// M14 234 — Island (reprint)

// M14 235 — Island (alternate printing)

// M14 236 — Island (alternate printing)

// M14 237 — Island (alternate printing)

// M14 238 — Swamp (reprint)

// M14 239 — Swamp (alternate printing)

// M14 240 — Swamp (alternate printing)

// M14 241 — Swamp (alternate printing)

// M14 242 — Mountain (reprint)

// M14 243 — Mountain (alternate printing)

// M14 244 — Mountain (alternate printing)

// M14 245 — Mountain (alternate printing)

// M14 246 — Forest (reprint)

// M14 247 — Forest (alternate printing)

// M14 248 — Forest (alternate printing)

// M14 249 — Forest (alternate printing)

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AJANI_S_CHOSEN,
    &ANGELIC_ACCORD,
    &ARCHANGEL_OF_THUNE,
    &AURAMANCER,
    &BANISHER_PRIEST,
    &BONESCYTHE_SLIVER,
    &BRAVE_THE_ELEMENTS,
    &CAPASHEN_KNIGHT,
    &CELESTIAL_FLARE,
    &CHARGING_GRIFFIN,
    &CONGREGATE,
    &DAWNSTRIKE_PALADIN,
    &DEVOUT_INVOCATION,
    &FIENDSLAYER_PALADIN,
    &FORTIFY,
    &GRIFFIN_SENTINEL,
    &HIVE_STIRRINGS,
    &IMPOSING_SOVEREIGN,
    &INDESTRUCTIBILITY,
    &MASTER_OF_DIVERSION,
    &PATH_OF_BRAVERY,
    &SENTINEL_SLIVER,
    &SERAPH_OF_THE_SWORD,
    &SIEGE_MASTODON,
    &SOLEMN_OFFERING,
    &SOULMENDER,
    &STEELFORM_SLIVER,
    &STONEHORN_CHANTER,
    &SUNTAIL_HAWK,
    &AIR_SERVANT,
    &ARMORED_CANCRIX,
    &COLOSSAL_WHALE,
    &CORAL_MERFOLK,
    &DISMISS_INTO_DREAM,
    &DISPERSE,
    &ELITE_ARCANIST,
    &FROST_BREATH,
    &GALERIDER_SLIVER,
    &GLIMPSE_THE_FUTURE,
    &ILLUSIONARY_ARMOR,
    &JACE_S_MINDSEEKER,
    &MESSENGER_DRAKE,
    &OPPORTUNITY,
    &PHANTOM_WARRIOR,
    &QUICKEN,
    &SEACOAST_DRAKE,
    &TIDEBINDER_MAGE,
    &TIME_EBB,
    &TOME_SCOUR,
    &TRAINED_CONDOR,
    &TRAUMATIZE,
    &WALL_OF_FROST,
    &WARDEN_OF_EVOS_ISLE,
    &WATER_SERVANT,
    &WINDREADER_SPHINX,
    &ZEPHYR_CHARGE,
    &ACCURSED_SPIRIT,
    &ARTIFICER_S_HEX,
    &BLIGHTCASTER,
    &BLOOD_BAIRN,
    &BOGBREW_WITCH,
    &CHILD_OF_NIGHT,
    &CORPSE_HAULER,
    &DARK_PROPHECY,
    &DEATHGAZE_COCKATRICE,
    &DIABOLIC_TUTOR,
    &DOOM_BLADE,
    &FESTERING_NEWT,
    &GNAWING_ZOMBIE,
    &GRIM_RETURN,
    &LIFEBANE_ZOMBIE,
    &LILIANAS_REAVER,
    &LITURGY_OF_BLOOD,
    &MINOTAUR_ABOMINATION,
    &NIGHTWING_SHADE,
    &QUAG_SICKNESS,
    &RISE_OF_THE_DARK_REALMS,
    &SANGUINE_BOND,
    &SHADOWBORN_APOSTLE,
    &SHADOWBORN_DEMON,
    &SHRIVEL,
    &SYPHON_SLIVER,
    &TENACIOUS_DEAD,
    &UNDEAD_MINOTAUR,
    &VAMPIRE_WARLORD,
    &WRING_FLESH,
    &XATHRID_NECROMANCER,
    &ACADEMY_RAIDER,
    &AWAKEN_THE_ANCIENT,
    &BARRAGE_OF_EXPENDABLES,
    &BATTLE_SLIVER,
    &BLUR_SLIVER,
    &BURNING_EARTH,
    &CHANDRA_PYROMASTER,
    &CHANDRAS_OUTRAGE,
    &CHANDRA_S_PHOENIX,
    &CYCLOPS_TYRANT,
    &DRAGON_EGG,
    &FLESHPULPER_GIANT,
    &GOBLIN_DIPLOMATS,
    &GOBLIN_SHORTCUTTER,
    &LAVA_AXE,
    &LIGHTNING_TALONS,
    &MARAUDING_MAULHORN,
    &MINDSPARKER,
    &MOLTEN_BIRTH,
    &OGRE_BATTLEDRIVER,
    &REGATHAN_FIRECAT,
    &SCOURGE_OF_VALKAS,
    &SEISMIC_STOMP,
    &SHOCK,
    &STRIKING_SLIVER,
    &THORNCASTER_SLIVER,
    &THUNDER_STRIKE,
    &YOUNG_PYROMANCER,
    &ADVOCATE_OF_THE_BEAST,
    &BRINDLE_BOAR,
    &ELVISH_MYSTIC,
    &ENLARGE,
    &GARRUK_CALLER_OF_BEASTS,
    &GARRUK_S_HORDE,
    &GLADECOVER_SCOUT,
    &GROUNDSHAKER_SLIVER,
    &HOWL_OF_THE_NIGHT_PACK,
    &HUNT_THE_WEAK,
    &INTO_THE_WILDS,
    &KALONIAN_HYDRA,
    &KALONIAN_TUSKER,
    &LAY_OF_THE_LAND,
    &MANAWEFT_SLIVER,
    &MEGANTIC_SLIVER,
    &OATH_OF_THE_ANCIENT_WOOD,
    &PREDATORY_SLIVER,
    &PRIMEVAL_BOUNTY,
    &ROOTWALLA,
    &RUMBLING_BALOTH,
    &SAVAGE_SUMMONING,
    &SCAVENGING_OOZE,
    &SPOREMOUND,
    &TROLLHIDE,
    &VASTWOOD_HYDRA,
    &VORACIOUS_WURM,
    &WINDSTORM,
    &WITCHSTALKER,
    &WOODBORN_BEHEMOTH,
    &ACCORDERS_SHIELD,
    &BUBBLING_CAULDRON,
    &DARKSTEEL_FORGE,
    &DOOR_OF_DESTINIES,
    &FIRESHRIEKER,
    &GUARDIAN_OF_THE_AGES,
    &HAUNTED_PLATE_MAIL,
    &PYROMANCER_S_GAUNTLET,
    &RATCHET_BOMB,
    &RING_OF_THREE_WISHES,
    &SLIVER_CONSTRUCT,
    &STAFF_OF_THE_DEATH_MAGUS,
    &STAFF_OF_THE_FLAME_MAGUS,
    &STAFF_OF_THE_MIND_MAGUS,
    &STAFF_OF_THE_SUN_MAGUS,
    &STAFF_OF_THE_WILD_MAGUS,
    &STRIONIC_RESONATOR,
    &VIAL_OF_POISON,
    &ENCROACHING_WASTES,
    &MUTAVAULT,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&catalog_m13::AJANI_CALLER_OF_THE_PRIDE), // M14 1
    PrintingRecord::reprint(&avacyn_restored::ANGELIC_WALL),          // M14 4
    PrintingRecord::reprint(&alpha::BLESSING),                        // M14 8
    PrintingRecord::reprint(&magic_2013::DIVINE_FAVOR),               // M14 17
    PrintingRecord::reprint(&catalog_m13::PACIFISM),                  // M14 25
    PrintingRecord::reprint(&catalog_tor::PAY_NO_HEED),               // M14 27
    PrintingRecord::reprint(&magic_2013::PILLARFIELD_OX),             // M14 28
    PrintingRecord::reprint(&magic_2013::PLANAR_CLEANSING),           // M14 29
    PrintingRecord::reprint(&alpha::SERRA_ANGEL),                     // M14 32
    PrintingRecord::reprint(&magic_2013::SHOW_OF_VALOR),              // M14 33
    PrintingRecord::reprint(&catalog_m11::SILENCE),                   // M14 35
    PrintingRecord::reprint(&alpha::WALL_OF_SWORDS),                  // M14 41
    PrintingRecord::reprint(&magic_2013::ARCHAEOMANCER),              // M14 43
    PrintingRecord::reprint(&return_to_ravnica::CANCEL),              // M14 45
    PrintingRecord::reprint(&innistrad::CLAUSTROPHOBIA),              // M14 46
    PrintingRecord::reprint(&alpha::CLONE),                           // M14 47
    PrintingRecord::reprint(&dark_ascension::DIVINATION),             // M14 52
    PrintingRecord::reprint(&catalog_roe::DOMESTICATION),             // M14 53
    PrintingRecord::reprint(&magic_2013::ESSENCE_SCATTER),            // M14 55
    PrintingRecord::reprint(&magic_2013::JACE_MEMORY_ADEPT),          // M14 60
    PrintingRecord::reprint(&catalog_m11::MERFOLK_SPY),               // M14 62
    PrintingRecord::reprint(&magic_2013::NEGATE),                     // M14 64
    PrintingRecord::reprint(&dark_ascension::NEPHALIA_SEAKITE),       // M14 65
    PrintingRecord::reprint(&magic_2013::SCROLL_THIEF),               // M14 69
    PrintingRecord::reprint(&innistrad::SENSORY_DEPRIVATION),         // M14 71
    PrintingRecord::reprint(&alpha::SPELL_BLAST),                     // M14 72
    PrintingRecord::reprint(&catalog_isd::ALTARS_REAP),               // M14 84
    PrintingRecord::reprint(&catalog_usg::CORRUPT),                   // M14 91
    PrintingRecord::reprint(&magic_2013::DARK_FAVOR),                 // M14 92
    PrintingRecord::reprint(&magic_2013::DURESS),                     // M14 97
    PrintingRecord::reprint(&catalog_m13::LILIANA_OF_THE_DARK_REALMS), // M14 102
    PrintingRecord::reprint(&magic_2013::MARK_OF_THE_VAMPIRE),        // M14 105
    PrintingRecord::reprint(&magic_2013::MIND_ROT),                   // M14 106
    PrintingRecord::reprint(&catalog_lea::NIGHTMARE),                 // M14 108
    PrintingRecord::reprint(&alpha::SENGIR_VAMPIRE),                  // M14 113
    PrintingRecord::reprint(&magic_2013::VILE_REBIRTH),               // M14 121
    PrintingRecord::reprint(&gatecrash::ACT_OF_TREASON),              // M14 125
    PrintingRecord::reprint(&magic_2013::CANYON_MINOTAUR),            // M14 131
    PrintingRecord::reprint(&avacyn_restored::DEMOLISH),              // M14 136
    PrintingRecord::reprint(&magic_2013::DRAGON_HATCHLING),           // M14 138
    PrintingRecord::reprint(&magic_2013::FLAMES_OF_THE_FIREBRAND),    // M14 139
    PrintingRecord::reprint(&innistrad::PITCHBURN_DEVILS),            // M14 149
    PrintingRecord::reprint(&catalog_usg::SHIV_S_EMBRACE),            // M14 153
    PrintingRecord::reprint(&alpha::SHIVAN_DRAGON),                   // M14 154
    PrintingRecord::reprint(&magic_2013::SMELT),                      // M14 156
    PrintingRecord::reprint(&magic_2013::VOLCANIC_GEYSER),            // M14 160
    PrintingRecord::reprint(&catalog_m13::WILD_GUESS),                // M14 161
    PrintingRecord::reprint(&catalog_lrw::WILD_RICOCHET),             // M14 162
    PrintingRecord::reprint(&innistrad::BRAMBLECRUSH),                // M14 165
    PrintingRecord::reprint(&dark_ascension::BRIARPACK_ALPHA),        // M14 166
    PrintingRecord::reprint(&magic_2013::DEADLY_RECLUSE),             // M14 168
    PrintingRecord::reprint(&catalog_lea::FOG),                       // M14 171
    PrintingRecord::reprint(&alpha::GIANT_GROWTH),                    // M14 174
    PrintingRecord::reprint(&alpha::GIANT_SPIDER),                    // M14 175
    PrintingRecord::reprint(&onslaught::NATURALIZE),                  // M14 186
    PrintingRecord::reprint(&magic_2013::PLUMMET),                    // M14 188
    PrintingRecord::reprint(&innistrad::RANGERS_GUILE),               // M14 191
    PrintingRecord::reprint(&catalog_gtc::VERDANT_HAVEN),             // M14 199
    PrintingRecord::reprint(&darksteel::DARKSTEEL_INGOT),             // M14 207
    PrintingRecord::reprint(&catalog_m13::ELIXIR_OF_IMMORTALITY),     // M14 209
    PrintingRecord::reprint(&antiquities::MILLSTONE),                 // M14 213
    PrintingRecord::reprint(&alpha::ROD_OF_RUIN),                     // M14 217
    PrintingRecord::reprint(&catalog_m13::TRADING_POST),              // M14 225
    PrintingRecord::reprint(&catalog_lrw::SHIMMERING_GROTTO),         // M14 229
    PrintingRecord::reprint(&alpha::PLAINS),                          // M14 230
    PrintingRecord::alternate(&alpha::PLAINS, 1),                     // M14 231
    PrintingRecord::alternate(&alpha::PLAINS, 2),                     // M14 232
    PrintingRecord::alternate(&alpha::PLAINS, 3),                     // M14 233
    PrintingRecord::reprint(&alpha::ISLAND),                          // M14 234
    PrintingRecord::alternate(&alpha::ISLAND, 1),                     // M14 235
    PrintingRecord::alternate(&alpha::ISLAND, 2),                     // M14 236
    PrintingRecord::alternate(&alpha::ISLAND, 3),                     // M14 237
    PrintingRecord::reprint(&alpha::SWAMP),                           // M14 238
    PrintingRecord::alternate(&alpha::SWAMP, 1),                      // M14 239
    PrintingRecord::alternate(&alpha::SWAMP, 2),                      // M14 240
    PrintingRecord::alternate(&alpha::SWAMP, 3),                      // M14 241
    PrintingRecord::reprint(&alpha::MOUNTAIN),                        // M14 242
    PrintingRecord::alternate(&alpha::MOUNTAIN, 1),                   // M14 243
    PrintingRecord::alternate(&alpha::MOUNTAIN, 2),                   // M14 244
    PrintingRecord::alternate(&alpha::MOUNTAIN, 3),                   // M14 245
    PrintingRecord::reprint(&alpha::FOREST),                          // M14 246
    PrintingRecord::alternate(&alpha::FOREST, 1),                     // M14 247
    PrintingRecord::alternate(&alpha::FOREST, 2),                     // M14 248
    PrintingRecord::alternate(&alpha::FOREST, 3),                     // M14 249
];
