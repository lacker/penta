//! Magic 2014 card records used by the built-in ISD–DGM Standard decks.

use super::{CardRecord, PrintingRecord, gatecrash};
use crate::card::sets::{
    y1993::alpha,
    y1994::antiquities,
    y2002::onslaught,
    y2004::darksteel,
    y2011::innistrad,
    y2012::{avacyn_restored, dark_ascension, magic_2013, return_to_ravnica},
};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    AddManaEffectDef, AppliedEffectDef, AppliedRuleDef, BasicLandType,
    BattlefieldEntryModificationDef, CardArt, CardRules, CardSet, CardSupertype, CardType,
    CardTypeSet, ChoiceVisibilityDef, ChooseDef, ColorChoiceOperationDef, ComparisonDef,
    CounterKind, CreatureTypeSetDef, DamageEventMatcherDef, DiscardSelectionDef, EffectDef,
    EffectRecipientDef, HalvedValueDef, ManaColor, ObjectChoiceBindingDef, ObjectPredicateDef,
    ObjectQueryDef, ObjectRefDef, ObjectSetDef, PlayerRefDef, PlayerRelation, PlayerSetDef,
    ReplacementEffectDef, ReplacementEventDef, ResolvedEffectDurationDef, RoundingDef,
    SacrificedAmountDef, ScaledValueDef, TargetConditionDef, TopCardSelectionDef,
    TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement,
    abilities, cards,
};
use crate::ids::{ObjectBindingIndex, TargetIndex};
use crate::mana_cost;

// M14 2 — Ajani's Chosen
// Audit: blocked — CreateToken cannot continue by conditionally attaching the triggering Aura to the newly created token.

// M14 3 — Angelic Accord
// Audit: blocked — Trigger conditions cannot track life gained during the turn or compare that total with four.

// M14 5 — Archangel of Thune
pub(in crate::card::sets) static ARCHANGEL_OF_THUNE: CardRecord = CardRecord::new(
    cards::ARCHANGEL_OF_THUNE,
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
pub(in crate::card::sets) static AURAMANCER: CardRecord = CardRecord::new(
    cards::AURAMANCER,
    "Auramancer",
    CardArt::new("0a3dc4ab-1c45-4495-91b6-27d62087380c", "Rebecca Guay"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Wizard"], 2, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature enters, you may return target enchantment card from your graveyard to your hand.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)),
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Enchantment),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            })],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Hand,
                    controller: None,
                    placement: ZonePlacement::Top,
                    arrival_effect: None,
                    attachment: None,
                },
            },
        ),
    ),
);

// M14 7 — Banisher Priest
// Audit: blocked — Linked exile cannot express Banisher Priest's one-shot duration, which must do nothing if the source left before the enter trigger resolved.

