//! Lorwyn Eclipsed Commander card records required by the cEDH corpus.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AddManaEffectDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ComparisonDef;
use crate::card::ConditionDef;
use crate::card::CostDef;
use crate::card::CreateTokenDef;
use crate::card::EffectDef;
use crate::card::ManaColor;
use crate::card::ManaRestrictionDef;
use crate::card::ObjectCountConditionDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::PlayerRelation;
use crate::card::ReplacementEffectDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::ZoneKind;
use crate::card::abilities;

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "ECC",
    slug: "lorwyn-eclipsed-commander",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// ECC 22 — Abundant Countryside
pub(in crate::card::sets) static ABUNDANT_COUNTRYSIDE: CardRecord = CardRecord::new(
    "Abundant Countryside",
    "37478625-dd07-476d-bd9b-b2e0d71ac0d1",
    "Iris Compiet",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color. Spend this mana only to cast a creature spell.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color().with_restrictions(&[
                ManaRestrictionDef::CastSpell(ObjectPredicateDef::HasType(CardType::Creature)),
            ])),
        ),
        AbilityDef::activated(
            "{6}, {T}: Create a 1/1 colorless Shapeshifter creature token with changeling. \
             (It's every creature type.)",
            &[CostDef::Mana(crate::mana_cost!("{6}")), CostDef::TapSource],
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Shapeshifter"], &[], 1, 1)
                    .with_abilities(&[abilities::changeling()])
                    .with_art(CardArt::new(
                        "c2963ce1-f9d8-437a-9489-e0913a8b8d26",
                        "Jeff Miracola",
                    )),
            ))),
        ),
    ]),
);

// ECC 44 — Sodden Verdure
pub(in crate::card::sets) static SODDEN_VERDURE_44: CardRecord = CardRecord::new(
    "Sodden Verdure",
    "9030440a-a049-4152-afcf-b19648b20ce6",
    "Raymond Bonilla",
    CardRules::new_land(&["Forest", "Island"]).with_abilities(&[AbilityDef::as_enters(
        "This land enters tapped unless you control two or more basic lands.",
        ReplacementEffectDef::Conditional {
            condition: ConditionDef::ObjectCount(&ObjectCountConditionDef {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 2,
            }),
            if_true: &[],
            if_false: &[ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::Tapped,
            )],
        },
    )]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&ABUNDANT_COUNTRYSIDE, &SODDEN_VERDURE_44];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
