//! Mirrodin cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, CardArt, CardNameSetDef, CardRules, CardSet, CardType, ChoiceVisibilityDef,
    ChooseDef, CostAdjustmentDef, CostAmountDef, EffectDef, EffectRecipientDef, ManaColor,
    ManaTypeSetDef, ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef,
    ObjectSetDef, PlayerRefDef, PlayerRelation, SpellCostConditionDef, TriggerEventDef, ValueDef,
    ZoneKind, ZonePlacement, abilities,
};
use crate::ids::ParentBinding;
use crate::{TargetIndex, mana_cost};

// MRD 57 — Barter in Blood
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARTER_IN_BLOOD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("beccbb2c-ca1d-4b72-9eca-a64a313fd830"),
    "Barter in Blood",
    crate::card::CardArt::new("beccbb2c-ca1d-4b72-9eca-a64a313fd830", "Paolo Parente"),
    crate::card::CardSet::Mirrodin,
    crate::card::CardRules::unsupported(),
);

// MRD 122 — Hum of the Radix
pub(in crate::card::sets) static HUM_OF_THE_RADIX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("328f3afb-1a56-42a5-bd1e-3e704291972f"),
    "Hum of the Radix",
    CardArt::new("328f3afb-1a56-42a5-bd1e-3e704291972f", "John Avon"),
    CardSet::Mirrodin,
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
    PrintingAnchor::scryfall("f3792e8b-4ad7-4e2d-994c-c4eaac0fa55f"),
    "Aether Spellbomb",
    CardArt::new("f3792e8b-4ad7-4e2d-994c-c4eaac0fa55f", "Jim Nelson"),
    CardSet::Mirrodin,
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
pub(in crate::card::sets) static BONESPLITTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ae31d513-7412-4467-b497-a7183ff29a42"),
    "Bonesplitter",
    CardArt::new("465a7990-c9f9-4716-a833-fd41458b9cee", "Darrell Riche"),
    CardSet::Mirrodin,
    // Two mana total for +2/+0, and the Equipment survives whatever it was
    // holding, which is why it never stops being playable in a limited deck.
    CardRules::new_artifact(mana_cost!("{1}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +2/+0.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// MRD 152 — Chrome Mox
pub(in crate::card::sets) static CHROME_MOX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6a058e68-70af-4a64-859c-c881e5578368"),
    "Chrome Mox",
    CardArt::new("6a058e68-70af-4a64-859c-c881e5578368", "Donato Giancola"),
    CardSet::Mirrodin,
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

// MRD 169 — Extraplanar Lens
pub(in crate::card::sets) static EXTRAPLANAR_LENS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("622a6523-3b12-4657-a656-00a57a3ae59c"),
    "Extraplanar Lens",
    CardArt::new("622a6523-3b12-4657-a656-00a57a3ae59c", "Lars Grant-West"),
    CardSet::Mirrodin,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this artifact enters, you may exile target land you control.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Land),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            })],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::ExileLinkedToSource {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    until_source_leaves: false,
                    face_down: false,
                    then: None,
                },
            },
        ),
        AbilityDef::triggered_mana(
            "Whenever a land with the same name as the exiled card is tapped for mana, its controller adds one mana of any type that land produced.",
            TriggerEventDef::tapped_for_mana(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Land),
                ObjectPredicateDef::NameIn(&CardNameSetDef::NamesOf(
                    &ObjectSetDef::LinkedExiles,
                )),
            ])),
            EffectDef::AddMana(
                AddManaEffectDef::choice_from(ManaTypeSetDef::produced_by(
                    ObjectRefDef::TriggeringObject,
                ))
                .to_triggering_objects_controller(),
            ),
        ),
    ]),
);

// MRD 199 — Lightning Greaves
pub(in crate::card::sets) static LIGHTNING_GREAVES: CardRecord = CardRecord::new_with_legacy_id(
    2170,
    "Lightning Greaves",
    CardArt::new("61a28870-cf78-4323-9d82-cee764067764", "Jeremy Jarvis"),
    CardSet::Mirrodin,
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

// MRD 253 — Talisman of Dominance
pub(in crate::card::sets) static TALISMAN_OF_DOMINANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("991037a2-fea2-49f5-8ace-ebbf9f678cff"),
    "Talisman of Dominance",
    CardArt::new("991037a2-fea2-49f5-8ace-ebbf9f678cff", "Mike Dringenberg"),
    CardSet::Mirrodin,
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
    PrintingAnchor::scryfall("41ff849e-2439-4690-8aa4-769039b6da4c"),
    "Talisman of Progress",
    CardArt::new("41ff849e-2439-4690-8aa4-769039b6da4c", "Mike Dringenberg"),
    CardSet::Mirrodin,
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

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &BARTER_IN_BLOOD,
    &HUM_OF_THE_RADIX,
    &AETHER_SPELLBOMB,
    &BONESPLITTER,
    &CHROME_MOX,
    &EXTRAPLANAR_LENS,
    &LIGHTNING_GREAVES,
    &TALISMAN_OF_DOMINANCE,
    &TALISMAN_OF_PROGRESS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
