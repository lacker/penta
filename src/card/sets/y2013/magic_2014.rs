//! Magic 2014 card records used by the built-in ISD–M14 Standard decks.

use super::{CardRecord, PrintingRecord};
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
    y2012::{dark_ascension, magic_2013},
};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, BasicLandType, BattlefieldEntryModificationDef, CardArt,
    CardRules, CardSet, CardSupertype, CardType, CardTypeSet, ChoiceVisibilityDef, ChooseDef,
    ClassifyObjectsDef, ColorSet, ComparisonDef, CounterKind, CreatureTypeSetDef,
    DamageEventMatcherDef, DiscardSelectionDef, EffectDef, EffectRecipientDef, ManaColor,
    MoveObjectsDef, ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef, ObjectSetDef,
    PlayerRefDef, PlayerRelation, ReplacementEffectDef, ReplacementEventDef,
    ResolvedEffectDurationDef, SacrificedAmountDef, TriggerConditionDef, TriggerEventDef,
    TurnStepDef, ValueDef, ZoneKind, ZonePlacement, abilities, tokens,
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
const AJANI_CALLER_OF_THE_PRIDE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m13::AJANI_CALLER_OF_THE_PRIDE,
    "33e44e04-8330-49ca-906c-bb9bf0bc84ce",
    "D. Alexander Gregory",
);

// M14 2 — Ajani's Chosen
// Audit: unsupported — CreateToken cannot continue by conditionally attaching the triggering Aura to the newly created token.
pub(in crate::card::sets) static AJANI_S_CHOSEN: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Ajani's Chosen",
    "583bfbc1-638b-4de5-b865-0b00a69dd073",
    "Wayne Reynolds",
    crate::card::CardRules::unsupported(),
);

// M14 3 — Angelic Accord
// Audit: unsupported — Trigger conditions cannot track life gained during the turn or compare that total with four.
pub(in crate::card::sets) static ANGELIC_ACCORD: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Angelic Accord",
    "03f26bc2-53d7-4448-8021-de35aa82fcc6",
    "Michael C. Hayes",
    crate::card::CardRules::unsupported(),
);

// M14 4 — Angelic Wall (reprint)
const ANGELIC_WALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1998::portal_second_age::ANGELIC_WALL,
    "7fe8f88e-8a51-494b-a008-fbfe624f97f7",
    "Allen Williams",
);

// M14 5 — Archangel of Thune
pub(in crate::card::sets) static ARCHANGEL_OF_THUNE: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Archangel of Thune",
    "531cba81-afd7-4be4-adec-87edb77ba2a9",
    "James Ryman",
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

// M14 6 — Auramancer (reprint)
const AURAMANCER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2001::odyssey::AURAMANCER,
    "0a3dc4ab-1c45-4495-91b6-27d62087380c",
    "Rebecca Guay",
);

// M14 7 — Banisher Priest
// Audit: unsupported — Linked exile cannot express Banisher Priest's one-shot duration, which must do nothing if the source left before the enter trigger resolved.
pub(in crate::card::sets) static BANISHER_PRIEST: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Banisher Priest",
    "06823bf8-2fca-49e1-ba40-9b61c9ae55b3",
    "Willian Murai",
    crate::card::CardRules::unsupported(),
);

// M14 8 — Blessing (reprint)
const BLESSING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BLESSING,
    "4273cf50-db65-4b1f-95e1-f24ba6582c8b",
    "Jason A. Engle",
);

// M14 9 — Bonescythe Sliver
pub(in crate::card::sets) static BONESCYTHE_SLIVER: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Bonescythe Sliver",
    "a26bb68b-1830-470a-8cea-91edc7db0c57",
    "Trevor Claxton",
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

// M14 10 — Brave the Elements (reprint)
const BRAVE_THE_ELEMENTS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::zendikar::BRAVE_THE_ELEMENTS,
    "097d7838-ae58-4306-ba0f-e914601b31b6",
    "Goran Josic",
);

// M14 11 — Capashen Knight (reprint)
const CAPASHEN_KNIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1999::urzas_destiny::CAPASHEN_KNIGHT,
    "78802af4-46b5-4bac-8cdf-5b77d0b19895",
    "Jasper Sandner",
);

// M14 12 — Celestial Flare
pub(in crate::card::sets) static CELESTIAL_FLARE: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Celestial Flare",
    "6c8d1320-0f1a-4c66-86c9-9f8da0f1d9ef",
    "Clint Cearley",
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
pub(in crate::card::sets) static CHARGING_GRIFFIN: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Charging Griffin",
    "88637cc0-3b2a-402c-b491-26fcc2d21fb8",
    "Erica Yang",
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

// M14 14 — Congregate (reprint)
const CONGREGATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1998::urzas_saga::CONGREGATE,
    "b792574a-4d8f-4c80-a958-7c0edbe391fc",
    "Mark Zug",
);

// M14 15 — Dawnstrike Paladin
pub(in crate::card::sets) static DAWNSTRIKE_PALADIN: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Dawnstrike Paladin",
    "93cf5fb3-bb41-4efa-9721-2c2d169b05cd",
    "Tyler Jacobson",
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Human", "Knight"], 2, 4)
        .with_abilities(&[abilities::vigilance(), abilities::lifelink()]),
);

// M14 16 — Devout Invocation
// Audit: unsupported — Spell costs cannot tap a freely chosen number of creatures and carry that paid count into token creation.
pub(in crate::card::sets) static DEVOUT_INVOCATION: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Devout Invocation",
    "8a286954-fb40-4440-9f0e-a28367c6823c",
    "David Palumbo",
    crate::card::CardRules::unsupported(),
);

// M14 17 — Divine Favor (reprint)
const DIVINE_FAVOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2011::magic_2012::DIVINE_FAVOR,
    "e3c19a11-ff0f-4dbd-b872-30276b8ecf22",
    "Allen Williams",
);

// M14 18 — Fiendslayer Paladin
// Audit: unsupported — Targeting restrictions cannot filter opposing spell sources by black or red color while leaving abilities unaffected.
pub(in crate::card::sets) static FIENDSLAYER_PALADIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Fiendslayer Paladin",
    "5cfb0f4a-e273-4ffb-91cd-dd1a7b6f6a8f",
    "Wesley Burt",
    crate::card::CardRules::unsupported(),
);

// M14 19 — Fortify (reprint)
const FORTIFY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2006::time_spiral::FORTIFY,
    "1eff4028-d4f9-4822-81d6-9f5e5e6f3011",
    "Christopher Moeller",
);

// M14 20 — Griffin Sentinel (reprint)
const GRIFFIN_SENTINEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::GRIFFIN_SENTINEL,
    "b40d6626-a85f-4116-9721-19e39b83cba0",
    "Warren Mahy",
);

// M14 21 — Hive Stirrings
pub(in crate::card::sets) static HIVE_STIRRINGS: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Hive Stirrings",
    "e4399e19-d05d-4bb3-9aff-c4133ddd2850",
    "Maciej Kuciara",
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
pub(in crate::card::sets) static IMPOSING_SOVEREIGN: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Imposing Sovereign",
    "0f672328-3361-498e-a9f4-2d8e69a8b072",
    "Scott M. Fischer",
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

// M14 23 — Indestructibility (reprint)
const INDESTRUCTIBILITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::INDESTRUCTIBILITY,
    "e086a062-d39b-4e2a-bde0-f4d6d1797a5f",
    "Darrell Riche",
);

// M14 24 — Master of Diversion
// Audit: unsupported — Attack triggers cannot restrict a target to the creature controlled by that attack's defending player.
pub(in crate::card::sets) static MASTER_OF_DIVERSION: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Master of Diversion",
    "2bec89b3-640e-4093-a6e9-5639610769b9",
    "Michael Komarck",
    crate::card::CardRules::unsupported(),
);

// M14 25 — Pacifism (reprint)
const PACIFISM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1996::mirage::PACIFISM,
    "87c7ff88-c23e-4be6-989b-99d055db36df",
    "Robert Bliss",
);

// M14 26 — Path of Bravery
// Audit: unsupported — Continuous effects cannot compare current and starting life, and attack events cannot aggregate one combat's attackers into one trigger amount.
pub(in crate::card::sets) static PATH_OF_BRAVERY: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Path of Bravery",
    "5222e200-df1b-46c6-a194-c341e8c1d516",
    "Chris Rahn",
    crate::card::CardRules::unsupported(),
);

// M14 27 — Pay No Heed (reprint)
const PAY_NO_HEED_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tor::PAY_NO_HEED,
    "3bb836f0-527d-445b-877e-7158a4579c33",
    "Adam Rex",
);

// M14 28 — Pillarfield Ox (reprint)
const PILLARFIELD_OX_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::zendikar::PILLARFIELD_OX,
    "f79d3bba-18b0-4c56-a90b-8e28935a6a7a",
    "Andrew Robinson",
);

// M14 29 — Planar Cleansing (reprint)
const PLANAR_CLEANSING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::PLANAR_CLEANSING,
    "99873316-4872-4500-a225-cf483e1ebaa9",
    "Michael Komarck",
);

