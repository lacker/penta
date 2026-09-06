//! Innistrad: Crimson Vow cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, ActivationTimingDef, AppliedEffectDef,
    CardArt, CardRules, CardSet, CardType, ChoiceVisibilityDef, ChooseDef, CostDef, EffectDef,
    EffectRecipientDef, ExilePlayDurationDef, ManaColor, ObjectChoiceBindingDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef, PlayerRefDef, PlayerRelation,
    PlayerSetDef, ResolvedEffectDurationDef, ScaledValueDef, TriggerEventDef, ValueDef, ZoneKind,
    ZonePlacement, abilities, tokens,
};
use crate::ids::{ParentBinding, TargetIndex};
use crate::mana_cost;

// VOW 55 — Cruel Witness
pub(in crate::card::sets) static CRUEL_WITNESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5bf2c686-efb0-46c7-b34e-c77987914b96"),
    "Cruel Witness",
    CardArt::new("5bf2c686-efb0-46c7-b34e-c77987914b96", "Vincent Proce"),
    CardSet::InnistradCrimsonVow,
    // A four-mana flier that also fixes every draw afterwards, in a deck
    // already casting the spells that turn it on.
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Bird", "Horror"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever you cast a noncreature spell, surveil 1.",
            // On the cast rather than the resolution, so a countered spell
            // has already paid for its surveil.
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::NoncreatureSpell,
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            abilities::surveil(ValueDef::Constant(1)),
        ),
    ]),
);

