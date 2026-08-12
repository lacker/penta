//! Magic 2013 card records used by the built-in ISD–RTR Standard deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    AddManaEffectDef, AppliedEffectDef, BasicLandType, CardArt, CardBehavior, CardRules, CardSet,
    CardSupertype, CardType, DividedTotal, EffectDef, EffectDurationDef, EffectExecutionDef,
    EffectRecipientDef, KeywordAbility, ManaColor, ObjectPredicateDef, ObjectQueryDef,
    PlayerRelation, ReplacementEventDef, TriggerEventDef, ValueDef, ZoneKind, abilities, cards,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

// M13 22 — Oblivion Ring
pub(in crate::card::sets) static OBLIVION_RING: CardRecord = CardRecord::new(
    cards::OBLIVION_RING,
    "Oblivion Ring",
    CardArt::new("1e2a73ec-39be-4d23-8c25-17d7c174dcee", "Franz Vohwinkel"),
    CardSet::Magic2013,
    CardRules::new_enchantment(mana_cost!("{2}{W}")).with_abilities(&[
        AbilityDef::triggered_with_targets("When this enchantment enters, exile another target nonland permanent.", TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            }, &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )], EffectDef::ExileLinkedToSource {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            }),
        AbilityDef::triggered(
            "When this enchantment leaves the battlefield, return the exiled card to the battlefield under its owner's control.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: Some(ZoneKind::Battlefield),
                to: None,
            },
            EffectDef::ReturnLinkedExiles {
                zone: ZoneKind::Battlefield,
                grant: None,
            },
        ),
    ]),
);

// M13 29 — Rhox Faithmender
pub(in crate::card::sets) static RHOX_FAITHMENDER: CardRecord = CardRecord::new(
    cards::RHOX_FAITHMENDER,
    "Rhox Faithmender",
    CardArt::new("85ea185a-7b38-49f3-be73-be8180fb6295", "Wesley Burt"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Rhino", "Monk"], 1, 5).with_abilities(&[
        abilities::lifelink(),
        AbilityDef::replacement_for(
            "If you would gain life, you gain twice that much life instead.",
            ReplacementEventDef::WouldGainLife(PlayerRelation::You),
            EffectDef::MultiplyEventAmount(2),
        ),
    ]),
);

// M13 39 — War Priest of Thune
pub(in crate::card::sets) static WAR_PRIEST_OF_THUNE: CardRecord = CardRecord::new(
    cards::WAR_PRIEST_OF_THUNE,
    "War Priest of Thune",
    CardArt::new("d28eb320-aea7-466e-8718-de8652a2b191", "Izzy"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Cleric"], 2, 2).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "When this creature enters, you may destroy target enchantment.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            &[AbilityTargetDef {
                predicate: AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Enchantment),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                // "You may" is an optional target: declining to choose one is how the
                // trigger does nothing, so the minimum is zero rather than one.
                minimum: 0,
                maximum: 1,
                divided_total: None,
            }],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
        ),
    ]),
);
// M13 43 — Augur of Bolas
pub(in crate::card::sets) static AUGUR_OF_BOLAS: CardRecord = CardRecord::new(
    cards::AUGUR_OF_BOLAS,
    "Augur of Bolas",
    CardArt::new("2e6ec8a6-ad88-45c9-ab4b-dd7de2418bb7", "Slawomir Maniak"),
    CardSet::Magic2013,
    CardRules::new_creature(
        mana_cost!("{1}{U}"),
        &["Merfolk", "Wizard"],
        1,
        3,
    )
    .with_abilities(&[
        AbilityDef::triggered(
            "When this creature enters, look at the top three cards of your library. You may reveal an instant or sorcery card from among them and put it into your hand. Put the rest on the bottom of your library in any order.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::None,
        )
        .with_effect_execution(EffectExecutionDef::Custom(CardBehavior::AugurOfBolas))
        .with_coverage(AbilityCoverageDef::explained_complete(
            "The trigger uses the shared stack and a card-local library-selection resolver.",
        )),
    ]),
);

// M13 50 — Essence Scatter
pub(in crate::card::sets) static ESSENCE_SCATTER: CardRecord = CardRecord::new(
    cards::ESSENCE_SCATTER,
    "Essence Scatter",
    CardArt::new("fcd965f9-bdaa-4434-a9c8-53fc57e997db", "Jon Foster"),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::custom_full(
        "Counter target creature spell.",
        CardBehavior::EssenceScatter,
        "Implemented by the named card-local special behavior.",
    )),
);