// M14 9 — Bonescythe Sliver
pub(in crate::card::sets) static BONESCYTHE_SLIVER: CardRecord = CardRecord::new(
    cards::BONESCYTHE_SLIVER,
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
pub(in crate::card::sets) static BRAVE_THE_ELEMENTS: CardRecord = CardRecord::new(
    cards::BRAVE_THE_ELEMENTS,
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
pub(in crate::card::sets) static CAPASHEN_KNIGHT: CardRecord = CardRecord::new(
    cards::CAPASHEN_KNIGHT,
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
pub(in crate::card::sets) static CELESTIAL_FLARE: CardRecord = CardRecord::new(
    cards::CELESTIAL_FLARE,
    "Celestial Flare",
    CardArt::new("6c8d1320-0f1a-4c66-86c9-9f8da0f1d9ef", "Clint Cearley"),
    CardSet::Magic2014,
    CardRules::new_instant(mana_cost!("{W}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target player sacrifices an attacking or blocking creature of their choice.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::SacrificeOfChoice {
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
pub(in crate::card::sets) static CHARGING_GRIFFIN: CardRecord = CardRecord::new(
    cards::CHARGING_GRIFFIN,
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

/// Every creature on the battlefield, both sides included: the card says "on
/// the battlefield" rather than "you control".
static CONGREGATE_CREATURES: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::Any,
);

static CONGREGATE_AMOUNT: ScaledValueDef =
    ScaledValueDef::new(ValueDef::CountMatchingObjects(&CONGREGATE_CREATURES), 2);

// M14 14 — Congregate
pub(in crate::card::sets) static CONGREGATE: CardRecord = CardRecord::new(
    cards::CONGREGATE,
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
            amount: ValueDef::Scaled(&CONGREGATE_AMOUNT),
        },
    )),
);

// M14 15 — Dawnstrike Paladin
pub(in crate::card::sets) static DAWNSTRIKE_PALADIN: CardRecord = CardRecord::new(
    cards::DAWNSTRIKE_PALADIN,
    "Dawnstrike Paladin",
    CardArt::new("93cf5fb3-bb41-4efa-9721-2c2d169b05cd", "Tyler Jacobson"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Human", "Knight"], 2, 4)
        .with_abilities(&[abilities::vigilance(), abilities::lifelink()]),
);

// M14 16 — Devout Invocation
// Audit: blocked — Spell costs cannot tap a freely chosen number of creatures and carry that paid count into token creation.

// M14 18 — Fiendslayer Paladin
// Audit: blocked — Targeting restrictions cannot filter opposing spell sources by black or red color while leaving abilities unaffected.

// M14 19 — Fortify
pub(in crate::card::sets) static FORTIFY: CardRecord = CardRecord::new(
    cards::FORTIFY,
    "Fortify",
    CardArt::new("1eff4028-d4f9-4822-81d6-9f5e5e6f3011", "Christopher Moeller"),
    CardSet::Magic2014,
    CardRules::new_instant(mana_cost!("{2}{W}")).with_ability(AbilityDef::choose_one_spell(
        "Choose one —\n• Creatures you control get +2/+0 until end of turn.\n• Creatures you control get +0/+2 until end of turn.",
        &[
            AbilityDef::spell(
                "Creatures you control get +2/+0 until end of turn.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::You),
                    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(0)),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell(
                "Creatures you control get +0/+2 until end of turn.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::You),
                    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(0), ValueDef::Constant(2)),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    )),
);

// M14 20 — Griffin Sentinel
pub(in crate::card::sets) static GRIFFIN_SENTINEL: CardRecord = CardRecord::new(
    cards::GRIFFIN_SENTINEL,
    "Griffin Sentinel",
    CardArt::new("b40d6626-a85f-4116-9721-19e39b83cba0", "Warren Mahy"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Griffin"], 1, 3)
        .with_abilities(&[abilities::flying(), abilities::vigilance()]),
);

// M14 21 — Hive Stirrings
pub(in crate::card::sets) static HIVE_STIRRINGS: CardRecord = CardRecord::new(
    cards::HIVE_STIRRINGS,
    "Hive Stirrings",
    CardArt::new("e4399e19-d05d-4bb3-9aff-c4133ddd2850", "Maciej Kuciara"),
    CardSet::Magic2014,
    CardRules::new_sorcery(mana_cost!("{2}{W}")).with_ability(AbilityDef::spell(
        "Create two 1/1 colorless Sliver creature tokens.",
        EffectDef::CreateToken {
            token: cards::SLIVER_TOKEN_1_1_COLORLESS,
            controller: None,
            count: ValueDef::Constant(2),
            tapped: false,
            attacking: false,
            counters: None,
            created: None,
        },
    )),
);

// M14 22 — Imposing Sovereign
pub(in crate::card::sets) static IMPOSING_SOVEREIGN: CardRecord = CardRecord::new(
    cards::IMPOSING_SOVEREIGN,
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
pub(in crate::card::sets) static INDESTRUCTIBILITY: CardRecord = CardRecord::new(
    cards::INDESTRUCTIBILITY,
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
// Audit: blocked — Attack triggers cannot restrict a target to the creature controlled by that attack's defending player.

// M14 26 — Path of Bravery
// Audit: blocked — Continuous effects cannot compare current and starting life, and attack events cannot aggregate one combat's attackers into one trigger amount.

// M14 27 — Pay No Heed
// Audit: blocked — Effects cannot choose a nontarget source and install a turn-long prevention shield against all damage from it.

// M14 30 — Sentinel Sliver
pub(in crate::card::sets) static SENTINEL_SLIVER: CardRecord = CardRecord::new(
    cards::SENTINEL_SLIVER,
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
pub(in crate::card::sets) static SERAPH_OF_THE_SWORD: CardRecord = CardRecord::new(
    cards::SERAPH_OF_THE_SWORD,
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

// M14 34 — Siege Mastodon
pub(in crate::card::sets) static SIEGE_MASTODON: CardRecord = CardRecord::new(
    cards::SIEGE_MASTODON,
    "Siege Mastodon",
    CardArt::new("40e7a30f-bb29-4c6b-bf70-53e9e4292814", "Matt Cavotta"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Elephant"], 3, 5),
);

// M14 35 — Silence
// Audit: blocked — The turn-scoped cast prohibition only covers noncreature spells, not every spell an opponent could cast.

// M14 36 — Solemn Offering
pub(in crate::card::sets) static SOLEMN_OFFERING: CardRecord = CardRecord::new(
    cards::SOLEMN_OFFERING,
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
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(4),
            },
        ]),
    )),
);

// M14 37 — Soulmender
pub(in crate::card::sets) static SOULMENDER: CardRecord = CardRecord::new(
    cards::SOULMENDER,
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
pub(in crate::card::sets) static STEELFORM_SLIVER: CardRecord = CardRecord::new(
    cards::STEELFORM_SLIVER,
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
pub(in crate::card::sets) static STONEHORN_CHANTER: CardRecord = CardRecord::new(
    cards::STONEHORN_CHANTER,
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
pub(in crate::card::sets) static SUNTAIL_HAWK: CardRecord = CardRecord::new(
    cards::SUNTAIL_HAWK,
    "Suntail Hawk",
    CardArt::new("28a1f83c-a9ef-463e-97b5-2ca3b7232f82", "Heather Hudson"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{W}"), &["Bird"], 1, 1)
        .with_abilities(&[abilities::flying()]),
);

// M14 42 — Air Servant
pub(in crate::card::sets) static AIR_SERVANT: CardRecord = CardRecord::new(
    cards::AIR_SERVANT,
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

// M14 44 — Armored Cancrix
pub(in crate::card::sets) static ARMORED_CANCRIX: CardRecord = CardRecord::new(
    cards::ARMORED_CANCRIX,
    "Armored Cancrix",
    CardArt::new("3b455b0f-a69c-43b4-bbf5-605ed41f10e0", "Tomasz Jedruszek"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Crab"], 2, 5),
);

// M14 48 — Colossal Whale
// Audit: blocked — Islandwalk and defending-player targeting are unsupported, and linked exile cannot express the required one-shot duration if the source leaves early.

// M14 49 — Coral Merfolk
pub(in crate::card::sets) static CORAL_MERFOLK: CardRecord = CardRecord::new(
    cards::CORAL_MERFOLK,
    "Coral Merfolk",
    CardArt::new("09ef366b-26f5-473a-ab96-e668ed54d691", "rk post"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Merfolk"], 2, 1),
);

// M14 50 — Dismiss into Dream
// Audit: blocked — Static effects cannot add a creature subtype, and no trigger event observes a permanent becoming a target.

// M14 51 — Disperse
pub(in crate::card::sets) static DISPERSE: CardRecord = CardRecord::new(
    cards::DISPERSE,
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
            controller: None,
            placement: ZonePlacement::Top,
            arrival_effect: None,
            attachment: None,
        },
    )),
);

// M14 53 — Domestication
// Audit: blocked — There is no source-bound continuous control effect, and the power condition would need effective power plus intervening-if rechecks.

// M14 54 — Elite Arcanist
// Audit: blocked — Imprint cannot retain a chosen hand card for a later X cost, spell copy, and free-cast permission.

/// "Up to two", so nothing at all is a legal declaration, and the skip is
/// counted on each creature separately -- the two may belong to different
/// players, who do not reach their untap steps together.
static FROST_BREATH_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::up_to(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
    2,
)];

static FROST_BREATH_EFFECT: [EffectDef; 2] = [
    EffectDef::Tap {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::SkipNextUntapSteps {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        count: 1,
    },
];

// M14 56 — Frost Breath
pub(in crate::card::sets) static FROST_BREATH: CardRecord = CardRecord::new(
    cards::FROST_BREATH,
    "Frost Breath",
    CardArt::new("85d3f777-7660-48ae-8c32-6777ec8427d4", "Mike Bierek"),
    CardSet::Magic2014,
    CardRules::new_instant(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Tap up to two target creatures. Those creatures don't untap during their controller's \
         next untap step.",
        &FROST_BREATH_TARGETS,
        EffectDef::Sequence(&FROST_BREATH_EFFECT),
    )),
);

// M14 57 — Galerider Sliver
pub(in crate::card::sets) static GALERIDER_SLIVER: CardRecord = CardRecord::new(
    cards::GALERIDER_SLIVER,
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
pub(in crate::card::sets) static GLIMPSE_THE_FUTURE: CardRecord = CardRecord::new(
    cards::GLIMPSE_THE_FUTURE,
    "Glimpse the Future",
    CardArt::new("f4d875e9-713d-4ddb-ae0a-db8483366319", "Andrew Robinson"),
    CardSet::Magic2014,
    CardRules::new_sorcery(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell(
        "Look at the top three cards of your library. Put one of them into your hand and the rest into your graveyard.",
        EffectDef::LookAtTopAndSelect {
            player: EffectRecipientDef::Controller,
            looker: EffectRecipientDef::Controller,
            selection: &TopCardSelectionDef {
                count: ValueDef::Constant(3),
                object: None,
                minimum: 1,
                maximum: 1,
                select_all_matching: false,
                reveal_selected: false,
                selected_zone: ZoneKind::Hand,
                selected_placement: ZonePlacement::Top,
                rest_zone: ZoneKind::Graveyard,
                rest_placement: ZonePlacement::Top,
                selected_order_follows_choice: false,
                then: None,
            selected_face_down: false,},
        },
    )),
);

// M14 59 — Illusionary Armor
// Audit: blocked — No trigger event observes the enchanted creature becoming a target.

// M14 61 — Jace's Mindseeker
// Audit: blocked — Mill cannot retain the exact milled batch for a filtered optional free-cast choice.

// M14 62 — Merfolk Spy
// Audit: blocked — Islandwalk is unsupported and hand reveal cannot select one uniformly random card.

// M14 63 — Messenger Drake
pub(in crate::card::sets) static MESSENGER_DRAKE: CardRecord = CardRecord::new(
    cards::MESSENGER_DRAKE,
    "Messenger Drake",
    CardArt::new("13dd3172-0b45-4dc8-adc6-9e0ba112e664", "Yeong-Hao Han"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{3}{U}{U}"), &["Drake"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "When this creature dies, draw a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// M14 66 — Opportunity
pub(in crate::card::sets) static OPPORTUNITY: CardRecord = CardRecord::new(
    cards::OPPORTUNITY,
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
pub(in crate::card::sets) static PHANTOM_WARRIOR: CardRecord = CardRecord::new(
    cards::PHANTOM_WARRIOR,
    "Phantom Warrior",
    CardArt::new("e12a1a64-5b32-4b85-8fae-c407d7926547", "Greg Staples"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Illusion", "Warrior"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "This creature can't be blocked.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeBlockedBy(
                    ObjectPredicateDef::Any,
                )),
            },
        ),
    ),
);

// M14 68 — Quicken
pub(in crate::card::sets) static QUICKEN: CardRecord = CardRecord::new(
    cards::QUICKEN,
    "Quicken",
    CardArt::new("066bef3d-c785-4b25-9b91-8f676aa9906f", "Aleksi Briclot"),
    CardSet::Magic2014,
    // One spell ability per part, so the card's two sentences are one clause
    // with a sequence rather than two spell clauses.
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell(
        "The next sorcery spell you cast this turn can be cast as though it had flash. (It can be cast any time you could cast an instant.)\nDraw a card.",
        EffectDef::Sequence(&[
            EffectDef::GrantFlashToNextSorcery,
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// M14 70 — Seacoast Drake
pub(in crate::card::sets) static SEACOAST_DRAKE: CardRecord = CardRecord::new(
    cards::SEACOAST_DRAKE,
    "Seacoast Drake",
    CardArt::new("5333de10-a6d4-47ff-ab57-4edb49535739", "Scott Chou"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Drake"], 1, 3)
        .with_abilities(&[abilities::flying()]),
);

// M14 73 — Tidebinder Mage
// Audit: blocked — Effect durations cannot end when the source changes controller while remaining on the battlefield.

// M14 74 — Time Ebb
pub(in crate::card::sets) static TIME_EBB: CardRecord = CardRecord::new(
    cards::TIME_EBB,
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
            controller: None,
            placement: ZonePlacement::Top,
            arrival_effect: None,
            attachment: None,
        },
    )),
);

// M14 75 — Tome Scour
pub(in crate::card::sets) static TOME_SCOUR: CardRecord = CardRecord::new(
    cards::TOME_SCOUR,
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
            binding: None,
            then: None,
        },
    )),
);

// M14 76 — Trained Condor
pub(in crate::card::sets) static TRAINED_CONDOR: CardRecord = CardRecord::new(
    cards::TRAINED_CONDOR,
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

/// Half of what the library holds when the spell resolves, rounded down.
/// Reading it from the target rather than from a fixed count is the whole
/// clause: a Traumatize into an empty library mills nothing.
static HALF_THEIR_LIBRARY: HalvedValueDef = HalvedValueDef::new(
    ValueDef::TargetLibrarySize(TargetIndex::PRIMARY),
    RoundingDef::Down,
);

// M14 77 — Traumatize
pub(in crate::card::sets) static TRAUMATIZE: CardRecord = CardRecord::new(
    cards::TRAUMATIZE,
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
            amount: ValueDef::Halved(&HALF_THEIR_LIBRARY),
            binding: None,
            then: None,
        },
    )),
);

// M14 78 — Wall of Frost
pub(in crate::card::sets) static WALL_OF_FROST: CardRecord = CardRecord::new(
    cards::WALL_OF_FROST,
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
// Audit: blocked — Generic-cost reduction cannot be filtered to creature spells with effective flying.

// M14 80 — Water Servant
pub(in crate::card::sets) static WATER_SERVANT: CardRecord = CardRecord::new(
    cards::WATER_SERVANT,
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
pub(in crate::card::sets) static WINDREADER_SPHINX: CardRecord = CardRecord::new(
    cards::WINDREADER_SPHINX,
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
pub(in crate::card::sets) static ZEPHYR_CHARGE: CardRecord = CardRecord::new(
    cards::ZEPHYR_CHARGE,
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
pub(in crate::card::sets) static ACCURSED_SPIRIT: CardRecord = CardRecord::new(
    cards::ACCURSED_SPIRIT,
    "Accursed Spirit",
    CardArt::new("cf08313b-14c9-4e0b-aad7-05cbd90b1ed8", "Kev Walker"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Spirit"], 3, 2)
        .with_abilities(&[abilities::intimidate()]),
);

// M14 85 — Artificer's Hex
// Audit: blocked — Conditions and recipients cannot follow an Aura to an Equipment and then to the creature that Equipment is attached to.

// M14 86 — Blightcaster
pub(in crate::card::sets) static BLIGHTCASTER: CardRecord = CardRecord::new(
    cards::BLIGHTCASTER,
    "Blightcaster",
    CardArt::new("61752b13-255a-44d0-9fb0-5ed5680b954e", "Winona Nelson"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Human", "Wizard"], 2, 3).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever you cast an enchantment spell, you may have target creature get -2/-2 until end of turn.",
            TriggerEventDef::SpellCast(ObjectPredicateDef::All(&[
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
pub(in crate::card::sets) static BLOOD_BAIRN: CardRecord = CardRecord::new(
    cards::BLOOD_BAIRN,
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
// Audit: blocked — SearchZone cannot match either exact printed name or make the selected permanent enter tapped.

// M14 89 — Child of Night
pub(in crate::card::sets) static CHILD_OF_NIGHT: CardRecord = CardRecord::new(
    cards::CHILD_OF_NIGHT,
    "Child of Night",
    CardArt::new("c21b5476-5f5f-46b5-b627-398e9fcd04aa", "Ash Wood"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Vampire"], 2, 1)
        .with_abilities(&[abilities::lifelink()]),
);

// M14 90 — Corpse Hauler
pub(in crate::card::sets) static CORPSE_HAULER: CardRecord = CardRecord::new(
    cards::CORPSE_HAULER,
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
                controller: None,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
            },
        ),
    ),
);

// M14 91 — Corrupt
// Audit: blocked — No effect can gain exactly the damage Corrupt actually dealt after prevention without incorrectly capping overkill damage.

// M14 93 — Dark Prophecy
pub(in crate::card::sets) static DARK_PROPHECY: CardRecord = CardRecord::new(
    cards::DARK_PROPHECY,
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
pub(in crate::card::sets) static DEATHGAZE_COCKATRICE: CardRecord = CardRecord::new(
    cards::DEATHGAZE_COCKATRICE,
    "Deathgaze Cockatrice",
    CardArt::new("9f17b58c-9738-4cdb-a408-e1595c384b92", "Kev Walker"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Cockatrice"], 2, 2)
        .with_abilities(&[abilities::flying(), abilities::deathtouch()]),
);

// M14 95 — Diabolic Tutor
pub(in crate::card::sets) static DIABOLIC_TUTOR: CardRecord = CardRecord::new(
    cards::DIABOLIC_TUTOR,
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
            binding: None,
            then: None,
        },
    )),
);

// M14 96 — Doom Blade
pub(in crate::card::sets) static DOOM_BLADE: CardRecord = CardRecord::new(
    cards::DOOM_BLADE,
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

// M14 98 — Festering Newt
// Audit: blocked — Object predicates cannot test for a different exact card name to choose between two effect amounts.

// M14 99 — Gnawing Zombie
pub(in crate::card::sets) static GNAWING_ZOMBIE: CardRecord = CardRecord::new(
    cards::GNAWING_ZOMBIE,
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
// Audit: blocked — Graveyard target predicates cannot inspect whether a card moved there from the battlefield this turn.

static LIFEBANE_EXILE: EffectDef = EffectDef::MoveToZone {
    object: EffectRecipientDef::object(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
    zone: ZoneKind::Exile,
    placement: ZonePlacement::Top,
    arrival_effect: None,
    attachment: None,
    controller: None,
};

static LIFEBANE_EFFECTS: [EffectDef; 2] = [
    EffectDef::LookAtHand {
        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::Choose(ChooseDef {
        binding: ObjectChoiceBindingDef::Object(ObjectBindingIndex::PRIMARY),
        unchosen: None,
        chooser: PlayerRefDef::EffectController,
        candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Color(ManaColor::Green),
                    ObjectPredicateDef::Color(ManaColor::White),
                ]),
            ]),
            &[ZoneKind::Hand],
            PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
        )),
        exclude: None,
        minimum: 1,
        maximum: 1,
        visibility: ChoiceVisibilityDef::Public,
        then: &LIFEBANE_EXILE,
    }),
];

// M14 101 — Lifebane Zombie
pub(in crate::card::sets) static LIFEBANE_ZOMBIE: CardRecord = CardRecord::new(
    cards::LIFEBANE_ZOMBIE,
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
        AbilityDef::triggered_with_targets("When this creature enters, target opponent reveals their hand. You choose a green or white creature card from it and exile that card.", TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)), &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Opponent),
        )], EffectDef::Sequence(&LIFEBANE_EFFECTS)),
    ]),
);

static TAPPED_ZOMBIE: EffectDef = EffectDef::CreateToken {
    token: cards::ZOMBIE_TOKEN_2_2_BLACK,
    controller: None,
    count: ValueDef::Constant(1),
    tapped: true,
    attacking: false,
    counters: None,
    created: None,
};

static LILIANAS_REAVER_STRIKE: [EffectDef; 2] = [
    EffectDef::Discard {
        recipient: EffectRecipientDef::EventPlayer,
        amount: ValueDef::Constant(1),
        selection: DiscardSelectionDef::RecipientChooses,
        then: None,
    },
    TAPPED_ZOMBIE,
];

// M14 103 — Liliana's Reaver
pub(in crate::card::sets) static LILIANAS_REAVER: CardRecord = CardRecord::new(
    cards::LILIANAS_REAVER,
    "Liliana's Reaver",
    CardArt::new("a734c33c-4fa0-4f7a-943c-14a8aecea1a6", "Karl Kopinski"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Zombie"], 4, 3).with_abilities(&[
        abilities::deathtouch(),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, that player discards a \
             card and you create a tapped 2/2 black Zombie creature token.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::Sequence(&LILIANAS_REAVER_STRIKE),
        ),
    ]),
);

// M14 104 — Liturgy of Blood
pub(in crate::card::sets) static LITURGY_OF_BLOOD: CardRecord = CardRecord::new(
    cards::LITURGY_OF_BLOOD,
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
            },
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black).with_amount(3)),
        ]),
    )),
);