// M14 30 — Sentinel Sliver
pub(in crate::card::sets) static SENTINEL_SLIVER: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Sentinel Sliver",
    "74c28560-e6ac-4be9-a253-22c4613b0d90",
    "Maciej Kuciara",
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
    CardSet::Magic2014,
    "Seraph of the Sword",
    "d9789cac-5774-4f72-82c3-18f11f9d4a62",
    "Jaime Jones",
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
const SERRA_ANGEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SERRA_ANGEL,
    "475f2a7b-9bd5-4ad7-868a-7652d06f3f6c",
    "Greg Staples",
);

// M14 33 — Show of Valor (reprint)
const SHOW_OF_VALOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &magic_2013::SHOW_OF_VALOR,
    "c07932c2-7b97-4abe-be2b-02fc04de780f",
    "Anthony Palumbo",
);

// M14 34 — Siege Mastodon (reprint)
const SIEGE_MASTODON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::SIEGE_MASTODON,
    "40e7a30f-bb29-4c6b-bf70-53e9e4292814",
    "Matt Cavotta",
);

// M14 35 — Silence (reprint)
const SILENCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::SILENCE,
    "1c2b13b1-31f0-4676-88a7-53f3a190e9a2",
    "Wayne Reynolds",
);

// M14 36 — Solemn Offering (reprint)
const SOLEMN_OFFERING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::SOLEMN_OFFERING,
    "9ca09fed-f9b3-49ee-be89-404581a4cbd2",
    "Sam Wood",
);

// M14 37 — Soulmender
pub(in crate::card::sets) static SOULMENDER: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Soulmender",
    "37f45133-6134-4664-9952-67c03d60f9a0",
    "James Ryman",
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
    CardSet::Magic2014,
    "Steelform Sliver",
    "c15d6329-ffb1-43fd-8558-60c8315f5b91",
    "Chase Stone",
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
    CardSet::Magic2014,
    "Stonehorn Chanter",
    "cd6ec61b-c039-4526-a359-a7947eeba5c3",
    "Raymond Swanland",
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

// M14 40 — Suntail Hawk (reprint)
const SUNTAIL_HAWK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2002::judgment::SUNTAIL_HAWK,
    "28a1f83c-a9ef-463e-97b5-2ca3b7232f82",
    "Heather Hudson",
);

// M14 41 — Wall of Swords (reprint)
const WALL_OF_SWORDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::WALL_OF_SWORDS,
    "7c733fef-8372-4a40-b340-7aa32922799e",
    "Zoltan Boros & Gabor Szikszai",
);

// M14 42 — Air Servant (reprint)
const AIR_SERVANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::AIR_SERVANT,
    "0cbc279d-952a-4b8d-b6ff-37166daa2dd5",
    "Lars Grant-West",
);

// M14 43 — Archaeomancer (reprint)
const ARCHAEOMANCER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &magic_2013::ARCHAEOMANCER,
    "32d0ae8f-3d46-4692-a23c-c461f8aa6a58",
    "Zoltan Boros",
);

// M14 44 — Armored Cancrix (reprint)
const ARMORED_CANCRIX_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::ARMORED_CANCRIX,
    "3b455b0f-a69c-43b4-bbf5-605ed41f10e0",
    "Tomasz Jedruszek",
);

// M14 45 — Cancel (reprint)
const CANCEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2006::time_spiral::CANCEL,
    "954eec32-7e40-452d-94c2-f704b819f338",
    "David Palumbo",
);

// M14 46 — Claustrophobia (reprint)
const CLAUSTROPHOBIA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &innistrad::CLAUSTROPHOBIA,
    "bca9c638-f923-4e85-bd3c-c95854b4f0fb",
    "Ryan Pancoast",
);

// M14 47 — Clone (reprint)
const CLONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::CLONE,
    "5e648262-3b9b-4c58-8e29-48356e3cb064",
    "Kev Walker",
);

// M14 48 — Colossal Whale
// Audit: unsupported — Islandwalk and defending-player targeting are unsupported, and linked exile cannot express the required one-shot duration if the source leaves early.
pub(in crate::card::sets) static COLOSSAL_WHALE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Colossal Whale",
    "f7f7caca-14ee-4d6a-97c3-e19898f86635",
    "Adam Paquette",
    crate::card::CardRules::unsupported(),
);

// M14 49 — Coral Merfolk (reprint)
const CORAL_MERFOLK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1998::urzas_saga::CORAL_MERFOLK,
    "09ef366b-26f5-473a-ab96-e668ed54d691",
    "rk post",
);

// M14 50 — Dismiss into Dream
// Audit: unsupported — Static effects cannot add a creature subtype, and no trigger event observes a permanent becoming a target.
pub(in crate::card::sets) static DISMISS_INTO_DREAM: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Dismiss into Dream",
    "af4cd7fe-639c-45a5-97af-9529904e3975",
    "Sam Wolfe Connelly",
    crate::card::CardRules::unsupported(),
);

// M14 51 — Disperse (reprint)
const DISPERSE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2008::morningtide::DISPERSE,
    "e6b415d2-53fe-4540-aea6-9cd2c498134c",
    "Steve Ellis",
);

// M14 52 — Divination (reprint)
const DIVINATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::DIVINATION,
    "bfdc821e-0e4b-43ff-86d6-134ac0b4e958",
    "Howard Lyon",
);

// M14 53 — Domestication (reprint)
const DOMESTICATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_roe::DOMESTICATION,
    "7bf0b7e3-fe3f-4710-9b92-9ffdb242e92e",
    "Jesper Ejsing",
);

// M14 54 — Elite Arcanist
// Audit: unsupported — Imprint cannot retain a chosen hand card for a later X cost, spell copy, and free-cast permission.
pub(in crate::card::sets) static ELITE_ARCANIST: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Elite Arcanist",
    "99b225fe-c07d-4d8a-bf2b-c1777bd29061",
    "James Zapata",
    crate::card::CardRules::unsupported(),
);

// M14 55 — Essence Scatter (reprint)
const ESSENCE_SCATTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::ESSENCE_SCATTER,
    "49bd28e3-1612-41b3-88a0-bb5c1ee60ace",
    "Jon Foster",
);

// M14 56 — Frost Breath (reprint)
const FROST_BREATH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2011::magic_2012::FROST_BREATH,
    "85d3f777-7660-48ae-8c32-6777ec8427d4",
    "Mike Bierek",
);

// M14 57 — Galerider Sliver
pub(in crate::card::sets) static GALERIDER_SLIVER: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Galerider Sliver",
    "425f5d1b-9989-4fd1-88e2-6c3108aefa0b",
    "James Zapata",
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
    CardSet::Magic2014,
    "Glimpse the Future",
    "f4d875e9-713d-4ddb-ae0a-db8483366319",
    "Andrew Robinson",
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
// Audit: unsupported — No trigger event observes the enchanted creature becoming a target.
pub(in crate::card::sets) static ILLUSIONARY_ARMOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Illusionary Armor",
    "d09346ff-6e63-499b-9265-c15a7b2cdece",
    "Mathias Kollros",
    crate::card::CardRules::unsupported(),
);

// M14 60 — Jace, Memory Adept (reprint)
const JACE_MEMORY_ADEPT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2011::magic_2012::JACE_MEMORY_ADEPT,
    "2801a4ad-5d44-4414-af62-458c7f90dad6",
    "D. Alexander Gregory",
);

// M14 61 — Jace's Mindseeker
// Audit: unsupported — Mill cannot retain the exact milled batch for a filtered optional free-cast choice.
pub(in crate::card::sets) static JACE_S_MINDSEEKER: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Jace's Mindseeker",
    "f67852a6-ae75-44e7-9e2d-d458c7b9d869",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// M14 62 — Merfolk Spy (reprint)
const MERFOLK_SPY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m11::MERFOLK_SPY,
    "30c89aa2-3375-4681-ad26-b5d5c7d3d842",
    "Matt Cavotta & Richard Whitters",
);

// M14 63 — Messenger Drake
pub(in crate::card::sets) static MESSENGER_DRAKE: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Messenger Drake",
    "13dd3172-0b45-4dc8-adc6-9e0ba112e664",
    "Yeong-Hao Han",
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
const NEGATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2008::morningtide::NEGATE,
    "6276afe2-1bd9-456e-82d5-389909ae5ab0",
    "Jeremy Jarvis",
);

// M14 65 — Nephalia Seakite (reprint)
const NEPHALIA_SEAKITE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &dark_ascension::NEPHALIA_SEAKITE,
    "39714ec8-b43c-4d61-9e53-46ac62da2c9f",
    "Wayne England",
);

// M14 66 — Opportunity (reprint)
const OPPORTUNITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1999::urzas_legacy::OPPORTUNITY,
    "e1b242f3-9398-4d65-a2c7-4de56ee58933",
    "Allen Williams",
);

// M14 67 — Phantom Warrior (reprint)
const PHANTOM_WARRIOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::PHANTOM_WARRIOR,
    "e12a1a64-5b32-4b85-8fae-c407d7926547",
    "Greg Staples",
);

// M14 68 — Quicken (reprint)
const QUICKEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2006::guildpact::QUICKEN,
    "066bef3d-c785-4b25-9b91-8f676aa9906f",
    "Aleksi Briclot",
);

