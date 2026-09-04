//! Mirrodin cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AddManaEffectDef, AppliedEffectDef, CardRules,
    CardSet, CardType, ChoiceVisibilityDef, ChooseDef, CostAdjustmentDef, CostAmountDef, EffectDef,
    EffectRecipientDef, ManaColor, ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef,
    ObjectRefDef, ObjectSetDef, PlayerRefDef, PlayerRelation, SpellCostConditionDef, ValueDef,
    ZoneKind, ZonePlacement, abilities,
};
use crate::ids::ParentBinding;
use crate::{TargetIndex, mana_cost};

// MRD 11 — Leonin Skyhunter
pub(in crate::card::sets) static LEONIN_SKYHUNTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirrodin,
    "Leonin Skyhunter",
    "275a47e1-816c-44f9-bd05-b8b56410436f",
    "Kev Walker",
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Cat", "Knight"], 2, 2)
        .with_abilities(&[abilities::flying()]),
);

// MRD 16 — Raise the Alarm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAISE_THE_ALARM: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirrodin,
    "Raise the Alarm",
    "4be510c8-fc01-4374-ac04-7968d24480fe",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// MRD 57 — Barter in Blood
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARTER_IN_BLOOD: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirrodin,
    "Barter in Blood",
    "beccbb2c-ca1d-4b72-9eca-a64a313fd830",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// MRD 60 — Consume Spirit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONSUME_SPIRIT: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirrodin,
    "Consume Spirit",
    "f375a49c-806a-4d8b-9513-6b4afc19497b",
    "Matt Thompson",
    crate::card::CardRules::unsupported(),
);

// MRD 122 — Hum of the Radix
pub(in crate::card::sets) static HUM_OF_THE_RADIX: CardRecord = CardRecord::new(
    CardSet::Mirrodin,
    "Hum of the Radix",
    "328f3afb-1a56-42a5-bd1e-3e704291972f",
    "John Avon",
    CardRules::new_enchantment(mana_cost!("{2}{G}{G}")).with_ability(
        abilities::spell_cost_adjustment(
            "Each artifact spell costs {1} more to cast for each artifact its controller controls.",
            ObjectPredicateDef::HasType(CardType::Artifact),
            PlayerRelation::Any,
            SpellCostConditionDef::Always,
            CostAdjustmentDef::Add(CostAmountDef::Generic(ValueDef::CountMatchingObjects(
                &ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
            ))),
        ),
    ),
);

// MRD 141 — Aether Spellbomb
pub(in crate::card::sets) static AETHER_SPELLBOMB: CardRecord = CardRecord::new(
    CardSet::Mirrodin,
    "Aether Spellbomb",
    "f3792e8b-4ad7-4e2d-994c-c4eaac0fa55f",
    "Jim Nelson",
    // One mana that answers a creature for a turn if it has to and replaces
    // itself if it does not, which is why it costs a deck nothing to play.
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{U}, Sacrifice this artifact: Return target creature to its owner's hand.",
            &[
                AbilityCostDef::Mana(mana_cost!("{U}")),
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
        AbilityDef::activated(
            "{1}, Sacrifice this artifact: Draw a card.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::SacrificeSource,
            ],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// MRD 146 — Bonesplitter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BONESPLITTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirrodin,
    "Bonesplitter",
    "465a7990-c9f9-4716-a833-fd41458b9cee",
    "Darrell Riche",
    crate::card::CardRules::unsupported(),
);

// MRD 152 — Chrome Mox
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHROME_MOX: CardRecord = CardRecord::new(
    CardSet::Mirrodin,
    "Chrome Mox",
    "6a058e68-70af-4a64-859c-c881e5578368",
    "Donato Giancola",
    // A free artifact whose cost is a card, paid in advance and in full: the
    // imprinted card is gone, and what it leaves behind is one mana a turn
    // in whatever colours it was.
    CardRules::new_artifact(mana_cost!("{0}")).with_abilities(&[
        abilities::enters_trigger(
            "Imprint — When this artifact enters, you may exile a nonartifact, nonland card from \
             your hand.",
            // "You may": a minimum of none, so a hand with nothing worth paying leaves
            // the Mox on the battlefield making nothing.
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Object(ParentBinding),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                // "A nonartifact, nonland card from your hand": the two types it may not
                // take are the ones that would make it free twice over.
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Artifact)),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    ]),
                    &[ZoneKind::Hand],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 0,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Public,
                // The exile is linked to the Mox, which is what makes the mana ability
                // able to read the card's colours later.
                then: &EffectDef::ExileLinkedToSource {
                    until_source_leaves: false,
                    object: EffectRecipientDef::object(ObjectRefDef::Binding(ParentBinding)),
                    face_down: false,
                    then: None,
                },
            }),
        ),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any of the exiled card's colors.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::colors_of_linked_exiles()),
        ),
    ]),
);