// M14 107 — Minotaur Abomination
pub(in crate::card::sets) static MINOTAUR_ABOMINATION: CardRecord = CardRecord::new(
    cards::MINOTAUR_ABOMINATION,
    "Minotaur Abomination",
    CardArt::new("9dca75a1-443d-4f8e-b12b-2aada3a8e3e4", "Karl Kopinski"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Zombie", "Minotaur"], 4, 6),
);

// M14 108 — Nightmare
// Audit: blocked — Dynamic power/toughness effects are battlefield-only and cannot implement a characteristic-defining ability in every zone.

// M14 109 — Nightwing Shade
pub(in crate::card::sets) static NIGHTWING_SHADE: CardRecord = CardRecord::new(
    cards::NIGHTWING_SHADE,
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
// Audit: partial — Static power/toughness modifiers cannot negate a dynamic count of Swamps you control.
pub(in crate::card::sets) static QUAG_SICKNESS: CardRecord = CardRecord::new(
    cards::QUAG_SICKNESS,
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
            AbilityDef::not_implemented(
                "Enchanted creature gets -1/-1 for each Swamp you control.",
                "Static power/toughness modifiers cannot negate a dynamic battlefield-object count.",
            ),
        ]),
);

// M14 111 — Rise of the Dark Realms
// Audit: blocked — MoveToZone cannot sweep matching cards from every graveyard into one player's control.