// VOW 95 — Blood Fountain
pub(in crate::card::sets) static BLOOD_FOUNTAIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dd03651e-ada0-41dc-8722-0eba476943e3"),
    "Blood Fountain",
    CardArt::new("dd03651e-ada0-41dc-8722-0eba476943e3", "Evyn Fong"),
    CardSet::InnistradCrimsonVow,
    // One mana smooths the draw now; the same card buys back two creatures
    // later, which is why a graveyard deck runs it over a plain rummage.
    CardRules::new_artifact(mana_cost!("{B}")).with_abilities(&[
        abilities::enters_trigger(
            "When this artifact enters, create a Blood token. (It's an artifact with \"{1}, {T}, \
             Discard a card, Sacrifice this token: Draw a card.\")",
            EffectDef::create_token(tokens::blood()),
        ),
        AbilityDef::activated_with_targets(
            "{3}{B}, {T}, Sacrifice this artifact: Return up to two target creature cards from \
             your graveyard to your hand.",
            &[
                CostDef::Mana(mana_cost!("{3}{B}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            // One slot holding up to two, so a graveyard with a single
            // creature in it still activates.
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
                2,
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// VOW 101 — Concealing Curtains // Revealing Eye
pub(in crate::card::sets) static CONCEALING_CURTAINS: CardRecord = CardRecord::new_dfc(
    PrintingAnchor::scryfall("612b2e6e-fe8d-49ad-b845-6fa7fa59ffd1"),
    "Concealing Curtains // Revealing Eye",
    CardArt::new("612b2e6e-fe8d-49ad-b845-6fa7fa59ffd1", "Brian Valeza"),
    CardSet::InnistradCrimsonVow,
    &[
        (
            "Concealing Curtains",
            const {
                CardRules::new_creature(mana_cost!("{B}"), &const { ["Wall"] }, 0, 4)
                    .with_abilities(
                        &const {
                            [
                                abilities::defender(),
                                AbilityDef::activated(
                                    "{2}{B}: Transform this creature. Activate only as a sorcery.",
                                    &const { [CostDef::Mana(mana_cost!("{2}{B}"))] },
                                    EffectDef::Transform {
                                        object: EffectRecipientDef::Source,
                                    },
                                )
                                .with_activation_timing(ActivationTimingDef::SorcerySpeed),
                            ]
                        },
                    )
            },
        ),
        (
            "Revealing Eye",
            const {
                CardRules::new_creature_without_mana_cost(&const { ["Eye", "Horror"] }, 3, 4)
                    // A back face has no mana cost to read a colour off; the
                    // colour indicator is what says she is still black.
                    .printed_colors(&const { [ManaColor::Black] })
                .with_abilities(&const { [
                    abilities::menace(),
                    AbilityDef::triggered_with_targets(
                        "When this creature transforms into Revealing Eye, target opponent reveals their hand. \
                         You may choose a nonland card from it. If you do, that player discards that card, then \
                         draws a card.",
                        TriggerEventDef::transforms(ObjectPredicateDef::Source),
                        &const { [AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                        )] },
                        // "You may choose a nonland card from it": a choice of none is a legal
                        // answer, which is why the minimum is zero rather than one.
                        EffectDef::Sequence(&const { [
                            EffectDef::RevealHand {
                                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            },
                            EffectDef::Choose(ChooseDef {
                                binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                                unchosen: None,
                                chooser: PlayerRefDef::EffectController,
                                candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
                                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                                    &const { [ZoneKind::Hand] },
                                    PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                                )),
                                exclude: None,
                                minimum: 0,
                                maximum: 1,
                                visibility: ChoiceVisibilityDef::Public,
                                then: &EffectDef::ForEachInBinding {
                                    objects: ParentBinding,
                                    binding: ParentBinding,
                                    // What the Eye does with the card it picked. Written as a walk over the
                                    // chosen set rather than a plain sequence, because "if you do" gates the
                                    // draw as well as the discard: an Eye that looked and took nothing leaves
                                    // the opponent with the hand they had.
                                    effect: &EffectDef::Sequence(&const { [
                                        EffectDef::DiscardCards {
                                            object: EffectRecipientDef::object(ObjectRefDef::Binding(ParentBinding)),
                                        },
                                        EffectDef::DrawCards {
                                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                            amount: ValueDef::Constant(1),
                                        },
                                    ] }),
                                },
                            }),
                        ] }),
                    ),
                ] })
            },
        ),
    ],
);

// VOW 174 — Reckless Impulse
pub(in crate::card::sets) static RECKLESS_IMPULSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6943c07f-ab0d-4f5a-bbe9-c0a83dc98546"),
    "Reckless Impulse",
    CardArt::new("6943c07f-ab0d-4f5a-bbe9-c0a83dc98546", "Mathias Kollros"),
    CardSet::InnistradCrimsonVow,
    // Two cards for two mana in a colour that does not draw them. The extra
    // turn is what makes it a real two-for-one: a red deck casting this on
    // three still has the mana to spend both halves before they lapse.
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell(
        "Exile the top two cards of your library. Until the end of your next turn, you may play \
         those cards.",
        EffectDef::ExileTopOfLibraryToPlay {
            player: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(2),
            // "You may play those cards", not "without paying their mana
            // costs": the cards are still bought at full price.
            free: false,
            face_down: false,
            duration: ExilePlayDurationDef::UntilEndOfYourNextTurn,
            spend_any_color: false,
            play_condition: None,
            cast_only: false,
        },
    )),
);

// VOW 182 — Voldaren Epicure
pub(in crate::card::sets) static VOLDAREN_EPICURE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ae154e64-f626-45fb-bd52-840c1c27b2d3"),
    "Voldaren Epicure",
    CardArt::new("ae154e64-f626-45fb-bd52-840c1c27b2d3", "Martina Fačková"),
    CardSet::InnistradCrimsonVow,
    // One mana for a body, a point of damage, and a card the Blood turns a
    // dead draw into later.
    CardRules::new_creature(mana_cost!("{R}"), &["Vampire"], 1, 1).with_ability(
        abilities::enters_trigger(
            "When this creature enters, it deals 1 damage to each opponent. Create a Blood token. \
             (It's an artifact with \"{1}, {T}, Discard a card, Sacrifice this token: Draw a \
             card.\")",
            // One clause with two instructions in the order it prints them: the damage
            // is the reason the one-drop is played and the Blood is what it leaves
            // behind.
            EffectDef::Sequence(&[
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::create_token(crate::card::tokens::blood()),
            ]),
        ),
    ),
);