// MRD 158 — Copper Myr
pub(in crate::card::sets) static COPPER_MYR: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirrodin,
    "Copper Myr",
    "a52b2dc4-4fb3-4ddf-bdb6-c63e8c8efc09",
    "Kev Walker",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Myr"], 1, 1).with_ability(
        AbilityDef::activated_mana(
            "{T}: Add {G}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
        ),
    ),
);

// MRD 171 — Fireshrieker
pub(in crate::card::sets) static FIRESHRIEKER: CardRecord = CardRecord::new(
    CardSet::Mirrodin,
    "Fireshrieker",
    "8da0fcc6-6209-4b8e-997d-ad3cc4ff0856",
    "Christopher Moeller",
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

// MRD 175 — Gilded Lotus
pub(in crate::card::sets) static GILDED_LOTUS: CardRecord = CardRecord::new(
    CardSet::Mirrodin,
    "Gilded Lotus",
    "a1d5e4c8-dfd0-45bc-8000-ebfaccfefec3",
    "Martina Pilcerova",
    CardRules::new_artifact(mana_cost!("{5}")).with_ability(AbilityDef::activated_mana(
        "{T}: Add three mana of any one color.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::any_color().with_amount(3)),
    )),
);

// MRD 180 — Gold Myr
pub(in crate::card::sets) static GOLD_MYR: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirrodin,
    "Gold Myr",
    "fa9b4040-ab49-476b-b101-5ef2b1824e10",
    "Kev Walker",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Myr"], 1, 1).with_ability(
        AbilityDef::activated_mana(
            "{T}: Add {W}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::White)),
        ),
    ),
);

// MRD 187 — Iron Myr
pub(in crate::card::sets) static IRON_MYR: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirrodin,
    "Iron Myr",
    "08e17883-0767-40b5-ac44-a52a1ea54993",
    "Kev Walker",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Myr"], 1, 1).with_ability(
        AbilityDef::activated_mana(
            "{T}: Add {R}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
        ),
    ),
);

// MRD 191 — Leaden Myr
pub(in crate::card::sets) static LEADEN_MYR: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirrodin,
    "Leaden Myr",
    "555efe5f-848f-44da-92b5-69c8e852f179",
    "Kev Walker",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Myr"], 1, 1).with_ability(
        AbilityDef::activated_mana(
            "{T}: Add {B}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black)),
        ),
    ),
);

// MRD 199 — Lightning Greaves
pub(in crate::card::sets) static LIGHTNING_GREAVES: CardRecord = CardRecord::new(
    CardSet::Mirrodin,
    "Lightning Greaves",
    "61a28870-cf78-4323-9d82-cee764067764",
    "Jeremy Jarvis",
    // Equipping for nothing is the whole card: the Greaves move to whatever
    // just arrived, every turn, for as long as they are on the battlefield.
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature has haste and shroud.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    // The two halves are why the card is played: haste makes the creature useful
                    // the turn it arrives, and shroud makes it hard to answer -- including by
                    // its own controller, who cannot target it either.
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_ability(&abilities::haste()),
                        AppliedEffectDef::add_ability(&abilities::shroud()),
                    ]),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{0}"))], "Equip {0}"),
        ]),
);