// M14 112 — Sanguine Bond
pub(in crate::card::sets) static SANGUINE_BOND: CardRecord = CardRecord::new(
    cards::SANGUINE_BOND,
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

// M14 114 — Shadowborn Apostle
// Audit: blocked — Deck construction has no any-number exception, and activated costs cannot choose and sacrifice six matching permanents as one payment.

/// Fewer than six is at most five. The count is of creature cards in your
/// own graveyard, which the Demon feeds on and which is why it stops eating
/// your board once the graveyard is full enough.
static SHADOWBORN_DEMON_UPKEEP_CONDITION: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef::matching(
        ObjectPredicateDef::HasType(CardType::Creature),
        &[ZoneKind::Graveyard],
        PlayerRelation::You,
    ),
    comparison: ComparisonDef::LessOrEqual,
    amount: 5,
};

// M14 115 — Shadowborn Demon
pub(in crate::card::sets) static SHADOWBORN_DEMON: CardRecord = CardRecord::new(
    cards::SHADOWBORN_DEMON,
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
        AbilityDef::triggered_with_targets("When this creature enters, destroy target non-Demon creature.", TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)), &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Demon")),
            ]),
        )], EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            }),
        AbilityDef::triggered_if(
            "At the beginning of your upkeep, if there are fewer than six creature cards in your graveyard, sacrifice a creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &SHADOWBORN_DEMON_UPKEEP_CONDITION,
            EffectDef::SacrificeOfChoice {
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
pub(in crate::card::sets) static SHRIVEL: CardRecord = CardRecord::new(
    cards::SHRIVEL,
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
pub(in crate::card::sets) static SYPHON_SLIVER: CardRecord = CardRecord::new(
    cards::SYPHON_SLIVER,
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
// Audit: blocked — The effect vocabulary cannot make the returned source enter tapped after a death trigger.

// M14 119 — Undead Minotaur
pub(in crate::card::sets) static UNDEAD_MINOTAUR: CardRecord = CardRecord::new(
    cards::UNDEAD_MINOTAUR,
    "Undead Minotaur",
    CardArt::new("5e5ae910-ee1d-4958-92d9-0b06872913c6", "Karl Kopinski"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Zombie", "Minotaur"], 2, 3),
);

// M14 120 — Vampire Warlord
pub(in crate::card::sets) static VAMPIRE_WARLORD: CardRecord = CardRecord::new(
    cards::VAMPIRE_WARLORD,
    "Vampire Warlord",
    CardArt::new("7e07929b-450c-45b0-85e6-512ad280a122", "Wesley Burt"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Vampire", "Warrior"], 4, 2).with_ability(
        abilities::regenerate_self(
            "Sacrifice another creature: Regenerate this creature.",
            &[AbilityCostDef::SacrificePermanent {
                object: ANOTHER_CREATURE,
                controller: PlayerRelation::You,
            }],
        ),
    ),
);

static ANOTHER_CREATURE: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
]);

// M14 122 — Wring Flesh
pub(in crate::card::sets) static WRING_FLESH: CardRecord = CardRecord::new(
    cards::WRING_FLESH,
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

/// "This creature or another Human creature you control" is every Human
/// creature its controller controls, since the Necromancer is one itself.
static YOUR_HUMAN_CREATURES: [ObjectPredicateDef; 3] = [
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::Subtype("Human"),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
];

// M14 123 — Xathrid Necromancer
pub(in crate::card::sets) static XATHRID_NECROMANCER: CardRecord = CardRecord::new(
    cards::XATHRID_NECROMANCER,
    "Xathrid Necromancer",
    CardArt::new("26494f96-1d97-4435-a116-3ade1becaab4", "Maciej Kuciara"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Human", "Wizard"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever this creature or another Human creature you control dies, create a \
             tapped 2/2 black Zombie creature token.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&YOUR_HUMAN_CREATURES),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            TAPPED_ZOMBIE,
        ),
    ),
);

// M14 124 — Academy Raider
// Audit: blocked — The optional discard cost needs a continuation that draws only when a card was actually discarded.

// M14 126 — Awaken the Ancient
// Audit: partial — Static effects cannot animate an attached land while preserving its land characteristics through the shared runtime.
pub(in crate::card::sets) static AWAKEN_THE_ANCIENT: CardRecord = CardRecord::new(
    cards::AWAKEN_THE_ANCIENT,
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
            AbilityDef::not_implemented(
                "Enchanted Mountain is a 7/7 red Giant creature with haste. It's still a land.",
                "Animating an attached land through a static effect is outside the shared continuous-effect runtime.",
            ),
        ]),
);

// M14 127 — Barrage of Expendables
pub(in crate::card::sets) static BARRAGE_OF_EXPENDABLES: CardRecord = CardRecord::new(
    cards::BARRAGE_OF_EXPENDABLES,
    "Barrage of Expendables",
    CardArt::new("b9e0912d-b4b9-497c-bce7-ed80b79bab32", "Trevor Claxton"),
    CardSet::Magic2014,
    CardRules::new_enchantment(mana_cost!("{R}")).with_ability(AbilityDef::activated_with_targets(
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
    )),
);

// M14 128 — Battle Sliver
pub(in crate::card::sets) static BATTLE_SLIVER: CardRecord = CardRecord::new(
    cards::BATTLE_SLIVER,
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
pub(in crate::card::sets) static BLUR_SLIVER: CardRecord = CardRecord::new(
    cards::BLUR_SLIVER,
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
pub(in crate::card::sets) static BURNING_EARTH: CardRecord = CardRecord::new(
    cards::BURNING_EARTH,
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

// M14 132 — Chandra, Pyromaster
// Audit: blocked — Planeswalker support lacks a turn-long cannot-block effect, top-card exile/play permission, and the ultimate's repeatable spell-copy procedure.

// M14 133 — Chandra's Outrage
pub(in crate::card::sets) static CHANDRAS_OUTRAGE: CardRecord = CardRecord::new(
    cards::CHANDRAS_OUTRAGE,
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
// Audit: blocked — Damage events cannot filter the source as a red instant, sorcery, or planeswalker and then return this card from the graveyard.

// M14 135 — Cyclops Tyrant
// Audit: blocked — Combat restrictions cannot compare a prospective attacker's effective power when deciding whether this creature may block it.

// M14 137 — Dragon Egg
pub(in crate::card::sets) static DRAGON_EGG: CardRecord = CardRecord::new(
    cards::DRAGON_EGG,
    "Dragon Egg",
    CardArt::new("dc2048f7-0c68-4142-9aad-de9b91fe5958", "Jack Wang"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Dragon", "Egg"], 0, 2).with_abilities(&[
        abilities::defender(),
        AbilityDef::triggered(
            "When this creature dies, create a 2/2 red Dragon creature token with flying and \"{R}: This token gets +1/+0 until end of turn.\"",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, Some(ZoneKind::Battlefield), Some(ZoneKind::Graveyard)),
            EffectDef::CreateToken {
                token: cards::DRAGON_TOKEN_2_2_RED,
                controller: None,
                count: ValueDef::Constant(1),
                tapped: false,
                attacking: false,
            counters: None,
            created: None,},
        ),
    ]),
);

// M14 140 — Fleshpulper Giant
// Audit: partial — The toughness target predicate does not yet account for continuous static power/toughness effects.
pub(in crate::card::sets) static FLESHPULPER_GIANT: CardRecord = CardRecord::new(
    cards::FLESHPULPER_GIANT,
    "Fleshpulper Giant",
    CardArt::new(
        "f2726d3c-c182-4d8a-a723-0de2c5c4b152",
        "Alex Horley-Orlandelli",
    ),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{5}{R}{R}"), &["Giant"], 4, 4).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature enters, you may destroy target creature with toughness 2 or less.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)),
            &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ToughnessLessThan(ValueDef::Constant(3)),
            ]))],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                },
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "The toughness target predicate does not yet account for continuous static power/toughness effects.",
        )),
    ),
);

