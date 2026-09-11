//! Streets of New Capenna Commander cards cataloged for the Vintage Cube
//! pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::CostDef;
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
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::tokens;
use crate::ids::ParentBinding;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet =
    crate::card::CardSet::new("NCC", "streets-of-new-capenna-commander");

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// NCC 81 — Currency Converter
/// The card goes back to the graveyard it came from -- its owner's, which is
/// where a card exiled from a graveyard belongs however it got to exile.
static CONVERTER_RETURNS_THE_CARD: EffectDef = EffectDef::MoveToZone {
    object: EffectRecipientDef::object(ObjectRefDef::Binding(ParentBinding)),
    zone: ZoneKind::Graveyard,
    placement: ZonePlacement::Top,
};

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
                        EffectDef::create_token(tokens::treasure()),
                    ]),
                    otherwise: &EffectDef::Sequence(&[
                        CONVERTER_RETURNS_THE_CARD,
                        EffectDef::create_token(tokens::creature(
                            &["Rogue"],
                            &[ManaColor::Black],
                            2,
                            2,
                        )),
                    ]),
                },
            }),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&CURRENCY_CONVERTER];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