// MRD 206 — Mindslaver
// Audit: unsupported — The engine has no continuous effect that lets one player make every game choice for another player during that player's next turn.
pub(in crate::card::sets) static MINDSLAVER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirrodin,
    "Mindslaver",
    "98fb1eaa-2871-491a-a4f5-3e358778ba40",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// MRD 226 — Pentavus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PENTAVUS: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirrodin,
    "Pentavus",
    "32a11f0a-7547-4fda-a8ed-caf76ce98f10",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// MRD 241 — Silver Myr
pub(in crate::card::sets) static SILVER_MYR: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirrodin,
    "Silver Myr",
    "b83a73a2-fedb-40bd-8e29-82a7abd6f211",
    "Kev Walker",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Myr"], 1, 1).with_ability(
        AbilityDef::activated_mana(
            "{T}: Add {U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue)),
        ),
    ),
);

// MRD 245 — Solemn Simulacrum
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLEMN_SIMULACRUM: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirrodin,
    "Solemn Simulacrum",
    "00f9955f-a522-47bf-b064-92dd21a76b18",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// MRD 253 — Talisman of Dominance
pub(in crate::card::sets) static TALISMAN_OF_DOMINANCE: CardRecord = CardRecord::new(
    CardSet::Mirrodin,
    "Talisman of Dominance",
    "991037a2-fea2-49f5-8ace-ebbf9f678cff",
    "Mike Dringenberg",
    // Two mana that fixes for a life a turn, or for nothing at all when
    // colourless is what the next spell wants.
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {C}.",
            &TALISMAN_TAP,
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
        AbilityDef::activated_mana(
            "{T}: Add {U} or {B}. This artifact deals 1 damage to you.",
            &TALISMAN_TAP,
            EffectDef::AddMana(
                // The pair this Talisman is for, read the same way its siblings are.
                AddManaEffectDef::choice(&[ManaColor::Blue, ManaColor::Black])
                    .with_damage_to_controller(1),
            ),
        ),
    ]),
);

// MRD 256 — Talisman of Progress
static TALISMAN_TAP: [AbilityCostDef; 1] = [AbilityCostDef::TapSource];

pub(in crate::card::sets) static TALISMAN_OF_PROGRESS: CardRecord = CardRecord::new(
    CardSet::Mirrodin,
    "Talisman of Progress",
    "41ff849e-2439-4690-8aa4-769039b6da4c",
    "Mike Dringenberg",
    // Two mana that fixes for a life a turn, or for nothing at all when
    // colourless is what the next spell wants.
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {C}.",
            &TALISMAN_TAP,
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
        AbilityDef::activated_mana(
            "{T}: Add {W} or {U}. This artifact deals 1 damage to you.",
            &TALISMAN_TAP,
            // The pair this Talisman is for. Which of the two an activation makes
            // belongs to the activation, so the choice is one printed ability rather
            // than two.
            EffectDef::AddMana(
                AddManaEffectDef::choice(&[ManaColor::White, ManaColor::Blue])
                    .with_damage_to_controller(1),
            ),
        ),
    ]),
);

// MRD 276 — Worldslayer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORLDSLAYER: CardRecord = CardRecord::new(
    crate::card::CardSet::Mirrodin,
    "Worldslayer",
    "3cb1b869-3e2d-4447-a12d-e790883feeee",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &LEONIN_SKYHUNTER,
    &RAISE_THE_ALARM,
    &BARTER_IN_BLOOD,
    &CONSUME_SPIRIT,
    &HUM_OF_THE_RADIX,
    &AETHER_SPELLBOMB,
    &BONESPLITTER,
    &CHROME_MOX,
    &COPPER_MYR,
    &FIRESHRIEKER,
    &GILDED_LOTUS,
    &GOLD_MYR,
    &IRON_MYR,
    &LEADEN_MYR,
    &LIGHTNING_GREAVES,
    &MINDSLAVER,
    &PENTAVUS,
    &SILVER_MYR,
    &SOLEMN_SIMULACRUM,
    &TALISMAN_OF_DOMINANCE,
    &TALISMAN_OF_PROGRESS,
    &WORLDSLAYER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