// M13 56 — Jace, Memory Adept
pub(in crate::card::sets) static JACE_MEMORY_ADEPT: CardRecord = CardRecord::new(
    cards::JACE_MEMORY_ADEPT,
    "Jace, Memory Adept",
    CardArt::new(
        "96b2a335-2f01-4ba7-a037-453dbb1045e9",
        "D. Alexander Gregory",
    ),
    CardSet::Magic2013,
    CardRules::new_planeswalker(mana_cost!("{3}{U}{U}"), &["Jace"], 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_with_targets(
                "+1: Draw a card. Target player mills a card.",
                &[AbilityCostDef::Loyalty(1)],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::Sequence(&[
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::Mill {
                        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
            AbilityDef::activated_with_targets(
                "0: Target player mills ten cards.",
                &[AbilityCostDef::Loyalty(0)],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::Mill {
                    player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(10),
                },
            ),
            AbilityDef::activated_with_targets(
                "−7: Any number of target players each draw twenty cards.",
                &[AbilityCostDef::Loyalty(-7)],
                // Two players means "any number" is up to two.
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                    2,
                )],
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(20),
                },
            ),
        ]),
);

// M13 62 — Negate
pub(in crate::card::sets) static NEGATE: CardRecord = CardRecord::new(
    cards::NEGATE,
    "Negate",
    CardArt::new("8da17a86-3666-46b8-932e-daafd6a0cd69", "Jeremy Jarvis"),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::custom_full(
        "Counter target noncreature spell.",
        CardBehavior::Negate,
        "Implemented by the named card-local special behavior.",
    )),
);

/// X is read off the sacrificed creature, so both halves take the power the
/// sacrifice recorded rather than counting anything on the board.
static DISCIPLE_OF_BOLAS_PAYOFF: EffectDef = EffectDef::Sequence(&[
    EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::TriggerEventAmount,
    },
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::TriggerEventAmount,
    },
]);

// M13 88 — Disciple of Bolas
pub(in crate::card::sets) static DISCIPLE_OF_BOLAS: CardRecord = CardRecord::new(
    cards::DISCIPLE_OF_BOLAS,
    "Disciple of Bolas",
    CardArt::new("c4dd57f8-27bc-4ad9-a79e-48a68af33b02", "Slawomir Maniak"),
    CardSet::Magic2013,
    CardRules::new_creature(
        mana_cost!("{3}{B}"),
        &["Human", "Wizard"],
        2,
        1,
    )
    .with_ability(AbilityDef::triggered(
        "When this creature enters, sacrifice another creature. You gain X life and draw X cards, where X is that creature's power.",
        TriggerEventDef::ZoneChanged {
            object: ObjectPredicateDef::Source,
            from: None,
            to: Some(ZoneKind::Battlefield),
        },
        EffectDef::SacrificeOfChoice {
            player: EffectRecipientDef::Controller,
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                // "Another" creature, so the Disciple cannot eat itself.
                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
            ]),
            then: Some(&DISCIPLE_OF_BOLAS_PAYOFF),
            optional: false,
        },
    )),
);

// M13 90 — Duress
pub(in crate::card::sets) static DURESS: CardRecord = CardRecord::new(
    cards::DURESS,
    "Duress",
    CardArt::new("f7201d43-ae2e-4faa-a508-8555079c3bc7", "Steven Belledin"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{B}")).with_ability(
        AbilityDef::custom_full(
            "Target opponent reveals their hand. You choose a noncreature, nonland card from it. That player discards that card.",
            CardBehavior::Duress,
            "Implemented by the named card-local special behavior.",
        ),
    ),
);

/// Mutilate scales with your Swamps, and reads the same count twice.
static SWAMPS_YOU_CONTROL: ValueDef = ValueDef::CountMatchingObjects(&ObjectQueryDef {
    object: ObjectPredicateDef::Subtype("Swamp"),
    zones: &[ZoneKind::Battlefield],
    controller: PlayerRelation::You,
});

// M13 102 — Mutilate
pub(in crate::card::sets) static MUTILATE: CardRecord = CardRecord::new(
    cards::MUTILATE,
    "Mutilate",
    CardArt::new("c48bc86b-df0a-4a9c-8aad-c3ffb742a5ff", "Tyler Jacobson"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{2}{B}{B}")).with_abilities(&[AbilityDef::spell(
        "All creatures get -1/-1 until end of turn for each Swamp you control.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
            effect: AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Negate(&SWAMPS_YOU_CONTROL),
                toughness: ValueDef::Negate(&SWAMPS_YOU_CONTROL),
            },
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// M13 110 — Sign in Blood
pub(in crate::card::sets) static SIGN_IN_BLOOD: CardRecord = CardRecord::new(
    cards::SIGN_IN_BLOOD,
    "Sign in Blood",
    CardArt::new("64f6600b-36c4-43bd-8c01-cfbca402ecd6", "Howard Lyon"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{B}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target player draws two cards and loses 2 life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ]),
    )]),
);

