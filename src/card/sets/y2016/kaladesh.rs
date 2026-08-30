//! Kaladesh cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AttackEventMatcherDef, CardArt, CardRules, CardSet, CardSupertype, CardType,
    DiscardSelectionDef, EffectDef, EffectRecipientDef, ManaColor, ObjectPredicateDef,
    ObjectSetDef, PlayerRelation, TargetChooserDef, TriggerEventDef, ValueDef, ZoneKind,
    ZonePlacement, abilities,
};
use crate::ids::ObjectSetBindingIndex;
use crate::{TargetIndex, mana_cost};

/// The fastland cycle: untapped while the board is still small, an expensive
/// tapped land after that. Every one of the ten prints this same clause, and
/// only the colour pair below it differs.
static FAST_LAND_ENTERS: AbilityDef = abilities::fast_land_enters(
    "This land enters tapped unless you control two or fewer other lands.",
);

// KLD 60 — Paradoxical Outcome
const OUTCOME_OWNED_BY_YOU: ObjectSetBindingIndex = ObjectSetBindingIndex::new(1);
pub(in crate::card::sets) static PARADOXICAL_OUTCOME: CardRecord = CardRecord::new_with_legacy_id(
    2242,
    "Paradoxical Outcome",
    CardArt::new("17e50157-bf49-4c5f-9b8a-bf73484e63a5", "Nils Hamm"),
    CardSet::Kaladesh,
    // Four mana and a fistful of Moxen back, which is a bad rate for a deck
    // that has to pay for them again and a broken one for a deck that does
    // not.
    CardRules::new_instant(mana_cost!("{3}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return any number of target nonland, nontoken permanents you control to their owners' \
         hands. Draw a card for each card returned to your hand this way.",
        &[AbilityTargetDef {
            predicate: AbilityTargetPredicate::Object {
                // A permanent that is neither a land nor a token. The slot names the
                // controller, so the predicate only has to say what kind of thing it is.
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
            minimum: 0,
            maximum: AbilityTargetDef::UNLIMITED,
            divided_total: None,
            another: false,
            excludes_source: false,
            chooser: TargetChooserDef::Controller,
        }],
        // Only the targets still legal as this resolves are returned, which is what
        // "each card returned this way" counts.
        abilities::bind_objects_then(
            crate::card::ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::LegalTargets(
                TargetIndex::PRIMARY,
            )),
            ObjectSetBindingIndex::PRIMARY,
            &const {
                abilities::bind_objects_then(
                    crate::card::ObjectCollectionSourceDef::ObjectSet(
                        ObjectSetDef::MatchingBinding {
                            binding: ObjectSetBindingIndex::PRIMARY,
                            object: ObjectPredicateDef::OwnedBy(PlayerRelation::You),
                        },
                    ),
                    OUTCOME_OWNED_BY_YOU,
                    // The draw counts what reached your hand, which is not always what left the
                    // battlefield: a permanent you control but do not own goes back to somebody
                    // else's hand and pays you nothing. The count is taken before the move,
                    // because afterwards the cards have new identities.
                    &EffectDef::Sequence(
                        &const {
                            [
                                EffectDef::MoveToZone {
                                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                        ObjectSetBindingIndex::PRIMARY,
                                    )),
                                    zone: ZoneKind::Hand,
                                    placement: ZonePlacement::Top,
                                },
                                EffectDef::DrawCards {
                                    recipient: EffectRecipientDef::Controller,
                                    amount: ValueDef::BoundObjectCount(OUTCOME_OWNED_BY_YOU),
                                },
                            ]
                        },
                    ),
                )
            },
        ),
    )),
);

