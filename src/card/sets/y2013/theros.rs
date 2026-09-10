//! Theros cards cataloged as cross-format rules-engine test cases.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::BasicLandType;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::ColorChoiceOperationDef;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::ids::TargetIndex;
use crate::mana_cost;

// THS 16 — Gods Willing
pub(in crate::card::sets) static GODS_WILLING: CardRecord = CardRecord::new(
    "Gods Willing",
    "abafabb3-b2e7-4d78-b4b7-d8f701d3ee8b",
    "Mark Winters",
    // One mana that beats a removal spell and pushes damage through, and the
    // scry is what keeps it from being a dead card when neither is needed.
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature you control gains protection from the color of your choice until end of \
         turn. (It can't be blocked, targeted, dealt damage, enchanted, or equipped by anything \
         of that color.)\nScry 1.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::ChooseColor {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                operation: ColorChoiceOperationDef::ProtectionFromChosenColor,
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            // The scry happens even when the protection did nothing, so it
            // is sequenced after rather than made conditional.
            abilities::scry(ValueDef::Constant(1)),
        ]),
    )),
);

// THS 89 — Gray Merchant of Asphodel
pub(in crate::card::sets) static GRAY_MERCHANT_OF_ASPHODEL: CardRecord = CardRecord::new(
    "Gray Merchant of Asphodel",
    "b06078ce-f534-4e16-9a70-d51620a33eb2",
    "Robbie Trevino",
// Its own two black pips count, so the Merchant is never worth less than
    // two even on an otherwise empty board.
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Zombie"], 2, 4).with_ability(
        abilities::enters_trigger(
            "When this creature enters, each opponent loses X life, where X is your devotion to black. You gain life equal to the life lost this way.",
            // Devotion is counted once for the whole resolution, so both
            // halves read the same number and the gain always matches the
            // loss.
            EffectDef::Sequence(&[
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::players(PlayerSetDef::Related(
                        PlayerRelation::Opponent,
                    )),
                    amount: ValueDef::DevotionTo(ManaColor::Black),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::DevotionTo(ManaColor::Black),
                },
            ]),
        ),
    ),
);

// THS 127 — Lightning Strike
pub(in crate::card::sets) static LIGHTNING_STRIKE: CardRecord = CardRecord::new(
    "Lightning Strike",
    "bbb03f2e-2b92-4aa1-afae-301ed5d151d3",
    "Adam Paquette",
    // Lightning Bolt at two mana, which is the rate every later red burn
    // spell is measured against.
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Lightning Strike deals 3 damage to any target.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::damage(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ValueDef::Constant(3),
        ),
    )),
);

// THS 169 — Nylea's Presence
pub(in crate::card::sets) static NYLEAS_PRESENCE: CardRecord = CardRecord::new(
    "Nylea's Presence",
    "e68f1fd4-1a2f-405b-a592-6c4af6214eae",
    "Ralph Horsley",
    CardRules::new_enchantment(mana_cost!("{1}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant land",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Land),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            abilities::enters_trigger(
                "When Nylea's Presence enters, draw a card.",
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted land is every basic land type in addition to its other types.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_basic_land_types(&BasicLandType::ALL),
                },
            ),
        ]),
);

// THS 180 — Sylvan Caryatid
pub(in crate::card::sets) static SYLVAN_CARYATID: CardRecord = CardRecord::new(
    "Sylvan Caryatid",
    "d40b65c1-b24d-492d-81b9-d8474ebdc08c",
    "Chase Stone",
    // Hexproof is what separates it from every other two-mana accelerant: the
    // removal that answers a mana creature cannot be pointed at this one, and
    // a 0/3 wall survives most of what is left.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Plant"], 0, 3).with_abilities(&[
        abilities::defender(),
        abilities::hexproof(),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &GODS_WILLING,
    &GRAY_MERCHANT_OF_ASPHODEL,
    &LIGHTNING_STRIKE,
    &NYLEAS_PRESENCE,
    &SYLVAN_CARYATID,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
