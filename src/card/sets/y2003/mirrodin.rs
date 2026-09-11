//! Mirrodin cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::CardNameSetDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::CostAdjustmentDef;
use crate::card::CostAmountDef;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ManaTypeSetDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::ReplacementEffectDef;
use crate::card::SacrificedAmountDef;
use crate::card::SpellCostConditionDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::ids::ParentBinding;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "MRD",
    slug: "mirrodin",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// MRD 11 — Leonin Skyhunter
pub(in crate::card::sets) static LEONIN_SKYHUNTER: CardRecord = CardRecord::new(
    "Leonin Skyhunter",
    "275a47e1-816c-44f9-bd05-b8b56410436f",
    "Kev Walker",
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Cat", "Knight"], 2, 2)
        .with_abilities(&[abilities::flying()]),
);

// MRD 16 — Raise the Alarm
pub(in crate::card::sets) static RAISE_THE_ALARM: CardRecord = CardRecord::new(
    "Raise the Alarm",
    "4be510c8-fc01-4374-ac04-7968d24480fe",
    "John Matson",
    // Two bodies at instant speed, which is what a token deck pays the extra
    // mana for: it holds up the trick and still develops the board.
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell(
        "Create two 1/1 white Soldier creature tokens.",
        EffectDef::create_creature_token(&["Soldier"], &[ManaColor::White], 1, 1).with_amount(2),
    )),
);

// MRD 57 — Barter in Blood
pub(in crate::card::sets) static BARTER_IN_BLOOD: CardRecord = CardRecord::new(
    "Barter in Blood",
    "beccbb2c-ca1d-4b72-9eca-a64a313fd830",
    "Paolo Parente",
    // Two creatures apiece, so it only comes out ahead when the caster has
    // fewer than the opponent.
    CardRules::new_sorcery(mana_cost!("{2}{B}{B}")).with_ability(AbilityDef::spell(
        "Each player sacrifices two creatures of their choice.",
        EffectDef::SacrificeOfChoice {
            player: EffectRecipientDef::EachPlayer,
            object: ObjectPredicateDef::HasType(CardType::Creature),
            count: ValueDef::Constant(2),
            then: None,
            amount: SacrificedAmountDef::Power,
            otherwise: None,
            optional: false,
        },
    )),
);

// MRD 60 — Consume Spirit
pub(in crate::card::sets) static CONSUME_SPIRIT: CardRecord = CardRecord::new(
    "Consume Spirit",
    "f375a49c-806a-4d8b-9513-6b4afc19497b",
    "Matt Thompson",
CardRules::new_sorcery(mana_cost!("{X}{1}{B}"))
        .spend_only_on_x(ManaColor::Black)
        .with_abilities(&[
            AbilityDef::enforced_when_cast(
                "Spend only black mana on X.",
                "The payment layer folds X into the black requirement, so no other mana can cover it.",
            ),
            AbilityDef::spell_with_targets(
                "This spell deals X damage to any target and you gain X life.",
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)],
                EffectDef::Sequence(&[
                    EffectDef::damage(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ValueDef::ChosenX,
                    ),
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::ChosenX,
                    },
                ]),
            ),
        ]),
);