// KLD 110 — Chandra, Torch of Defiance
pub(in crate::card::sets) static CHANDRA_TORCH_OF_DEFIANCE: CardRecord =
    CardRecord::new_with_legacy_id(
        2217,
        "Chandra, Torch of Defiance",
        CardArt::new("ff8086cd-b868-4f4e-823e-2635ad7ebc07", "Magali Villeneuve"),
        CardSet::Kaladesh,
        // Four abilities and no bad one: she draws, she ramps, she kills, and if
        // the game somehow goes long she ends it by herself.
        CardRules::new_planeswalker(mana_cost!("{2}{R}{R}"), &["Chandra"], 4)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&[
                AbilityDef::activated(
                    "+1: Exile the top card of your library. You may cast that card. If you don't, Chandra, \
                     Torch of Defiance deals 2 damage to each opponent.",
                    &[AbilityCostDef::Loyalty(1)],
                    EffectDef::ExileTopAndMayCast {
                        player: EffectRecipientDef::Controller,
                        // "If you don't" is the whole of the first ability's tension: the exile
                        // happens either way, and the card is either spent now at its own cost or
                        // traded for two damage.
                        otherwise: Some(&EffectDef::DealDamage {
                            recipient: EffectRecipientDef::Opponent,
                            amount: ValueDef::Constant(2),
                        }),
                    },
                ),
                // A loyalty ability is never a mana ability (CR 605.1a), so this one uses
                // the stack like the rest of her.
                AbilityDef::activated(
                    "+1: Add {R}{R}.",
                    &[AbilityCostDef::Loyalty(1)],
                    EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red).with_amount(2)),
                ),
                AbilityDef::activated_with_targets(
                    "−3: Chandra, Torch of Defiance deals 4 damage to target creature.",
                    &[AbilityCostDef::Loyalty(-3)],
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    )],
                    EffectDef::DealDamage {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(4),
                    },
                ),
                AbilityDef::activated(
                    "−7: You get an emblem with \"Whenever you cast a spell, this emblem deals 5 damage to \
                     any target.\"",
                    &[AbilityCostDef::Loyalty(-7)],
                    EffectDef::create_emblem(
                        "Chandra, Torch of Defiance emblem",
                        &[AbilityDef::triggered_with_targets(
                                "Whenever you cast a spell, this emblem deals 5 damage to any target.",
                                TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(PlayerRelation::You)),
                                &[AbilityTargetDef::exactly_one(
                                    AbilityTargetPredicate::AnyTarget,
                                )],
                                EffectDef::DealDamage {
                                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                    amount: ValueDef::Constant(5),
                                },
                            )],
                    ),
                ),
            ]),
    );

// KLD 138 — Thriving Grubs
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THRIVING_GRUBS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bbc3184a-eeda-4f22-92de-257c20cff6e2"),
    "Thriving Grubs",
    crate::card::CardArt::new("bbc3184a-eeda-4f22-92de-257c20cff6e2", "Steve Prescott"),
    crate::card::CardSet::Kaladesh,
    crate::card::CardRules::unsupported(),
);

// KLD 212 — Filigree Familiar
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FILIGREE_FAMILIAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9cc9ecfd-6cf0-4488-a14a-afec1bc0d253"),
    "Filigree Familiar",
    crate::card::CardArt::new("9cc9ecfd-6cf0-4488-a14a-afec1bc0d253", "Izzy"),
    crate::card::CardSet::Kaladesh,
    crate::card::CardRules::unsupported(),
);

// KLD 230 — Renegade Freighter
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RENEGADE_FREIGHTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7a10e2c3-0132-4eb2-94f0-5915caca2a17"),
    "Renegade Freighter",
    crate::card::CardArt::new("7a10e2c3-0132-4eb2-94f0-5915caca2a17", "Izzy"),
    crate::card::CardSet::Kaladesh,
    crate::card::CardRules::unsupported(),
);

// KLD 235 — Smuggler's Copter
pub(in crate::card::sets) static SMUGGLER_S_COPTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7832abb5-5107-4603-904e-491b221bd3e3"),
    "Smuggler's Copter",
    CardArt::new(
        "7832abb5-5107-4603-904e-491b221bd3e3",
        "Florian de Gesincourt",
    ),
    CardSet::Kaladesh,
    // Two mana for a 3/3 flier that any one creature can turn on, and that
    // fixes every draw it connects with. Banned in Standard for exactly
    // that.
    CardRules::new_vehicle(mana_cost!("{2}"), 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever this Vehicle attacks or blocks, you may draw a card. If you do, discard a card.",
            // "Attacks or blocks" is one printed clause with two ways in, so it is one
            // ability rather than two: a Copter that does both in a turn still loots
            // once for each.
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::Attacks(AttackEventMatcherDef::any(ObjectPredicateDef::Source)),
                TriggerEventDef::Blocks {
                    blocked: ObjectPredicateDef::Any,
                },
            ]),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                // "If you do" rather than a second clause: declining the draw declines the
                // discard with it.
                effect: &EffectDef::Sequence(&[
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
            },
        ),
        abilities::crew(
            "Crew 1 (Tap any number of creatures you control with total power 1 or more: This \
             Vehicle becomes an artifact creature until end of turn.)",
            1,
        ),
    ]),
);

