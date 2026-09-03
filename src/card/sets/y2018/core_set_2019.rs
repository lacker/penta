//! M19 card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityPredicateDef, AbilityTargetDef, AbilityTargetPredicate,
    AddManaEffectDef, AppliedEffectDef, CardArt, CardRules, CardSet, CardType,
    CharacteristicOperationDef, EffectDef, EffectRecipientDef, ExilePlayDurationDef, LAND_SUBTYPES,
    ObjectPredicateDef, PlayerRelation, ResolvedEffectDurationDef, SetOperationDef, ValueDef,
    ZoneKind, abilities,
};
use crate::{TargetIndex, mana_cost};

// M19 29 — Militia Bugler
pub(in crate::card::sets) static MILITIA_BUGLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("43c5bf25-937c-4e17-9ed4-b4c4579fa9dc"),
    "Militia Bugler",
    CardArt::new("43c5bf25-937c-4e17-9ed4-b4c4579fa9dc", "David Gaillet"),
    CardSet::CoreSet2019,
    // The power restriction is what keeps this honest: it finds the small
    // creatures a white deck is already full of, and none of the payoffs.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 2, 3).with_abilities(&[
        abilities::vigilance(),
        abilities::enters_trigger(
            "When this creature enters, look at the top four cards of your library. You may \
             reveal a creature card with power 2 or less from among them and put it into your \
             hand. Put the rest on the bottom of your library in a random order.",
            abilities::look_at_top_cards_reveal_choice_to_hand_rest_random_bottom(
                ValueDef::Constant(4),
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    // "Power 2 or less" has to be written as a strict
                    // comparison because power only reads upward here.
                    ObjectPredicateDef::PowerLessThan(ValueDef::Constant(3)),
                ]),
                0,
                1,
            ),
        ),
    ]),
);

// M19 125 — Vampire Sovereign
pub(in crate::card::sets) static VAMPIRE_SOVEREIGN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ee338221-ead9-4b89-8b0c-12745c4ca13d"),
    "Vampire Sovereign",
    CardArt::new("ee338221-ead9-4b89-8b0c-12745c4ca13d", "Volkan Baǵa"),
    CardSet::CoreSet2019,
    // A six-point swing attached to a flier, which is what makes five mana
    // a fair price in a format where the race is the game.
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Vampire", "Noble"], 3, 4).with_abilities(
        &[
            abilities::flying(),
            abilities::enters_trigger_with_targets(
                "When this creature enters, target opponent loses 3 life and you gain 3 life.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::Sequence(&[
                    EffectDef::LoseLife {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(3),
                    },
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(3),
                    },
                ]),
            ),
        ],
    ),
);

// M19 128 — Alpine Moon
pub(in crate::card::sets) static ALPINE_MOON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2435c810-2baf-4e3b-80ce-542b94694901"),
    "Alpine Moon",
    crate::card::CardArt::new("2435c810-2baf-4e3b-80ce-542b94694901", "Alayna Danner"),
    crate::card::CardSet::CoreSet2019,
    CardRules::new_enchantment(mana_cost!("{R}")).with_abilities(&[
        abilities::choose_card_name_as_enters(
            "As this enchantment enters, choose a nonbasic land card name.",
            crate::card::BattlefieldEntryScalarChoiceDef::NONBASIC_LAND_CARD_NAME,
        ),
        AbilityDef::static_ability(
            "Lands your opponents control with the chosen name lose all land types and abilities, and they gain \"{T}: Add one mana of any color.\"",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::NameEquals(
                            crate::card::CardNameDef::SourceChoice,
                        ),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Opponent,
                ),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::Characteristic(CharacteristicOperationDef::Subtypes(
                        SetOperationDef::Remove(LAND_SUBTYPES),
                    )),
                    AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any),
                    AppliedEffectDef::add_ability(&AbilityDef::activated_mana(
                        "{T}: Add one mana of any color.",
                        &[AbilityCostDef::TapSource],
                        EffectDef::AddMana(AddManaEffectDef::any_color()),
                    )),
                ]),
            },
        ),
    ]),
);

// M19 134 — Dark-Dweller Oracle
pub(in crate::card::sets) static DARK_DWELLER_ORACLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("69a57bfc-1de2-4b3a-84bc-19ec41087f0d"),
    "Dark-Dweller Oracle",
    CardArt::new(
        "69a57bfc-1de2-4b3a-84bc-19ec41087f0d",
        "Deruchenko Alexander",
    ),
    CardSet::CoreSet2019,
    // A sacrifice outlet that turns each body into a look at the top card,
    // and it can eat itself once the board is empty.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Shaman"], 2, 2).with_ability(
        AbilityDef::activated(
            "{1}, Sacrifice a creature: Exile the top card of your library. You may play that card this turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::ExileTopOfLibraryToPlay {
                player: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
                // "You may play that card", not play it for free: the Oracle
                // still charges for whatever it turns up.
                free: false,
                face_down: false,
                duration: ExilePlayDurationDef::ThisTurn,
                spend_any_color: false,
                play_condition: None,
                cast_only: false,
            },
        ),
    ),
);

// M19 143 — Goblin Motivator
pub(in crate::card::sets) static GOBLIN_MOTIVATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("94b3a4fb-9024-45ef-a54b-cf3a9fa5b9c2"),
    "Goblin Motivator",
    CardArt::new("94b3a4fb-9024-45ef-a54b-cf3a9fa5b9c2", "Johann Bodin"),
    CardSet::CoreSet2019,
    // Any creature, not only yours, though the haste is only worth giving
    // to something that just arrived on your own side.
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin", "Warrior"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Target creature gains haste until end of turn.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::haste()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &MILITIA_BUGLER,
    &VAMPIRE_SOVEREIGN,
    &ALPINE_MOON,
    &DARK_DWELLER_ORACLE,
    &GOBLIN_MOTIVATOR,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
