//! M19 card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityPredicateDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::CharacteristicOperationDef;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ExilePlayDurationDef;
use crate::card::LAND_SUBTYPES;
use crate::card::ObjectPredicateDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SetOperationDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "M19",
    slug: "core-set-2019",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// M19 13 — Herald of Faith
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HERALD_OF_FAITH: CardRecord = CardRecord::new(
    "Herald of Faith",
    "452591ca-7273-4e47-820b-3ff89697a036",
    "Tommy Arnold",
    crate::card::CardRules::unsupported(),
);

// M19 22 — Leonin Vanguard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEONIN_VANGUARD: CardRecord = CardRecord::new(
    "Leonin Vanguard",
    "724738ad-6a9b-4ef6-b637-558645cd8151",
    "Aaron Miller",
    crate::card::CardRules::unsupported(),
);

// M19 29 — Militia Bugler
pub(in crate::card::sets) static MILITIA_BUGLER: CardRecord = CardRecord::new(
    "Militia Bugler",
    "43c5bf25-937c-4e17-9ed4-b4c4579fa9dc",
    "David Gaillet",
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

// M19 34 — Resplendent Angel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RESPLENDENT_ANGEL: CardRecord = CardRecord::new(
    "Resplendent Angel",
    "586854d1-edfd-4c66-873d-df459324dbfd",
    "Volkan Baǵa",
    crate::card::CardRules::unsupported(),
);

// M19 55 — Exclusion Mage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXCLUSION_MAGE: CardRecord = CardRecord::new(
    "Exclusion Mage",
    "ccad82f5-5c5c-42ad-b66e-942f0d9631ca",
    "Chris Seaman",
    crate::card::CardRules::unsupported(),
);

// M19 63 — Mystic Archaeologist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MYSTIC_ARCHAEOLOGIST: CardRecord = CardRecord::new(
    "Mystic Archaeologist",
    "1b19cad0-5754-4625-8303-c8310bc7cbd5",
    "Eric Deschamps",
    crate::card::CardRules::unsupported(),
);

// M19 118 — Skeleton Archer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKELETON_ARCHER: CardRecord = CardRecord::new(
    "Skeleton Archer",
    "8fee5cc4-a686-4ce6-aa6b-1b8a88e6dea3",
    "Randy Vargas",
    crate::card::CardRules::unsupported(),
);

// M19 124 — Vampire Neonate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VAMPIRE_NEONATE: CardRecord = CardRecord::new(
    "Vampire Neonate",
    "167822a5-2ab5-42f5-afa4-562fe2d7501b",
    "Daarken",
    crate::card::CardRules::unsupported(),
);

// M19 125 — Vampire Sovereign
pub(in crate::card::sets) static VAMPIRE_SOVEREIGN: CardRecord = CardRecord::new(
    "Vampire Sovereign",
    "ee338221-ead9-4b89-8b0c-12745c4ca13d",
    "Volkan Baǵa",
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
    "Alpine Moon",
    "2435c810-2baf-4e3b-80ce-542b94694901",
    "Alayna Danner",
CardRules::new_enchantment(mana_cost!("{R}")).with_abilities(&[
        AbilityDef::as_enters(
            "As this enchantment enters, choose a nonbasic land card name.",
            crate::card::ReplacementEffectDef::BindOutput {
                binding: Binding!("alpine_moon_name"),
                effect: &abilities::choose_card_name_as_enters(
                    crate::card::CardNameSetDef::NonbasicLandCardNames,
                ),
            },
        ),
        AbilityDef::static_ability(
            "Lands your opponents control with the chosen name lose all land types and abilities, and they gain \"{T}: Add one mana of any color.\"",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::NameEquals(
                            crate::card::CardNameDef::Binding(Binding!("alpine_moon_name")),
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
                        &[CostDef::TapSource],
                        EffectDef::AddMana(AddManaEffectDef::any_color()),
                    )),
                ]),
            },
        ),
    ]),
);