// M13 112 — Vampire Nighthawk
pub(in crate::card::sets) static VAMPIRE_NIGHTHAWK: CardRecord = CardRecord::new(
    cards::VAMPIRE_NIGHTHAWK,
    "Vampire Nighthawk",
    CardArt::new("9ba96d96-8d9e-47c8-ab39-17479564aadf", "Jason Chan"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Vampire", "Shaman"], 2, 3).with_abilities(
        &[
            abilities::flying(),
            abilities::deathtouch(),
            abilities::lifelink(),
        ],
    ),
);

// M13 132 — Flames of the Firebrand
pub(in crate::card::sets) static FLAMES_OF_THE_FIREBRAND: CardRecord = CardRecord::new(
    cards::FLAMES_OF_THE_FIREBRAND,
    "Flames of the Firebrand",
    CardArt::new("aca215b1-7b98-49ce-afae-eeb61058125a", "Steve Argyle"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Flames of the Firebrand deals 3 damage divided as you choose among one, two, or three targets.",
            &[AbilityTargetDef {
                predicate: AbilityTargetPredicate::AnyTarget,
                // One, two, or three targets is not a separate rule: three damage
                // split with every share at least one says the same thing.
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

/// The damage and the tap name the same creatures, so both clauses ask the
/// same question.
const OPPOSING_FLIERS: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
]);

// M13 150 — Thundermaw Hellkite
pub(in crate::card::sets) static THUNDERMAW_HELLKITE: CardRecord = CardRecord::new(
    cards::THUNDERMAW_HELLKITE,
    "Thundermaw Hellkite",
    CardArt::new("d0476e0f-61df-46a6-aaf1-8ee79c701160", "Svetlin Velinov"),
    CardSet::Magic2013,
    CardRules::new_creature(
        mana_cost!("{3}{R}{R}"),
        &["Dragon"],
        5,
        5,
    )
    .with_abilities(&[
        abilities::flying(),
        abilities::haste(),
        AbilityDef::triggered(
            "When this creature enters, it deals 1 damage to each creature with flying your opponents control. Tap those creatures.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::Sequence(&[
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::MatchingObjects {
                        object: OPPOSING_FLIERS,
                        zones: &[ZoneKind::Battlefield],
                        controller: PlayerRelation::Opponent,
                    },
                    amount: ValueDef::Constant(1),
                },
                EffectDef::Tap {
                    object: EffectRecipientDef::MatchingObjects {
                        object: OPPOSING_FLIERS,
                        zones: &[ZoneKind::Battlefield],
                        controller: PlayerRelation::Opponent,
                    },
                },
            ]),
        ),
    ]),
);

// M13 155 — Volcanic Strength
pub(in crate::card::sets) static VOLCANIC_STRENGTH: CardRecord = CardRecord::new(
    cards::VOLCANIC_STRENGTH,
    "Volcanic Strength",
    CardArt::new("f1963f08-1765-4f3e-92be-479773de47a0", "Izzy"),
    CardSet::Magic2013,
    CardRules::new_enchantment(mana_cost!("{1}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
        AbilityDef::spell_with_targets("Enchant creature", &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )], EffectDef::Attach {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            }),
        AbilityDef::static_ability(
            "Enchanted creature gets +2/+2 and has mountainwalk. (It can't be blocked as long as defending player controls a Mountain.)",
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(2),
                        toughness: ValueDef::Constant(2),
                    },
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::GrantAbility(&abilities::mountainwalk()),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ]),
        ),
    ]),
);