// KLD 243 — Blooming Marsh
pub(in crate::card::sets) static BLOOMING_MARSH: CardRecord = CardRecord::new_with_legacy_id(
    2136,
    "Blooming Marsh",
    CardArt::new("90da33d4-fe9c-42fe-b326-2fe337dc3ecd", "Adam Paquette"),
    CardSet::Kaladesh,
    CardRules::new_land(&[]).with_abilities(&[
        FAST_LAND_ENTERS,
        AbilityDef::activated_mana(
            "{T}: Add {B} or {G}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Green,
            ])),
        ),
    ]),
);

// KLD 244 — Botanical Sanctum
pub(in crate::card::sets) static BOTANICAL_SANCTUM: CardRecord = CardRecord::new_with_legacy_id(
    2137,
    "Botanical Sanctum",
    CardArt::new("8744471b-a528-47d9-84d0-4526273f55e9", "Christine Choi"),
    CardSet::Kaladesh,
    CardRules::new_land(&[]).with_abilities(&[
        FAST_LAND_ENTERS,
        AbilityDef::activated_mana(
            "{T}: Add {G} or {U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Green,
                ManaColor::Blue,
            ])),
        ),
    ]),
);

// KLD 245 — Concealed Courtyard
pub(in crate::card::sets) static CONCEALED_COURTYARD: CardRecord = CardRecord::new_with_legacy_id(
    2138,
    "Concealed Courtyard",
    CardArt::new("c8769e97-aee8-4466-a9d7-0f4245ae4a97", "Jung Park"),
    CardSet::Kaladesh,
    CardRules::new_land(&[]).with_abilities(&[
        FAST_LAND_ENTERS,
        AbilityDef::activated_mana(
            "{T}: Add {W} or {B}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Black,
            ])),
        ),
    ]),
);

// KLD 246 — Inspiring Vantage
pub(in crate::card::sets) static INSPIRING_VANTAGE: CardRecord = CardRecord::new_with_legacy_id(
    2139,
    "Inspiring Vantage",
    CardArt::new("160ac412-005f-48ca-a204-10207307c6c2", "Jonas De Ro"),
    CardSet::Kaladesh,
    CardRules::new_land(&[]).with_abilities(&[
        FAST_LAND_ENTERS,
        AbilityDef::activated_mana(
            "{T}: Add {R} or {W}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Red,
                ManaColor::White,
            ])),
        ),
    ]),
);

// KLD 249 — Spirebluff Canal
pub(in crate::card::sets) static SPIREBLUFF_CANAL: CardRecord = CardRecord::new_with_legacy_id(
    2140,
    "Spirebluff Canal",
    CardArt::new("4e587ea7-0632-4789-ba75-3c410da2bb96", "Adam Paquette"),
    CardSet::Kaladesh,
    CardRules::new_land(&[]).with_abilities(&[
        FAST_LAND_ENTERS,
        AbilityDef::activated_mana(
            "{T}: Add {U} or {R}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[ManaColor::Blue, ManaColor::Red])),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &PARADOXICAL_OUTCOME,
    &CHANDRA_TORCH_OF_DEFIANCE,
    &THRIVING_GRUBS,
    &FILIGREE_FAMILIAR,
    &RENEGADE_FREIGHTER,
    &SMUGGLER_S_COPTER,
    &BLOOMING_MARSH,
    &BOTANICAL_SANCTUM,
    &CONCEALED_COURTYARD,
    &INSPIRING_VANTAGE,
    &SPIREBLUFF_CANAL,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
