//! Dominaria United Commander card records required by supported formats.

use crate::card::CostDef;
use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CopyExceptionsDef;
use crate::card::CreateTokenDef;
use crate::card::CreatedTokensDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::InstalledTriggerDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRelation;
use crate::card::PlayerRuleDef;
use crate::card::PlayerSetDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::ids::ParentBinding;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "DMC",
    slug: "dominaria-united-commander",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// DMC 8 — The Reaver Cleaver
pub(in crate::card::sets) static THE_REAVER_CLEAVER_8: CardRecord = CardRecord::new(
    "The Reaver Cleaver",
    "5bcd1591-b5b9-49fc-9f2a-45f31ed1871e",
    "Yigit Koroglu",
    CardRules::new_artifact(mana_cost!("{2}{R}")).with_subtypes(&["Equipment"]).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::static_ability("Equipped creature gets +1/+1 and has trample and \"Whenever this creature deals combat damage to a player or planeswalker, create that many Treasure tokens.\"", EffectDef::StaticApply { recipient: EffectRecipientDef::AttachedPermanent, effect: AppliedEffectDef::Composite(&[AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)), AppliedEffectDef::add_ability(&abilities::trample()), AppliedEffectDef::add_ability(&AbilityDef::triggered("Whenever this creature deals combat damage to a player or planeswalker, create that many Treasure tokens.", TriggerEventDef::combat_damage_to_player_or_planeswalker(ObjectPredicateDef::Source), EffectDef::create_token(crate::card::tokens::treasure()).with_count(ValueDef::DamageEventAmount)))]) }),
abilities::equip(&[CostDef::Mana(mana_cost!("{3}"))], "Equip {3}")
]),
);

// DMC 10 — Cadric, Soul Kindler
pub(in crate::card::sets) static CADRIC_SOUL_KINDLER: CardRecord = CardRecord::new(
    "Cadric, Soul Kindler",
    "f82f8cab-5039-4e3a-a2ba-cbf829db80ed",
    "Joseph Weston",
CardRules::new_creature(mana_cost!("{2}{R}{W}"), &["Dwarf", "Wizard"], 4, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "The \"legend rule\" doesn't apply to tokens you control.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::players(PlayerSetDef::Related(
                        PlayerRelation::You,
                    )),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::PlayerRule(
                        PlayerRuleDef::LegendRuleDoesNotApplyTo(&ObjectPredicateDef::Token),
                    )),
                },
            ),
            AbilityDef::triggered(
                "Whenever another nontoken legendary permanent you control enters, you may pay {1}. If you do, create a token that's a copy of it. That token gains haste. Sacrifice it at the beginning of the next end step.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::PayOr(crate::card::PayOrDef::optional(
                    &[crate::CostDef::Mana(mana_cost!("{1}"))],
                    &EffectDef::CreateToken(
                        CreateTokenDef::new(TokenDef::Copy(&crate::card::TokenCopyDef {
                            object: &EffectRecipientDef::TriggeringObject,
                            exceptions: CopyExceptionsDef::NONE,
                        }))
                        .with_created_tokens(CreatedTokensDef {
                            binding: ParentBinding,
                            then: &EffectDef::Sequence(&[
                                EffectDef::Apply {
                                    recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                                    effect: AppliedEffectDef::add_ability(&abilities::haste()),
                                    duration: ResolvedEffectDurationDef::Permanent,
                                },
                                EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                                    "Sacrifice it at the beginning of the next end step.",
                                    TriggerEventDef::StepBegins {
                                        step: TurnStepDef::End,
                                        player: PlayerRelation::Any,
                                    },
                                    EffectDef::sacrifice(EffectRecipientDef::objects(ObjectSetDef::Binding(
                                        ParentBinding,
                                    ))),
                                ))),
                            ]),
                        }),
                    ),
                )),
            ),
        ]),
);

// DMC 47 — Torsten, Founder of Benalia
pub(in crate::card::sets) static TORSTEN_FOUNDER_OF_BENALIA: CardRecord = CardRecord::new(
    "Torsten, Founder of Benalia",
    "0783b426-a527-42c1-9271-be28b229e1c6",
    "Volkan Baǵa",
// Seven mana, and the two halves answer the two ways it goes wrong: it
    // refills your hand the turn it lands, and leaves seven bodies behind if
    // somebody kills it.
    CardRules::new_creature(mana_cost!("{5}{G}{W}"), &["Human", "Soldier"], 7, 7)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When Torsten enters, reveal the top seven cards of your library. Put any number of \
                 creature and/or land cards from among them into your hand and the rest on the bottom of \
                 your library in a random order.",
                // "Any number", so the choice is real: a land you would rather not draw
                // later can be left to the bottom, which is the only reason the clause is
                // bounded rather than mandatory. All seven are revealed, and what remains
                // is randomized rather than ordered as a plan for later.
                abilities::reveal_top_cards_choose_to_hand_rest_random_bottom(
                    ValueDef::Constant(7),
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Land),
                    ]),
                    0,
                    7,
                ),
            ),
            abilities::dies_trigger(
                "When Torsten dies, create seven 1/1 white Soldier creature tokens.",
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(
                        TokenCharacteristics::creature(&["Soldier"], &[ManaColor::White], 1, 1).with_art(
                            CardArt::new("8c4b0257-2ca5-4015-9d63-d7cf6e87ab9d", "Justine Cruz"),
                        ),
                    ))
                    .with_count(ValueDef::Constant(7)),
                ),
            ),
        ]),
);

// DMC 49 — Dihada, Binder of Wills
// Audit: unsupported — The ultimate must freeze all affected permanents across the control change before granting haste. Ability-grant validation cannot retain battlefield provenance through an ordinary object binding, and re-querying nonlands after the control change can miss permanents whose types changed.
pub(in crate::card::sets) static DIHADA_BINDER_OF_WILLS_49: CardRecord = CardRecord::new(
    "Dihada, Binder of Wills",
    "cea0ea07-6963-4de1-953d-b1ac41d8c6b5",
    "Néstor Ossandón Leal",
    crate::card::CardRules::unsupported(),
);

// DMC 93 — Gerrard's Hourglass Pendant
// Audit: unsupported — The graveyard-return activation needs identities of cards put there from the battlefield during this turn. That turn-scoped zone-change history is not retained.
pub(in crate::card::sets) static GERRARD_S_HOURGLASS_PENDANT_93: CardRecord = CardRecord::new(
    "Gerrard's Hourglass Pendant",
    "091135ec-4f4c-432c-bd6c-e7e2fb7561a3",
    "Sam Burley",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[
    &THE_REAVER_CLEAVER_8,
    &CADRIC_SOUL_KINDLER,
    &TORSTEN_FOUNDER_OF_BENALIA,
    &DIHADA_BINDER_OF_WILLS_49,
    &GERRARD_S_HOURGLASS_PENDANT_93,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