// M19 134 — Dark-Dweller Oracle
pub(in crate::card::sets) static DARK_DWELLER_ORACLE: CardRecord = CardRecord::new(
    "Dark-Dweller Oracle",
    "69a57bfc-1de2-4b3a-84bc-19ec41087f0d",
    "Deruchenko Alexander",
// A sacrifice outlet that turns each body into a look at the top card,
    // and it can eat itself once the board is empty.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Shaman"], 2, 2).with_ability(
        AbilityDef::activated(
            "{1}, Sacrifice a creature: Exile the top card of your library. You may play that card this turn.",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::SacrificePermanent {
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
    "Goblin Motivator",
    "94b3a4fb-9024-45ef-a54b-cf3a9fa5b9c2",
    "Johann Bodin",
    // Any creature, not only yours, though the haste is only worth giving
    // to something that just arrived on your own side.
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin", "Warrior"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Target creature gains haste until end of turn.",
            &[CostDef::TapSource],
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

// M19 149 — Lathliss, Dragon Queen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LATHLISS_DRAGON_QUEEN: CardRecord = CardRecord::new(
    "Lathliss, Dragon Queen",
    "54a4c37d-5eeb-42c9-9688-c2ed0d5044cd",
    "Alex Konstad",
    crate::card::CardRules::unsupported(),
);

// M19 166 — Viashino Pyromancer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIASHINO_PYROMANCER: CardRecord = CardRecord::new(
    "Viashino Pyromancer",
    "a82fdb2f-b199-44ff-9615-90fc074cb8b0",
    "Jesper Ejsing",
    crate::card::CardRules::unsupported(),
);

// M19 168 — Volley Veteran
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOLLEY_VETERAN: CardRecord = CardRecord::new(
    "Volley Veteran",
    "164960fc-6e80-4a53-90d7-5a18c0a28083",
    "Craig J Spearing",
    crate::card::CardRules::unsupported(),
);

// M19 185 — Gigantosaurus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GIGANTOSAURUS: CardRecord = CardRecord::new(
    "Gigantosaurus",
    "c1db84d8-d426-4c0d-b44e-5be7b0f5f5bf",
    "Jonathan Kuo",
    crate::card::CardRules::unsupported(),
);

// M19 208 — Vivien Reid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIVIEN_REID: CardRecord = CardRecord::new(
    "Vivien Reid",
    "681fbd66-b622-4f20-a860-f101aff21109",
    "Anna Steinbauer",
    crate::card::CardRules::unsupported(),
);

// M19 217 — Heroic Reinforcements
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEROIC_REINFORCEMENTS: CardRecord = CardRecord::new(
    "Heroic Reinforcements",
    "33c35bf8-ae43-41aa-aae9-4d7513f9058c",
    "Scott Murphy",
    crate::card::CardRules::unsupported(),
);

// M19 231 — Diamond Mare
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIAMOND_MARE: CardRecord = CardRecord::new(
    "Diamond Mare",
    "ca600b3f-2c70-489b-b218-6e3245b90114",
    "Alayna Danner",
    crate::card::CardRules::unsupported(),
);

// M19 241 — Meteor Golem
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static METEOR_GOLEM: CardRecord = CardRecord::new(
    "Meteor Golem",
    "1bdb0b15-d651-4730-8be9-d0e01145311b",
    "Lake Hurwitz",
    crate::card::CardRules::unsupported(),
);

// M19 297 — Kargan Dragonrider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KARGAN_DRAGONRIDER: CardRecord = CardRecord::new(
    "Kargan Dragonrider",
    "34750304-c536-47d4-922d-a3654c37ffbc",
    "Greg Opalinski",
    crate::card::CardRules::unsupported(),
);

// M19 302 — Aggressive Mammoth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGGRESSIVE_MAMMOTH: CardRecord = CardRecord::new(
    "Aggressive Mammoth",
    "323f3c76-5e79-43e6-ae78-f555810edbc3",
    "Filip Burburan",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &HERALD_OF_FAITH,
    &LEONIN_VANGUARD,
    &MILITIA_BUGLER,
    &RESPLENDENT_ANGEL,
    &EXCLUSION_MAGE,
    &MYSTIC_ARCHAEOLOGIST,
    &SKELETON_ARCHER,
    &VAMPIRE_NEONATE,
    &VAMPIRE_SOVEREIGN,
    &ALPINE_MOON,
    &DARK_DWELLER_ORACLE,
    &GOBLIN_MOTIVATOR,
    &LATHLISS_DRAGON_QUEEN,
    &VIASHINO_PYROMANCER,
    &VOLLEY_VETERAN,
    &GIGANTOSAURUS,
    &VIVIEN_REID,
    &HEROIC_REINFORCEMENTS,
    &DIAMOND_MARE,
    &METEOR_GOLEM,
    &KARGAN_DRAGONRIDER,
    &AGGRESSIVE_MAMMOTH,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
