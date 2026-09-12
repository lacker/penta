//! Born of the Gods card records required by supported formats.

use crate::TargetIndex;
use crate::card::AbilityTargetDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::CardNameDef;
use crate::card::CounterKind;
use crate::card::ObjectRefDef;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::ZonePlacement;
use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::PlayActionMatcherDef;
use crate::card::PlayRestrictionDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::SumValueDef;
use crate::card::TopOfLibraryCostDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "BNG",
    slug: "born-of-the-gods",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// BNG 27 — Spirit of the Labyrinth
pub(in crate::card::sets) static SPIRIT_OF_THE_LABYRINTH_27: CardRecord = CardRecord::new(
    "Spirit of the Labyrinth",
    "f44e5128-e146-4e46-b313-a40d82719d1d",
    "Jason Chan",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Spirit"], 3, 1)
        .with_abilities(&[AbilityDef::static_ability(
            "Each player can't draw more than one card each turn.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::EachPlayer,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotDrawMoreThanEachTurn(1)),
            },
        )])
        .with_type(crate::card::CardType::Enchantment),
);

// BNG 49 — Retraction Helix
pub(in crate::card::sets) static RETRACTION_HELIX_49: CardRecord = CardRecord::new(
    "Retraction Helix",
    "4fe8c0b9-fdf4-4fc0-aa7c-774546cdd792",
    "Phill Simmer",
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[
AbilityDef::spell_with_targets("Until end of turn, target creature gains \"{T}: Return target nonland permanent to its owner's hand.\"", &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Creature))], EffectDef::Apply { recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY), effect: AppliedEffectDef::add_ability(&AbilityDef::activated_with_targets("{T}: Return target nonland permanent to its owner's hand.", &[CostDef::TapSource], &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)))], EffectDef::move_to_zone(EffectRecipientDef::Target(TargetIndex::PRIMARY), ZoneKind::Hand, ZonePlacement::Top))), duration: ResolvedEffectDurationDef::UntilEndOfTurn })
]),
);

// BNG 61 — Bile Blight
pub(in crate::card::sets) static BILE_BLIGHT_61: CardRecord = CardRecord::new(
    "Bile Blight",
    "3ca11057-e50a-4817-924a-5bb504d0780f",
    "Vincent Proce",
    CardRules::new_instant(mana_cost!("{B}{B}")).with_abilities(&[
AbilityDef::spell_with_targets("Target creature and all other creatures with the same name as that creature get -3/-3 until end of turn.", &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Creature))], EffectDef::Apply { recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::NameEquals(CardNameDef::NameOf(ObjectRefDef::Target(TargetIndex::PRIMARY)))]), &[ZoneKind::Battlefield], PlayerRelation::Any), effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(-3), ValueDef::Constant(-3)), duration: ResolvedEffectDurationDef::UntilEndOfTurn })
]),
);

// BNG 119 — Courser of Kruphix
pub(in crate::card::sets) static COURSER_OF_KRUPHIX: CardRecord = CardRecord::new(
    "Courser of Kruphix",
    "da5a807f-58e8-4d92-a61c-47bb9b28977f",
    "Eric Deschamps",
    // Two toughness past what red reaches, which is most of why the body is
    // worth three mana at all.
    CardRules::new_enchantment_creature(mana_cost!("{1}{G}{G}"), &["Centaur"], 2, 4)
        .with_abilities(&[
            // The reveal is what makes the permission worth having: a top card you
            // cannot see is a land drop you cannot plan. It is public rather than
            // private, so the other player plans around it too.
            AbilityDef::static_ability(
                "Play with the top card of your library revealed.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::players(PlayerSetDef::Related(
                        PlayerRelation::You,
                    )),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::PlaysWithTopOfLibraryRevealed),
                },
            ),
            AbilityDef::static_ability(
                "You may play lands from the top of your library.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::players(PlayerSetDef::Related(
                        PlayerRelation::You,
                    )),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromTopOfLibrary {
                        // Lands only, and at their printed cost -- which for a land is no cost at
                        // all beyond the land drop it still has to spend.
                        restriction: PlayRestrictionDef::new(
                            PlayActionMatcherDef::PlayLand,
                            ObjectPredicateDef::HasType(CardType::Land),
                        ),
                        cost: TopOfLibraryCostDef::Printed,
                    }),
                },
            ),
            AbilityDef::triggered(
                "Landfall — Whenever a land you control enters, you gain 1 life.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

static OTHER_LEGENDS_YOU_CONTROL: ValueDef = ValueDef::Sum(&SumValueDef::new(
    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
        ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Supertype(CardSupertype::Legendary),
        ]),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    )),
    ValueDef::Constant(-1),
));

