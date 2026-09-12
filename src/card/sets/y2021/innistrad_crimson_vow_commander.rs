//! Innistrad: Crimson Vow Commander cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::ParentBinding;
use crate::card::AbilityDef;
use crate::card::AppliedEffectDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CostDef;
use crate::card::CreateTokenDef;
use crate::card::DiscardFollowUpDef;
use crate::card::DiscardSelectionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectSetDef;
use crate::card::PayOrDef;
use crate::card::PlayerRelation;
use crate::card::SubtypeDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "VOC",
    slug: "innistrad-crimson-vow-commander",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// VOC 14 — Occult Epiphany
pub(in crate::card::sets) static OCCULT_EPIPHANY: CardRecord = CardRecord::new(
    "Occult Epiphany",
    "6920c895-bc98-4871-a53f-219fa27a74e5",
    "Jason Rainville",
    // The draw is a wash and the Spirits are the card: a hand with five
    // types in it turns X of nothing into five fliers.
    CardRules::new_instant(mana_cost!("{X}{U}")).with_ability(AbilityDef::spell(
        "Draw X cards, then discard X cards. Create a 1/1 white Spirit creature token with \
             flying for each card type among cards discarded this way.",
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::ChosenX,
            },
            EffectDef::Discard {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::ChosenX,
                selection: DiscardSelectionDef::RecipientChooses,
                then: Some(DiscardFollowUpDef {
                    counted: ObjectPredicateDef::Any,
                    bound: Some(ParentBinding),
                    // A Spirit for every card type the discard turned up. Every discarded card
                    // is counted, so the predicate is anything at all; what the value counts is
                    // the types between them rather than the cards.
                    effect: &EffectDef::CreateToken(
                        CreateTokenDef::new(TokenDef::Literal(
                            TokenCharacteristics::creature(&["Spirit"], &[ManaColor::White], 1, 1)
                                .with_abilities(&[abilities::flying()])
                                .with_art(CardArt::new(
                                    "6bee4081-5d74-4cc2-ba2f-887bc8799513",
                                    "Kim Sokol",
                                )),
                        ))
                        .with_count(ValueDef::CardTypesAmongObjects(
                            &crate::card::ObjectSetDef::Binding(ParentBinding),
                        )),
                    ),
                }),
            },
        ]),
    )),
);

// VOC 17 — Crossway Troublemakers
pub(in crate::card::sets) static CROSSWAY_TROUBLEMAKERS: CardRecord = CardRecord::new(
    "Crossway Troublemakers",
    "431711c5-c04f-4d34-97c9-5199cfbf9da9",
    "Aaron J. Riley",
    CardRules::new_creature(mana_cost!("{5}{B}"), &["Vampire"], 5, 5).with_abilities(&[
        AbilityDef::static_ability(
            "Attacking Vampires you control have deathtouch and lifelink. \
             (Any amount of damage they deal to a creature is enough to \
             destroy it. Damage dealt by those creatures also causes their \
             controller to gain that much life.)",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vampire")),
                            ObjectPredicateDef::Attacking,
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_ability(&abilities::deathtouch()),
                    AppliedEffectDef::add_ability(&abilities::lifelink()),
                ]),
            },
        ),
        AbilityDef::triggered(
            "Whenever a Vampire you control dies, you may pay 2 life. If \
             you do, draw a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vampire")),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::PayLife(2)],
                &abilities::draw_cards(ValueDef::Constant(1)),
            )),
        ),
    ]),
);

// VOC 60 — Shadowgrange Archfiend
// Audit: unsupported — Madness needs its discard-to-exile replacement and linked cast-or-graveyard procedure.
pub(in crate::card::sets) static SHADOWGRANGE_ARCHFIEND_60: CardRecord = CardRecord::new(
    "Shadowgrange Archfiend",
    "598bf482-99ce-4b39-a969-7685752382ed",
    "Oleksandr Kozachenko",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[
    &OCCULT_EPIPHANY,
    &CROSSWAY_TROUBLEMAKERS,
    &SHADOWGRANGE_ARCHFIEND_60,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
