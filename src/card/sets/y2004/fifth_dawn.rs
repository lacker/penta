//! Fifth Dawn cards cataloged for the Vintage Cube.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, BattlefieldEntryModificationDef, CardArt, CardRules, CardSet,
    CardType, CounterKind, EffectDef, EffectRecipientDef, GraveyardPlayPermissionDef, ManaColor,
    ObjectPredicateDef, PlayActionMatcherDef, PlayRestrictionDef, PlayerRelation,
    ReplacementEffectDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::{TargetIndex, mana_cost};

// 5DN 27 — Condescend
pub(in crate::card::sets) static CONDESCEND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e8303b80-e29a-46b8-90b0-c0cfe551b435"),
    "Condescend",
    CardArt::new("e8303b80-e29a-46b8-90b0-c0cfe551b435", "Ron Spears"),
    CardSet::FifthDawn,
    // The scry is what keeps this live once X is too small to counter
    // anything, which is why a tempo deck can cast it for one.
    CardRules::new_instant(mana_cost!("{X}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell unless its controller pays {X}. Scry 2. (Look at the top two cards \
         of your library, then put any number of them on the bottom and the rest on top in any \
         order.)",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            // The demand is the same X this was cast for, so paying more for
            // it raises what the other player has to find.
            abilities::counter_target_unless_paid(ValueDef::ChosenX),
            // Scrying happens either way: the spell resolving through does
            // not stop the second half.
            abilities::scry(ValueDef::Constant(2)),
        ]),
    )),
);

// 5DN 36 — Serum Visions
pub(in crate::card::sets) static SERUM_VISIONS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("77e241f0-4cdc-4e37-b5b1-6f47f385d381"),
    "Serum Visions",
    CardArt::new("4bc61952-88ba-447a-835a-f1e9643fcd0d", "Ben Thompson"),
    CardSet::FifthDawn,
    // The draw comes first and the scry second, which is the whole
    // difference from Preordain: this fixes the next two draws, not this one.
    CardRules::new_sorcery(mana_cost!("{U}")).with_ability(AbilityDef::spell(
        "Draw a card. Scry 2. (Look at the top two cards of your library, then put any number of \
         them on the bottom and the rest on top in any order.)",
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
            abilities::scry(ValueDef::Constant(2)),
        ]),
    )),
);

// 5DN 55 — Night's Whisper
pub(in crate::card::sets) static NIGHTS_WHISPER: CardRecord = CardRecord::new_with_legacy_id(
    2300,
    "Night's Whisper",
    CardArt::new("61f0c6f6-b90d-4eb1-a5db-86e0a3997501", "David Martin"),
    CardSet::FifthDawn,
    // Two mana and two life for two which is the rate every black
    // deck in the cube is happy to pay and no other colour is offered.
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_ability(AbilityDef::spell(
        "You draw two cards and lose 2 life.",
        // "You draw two cards and lose 2 life" is one sentence about you, so the
        // life is not a cost and nothing stops it: a player at 2 who casts this
        // draws the two cards and loses the game.
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ]),
    )),
);

// 5DN 85 — Dawn's Reflection
pub(in crate::card::sets) static DAWNS_REFLECTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("131a124f-f11e-4ea1-a7b2-b94eea988d4e"),
    "Dawn's Reflection",
    CardArt::new("131a124f-f11e-4ea1-a7b2-b94eea988d4e", "John Avon"),
    CardSet::FifthDawn,
    CardRules::new_enchantment(mana_cost!("{3}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_land(),
            AbilityDef::triggered_mana(
                "Whenever enchanted land is tapped for mana, its controller adds an additional two mana in any combination of colors.",
                TriggerEventDef::tapped_for_mana(ObjectPredicateDef::AttachedToSource),
                EffectDef::AddMana(
                    AddManaEffectDef::combination(&ManaColor::COLORS, 2)
                        .to_triggering_objects_controller(),
                ),
            ),
        ]),
);

// 5DN 86 — Eternal Witness
pub(in crate::card::sets) static ETERNAL_WITNESS: CardRecord = CardRecord::new_with_legacy_id(
    2266,
    "Eternal Witness",
    CardArt::new("c7e10ca7-1e5d-4224-82cf-798a4d436d72", "Terese Nielsen"),
    CardSet::FifthDawn,
    // A 2/1 body nobody plays it for. What it is worth is the card, and
    // every way of making it enter again is worth another one.
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Human", "Shaman"], 2, 1).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, you may return target card from your graveyard to your \
             hand.",
            // Your own graveyard, and any card in it: a land comes back as readily as
            // the spell that killed the Witness.
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            // The target is chosen as the trigger goes on the stack; the "may" is
            // answered as it resolves. A Witness whose card was exiled in response
            // still asks, and taking it back is what the answer refuses.
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &const {
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        zone: ZoneKind::Hand,
                        placement: ZonePlacement::Top,
                    }
                },
            },
        ),
    ),
);