// M14 69 — Scroll Thief (reprint)
const SCROLL_THIEF_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::SCROLL_THIEF,
    "fd793eb0-0ddb-4b49-88c6-b3565574b92f",
    "Alex Horley-Orlandelli",
);

// M14 70 — Seacoast Drake
pub(in crate::card::sets) static SEACOAST_DRAKE: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Seacoast Drake",
    "5333de10-a6d4-47ff-ab57-4edb49535739",
    "Scott Chou",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Drake"], 1, 3)
        .with_abilities(&[abilities::flying()]),
);

// M14 71 — Sensory Deprivation (reprint)
const SENSORY_DEPRIVATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &innistrad::SENSORY_DEPRIVATION,
    "7050735c-b232-47a6-a342-01795bfd0d46",
    "Steven Belledin",
);

// M14 72 — Spell Blast (reprint)
const SPELL_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SPELL_BLAST,
    "42d7af6a-bfd1-4e89-965a-68336507a9ee",
    "Jaime Jones",
);

// M14 73 — Tidebinder Mage
// Audit: unsupported — Effect durations cannot end when the source changes controller while remaining on the battlefield.
pub(in crate::card::sets) static TIDEBINDER_MAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Tidebinder Mage",
    "e032d1dd-6efc-4f6c-ad3b-30fe74845edf",
    "John Severin Brassell",
    crate::card::CardRules::unsupported(),
);

// M14 74 — Time Ebb (reprint)
const TIME_EBB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::TIME_EBB,
    "bf0c48f6-8b2e-4eff-aa1e-10e6ccae426a",
    "Alan Rabinowitz",
);

// M14 75 — Tome Scour (reprint)
const TOME_SCOUR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::TOME_SCOUR,
    "aed4cfec-5cea-4987-890e-825b2802e9f9",
    "Steven Belledin",
);

// M14 76 — Trained Condor
pub(in crate::card::sets) static TRAINED_CONDOR: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Trained Condor",
    "6e1eaa5a-3f9d-4166-b418-fd82fff86c73",
    "Alex Horley-Orlandelli",
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

// M14 77 — Traumatize (reprint)
const TRAUMATIZE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2001::odyssey::TRAUMATIZE,
    "9b8784dd-83f9-41f8-aedc-f0f81073ffcb",
    "Greg Staples",
);

// M14 78 — Wall of Frost (reprint)
const WALL_OF_FROST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::WALL_OF_FROST,
    "d4000b46-7843-4c07-8332-a10f207e2cdc",
    "Mike Bierek",
);

// M14 79 — Warden of Evos Isle
// Audit: unsupported — Generic-cost reduction cannot be filtered to creature spells with effective flying.
pub(in crate::card::sets) static WARDEN_OF_EVOS_ISLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Warden of Evos Isle",
    "2316d281-21a4-460d-9062-f0737249484e",
    "Nils Hamm",
    crate::card::CardRules::unsupported(),
);

// M14 80 — Water Servant (reprint)
const WATER_SERVANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::WATER_SERVANT,
    "a2c7562e-3e25-447d-b9f4-eb96960511b8",
    "Igor Kieryluk",
);

// M14 81 — Windreader Sphinx
pub(in crate::card::sets) static WINDREADER_SPHINX: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Windreader Sphinx",
    "f566741d-a847-4f24-b6fc-7873f0797d59",
    "Min Yum",
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
    CardSet::Magic2014,
    "Zephyr Charge",
    "f9ea2808-0dde-4065-ae7d-905aae98703f",
    "Steve Prescott",
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
    CardSet::Magic2014,
    "Accursed Spirit",
    "cf08313b-14c9-4e0b-aad7-05cbd90b1ed8",
    "Kev Walker",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Spirit"], 3, 2)
        .with_abilities(&[abilities::intimidate()]),
);

// M14 84 — Altar's Reap (reprint)
const ALTARS_REAP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_isd::ALTARS_REAP,
    "f3053af2-715c-4549-9003-bf4279029a95",
    "Donato Giancola",
);

// M14 85 — Artificer's Hex
// Audit: unsupported — Conditions and recipients cannot follow an Aura to an Equipment and then to the creature that Equipment is attached to.
pub(in crate::card::sets) static ARTIFICER_S_HEX: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Artificer's Hex",
    "3a5cd9a1-da2e-44ef-9f2e-352dc9f92c50",
    "Franz Vohwinkel",
    crate::card::CardRules::unsupported(),
);

// M14 86 — Blightcaster
pub(in crate::card::sets) static BLIGHTCASTER: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Blightcaster",
    "61752b13-255a-44d0-9fb0-5ed5680b954e",
    "Winona Nelson",
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
pub(in crate::card::sets) static BLOOD_BAIRN: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Blood Bairn",
    "a3fcbbd1-ee51-42a3-ad11-2fd41728c35d",
    "Ryan Yee",
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
// Audit: unsupported — SearchZone cannot match either exact printed name or make the selected permanent enter tapped.
pub(in crate::card::sets) static BOGBREW_WITCH: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Bogbrew Witch",
    "7559cf3e-7fad-4bcf-8551-045f9150e014",
    "Eric Deschamps",
    crate::card::CardRules::unsupported(),
);

// M14 89 — Child of Night (reprint)
const CHILD_OF_NIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::CHILD_OF_NIGHT,
    "c21b5476-5f5f-46b5-b627-398e9fcd04aa",
    "Ash Wood",
);

// M14 90 — Corpse Hauler
pub(in crate::card::sets) static CORPSE_HAULER: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Corpse Hauler",
    "ca6adc5e-9221-4a18-8d41-4675797e5d46",
    "Jesper Ejsing",
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
const CORRUPT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::CORRUPT,
    "d4a85ae1-7880-4612-9878-c22225bfdce1",
    "Dave Allsop",
);

// M14 92 — Dark Favor (reprint)
const DARK_FAVOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2011::magic_2012::DARK_FAVOR,
    "5c1591ae-07aa-4013-bc6d-4cafd09927f0",
    "Allen Williams",
);

// M14 93 — Dark Prophecy
pub(in crate::card::sets) static DARK_PROPHECY: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Dark Prophecy",
    "ecf82c3b-7a35-43dd-8bf3-ebc68dc1b8fc",
    "Scott Chou",
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
    CardSet::Magic2014,
    "Deathgaze Cockatrice",
    "9f17b58c-9738-4cdb-a408-e1595c384b92",
    "Kev Walker",
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Cockatrice"], 2, 2)
        .with_abilities(&[abilities::flying(), abilities::deathtouch()]),
);

// M14 95 — Diabolic Tutor (reprint)
const DIABOLIC_TUTOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2001::odyssey::DIABOLIC_TUTOR,
    "d75a7c8b-f29f-4574-96c0-daac17fc75bb",
    "Greg Staples",
);

// M14 96 — Doom Blade (reprint)
const DOOM_BLADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::DOOM_BLADE,
    "75d96a37-bdbe-46ae-926f-8742699a0b20",
    "Chippy",
);

// M14 97 — Duress (reprint)
const DURESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1998::urzas_saga::DURESS,
    "ccfa743c-b30a-4576-8205-8f00970d1076",
    "Steven Belledin",
);

// M14 98 — Festering Newt
// Audit: unsupported — Object predicates cannot test for a different exact card name to choose between two effect amounts.
pub(in crate::card::sets) static FESTERING_NEWT: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Festering Newt",
    "eaee5261-416c-41e9-9ad7-bf7bd169aa08",
    "Eric Deschamps",
    crate::card::CardRules::unsupported(),
);

// M14 99 — Gnawing Zombie
pub(in crate::card::sets) static GNAWING_ZOMBIE: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Gnawing Zombie",
    "56653d9e-0c29-440b-8724-cae746abb1a9",
    "Greg Staples",
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
// Audit: unsupported — Graveyard target predicates cannot inspect whether a card moved there from the battlefield this turn.
pub(in crate::card::sets) static GRIM_RETURN: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Grim Return",
    "15b69f74-3b54-4db4-abf3-b71db8cc9562",
    "Seb McKinnon",
    crate::card::CardRules::unsupported(),
);

// M14 101 — Lifebane Zombie
pub(in crate::card::sets) static LIFEBANE_ZOMBIE: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Lifebane Zombie",
    "98370735-5303-40d4-9e80-cdb40dee18e2",
    "Min Yum",
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
const LILIANA_OF_THE_DARK_REALMS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m13::LILIANA_OF_THE_DARK_REALMS,
    "00cbe506-7332-4d29-9404-b7c6e1e791d8",
    "D. Alexander Gregory",
);

// M14 103 — Liliana's Reaver
pub(in crate::card::sets) static LILIANAS_REAVER: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Liliana's Reaver",
    "a734c33c-4fa0-4f7a-943c-14a8aecea1a6",
    "Karl Kopinski",
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
pub(in crate::card::sets) static LITURGY_OF_BLOOD: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Liturgy of Blood",
    "3532105d-c550-4c20-8465-a6a19169efbd",
    "Zack Stella",
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
const MARK_OF_THE_VAMPIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &magic_2013::MARK_OF_THE_VAMPIRE,
    "71c2f0fb-3291-489c-92cf-8d326f2e6735",
    "Winona Nelson",
);