// M14 141 — Goblin Diplomats
pub(in crate::card::sets) static GOBLIN_DIPLOMATS: CardRecord = CardRecord::new(
    cards::GOBLIN_DIPLOMATS,
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
pub(in crate::card::sets) static GOBLIN_SHORTCUTTER: CardRecord = CardRecord::new(
    cards::GOBLIN_SHORTCUTTER,
    "Goblin Shortcutter",
    CardArt::new("71bccbec-6e1e-43d5-b0dc-eddf942fa798", "Jesper Ejsing"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Scout"], 2, 1).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature enters, target creature can't block this turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBlock),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// M14 143 — Lava Axe
pub(in crate::card::sets) static LAVA_AXE: CardRecord = CardRecord::new(
    cards::LAVA_AXE,
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
pub(in crate::card::sets) static LIGHTNING_TALONS: CardRecord = CardRecord::new(
    cards::LIGHTNING_TALONS,
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
// Audit: blocked — Attack requirements cannot be conditional on controlling a permanent with a different exact name.

// M14 146 — Mindsparker
pub(in crate::card::sets) static MINDSPARKER: CardRecord = CardRecord::new(
    cards::MINDSPARKER,
    "Mindsparker",
    CardArt::new("a94295dc-d078-4f3f-9856-bd0a1899a9ca", "Wayne Reynolds"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Elemental"], 3, 2).with_abilities(&[
        abilities::first_strike(),
        AbilityDef::triggered(
            "Whenever an opponent casts a white or blue instant or sorcery spell, this creature deals 2 damage to that player.",
            TriggerEventDef::SpellCast(ObjectPredicateDef::All(&[
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
// Audit: blocked — There is no coin-flip decision or result-conditioned self-return effect.

// M14 148 — Ogre Battledriver
pub(in crate::card::sets) static OGRE_BATTLEDRIVER: CardRecord = CardRecord::new(
    cards::OGRE_BATTLEDRIVER,
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

// M14 150 — Regathan Firecat
pub(in crate::card::sets) static REGATHAN_FIRECAT: CardRecord = CardRecord::new(
    cards::REGATHAN_FIRECAT,
    "Regathan Firecat",
    CardArt::new("4b4df1dd-886d-4fe7-b3f7-2dca044de41c", "Eric Velhagen"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Elemental", "Cat"], 4, 1),
);

// M14 151 — Scourge of Valkas
pub(in crate::card::sets) static SCOURGE_OF_VALKAS: CardRecord = CardRecord::new(
    cards::SCOURGE_OF_VALKAS,
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
// Audit: blocked — No group effect prohibits blocking for the turn, and the flying predicate must use effective continuous abilities.

// M14 153 — Shiv's Embrace
// Audit: blocked — A granted activated ability cannot retain the enchanted creature as its recipient if the Aura leaves before resolution.

// M14 155 — Shock
pub(in crate::card::sets) static SHOCK: CardRecord = CardRecord::new(
    cards::SHOCK,
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

// M14 157 — Striking Sliver
pub(in crate::card::sets) static STRIKING_SLIVER: CardRecord = CardRecord::new(
    cards::STRIKING_SLIVER,
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

static THORNCASTER_SLIVER_GRANTED_ABILITY: AbilityDef = AbilityDef::triggered_with_targets(
    "Whenever this creature attacks, it deals 1 damage to any target.",
    TriggerEventDef::attacks(ObjectPredicateDef::Source),
    &[AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::AnyTarget,
    )],
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: ValueDef::Constant(1),
    },
);

// M14 158 — Thorncaster Sliver
pub(in crate::card::sets) static THORNCASTER_SLIVER: CardRecord = CardRecord::new(
    cards::THORNCASTER_SLIVER,
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
                effect: AppliedEffectDef::add_ability(&THORNCASTER_SLIVER_GRANTED_ABILITY),
            },
        ),
    ),
);

// M14 159 — Thunder Strike
pub(in crate::card::sets) static THUNDER_STRIKE: CardRecord = CardRecord::new(
    cards::THUNDER_STRIKE,
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

// M14 162 — Wild Ricochet
// Audit: blocked — Stack effects cannot retarget a spell, copy it, and independently retarget the copy.

// M14 163 — Young Pyromancer
pub(in crate::card::sets) static YOUNG_PYROMANCER: CardRecord = CardRecord::new(
    cards::YOUNG_PYROMANCER,
    "Young Pyromancer",
    CardArt::new("e349c204-3a93-4bf7-b79a-5f5f261ea2d3", "Cynthia Sheppard"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Shaman"], 2, 1).with_ability(
        AbilityDef::triggered(
            "Whenever you cast an instant or sorcery spell, create a 1/1 red Elemental creature token.",
            TriggerEventDef::SpellCast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
            ])),
            EffectDef::CreateToken {
                token: cards::ELEMENTAL_TOKEN_1_1_RED,
                controller: None,
                count: ValueDef::Constant(1),
                tapped: false,
                attacking: false,
            counters: None,
            created: None,},
        ),
    ),
);

// M14 164 — Advocate of the Beast
pub(in crate::card::sets) static ADVOCATE_OF_THE_BEAST: CardRecord = CardRecord::new(
    cards::ADVOCATE_OF_THE_BEAST,
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

// M14 167 — Brindle Boar
pub(in crate::card::sets) static BRINDLE_BOAR: CardRecord = CardRecord::new(
    cards::BRINDLE_BOAR,
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

// M14 169 — Elvish Mystic
pub(in crate::card::sets) static ELVISH_MYSTIC: CardRecord = CardRecord::new(
    cards::ELVISH_MYSTIC,
    "Elvish Mystic",
    CardArt::new("60d0e6a6-629a-45a7-bfcb-25ba7156788b", "Wesley Burt"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Druid"], 1, 1)
        .with_abilities(&[abilities::tap_for(ManaColor::Green)]),
);

/// Trample and the requirement work together: something has to block it, and
/// blocking barely slows seven extra power down.
static ENLARGE_GROWTH: [AppliedEffectDef; 3] = [
    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(7), ValueDef::Constant(7)),
    AppliedEffectDef::add_ability(&abilities::trample()),
    AppliedEffectDef::Rule(AppliedRuleDef::MustBeBlockedBy(
        ObjectPredicateDef::HasType(CardType::Creature),
    )),
];

// M14 170 — Enlarge
pub(in crate::card::sets) static ENLARGE: CardRecord = CardRecord::new(
    cards::ENLARGE,
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
            effect: AppliedEffectDef::Composite(&ENLARGE_GROWTH),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M14 172 — Garruk, Caller of Beasts
// Audit: blocked — The planeswalker modes need filtered multi-card top selection, a hand-to-battlefield choice, and an emblem carrying an optional creature tutor trigger.

// M14 173 — Garruk's Horde
// Audit: blocked — The engine cannot reveal the top library card continuously or offer creature spells there as castable play options.

// M14 176 — Gladecover Scout
pub(in crate::card::sets) static GLADECOVER_SCOUT: CardRecord = CardRecord::new(
    cards::GLADECOVER_SCOUT,
    "Gladecover Scout",
    CardArt::new("e112d77d-f019-4709-b31a-b02952df0e35", "Allen Williams"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Scout"], 1, 1)
        .with_abilities(&[abilities::hexproof()]),
);

// M14 177 — Groundshaker Sliver
pub(in crate::card::sets) static GROUNDSHAKER_SLIVER: CardRecord = CardRecord::new(
    cards::GROUNDSHAKER_SLIVER,
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

static M14_FORESTS_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

// M14 178 — Howl of the Night Pack
pub(in crate::card::sets) static HOWL_OF_THE_NIGHT_PACK: CardRecord = CardRecord::new(
    cards::HOWL_OF_THE_NIGHT_PACK,
    "Howl of the Night Pack",
    CardArt::new("20fc5ff1-b8bd-44d5-a659-17eeae06736a", "Lars Grant-West"),
    CardSet::Magic2014,
    CardRules::new_sorcery(mana_cost!("{6}{G}")).with_ability(AbilityDef::spell(
        "Create a 2/2 green Wolf creature token for each Forest you control.",
        EffectDef::CreateToken {
            token: cards::WOLF_TOKEN_2_2_GREEN,
            controller: None,
            count: ValueDef::CountMatchingObjects(&M14_FORESTS_YOU_CONTROL),
            tapped: false,
            attacking: false,
            counters: None,
            created: None,
        },
    )),
);

// M14 179 — Hunt the Weak
// Audit: blocked — DealDamage can only attribute damage to the resolving spell, not to each fighting creature.

/// Only a land may be taken, and taking it is optional -- a minimum of zero
/// is the "you may". Whatever is not taken goes back on top rather than
/// anywhere else, so a nonland card is still the next draw.
static INTO_THE_WILDS_LOOK: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::Constant(1),
    object: Some(ObjectPredicateDef::HasType(CardType::Land)),
    minimum: 0,
    maximum: 1,
    select_all_matching: false,
    reveal_selected: false,
    selected_zone: ZoneKind::Battlefield,
    selected_placement: ZonePlacement::Top,
    rest_zone: ZoneKind::Library,
    rest_placement: ZonePlacement::Top,
    selected_order_follows_choice: false,
    then: None,
    selected_face_down: false,
};

// M14 180 — Into the Wilds
pub(in crate::card::sets) static INTO_THE_WILDS: CardRecord = CardRecord::new(
    cards::INTO_THE_WILDS,
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
        EffectDef::LookAtTopAndSelect {
            player: EffectRecipientDef::Controller,
            looker: EffectRecipientDef::Controller,
            selection: &INTO_THE_WILDS_LOOK,
        },
    )),
);

// M14 181 — Kalonian Hydra
// Audit: blocked — Counter effects cannot double each creature's existing +1/+1 counter count.

// M14 182 — Kalonian Tusker
pub(in crate::card::sets) static KALONIAN_TUSKER: CardRecord = CardRecord::new(
    cards::KALONIAN_TUSKER,
    "Kalonian Tusker",
    CardArt::new("135946fc-fe67-401f-821d-d7145c63f030", "Svetlin Velinov"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{G}{G}"), &["Beast"], 3, 3),
);

// M14 183 — Lay of the Land
pub(in crate::card::sets) static LAY_OF_THE_LAND: CardRecord = CardRecord::new(
    cards::LAY_OF_THE_LAND,
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
            binding: None,
            then: None,
        },
    )),
);

static MANAWEFT_SLIVER_GRANTED_ABILITY: AbilityDef = AbilityDef::activated_mana(
    "{T}: Add one mana of any color.",
    &[AbilityCostDef::TapSource],
    EffectDef::AddMana(AddManaEffectDef::any_color()),
);

// M14 184 — Manaweft Sliver
pub(in crate::card::sets) static MANAWEFT_SLIVER: CardRecord = CardRecord::new(
    cards::MANAWEFT_SLIVER,
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
                effect: AppliedEffectDef::add_ability(&MANAWEFT_SLIVER_GRANTED_ABILITY),
            },
        ),
    ),
);

// M14 185 — Megantic Sliver
pub(in crate::card::sets) static MEGANTIC_SLIVER: CardRecord = CardRecord::new(
    cards::MEGANTIC_SLIVER,
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

// M14 187 — Oath of the Ancient Wood
pub(in crate::card::sets) static OATH_OF_THE_ANCIENT_WOOD: CardRecord = CardRecord::new(
    cards::OATH_OF_THE_ANCIENT_WOOD,
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

// M14 189 — Predatory Sliver
pub(in crate::card::sets) static PREDATORY_SLIVER: CardRecord = CardRecord::new(
    cards::PREDATORY_SLIVER,
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
pub(in crate::card::sets) static PRIMEVAL_BOUNTY: CardRecord = CardRecord::new(
    cards::PRIMEVAL_BOUNTY,
    "Primeval Bounty",
    CardArt::new("e750d55d-d5e8-4abe-99cf-f6b8ba86cf16", "Christine Choi"),
    CardSet::Magic2014,
    CardRules::new_enchantment(mana_cost!("{5}{G}")).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you cast a creature spell, create a 3/3 green Beast creature token.",
            TriggerEventDef::SpellCast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::CreateToken {
                token: cards::BEAST_TOKEN_3_3_GREEN,
                controller: None,
                count: ValueDef::Constant(1),
                tapped: false,
                attacking: false,
            counters: None,
            created: None,},
        ),
        AbilityDef::triggered_with_targets("Whenever you cast a noncreature spell, put three +1/+1 counters on target creature you control.", TriggerEventDef::SpellCast(ObjectPredicateDef::All(&[
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

// M14 192 — Rootwalla
pub(in crate::card::sets) static ROOTWALLA: CardRecord = CardRecord::new(
    cards::ROOTWALLA,
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
pub(in crate::card::sets) static RUMBLING_BALOTH: CardRecord = CardRecord::new(
    cards::RUMBLING_BALOTH,
    "Rumbling Baloth",
    CardArt::new("d8610ff1-064b-4c75-a8df-d3b076370d1e", "Jesper Ejsing"),
    CardSet::Magic2014,
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Beast"], 4, 4),
);

// M14 194 — Savage Summoning
// Audit: blocked — No continuation can tag the next creature spell with flash, uncounterability, and an entry counter while also making this spell uncounterable.

/// One when the exiled card was a creature, nothing otherwise.
static EXILED_A_CREATURE: TargetConditionDef = TargetConditionDef {
    slot: TargetIndex::PRIMARY,
    object: ObjectPredicateDef::HasType(CardType::Creature),
    then: ValueDef::Constant(1),
    otherwise: ValueDef::Constant(0),
};

// M14 195 — Scavenging Ooze
pub(in crate::card::sets) static SCAVENGING_OOZE: CardRecord = CardRecord::new(
    cards::SCAVENGING_OOZE,
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
                    controller: None,
                    placement: ZonePlacement::Top,
                    arrival_effect: None,
                    attachment: None,
                },
            ])),
    ),
);

// M14 196 — Sporemound
pub(in crate::card::sets) static SPOREMOUND: CardRecord = CardRecord::new(
    cards::SPOREMOUND,
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
            EffectDef::CreateToken {
                token: cards::SAPROLING_TOKEN_1_1_GREEN,
                controller: None,
                count: ValueDef::Constant(1),
                tapped: false,
                attacking: false,
            counters: None,
            created: None,},
        ),
    ),
);

// M14 197 — Trollhide
pub(in crate::card::sets) static TROLLHIDE: CardRecord = CardRecord::new(
    cards::TROLLHIDE,
    "Trollhide",
    CardArt::new("08b9c400-dc8f-4fe6-a868-fdf0d247086a", "Steven Belledin"),
    CardSet::Magic2014,
    CardRules::new_enchantment(mana_cost!("{2}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+2 and has \"{1}{G}: Regenerate this creature.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&TROLLHIDE_GRANT),
                },
            ),
        ]),
);

static TROLLHIDE_GRANT: [AppliedEffectDef; 2] = [
    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
    AppliedEffectDef::add_ability(&TROLLHIDE_REGENERATION),
];

static TROLLHIDE_REGENERATION: AbilityDef = abilities::regenerate_self(
    "{1}{G}: Regenerate this creature.",
    &[AbilityCostDef::Mana(mana_cost!("{1}{G}"))],
);

// M14 198 — Vastwood Hydra
// Audit: blocked — Entry replacements cannot add chosen-X counters, and counter distribution cannot read the dead source's last-known counter count.

// M14 200 — Voracious Wurm
// Audit: blocked — Entry replacements cannot derive a counter amount from life gained during the turn.

// M14 201 — Windstorm
pub(in crate::card::sets) static WINDSTORM: CardRecord = CardRecord::new(
    cards::WINDSTORM,
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
// Audit: blocked — A non-intervening spell-cast trigger cannot capture “during your turn” without incorrectly rechecking that restriction on resolution.

static WOODBORN_BEHEMOTH_TRAMPLE: AbilityDef = abilities::trample();

static WOODBORN_BEHEMOTH_BONUS: [AppliedEffectDef; 2] = [
    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(4), ValueDef::Constant(4)),
    AppliedEffectDef::add_ability(&WOODBORN_BEHEMOTH_TRAMPLE),
];

static EIGHT_LANDS: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef::matching(
        ObjectPredicateDef::HasType(CardType::Land),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    ),
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 8,
};

static WOODBORN_BEHEMOTH_GRANT: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::Source,
    effect: AppliedEffectDef::Composite(&WOODBORN_BEHEMOTH_BONUS),
};

// M14 203 — Woodborn Behemoth
pub(in crate::card::sets) static WOODBORN_BEHEMOTH: CardRecord = CardRecord::new(
    cards::WOODBORN_BEHEMOTH,
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
                condition: &EIGHT_LANDS,
                then: &WOODBORN_BEHEMOTH_GRANT,
            },
        ),
    ),
);