// 5DN 110 — Clock of Omens
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLOCK_OF_OMENS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0ffce71b-eb60-4649-a62b-a1b4acaa9d2d"),
    "Clock of Omens",
    crate::card::CardArt::new(
        "0ffce71b-eb60-4649-a62b-a1b4acaa9d2d",
        "Alex Horley-Orlandelli",
    ),
    crate::card::CardSet::FifthDawn,
    crate::card::CardRules::unsupported(),
);

// 5DN 114 — Crucible of Worlds
pub(in crate::card::sets) static CRUCIBLE_OF_WORLDS: CardRecord = CardRecord::new_with_legacy_id(
    2203,
    "Crucible of Worlds",
    CardArt::new("312a6058-de08-487d-95bd-b3c56807fdd6", "Ron Spencer"),
    CardSet::FifthDawn,
    // One line, and it turns every fetchland, every Wasteland, and every
    // land anything made you discard back into a land drop.
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::static_ability(
        "You may play lands from your graveyard.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Controller,
            effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromGraveyard(
                // A permission rather than a prohibition, in the same vocabulary: which
                // action it opens, and which cards it opens it for.
                GraveyardPlayPermissionDef::unlimited(PlayRestrictionDef::new(
                    PlayActionMatcherDef::PlayLand,
                    ObjectPredicateDef::HasType(CardType::Land),
                )),
            )),
        },
    )),
);

// 5DN 118 — Engineered Explosives
pub(in crate::card::sets) static ENGINEERED_EXPLOSIVES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8492a272-e595-4f94-a6eb-08d29f211fd6"),
    "Engineered Explosives",
    CardArt::new("8492a272-e595-4f94-a6eb-08d29f211fd6", "Ron Spears"),
    CardSet::FifthDawn,
    CardRules::new_artifact(mana_cost!("{X}")).with_abilities(&[
            AbilityDef::as_enters(
                "Sunburst (This artifact enters with a charge counter on it for each color of mana spent to cast it.)",
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::AddCountersValue {
                        kind: CounterKind::named("charge"),
                        amount: ValueDef::ColorsOfManaSpent,
                    },
                ),
            ),
            AbilityDef::activated(
                "{2}, Sacrifice this artifact: Destroy each nonland permanent with mana value equal to the number of charge counters on this artifact.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{2}")),
                    AbilityCostDef::SacrificeSource,
                ],
                EffectDef::Destroy {
                    object: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                            ObjectPredicateDef::ManaValueEqualTo(ValueDef::CountersOnSource(
                                CounterKind::named("charge"),
                            )),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    can_regenerate: true,
                    then: None,
                },
            ),
        ]),
);

// 5DN 143 — Pentad Prism
pub(in crate::card::sets) static PENTAD_PRISM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("672b9b16-daef-44e6-9a3a-cfd9f3c78bc7"),
    "Pentad Prism",
    CardArt::new("672b9b16-daef-44e6-9a3a-cfd9f3c78bc7", "David Martin"),
    CardSet::FifthDawn,
    // Two mana of two colours for two mana of any colours, later: a ritual
    // that waits, which is why it wants a deck already casting things in
    // more than one colour on turn two.
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
            AbilityDef::as_enters(
                "Sunburst (This artifact enters with a charge counter on it for each color of mana spent \
                 to cast it.)",
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::AddCountersValue {
                        kind: CounterKind::named("charge"),
                        amount: ValueDef::ColorsOfManaSpent,
                    },
                ),
            ),
            AbilityDef::activated_mana(
                "Remove a charge counter from this artifact: Add one mana of any color.",
                &[AbilityCostDef::RemoveCountersFromSource {
                    kind: CounterKind::named("charge"),
                    amount: 1,
                }],
                EffectDef::AddMana(AddManaEffectDef::any_color()),
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &CONDESCEND,
    &SERUM_VISIONS,
    &NIGHTS_WHISPER,
    &DAWNS_REFLECTION,
    &ETERNAL_WITNESS,
    &CLOCK_OF_OMENS,
    &CRUCIBLE_OF_WORLDS,
    &ENGINEERED_EXPLOSIVES,
    &PENTAD_PRISM,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
