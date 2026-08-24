//! Innistrad: Crimson Vow cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, ActivationTimingDef,
    AppliedEffectDef, CardArt, CardRules, CardSet, CardType, ChoiceVisibilityDef, ChooseDef,
    EffectDef, EffectRecipientDef, ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef,
    ObjectRefDef, ObjectSetDef, PlayerRefDef, PlayerRelation, PlayerSetDef, TriggerEventDef,
    ValueDef, ZoneKind, abilities,
};
use crate::ids::{ObjectBindingIndex, ObjectSetBindingIndex, TargetIndex};
use crate::mana_cost;

// VOW 55 — Cruel Witness
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CRUEL_WITNESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5bf2c686-efb0-46c7-b34e-c77987914b96"),
    "Cruel Witness",
    crate::card::CardArt::new("5bf2c686-efb0-46c7-b34e-c77987914b96", "Vincent Proce"),
    crate::card::CardSet::InnistradCrimsonVow,
    crate::card::CardRules::unsupported(),
);

// VOW 95 — Blood Fountain
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLOOD_FOUNTAIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dd03651e-ada0-41dc-8722-0eba476943e3"),
    "Blood Fountain",
    crate::card::CardArt::new("dd03651e-ada0-41dc-8722-0eba476943e3", "Evyn Fong"),
    crate::card::CardSet::InnistradCrimsonVow,
    crate::card::CardRules::unsupported(),
);

// VOW 101 — Concealing Curtains // Revealing Eye
/// What the Eye does with the card it picked. Written as a walk over the
/// chosen set rather than a plain sequence, because "if you do" gates the
/// draw as well as the discard: an Eye that looked and took nothing leaves
/// the opponent with the hand they had.
static REVEALING_EYE_TAKE_IT: EffectDef = EffectDef::Sequence(&[
    EffectDef::DiscardCards {
        object: EffectRecipientDef::object(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
    },
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: ValueDef::Constant(1),
    },
]);

static REVEALING_EYE_CHOSEN: EffectDef = EffectDef::ForEachInBinding {
    objects: ObjectSetBindingIndex::PRIMARY,
    binding: ObjectBindingIndex::PRIMARY,
    effect: &REVEALING_EYE_TAKE_IT,
};

/// "You may choose a nonland card from it": a choice of none is a legal
/// answer, which is why the minimum is zero rather than one.
static REVEALING_EYE_EFFECT: [EffectDef; 2] = [
    EffectDef::RevealHand {
        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::Choose(ChooseDef {
        binding: ObjectChoiceBindingDef::Objects(ObjectSetBindingIndex::PRIMARY),
        unchosen: None,
        chooser: PlayerRefDef::EffectController,
        candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
            &[ZoneKind::Hand],
            PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
        )),
        exclude: None,
        minimum: 0,
        maximum: 1,
        visibility: ChoiceVisibilityDef::Public,
        then: &REVEALING_EYE_CHOSEN,
    }),
];

static AN_OPPONENT: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
)];

static CURTAINS_ABILITIES: [AbilityDef; 2] = [
    abilities::defender(),
    AbilityDef::activated(
        "{2}{B}: Transform this creature. Activate only as a sorcery.",
        &[AbilityCostDef::Mana(mana_cost!("{2}{B}"))],
        EffectDef::Transform {
            object: EffectRecipientDef::Source,
        },
    )
    .with_activation_timing(ActivationTimingDef::SorcerySpeed),
];

static REVEALING_EYE_ABILITIES: [AbilityDef; 2] = [
    abilities::menace(),
    AbilityDef::triggered_with_targets(
        "When this creature transforms into Revealing Eye, target opponent reveals their hand. \
         You may choose a nonland card from it. If you do, that player discards that card, then \
         draws a card.",
        TriggerEventDef::transforms(ObjectPredicateDef::Source),
        &AN_OPPONENT,
        EffectDef::Sequence(&REVEALING_EYE_EFFECT),
    ),
];

const fn concealing_curtains_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{B}"), &["Wall"], 0, 4).with_abilities(&CURTAINS_ABILITIES)
}

const fn revealing_eye_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Eye", "Horror"], 3, 4)
        .with_abilities(&REVEALING_EYE_ABILITIES)
}

pub(in crate::card::sets) static CONCEALING_CURTAINS: CardRecord = CardRecord::new_dfc(
    PrintingAnchor::scryfall("612b2e6e-fe8d-49ad-b845-6fa7fa59ffd1"),
    "Concealing Curtains // Revealing Eye",
    CardArt::new("612b2e6e-fe8d-49ad-b845-6fa7fa59ffd1", "Brian Valeza"),
    CardSet::InnistradCrimsonVow,
    &[
        ("Concealing Curtains", concealing_curtains_rules()),
        ("Revealing Eye", revealing_eye_rules()),
    ],
);