// M14 106 — Mind Rot (reprint)
const MIND_ROT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::MIND_ROT,
    "362e8f06-3d52-4368-a686-8fec3417b034",
    "Steve Luke",
);

// M14 107 — Minotaur Abomination
pub(in crate::card::sets) static MINOTAUR_ABOMINATION: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Minotaur Abomination",
    "9dca75a1-443d-4f8e-b12b-2aada3a8e3e4",
    "Karl Kopinski",
    CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Zombie", "Minotaur"], 4, 6),
);

// M14 108 — Nightmare (reprint)
const NIGHTMARE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NIGHTMARE,
    "b7cf63e7-9143-4236-a887-afd3628d0c03",
    "Vance Kovacs",
);

// M14 109 — Nightwing Shade (reprint)
const NIGHTWING_SHADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::NIGHTWING_SHADE,
    "a3112a8a-dc80-4099-966c-8fa1807a189b",
    "Lucas Graciano",
);

// M14 110 — Quag Sickness (reprint)
const QUAG_SICKNESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::QUAG_SICKNESS,
    "a759dcd2-ca07-4428-a3ea-b2e829b1fcb4",
    "Martina Pilcerova",
);

// M14 111 — Rise of the Dark Realms
// Audit: unsupported — MoveToZone cannot sweep matching cards from every graveyard into one player's control.
pub(in crate::card::sets) static RISE_OF_THE_DARK_REALMS: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Rise of the Dark Realms",
    "073f81e8-8c0c-4430-bd3e-95ed3625340f",
    "Michael Komarck",
    crate::card::CardRules::unsupported(),
);

// M14 112 — Sanguine Bond (reprint)
const SANGUINE_BOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::SANGUINE_BOND,
    "e50e807d-b2eb-4b62-8663-8ad17eed2a39",
    "Jaime Jones",
);

// M14 113 — Sengir Vampire (reprint)
const SENGIR_VAMPIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SENGIR_VAMPIRE,
    "8a78dbdc-1c5a-4b48-8cd1-64f7b4c29dd6",
    "Kev Walker",
);

// M14 114 — Shadowborn Apostle
// Audit: unsupported — Deck construction has no any-number exception, and activated costs cannot choose and sacrifice six matching permanents as one payment.
pub(in crate::card::sets) static SHADOWBORN_APOSTLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Shadowborn Apostle",
    "202c2323-6589-457a-af51-5528a98e7b30",
    "Lucas Graciano",
    crate::card::CardRules::unsupported(),
);

// M14 115 — Shadowborn Demon
pub(in crate::card::sets) static SHADOWBORN_DEMON: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Shadowborn Demon",
    "3884c05b-c10e-4f1d-a8bd-8b5118657972",
    "Lucas Graciano",
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

// M14 116 — Shrivel (reprint)
const SHRIVEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::rise_of_the_eldrazi::SHRIVEL,
    "47b2ffdd-f8a4-49e4-aab1-a8096ba2b7cb",
    "Jung Park",
);

// M14 117 — Syphon Sliver
pub(in crate::card::sets) static SYPHON_SLIVER: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Syphon Sliver",
    "85cb40e3-c3ed-4b3f-88ad-6f1305297c6f",
    "Tyler Jacobson",
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
// Audit: unsupported — The effect vocabulary cannot make the returned source enter tapped after a death trigger.
pub(in crate::card::sets) static TENACIOUS_DEAD: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Tenacious Dead",
    "5b96fed2-0be9-4181-94ae-10f031e2aeb2",
    "John Stanko",
    crate::card::CardRules::unsupported(),
);

// M14 119 — Undead Minotaur
pub(in crate::card::sets) static UNDEAD_MINOTAUR: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Undead Minotaur",
    "5e5ae910-ee1d-4958-92d9-0b06872913c6",
    "Karl Kopinski",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Zombie", "Minotaur"], 2, 3),
);

// M14 120 — Vampire Warlord
pub(in crate::card::sets) static VAMPIRE_WARLORD: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Vampire Warlord",
    "7e07929b-450c-45b0-85e6-512ad280a122",
    "Wesley Burt",
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
const VILE_REBIRTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &magic_2013::VILE_REBIRTH,
    "3fd46f2b-1458-44b1-9c65-960b261b81a5",
    "Erica Yang",
);

// M14 122 — Wring Flesh (reprint)
const WRING_FLESH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2011::magic_2012::WRING_FLESH,
    "d6b77692-08aa-40b6-b21b-c29a2dc87709",
    "Izzy",
);

// M14 123 — Xathrid Necromancer
pub(in crate::card::sets) static XATHRID_NECROMANCER: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Xathrid Necromancer",
    "26494f96-1d97-4435-a116-3ade1becaab4",
    "Maciej Kuciara",
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
// Audit: unsupported — The optional discard cost needs a continuation that draws only when a card was actually discarded.
pub(in crate::card::sets) static ACADEMY_RAIDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Academy Raider",
    "6652ed29-ee90-4abc-a6cf-6b18a6cbae86",
    "Karl Kopinski",
    crate::card::CardRules::unsupported(),
);

// M14 125 — Act of Treason (reprint)
const ACT_OF_TREASON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::ACT_OF_TREASON,
    "babcfccf-83db-40e7-8a2c-f77358ba3cc0",
    "Eric Deschamps",
);

// M14 126 — Awaken the Ancient
pub(in crate::card::sets) static AWAKEN_THE_ANCIENT: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Awaken the Ancient",
    "e4125304-fd68-4051-96d5-625ffa9b0d3c",
    "Jaime Jones",
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
pub(in crate::card::sets) static BARRAGE_OF_EXPENDABLES: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Barrage of Expendables",
    "b9e0912d-b4b9-497c-bce7-ed80b79bab32",
    "Trevor Claxton",
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
    CardSet::Magic2014,
    "Battle Sliver",
    "68490b8c-e9d1-4f5c-9001-750be0e0569f",
    "Slawomir Maniak",
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
    CardSet::Magic2014,
    "Blur Sliver",
    "63227937-86cc-45e0-9e9e-8c7ab80cbaef",
    "Daarken",
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
    CardSet::Magic2014,
    "Burning Earth",
    "1df3a7c9-5c8d-438c-a5ad-3c9754c6ea5d",
    "rk post",
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
const CANYON_MINOTAUR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::conflux::CANYON_MINOTAUR,
    "3469d73e-6de1-4b91-83e3-b1714ac29268",
    "Steve Prescott",
);

// M14 132 — Chandra, Pyromaster
// Audit: unsupported — Planeswalker support lacks a turn-long cannot-block effect, top-card exile/play permission, and the ultimate's repeatable spell-copy procedure.
pub(in crate::card::sets) static CHANDRA_PYROMASTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Chandra, Pyromaster",
    "bcb4f983-a4b4-46df-830d-ab3d892c93bb",
    "Winona Nelson",
    crate::card::CardRules::unsupported(),
);

// M14 133 — Chandra's Outrage (reprint)
const CHANDRAS_OUTRAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::archenemy::CHANDRAS_OUTRAGE,
    "65d1b479-f6f6-4fec-a5a6-1a74d426fb13",
    "Christopher Moeller",
);

// M14 134 — Chandra's Phoenix (reprint)
const CHANDRA_S_PHOENIX_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2011::magic_2012::CHANDRA_S_PHOENIX,
    "ca08d3ce-a3a7-49ca-aa2f-4dcdacbf923d",
    "Aleksi Briclot",
);

// M14 135 — Cyclops Tyrant
pub(in crate::card::sets) static CYCLOPS_TYRANT: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Cyclops Tyrant",
    "f0b8e733-22a7-4696-83b3-297cbe75dadc",
    "Zack Stella",
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
const DEMOLISH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2001::odyssey::DEMOLISH,
    "7827a7f9-132e-40aa-b881-423440a273bd",
    "John Avon",
);

// M14 137 — Dragon Egg
pub(in crate::card::sets) static DRAGON_EGG: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Dragon Egg",
    "dc2048f7-0c68-4142-9aad-de9b91fe5958",
    "Jack Wang",
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
const DRAGON_HATCHLING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &magic_2013::DRAGON_HATCHLING,
    "98f2e830-a8ef-4c91-978d-23b5f851edae",
    "David Palumbo",
);

// M14 139 — Flames of the Firebrand (reprint)
const FLAMES_OF_THE_FIREBRAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &magic_2013::FLAMES_OF_THE_FIREBRAND,
    "aba659ce-b15c-4af9-bba1-0fb79b23f444",
    "Steve Argyle",
);

// M14 140 — Fleshpulper Giant
pub(in crate::card::sets) static FLESHPULPER_GIANT: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Fleshpulper Giant",
    "f2726d3c-c182-4d8a-a723-0de2c5c4b152",
    "Alex Horley-Orlandelli",
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
pub(in crate::card::sets) static GOBLIN_DIPLOMATS: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Goblin Diplomats",
    "4620c581-fef7-45e8-ba20-d00903c2f4c5",
    "Izzy",
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

// M14 142 — Goblin Shortcutter (reprint)
const GOBLIN_SHORTCUTTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::zendikar::GOBLIN_SHORTCUTTER,
    "a9b69456-c267-4f0d-bd0f-e2da96b9e053",
    "Jesper Ejsing",
);

