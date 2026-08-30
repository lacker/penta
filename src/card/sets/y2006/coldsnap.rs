//! Coldsnap card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    BattlefieldEntryModificationDef, CardArt, CardRules, CardSet, CardSupertype, ComparisonDef,
    CounterKind, EffectDef, EffectRecipientDef, InstalledTriggerDef, ManaColor, PlayerRefDef,
    PlayerRelation, ReplacementEffectDef, TokenCharacteristics, TriggerConditionDef,
    TriggerEventDef, TurnStepDef, ValueDef, abilities,
};
use crate::{TargetIndex, mana_cost};

// CSP 138 — Mishra's Bauble
pub(in crate::card::sets) static MISHRA_S_BAUBLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8a720448-017f-4f4a-9501-678245eaed17"),
    "Mishra's Bauble",
    CardArt::new("8a720448-017f-4f4a-9501-678245eaed17", "Chippy"),
    CardSet::Coldsnap,
    // A free artifact that replaces itself a turn later. The looking is
    // incidental; what the card is played for is being an artifact that cost
    // nothing and a card that comes back.
    CardRules::new_artifact(mana_cost!("{0}")).with_ability(AbilityDef::activated_with_targets(
        "{T}, Sacrifice this artifact: Look at the top card of target player's library. Draw a \
         card at the beginning of the next turn's upkeep.",
        &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        abilities::look_at_top_cards_then(
            PlayerRefDef::Target(TargetIndex::PRIMARY),
            ValueDef::Constant(1),
            &EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                "Draw a card at the beginning of the next turn's upkeep.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::Any,
                },
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ))),
        ),
    )),
);

// CSP 145 — Dark Depths
pub(in crate::card::sets) static DARK_DEPTHS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("92409c3a-fb1a-4205-9fe1-0f5affc7b21d"),
    "Dark Depths",
    CardArt::new("92409c3a-fb1a-4205-9fe1-0f5affc7b21d", "Stephan Martiniere"),
    CardSet::Coldsnap,
    // Thirty mana the long way round, or none at all if something else takes
    // the counters off.
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_supertype(CardSupertype::Snow)
        .with_abilities(&[
            AbilityDef::as_enters(
                "Dark Depths enters with ten ice counters on it.",
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::AddCounters {
                        kind: CounterKind::named("ice"),
                        amount: 10,
                    },
                ),
            ),
            AbilityDef::activated(
                "{3}: Remove an ice counter from Dark Depths.",
                &[AbilityCostDef::Mana(mana_cost!("{3}"))],
                EffectDef::RemoveCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::named("ice"),
                    amount: ValueDef::Constant(1),
                },
            ),
            // A state trigger (CR 603.8): it has no event, and it fires whenever the
            // counters are gone -- however they went. Removing them all at once is
            // what the deck is really built to do.
            AbilityDef::triggered_if(
                "When Dark Depths has no ice counters on it, sacrifice it. If you do, create Marit Lage, \
                 a legendary 20/20 black Avatar creature token with flying and indestructible.",
                TriggerEventDef::StateCondition,
                &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("ice"),
                    comparison: ComparisonDef::Equal,
                    amount: 0,
                },
                // "Sacrifice it. If you do, create Marit Lage." Nothing stops a player
                // sacrificing their own permanent, so the only way the sacrifice fails is
                // that the land is no longer there to sacrifice -- which is what this asks,
                // and why an answer in response to the trigger denies the token.
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceOnBattlefield,
                    then: &EffectDef::Sequence(&[
                        EffectDef::Sacrifice {
                            object: EffectRecipientDef::Source,
                        },
                        // Twenty power for no mana at all, which is what the ten counters are
                        // paying for. Legendary, so a second one is not a plan.
                        EffectDef::create_token(TokenCharacteristics::creature(&["Avatar"], &[ManaColor::Black], 20, 20)
                                .with_supertype(CardSupertype::Legendary)
                                .with_name("Marit Lage")
                                .with_abilities(&[abilities::flying(), abilities::indestructible()])),
                    ]),
                },
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&MISHRA_S_BAUBLE, &DARK_DEPTHS];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
