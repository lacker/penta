//! Streets of New Capenna Commander cards cataloged for the Vintage Cube
//! pool.

use crate::card::AppliedEffectDef;
use crate::card::EffectChoiceDef;
use crate::card::GameActionDef;
use crate::card::ObjectQueryDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::abilities;
use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::CopyExceptionsDef;
use crate::card::CostDef;
use crate::card::CreateTokenDef;
use crate::card::DiscardSelectionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::TokenCharacteristics;
use crate::card::TokenCopyDef;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::tokens;
use crate::ids::ParentBinding;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "NCC",
    slug: "streets-of-new-capenna-commander",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const TREASURE_TOKEN: TokenCharacteristics = tokens::treasure().with_art(CardArt::new(
    "1be23c27-d8b6-4f59-8ab8-9ce80e9e29dd",
    "Nadia Hurianova",
));

// NCC 25 — Extravagant Replication
pub(in crate::card::sets) static EXTRAVAGANT_REPLICATION: CardRecord = CardRecord::new(
    "Extravagant Replication",
    "6a6f55d7-d689-43eb-a59a-b8be88269ee6",
    "Pauline Voss",
    CardRules::new_enchantment(mana_cost!("{4}{U}{U}")).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "At the beginning of your upkeep, create a token that's a copy \
             of another target nonland permanent you control.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Copy(&TokenCopyDef {
                object: &EffectRecipientDef::Target(TargetIndex::PRIMARY),
                exceptions: CopyExceptionsDef::NONE,
            }))),
        ),
    ]),
);

// NCC 36 — Lethal Scheme
// Audit: unsupported — The cast context does not expose the identities of the creatures that convoked the spell for a later connive instruction.
pub(in crate::card::sets) static LETHAL_SCHEME_36: CardRecord = CardRecord::new(
    "Lethal Scheme",
    "65864680-9520-4eb3-9774-fa478e54a290",
    "Tuan Duong Chu",
    crate::card::CardRules::unsupported(),
);

// NCC 52 — Seize the Spotlight
pub(in crate::card::sets) static SEIZE_THE_SPOTLIGHT_52: CardRecord = CardRecord::new(
    "Seize the Spotlight",
    "3b4df2f7-8d17-4484-85a3-b8f3e4dd0c7c",
    "Ernanda Souza",
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_abilities(&[
AbilityDef::spell("Each opponent chooses fame or fortune. For each player who chose fame, gain control of a creature that player controls until end of turn. Untap those creatures and they gain haste until end of turn. For each player who chose fortune, you draw a card and create a Treasure token.", EffectDef::ChooseEffect { player: EffectRecipientDef::Opponent, choices: &[EffectChoiceDef { label: "Fame", effect: EffectDef::Choose(ChooseDef { binding: ObjectChoiceBindingDef::Objects(Binding!("spotlight_creature")), unchosen: None, chooser: PlayerRefDef::EffectController, candidates: ObjectSetDef::Query(ObjectQueryDef::matching(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::Opponent)), exclude: None, minimum: 1, maximum: 1, visibility: ChoiceVisibilityDef::Public, then: &EffectDef::Sequence(&[EffectDef::Perform(GameActionDef::GainControl { object: EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("spotlight_creature"))), controller: PlayerRefDef::EffectController, duration: crate::card::ControlDurationDef::UntilEndOfTurn }), EffectDef::Untap { object: EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("spotlight_creature"))) }, EffectDef::Apply { recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("spotlight_creature"))), effect: AppliedEffectDef::add_ability(&abilities::haste()), duration: ResolvedEffectDurationDef::UntilEndOfTurn }]) }) }, EffectChoiceDef { label: "Fortune", effect: EffectDef::Sequence(&[abilities::draw_cards(ValueDef::Constant(1)), EffectDef::create_token(crate::card::tokens::treasure())]) }] })
]),
);

// NCC 81 — Currency Converter
/// The card goes back to the graveyard it came from -- its owner's, which is
/// where a card exiled from a graveyard belongs however it got to exile.
static CONVERTER_RETURNS_THE_CARD: EffectDef = EffectDef::move_to_zone(
    EffectRecipientDef::object(ObjectRefDef::Binding(ParentBinding)),
    ZoneKind::Graveyard,
    ZonePlacement::Top,
);

pub(in crate::card::sets) static CURRENCY_CONVERTER: CardRecord = CardRecord::new(
    "Currency Converter",
    "187b6719-e5ed-4615-a00b-3313ceca055b",
    "Sean Murray",
// One mana for a bank: every card you throw away is held rather than
    // spent, and later it comes back out as a Treasure or a body.
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you discard a card, you may exile that card from your graveyard.",
            TriggerEventDef::Discarded(PlayerRelation::You),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                // "That card", which is the card as it now lies in the graveyard: the
                // discard is over by the time this resolves, so what the trigger points at
                // is the graveyard object rather than the one that was in hand.
                effect: &EffectDef::ExileLinkedToSource {
                    until_source_leaves: false,
                    object: EffectRecipientDef::TriggeringObject,
                    face_down: false,
                    then: None,
                },
            },
        ),
        AbilityDef::activated(
            "{2}, {T}: Draw a card, then discard a card.",
            &[
                CostDef::Mana(mana_cost!("{2}")),
                CostDef::TapSource,
            ],
            EffectDef::Sequence(&[
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            ]),
        ),
        AbilityDef::activated(
            "{T}: Put a card exiled with this artifact into its owner's graveyard. If it's a land \
             card, create a Treasure token. If it's a nonland card, create a 2/2 black Rogue creature \
             token.",
            &[CostDef::TapSource],
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Object(ParentBinding),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::LinkedExiles,
                exclude: None,
                minimum: 1,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::IfElseCondition {
                    // The card the cash-out chose, asked about while the choice still names it.
                    // A land pays a Treasure and anything else pays a body.
                    condition: &TriggerConditionDef::BoundObjectMatches {
                        binding: ParentBinding,
                        object: ObjectPredicateDef::HasType(CardType::Land),
                    },
                    then: &EffectDef::Sequence(&[
                        CONVERTER_RETURNS_THE_CARD,
                        EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))),
                    ]),
                    otherwise: &EffectDef::Sequence(&[
                        CONVERTER_RETURNS_THE_CARD,
                        EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                            TokenCharacteristics::creature(&["Rogue"], &[ManaColor::Black], 2, 2),
                        ))),
                    ]),
                },
            }),
        ),
    ]),
);

// NCC 109 — Tivit, Seller of Secrets
// Audit: unsupported — The vote procedure cannot collect evidence/bribery votes with an additional vote by this controller and apply one different token action per vote.
pub(in crate::card::sets) static TIVIT_SELLER_OF_SECRETS_109: CardRecord = CardRecord::new(
    "Tivit, Seller of Secrets",
    "5326a876-0c56-4368-af10-e9bbd1188d45",
    "Chris Rahn",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[
    &EXTRAVAGANT_REPLICATION,
    &LETHAL_SCHEME_36,
    &SEIZE_THE_SPOTLIGHT_52,
    &CURRENCY_CONVERTER,
    &TIVIT_SELLER_OF_SECRETS_109,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
