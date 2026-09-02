//! Phyrexia: All Will Be One cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::CostQuantityDef;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, CardArt, CardRules, CardSet, CardSupertype,
    CardType, ChoiceVisibilityDef, ChooseOneOfEachDef, EffectDef, EffectRecipientDef,
    MoveObjectsDef, ObjectPredicateDef, ObjectSetDef, PlayerRefDef, RandomizeObjectOrderDef,
    RevealObjectsDef, SacrificedAmountDef, SpellAdditionalCostDef, ValueDef, ZoneKind,
    ZonePlacement, abilities,
};
use crate::{Binding, ParentBinding, mana_cost};

// ONE 28 — Planar Disruption
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLANAR_DISRUPTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8ee69a1f-aeed-4eb4-8987-fa720fc99715"),
    "Planar Disruption",
    crate::card::CardArt::new("8ee69a1f-aeed-4eb4-8987-fa720fc99715", "Campbell White"),
    crate::card::CardSet::PhyrexiaAllWillBeOne,
    crate::card::CardRules::unsupported(),
);

// ONE 80 — Annihilating Glare
pub(in crate::card::sets) static ANNIHILATING_GLARE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("be5d0b95-ec12-4e8e-99a0-7aca457f9107"),
    "Annihilating Glare",
    CardArt::new("be5d0b95-ec12-4e8e-99a0-7aca457f9107", "Konstantin Porubov"),
    CardSet::PhyrexiaAllWillBeOne,
    CardRules::new_sorcery(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_additional_cost(
        "As an additional cost to cast this spell, pay {4} or sacrifice an artifact or \
             creature.\nDestroy target creature or planeswalker.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasType(CardType::Planeswalker),
            ]),
        )],
        SpellAdditionalCostDef::choice(&[
            SpellAdditionalCostDef::pay_mana(mana_cost!("{4}")),
            SpellAdditionalCostDef::sacrifice(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]),
                CostQuantityDef::Fixed(1),
            ),
        ]),
        EffectDef::destroy_target(crate::TargetIndex::PRIMARY, true),
    )),
);

// ONE 108 — Sheoldred's Edict
/// "Of their choice", which is what makes it an edict: the sacrifice is
/// theirs to make, so hexproof and protection never come into it.
const fn edict(text: &'static str, object: ObjectPredicateDef) -> AbilityDef {
    AbilityDef::spell(
        text,
        EffectDef::SacrificeOfChoice {
            player: EffectRecipientDef::Opponent,
            object,
            count: ValueDef::Constant(1),
            then: None,
            amount: SacrificedAmountDef::Power,
            otherwise: None,
            optional: false,
        },
    )
}

pub(in crate::card::sets) static SHEOLDRED_S_EDICT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a9225cc3-90f0-448f-a8d9-7c6c2796d077"),
    "Sheoldred's Edict",
    CardArt::new("a9225cc3-90f0-448f-a8d9-7c6c2796d077", "Helge C. Balzer"),
    CardSet::PhyrexiaAllWillBeOne,
    // Two mana at instant speed for the one creature a protected threat
    // cannot dodge, as long as it is the only one they have.
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            edict(
                "Each opponent sacrifices a nontoken creature of their choice.",
                // Three edicts in one card, and the split is what makes it an answer
                // rather than a gamble: the mode that names tokens leaves the real
                // creature alone, and the mode that names nontokens cannot be paid with a
                // Servo.
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                ]),
            ),
            edict(
                "Each opponent sacrifices a creature token of their choice.",
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Token,
                ]),
            ),
            edict(
                "Each opponent sacrifices a planeswalker of their choice.",
                ObjectPredicateDef::HasType(CardType::Planeswalker),
            ),
        ],
    )),
);

// ONE 121 — Barbed Batterfist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARBED_BATTERFIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("de1d02d1-91dc-47d6-bdbe-87602428abfb"),
    "Barbed Batterfist",
    crate::card::CardArt::new("de1d02d1-91dc-47d6-bdbe-87602428abfb", "Randy Gallegos"),
    crate::card::CardSet::PhyrexiaAllWillBeOne,
    crate::card::CardRules::unsupported(),
);

// ONE 133 — Furnace Strider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FURNACE_STRIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aa625ab0-1e79-4497-a5da-98fe1abfd024"),
    "Furnace Strider",
    crate::card::CardArt::new("aa625ab0-1e79-4497-a5da-98fe1abfd024", "Denis Zhbankov"),
    crate::card::CardSet::PhyrexiaAllWillBeOne,
    crate::card::CardRules::unsupported(),
);

// ONE 161 — Cankerbloom
pub(in crate::card::sets) static CANKERBLOOM: CardRecord = CardRecord::new_with_legacy_id(
    2292,
    "Cankerbloom",
    CardArt::new("89b39293-6f57-4294-85fc-c718bdbb4d40", "Nicholas Gregory"),
    CardSet::PhyrexiaAllWillBeOne,
    // A 3/2 for two that is also the artifact removal the deck was going to
    // have to find room for, which is the whole reason it is in a cube.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Phyrexian", "Fungus"], 3, 2).with_ability(
        AbilityDef::modal_activated(
            "{1}, Sacrifice this creature: Choose one —\n• Destroy target artifact.\n• Destroy \
             target enchantment.\n• Proliferate.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::SacrificeSource,
            ],
            // Two of the three answer something and the third answers nothing, which is
            // the point: a mode that only needs a counter on the board is what keeps
            // the card from being dead against a deck with no artifacts.
            &[
                AbilityDef::destroy_target("Destroy target artifact.", &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                )][0], true),
                AbilityDef::destroy_target("Destroy target enchantment.", &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                )][0], true),
                AbilityDef::spell(
                    "Proliferate. (Choose any number of permanents and/or players, then give each another \
                     counter of each kind already there.)",
                    EffectDef::Proliferate,
                ),
            ],
            1,
            1,
            false,
        ),
    ),
);