// M13 160 — Arbor Elf
pub(in crate::card::sets) static ARBOR_ELF: CardRecord = CardRecord::new(
    cards::ARBOR_ELF,
    "Arbor Elf",
    CardArt::new("b7d6b117-0c14-4455-92fc-29555ee75d97", "rk post"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Druid"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: Untap target Forest.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Subtype("Forest"),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

/// A second Mountain does not make the bonus bigger, so this is asked as a
/// condition rather than counted.
static MOUNTAIN_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef {
    object: ObjectPredicateDef::Subtype("Mountain"),
    zones: &[ZoneKind::Battlefield],
    controller: PlayerRelation::You,
};

// M13 171 — Flinthoof Boar
pub(in crate::card::sets) static FLINTHOOF_BOAR: CardRecord = CardRecord::new(
    cards::FLINTHOOF_BOAR,
    "Flinthoof Boar",
    CardArt::new("7e380b99-0173-4083-a4a2-222ad98b904a", "Erica Yang"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Boar"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "This creature gets +1/+1 as long as you control a Mountain.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::AnyMatchingObject(&MOUNTAIN_YOU_CONTROL),
                    toughness: ValueDef::AnyMatchingObject(&MOUNTAIN_YOU_CONTROL),
                },
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
        AbilityDef::activated(
            "{R}: This creature gains haste until end of turn. (It can attack and {T} this turn.)",
            &[AbilityCostDef::Mana(mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::GrantAbility(&abilities::haste()),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M13 193 — Thragtusk
pub(in crate::card::sets) static THRAGTUSK: CardRecord = CardRecord::new(
    cards::THRAGTUSK,
    "Thragtusk",
    CardArt::new("28667c8b-d02c-4e57-a050-1549207b65d1", "Nils Hamm"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Beast"], 5, 3).with_abilities(&[
        AbilityDef::triggered(
            "When this creature enters, you gain 5 life.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(5),
            },
        ),
        AbilityDef::triggered(
            "When this creature leaves the battlefield, create a 3/3 green Beast creature token.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: Some(ZoneKind::Battlefield),
                to: None,
            },
            EffectDef::CreateToken {
                token: cards::BEAST_TOKEN_3_3_GREEN,
                count: ValueDef::Constant(1),
            },
        ),
    ]),
);

// M13 225 — Glacial Fortress
pub(in crate::card::sets) static GLACIAL_FORTRESS: CardRecord = CardRecord::new(
    cards::GLACIAL_FORTRESS,
    "Glacial Fortress",
    CardArt::new("bc9d29ee-1a21-4c3e-99c1-f815d40e8f19", "Franz Vohwinkel"),
    CardSet::Magic2013,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::check_land_enters(
            "This land enters tapped unless you control a Plains or an Island.",
            &[BasicLandType::Plains, BasicLandType::Island],
        ),
        AbilityDef::activated_mana(
            "{T}: Add {W} or {U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Blue,
            ])),
        ),
    ]),
);

// M13 228 — Rootbound Crag
pub(in crate::card::sets) static ROOTBOUND_CRAG: CardRecord = CardRecord::new(
    cards::ROOTBOUND_CRAG,
    "Rootbound Crag",
    CardArt::new("76364643-bfcb-4c50-9224-bf9e35648ddf", "Matt Stewart"),
    CardSet::Magic2013,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::check_land_enters(
            "This land enters tapped unless you control a Mountain or a Forest.",
            &[BasicLandType::Mountain, BasicLandType::Forest],
        ),
        AbilityDef::activated_mana(
            "{T}: Add {R} or {G}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Red,
                ManaColor::Green,
            ])),
        ),
    ]),
);

// M13 229 — Sunpetal Grove
pub(in crate::card::sets) static SUNPETAL_GROVE: CardRecord = CardRecord::new(
    cards::SUNPETAL_GROVE,
    "Sunpetal Grove",
    CardArt::new("15663129-9deb-4c34-84a0-f94cf1a723f0", "Jason Chan"),
    CardSet::Magic2013,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::check_land_enters(
            "This land enters tapped unless you control a Forest or a Plains.",
            &[BasicLandType::Forest, BasicLandType::Plains],
        ),
        AbilityDef::activated_mana(
            "{T}: Add {G} or {W}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Green,
                ManaColor::White,
            ])),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &OBLIVION_RING,
    &RHOX_FAITHMENDER,
    &WAR_PRIEST_OF_THUNE,
    &AUGUR_OF_BOLAS,
    &ESSENCE_SCATTER,
    &JACE_MEMORY_ADEPT,
    &NEGATE,
    &DISCIPLE_OF_BOLAS,
    &DURESS,
    &MUTILATE,
    &SIGN_IN_BLOOD,
    &VAMPIRE_NIGHTHAWK,
    &FLAMES_OF_THE_FIREBRAND,
    &THUNDERMAW_HELLKITE,
    &VOLCANIC_STRENGTH,
    &ARBOR_ELF,
    &FLINTHOOF_BOAR,
    &THRAGTUSK,
    &GLACIAL_FORTRESS,
    &ROOTBOUND_CRAG,
    &SUNPETAL_GROVE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