// M14 143 — Lava Axe (reprint)
const LAVA_AXE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::LAVA_AXE,
    "1c4f1041-8bbe-46fa-bbe4-40cd993f53a2",
    "Brian Snõddy",
);

// M14 144 — Lightning Talons (reprint)
const LIGHTNING_TALONS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2008::shards_of_alara::LIGHTNING_TALONS,
    "87186a8a-45da-4cde-a167-c16a6abc4d24",
    "Johann Bodin",
);

// M14 145 — Marauding Maulhorn
// Audit: unsupported — Attack requirements cannot be conditional on controlling a permanent with a different exact name.
pub(in crate::card::sets) static MARAUDING_MAULHORN: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Marauding Maulhorn",
    "b7d5e3dc-f307-4f91-a5ee-e7c5d03d8102",
    "Jesper Ejsing",
    crate::card::CardRules::unsupported(),
);

// M14 146 — Mindsparker
pub(in crate::card::sets) static MINDSPARKER: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Mindsparker",
    "a94295dc-d078-4f3f-9856-bd0a1899a9ca",
    "Wayne Reynolds",
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
// Audit: unsupported — There is no coin-flip decision or result-conditioned self-return effect.
pub(in crate::card::sets) static MOLTEN_BIRTH: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Molten Birth",
    "0cd182be-1604-47e1-858f-3c304fd0ee63",
    "Jaime Jones",
    crate::card::CardRules::unsupported(),
);

// M14 148 — Ogre Battledriver
pub(in crate::card::sets) static OGRE_BATTLEDRIVER: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Ogre Battledriver",
    "bff2d740-22cc-4719-ac58-28621951e68d",
    "Greg Staples",
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
const PITCHBURN_DEVILS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &innistrad::PITCHBURN_DEVILS,
    "f9e0a702-6984-4ccd-9ce8-4518c9b19e22",
    "Johann Bodin",
);

// M14 150 — Regathan Firecat
pub(in crate::card::sets) static REGATHAN_FIRECAT: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Regathan Firecat",
    "4b4df1dd-886d-4fe7-b3f7-2dca044de41c",
    "Eric Velhagen",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Elemental", "Cat"], 4, 1),
);

// M14 151 — Scourge of Valkas
pub(in crate::card::sets) static SCOURGE_OF_VALKAS: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Scourge of Valkas",
    "27ce2b55-45bf-4852-a74a-d0b17c6c9c3f",
    "Lucas Graciano",
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
    crate::card::CardSet::Magic2014,
    "Seismic Stomp",
    "f55a02a3-8b65-44a7-82ef-2d3dc05d00ab",
    "Chase Stone",
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
const SHIV_S_EMBRACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::SHIV_S_EMBRACE,
    "8a42fcd6-32ce-4a20-af4d-83bd32a7ed3e",
    "Dave Kendall",
);

// M14 154 — Shivan Dragon (reprint)
const SHIVAN_DRAGON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SHIVAN_DRAGON,
    "3815cf4a-b694-432b-b618-0893f8f3dc1b",
    "Donato Giancola",
);

// M14 155 — Shock (reprint)
const SHOCK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1998::stronghold::SHOCK,
    "2fbec2ea-7b60-4c51-9782-52ccdd96c4b7",
    "Jon Foster",
);

// M14 156 — Smelt (reprint)
const SMELT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &magic_2013::SMELT,
    "90515c37-93cc-4bb2-8237-21e39076c995",
    "Zoltan Boros",
);

// M14 157 — Striking Sliver
pub(in crate::card::sets) static STRIKING_SLIVER: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Striking Sliver",
    "4ee9254b-3d98-4477-a82e-1450cf3ee96e",
    "Maciej Kuciara",
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
pub(in crate::card::sets) static THORNCASTER_SLIVER: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Thorncaster Sliver",
    "3655d837-945f-4ff5-8952-cff5f7b2d18f",
    "Trevor Claxton",
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

// M14 159 — Thunder Strike (reprint)
const THUNDER_STRIKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::THUNDER_STRIKE,
    "61aa445d-d734-4e4f-800d-fe7bea86eb70",
    "Wayne Reynolds",
);

// M14 160 — Volcanic Geyser (reprint)
const VOLCANIC_GEYSER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1996::mirage::VOLCANIC_GEYSER,
    "3d2b0fdb-2209-4b98-87e1-1eb870706cec",
    "Clint Cearley",
);

// M14 161 — Wild Guess (reprint)
const WILD_GUESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m13::WILD_GUESS,
    "e7c4556f-652b-4df0-a501-d413d32e7a91",
    "Lucas Graciano",
);

// M14 162 — Wild Ricochet (reprint)
const WILD_RICOCHET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lrw::WILD_RICOCHET,
    "962273bb-2186-4e33-99c5-65eedc4a93e9",
    "Dan Murayama Scott",
);

// M14 163 — Young Pyromancer
pub(in crate::card::sets) static YOUNG_PYROMANCER: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Young Pyromancer",
    "e349c204-3a93-4bf7-b79a-5f5f261ea2d3",
    "Cynthia Sheppard",
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
pub(in crate::card::sets) static ADVOCATE_OF_THE_BEAST: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Advocate of the Beast",
    "b1320400-5aa8-48d6-be84-197b4559456f",
    "Jesper Ejsing",
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
const BRAMBLECRUSH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &innistrad::BRAMBLECRUSH,
    "2e1bd3ff-f3cd-4e9d-ae2e-2c51d0aaec7f",
    "Drew Baker",
);

// M14 166 — Briarpack Alpha (reprint)
const BRIARPACK_ALPHA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &dark_ascension::BRIARPACK_ALPHA,
    "585c11e2-4c30-436c-9dbd-354b154f6def",
    "Daarken",
);

// M14 167 — Brindle Boar (reprint)
const BRINDLE_BOAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::BRINDLE_BOAR,
    "a30b4a78-afdd-4067-810e-1fa0ddf8fb0e",
    "Dave Allsop",
);

// M14 168 — Deadly Recluse (reprint)
const DEADLY_RECLUSE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::DEADLY_RECLUSE,
    "c82cc190-7ec0-4493-b3dc-dad0cbffa2f8",
    "Warren Mahy",
);

// M14 169 — Elvish Mystic
pub(in crate::card::sets) static ELVISH_MYSTIC: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Elvish Mystic",
    "60d0e6a6-629a-45a7-bfcb-25ba7156788b",
    "Wesley Burt",
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Druid"], 1, 1)
        .with_abilities(&[abilities::tap_for(ManaColor::Green)]),
);

// M14 170 — Enlarge
pub(in crate::card::sets) static ENLARGE: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Enlarge",
    "b46cd181-59d8-4d4c-a8b6-e6b38704009c",
    "Michael Komarck",
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
const FOG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOG,
    "8b55d04f-c52f-4b94-b25c-f5c7d8c1c18e",
    "Jaime Jones",
);

// M14 172 — Garruk, Caller of Beasts
// Audit: unsupported — The planeswalker modes need filtered multi-card top selection, a hand-to-battlefield choice, and an emblem carrying an optional creature tutor trigger.
pub(in crate::card::sets) static GARRUK_CALLER_OF_BEASTS: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Garruk, Caller of Beasts",
    "a96d0c67-e9f4-46d9-bd74-13a8606fdfe3",
    "Karl Kopinski",
    crate::card::CardRules::unsupported(),
);

// M14 173 — Garruk's Horde (reprint)
const GARRUK_S_HORDE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2011::magic_2012::GARRUK_S_HORDE,
    "88b24651-1814-440e-a415-a96c03e51544",
    "Steve Prescott",
);

// M14 174 — Giant Growth (reprint)
const GIANT_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::GIANT_GROWTH,
    "e6b32578-a074-4a46-b742-84b974748903",
    "Matt Cavotta",
);

// M14 175 — Giant Spider (reprint)
const GIANT_SPIDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::GIANT_SPIDER,
    "253ebe04-9605-424e-8cff-51d3a54f91a7",
    "Randy Gallegos",
);

// M14 176 — Gladecover Scout (reprint)
const GLADECOVER_SCOUT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2011::magic_2012::GLADECOVER_SCOUT,
    "e112d77d-f019-4709-b31a-b02952df0e35",
    "Allen Williams",
);

// M14 177 — Groundshaker Sliver
pub(in crate::card::sets) static GROUNDSHAKER_SLIVER: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Groundshaker Sliver",
    "712f0ce4-9189-4c75-9c2b-d370bce89052",
    "Chase Stone",
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

// M14 178 — Howl of the Night Pack (reprint)
const HOWL_OF_THE_NIGHT_PACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2008::shadowmoor::HOWL_OF_THE_NIGHT_PACK,
    "20fc5ff1-b8bd-44d5-a659-17eeae06736a",
    "Lars Grant-West",
);

// M14 179 — Hunt the Weak
// Audit: unsupported — DealDamage can only attribute damage to the resolving spell, not to each fighting creature.
pub(in crate::card::sets) static HUNT_THE_WEAK: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Hunt the Weak",
    "8f7a6df7-acfc-4047-b119-505f4277225c",
    "Raoul Vitale",
    crate::card::CardRules::unsupported(),
);