// VOW 174 — Reckless Impulse
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RECKLESS_IMPULSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6943c07f-ab0d-4f5a-bbe9-c0a83dc98546"),
    "Reckless Impulse",
    crate::card::CardArt::new("6943c07f-ab0d-4f5a-bbe9-c0a83dc98546", "Mathias Kollros"),
    crate::card::CardSet::InnistradCrimsonVow,
    crate::card::CardRules::unsupported(),
);

// VOW 182 — Voldaren Epicure
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VOLDAREN_EPICURE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ae154e64-f626-45fb-bd52-840c1c27b2d3"),
    "Voldaren Epicure",
    crate::card::CardArt::new("ae154e64-f626-45fb-bd52-840c1c27b2d3", "Martina Fačková"),
    crate::card::CardSet::InnistradCrimsonVow,
    crate::card::CardRules::unsupported(),
);

// VOW 189 — Bramble Wurm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BRAMBLE_WURM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8f16f137-4ceb-469c-a381-e575d58f456b"),
    "Bramble Wurm",
    crate::card::CardArt::new("8f16f137-4ceb-469c-a381-e575d58f456b", "Lars Grant-West"),
    crate::card::CardSet::InnistradCrimsonVow,
    crate::card::CardRules::unsupported(),
);

// VOW 225 — Ulvenwald Oddity // Ulvenwald Behemoth
static ODDITY_TRAMPLE: AbilityDef = abilities::trample();

static ODDITY_HASTE: AbilityDef = abilities::haste();

/// What the back face hands the rest of the board. The keywords are the ones
/// it already has, which is the joke: the 8/8 makes everything else look
/// like a smaller version of itself.
static BEHEMOTH_GRANT: [AppliedEffectDef; 3] = [
    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
    AppliedEffectDef::add_ability(&ODDITY_TRAMPLE),
    AppliedEffectDef::add_ability(&ODDITY_HASTE),
];

/// "Other creatures you control", which excludes the Behemoth itself: it
/// already has both keywords and does not need the counters.
static OTHER_CREATURES_YOU_CONTROL: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

static ODDITY_ABILITIES: [AbilityDef; 3] = [
    abilities::trample(),
    abilities::haste(),
    AbilityDef::activated(
        "{5}{G}{G}: Transform this creature.",
        &[AbilityCostDef::Mana(mana_cost!("{5}{G}{G}"))],
        EffectDef::Transform {
            object: EffectRecipientDef::Source,
        },
    ),
];

static BEHEMOTH_ABILITIES: [AbilityDef; 3] = [
    abilities::trample(),
    abilities::haste(),
    AbilityDef::static_ability(
        "Other creatures you control get +1/+1 and have trample and haste.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                OTHER_CREATURES_YOU_CONTROL,
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::Composite(&BEHEMOTH_GRANT),
        },
    ),
];

const fn ulvenwald_oddity_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Beast"], 4, 4)
        .with_abilities(&ODDITY_ABILITIES)
}

const fn ulvenwald_behemoth_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Beast", "Horror"], 8, 8)
        .with_abilities(&BEHEMOTH_ABILITIES)
}

pub(in crate::card::sets) static ULVENWALD_ODDITY: CardRecord = CardRecord::new_dfc(
    PrintingAnchor::scryfall("5fdf5fc4-69c8-4a59-9095-c2feefb64371"),
    "Ulvenwald Oddity // Ulvenwald Behemoth",
    CardArt::new("5fdf5fc4-69c8-4a59-9095-c2feefb64371", "Brent Hollowell"),
    CardSet::InnistradCrimsonVow,
    &[
        ("Ulvenwald Oddity", ulvenwald_oddity_rules()),
        ("Ulvenwald Behemoth", ulvenwald_behemoth_rules()),
    ],
);

// VOW 310 — Bloodtithe Harvester
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLOODTITHE_HARVESTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("01182501-2b50-4b87-835a-fea3c5e6e330"),
    "Bloodtithe Harvester",
    crate::card::CardArt::new("01182501-2b50-4b87-835a-fea3c5e6e330", "Sami Makkonen"),
    crate::card::CardSet::InnistradCrimsonVow,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &CRUEL_WITNESS,
    &BLOOD_FOUNTAIN,
    &CONCEALING_CURTAINS,
    &RECKLESS_IMPULSE,
    &VOLDAREN_EPICURE,
    &BRAMBLE_WURM,
    &ULVENWALD_ODDITY,
    &BLOODTITHE_HARVESTER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