// ONE 164 — Contagious Vorrac
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONTAGIOUS_VORRAC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("18af2c85-e58f-4043-99d3-e90121348aca"),
    "Contagious Vorrac",
    crate::card::CardArt::new("18af2c85-e58f-4043-99d3-e90121348aca", "Maxime Minard"),
    crate::card::CardSet::PhyrexiaAllWillBeOne,
    crate::card::CardRules::unsupported(),
);

// ONE 196 — Atraxa, Grand Unifier
/// Ten cards face up, and one pick per card type from among them: an
/// artifact, a creature, an enchantment, an instant, a land, a planeswalker,
/// and a sorcery, each optional and each from what is left, so an artifact
/// creature taken as the artifact is no longer there to be the creature.
/// The printed reminder counts battle as an eighth type; nothing in this
/// engine is one, so the seven it has are the whole list. The rest go back
/// underneath in a random order, which is why the look is worth so much
/// less to the player who did it than the cards it kept.
const ATRAXA_CHOSEN: Binding = Binding!("atraxa_chosen");
const ATRAXA_REST: Binding = Binding!("atraxa_rest");
pub(in crate::card::sets) static ATRAXA_GRAND_UNIFIER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4a1f905f-1d55-4d02-9d24-e58070793d3f"),
    "Atraxa, Grand Unifier",
    crate::card::CardArt::new("4a1f905f-1d55-4d02-9d24-e58070793d3f", "Marta Nael"),
    crate::card::CardSet::PhyrexiaAllWillBeOne,
    // Seven mana across four colours for a 7/7 that blocks everything, gains
    // the life back, and refills the hand on the way in.
    CardRules::new_creature(mana_cost!("{3}{G}{W}{U}{B}"), &["Phyrexian", "Angel"], 7, 7)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            abilities::vigilance(),
            abilities::deathtouch(),
            abilities::lifelink(),
            abilities::enters_trigger(
                "When this creature enters, reveal the top ten cards of your library. For each card \
                 type, you may put a card of that type from among the revealed cards into your hand. Put \
                 the rest on the bottom of your library in a random order.",
                abilities::bind_top_cards_then(
                    PlayerRefDef::EffectController,
                    ValueDef::Constant(10),
                    &const {
                        EffectDef::Sequence(&[
                            EffectDef::RevealObjects(RevealObjectsDef {
                                input: ObjectSetDef::Binding(ParentBinding),
                                then: &EffectDef::None,
                            }),
                            EffectDef::ChooseOneOfEach(ChooseOneOfEachDef {
                                    actor: PlayerRefDef::EffectController,
                                    input: ObjectSetDef::Binding(ParentBinding),
                                    predicates: &const {
                                        [
                                            ObjectPredicateDef::HasType(CardType::Artifact),
                                            ObjectPredicateDef::HasType(CardType::Creature),
                                            ObjectPredicateDef::HasType(CardType::Enchantment),
                                            ObjectPredicateDef::HasType(CardType::Instant),
                                            ObjectPredicateDef::HasType(CardType::Land),
                                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                                            ObjectPredicateDef::HasType(CardType::Sorcery),
                                        ]
                                    },
                                    chosen: ATRAXA_CHOSEN,
                                    remainder: ATRAXA_REST,
                                    visibility: ChoiceVisibilityDef::Public,
                                    then: &const {
                                        EffectDef::Sequence(&[
                                            EffectDef::MoveObjects(MoveObjectsDef {
                                                input: ObjectSetDef::Binding(ATRAXA_CHOSEN),
                                                from: Some(ZoneKind::Library),
                                                zone: ZoneKind::Hand,
                                                placement: ZonePlacement::Top,
                                                moved: None,
                                                then: &EffectDef::None,
                                            }),
                                                EffectDef::RandomizeObjectOrder(
                                                    RandomizeObjectOrderDef {
                                                        input: ObjectSetDef::Binding(ATRAXA_REST),
                                                        randomized: ParentBinding,
                                                        then: &EffectDef::MoveObjects(
                                                            MoveObjectsDef {
                                                                input: ObjectSetDef::Binding(
                                                                    ParentBinding,
                                                                ),
                                                                from: Some(ZoneKind::Library),
                                                                zone: ZoneKind::Library,
                                                                placement: ZonePlacement::Bottom,
                                                                moved: None,
                                                                then: &EffectDef::None,
                                                            },
                                                        ),
                                                    },
                                                )
                                        ])
                                    },
                            }),
                        ])
                    },
                ),
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &PLANAR_DISRUPTION,
    &ANNIHILATING_GLARE,
    &SHEOLDRED_S_EDICT,
    &BARBED_BATTERFIST,
    &FURNACE_STRIDER,
    &CANKERBLOOM,
    &CONTAGIOUS_VORRAC,
    &ATRAXA_GRAND_UNIFIER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