// BNG 157 — Astral Cornucopia
pub(in crate::card::sets) static ASTRAL_CORNUCOPIA_157: CardRecord = CardRecord::new(
    "Astral Cornucopia",
    "a72b8011-c712-418f-869e-42fda3dc0830",
    "Aleksi Briclot",
    CardRules::new_artifact(mana_cost!("{X}{X}{X}")).with_abilities(&[
AbilityDef::replacement("This artifact enters with X charge counters on it.", ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::AddCastXCounters { kind: CounterKind::named("charge") })),
AbilityDef::activated_mana("{T}: Choose a color. Add one mana of that color for each charge counter on this artifact.", &[CostDef::TapSource], EffectDef::AddMana(AddManaEffectDef::any_color().with_variable_amount(ValueDef::CountersOnSource(CounterKind::named("charge")))))
]),
);

// BNG 159 — Heroes' Podium
pub(in crate::card::sets) static HEROES_PODIUM: CardRecord = CardRecord::new(
    "Heroes' Podium",
    "a3cb14f9-343c-4672-b4ee-db7f1d1a98ff",
    "Willian Murai",
CardRules::new_artifact(mana_cost!("{5}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Each legendary creature you control gets +1/+1 for each other legendary creature you control.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        OTHER_LEGENDS_YOU_CONTROL,
                        OTHER_LEGENDS_YOU_CONTROL,
                    ),
                },
            ),
            AbilityDef::activated(
                "{X}, {T}: Look at the top X cards of your library. You may reveal a legendary creature card from among them and put it into your hand. Put the rest on the bottom of your library in a random order.",
                &[
                    CostDef::Mana(mana_cost!("{X}")),
                    CostDef::TapSource,
                ],
                abilities::look_at_top_cards_reveal_choice_to_hand_rest_random_bottom(
                    ValueDef::ChosenX,
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                    ]),
                    0,
                    1,
                ),
            ),
        ]),
);

// BNG 163 — Temple of Enlightenment
pub(in crate::card::sets) static TEMPLE_OF_ENLIGHTENMENT: CardRecord = CardRecord::new(
    "Temple of Enlightenment",
    "c657a645-f454-4eaf-be0d-15c9989fa4ef",
    "Svetlin Velinov",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, scry 1. (Look at the top card of your \
             library. You may put that card on the bottom.)",
            abilities::scry(ValueDef::Constant(1)),
        ),
        AbilityDef::activated_mana(
            "{T}: Add {W} or {U}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Blue,
            ])),
        ),
    ]),
);

// BNG 164 — Temple of Malice
pub(in crate::card::sets) static TEMPLE_OF_MALICE: CardRecord = CardRecord::new(
    "Temple of Malice",
    "52f50818-aede-4667-883a-e0339d86d870",
    "Sam Burley",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, scry 1. (Look at the top card of your \
             library. You may put that card on the bottom.)",
            abilities::scry(ValueDef::Constant(1)),
        ),
        AbilityDef::activated_mana(
            "{T}: Add {B} or {R}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Red,
            ])),
        ),
    ]),
);

// BNG 165 — Temple of Plenty
pub(in crate::card::sets) static TEMPLE_OF_PLENTY: CardRecord = CardRecord::new(
    "Temple of Plenty",
    "b0830054-b140-49c3-90cb-24e2502757be",
    "Noah Bradley",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, scry 1. (Look at the top card of your \
             library. You may put that card on the bottom.)",
            abilities::scry(ValueDef::Constant(1)),
        ),
        AbilityDef::activated_mana(
            "{T}: Add {G} or {W}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Green,
                ManaColor::White,
            ])),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &SPIRIT_OF_THE_LABYRINTH_27,
    &RETRACTION_HELIX_49,
    &BILE_BLIGHT_61,
    &COURSER_OF_KRUPHIX,
    &ASTRAL_CORNUCOPIA_157,
    &HEROES_PODIUM,
    &TEMPLE_OF_ENLIGHTENMENT,
    &TEMPLE_OF_MALICE,
    &TEMPLE_OF_PLENTY,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
