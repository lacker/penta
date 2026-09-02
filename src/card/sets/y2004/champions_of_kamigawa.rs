//! Champions of Kamigawa cards cataloged for the Vintage Cube.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AppliedEffectDef, CardArt, CardChoiceSourceDef, CardRules, CardSet,
    CardType, EffectDef, EffectRecipientDef, ObjectPredicateDef, PlayerRefDef, PlayerRelation,
    ResolvedEffectDurationDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement,
    abilities,
};
use crate::ids::ParentBinding;
use crate::mana_cost;

// CHK 193 — Through the Breach
pub(in crate::card::sets) static THROUGH_THE_BREACH: CardRecord = CardRecord::new_with_legacy_id(
    2190,
    "Through the Breach",
    CardArt::new("6da09e6a-2965-4855-bd41-41b41ba188fb", "Hugh Jamieson"),
    CardSet::ChampionsOfKamigawa,
    CardRules::new_instant(mana_cost!("{4}{R}"))
        .with_subtypes(&["Arcane"])
        .with_abilities(&[
            AbilityDef::spell(
                "You may put a creature card from your hand onto the battlefield. That creature gains haste. Sacrifice that creature at the beginning of the next end step.",
                EffectDef::WithZoneMoveResult {
                    // A minimum of zero is the printed "you may": the offer may be answered
                    // with nothing, and with no creature in hand it is never made at all.
                    effect: &const {
                        EffectDef::ChooseCards {
                            player: EffectRecipientDef::Controller,
                            sources: &const { [CardChoiceSourceDef::Zone(ZoneKind::Hand)] },
                            object: ObjectPredicateDef::HasType(CardType::Creature),
                            minimum: 0,
                            maximum: 1,
                            reveal: false,
                            destination: ZoneKind::Battlefield,
                            placement: ZonePlacement::Top,
                        }
                    },
                    binding: ParentBinding,
                    then: &const {
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::binding_zone_change_successors(
                                ParentBinding,
                            ),
                            effect: AppliedEffectDef::Composite(&const {
                                [
                                    AppliedEffectDef::add_ability(&const { abilities::haste() }),
                                    // The creature sacrifices itself rather than being named by a delayed
                                    // trigger the spell installs: it is the object that arrived, and it carries
                                    // the clause with it. Nothing else can name it -- the card was chosen only
                                    // as this spell resolved, and what entered is a new object.
                                    AppliedEffectDef::add_ability(&const {
                                        AbilityDef::triggered(
                                            "At the beginning of the next end step, sacrifice this creature.",
                                            TriggerEventDef::StepBegins {
                                                step: TurnStepDef::End,
                                                player: PlayerRelation::Any,
                                            },
                                            EffectDef::Sacrifice {
                                                object: EffectRecipientDef::Source,
                                            },
                                        )
                                    }),
                                ]
                            }),
                            duration: ResolvedEffectDurationDef::Permanent,
                        }
                    },
                },
            ),
            // Not a second spell ability and not a way to cast this card:
            // splice is a cast-time option on the card in hand, so the
            // clause exists to give the splice cost somewhere printed to
            // live, exactly as plot's does.
            abilities::splice_onto_arcane(mana_cost!("{2}{R}{R}")),
        ]),
);

// CHK 239 — Sakura-Tribe Elder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAKURA_TRIBE_ELDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("91c7707a-bae0-4196-bf26-d276f57b7369"),
    "Sakura-Tribe Elder",
    crate::card::CardArt::new("91c7707a-bae0-4196-bf26-d276f57b7369", "Carl Critchlow"),
    crate::card::CardSet::ChampionsOfKamigawa,
    crate::card::CardRules::unsupported(),
);

// CHK 268 — Sensei's Divining Top
pub(in crate::card::sets) static SENSEIS_DIVINING_TOP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4a08ca06-58db-4ce6-b490-be4bea8956a1"),
    "Sensei's Divining Top",
    CardArt::new("4a08ca06-58db-4ce6-b490-be4bea8956a1", "Michael Sutfin"),
    CardSet::ChampionsOfKamigawa,
    // One mana that fixes every draw for the rest of the game: the tap
    // trades the card it just arranged for itself, and the {1} sets up the
    // next one.
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::activated(
            "{1}: Look at the top three cards of your library, then put them back in any order.",
            &[AbilityCostDef::Mana(mana_cost!("{1}"))],
            abilities::look_at_top_cards_and_reorder(
                PlayerRefDef::EffectController,
                ValueDef::Constant(3),
            ),
        ),
        AbilityDef::activated(
            "{T}: Draw a card, then put this artifact on top of its owner's library.",
            &[AbilityCostDef::TapSource],
            // The draw and the trip back to the library are one clause: the Top is on
            // the battlefield as the card is drawn and gone by the time anything could
            // answer it, which is why it is never really spent.
            EffectDef::Sequence(
                &const {
                    [
                        EffectDef::DrawCards {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(1),
                        },
                        EffectDef::MoveToZone {
                            object: EffectRecipientDef::Source,
                            zone: ZoneKind::Library,
                            placement: ZonePlacement::Top,
                        },
                    ]
                },
            ),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &THROUGH_THE_BREACH,
    &SAKURA_TRIBE_ELDER,
    &SENSEIS_DIVINING_TOP,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