// M14 180 — Into the Wilds
/// Only a land may be taken, and taking it is optional. Whatever is not
/// taken remains on top, so a nonland card is still the next draw.
const WILDS_LAND: Binding = Binding!("wilds_land");
pub(in crate::card::sets) static INTO_THE_WILDS: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Into the Wilds",
    "ecfa6c8d-b5b5-4b68-9ad4-c9d8169659d6",
    "Véronique Meignaud",
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
// Audit: unsupported — Counter effects cannot double each creature's existing +1/+1 counter count.
pub(in crate::card::sets) static KALONIAN_HYDRA: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Kalonian Hydra",
    "438bd3c1-98f2-4fcc-8521-995c6c5c1a79",
    "Chris Rahn",
    crate::card::CardRules::unsupported(),
);

// M14 182 — Kalonian Tusker
pub(in crate::card::sets) static KALONIAN_TUSKER: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Kalonian Tusker",
    "135946fc-fe67-401f-821d-d7145c63f030",
    "Svetlin Velinov",
    CardRules::new_creature(mana_cost!("{G}{G}"), &["Beast"], 3, 3),
);

// M14 183 — Lay of the Land (reprint)
const LAY_OF_THE_LAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2001::apocalypse::LAY_OF_THE_LAND,
    "3bb3410b-d6c3-4e42-b3c9-fb557f9a16f0",
    "Chuck Lukacs",
);

// M14 184 — Manaweft Sliver
pub(in crate::card::sets) static MANAWEFT_SLIVER: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Manaweft Sliver",
    "fe45433b-e124-44d7-9463-dada39310148",
    "Trevor Claxton",
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
pub(in crate::card::sets) static MEGANTIC_SLIVER: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Megantic Sliver",
    "7745f6a9-400c-4200-9732-86c54247de46",
    "Ryan Barger",
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
const NATURALIZE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &onslaught::NATURALIZE,
    "2bac967b-f684-4916-b7f4-8fffdd752a93",
    "Tim Hildebrandt",
);

// M14 187 — Oath of the Ancient Wood
pub(in crate::card::sets) static OATH_OF_THE_ANCIENT_WOOD: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Oath of the Ancient Wood",
    "9bc42032-8727-4f78-b369-ba103d965b73",
    "Dan Murayama Scott",
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
const PLUMMET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::archenemy::PLUMMET,
    "7cbac2e5-cc3a-4be1-9891-6098b1066de8",
    "Pete Venters",
);

// M14 189 — Predatory Sliver
pub(in crate::card::sets) static PREDATORY_SLIVER: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Predatory Sliver",
    "a2e37de8-66a1-4afa-aa6f-1151f849dfa8",
    "Mathias Kollros",
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
    CardSet::Magic2014,
    "Primeval Bounty",
    "e750d55d-d5e8-4abe-99cf-f6b8ba86cf16",
    "Christine Choi",
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
const RANGERS_GUILE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &innistrad::RANGERS_GUILE,
    "24976e3a-9f34-4dce-8bb3-efecfdfff160",
    "Steve Prescott",
);

// M14 192 — Rootwalla (reprint)
const ROOTWALLA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::tempest::ROOTWALLA,
    "2b84b6dc-d78d-4d6a-9e9a-2b40854a102b",
    "Roger Raupp",
);

// M14 193 — Rumbling Baloth
pub(in crate::card::sets) static RUMBLING_BALOTH: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Rumbling Baloth",
    "d8610ff1-064b-4c75-a8df-d3b076370d1e",
    "Jesper Ejsing",
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Beast"], 4, 4),
);

// M14 194 — Savage Summoning
// Audit: unsupported — No continuation can tag the next creature spell with flash, uncounterability, and an entry counter while also making this spell uncounterable.
pub(in crate::card::sets) static SAVAGE_SUMMONING: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Savage Summoning",
    "b5346ed7-2e17-4d8c-9c4b-b5efdd26380d",
    "Johann Bodin",
    crate::card::CardRules::unsupported(),
);

// M14 195 — Scavenging Ooze (reprint)
const SCAVENGING_OOZE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2011::commander_2011::SCAVENGING_OOZE,
    "ec30153a-36b5-42f8-beed-9efab09f1051",
    "Austin Hsu",
);

// M14 196 — Sporemound
pub(in crate::card::sets) static SPOREMOUND: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Sporemound",
    "2d256cd0-6fe9-4905-9886-fb1457292db5",
    "Svetlin Velinov",
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

// M14 197 — Trollhide (reprint)
const TROLLHIDE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2011::magic_2012::TROLLHIDE,
    "08b9c400-dc8f-4fe6-a868-fdf0d247086a",
    "Steven Belledin",
);

// M14 198 — Vastwood Hydra
// Audit: unsupported — Entry replacements cannot add chosen-X counters, and counter distribution cannot read the dead source's last-known counter count.
pub(in crate::card::sets) static VASTWOOD_HYDRA: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Vastwood Hydra",
    "9e635174-7f7d-4c04-a6aa-8674da6863ff",
    "Slawomir Maniak",
    crate::card::CardRules::unsupported(),
);

// M14 199 — Verdant Haven (reprint)
const VERDANT_HAVEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_gtc::VERDANT_HAVEN,
    "f55e37e6-6f19-4dfa-bd13-6e261177c1cf",
    "Daniel Ljunggren",
);

// M14 200 — Voracious Wurm
// Audit: unsupported — Entry replacements cannot derive a counter amount from life gained during the turn.
pub(in crate::card::sets) static VORACIOUS_WURM: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Voracious Wurm",
    "da15100b-2934-438c-9917-84ad8bdc4181",
    "Igor Kieryluk",
    crate::card::CardRules::unsupported(),
);

// M14 201 — Windstorm (reprint)
const WINDSTORM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::WINDSTORM,
    "3cb7d122-34e8-48e1-a978-831c78a37d0c",
    "Rob Alexander",
);

// M14 202 — Witchstalker
// Audit: unsupported — A non-intervening spell-cast trigger cannot capture “during your turn” without incorrectly rechecking that restriction on resolution.
pub(in crate::card::sets) static WITCHSTALKER: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Witchstalker",
    "5a5ce47d-ea4f-4e15-adb6-5bb66981ed24",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// M14 203 — Woodborn Behemoth
pub(in crate::card::sets) static WOODBORN_BEHEMOTH: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Woodborn Behemoth",
    "8c73dbf3-e68e-4f21-b6ca-94302bf5574c",
    "Matt Stewart",
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

// M14 204 — Accorder's Shield (reprint)
const ACCORDERS_SHIELD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::scars_of_mirrodin::ACCORDERS_SHIELD,
    "c5a4c2ab-c5bc-4e07-8671-a688ebd5471c",
    "Alan Pollack",
);

// M14 205 — Bubbling Cauldron
// Audit: unsupported — Costs cannot select an exact named sacrifice, and effects cannot total life actually lost by all opponents for the linked gain.
pub(in crate::card::sets) static BUBBLING_CAULDRON: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Bubbling Cauldron",
    "9af87c24-a534-462b-968b-dccf6ac63299",
    "Eric Deschamps",
    crate::card::CardRules::unsupported(),
);

// M14 206 — Darksteel Forge (reprint)
const DARKSTEEL_FORGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2004::darksteel::DARKSTEEL_FORGE,
    "2c95a0a1-9c2c-44df-b0fe-c22efb6d87ee",
    "Martina Pilcerova",
);

// M14 207 — Darksteel Ingot (reprint)
const DARKSTEEL_INGOT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &darksteel::DARKSTEEL_INGOT,
    "290a80b6-2e9e-495f-81b6-845ee80fb9c2",
    "Martina Pilcerova",
);

// M14 208 — Door of Destinies (reprint)
const DOOR_OF_DESTINIES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2008::morningtide::DOOR_OF_DESTINIES,
    "68a6bf1a-7152-496f-a4c7-e720ef4294d8",
    "Larry MacDougall",
);

// M14 209 — Elixir of Immortality (reprint)
const ELIXIR_OF_IMMORTALITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::magic_2011::ELIXIR_OF_IMMORTALITY,
    "3d19d950-9e63-4b15-8531-f4b16f5b82fa",
    "Zoltan Boros & Gabor Szikszai",
);

// M14 210 — Fireshrieker (reprint)
const FIRESHRIEKER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2003::mirrodin::FIRESHRIEKER,
    "9f653742-b92a-4cfa-b3b5-8d20aabdb5dd",
    "Christopher Moeller",
);

// M14 211 — Guardian of the Ages
// Audit: unsupported — Attack events cannot match attacks at you or your planeswalker, and abilities cannot permanently remove defender from the source after that trigger.
pub(in crate::card::sets) static GUARDIAN_OF_THE_AGES: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Guardian of the Ages",
    "c825c138-97de-44b9-8aec-70608ae035b6",
    "Ryan Pancoast",
    crate::card::CardRules::unsupported(),
);

// M14 212 — Haunted Plate Mail
pub(in crate::card::sets) static HAUNTED_PLATE_MAIL: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Haunted Plate Mail",
    "e2dc1e07-7894-4f22-936d-bf5df3f8d5a5",
    "Izzy",
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
const MILLSTONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &antiquities::MILLSTONE,
    "23f28acb-8ccb-4b89-ba7f-ff7ce59852aa",
    "Yeong-Hao Han",
);

