//! SOS card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::CostQuantityDef;
use crate::card::{
    AbilityCostDef, AbilityCostList, AbilityDef, AddManaEffectDef, AppliedEffectDef, CardArt,
    CardRules, CardSet, CardType, EffectDef, EffectRecipientDef, ManaColor, ManaCost,
    ObjectPredicateDef, PlayerRelation, SpellAdditionalCostDef, ValueDef, ZoneKind, abilities,
};
use crate::mana_cost;

// SOS 12 — Elite Interceptor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELITE_INTERCEPTOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2970683e-e69c-42cb-a067-34abd56fb42b"),
    "Elite Interceptor",
    crate::card::CardArt::new("2970683e-e69c-42cb-a067-34abd56fb42b", "Lindsey Look"),
    crate::card::CardSet::SecretsOfStrixhaven,
    crate::card::CardRules::unsupported(),
);

// SOS 42 — Deluge Virtuoso
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DELUGE_VIRTUOSO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2e3b16ed-8727-48fd-8b1f-c0cbd329385e"),
    "Deluge Virtuoso",
    crate::card::CardArt::new("2e3b16ed-8727-48fd-8b1f-c0cbd329385e", "Justine Cruz"),
    crate::card::CardSet::SecretsOfStrixhaven,
    crate::card::CardRules::unsupported(),
);

// SOS 68 — Spellbook Seeker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPELLBOOK_SEEKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cc44eaa4-59a4-419e-b1d1-d92f354ff588"),
    "Spellbook Seeker",
    crate::card::CardArt::new("cc44eaa4-59a4-419e-b1d1-d92f354ff588", "Scott Murphy"),
    crate::card::CardSet::SecretsOfStrixhaven,
    crate::card::CardRules::unsupported(),
);

// SOS 241 — Vicious Rivalry
pub(in crate::card::sets) static VICIOUS_RIVALRY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6fa9cd18-3181-4373-ab65-49bf9de9487f"),
    "Vicious Rivalry",
    CardArt::new("6fa9cd18-3181-4373-ab65-49bf9de9487f", "Chris Rallis"),
    CardSet::SecretsOfStrixhaven,
    CardRules::new_sorcery(mana_cost!("{2}{B}{G}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, pay X life.\nDestroy all artifacts and \
             creatures with mana value X or less.",
            &[],
            SpellAdditionalCostDef::pay_life(CostQuantityDef::ChosenX),
            EffectDef::Destroy {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Creature),
                        ]),
                        ObjectPredicateDef::ManaValueAtMostValue(ValueDef::ChosenX),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                can_regenerate: true,
                then: None,
            },
        ),
    ),
);

// SOS 242 — Visionary's Dance
pub(in crate::card::sets) static VISIONARY_S_DANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("846a0e79-a530-429e-8f7f-4b87f1b0156e"),
    "Visionary's Dance",
    CardArt::new(
        "846a0e79-a530-429e-8f7f-4b87f1b0156e",
        "Josiah \"Jo\" Cameron",
    ),
    CardSet::SecretsOfStrixhaven,
    // Seven mana is more than a limited deck usually reaches, which is what
    // the discard half is for: the card is never stranded in hand.
    CardRules::new_sorcery(mana_cost!("{5}{U}{R}")).with_abilities(&[
        AbilityDef::spell(
            "Create two 3/3 blue and red Elemental creature tokens with flying.",
            EffectDef::create_creature_token(
                &["Elemental"],
                &[ManaColor::Blue, ManaColor::Red],
                3,
                3,
            )
            .with_abilities(&[abilities::flying()])
            .with_amount(2),
        ),
        AbilityDef::activated(
            "{2}, Discard this card: Look at the top two cards of your library. Put one of them into your hand and the other into your graveyard.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}")),
                AbilityCostDef::DiscardSource,
            ],
            abilities::look_at_top_cards_choose_to_hand_rest_graveyard(
                ValueDef::Constant(2),
                ObjectPredicateDef::Any,
                1,
                1,
            ),
        )
        // Activated from hand, which is the only place a card can be
        // discarded from.
        .with_source_zones(&[ZoneKind::Hand]),
    ]),
);

// SOS 255 — Fields of Strife
pub(in crate::card::sets) static FIELDS_OF_STRIFE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3dc7a4c3-c356-4fba-bea0-e8788da3eb57"),
    "Fields of Strife",
    CardArt::new("3dc7a4c3-c356-4fba-bea0-e8788da3eb57", "Josu Solano"),
    CardSet::SecretsOfStrixhaven,
    // Titan's Grave in red and white, and its sink costs both colours: the
    // surveil is only reachable in the deck the land is already fixing for.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {R} or {W}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Red,
                ManaColor::White,
            ])),
        ),
        AbilityDef::activated(
            "{2}{R}{W}, {T}: Surveil 1.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{R}{W}")),
                AbilityCostDef::TapSource,
            ],
            abilities::surveil(ValueDef::Constant(1)),
        ),
    ]),
);

