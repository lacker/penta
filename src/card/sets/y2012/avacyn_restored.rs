//! Avacyn Restored card records used by the built-in ISD–RTR Standard deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, CardArt, CardBehavior, CardRules, CardSet, CardSupertype, CardType,
    CountConditionDef, EffectDef, EffectDurationDef, EffectRecipientDef, LibraryPlacement,
    ManaColor, ManaRestrictionDef, ManaSpendEffectDef, ObjectPredicateDef, ObjectQueryDef,
    PlayerRelation, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, abilities, cards,
};
use crate::{TargetIndex, mana_cost};

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

static CAVERN_COLORED_MANA_RESTRICTIONS: [ManaRestrictionDef; 1] =
    [ManaRestrictionDef::CastCreatureSpellOfChosenType];

static CAVERN_COLORED_MANA_SPEND_EFFECTS: [ManaSpendEffectDef; 1] =
    [ManaSpendEffectDef::ApplyToPaidSpell(
        AppliedEffectDef::CannotBeCountered,
    )];

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
                AddManaEffectDef::choice(&[
                    ManaColor::White,
                    ManaColor::Blue,
                    ManaColor::Black,
                    ManaColor::Red,
                    ManaColor::Green,
                ])
                .with_restrictions(&CAVERN_COLORED_MANA_RESTRICTIONS)
                .with_spend_effects(&CAVERN_COLORED_MANA_SPEND_EFFECTS),
            ),
        ),
    ]),
);

/// One Demon when its controller has exactly one creature, none otherwise.
static EXACTLY_ONE_CREATURE: CountConditionDef = CountConditionDef {
    query: ObjectQueryDef {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: PlayerRelation::You,
    },
    equals: 1,
    then: ValueDef::Constant(1),
    otherwise: ValueDef::Constant(0),
};

pub(in crate::card::sets) static DEMONIC_RISING: CardRecord = CardRecord::new(
    cards::DEMONIC_RISING,
    "Demonic Rising",
    CardArt::new("a2136a82-b535-47f6-9eee-5b7585ac5cf1", "Trevor Claxton"),
    CardSet::AvacynRestored,
    CardRules::new_enchantment(mana_cost!("{3}{B}{B}")).with_ability(
        AbilityDef::triggered(
            "At the beginning of your end step, if you control exactly one creature, create a 5/5 black Demon creature token with flying.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            // The intervening if becomes a count: one Demon when the
            // condition holds, none when it does not.
            EffectDef::CreateToken {
                token: cards::DEMON_TOKEN_5_5_BLACK,
                count: ValueDef::IfMatchingObjectCount(&EXACTLY_ONE_CREATURE),
            },
        ),
    ),
);

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
            ]))),
    ]),
);

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
                placement: LibraryPlacement::Bottom,
            },
        ),
        abilities::miracle(mana_cost!("{W}")),
    ]),
);

/// Haste matters here because the permanent has not been under its new
/// controller's control since the turn began.
static HASTE_GRANT: AbilityDef = abilities::haste();

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
pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &BONFIRE_OF_THE_DAMNED,
    &CAVERN_OF_SOULS,
    &DEMONIC_RISING,
    &PILLAR_OF_FLAME,
    &RESTORATION_ANGEL,
    &SIGARDA_HOST_OF_HERONS,
    &TERMINUS,
    &ZEALOUS_CONSCRIPTS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