static ACCORDERS_SHIELD_VIGILANCE: AbilityDef = abilities::vigilance();

static ACCORDERS_SHIELD_BONUS: [AppliedEffectDef; 2] = [
    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(0), ValueDef::Constant(3)),
    AppliedEffectDef::add_ability(&ACCORDERS_SHIELD_VIGILANCE),
];

// M14 204 — Accorder's Shield
pub(in crate::card::sets) static ACCORDERS_SHIELD: CardRecord = CardRecord::new(
    cards::ACCORDERS_SHIELD,
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
                    effect: AppliedEffectDef::Composite(&ACCORDERS_SHIELD_BONUS),
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
// Audit: blocked — Costs cannot select an exact named sacrifice, and effects cannot total life actually lost by all opponents for the linked gain.

// M14 206 — Darksteel Forge
pub(in crate::card::sets) static DARKSTEEL_FORGE: CardRecord = CardRecord::new(
    cards::DARKSTEEL_FORGE,
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

// M14 208 — Door of Destinies
// Audit: blocked — Predicates cannot consume a stored creature-type choice for both spell triggers and a counter-scaled continuous bonus.

static FIRESHRIEKER_DOUBLE_STRIKE: AbilityDef = abilities::double_strike();

// M14 210 — Fireshrieker
pub(in crate::card::sets) static FIRESHRIEKER: CardRecord = CardRecord::new(
    cards::FIRESHRIEKER,
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
                    effect: AppliedEffectDef::add_ability(&FIRESHRIEKER_DOUBLE_STRIKE),
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
// Audit: blocked — Attack events cannot match attacks at you or your planeswalker, and abilities cannot permanently remove defender from the source after that trigger.

static HAUNTED_PLATE_MAIL_NO_CREATURES: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef::matching(
        ObjectPredicateDef::HasType(CardType::Creature),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    ),
    comparison: ComparisonDef::Equal,
    amount: 0,
};

static HAUNTED_PLATE_MAIL_ANIMATION: [AppliedEffectDef; 4] = [
    AppliedEffectDef::set_card_types(
        CardTypeSet::single(CardType::Artifact).with(CardType::Creature),
    ),
    AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Spirit"])),
    AppliedEffectDef::remove_subtypes(&["Equipment"]),
    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(4), ValueDef::Constant(4)),
];

// M14 212 — Haunted Plate Mail
pub(in crate::card::sets) static HAUNTED_PLATE_MAIL: CardRecord = CardRecord::new(
    cards::HAUNTED_PLATE_MAIL,
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
                    effect: AppliedEffectDef::Composite(&HAUNTED_PLATE_MAIL_ANIMATION),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            )
            .with_activation_condition(&HAUNTED_PLATE_MAIL_NO_CREATURES),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{4}"))],
                "Equip {4} ({4}: Attach to target creature you control. Equip only as a sorcery.)",
            ),
        ]),
);

