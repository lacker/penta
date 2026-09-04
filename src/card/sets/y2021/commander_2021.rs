//! Commander 2021 cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, CardArt, CardRules, CardSet,
    CardSupertype, CardType, CounterKind, EffectDef, EffectRecipientDef, ExilePlayDurationDef,
    ObjectPredicateDef, PlayerRelation, ScaledValueDef, TriggerEventDef, ValueDef, ZoneKind,
    abilities, tokens,
};
use crate::{TargetIndex, mana_cost};

// C21 53 — Laelia, the Blade Reforged
pub(in crate::card::sets) static LAELIA_THE_BLADE_REFORGED: CardRecord =
    CardRecord::new(
        CardSet::Commander2021,
    "Laelia, the Blade Reforged",
    "a3bb2881-e8fb-4fba-a9f9-d93e6ca24378",
    "Wisnu Tan",
        // Three mana with haste that attacks as a 3/3 on the turn it lands, and
        // grows every attack after because her own trigger feeds the other one.
        CardRules::new_creature(mana_cost!("{2}{R}"), &["Spirit", "Warrior"], 2, 2)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&[
                abilities::haste(),
                AbilityDef::triggered(
                    "Whenever Laelia attacks, exile the top card of your library. You may play that card this \
                     turn.",
                    TriggerEventDef::attacks(ObjectPredicateDef::Source),
                    EffectDef::ExileTopOfLibraryToPlay {
                        player: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                        // "You may play that card this turn", which is not the same as
                        // playing it for nothing: Laelia still pays for what she finds.
                        free: false,
                        face_down: false,
                        duration: ExilePlayDurationDef::ThisTurn,
                        spend_any_color: false,
                        play_condition: None,
                        cast_only: false,
                    },
                ),
                // One counter for the move rather than one per card, which is what "one
                // or more" means: her own attack trigger gives one, and a Breach exiling
                // three still gives one.
                AbilityDef::triggered(
                    "Whenever one or more cards are put into exile from your library and/or your graveyard, \
                     put a +1/+1 counter on Laelia.",
                    TriggerEventDef::CardsExiled {
                        // "From your library and/or your graveyard": one clause naming two zones,
                        // so either answers it and a move that takes cards from both is still one
                        // trigger.
                        zones: &[ZoneKind::Library, ZoneKind::Graveyard],
                        owner: PlayerRelation::You,
                    },
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                ),
            ]),
    );

// C21 65 — Pest Infestation
pub(in crate::card::sets) static PEST_INFESTATION: CardRecord = CardRecord::new(
    crate::card::CardSet::Commander2021,
    "Pest Infestation",
    "4720b4f2-e6af-4223-9250-a0ed21ed5693",
    "Brian Valeza",
    // Two mana per artifact answered, and the two Pests that come with each
    // are what makes paying it twice over worth doing.
    CardRules::new_sorcery(mana_cost!("{X}{X}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy up to X target artifacts and/or enchantments. Create twice X 1/1 black and \
         green Pest creature tokens with \"When this token dies, you gain 1 life.\"",
        // "Up to X", so a board with nothing worth destroying is no reason not to
        // cast it: the Pests come either way.
        &[AbilityTargetDef::up_to_chosen_x(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
            EffectDef::create_token(tokens::pest())
                // Twice X, and X is paid twice over in the cost, so every Pest costs a
                // mana and every artifact destroyed comes with two of them.
                .with_count(ValueDef::Scaled(&ScaledValueDef::new(ValueDef::ChosenX, 2)))
                .with_art(CardArt::new(
                    "d0ddbe3e-4a66-494d-9304-7471232549bf",
                    "Ilse Gort",
                )),
        ]),
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&LAELIA_THE_BLADE_REFORGED, &PEST_INFESTATION];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