// MRD 122 — Hum of the Radix
pub(in crate::card::sets) static HUM_OF_THE_RADIX: CardRecord = CardRecord::new(
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
    "Aether Spellbomb",
    "f3792e8b-4ad7-4e2d-994c-c4eaac0fa55f",
    "Jim Nelson",
    // One mana that answers a creature for a turn if it has to and replaces
    // itself if it does not, which is why it costs a deck nothing to play.
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{U}, Sacrifice this artifact: Return target creature to its owner's hand.",
            &[CostDef::Mana(mana_cost!("{U}")), CostDef::SacrificeSource],
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
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::SacrificeSource],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// MRD 146 — Bonesplitter
pub(in crate::card::sets) static BONESPLITTER: CardRecord = CardRecord::new(
    "Bonesplitter",
    "465a7990-c9f9-4716-a833-fd41458b9cee",
    "Darrell Riche",
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
            abilities::equip(&[CostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// MRD 150 — Chalice of the Void
pub(in crate::card::sets) static CHALICE_OF_THE_VOID: CardRecord = CardRecord::new(
    "Chalice of the Void",
    "1a02ca71-5e39-4a5f-aaba-a1e3e10a6a3e",
    "Mark Zug",
CardRules::new_artifact(mana_cost!("{X}{X}")).with_abilities(&[
        AbilityDef::as_enters(
            "This artifact enters with X charge counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCastXCounters {
                    kind: CounterKind::named("charge"),
                },
            ),
        ),
        AbilityDef::triggered(
            "Whenever a player casts a spell with mana value equal to the number of charge counters on this artifact, counter that spell.",
            // The comparison belongs to the cast event, so changing the counters
            // afterward does not change whether the captured trigger counters it.
            TriggerEventDef::spell_cast(ObjectPredicateDef::ManaValueEqualTo(
                ValueDef::CountersOnSource(CounterKind::named("charge")),
            )),
            EffectDef::Counter {
                object: EffectRecipientDef::TriggeringObject,
                zone: ZoneKind::Graveyard,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// MRD 152 — Chrome Mox
pub(in crate::card::sets) static CHROME_MOX: CardRecord = CardRecord::new(
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
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::colors_of_linked_exiles()),
        ),
    ]),
);

// MRD 158 — Copper Myr
pub(in crate::card::sets) static COPPER_MYR: CardRecord = CardRecord::new(
    "Copper Myr",
    "a52b2dc4-4fb3-4ddf-bdb6-c63e8c8efc09",
    "Kev Walker",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Myr"], 1, 1).with_ability(
        AbilityDef::activated_mana(
            "{T}: Add {G}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
        ),
    ),
);

// MRD 169 — Extraplanar Lens
pub(in crate::card::sets) static EXTRAPLANAR_LENS: CardRecord = CardRecord::new(
    "Extraplanar Lens",
    "622a6523-3b12-4657-a656-00a57a3ae59c",
    "Lars Grant-West",
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

// MRD 171 — Fireshrieker
pub(in crate::card::sets) static FIRESHRIEKER: CardRecord = CardRecord::new(
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
                &[CostDef::Mana(mana_cost!("{2}"))],
                "Equip {2} ({2}: Attach to target creature you control. Equip only as a \
                 sorcery.)",
            ),
        ]),
);

// MRD 175 — Gilded Lotus
pub(in crate::card::sets) static GILDED_LOTUS: CardRecord = CardRecord::new(
    "Gilded Lotus",
    "a1d5e4c8-dfd0-45bc-8000-ebfaccfefec3",
    "Martina Pilcerova",
    CardRules::new_artifact(mana_cost!("{5}")).with_ability(AbilityDef::activated_mana(
        "{T}: Add three mana of any one color.",
        &[CostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::any_color().with_amount(3)),
    )),
);

// MRD 180 — Gold Myr
pub(in crate::card::sets) static GOLD_MYR: CardRecord = CardRecord::new(
    "Gold Myr",
    "fa9b4040-ab49-476b-b101-5ef2b1824e10",
    "Kev Walker",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Myr"], 1, 1).with_ability(
        AbilityDef::activated_mana(
            "{T}: Add {W}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::White)),
        ),
    ),
);

// MRD 187 — Iron Myr
pub(in crate::card::sets) static IRON_MYR: CardRecord = CardRecord::new(
    "Iron Myr",
    "08e17883-0767-40b5-ac44-a52a1ea54993",
    "Kev Walker",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Myr"], 1, 1).with_ability(
        AbilityDef::activated_mana(
            "{T}: Add {R}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
        ),
    ),
);

// MRD 191 — Leaden Myr
pub(in crate::card::sets) static LEADEN_MYR: CardRecord = CardRecord::new(
    "Leaden Myr",
    "555efe5f-848f-44da-92b5-69c8e852f179",
    "Kev Walker",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Myr"], 1, 1).with_ability(
        AbilityDef::activated_mana(
            "{T}: Add {B}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black)),
        ),
    ),
);