// M14 214 — Pyromancer's Gauntlet
// Audit: unsupported — Damage replacement cannot filter red instant, sorcery, or planeswalker sources and add a fixed amount to the event.
pub(in crate::card::sets) static PYROMANCER_S_GAUNTLET: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Pyromancer's Gauntlet",
    "9bde6763-2102-4adb-8048-fc9fe921205b",
    "Christine Choi",
    crate::card::CardRules::unsupported(),
);

// M14 215 — Ratchet Bomb (reprint)
const RATCHET_BOMB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2010::scars_of_mirrodin::RATCHET_BOMB,
    "3e9045df-3eff-4236-9bbb-77537b302e27",
    "Austin Hsu",
);

// M14 216 — Ring of Three Wishes
// Audit: unsupported — CounterKind has no wish counter, so the entry counters and removal cost cannot share the printed counter identity.
pub(in crate::card::sets) static RING_OF_THREE_WISHES: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2014,
    "Ring of Three Wishes",
    "219ab03a-2b3b-4eef-8a42-2cbe793d2f33",
    "Mark Winters",
    crate::card::CardRules::unsupported(),
);

// M14 217 — Rod of Ruin (reprint)
const ROD_OF_RUIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ROD_OF_RUIN,
    "9a0e90b8-bc38-4e1c-92ca-ac562cc57e31",
    "Mark Zug",
);

// M14 218 — Sliver Construct
pub(in crate::card::sets) static SLIVER_CONSTRUCT: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Sliver Construct",
    "3129645a-221c-4eb5-88fd-12cc742a1dfe",
    "Mathias Kollros",
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Sliver", "Construct"], 2, 2),
);

// M14 219 — Staff of the Death Magus
pub(in crate::card::sets) static STAFF_OF_THE_DEATH_MAGUS: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Staff of the Death Magus",
    "624fe171-8bd8-4156-b40e-74e2a847d380",
    "Daniel Ljunggren",
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
pub(in crate::card::sets) static STAFF_OF_THE_FLAME_MAGUS: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Staff of the Flame Magus",
    "6d6befbd-4fe3-4338-b8ea-13b8b70a7664",
    "Daniel Ljunggren",
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
pub(in crate::card::sets) static STAFF_OF_THE_MIND_MAGUS: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Staff of the Mind Magus",
    "f86bf36b-b83f-4451-8cdc-2a4ccffb93c7",
    "Daniel Ljunggren",
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
pub(in crate::card::sets) static STAFF_OF_THE_SUN_MAGUS: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Staff of the Sun Magus",
    "90a1f830-d19a-4ebf-9573-09b677693dd6",
    "Daniel Ljunggren",
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
pub(in crate::card::sets) static STAFF_OF_THE_WILD_MAGUS: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Staff of the Wild Magus",
    "d207f03d-4c7b-444f-bf95-e63f7004d525",
    "Daniel Ljunggren",
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
    crate::card::CardSet::Magic2014,
    "Strionic Resonator",
    "94d1fc0f-5c8b-4e47-aaf8-8888c025f70f",
    "Noah Bradley",
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
const TRADING_POST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m13::TRADING_POST,
    "132c5358-c258-4b86-862a-2d140011dc3f",
    "Adam Paquette",
);

// M14 226 — Vial of Poison
pub(in crate::card::sets) static VIAL_OF_POISON: CardRecord = CardRecord::new(
    CardSet::Magic2014,
    "Vial of Poison",
    "7769159b-5a6a-45e5-b69b-8db2a6ef5418",
    "Franz Vohwinkel",
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
    CardSet::Magic2014,
    "Encroaching Wastes",
    "1ad5a84b-ae9b-4ed1-a4de-b91bbf8ed0a5",
    "Noah Bradley",
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

// M14 228 — Mutavault (reprint)
const MUTAVAULT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2008::morningtide::MUTAVAULT,
    "927ed667-c228-4b96-a9f6-7cbadade8134",
    "Fred Fields",
);

// M14 229 — Shimmering Grotto (reprint)
const SHIMMERING_GROTTO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lrw::SHIMMERING_GROTTO,
    "7fd26b14-a920-4b82-91b8-8de0e7d03f6e",
    "Cliff Childs",
);

// M14 230 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::PLAINS,
    "334ca29a-7fb3-426f-922b-5a2b905a5565",
    "John Avon",
);

// M14 231 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::PLAINS,
    1,
    "d8098d41-38dc-4f82-b65e-d3bc7d8e0fcc",
    "Jonas De Ro",
);

// M14 232 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::PLAINS,
    2,
    "241d9f61-b94d-4ba3-b709-791ec647a716",
    "Nils Hamm",
);

// M14 233 — Plains (alternate printing)
const PLAINS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &alpha::PLAINS,
    3,
    "90742ab3-89fb-4ac1-9bf0-0e2d17252bff",
    "Andreas Rocha",
);

// M14 234 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ISLAND,
    "94de0250-53a6-45c8-85a7-a473f271102e",
    "Noah Bradley",
);

// M14 235 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::ISLAND,
    1,
    "d2814990-4c7a-44ae-8d76-157156aa79bb",
    "Cliff Childs",
);

// M14 236 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::ISLAND,
    2,
    "78d37787-be3a-4ed3-94b8-e675f2beecf0",
    "Jonas De Ro",
);

// M14 237 — Island (alternate printing)
const ISLAND_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &alpha::ISLAND,
    3,
    "b59cd300-c4f3-4ba6-8018-134eaf6a399c",
    "Andreas Rocha",
);

// M14 238 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SWAMP,
    "4510090f-b42d-4df1-af71-64a77dfbc1b2",
    "Cliff Childs",
);

// M14 239 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::SWAMP,
    1,
    "2cce9570-e5d7-44e5-bc93-6d03dd1a3794",
    "Jonas De Ro",
);

// M14 240 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::SWAMP,
    2,
    "3ce4394a-14c9-4a6e-898c-056108b16e09",
    "Jung Park",
);

// M14 241 — Swamp (alternate printing)
const SWAMP_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &alpha::SWAMP,
    3,
    "07504e2d-9d70-4957-bee6-abab92368a33",
    "Andreas Rocha",
);

// M14 242 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MOUNTAIN,
    "6e82bb8b-2a95-4935-a728-6898f64ce39a",
    "Cliff Childs",
);

// M14 243 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::MOUNTAIN,
    1,
    "1ffc2e95-dd9b-4581-988f-850d9e240a30",
    "Jonas De Ro",
);

// M14 244 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::MOUNTAIN,
    2,
    "67057abf-749d-4512-968a-832c56324e13",
    "Karl Kopinski",
);

// M14 245 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &alpha::MOUNTAIN,
    3,
    "15c28158-fb96-4006-bc65-425dba031395",
    "Andreas Rocha",
);

// M14 246 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::FOREST,
    "5b5277ee-f1e0-4777-9011-d7a23c855919",
    "Volkan Baǵa",
);

// M14 247 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::FOREST,
    1,
    "dd98de13-d556-4cf3-99f3-d1c0db85cb3f",
    "Steven Belledin",
);

// M14 248 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::FOREST,
    2,
    "04db1116-3819-4a57-a001-dfa7578f0f12",
    "Jonas De Ro",
);