/// The SOS cycle of tapped duals that surveil. Unlike the flat-cost tapped
/// duals elsewhere, each of these prices its surveil in its own two colours,
/// so the activation cost is a parameter beside the colours rather than
/// something the helper can derive.
///
/// The abilities are added one at a time in printed order: an array holding
/// the parameterized ones could not be given a `'static` lifetime.
const fn guildhall_surveil_land(
    mana_text: &'static str,
    colors: &'static [ManaColor],
    surveil_text: &'static str,
    surveil_cost: ManaCost,
) -> CardRules {
    CardRules::new_land(&[])
        .with_ability(abilities::enters_tapped(CardType::Land))
        .with_ability(AbilityDef::activated_mana(
            mana_text,
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(colors)),
        ))
        // A cost list rather than a slice: the mana cost is a parameter, and
        // a slice holding it could not be given a 'static lifetime.
        .with_ability(AbilityDef::activated_with_cost_list_and_targets(
            surveil_text,
            AbilityCostList::two(
                AbilityCostDef::Mana(surveil_cost),
                AbilityCostDef::TapSource,
            ),
            &[],
            abilities::surveil(ValueDef::Constant(1)),
        ))
}

// SOS 256 — Forum of Amity
pub(in crate::card::sets) static FORUM_OF_AMITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1de6c6cc-0c55-4997-8623-d7f796bd9ab8"),
    "Forum of Amity",
    CardArt::new("1de6c6cc-0c55-4997-8623-d7f796bd9ab8", "Richard Wright"),
    CardSet::SecretsOfStrixhaven,
    guildhall_surveil_land(
        "{T}: Add {W} or {B}.",
        &[ManaColor::White, ManaColor::Black],
        "{2}{W}{B}, {T}: Surveil 1. (Look at the top card of your library. You may put it into \
         your graveyard.)",
        mana_cost!("{2}{W}{B}"),
    ),
);

// SOS 258 — Paradox Gardens
pub(in crate::card::sets) static PARADOX_GARDENS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dbc3447e-1329-4ea1-b4ca-b321b0ffec8f"),
    "Paradox Gardens",
    CardArt::new("dbc3447e-1329-4ea1-b4ca-b321b0ffec8f", "Leon Tukker"),
    CardSet::SecretsOfStrixhaven,
    guildhall_surveil_land(
        "{T}: Add {G} or {U}.",
        &[ManaColor::Green, ManaColor::Blue],
        "{2}{G}{U}, {T}: Surveil 1. (Look at the top card of your library. You may put it into \
         your graveyard.)",
        mana_cost!("{2}{G}{U}"),
    ),
);

// SOS 259 — Petrified Hamlet
pub(in crate::card::sets) static PETRIFIED_HAMLET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("355dd460-b0e9-41f2-a058-b7f7e39ac387"),
    "Petrified Hamlet",
    CardArt::new("355dd460-b0e9-41f2-a058-b7f7e39ac387", "Richard Wright"),
    CardSet::SecretsOfStrixhaven,
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "As this land enters, choose a land card name.",
            crate::card::ReplacementEffectDef::BindOutput {
                effect: &abilities::choose_card_name_as_enters(
                    crate::card::CardNameSetDef::LandCardNames,
                ),
                binding: Binding!("petrified_hamlet_name"),
            },
        ),
        abilities::cannot_activate_nonmana_abilities_with_name(
            "Activated abilities of sources with the chosen name can't be activated unless they're mana abilities.",
            crate::card::CardNameDef::Binding(Binding!("petrified_hamlet_name")),
        ),
        AbilityDef::static_ability(
            "Lands with the chosen name have “{T}: Add {C}.”",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::NameEquals(
                            crate::card::CardNameDef::Binding(Binding!("petrified_hamlet_name")),
                        ),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::tap_for(ManaColor::Colorless)),
            },
        ),
        abilities::tap_for(ManaColor::Colorless),
    ]),
);

// SOS 262 — Spectacle Summit
pub(in crate::card::sets) static SPECTACLE_SUMMIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a0a66f7b-eab4-45da-8895-c2c2c7eb05f8"),
    "Spectacle Summit",
    CardArt::new("a0a66f7b-eab4-45da-8895-c2c2c7eb05f8", "Andreas Zafiratos"),
    CardSet::SecretsOfStrixhaven,
    guildhall_surveil_land(
        "{T}: Add {U} or {R}.",
        &[ManaColor::Blue, ManaColor::Red],
        "{2}{U}{R}, {T}: Surveil 1. (Look at the top card of your library. You may put it into \
         your graveyard.)",
        mana_cost!("{2}{U}{R}"),
    ),
);

// SOS 266 — Titan's Grave
pub(in crate::card::sets) static TITAN_S_GRAVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a9ab41c8-3ee2-4676-9b8b-20c34d9f5f21"),
    "Titan's Grave",
    CardArt::new(
        "a9ab41c8-3ee2-4676-9b8b-20c34d9f5f21",
        "Lorenzo Lanfranconi",
    ),
    CardSet::SecretsOfStrixhaven,
    // A tapped dual whose late-game half costs more than the land itself,
    // which is the point: it is a land first and a mana sink only when the
    // draw step has nothing better.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {B} or {G}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Green,
            ])),
        ),
        AbilityDef::activated(
            "{2}{B}{G}, {T}: Surveil 1.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{B}{G}")),
                AbilityCostDef::TapSource,
            ],
            abilities::surveil(ValueDef::Constant(1)),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ELITE_INTERCEPTOR,
    &DELUGE_VIRTUOSO,
    &SPELLBOOK_SEEKER,
    &VICIOUS_RIVALRY,
    &VISIONARY_S_DANCE,
    &FIELDS_OF_STRIFE,
    &FORUM_OF_AMITY,
    &PARADOX_GARDENS,
    &PETRIFIED_HAMLET,
    &SPECTACLE_SUMMIT,
    &TITAN_S_GRAVE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