// MRD 199 — Lightning Greaves
pub(in crate::card::sets) static LIGHTNING_GREAVES: CardRecord = CardRecord::new(
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
            abilities::equip(&[CostDef::Mana(mana_cost!("{0}"))], "Equip {0}"),
        ]),
);

// MRD 206 — Mindslaver
// Audit: unsupported — The engine has no continuous effect that lets one player make every game choice for another player during that player's next turn.
pub(in crate::card::sets) static MINDSLAVER: CardRecord = CardRecord::new(
    "Mindslaver",
    "98fb1eaa-2871-491a-a4f5-3e358778ba40",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// MRD 226 — Pentavus
pub(in crate::card::sets) static PENTAVUS: CardRecord = CardRecord::new(
    "Pentavus",
    "32a11f0a-7547-4fda-a8ed-caf76ce98f10",
    "Greg Staples",
CardRules::new_artifact_creature(mana_cost!("{7}"), &["Construct"], 0, 0).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with five +1/+1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 5,
                },
            ),
        ),
        AbilityDef::activated(
            "{1}, Remove a +1/+1 counter from this creature: Create a 1/1 colorless Pentavite artifact creature token with flying.",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::RemoveCountersFromSource {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 1,
                },
            ],
            EffectDef::create_artifact_creature_token(&["Pentavite"], &[], 1, 1)
                .with_abilities(&[abilities::flying()]),
        ),
        AbilityDef::activated(
            "{1}, Sacrifice a Pentavite: Put a +1/+1 counter on this creature.",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::Subtype("Pentavite"),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// MRD 241 — Silver Myr
pub(in crate::card::sets) static SILVER_MYR: CardRecord = CardRecord::new(
    "Silver Myr",
    "b83a73a2-fedb-40bd-8e29-82a7abd6f211",
    "Kev Walker",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Myr"], 1, 1).with_ability(
        AbilityDef::activated_mana(
            "{T}: Add {U}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue)),
        ),
    ),
);

// MRD 245 — Solemn Simulacrum
pub(in crate::card::sets) static SOLEMN_SIMULACRUM: CardRecord = CardRecord::new(
    "Solemn Simulacrum",
    "00f9955f-a522-47bf-b064-92dd21a76b18",
    "Greg Staples",
CardRules::new_artifact_creature(mana_cost!("{4}"), &["Golem"], 2, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, you may search your library for a basic land card, put that card onto the battlefield tapped, then shuffle.",
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Supertype(CardSupertype::Basic),
                ]),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: true,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: true,
                attachment: None,
                binding: None,
                then: None,
            },
        ),
        abilities::dies_trigger(
            "When this creature dies, you may draw a card.",
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

// MRD 253 — Talisman of Dominance
pub(in crate::card::sets) static TALISMAN_OF_DOMINANCE: CardRecord = CardRecord::new(
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
static TALISMAN_TAP: [CostDef; 1] = [CostDef::TapSource];

pub(in crate::card::sets) static TALISMAN_OF_PROGRESS: CardRecord = CardRecord::new(
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
pub(in crate::card::sets) static WORLDSLAYER: CardRecord = CardRecord::new(
    "Worldslayer",
    "3cb1b869-3e2d-4447-a12d-e790883feeee",
    "Greg Staples",
CardRules::new_artifact(mana_cost!("{5}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever equipped creature deals combat damage to a player, destroy all permanents other than this Equipment.",
                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::AttachedToSource),
                EffectDef::Destroy {
                    object: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    then: None,
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{5}"))], "Equip {5}"),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &LEONIN_SKYHUNTER,
    &RAISE_THE_ALARM,
    &BARTER_IN_BLOOD,
    &CONSUME_SPIRIT,
    &HUM_OF_THE_RADIX,
    &AETHER_SPELLBOMB,
    &BONESPLITTER,
    &CHALICE_OF_THE_VOID,
    &CHROME_MOX,
    &COPPER_MYR,
    &EXTRAPLANAR_LENS,
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