// M14 249 — Forest (alternate printing)
const FOREST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &alpha::FOREST,
    3,
    "10b3dce8-9c28-41f6-823f-a7d64dd9e33a",
    "Andreas Rocha",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AJANI_S_CHOSEN,
    &ANGELIC_ACCORD,
    &ARCHANGEL_OF_THUNE,
    &BANISHER_PRIEST,
    &BONESCYTHE_SLIVER,
    &CELESTIAL_FLARE,
    &CHARGING_GRIFFIN,
    &DAWNSTRIKE_PALADIN,
    &DEVOUT_INVOCATION,
    &FIENDSLAYER_PALADIN,
    &HIVE_STIRRINGS,
    &IMPOSING_SOVEREIGN,
    &MASTER_OF_DIVERSION,
    &PATH_OF_BRAVERY,
    &SENTINEL_SLIVER,
    &SERAPH_OF_THE_SWORD,
    &SOULMENDER,
    &STEELFORM_SLIVER,
    &STONEHORN_CHANTER,
    &COLOSSAL_WHALE,
    &DISMISS_INTO_DREAM,
    &ELITE_ARCANIST,
    &GALERIDER_SLIVER,
    &GLIMPSE_THE_FUTURE,
    &ILLUSIONARY_ARMOR,
    &JACE_S_MINDSEEKER,
    &MESSENGER_DRAKE,
    &SEACOAST_DRAKE,
    &TIDEBINDER_MAGE,
    &TRAINED_CONDOR,
    &WARDEN_OF_EVOS_ISLE,
    &WINDREADER_SPHINX,
    &ZEPHYR_CHARGE,
    &ACCURSED_SPIRIT,
    &ARTIFICER_S_HEX,
    &BLIGHTCASTER,
    &BLOOD_BAIRN,
    &BOGBREW_WITCH,
    &CORPSE_HAULER,
    &DARK_PROPHECY,
    &DEATHGAZE_COCKATRICE,
    &FESTERING_NEWT,
    &GNAWING_ZOMBIE,
    &GRIM_RETURN,
    &LIFEBANE_ZOMBIE,
    &LILIANAS_REAVER,
    &LITURGY_OF_BLOOD,
    &MINOTAUR_ABOMINATION,
    &RISE_OF_THE_DARK_REALMS,
    &SHADOWBORN_APOSTLE,
    &SHADOWBORN_DEMON,
    &SYPHON_SLIVER,
    &TENACIOUS_DEAD,
    &UNDEAD_MINOTAUR,
    &VAMPIRE_WARLORD,
    &XATHRID_NECROMANCER,
    &ACADEMY_RAIDER,
    &AWAKEN_THE_ANCIENT,
    &BARRAGE_OF_EXPENDABLES,
    &BATTLE_SLIVER,
    &BLUR_SLIVER,
    &BURNING_EARTH,
    &CHANDRA_PYROMASTER,
    &CYCLOPS_TYRANT,
    &DRAGON_EGG,
    &FLESHPULPER_GIANT,
    &GOBLIN_DIPLOMATS,
    &MARAUDING_MAULHORN,
    &MINDSPARKER,
    &MOLTEN_BIRTH,
    &OGRE_BATTLEDRIVER,
    &REGATHAN_FIRECAT,
    &SCOURGE_OF_VALKAS,
    &SEISMIC_STOMP,
    &STRIKING_SLIVER,
    &THORNCASTER_SLIVER,
    &YOUNG_PYROMANCER,
    &ADVOCATE_OF_THE_BEAST,
    &ELVISH_MYSTIC,
    &ENLARGE,
    &GARRUK_CALLER_OF_BEASTS,
    &GROUNDSHAKER_SLIVER,
    &HUNT_THE_WEAK,
    &INTO_THE_WILDS,
    &KALONIAN_HYDRA,
    &KALONIAN_TUSKER,
    &MANAWEFT_SLIVER,
    &MEGANTIC_SLIVER,
    &OATH_OF_THE_ANCIENT_WOOD,
    &PREDATORY_SLIVER,
    &PRIMEVAL_BOUNTY,
    &RUMBLING_BALOTH,
    &SAVAGE_SUMMONING,
    &SPOREMOUND,
    &VASTWOOD_HYDRA,
    &VORACIOUS_WURM,
    &WITCHSTALKER,
    &WOODBORN_BEHEMOTH,
    &BUBBLING_CAULDRON,
    &GUARDIAN_OF_THE_AGES,
    &HAUNTED_PLATE_MAIL,
    &PYROMANCER_S_GAUNTLET,
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
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    AJANI_CALLER_OF_THE_PRIDE_REPRINT,
    ANGELIC_WALL_REPRINT,
    AURAMANCER_REPRINT,
    BLESSING_REPRINT,
    BRAVE_THE_ELEMENTS_REPRINT,
    CAPASHEN_KNIGHT_REPRINT,
    CONGREGATE_REPRINT,
    DIVINE_FAVOR_REPRINT,
    FORTIFY_REPRINT,
    GRIFFIN_SENTINEL_REPRINT,
    INDESTRUCTIBILITY_REPRINT,
    PACIFISM_REPRINT,
    PAY_NO_HEED_REPRINT,
    PILLARFIELD_OX_REPRINT,
    PLANAR_CLEANSING_REPRINT,
    SERRA_ANGEL_REPRINT,
    SHOW_OF_VALOR_REPRINT,
    SIEGE_MASTODON_REPRINT,
    SILENCE_REPRINT,
    SOLEMN_OFFERING_REPRINT,
    SUNTAIL_HAWK_REPRINT,
    WALL_OF_SWORDS_REPRINT,
    AIR_SERVANT_REPRINT,
    ARCHAEOMANCER_REPRINT,
    ARMORED_CANCRIX_REPRINT,
    CANCEL_REPRINT,
    CLAUSTROPHOBIA_REPRINT,
    CLONE_REPRINT,
    CORAL_MERFOLK_REPRINT,
    DISPERSE_REPRINT,
    DIVINATION_REPRINT,
    DOMESTICATION_REPRINT,
    ESSENCE_SCATTER_REPRINT,
    FROST_BREATH_REPRINT,
    JACE_MEMORY_ADEPT_REPRINT,
    MERFOLK_SPY_REPRINT,
    NEGATE_REPRINT,
    NEPHALIA_SEAKITE_REPRINT,
    OPPORTUNITY_REPRINT,
    PHANTOM_WARRIOR_REPRINT,
    QUICKEN_REPRINT,
    SCROLL_THIEF_REPRINT,
    SENSORY_DEPRIVATION_REPRINT,
    SPELL_BLAST_REPRINT,
    TIME_EBB_REPRINT,
    TOME_SCOUR_REPRINT,
    TRAUMATIZE_REPRINT,
    WALL_OF_FROST_REPRINT,
    WATER_SERVANT_REPRINT,
    ALTARS_REAP_REPRINT,
    CHILD_OF_NIGHT_REPRINT,
    CORRUPT_REPRINT,
    DARK_FAVOR_REPRINT,
    DIABOLIC_TUTOR_REPRINT,
    DOOM_BLADE_REPRINT,
    DURESS_REPRINT,
    LILIANA_OF_THE_DARK_REALMS_REPRINT,
    MARK_OF_THE_VAMPIRE_REPRINT,
    MIND_ROT_REPRINT,
    NIGHTMARE_REPRINT,
    NIGHTWING_SHADE_REPRINT,
    QUAG_SICKNESS_REPRINT,
    SANGUINE_BOND_REPRINT,
    SENGIR_VAMPIRE_REPRINT,
    SHRIVEL_REPRINT,
    VILE_REBIRTH_REPRINT,
    WRING_FLESH_REPRINT,
    ACT_OF_TREASON_REPRINT,
    CANYON_MINOTAUR_REPRINT,
    CHANDRAS_OUTRAGE_REPRINT,
    CHANDRA_S_PHOENIX_REPRINT,
    DEMOLISH_REPRINT,
    DRAGON_HATCHLING_REPRINT,
    FLAMES_OF_THE_FIREBRAND_REPRINT,
    GOBLIN_SHORTCUTTER_REPRINT,
    LAVA_AXE_REPRINT,
    LIGHTNING_TALONS_REPRINT,
    PITCHBURN_DEVILS_REPRINT,
    SHIV_S_EMBRACE_REPRINT,
    SHIVAN_DRAGON_REPRINT,
    SHOCK_REPRINT,
    SMELT_REPRINT,
    THUNDER_STRIKE_REPRINT,
    VOLCANIC_GEYSER_REPRINT,
    WILD_GUESS_REPRINT,
    WILD_RICOCHET_REPRINT,
    BRAMBLECRUSH_REPRINT,
    BRIARPACK_ALPHA_REPRINT,
    BRINDLE_BOAR_REPRINT,
    DEADLY_RECLUSE_REPRINT,
    FOG_REPRINT,
    GARRUK_S_HORDE_REPRINT,
    GIANT_GROWTH_REPRINT,
    GIANT_SPIDER_REPRINT,
    GLADECOVER_SCOUT_REPRINT,
    HOWL_OF_THE_NIGHT_PACK_REPRINT,
    LAY_OF_THE_LAND_REPRINT,
    NATURALIZE_REPRINT,
    PLUMMET_REPRINT,
    RANGERS_GUILE_REPRINT,
    ROOTWALLA_REPRINT,
    SCAVENGING_OOZE_REPRINT,
    TROLLHIDE_REPRINT,
    VERDANT_HAVEN_REPRINT,
    WINDSTORM_REPRINT,
    ACCORDERS_SHIELD_REPRINT,
    DARKSTEEL_FORGE_REPRINT,
    DARKSTEEL_INGOT_REPRINT,
    DOOR_OF_DESTINIES_REPRINT,
    ELIXIR_OF_IMMORTALITY_REPRINT,
    FIRESHRIEKER_REPRINT,
    MILLSTONE_REPRINT,
    RATCHET_BOMB_REPRINT,
    ROD_OF_RUIN_REPRINT,
    TRADING_POST_REPRINT,
    MUTAVAULT_REPRINT,
    SHIMMERING_GROTTO_REPRINT,
    PLAINS_REPRINT,
    PLAINS_ALTERNATE_1,
    PLAINS_ALTERNATE_2,
    PLAINS_ALTERNATE_3,
    ISLAND_REPRINT,
    ISLAND_ALTERNATE_1,
    ISLAND_ALTERNATE_2,
    ISLAND_ALTERNATE_3,
    SWAMP_REPRINT,
    SWAMP_ALTERNATE_1,
    SWAMP_ALTERNATE_2,
    SWAMP_ALTERNATE_3,
    MOUNTAIN_REPRINT,
    MOUNTAIN_ALTERNATE_1,
    MOUNTAIN_ALTERNATE_2,
    MOUNTAIN_ALTERNATE_3,
    FOREST_REPRINT,
    FOREST_ALTERNATE_1,
    FOREST_ALTERNATE_2,
    FOREST_ALTERNATE_3,
];