// M14 214 — Pyromancer's Gauntlet
// Audit: blocked — Damage replacement cannot filter red instant, sorcery, or planeswalker sources and add a fixed amount to the event.

// M14 215 — Ratchet Bomb
pub(in crate::card::sets) static RATCHET_BOMB: CardRecord = CardRecord::new(
    cards::RATCHET_BOMB,
    "Ratchet Bomb",
    CardArt::new("3e9045df-3eff-4236-9bbb-77537b302e27", "Austin Hsu"),
    CardSet::Magic2014,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
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
            "{T}, Sacrifice this artifact: Destroy each nonland permanent with mana value equal to the number of charge counters on this artifact.",
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
            EffectDef::Destroy {
                object: EffectRecipientDef::matching_objects(ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        // The Bomb is already gone by the time this resolves,
                        // so the count comes from last-known information.
                        ObjectPredicateDef::ManaValueEqualTo(ValueDef::CountersOnSource(
                            CounterKind::Charge,
                        )),
                    ]), &[ZoneKind::Battlefield], PlayerRelation::Any),
                can_regenerate: true,
            },
        ),
    ]),
);

// M14 216 — Ring of Three Wishes
// Audit: blocked — CounterKind has no wish counter, so the entry counters and removal cost cannot share the printed counter identity.

// M14 218 — Sliver Construct
pub(in crate::card::sets) static SLIVER_CONSTRUCT: CardRecord = CardRecord::new(
    cards::SLIVER_CONSTRUCT,
    "Sliver Construct",
    CardArt::new("3129645a-221c-4eb5-88fd-12cc742a1dfe", "Mathias Kollros"),
    CardSet::Magic2014,
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Sliver", "Construct"], 2, 2),
);

