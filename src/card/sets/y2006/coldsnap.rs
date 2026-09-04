//! Coldsnap card records required by supported formats.

use super::{CardRecord, PrintingRecord};
use crate::CardType;
use crate::ObjectPredicateDef;
use crate::ZoneKind;
use crate::ZonePlacement;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    BattlefieldEntryModificationDef, CardRules, CardSet, CardSupertype, ComparisonDef, CounterKind,
    EffectDef, EffectRecipientDef, InstalledTriggerDef, ManaColor, PlayerRefDef, PlayerRelation,
    ReplacementEffectDef, TokenCharacteristics, TriggerConditionDef, TriggerEventDef, TurnStepDef,
    ValueDef, abilities,
};
use crate::{TargetIndex, mana_cost};

// CSP 33 — Flashfreeze
pub(in crate::card::sets) static FLASHFREEZE: CardRecord = CardRecord::new(
    crate::card::CardSet::Coldsnap,
    "Flashfreeze",
    "cefd9955-a195-4855-a00e-3809b96ca92b",
    "Brian Despain",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target red or green spell.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Spell,
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Color(ManaColor::Red),
                        ObjectPredicateDef::Color(ManaColor::Green),
                    ]),
                ]),
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Counter {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Graveyard,
            placement: ZonePlacement::Top,
        },
    )),
);

// CSP 54 — Deathmark
pub(in crate::card::sets) static DEATHMARK: CardRecord = CardRecord::new(
    crate::card::CardSet::Coldsnap,
    "Deathmark",
    "e72e8728-d0a0-4ee5-87c3-092ca94225e0",
    "Jeremy Jarvis",
    CardRules::new_sorcery(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target green or white creature.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Color(ManaColor::Green),
                    ObjectPredicateDef::Color(ManaColor::White),
                ]),
            ]),
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            can_regenerate: true,
            then: None,
        },
    )),
);

// CSP 138 — Mishra's Bauble
pub(in crate::card::sets) static MISHRA_S_BAUBLE: CardRecord = CardRecord::new(
    CardSet::Coldsnap,
    "Mishra's Bauble",
    "8a720448-017f-4f4a-9501-678245eaed17",
    "Chippy",
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
        EffectDef::Sequence(&[
            abilities::look_at_top_cards(
                PlayerRefDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
            EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
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
        ]),
    )),
);

// CSP 145 — Dark Depths
pub(in crate::card::sets) static DARK_DEPTHS: CardRecord = CardRecord::new(
    CardSet::Coldsnap,
    "Dark Depths",
    "92409c3a-fb1a-4205-9fe1-0f5affc7b21d",
    "Stephan Martiniere",
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

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&FLASHFREEZE, &DEATHMARK, &MISHRA_S_BAUBLE, &DARK_DEPTHS];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