// VOW 189 — Bramble Wurm
pub(in crate::card::sets) static BRAMBLE_WURM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8f16f137-4ceb-469c-a381-e575d58f456b"),
    "Bramble Wurm",
    CardArt::new("8f16f137-4ceb-469c-a381-e575d58f456b", "Lars Grant-West"),
    CardSet::InnistradCrimsonVow,
    // Seven mana is more than most decks reach, so the graveyard half is
    // what the card usually does: five life for three, from the bin.
    CardRules::new_creature(mana_cost!("{6}{G}"), &["Wurm"], 7, 6).with_abilities(&[
        abilities::reach(),
        abilities::trample(),
        abilities::enters_trigger(
            "When this creature enters, you gain 5 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(5),
            },
        ),
        AbilityDef::activated(
            "{2}{G}, Exile this card from your graveyard: You gain 5 life.",
            &[CostDef::Mana(mana_cost!("{2}{G}")), CostDef::ExileSource],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(5),
            },
        )
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// VOW 225 — Ulvenwald Oddity // Ulvenwald Behemoth
pub(in crate::card::sets) static ULVENWALD_ODDITY: CardRecord = CardRecord::new_dfc(
    PrintingAnchor::scryfall("5fdf5fc4-69c8-4a59-9095-c2feefb64371"),
    "Ulvenwald Oddity // Ulvenwald Behemoth",
    CardArt::new("5fdf5fc4-69c8-4a59-9095-c2feefb64371", "Brent Hollowell"),
    CardSet::InnistradCrimsonVow,
    &[
        (
            "Ulvenwald Oddity",
            const {
                CardRules::new_creature(mana_cost!("{2}{G}{G}"), &const { ["Beast"] }, 4, 4)
                    .with_abilities(
                        &const {
                            [
                                abilities::trample(),
                                abilities::haste(),
                                AbilityDef::activated(
                                    "{5}{G}{G}: Transform this creature.",
                                    &const { [CostDef::Mana(mana_cost!("{5}{G}{G}"))] },
                                    EffectDef::Transform {
                                        object: EffectRecipientDef::Source,
                                    },
                                ),
                            ]
                        },
                    )
            },
        ),
        (
            "Ulvenwald Behemoth",
            const {
                CardRules::new_creature_without_mana_cost(&const { ["Beast", "Horror"] }, 8, 8)
                    // Same again: the indicator keeps the Behemoth green.
                    .printed_colors(&const { [ManaColor::Green] })
                .with_abilities(&const { [
                    abilities::trample(),
                    abilities::haste(),
                    AbilityDef::static_ability(
                        "Other creatures you control get +1/+1 and have trample and haste.",
                        EffectDef::StaticApply {
                            recipient: EffectRecipientDef::matching_objects(
                                // "Other creatures you control", which excludes the Behemoth itself: it
                                // already has both keywords and does not need the counters.
                                ObjectPredicateDef::All(&const { [
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                                ] }),
                                &const { [ZoneKind::Battlefield] },
                                PlayerRelation::You,
                            ),
                            // What the back face hands the rest of the board. The keywords are the ones
                            // it already has, which is the joke: the 8/8 makes everything else look
                            // like a smaller version of itself.
                            effect: AppliedEffectDef::Composite(&const { [
                                AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
                                AppliedEffectDef::add_ability(&const { abilities::trample() }),
                                AppliedEffectDef::add_ability(&const { abilities::haste() }),
                            ] }),
                        },
                    ),
                ] })
            },
        ),
    ],
);

// VOW 310 — Bloodtithe Harvester
/// Twice the count, and downward. Two Blood is -4/-4, which is what makes
/// the second token worth keeping around rather than cashing in.
static HARVESTER_PENALTY: ValueDef = ValueDef::Scaled(&ScaledValueDef::new(
    // Nothing but a token carries the Blood artifact type, so naming it is
    // enough: the count is read as the ability resolves, and the token the
    // Harvester's own arrival made is one of them.
    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
        ObjectPredicateDef::Subtype("Blood"),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    )),
    -2,
));

pub(in crate::card::sets) static BLOODTITHE_HARVESTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("01182501-2b50-4b87-835a-fea3c5e6e330"),
    "Bloodtithe Harvester",
    crate::card::CardArt::new("01182501-2b50-4b87-835a-fea3c5e6e330", "Sami Makkonen"),
    crate::card::CardSet::InnistradCrimsonVow,
    // Two mana for a 3/2 that replaces itself with a card later, and can
    // instead be spent as removal the turn it stops attacking.
    CardRules::new_creature(mana_cost!("{B}{R}"), &["Vampire"], 3, 2)
        .with_abilities(&[
            abilities::enters_trigger(
                "When this creature enters, create a Blood token.",
                EffectDef::create_token(crate::card::tokens::blood()),
            ),
            // Sacrificing the Harvester is what pays for the removal, so the body
            // and the answer are the same card twice rather than both at once.
            AbilityDef::activated_with_targets(
                "{T}, Sacrifice this creature: Target creature gets -X/-X until end of turn, where X is \
                 twice the number of Blood tokens you control. Activate only as a sorcery.",
                &[CostDef::TapSource, CostDef::SacrificeSource],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(HARVESTER_PENALTY, HARVESTER_PENALTY),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            )
            .with_activation_timing(ActivationTimingDef::SorcerySpeed),
        ]),
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