// M14 219 — Staff of the Death Magus
pub(in crate::card::sets) static STAFF_OF_THE_DEATH_MAGUS: CardRecord = CardRecord::new(
    cards::STAFF_OF_THE_DEATH_MAGUS,
    "Staff of the Death Magus",
    CardArt::new("624fe171-8bd8-4156-b40e-74e2a847d380", "Daniel Ljunggren"),
    CardSet::Magic2014,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you cast a black spell, you gain 1 life.",
            TriggerEventDef::SpellCast(ObjectPredicateDef::All(&[
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
pub(in crate::card::sets) static STAFF_OF_THE_FLAME_MAGUS: CardRecord = CardRecord::new(
    cards::STAFF_OF_THE_FLAME_MAGUS,
    "Staff of the Flame Magus",
    CardArt::new("6d6befbd-4fe3-4338-b8ea-13b8b70a7664", "Daniel Ljunggren"),
    CardSet::Magic2014,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you cast a red spell, you gain 1 life.",
            TriggerEventDef::SpellCast(ObjectPredicateDef::All(&[
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
pub(in crate::card::sets) static STAFF_OF_THE_MIND_MAGUS: CardRecord = CardRecord::new(
    cards::STAFF_OF_THE_MIND_MAGUS,
    "Staff of the Mind Magus",
    CardArt::new("f86bf36b-b83f-4451-8cdc-2a4ccffb93c7", "Daniel Ljunggren"),
    CardSet::Magic2014,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you cast a blue spell, you gain 1 life.",
            TriggerEventDef::SpellCast(ObjectPredicateDef::All(&[
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
pub(in crate::card::sets) static STAFF_OF_THE_SUN_MAGUS: CardRecord = CardRecord::new(
    cards::STAFF_OF_THE_SUN_MAGUS,
    "Staff of the Sun Magus",
    CardArt::new("90a1f830-d19a-4ebf-9573-09b677693dd6", "Daniel Ljunggren"),
    CardSet::Magic2014,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you cast a white spell, you gain 1 life.",
            TriggerEventDef::SpellCast(ObjectPredicateDef::All(&[
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
pub(in crate::card::sets) static STAFF_OF_THE_WILD_MAGUS: CardRecord = CardRecord::new(
    cards::STAFF_OF_THE_WILD_MAGUS,
    "Staff of the Wild Magus",
    CardArt::new("d207f03d-4c7b-444f-bf95-e63f7004d525", "Daniel Ljunggren"),
    CardSet::Magic2014,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you cast a green spell, you gain 1 life.",
            TriggerEventDef::SpellCast(ObjectPredicateDef::All(&[
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
// Audit: blocked — Triggered abilities are not targetable stack objects and cannot be copied or retargeted declaratively.

// M14 226 — Vial of Poison
pub(in crate::card::sets) static VIAL_OF_POISON: CardRecord = CardRecord::new(
    cards::VIAL_OF_POISON,
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
pub(in crate::card::sets) static ENCROACHING_WASTES: CardRecord = CardRecord::new(
    cards::ENCROACHING_WASTES,
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
            },
        ),
    ]),
);

/// The animation keeps the land types Mutavault is printed with, so the
/// creature types are added rather than replacing anything.
static MUTAVAULT_ANIMATION: [AppliedEffectDef; 3] = [
    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::ALL),
    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
];

// M14 228 — Mutavault
pub(in crate::card::sets) static MUTAVAULT: CardRecord = CardRecord::new(
    cards::MUTAVAULT,
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
                effect: AppliedEffectDef::Composite(&MUTAVAULT_ANIMATION),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ARCHANGEL_OF_THUNE,
    &AURAMANCER,
    &BONESCYTHE_SLIVER,
    &BRAVE_THE_ELEMENTS,
    &CAPASHEN_KNIGHT,
    &CELESTIAL_FLARE,
    &CHARGING_GRIFFIN,
    &CONGREGATE,
    &DAWNSTRIKE_PALADIN,
    &FORTIFY,
    &GRIFFIN_SENTINEL,
    &HIVE_STIRRINGS,
    &IMPOSING_SOVEREIGN,
    &INDESTRUCTIBILITY,
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
    &CORAL_MERFOLK,
    &DISPERSE,
    &FROST_BREATH,
    &GALERIDER_SLIVER,
    &GLIMPSE_THE_FUTURE,
    &MESSENGER_DRAKE,
    &OPPORTUNITY,
    &PHANTOM_WARRIOR,
    &QUICKEN,
    &SEACOAST_DRAKE,
    &TIME_EBB,
    &TOME_SCOUR,
    &TRAINED_CONDOR,
    &TRAUMATIZE,
    &WALL_OF_FROST,
    &WATER_SERVANT,
    &WINDREADER_SPHINX,
    &ZEPHYR_CHARGE,
    &ACCURSED_SPIRIT,
    &BLIGHTCASTER,
    &BLOOD_BAIRN,
    &CHILD_OF_NIGHT,
    &CORPSE_HAULER,
    &DARK_PROPHECY,
    &DEATHGAZE_COCKATRICE,
    &DIABOLIC_TUTOR,
    &DOOM_BLADE,
    &GNAWING_ZOMBIE,
    &LIFEBANE_ZOMBIE,
    &LILIANAS_REAVER,
    &LITURGY_OF_BLOOD,
    &MINOTAUR_ABOMINATION,
    &NIGHTWING_SHADE,
    &QUAG_SICKNESS,
    &SANGUINE_BOND,
    &SHADOWBORN_DEMON,
    &SHRIVEL,
    &SYPHON_SLIVER,
    &UNDEAD_MINOTAUR,
    &VAMPIRE_WARLORD,
    &WRING_FLESH,
    &XATHRID_NECROMANCER,
    &AWAKEN_THE_ANCIENT,
    &BARRAGE_OF_EXPENDABLES,
    &BATTLE_SLIVER,
    &BLUR_SLIVER,
    &BURNING_EARTH,
    &CHANDRAS_OUTRAGE,
    &DRAGON_EGG,
    &FLESHPULPER_GIANT,
    &GOBLIN_DIPLOMATS,
    &GOBLIN_SHORTCUTTER,
    &LAVA_AXE,
    &LIGHTNING_TALONS,
    &MINDSPARKER,
    &OGRE_BATTLEDRIVER,
    &REGATHAN_FIRECAT,
    &SCOURGE_OF_VALKAS,
    &SHOCK,
    &STRIKING_SLIVER,
    &THORNCASTER_SLIVER,
    &THUNDER_STRIKE,
    &YOUNG_PYROMANCER,
    &ADVOCATE_OF_THE_BEAST,
    &BRINDLE_BOAR,
    &ELVISH_MYSTIC,
    &ENLARGE,
    &GLADECOVER_SCOUT,
    &GROUNDSHAKER_SLIVER,
    &HOWL_OF_THE_NIGHT_PACK,
    &INTO_THE_WILDS,
    &KALONIAN_TUSKER,
    &LAY_OF_THE_LAND,
    &MANAWEFT_SLIVER,
    &MEGANTIC_SLIVER,
    &OATH_OF_THE_ANCIENT_WOOD,
    &PREDATORY_SLIVER,
    &PRIMEVAL_BOUNTY,
    &ROOTWALLA,
    &RUMBLING_BALOTH,
    &SCAVENGING_OOZE,
    &SPOREMOUND,
    &TROLLHIDE,
    &WINDSTORM,
    &WOODBORN_BEHEMOTH,
    &ACCORDERS_SHIELD,
    &DARKSTEEL_FORGE,
    &FIRESHRIEKER,
    &HAUNTED_PLATE_MAIL,
    &RATCHET_BOMB,
    &SLIVER_CONSTRUCT,
    &STAFF_OF_THE_DEATH_MAGUS,
    &STAFF_OF_THE_FLAME_MAGUS,
    &STAFF_OF_THE_MIND_MAGUS,
    &STAFF_OF_THE_SUN_MAGUS,
    &STAFF_OF_THE_WILD_MAGUS,
    &VIAL_OF_POISON,
    &ENCROACHING_WASTES,
    &MUTAVAULT,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&avacyn_restored::ANGELIC_WALL), // M14 4
    PrintingRecord::reprint(&alpha::BLESSING),               // M14 8
    PrintingRecord::reprint(&magic_2013::DIVINE_FAVOR),      // M14 17
    PrintingRecord::reprint(&magic_2013::PILLARFIELD_OX),    // M14 28
    PrintingRecord::reprint(&magic_2013::PLANAR_CLEANSING),  // M14 29
    PrintingRecord::reprint(&alpha::SERRA_ANGEL),            // M14 32
    PrintingRecord::reprint(&magic_2013::SHOW_OF_VALOR),     // M14 33
    PrintingRecord::reprint(&alpha::WALL_OF_SWORDS),         // M14 41
    PrintingRecord::reprint(&magic_2013::ARCHAEOMANCER),     // M14 43
    PrintingRecord::reprint(&return_to_ravnica::CANCEL),     // M14 45
    PrintingRecord::reprint(&innistrad::CLAUSTROPHOBIA),     // M14 46
    PrintingRecord::reprint(&alpha::CLONE),                  // M14 47
    PrintingRecord::reprint(&dark_ascension::DIVINATION),    // M14 52
    PrintingRecord::reprint(&magic_2013::ESSENCE_SCATTER),   // M14 55
    PrintingRecord::reprint(&magic_2013::JACE_MEMORY_ADEPT), // M14 60
    PrintingRecord::reprint(&magic_2013::NEGATE),            // M14 64
    PrintingRecord::reprint(&dark_ascension::NEPHALIA_SEAKITE), // M14 65
    PrintingRecord::reprint(&magic_2013::SCROLL_THIEF),      // M14 69
    PrintingRecord::reprint(&innistrad::SENSORY_DEPRIVATION), // M14 71
    PrintingRecord::reprint(&alpha::SPELL_BLAST),            // M14 72
    PrintingRecord::reprint(&magic_2013::DARK_FAVOR),        // M14 92
    PrintingRecord::reprint(&magic_2013::DURESS),            // M14 97
    PrintingRecord::reprint(&magic_2013::MARK_OF_THE_VAMPIRE), // M14 105
    PrintingRecord::reprint(&magic_2013::MIND_ROT),          // M14 106
    PrintingRecord::reprint(&alpha::SENGIR_VAMPIRE),         // M14 113
    PrintingRecord::reprint(&magic_2013::VILE_REBIRTH),      // M14 121
    PrintingRecord::reprint(&gatecrash::ACT_OF_TREASON),     // M14 125
    PrintingRecord::reprint(&magic_2013::CANYON_MINOTAUR),   // M14 131
    PrintingRecord::reprint(&avacyn_restored::DEMOLISH),     // M14 136
    PrintingRecord::reprint(&magic_2013::DRAGON_HATCHLING),  // M14 138
    PrintingRecord::reprint(&magic_2013::FLAMES_OF_THE_FIREBRAND), // M14 139
    PrintingRecord::reprint(&innistrad::PITCHBURN_DEVILS),   // M14 149
    PrintingRecord::reprint(&alpha::SHIVAN_DRAGON),          // M14 154
    PrintingRecord::reprint(&magic_2013::SMELT),             // M14 156
    PrintingRecord::reprint(&magic_2013::VOLCANIC_GEYSER),   // M14 160
    PrintingRecord::reprint(&innistrad::BRAMBLECRUSH),       // M14 165
    PrintingRecord::reprint(&dark_ascension::BRIARPACK_ALPHA), // M14 166
    PrintingRecord::reprint(&magic_2013::DEADLY_RECLUSE),    // M14 168
    PrintingRecord::reprint(&alpha::GIANT_GROWTH),           // M14 174
    PrintingRecord::reprint(&alpha::GIANT_SPIDER),           // M14 175
    PrintingRecord::reprint(&onslaught::NATURALIZE),         // M14 186
    PrintingRecord::reprint(&magic_2013::PLUMMET),           // M14 188
    PrintingRecord::reprint(&innistrad::RANGERS_GUILE),      // M14 191
    PrintingRecord::reprint(&darksteel::DARKSTEEL_INGOT),    // M14 207
    PrintingRecord::reprint(&antiquities::MILLSTONE),        // M14 213
    PrintingRecord::reprint(&alpha::ROD_OF_RUIN),            // M14 217
    PrintingRecord::reprint(&alpha::PLAINS),                 // M14 230
    PrintingRecord::alternate(&alpha::PLAINS, 1),            // M14 231
    PrintingRecord::alternate(&alpha::PLAINS, 2),            // M14 232
    PrintingRecord::alternate(&alpha::PLAINS, 3),            // M14 233
    PrintingRecord::reprint(&alpha::ISLAND),                 // M14 234
    PrintingRecord::alternate(&alpha::ISLAND, 1),            // M14 235
    PrintingRecord::alternate(&alpha::ISLAND, 2),            // M14 236
    PrintingRecord::alternate(&alpha::ISLAND, 3),            // M14 237
    PrintingRecord::reprint(&alpha::SWAMP),                  // M14 238
    PrintingRecord::alternate(&alpha::SWAMP, 1),             // M14 239
    PrintingRecord::alternate(&alpha::SWAMP, 2),             // M14 240
    PrintingRecord::alternate(&alpha::SWAMP, 3),             // M14 241
    PrintingRecord::reprint(&alpha::MOUNTAIN),               // M14 242
    PrintingRecord::alternate(&alpha::MOUNTAIN, 1),          // M14 243
    PrintingRecord::alternate(&alpha::MOUNTAIN, 2),          // M14 244
    PrintingRecord::alternate(&alpha::MOUNTAIN, 3),          // M14 245
    PrintingRecord::reprint(&alpha::FOREST),                 // M14 246
    PrintingRecord::alternate(&alpha::FOREST, 1),            // M14 247
    PrintingRecord::alternate(&alpha::FOREST, 2),            // M14 248
    PrintingRecord::alternate(&alpha::FOREST, 3),            // M14 249
];
