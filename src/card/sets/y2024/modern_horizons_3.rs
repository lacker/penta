//! Modern Horizons 3 cards cataloged as attachment edge cases.

use super::super::y2016::eldritch_moon::escalate;
use super::super::y2020::theros_beyond_death::escape;
use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityPredicateDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AddManaEffectDef;
use crate::card::AggregateOperationDef;
use crate::card::AlternativeCastKindDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::AttackEventMatcherDef;
use crate::card::BasicLandType;
use crate::card::BattlefieldArrivalDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::BindObjectsDef;
use crate::card::CardArt;
use crate::card::CardChoiceSourceDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::CharacteristicOperationDef;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::ChooseForEachPlayerDef;
use crate::card::ClassifyObjectsDef;
use crate::card::ColorSet;
use crate::card::ComparisonDef;
use crate::card::ControlDurationDef;
use crate::card::CopyExceptionsDef;
use crate::card::CopyStackObjectDef;
use crate::card::CostDef;
use crate::card::CostQuantityDef;
use crate::card::CountConditionDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::CreatureTypeSetDef;
use crate::card::DrawEventMatcherDef;
use crate::card::EffectChoiceDef;
use crate::card::EffectDef;
use crate::card::EffectPaymentDef;
use crate::card::EffectRecipientDef;
use crate::card::EmblemCharacteristics;
use crate::card::ExiledCastPermissionDef;
use crate::card::HalvedValueDef;
use crate::card::InstalledTriggerDef;
use crate::card::InstalledTriggerLifetimeDef;
use crate::card::KeywordAbility;
use crate::card::ManaColor;
use crate::card::ManaSpendEffectDef;
use crate::card::MoveObjectsDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectSetFilterDef;
use crate::card::ObjectSetValueAtLeastDef;
use crate::card::ObjectSetValueDef;
use crate::card::ObjectValueDef;
use crate::card::PayOrDef;
use crate::card::PerPlayerSelectionDef;
use crate::card::PileExileDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerRuleDef;
use crate::card::PlayerSetDef;
use crate::card::PutObjectsOntoBattlefieldFaceDownDef;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::RevealObjectsDef;
use crate::card::RoundingDef;
use crate::card::SacrificedAmountDef;
use crate::card::ScaledValueDef;
use crate::card::SetOperationDef;
use crate::card::SpellCastQueryDef;
use crate::card::SubtypeDef;
use crate::card::SumValueDef;
use crate::card::TargetConditionDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenCountersDef;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePickDef;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::card::tokens;
use crate::ids::Binding;
use crate::ids::ParentBinding;
use crate::mana_cost;

const DEVOURER_TOP: Binding = Binding!("devourer_top");
const DEVOURER_EXILED: Binding = Binding!("devourer_exiled");
static DEVOURER_EXILE_REST: EffectDef = EffectDef::MoveObjects(MoveObjectsDef {
    input: ObjectSetDef::Binding(DEVOURER_EXILED),
    from: Some(ZoneKind::Library),
    zone: ZoneKind::Exile,
    placement: ZonePlacement::Top,
    moved: None,
    then: &EffectDef::None,
});
static DEVOURER_PUT_TOP: EffectDef = EffectDef::Sequence(&[
    EffectDef::MoveObjects(MoveObjectsDef {
        input: ObjectSetDef::Binding(DEVOURER_TOP),
        from: Some(ZoneKind::Library),
        zone: ZoneKind::Library,
        placement: ZonePlacement::Top,
        moved: None,
        then: &EffectDef::None,
    }),
    DEVOURER_EXILE_REST,
]);
static DEVOURER_CHOOSE: EffectDef = EffectDef::Choose(ChooseDef {
    binding: ObjectChoiceBindingDef::Objects(DEVOURER_TOP),
    unchosen: Some(DEVOURER_EXILED),
    chooser: PlayerRefDef::EffectController,
    candidates: ObjectSetDef::Binding(ParentBinding),
    exclude: None,
    minimum: 0,
    maximum: 1,
    visibility: ChoiceVisibilityDef::Private,
    then: &DEVOURER_PUT_TOP,
});
static DEVOURER_OPENING_LOOK: EffectDef = abilities::bind_top_cards_then(
    PlayerRefDef::EffectController,
    ValueDef::Constant(4),
    &DEVOURER_CHOOSE,
);

static DEVOURER_OPENING_TRIGGER: AbilityDef = AbilityDef::triggered(
    "At the beginning of your first upkeep, look at the top four cards of your library. You may put one of those cards back on top of your library. Exile the rest.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    DEVOURER_OPENING_LOOK,
);

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "MH3",
    slug: "modern-horizons-3",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const CLUE_TOKEN: TokenCharacteristics = tokens::clue().with_art(CardArt::new(
    "e604b9ca-6c5a-459e-b509-955c3428530a",
    "Michele Giorgi",
));

const FOOD_TOKEN: TokenCharacteristics = tokens::food().with_art(CardArt::new(
    "14fe0b7c-2d73-4c21-98ca-ee3a7d7f20c8",
    "Leanna Crossan",
));

const ELDRAZI_SPAWN_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Eldrazi", "Spawn"], &[], 0, 1)
        .with_abilities(&[AbilityDef::activated_mana(
            "Sacrifice this token: Add {C}.",
            &[CostDef::SacrificeSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        )])
        .with_art(CardArt::new(
            "0645cf05-4660-4a6e-b789-7e0f6b7660c0",
            "Aleksi Briclot",
        ));

// MH3 2 — Devourer of Destiny
pub(in crate::card::sets) static DEVOURER_OF_DESTINY: CardRecord = CardRecord::new(
    "Devourer of Destiny",
    "560debcd-feb4-4534-991e-a7aa1cca2409",
    "Raph Lomotan",
CardRules::new_creature(mana_cost!("{5}{C}{C}"), &["Eldrazi"], 6, 6).with_abilities(&[
        AbilityDef::opening_hand_reveal(
            "You may reveal this card from your opening hand. If you do, at the beginning of your first upkeep, look at the top four cards of your library. You may put one of those cards back on top of your library. Exile the rest.",
            EffectDef::InstallTrigger(InstalledTriggerDef::once(&DEVOURER_OPENING_TRIGGER)),
        ),
        AbilityDef::triggered_with_targets(
            "When you cast this spell, exile target permanent that's one or more colors.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Not(
                &ObjectPredicateDef::ColorCount(0),
            ))],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Exile,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

static LANDSCAPE_FETCH_COST: [CostDef; 2] = [CostDef::TapSource, CostDef::SacrificeSource];

/// The Landscape cycle: a land that taps for nothing useful, sacrifices
/// itself for one of three tapped basics, and is a cycling card when the
/// board does not need a land at all. Each member differs only in which
/// three basics it names and what its cycling costs.
const fn landscape_abilities(
    fetch_text: &'static str,
    basics: ObjectPredicateDef,
    cycling: &AbilityDef,
) -> [AbilityDef; 3] {
    [
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            fetch_text,
            &LANDSCAPE_FETCH_COST,
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: basics,
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: false,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: true,
                attachment: None,
                binding: None,
                then: None,
            },
        ),
        *cycling,
    ]
}

// MH3 7 — Glaring Fleshraker
pub(in crate::card::sets) static GLARING_FLESHRAKER_7: CardRecord = CardRecord::new(
    "Glaring Fleshraker",
    "80c2a3c7-1486-4ff9-88ec-79ec67a437f8",
    "Raph Lomotan",
    CardRules::new_creature(mana_cost!("{2}{C}"), &["Eldrazi", "Drone"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you cast a colorless spell, create a 0/1 colorless Eldrazi Spawn creature token with \"Sacrifice this token: Add {C}.\"",
            TriggerEventDef::spell_cast(ObjectPredicateDef::ColorCount(0)),
            EffectDef::CreateToken(crate::card::CreateTokenDef::new(crate::card::TokenDef::Literal(crate::card::TokenCharacteristics::creature(&["Eldrazi", "Spawn"], &[], 0, 1).with_abilities(&[AbilityDef::activated_mana(
                "Sacrifice this creature: Add {C}.", &[CostDef::SacrificeSource], EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
            )])))),
        ),
        AbilityDef::triggered(
            "Whenever another colorless creature you control enters, this creature deals 1 damage to each opponent.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::ColorCount(0), ObjectPredicateDef::ControlledBy(PlayerRelation::You), ObjectPredicateDef::Not(&ObjectPredicateDef::Source)]), None, Some(ZoneKind::Battlefield)),
            EffectDef::damage(EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::Opponent)), ValueDef::Constant(1)),
        ),
    ]),
);

// MH3 10 — Kozilek, the Broken Reality
pub(in crate::card::sets) static KOZILEK_THE_BROKEN_REALITY_10: CardRecord = CardRecord::new(
    "Kozilek, the Broken Reality",
    "04066abb-44d2-4730-9cc3-2584bc4c7d8c",
    "Brent Hollowell",CardRules::new_creature(mana_cost!("{9}"), &["Eldrazi"], 9, 9).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::triggered_with_targets("When you cast this spell, up to two target players each manifest two cards from their hands. For each card manifested this way, you draw a card. (To manifest a card, put it onto the battlefield face down as a 2/2 creature. Turn it face up any time for its mana cost if it's a creature card.)", TriggerEventDef::spell_cast(ObjectPredicateDef::Source), &[AbilityTargetDef::up_to(AbilityTargetPredicate::Player(PlayerRelation::Any), 2)], EffectDef::ChooseForEachPlayer(ChooseForEachPlayerDef { player: EffectRecipientDef::Target(TargetIndex::PRIMARY), candidates: ObjectPredicateDef::Any, zone: ZoneKind::Hand, selection: PerPlayerSelectionDef::Count(ValueDef::Constant(2)), visibility: ChoiceVisibilityDef::Private, chosen: Binding!("kozilek_hands"), unchosen: Binding!("kozilek_remaining"), then: &EffectDef::Sequence(&[EffectDef::ForEachInBinding { objects: Binding!("kozilek_hands"), binding: Binding!("manifest_card"), effect: &EffectDef::PutObjectsOntoBattlefieldFaceDown(PutObjectsOntoBattlefieldFaceDownDef { input: ObjectSetDef::One(ObjectRefDef::Binding(Binding!("manifest_card"))), controller: PlayerRefDef::OwnerOf(ObjectRefDef::Binding(Binding!("manifest_card"))), characteristics: crate::card::face_down::manifest(), turn_up_for_mana_cost: true, moved: None, then: &EffectDef::None }) }, EffectDef::DrawCards { recipient: EffectRecipientDef::Controller, amount: ValueDef::Sum(&SumValueDef { left: ValueDef::CountObjects(&ObjectSetDef::ZoneChangeSuccessorsOfBinding(Binding!("kozilek_hands"))), right: ValueDef::Sum(&SumValueDef { left: ValueDef::CountObjects(&ObjectSetDef::Query(ObjectQueryDef::matching(ObjectPredicateDef::Any, &[ZoneKind::Battlefield], PlayerRelation::Any))), right: ValueDef::Negate(&ValueDef::CountObjects(&ObjectSetDef::Union(&[ObjectSetDef::ZoneChangeSuccessorsOfBinding(Binding!("kozilek_hands")), ObjectSetDef::Query(ObjectQueryDef::matching(ObjectPredicateDef::Any, &[ZoneKind::Battlefield], PlayerRelation::Any))]))) }) }) }]) })),
AbilityDef::static_ability("Other colorless creatures you control get +3/+2.", EffectDef::StaticApply { recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::ColorCount(0), ObjectPredicateDef::Not(&ObjectPredicateDef::Source)]), &[ZoneKind::Battlefield], PlayerRelation::You), effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(3), ValueDef::Constant(2)) })
]),
);

// MH3 11 — Kozilek's Command
// Audit: unsupported — AbilityTargetDef has a fixed maximum and an exact dynamic count, but no dynamic up-to maximum. Its fourth mode needs any number from zero through chosen X, not exactly X targets.
pub(in crate::card::sets) static KOZILEK_S_COMMAND_11: CardRecord = CardRecord::new(
    "Kozilek's Command",
    "92585587-cfdc-406a-9114-4f6dd8802c37",
    "Yeong-Hao Han",
    crate::card::CardRules::unsupported(),
);

// MH3 18 — Aerie Auxiliary
pub(in crate::card::sets) static AERIE_AUXILIARY: CardRecord = CardRecord::new(
    "Aerie Auxiliary",
    "5e4c134b-a416-467e-a158-def84c92c6af",
    "Donato Giancola",
    // Four mana for five power across the board, in the air, which is the
    // rate a limited deck is happy with.
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Bird", "Soldier"], 3, 3).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, support 2. (Put a +1/+1 counter on each of up to two \
             other target creatures.)",
            // One slot holding up to two targets rather than two slots: the
            // "each of" is what makes them one group, and "other" is what
            // keeps this creature out of its own support.
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                2,
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// MH3 22 — Dog Umbra
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOG_UMBRA: CardRecord = CardRecord::new(
    "Dog Umbra",
    "8d4ba710-eddb-40ca-b2fe-0e4e778aab9c",
    "Brian Valeza",
    crate::card::CardRules::unsupported(),
);

// MH3 26 — Flare of Fortitude
pub(in crate::card::sets) static FLARE_OF_FORTITUDE_26: CardRecord = CardRecord::new(
    "Flare of Fortitude",
    "37b41b59-0296-443b-8a62-8d5c4641ef66",
    "Winona Nelson",
    CardRules::new_instant(mana_cost!("{2}{W}{W}")).with_abilities(&[
AbilityDef::alternative_cast(&[CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Not(&ObjectPredicateDef::Token), ObjectPredicateDef::Color(ManaColor::White)]))], AlternativeCastKindDef::AlternativeCost, Some("You may sacrifice a nontoken white creature rather than pay this spell's mana cost."), EffectDef::None),
AbilityDef::spell("Until end of turn, your life total can't change, and permanents you control gain hexproof and indestructible.", EffectDef::Sequence(&[EffectDef::Apply { recipient: EffectRecipientDef::Controller, effect: AppliedEffectDef::Rule(AppliedRuleDef::PlayerRule(PlayerRuleDef::LifeTotalCannotChange)), duration: ResolvedEffectDurationDef::UntilEndOfTurn }, EffectDef::Apply { recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::Any, &[ZoneKind::Battlefield], PlayerRelation::You), effect: AppliedEffectDef::Composite(&[AppliedEffectDef::add_ability(&abilities::hexproof()), AppliedEffectDef::add_ability(&abilities::indestructible())]), duration: ResolvedEffectDurationDef::UntilEndOfTurn }]))
]),
);

// MH3 34 — Mandibular Kite
pub(in crate::card::sets) static MANDIBULAR_KITE: CardRecord = CardRecord::new(
    "Mandibular Kite",
    "6b922f71-18e6-4a74-b792-d477d4a1deca",
    "Bruno Biazotto",
    // One mana for a 1/1 flier that is also an Equipment. The equip cost is
    // deliberately steep: moving the wings onto something that matters is the
    // expensive half, not getting them onto the board.
    CardRules::new_artifact(mana_cost!("{W}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::living_weapon(),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1 and has flying.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                    ]),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{3}{W}"))], "Equip {3}{W}"),
        ]),
);

// MH3 38 — Ocelot Pride
pub(in crate::card::sets) static OCELOT_PRIDE: CardRecord = CardRecord::new(
    "Ocelot Pride",
    "89cf6f57-230f-497e-a14e-ad1e8737fd42",
    "Chris Seaman",
// Its own lifelink turns the trigger on, and once the board is wide
    // enough to ascend every Cat it ever made comes back doubled.
    CardRules::new_creature(mana_cost!("{W}"), &["Cat"], 1, 1)
        .with_abilities(&[
            abilities::first_strike(),
            abilities::lifelink(),
            AbilityDef::static_ability(
                "Ascend (If you control ten or more permanents, you get the city's blessing for the rest \
                 of the game.)",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::You)),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::Ascend),
                },
            ),
            AbilityDef::triggered_if(
                "At the beginning of your end step, if you gained life this turn, create a 1/1 white Cat \
                 creature token. Then if you have the city's blessing, for each token you control that \
                 entered this turn, create a token that's a copy of it.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::You,
                },
                &TriggerConditionDef::ControllerGainedLifeThisTurn,
                EffectDef::Sequence(&[
                    EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                        TokenCharacteristics::creature(&["Cat"], &[ManaColor::White], 1, 1).with_art(CardArt::new(
                            "74bacab2-a4c6-4ba5-a208-6bd09ae4cf9f",
                            "Maxime Minard",
                        )),
                    ))),
                    // The blessing half is checked as this resolves rather than as it
                    // triggers, so ascending in response still doubles.
                    EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ControllerHasCitysBlessing,
                        then: &EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Copy(
                            &crate::card::TokenCopyDef {
                                // "Until this enchantment leaves the battlefield" is one printed ability,
                                // so the return rides on the same resolution as a delayed trigger rather
                                // than appearing as a second clause the card does not print.
                                // "For each token you control that entered this turn." The Cat the clause
                                // just made is one of them, which is what makes the doubling compound.
                                object: &EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::controlled_by(
                                    ObjectPredicateDef::All(&[
                                        ObjectPredicateDef::Token,
                                        ObjectPredicateDef::CameUnderControlThisTurn,
                                    ]),
                                    &[ZoneKind::Battlefield],
                                    PlayerSetDef::Related(PlayerRelation::You),
                                ))),
                                exceptions: CopyExceptionsDef::NONE,
                            },
                        ))),
                    },
                ]),
            ),
        ]),
);

// MH3 40 — Phelia, Exuberant Shepherd
pub(in crate::card::sets) static PHELIA_EXUBERANT_SHEPHERD: CardRecord = CardRecord::new(
    "Phelia, Exuberant Shepherd",
    "55707746-da6e-46e5-a5ca-7ac843fdc38e",
    "Rudy Siswanto",
// Two mana that answers something for a turn or blinks something of
    // yours forever, and grows every time it does the second.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Dog"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flash(),
            AbilityDef::triggered_with_targets(
                "Whenever this creature attacks, exile up to one other target nonland permanent. \
                 At the beginning of the next end step, return that card to the battlefield under \
                 its owner's control. If it entered under your control, put a +1/+1 counter on \
                 this creature.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                // "Up to one other target nonland permanent", which is what makes her a
                // blink as happily as a removal spell: the thing she takes may be yours.
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    1,
                )],
                EffectDef::Sequence(&[
                    EffectDef::ExileLinkedToSource {
                        until_source_leaves: false,
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        face_down: false,
                        then: None,
                    },
                    EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                        "At the beginning of the next end step, return that card to the battlefield under its \
                         owner's control. If it entered under your control, put a +1/+1 counter on this creature.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::End,
                            player: PlayerRelation::Any,
                        },
                        EffectDef::Sequence(&[
                            EffectDef::IfCondition {
                                // "If it entered under your control": what Phelia gives back goes to its
                                // owner, so who owned it is the whole of the question. Asked before the
                                // return rather than after, because by then there is no exile left to ask
                                // about.
                                condition: &TriggerConditionDef::ObjectSetCount(
                                    &crate::card::ObjectSetCountConditionDef {
                                        objects: &ObjectSetDef::LinkedExiles,
                                        predicate: crate::card::ObjectSetPredicateDef {
                                            filter: Some(ObjectSetFilterDef::Predicate(
                                                &ObjectPredicateDef::OwnedBy(PlayerRelation::You),
                                            )),
                                            comparison: ComparisonDef::GreaterOrEqual,
                                            amount: 1,
                                        },
                                    },
                                ),
                                then: &EffectDef::AddCounters {
                                    object: EffectRecipientDef::Source,
                                    kind: CounterKind::PlusOnePlusOne,
                                    amount: ValueDef::Constant(1),
                                },
                            },
                            EffectDef::ReturnLinkedExiles {
                                object: ObjectPredicateDef::Any,
                                zone: ZoneKind::Battlefield,
                                grant: None,
                                counters: None,
                                transformed: false,
                                controller: None,
                            },
                        ]),
                    ))),
                ]),
            ),
        ]),
);

// MH3 44 — Static Prison
pub(in crate::card::sets) static STATIC_PRISON: CardRecord = CardRecord::new(
    "Static Prison",
    "dd16222e-349c-4a2b-a7c8-8eb35a8ab332",
    "Jason A. Engle",
// One white answers anything, and the two energy it comes with buy two
    // more turns of holding it. After that the prison opens.
    CardRules::new_enchantment(mana_cost!("{W}")).with_abilities(&[
        abilities::enters_trigger_with_targets("When this enchantment enters, exile target nonland permanent an opponent controls until this enchantment leaves the battlefield. You get {E}{E} (two energy counters).", &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::Opponent),
                owner: None,
            },
        )], EffectDef::Sequence(&[
            // "Until this enchantment leaves the battlefield": a Prison answered
            // before its own trigger resolves exiles nobody (CR 610.3b).
            EffectDef::ExileLinkedToSource {
                until_source_leaves: true,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                face_down: false,
                then: None,
            },
            EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                "When this enchantment leaves the battlefield, return the exiled card to the battlefield under its owner's control.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    None,
                ),
                EffectDef::ReturnLinkedExiles {
                    object: ObjectPredicateDef::Any,
                    counters: None,
                    zone: ZoneKind::Battlefield,
                    grant: None,
                    controller: None,
                    transformed: false,
                },
            ))),
            // The energy arrives with the exile rather than paying for it: the first
            // upkeep tax is already covered, and the second is not.
            EffectDef::AddPlayerCounters {
                recipient: EffectRecipientDef::Controller,
                kind: CounterKind::named("energy"),
                amount: ValueDef::Constant(2),
            },
        ])),
        AbilityDef::triggered(
            "At the beginning of your first main phase, sacrifice this enchantment unless you pay {E}.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::PrecombatMain,
                player: PlayerRelation::You,
            },
            EffectDef::PayOr(PayOrDef::unless(
                &[CostDef::Energy(1)],
                &EffectDef::sacrifice(EffectRecipientDef::Source),
            )),
        ),
    ]),
);

// MH3 45 — Thraben Charm
pub(in crate::card::sets) static THRABEN_CHARM: CardRecord = CardRecord::new(
    "Thraben Charm",
    "dd28a646-f38f-4cdf-948c-969cd979e5e6",
    "Carlos Palma Cruchaga",
    // Removal that scales with the board, an answer to an enchantment, and
    // graveyard hate: two mana that is never quite dead.
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Thraben Charm deals damage equal to twice the number of creatures you control \
                 to target creature.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    // Counted as this resolves, and the target itself counts
                    // when it is one of yours.
                    ValueDef::Scaled(&ScaledValueDef {
                        value: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        factor: 2,
                    }),
                ),
            ),
            AbilityDef::spell_with_targets(
                "Destroy target enchantment.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            ),
            AbilityDef::spell_with_targets(
                "Exile any number of target players' graveyards.",
                // "Any number" includes none, so this mode is castable with
                // an empty board and no graveyards worth touching.
                &[AbilityTargetDef::any_number(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::move_to_zone(
                    // Every player the slot chose, not just one: the singular
                    // cards_owned_by_target helper names a single target and
                    // this slot holds any number of them.
                    EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::owned_by(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Graveyard],
                        PlayerSetDef::LegalTargets(TargetIndex::PRIMARY),
                    ))),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
            ),
        ],
    )),
);

// MH3 51 — Amphibian Downpour
pub(in crate::card::sets) static AMPHIBIAN_DOWNPOUR_51: CardRecord = CardRecord::new(
    "Amphibian Downpour",
    "2d8aeca5-622a-45be-8168-07e7c00e3092",
    "Omar Rayyan",
    CardRules::new_enchantment(mana_cost!("{2}{U}")).with_subtypes(&["Aura"]).with_abilities(&[
abilities::flash(),
abilities::storm(),
abilities::enchant_creature(),
AbilityDef::static_ability("Enchanted creature loses all abilities and is a blue Frog creature with base power and toughness 1/1.", EffectDef::StaticApply { recipient: EffectRecipientDef::AttachedPermanent, effect: AppliedEffectDef::Composite(&[AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any), AppliedEffectDef::set_card_types(CardTypeSet::single(CardType::Creature)), AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Frog"])), AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Blue])), AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1))]) })
]),
);

// MH3 53 — Brainsurge
pub(in crate::card::sets) static BRAINSURGE: CardRecord = CardRecord::new(
    "Brainsurge",
    "ed48f805-b57c-4d7f-a3c2-d16ae71bce2d",
    "Liiga Smilshkalne",
    // Two more cards than Brainstorm for two more mana, and the same catch:
    // what it really does is fix a hand, and without a shuffle the two that
    // go back are two draws you have already spent.
    CardRules::new_instant(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell(
        "Draw four cards, then put two cards from your hand on top of your library in any order.",
        // Brainstorm's two steps for one more card. The arrangement is the order
        // the two are named in: each is placed on top of the last, so the card
        // named second is the one drawn first.
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(4),
            },
            EffectDef::ChooseCards {
                player: EffectRecipientDef::Controller,
                sources: &[CardChoiceSourceDef::Zone(ZoneKind::Hand)],
                object: ObjectPredicateDef::Any,
                minimum: 2,
                maximum: 2,
                reveal: false,
                destination: ZoneKind::Library,
                placement: ZonePlacement::Top,
            },
        ]),
    )),
);

// MH3 54 — Consign to Memory
pub(in crate::card::sets) static CONSIGN_TO_MEMORY_54: CardRecord = CardRecord::new(
    "Consign to Memory",
    "bc95af55-d1dd-4fe6-adb0-3ad6db20d986",
    "Ben Hill",
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[
        abilities::replicate(&[CostDef::Mana(mana_cost!("{1}"))]),
        AbilityDef::spell_with_targets(
            "Counter target triggered ability or colorless spell.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::TriggeredAbility,
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Spell,
                            ObjectPredicateDef::ColorCount(0),
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
        ),
    ]),
);

// MH3 63 — Harbinger of the Seas
pub(in crate::card::sets) static HARBINGER_OF_THE_SEAS_63: CardRecord = CardRecord::new(
    "Harbinger of the Seas",
    "00212714-a410-4cbc-bf1c-f90d7d77378c",
    "Winona Nelson",
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Merfolk", "Wizard"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "Nonbasic lands are Islands.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(
                            CardSupertype::Basic,
                        )),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::set_basic_land_types(&[BasicLandType::Island]),
            },
        ),
    ),
);

// MH3 69 — Serum Visionary
pub(in crate::card::sets) static SERUM_VISIONARY: CardRecord = CardRecord::new(
    "Serum Visionary",
    "08a587f5-5910-405e-8982-c889dbbc7f98",
    "Warren Mahy",
    // Serum Visions on a body: the same draw-then-scry, so the smoothing
    // shapes the two draws after this one rather than this one.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Vedalken", "Wizard"], 2, 2).with_ability(
        abilities::enters_trigger(
            "When this creature enters, draw a card, then scry 2.",
            EffectDef::Sequence(&[
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
                abilities::scry(ValueDef::Constant(2)),
            ]),
        ),
    ),
);

// MH3 71 — Strix Serenade
pub(in crate::card::sets) static STRIX_SERENADE_71: CardRecord = CardRecord::new(
    "Strix Serenade",
    "42ac5ac7-b2f9-4e6f-af41-7e42ac816374",
    "Filipe Pagliuso",
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target artifact, creature, or planeswalker spell. Its controller creates a 2/2 blue Bird creature token with flying.",
        &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::All(&[ObjectPredicateDef::Spell, ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::HasType(CardType::Planeswalker)])]), zones: &[ZoneKind::Stack], controller: None, owner: None })],
        EffectDef::Sequence(&[EffectDef::counter_target(TargetIndex::PRIMARY), EffectDef::CreateToken(crate::card::CreateTokenDef::new(crate::card::TokenDef::Literal(crate::card::TokenCharacteristics::creature(&["Bird"], &[ManaColor::Blue], 2, 2).with_abilities(&[abilities::flying()]))).with_controller(PlayerRefDef::ControllerOf(ObjectRefDef::Target(TargetIndex::PRIMARY))))]),
    )),
);

// MH3 79 — Volatile Stormdrake
// Audit: unsupported — Exchange control is available, but protection and hexproof do not have an activated-and-triggered-abilities-only targeting restriction. Ordinary hexproof would incorrectly prohibit opponents' spells too.
pub(in crate::card::sets) static VOLATILE_STORMDRAKE_79: CardRecord = CardRecord::new(
    "Volatile Stormdrake",
    "2e6e3232-8bb8-4504-9597-dfdfc6d634bd",
    "Campbell White",
    crate::card::CardRules::unsupported(),
);

// MH3 80 — Accursed Marauder
pub(in crate::card::sets) static ACCURSED_MARAUDER: CardRecord = CardRecord::new(
    "Accursed Marauder",
    "5da14d86-0780-4821-a799-96f64b377df4",
    "Paolo Parente",
// Symmetrical on paper, one-sided in practice: the Marauder itself is a
    // legal answer for its own controller, so a token board pays nothing.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Zombie", "Warrior"], 2, 1).with_ability(
        abilities::enters_trigger(
            "When this creature enters, each player sacrifices a nontoken creature of their choice.",
            EffectDef::SacrificeOfChoice {
                player: EffectRecipientDef::EachPlayer,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                ]),
                count: ValueDef::Constant(1),
                then: None,
                amount: SacrificedAmountDef::Power,
                otherwise: None,
                // "Sacrifices" rather than "may sacrifice": a player holding
                // one gives it up.
                optional: false,
            },
        ),
    ),
);

// MH3 90 — Emperor of Bones
pub(in crate::card::sets) static EMPEROR_OF_BONES: CardRecord = CardRecord::new(
    "Emperor of Bones",
    "df9d9075-2d1e-4848-b661-816d539e05eb",
    "Josh Hass",
// Two mana that eats a graveyard one card a turn and then rents the best
    // of them back for an attack, which is what makes the adapt cost worth
    // paying twice.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Skeleton", "Noble"], 2, 2)
        .with_abilities(&[
            AbilityDef::triggered_with_targets(
                "At the beginning of combat on your turn, exile up to one target card from a graveyard.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::BeginningOfCombat,
                    player: PlayerRelation::You,
                },
                // Anybody's graveyard, and "up to one": an Emperor with nothing worth
                // taking still gets its combat trigger, and simply exiles nothing.
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Any,
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: None,
                    },
                    1,
                )],
                EffectDef::ExileLinkedToSource {
                    until_source_leaves: false,
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    face_down: false,
                    then: None,
                },
            ),
            AbilityDef::activated(
                "{1}{B}: Adapt 2. (If this creature has no +1/+1 counters on it, put two +1/+1 counters \
                 on it.)",
                &[CostDef::Mana(mana_cost!("{1}{B}"))],
                // Adapt is a conditional rather than a cost: the ability always resolves,
                // and finding a counter already there is what makes it do nothing.
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceCounters {
                        kind: CounterKind::PlusOnePlusOne,
                        comparison: ComparisonDef::LessOrEqual,
                        amount: 0,
                    },
                    then: &EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(2),
                    },
                },
            ),
            AbilityDef::triggered(
                "Whenever one or more +1/+1 counters are put on this creature, put a creature card exiled \
                 with this creature onto the battlefield under your control with a finality counter on \
                 it. It gains haste. Sacrifice it at the beginning of the next end step.",
                TriggerEventDef::CountersPlaced {
                    object: ObjectPredicateDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                },
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Object(ParentBinding),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    // "A creature card exiled with this creature": a pile no query can find,
                    // because what puts a card in it is which permanent exiled it.
                    candidates: ObjectSetDef::Matching {
                        objects: &ObjectSetDef::LinkedExiles,
                        object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::HasType(
                            CardType::Creature,
                        )),
                    },
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::PutOntoBattlefieldThen {
                        object: EffectRecipientDef::object(ObjectRefDef::Binding(ParentBinding)),
                        binding: ParentBinding,
                        counters: Some(TokenCountersDef {
                            kind: CounterKind::Finality,
                            amount: ValueDef::Constant(1),
                        }),
                        then: &EffectDef::Sequence(&[
                            EffectDef::Apply {
                                recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    ParentBinding,
                                )),
                                effect: AppliedEffectDef::add_ability(&abilities::haste()),
                                duration: ResolvedEffectDurationDef::Permanent,
                            },
                            EffectDef::InstallTrigger(InstalledTriggerDef::once(
                                &AbilityDef::triggered(
                                    "At the beginning of the next end step, sacrifice that creature.",
                                    TriggerEventDef::StepBegins {
                                        step: TurnStepDef::End,
                                        player: PlayerRelation::Any,
                                    },
                                    EffectDef::sacrifice(EffectRecipientDef::objects(
                                            ObjectSetDef::Binding(ParentBinding),
                                        )),
                                ),
                            )),
                        ]),
                    },
                }),
            ),
        ]),
);

// MH3 103 — Nethergoyf
pub(in crate::card::sets) static NETHERGOYF: CardRecord = CardRecord::new(
    "Nethergoyf",
    "3ee3945e-5089-4751-b7b3-5961c39d2a33",
    "Xavier Ribeiro",
// One mana for whatever the graveyard has made of it, and the graveyard
    // pays a second time to buy it back.
    CardRules::new_creature(mana_cost!("{B}"), &["Lhurgoyf"], 0, 1)
        .with_abilities(&[
            AbilityDef::static_ability(
                "This creature\'s power is equal to the number of card types among cards in your \
                 graveyard and its toughness is equal to that number plus 1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    // Each half is its own amount: the toughness is the count plus
                    // one rather than the count applied to a printed body -- the way
                    // Barrowgoyf reads it.
                    effect: AppliedEffectDef::define_power_toughness(
                        ValueDef::CardTypesAmongGraveyards(PlayerRelation::You),
                        // "That number plus 1", counted over your own graveyard alone.
                        ValueDef::Sum(&SumValueDef::new(
                            ValueDef::CardTypesAmongGraveyards(PlayerRelation::You),
                            ValueDef::Constant(1),
                        )),
                    ),
                },
            ),
            AbilityDef::alternative_cast(
                &[CostDef::Mana(mana_cost!("{2}{B}")), // The escape cost counts card types rather than cards: one Artifact
                // Creature Land pays three quarters of it by itself, which is why the deck
                // playing this is the one with a graveyard full of odd things.
                CostDef::exile(
                    ObjectPredicateDef::Any,
                    ZoneKind::Graveyard,
                    CostQuantityDef::ObjectSetValueAtLeast(&ObjectSetValueAtLeastDef {
                        value: ObjectSetValueDef::CardTypeCount,
                        minimum: 4,
                    }),
                )],
                AlternativeCastKindDef::Escape,
                Some(
                    "Escape—{2}{B}, Exile any number of other cards from your graveyard with four or \
                     more card types among them. (You may cast this card from your graveyard for its \
                     escape cost.)",
                ),
                EffectDef::None,
            ),
        ]),
);

// MH3 106 — Retrofitted Transmogrant
pub(in crate::card::sets) static RETROFITTED_TRANSMOGRANT: CardRecord = CardRecord::new(
    "Retrofitted Transmogrant",
    "12c1b83d-710b-4680-855a-02ba1f72abf0",
    "Kekai Kotaki",
    // A one-drop that comes back as a 3/3, which is what makes trading it
    // away early a fine outcome rather than a loss.
    CardRules::new_artifact_creature(mana_cost!("{B}"), &["Zombie"], 1, 1).with_ability(
        AbilityDef::activated(
            "{3}{B}: Return this card from your graveyard to the battlefield tapped with two \
             +1/+1 counters on it.",
            &[CostDef::Mana(mana_cost!("{3}{B}"))],
            EffectDef::WithBattlefieldArrival {
                effect: &EffectDef::move_to_zone(
                    EffectRecipientDef::Source,
                    ZoneKind::Battlefield,
                    ZonePlacement::Top,
                ),
                arrival: BattlefieldArrivalDef {
                    modifications: &[BattlefieldEntryModificationDef::Tapped],
                    counters: Some(TokenCountersDef {
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(2),
                    }),
                    ..BattlefieldArrivalDef::DEFAULT
                },
            },
        )
        // Activated from the graveyard, which is the only place this can be
        // returned from.
        .with_source_zones(&[ZoneKind::Graveyard]),
    ),
);

// MH3 108 — Scurrilous Sentry
pub(in crate::card::sets) static SCURRILOUS_SENTRY: CardRecord = CardRecord::new(
    "Scurrilous Sentry",
    "29e2805f-59fa-4a6d-97bc-266191b2aa8d",
    "Leonardo Santanna",
    // Menace is what makes the attack half reliable, so the two clauses
    // feed each other: it connives on the way in and again every swing.
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Human", "Knight", "Rogue"], 3, 3)
        .with_abilities(&[
            abilities::menace(),
            AbilityDef::triggered(
                "Whenever this creature enters or attacks, it connives. (Draw a card, then \
                 discard a card. If you discarded a nonland card, put a +1/+1 counter on this \
                 creature.)",
                // Entering and attacking are two ways for one printed ability
                // to fire, so what it does is written once.
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::Source,
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                    TriggerEventDef::attacks(ObjectPredicateDef::Source),
                ]),
                abilities::connive(),
            ),
        ]),
);

// MH3 111 — Wither and Bloom
pub(in crate::card::sets) static WITHER_AND_BLOOM: CardRecord = CardRecord::new(
    "Wither and Bloom",
    "95c2390f-71f1-4e42-83da-d603ca86a8d0",
    "Richard Kane Ferguson",
// Removal now and a counter later out of the same card, which is why it
    // is worth casting the front half even when it trades down.
    CardRules::new_instant(mana_cost!("{1}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature gets -3/-3 until end of turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-3),
                    ValueDef::Constant(-3),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated_with_targets(
            "{1}{B}, Exile this card from your graveyard: Put a +1/+1 counter on target creature you control. Activate only as a sorcery.",
            &[
                CostDef::Mana(mana_cost!("{1}{B}")),
                CostDef::ExileSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        )
        .with_source_zones(&[ZoneKind::Graveyard])
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// MH3 114 — Amped Raptor
pub(in crate::card::sets) static AMPED_RAPTOR: CardRecord = CardRecord::new(
    "Amped Raptor",
    "1ac0e78b-0fdd-44f9-8b7b-c4f28a32782e",
    "Alex Konstad",
// Two mana for a 2/1 first striker and a free spell off the top, as long
    // as the top of the deck is cheap enough for two energy to cover.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Dinosaur"], 2, 1)
        .with_abilities(&[
            abilities::first_strike(),
            abilities::enters_trigger(
                "When this creature enters, you get {E}{E} (two energy counters). Then if you cast it \
                 from your hand, exile cards from the top of your library until you exile a nonland card. \
                 You may cast that card by paying an amount of {E} equal to its mana value rather than \
                 paying its mana cost.",
                // "Then if you cast it from your hand" is part of the effect rather than an
                // intervening-if: a Raptor put onto the battlefield gets the energy and
                // nothing else.
                EffectDef::Sequence(&[
                    EffectDef::AddPlayerCounters {
                        recipient: EffectRecipientDef::Controller,
                        kind: CounterKind::named("energy"),
                        amount: ValueDef::Constant(2),
                    },
                    EffectDef::IfCondition {
                        condition: &TriggerConditionDef::SourceCastFrom(ZoneKind::Hand),
                        then: &EffectDef::ExileFromTopUntil {
                            player: EffectRecipientDef::Controller,
                            // A land is what the exile walks past; the first thing that is not one is
                            // what you get to keep.
                            object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                            permission: ExiledCastPermissionDef::EnergyEqualToManaValue,
                        },
                    },
                ]),
            ),
        ]),
);

// MH3 116 — Detective's Phoenix
pub(in crate::card::sets) static DETECTIVES_PHOENIX: CardRecord = CardRecord::new(
    "Detective's Phoenix",
    "e2a01edd-dbc0-4ed4-b827-9b608290e9a1",
    "Deruchenko Alexander",
    // A three-mana hasty flier that never really dies: once the graveyard is
    // six mana deep it comes back out of it for {R}, as an Aura, and comes
    // back again as a creature when whatever it was wearing is gone.
    CardRules::new_enchantment_creature(mana_cost!("{2}{R}"), &["Phoenix"], 2, 2)
        .with_abilities(&[
        AbilityDef::alternative_cast_with_targets(
            &[
                CostDef::Mana(mana_cost!("{R}")),
                CostDef::exile(
                    ObjectPredicateDef::Any,
                    ZoneKind::Graveyard,
                    CostQuantityDef::ObjectSetValueAtLeast(&ObjectSetValueAtLeastDef {
                        value: ObjectSetValueDef::Aggregate {
                            select: ObjectValueDef::ManaValue,
                            operation: AggregateOperationDef::Sum,
                        },
                        minimum: 6,
                    }),
                ),
            ],
            AlternativeCastKindDef::Bestow,
            Some(
                "Bestow—{R}, Collect evidence 6. (To pay this bestow cost, pay {R} and exile cards \
                     with total mana value 6 or greater from your graveyard.)",
            ),
            &abilities::ENCHANT_CREATURE_TARGET,
            EffectDef::Attach {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        )
        // Collect evidence 6 (CR 701.58a): cards out of your own graveyard whose
        // mana values add up to six, however many that takes.
        .with_alternative_from_graveyard(),
        abilities::flying(),
        abilities::haste(),
        // Only while it is an Aura: unattached, the recipient names nothing and
        // the clause does nothing, which is exactly CR 702.103d.
        AbilityDef::static_ability(
            "Enchanted creature gets +2/+2 and has flying and haste.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::AttachedPermanent,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                    AppliedEffectDef::add_ability(&abilities::flying()),
                    AppliedEffectDef::add_ability(&abilities::haste()),
                ]),
            },
        ),
        AbilityDef::static_ability(
            "You may cast this card from your graveyard using its bestow ability.",
            EffectDef::None,
        )
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// MH3 122 — Galvanic Discharge
pub(in crate::card::sets) static GALVANIC_DISCHARGE: CardRecord = CardRecord::new(
    "Galvanic Discharge",
    "32aa6e33-221f-414c-9b51-850d97a7e051",
    "Zoltan Boros",
    // One mana that kills a three-toughness creature and leaves the energy
    // behind when it does not need all of it.
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::spell_with_targets(
        "Choose target creature or planeswalker. You get {E}{E}{E} (three energy counters), then \
         you may pay any amount of {E}. Galvanic Discharge deals that much damage to that \
         permanent.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasType(CardType::Planeswalker),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::AddPlayerCounters {
                recipient: EffectRecipientDef::Controller,
                kind: CounterKind::named("energy"),
                amount: ValueDef::Constant(3),
            },
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::ChosenEnergy], // "That much damage": the amount the payment settled, which is what makes
                // the three energy it hands out into three damage the turn it is cast and
                // more than that on a board that has been banking it.
                &EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::PaidAmount,
                ),
            )),
        ]),
    )),
);

// MH3 123 — Ghostfire Slice
pub(in crate::card::sets) static GHOSTFIRE_SLICE_123: CardRecord = CardRecord::new(
    "Ghostfire Slice",
    "2adea3ee-138f-455b-a001-586883c44758",
    "Johann Bodin",
    CardRules::new_instant(mana_cost!("{2}{R}")).with_abilities(&[
        abilities::devoid(),
        abilities::spell_cost_reduction(
            "This spell costs {2} less to cast if an opponent controls a multicolored permanent.",
            ObjectPredicateDef::Source,
            PlayerRelation::You,
            ValueDef::IfMatchingObjectCount(&CountConditionDef {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::ColorCount(0),
                        ObjectPredicateDef::ColorCount(1),
                    ])),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Opponent,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
                then: ValueDef::Constant(2),
                otherwise: ValueDef::Constant(0),
            }),
        ),
        AbilityDef::spell_with_targets(
            "Ghostfire Slice deals 4 damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(4),
            ),
        ),
    ]),
);

// MH3 124 — Glimpse the Impossible
// Audit: unsupported — Exile identities and delayed moves can be bound, but an installed trigger checks its intervening-if condition without the captured object bindings. It cannot test whether any of this spell's particular cards remain exiled at the beginning of the end step. An unconditional trigger with a resolution-only check would still trigger when the printed ability must not.
pub(in crate::card::sets) static GLIMPSE_THE_IMPOSSIBLE_124: CardRecord = CardRecord::new(
    "Glimpse the Impossible",
    "133ad0dd-5b61-4c38-9264-0b0e75b95d95",
    "Justine Jones",
    crate::card::CardRules::unsupported(),
);

// MH3 128 — Molten Gatekeeper
// Audit: unsupported — Needs unearth; see First-Sphere Gargantua. The entry-damage trigger itself is expressible.
pub(in crate::card::sets) static MOLTEN_GATEKEEPER: CardRecord = CardRecord::new(
    "Molten Gatekeeper",
    "9f5ba065-2806-4e99-a330-168cfe76250f",
    "Joe Slucher",
    crate::card::CardRules::unsupported(),
);

// MH3 136 — Siege Smash
pub(in crate::card::sets) static SIEGE_SMASH_136: CardRecord = CardRecord::new(
    "Siege Smash",
    "f33e3b25-76f5-4263-a309-9ea97f2d8248",
    "Joshua Cairos",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[
        abilities::split_second(),
        AbilityDef::modal_spell(
            "Choose one —",
            &[
                AbilityDef::spell_with_targets(
                    "• Destroy target artifact.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Artifact),
                    )],
                    EffectDef::destroy_target(TargetIndex::PRIMARY),
                ),
                AbilityDef::spell_with_targets(
                    "• Target creature gets +3/+2 and gains trample until end of turn.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    )],
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::modify_power_toughness(
                                ValueDef::Constant(3),
                                ValueDef::Constant(2),
                            ),
                            AppliedEffectDef::add_ability(&abilities::trample()),
                        ]),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ),
            ],
        ),
    ]),
);

// MH3 145 — Basking Broodscale
pub(in crate::card::sets) static BASKING_BROODSCALE: CardRecord = CardRecord::new(
    "Basking Broodscale",
    "5feba5d6-99a6-4e9b-8a7d-90d955868fc3",
    "Caio Monteiro",
    // Adapt only ever fires once, so the token engine is what a deck is
    // really buying: anything else that puts counters on this keeps paying.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Eldrazi", "Lizard"], 2, 2).with_abilities(&[
        abilities::devoid(),
        AbilityDef::activated(
            "{1}{G}: Adapt 1. (If this creature has no +1/+1 counters on it, put a +1/+1 counter \
             on it.)",
            &[CostDef::Mana(mana_cost!("{1}{G}"))],
            // Adapt is a conditional rather than a cost: the ability always
            // resolves, and finding a counter already there is what makes it
            // do nothing.
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    comparison: ComparisonDef::LessOrEqual,
                    amount: 0,
                },
                then: &EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            },
        ),
        AbilityDef::triggered(
            "Whenever one or more +1/+1 counters are put on this creature, you may create a 0/1 \
             colorless Eldrazi Spawn creature token with \"Sacrifice this token: Add {C}.\"",
            // "One or more" is one trigger for the whole placement, not one
            // per counter.
            TriggerEventDef::CountersPlaced {
                object: ObjectPredicateDef::Source,
                kind: CounterKind::PlusOnePlusOne,
            },
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    ELDRAZI_SPAWN_TOKEN,
                ))),
            },
        ),
    ]),
);

// MH3 147 — Collective Resistance
pub(in crate::card::sets) static COLLECTIVE_RESISTANCE: CardRecord = CardRecord::new(
    "Collective Resistance",
    "f260bd08-68b6-44f4-ace9-e298cb13d82e",
    "Raoul Vitale",
    CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(escalate(
        "Escalate {G} (Pay this cost for each mode chosen beyond the first.)",
        CostDef::pay_mana(mana_cost!("{G}")),
        &[
            AbilityDef::destroy_target(
                "Destroy target artifact.",
                &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Artifact,
                )),
            ),
            AbilityDef::destroy_target(
                "Destroy target enchantment.",
                &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Enchantment,
                )),
            ),
            AbilityDef::spell_with_targets(
                "Target creature gains hexproof and indestructible until end of turn.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_ability(&abilities::hexproof()),
                        AppliedEffectDef::add_ability(&abilities::indestructible()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    )),
);

// MH3 148 — Colossal Dreadmask
pub(in crate::card::sets) static COLOSSAL_DREADMASK: CardRecord = CardRecord::new(
    "Colossal Dreadmask",
    "98164430-64c1-465f-b786-45753c965f44",
    "Caio Monteiro",
    CardRules::new_artifact(mana_cost!("{4}{G}{G}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::living_weapon(),
            AbilityDef::static_ability(
                "Equipped creature gets +6/+6 and has trample.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(6),
                            ValueDef::Constant(6),
                        ),
                        AppliedEffectDef::add_ability(&abilities::trample()),
                    ]),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{3}{G}{G}"))], "Equip {3}{G}{G}"),
        ]),
);

// MH3 150 — Eldrazi Repurposer
pub(in crate::card::sets) static ELDRAZI_REPURPOSER: CardRecord = CardRecord::new(
    "Eldrazi Repurposer",
    "37f79ba7-7b65-4387-b498-f770816ce8dd",
    "Daren Bader",
    // A Spawn on the way in and another on the way out, so trading it away
    // still leaves the mana behind.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Eldrazi", "Drone"], 3, 3).with_abilities(&[
        abilities::devoid(),
        AbilityDef::triggered(
            "When you cast this spell and when this creature dies, create a 0/1 colorless \
             Eldrazi Spawn creature token with \"Sacrifice this token: Add {C}.\"",
            // One printed ability with two ways in, so what it does is
            // written once. The cast half fires while this is still on the
            // stack; the death half is a separate later trigger.
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::spell_cast(ObjectPredicateDef::Source),
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
            ]),
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(ELDRAZI_SPAWN_TOKEN))),
        ),
    ]),
);

// MH3 151 — Evolution Witness
pub(in crate::card::sets) static EVOLUTION_WITNESS: CardRecord = CardRecord::new(
    "Evolution Witness",
    "4d89283e-9783-4006-9294-4ae0473d2ce6",
    "Nereida",
    // Adapt only ever fires once, so anything else that puts counters on it
    // is what turns this into a repeatable regrowth.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf", "Shaman", "Mutant"], 2, 1)
        .with_abilities(&[
            AbilityDef::activated(
                "{1}{G}: Adapt 2. (If this creature has no +1/+1 counters on it, put two +1/+1 \
                 counters on it.)",
                &[CostDef::Mana(mana_cost!("{1}{G}"))],
                // Adapt is a conditional rather than a cost: the ability
                // always resolves, and finding a counter already there is
                // what makes it do nothing.
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceCounters {
                        kind: CounterKind::PlusOnePlusOne,
                        comparison: ComparisonDef::LessOrEqual,
                        amount: 0,
                    },
                    then: &EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(2),
                    },
                },
            ),
            AbilityDef::triggered_with_targets(
                "Whenever one or more +1/+1 counters are put on this creature, return target \
                 permanent card from your graveyard to your hand.",
                // "One or more" is one trigger for the whole placement, not
                // one per counter, so adapt 2 buys back a single card.
                TriggerEventDef::CountersPlaced {
                    object: ObjectPredicateDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                },
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        // "Permanent card" is spelled out: there is no
                        // permanent card type, only the five that make one.
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                        ]),
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::You),
                    },
                )],
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            ),
        ]),
);

static FANATIC_TAP: [CostDef; 1] = [CostDef::TapSource];

// MH3 152 — Fanatic of Rhonas
pub(in crate::card::sets) static FANATIC_OF_RHONAS: CardRecord = CardRecord::new(
    "Fanatic of Rhonas",
    "1f9fb33a-3b39-4aff-93b8-aedafe0ea694",
    "Scott Murphy",
// Two mana for a 1/4 that taps for one, and for four the moment anything
    // large is beside it -- and a 4/4 out of the graveyard afterwards.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Snake", "Druid"], 1, 4)
        .with_abilities(&[
            AbilityDef::activated_mana(
                "{T}: Add {G}.",
                &FANATIC_TAP,
                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
            ),
            AbilityDef::activated_mana_if(
                "Ferocious — {T}: Add {G}{G}{G}{G}. Activate only if you control a creature with power 4 \
                 or greater.",
                &FANATIC_TAP,
                &TriggerConditionDef::ObjectCount {
                    // Ferocious: a creature with power four or greater, which the Fanatic is
                    // not, so something else has to be there.
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::PowerAtLeast(4),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green).with_amount(4)),
            ),
            abilities::eternalize!(
                "Eternalize {2}{G}{G} ({2}{G}{G}, Exile this card from your graveyard: Create a token \
                that's a copy of it, except it's a 4/4 black Zombie Snake Druid with no mana cost. \
                Eternalize only as a sorcery.)",
                &[CostDef::Mana(mana_cost!("{2}{G}{G}"))],
            ),
        ]),
);

// MH3 157 — Horrific Assault
pub(in crate::card::sets) static HORRIFIC_ASSAULT: CardRecord = CardRecord::new(
    "Horrific Assault",
    "cfa6ed13-7bba-40c0-8e0e-4ffd3cea6241",
    "Justine Jones",
    // One mana of removal priced off a board you already have, with the life
    // as a small bribe to the Eldrazi deck this set is built around.
    CardRules::new_sorcery(mana_cost!("{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature you control deals damage equal to its power to target creature or \
         planeswalker you don't control. If you control an Eldrazi, you gain 3 life.",
        &[
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            }),
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::NotYou),
                owner: None,
            }),
        ],
        EffectDef::Sequence(&[
            // One-sided, so the damage is dealt by the chosen creature
            // rather than by this spell.
            EffectDef::damage_from(
                ObjectRefDef::Target(TargetIndex::PRIMARY),
                EffectRecipientDef::Target(TargetIndex(1)),
                ValueDef::TargetPower(TargetIndex::PRIMARY),
            ),
            EffectDef::IfCondition {
                // Read as this resolves, and the creature that dealt the
                // damage may itself be the Eldrazi being counted.
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Eldrazi")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
                then: &EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
            },
        ]),
    )),
);

// MH3 161 — Malevolent Rumble
pub(in crate::card::sets) static MALEVOLENT_RUMBLE: CardRecord = CardRecord::new(
    "Malevolent Rumble",
    "a178cfe8-f9fa-4255-88d0-54a0bed079f5",
    "Néstor Ossandón Leal",
    // Two mana that finds a permanent, fills the graveyard with the three
    // it did not want, and leaves behind the mana that makes the next spell
    // a turn early.
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell(
        "Reveal the top four cards of your library. You may put a permanent card from among them \
         into your hand. Put the rest into your graveyard. Create a 0/1 colorless Eldrazi Spawn \
         creature token with \"Sacrifice this token: Add {C}.\"",
        EffectDef::Sequence(&[
            abilities::reveal_top_cards_choose_to_hand_rest_graveyard(
                ValueDef::Constant(4),
                // "A permanent card from among them": taking nothing is a legal answer,
                // and everything not taken is buried whether or not it could have been.
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                ]),
                0,
                1,
            ),
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(ELDRAZI_SPAWN_TOKEN))),
        ]),
    )),
);

// MH3 164 — Nyxborn Hydra
pub(in crate::card::sets) static NYXBORN_HYDRA: CardRecord = CardRecord::new(
    "Nyxborn Hydra",
    "902a969e-9f22-4e92-93eb-9d4536ca82e5",
    "Vincent Christiaens",
    // An X/X trampler, or two more mana to make something else that much
    // bigger and keep the body in reserve. Both halves read the same
    // counters, which is what makes bestowing it a real choice rather than
    // a worse Giant Growth.
    CardRules::new_creature(mana_cost!("{X}{G}"), &["Hydra"], 0, 0)
        .with_type(CardType::Enchantment)
        .with_abilities(&[
            AbilityDef::alternative_cast_with_targets(
                &[CostDef::Mana(mana_cost!("{X}{G}{G}"))],
                AlternativeCastKindDef::Bestow,
                Some(
                    "Bestow {X}{G}{G} (If you cast this card for its bestow cost, it's an Aura \
                     spell with enchant creature. It becomes a creature again if it's not \
                     attached.)",
                ),
                &abilities::ENCHANT_CREATURE_TARGET,
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            abilities::reach(),
            abilities::trample(),
            AbilityDef::as_enters(
                "This permanent enters with X +1/+1 counters on it.",
                // The X it was cast for, whichever cost paid it: bestowed, the
                // Aura carries the counters the creature half would have had,
                // and the clause below is what spends them.
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::AddCastXCounters {
                        kind: CounterKind::PlusOnePlusOne,
                    },
                ),
            ),
            // Only while it is an Aura (CR 702.103d). The bonus is read off
            // this permanent's own counters rather than fixed at attachment,
            // so anything that grows the Aura grows what it is wearing.
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+1 for each +1/+1 counter on this Aura and has reach \
                 and trample.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::CountersOnSource(CounterKind::PlusOnePlusOne),
                            ValueDef::CountersOnSource(CounterKind::PlusOnePlusOne),
                        ),
                        AppliedEffectDef::add_ability(&abilities::reach()),
                        AppliedEffectDef::add_ability(&abilities::trample()),
                    ]),
                },
            ),
        ]),
);

// MH3 169 — Six
/// Where the taken land is saved, kept apart from the milled pile so that
/// "them" and "the one you took" stay two different sets.
pub(in crate::card::sets) static SIX: CardRecord = CardRecord::new(
    "Six",
    "f9246b68-580f-4f53-883d-7900880e4b0d",
    "Andrew Mar",
// A blocker that fills the graveyard and then plays out of it: every
    // land the attack finds is another permanent cast back from the pile.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Treefolk"], 2, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::reach(),
            AbilityDef::triggered(
                "Whenever Six attacks, mill three cards. You may put a land card from among them \
                 into your hand.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                EffectDef::Sequence(&[
                    EffectDef::BindOutput {
                        effect: &EffectDef::Mill {
                            player: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(3),
                        },
                        binding: Binding!("milled_cards"),
                    },
                    // A minimum of zero is the "you may": milling three and taking nothing is a
                    // legal answer, and a pile with no land in it never asks.
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        // "From among them" is what the mill just put there, not what the graveyard
                        // already held -- and only a land among those.
                        candidates: ObjectSetDef::Matching {
                            objects: &ObjectSetDef::Binding(Binding!("milled_cards")),
                            object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::HasType(
                                CardType::Land,
                            )),
                        },
                        exclude: None,
                        minimum: 0,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                            ZoneKind::Hand,
                            ZonePlacement::Top,
                        ),
                    }),
                ]),
            ),
            AbilityDef::static_ability(
                "During your turn, nonland permanent cards in your graveyard have retrace. (You \
                 may cast permanent cards from your graveyard by discarding a land card in \
                 addition to paying their other costs.)",
                EffectDef::IfCondition {
                    // "During your turn" is a gate on the permission rather than on what it
                    // names: on their turn the cards in your graveyard have nothing.
                    condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::You)),
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::GrantsAlternativeCastFromGraveyard {
                            // "Nonland permanent cards": what the grant reaches is every card that
                            // would become a permanent, which is the whole of what a Treefolk deck
                            // throws away.
                            object: ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasType(CardType::Enchantment),
                                ObjectPredicateDef::HasType(CardType::Planeswalker),
                            ]),
                            ability: &AbilityDef::alternative_cast(
                                &[CostDef::ManaCostOf(crate::ObjectRefDef::Source), CostDef::discard(ObjectPredicateDef::HasType(CardType::Land))],
                                AlternativeCastKindDef::Retrace,
                                Some(
                                    "Retrace (You may cast this card from your graveyard by discarding a land card in \
                                     addition to paying its other costs.)",
                                ),
                                EffectDef::None,
                            )
                            // Retrace's own cost: the card's mana cost, plus a land out of your hand.
                            ,
                        }),
                    },
                },
            ),
        ]),
);

// MH3 170 — Sowing Mycospawn
pub(in crate::card::sets) static SOWING_MYCOSPAWN: CardRecord = CardRecord::new(
    "Sowing Mycospawn",
    "cdfadb17-76ad-4d4d-9fa7-33c4b88b4c0a",
    "Slawomir Maniak",
// Four mana finds a land and six exiles one, and both happen on the cast
    // rather than on arrival -- so countering the creature does not stop
    // either of them.
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Eldrazi", "Fungus"], 3, 3)
        .printed_colors(&[])
        .with_abilities(&[
            // Devoid is the empty printed colour set below; the keyword is here so
            // the card says what it is.
            abilities::devoid(),
            AbilityDef::alternative_cast(
                &[CostDef::Mana(mana_cost!("{4}{G}{C}"))],
                AlternativeCastKindDef::Kicked,
                Some("Kicker {1}{C} (You may pay an additional {1}{C} as you cast this spell.)"),
                EffectDef::None,
            ),
            AbilityDef::triggered(
                "When you cast this spell, search your library for a land card, put it onto the battlefield, then shuffle.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::Source),
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: false,
                    destination: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            ),
            AbilityDef::triggered_if_with_targets(
                "When you cast this spell, if it was kicked, exile target land.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::Source),
                // The kicked half changes nothing about how the spell resolves: it costs
                // more, and the second cast trigger reads that fact. That is why the
                // alternative carries no instructions of its own.
                &TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Land),
                )],
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
            ),
        ]),
);

// MH3 171 — Springheart Nantuko
pub(in crate::card::sets) static SPRINGHEART_NANTUKO: CardRecord = CardRecord::new(
    "Springheart Nantuko",
    "54a3ea87-005e-4985-b2a5-21711d0b71c0",
    "Valera Lutfullina",
// Two mana for a 1/1, or four to bestow it onto something worth copying
    // -- and then every land is another one of that.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Insect", "Monk"], 1, 1)
        .with_type(CardType::Enchantment)
        .with_abilities(&[
            AbilityDef::alternative_cast_with_targets(
                &[CostDef::Mana(mana_cost!("{1}{G}"))],
                AlternativeCastKindDef::Bestow,
                Some(
                    "Bestow {1}{G} (If you cast this card for its bestow cost, it's an Aura spell with \
                     enchant creature. It becomes a creature again if it's not attached to a creature.)",
                ),
                &abilities::ENCHANT_CREATURE_TARGET,
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            // Only while it is an Aura (CR 702.103d): as a creature it enchants
            // nothing and the clause names nothing.
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
            AbilityDef::triggered(
                "Landfall — Whenever a land you control enters, you may pay {1}{G} if this permanent is \
                 attached to a creature you control. If you do, create a token that's a copy of that \
                 creature. If you didn't create a token this way, create a 1/1 green Insect creature \
                 token.",
                TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]), None, Some(ZoneKind::Battlefield)),
                EffectDef::PayOr(
                    PayOrDef::optional_or(
                        &[CostDef::Mana(mana_cost!("{1}{G}"))], // The whole point of bestowing it: every land is another copy of whatever
                        // it is wearing.
                        &EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Copy(
                            &crate::card::TokenCopyDef {
                                object: &EffectRecipientDef::AttachedPermanent,
                                exceptions: CopyExceptionsDef::NONE,
                            },
                        ))), // "If you didn't create a token this way": declining, being unable to pay,
                        // and not being attached at all are the same answer, and each leaves an
                        // Insect behind.
                        &EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                            TokenCharacteristics::creature(&["Insect"], &[ManaColor::Green], 1, 1),
                        ))),
                    )
                    // "If this permanent is attached to a creature you control": read before
                    // the offer, because a Nantuko that is a creature rather than an Aura has
                    // nothing to copy and should not be asked to pay for one.
                    .only_if(&TriggerConditionDef::AttachedPermanentMatches {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ]),
                    }),
                ),
            ),
        ]),
);

// MH3 172 — Temperamental Oozewagg
// Audit: unsupported — Needs a "modified" object predicate. A creature is modified if it has a counter on it, is equipped, or is enchanted by an Aura its controller controls; ObjectPredicateDef can say the counter half but has no predicate for the whole, and covering only counters would silently miss the Equipment and Aura halves this set's payoffs are built around.
pub(in crate::card::sets) static TEMPERAMENTAL_OOZEWAGG: CardRecord = CardRecord::new(
    "Temperamental Oozewagg",
    "6625df2e-7046-411a-ae86-c46ac0953a0b",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// MH3 179 — Conduit Goblin
pub(in crate::card::sets) static CONDUIT_GOBLIN: CardRecord = CardRecord::new(
    "Conduit Goblin",
    "5c9ad04d-c4d4-4d06-93bb-a881be733717",
    "Bruno Biazotto",
    // Two energy in, one energy a turn out. The haste is the half that
    // matters: it turns whatever was cast this turn into an attacker, so the
    // Goblin keeps paying a deck that never stops adding bodies.
    CardRules::new_creature(mana_cost!("{R}{W}"), &["Goblin", "Warrior"], 2, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, you get {E}{E} (two energy counters).",
            EffectDef::AddPlayerCounters {
                recipient: EffectRecipientDef::Controller,
                kind: CounterKind::named("energy"),
                amount: ValueDef::Constant(2),
            },
        ),
        AbilityDef::triggered_with_targets(
            "At the beginning of combat on your turn, you may pay {E}. If you do, another target \
             creature you control gets +1/+0 and gains haste until end of turn.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            // "Another" excludes the Goblin itself, so a board holding nothing
            // else leaves the trigger with no legal target and it is removed
            // before anyone is asked to spend energy.
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
            )],
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::Energy(1)],
                &EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::add_ability(&abilities::haste()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            )),
        ),
    ]),
);

// MH3 184 — Expanding Ooze
// Audit: unsupported — Needs a modified predicate. "Target modified creature you control" asks whether a permanent carries an Aura, Equipment, or counter, and no predicate says it.
pub(in crate::card::sets) static EXPANDING_OOZE: CardRecord = CardRecord::new(
    "Expanding Ooze",
    "bbdb095d-b826-4e3e-8c61-0d408e52d6b8",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// MH3 185 — Faithful Watchdog
pub(in crate::card::sets) static FAITHFUL_WATCHDOG: CardRecord = CardRecord::new(
    "Faithful Watchdog",
    "b9afac99-a094-41a8-8323-90dec29691c4",
    "Samuel Perin",
    // Printed 0/0, so the counters are the body rather than a bonus: it
    // dies to anything that removes them.
    CardRules::new_creature(mana_cost!("{G}{W}"), &["Dog"], 0, 0).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::as_enters(
            "This creature enters with three +1/+1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 3,
                },
            ),
        ),
    ]),
);

// MH3 195 — Obstinate Gargoyle
pub(in crate::card::sets) static OBSTINATE_GARGOYLE_195: CardRecord = CardRecord::new(
    "Obstinate Gargoyle",
    "40cf39f2-7382-405d-a14b-7eb8726cd38a",
    "Craig J Spearing",
    CardRules::new_artifact_creature(mana_cost!("{1}{W}{B}"), &["Gargoyle"], 2, 2).with_abilities(&[
AbilityDef::static_ability("This creature has flying as long as it's modified. (Equipment, Auras you control, and counters are modifications.)", EffectDef::IfCondition { condition: &TriggerConditionDef::AnyOf(&[TriggerConditionDef::SourceMatches { object: ObjectPredicateDef::HasAnyCounter }, TriggerConditionDef::ValueComparison(&ValueComparisonDef { left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(ObjectPredicateDef::All(&[ObjectPredicateDef::Subtype(SubtypeDef::Literal("Equipment")), ObjectPredicateDef::AttachedTo(&ObjectPredicateDef::Source)]), &[ZoneKind::Battlefield], PlayerRelation::Any)), comparison: ComparisonDef::Greater, right: ValueDef::Constant(0) }), TriggerConditionDef::ValueComparison(&ValueComparisonDef { left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(ObjectPredicateDef::All(&[ObjectPredicateDef::Subtype(SubtypeDef::Literal("Aura")), ObjectPredicateDef::AttachedTo(&ObjectPredicateDef::Source)]), &[ZoneKind::Battlefield], PlayerRelation::You)), comparison: ComparisonDef::Greater, right: ValueDef::Constant(0) })]), then: &EffectDef::StaticApply { recipient: EffectRecipientDef::Source, effect: AppliedEffectDef::add_ability(&abilities::flying()) } }),
abilities::persist()
]),
);

// MH3 197 — Phlage, Titan of Fire's Fury
pub(in crate::card::sets) static PHLAGE_TITAN_OF_FIRES_FURY: CardRecord =
    CardRecord::new(
    "Phlage, Titan of Fire's Fury",
    "e419cd0b-2449-4cc5-9ead-b9e45e271700",
    "Lucas Graciano",
// A three-mana Lightning Helix that stays a Helix until the graveyard is
        // deep enough, and then is a 6/6 that helixes again every attack.
        CardRules::new_creature(mana_cost!("{1}{R}{W}"), &["Elder", "Giant"], 6, 6)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&[
                AbilityDef::triggered_if(
                    "When this creature enters, sacrifice it unless it escaped.",
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::Source,
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                    // "Unless it escaped" reads how the spell was cast, which the permanent
                    // remembers: a Phlage cast for its printed cost sacrifices itself and leaves
                    // the Lightning Helix behind.
                    &TriggerConditionDef::Not(
                        &TriggerConditionDef::All(&[
                            TriggerConditionDef::SourceWasCast,
                            TriggerConditionDef::SourceCastFrom(ZoneKind::Graveyard),
                            TriggerConditionDef::SourceCastWith(
                                AlternativeCastKindDef::Escape,
                            ),
                        ]),
                    ),
                    EffectDef::sacrifice(EffectRecipientDef::Source),
                ),
                AbilityDef::triggered_with_targets(
                    "Whenever this creature enters or attacks, it deals 3 damage to any target and you gain \
                     3 life.",
                    // Entering and attacking are two ways for one printed ability to fire, so
                    // the damage and the life are written once.
                    TriggerEventDef::AnyOf(&[
                        TriggerEventDef::zone_changed(
                            ObjectPredicateDef::Source,
                            None,
                            Some(ZoneKind::Battlefield),
                        ),
                        TriggerEventDef::attacks(ObjectPredicateDef::Source),
                    ]),
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::AnyTarget,
                    )],
                    EffectDef::Sequence(&[
                        EffectDef::damage(
                            EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            ValueDef::Constant(3),
                        ),
                        EffectDef::GainLife {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(3),
                        },
                    ]),
                ),
                escape(&[CostDef::Mana(mana_cost!("{R}{R}{W}{W}")), CostDef::exile(crate::ObjectPredicateDef::Any, crate::ZoneKind::Graveyard, crate::card::CostQuantityDef::Fixed(5))]),
            ]),
);

// MH3 199 — Psychic Frog
pub(in crate::card::sets) static PSYCHIC_FROG: CardRecord = CardRecord::new(
    "Psychic Frog",
    "68924203-c3d9-41ce-8ca8-c6dd491eb3ca",
    "Pete Venters",
    // Two mana that turns a full hand into a big evasive body and a full
    // graveyard into the evasion, and draws a card every time it connects.
    CardRules::new_creature(mana_cost!("{U}{B}"), &["Frog"], 1, 2).with_abilities(&[
        // A player or a planeswalker: the Frog is happy to be chumped by neither.
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player or planeswalker, draw a card.",
            TriggerEventDef::combat_damage_to_player_or_planeswalker(ObjectPredicateDef::Source),
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
        // No mana in either cost, and no tap: the Frog grows as often as the hand
        // allows and flies as often as the graveyard does.
        AbilityDef::activated(
            "Discard a card: Put a +1/+1 counter on this creature.",
            &[CostDef::discard(ObjectPredicateDef::Any)],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::apply_to_self_until_end_of_turn(
            "Exile three cards from your graveyard: This creature gains flying until end of turn.",
            &[CostDef::MoveToZone(crate::card::MoveToZoneCostDef::new(
                ObjectPredicateDef::Any,
                ZoneKind::Graveyard,
                ZoneKind::Exile,
                3,
            ))],
            AppliedEffectDef::add_ability(&abilities::flying()),
        ),
    ]),
);

// MH3 204 — Snapping Voidcraw
pub(in crate::card::sets) static SNAPPING_VOIDCRAW: CardRecord = CardRecord::new(
    "Snapping Voidcraw",
    "9185371c-2dde-48ad-ab27-08be04b3c522",
    "Camille Alquier",
    // A blocker that ramps two and then turns the spare colourless into
    // cards, which is what an Eldrazi deck wants from three mana.
    CardRules::new_creature(mana_cost!("{1}{G}{U}"), &["Eldrazi", "Turtle"], 1, 3).with_abilities(
        &[
            abilities::devoid(),
            AbilityDef::activated_mana(
                "{T}: Add {C}{C}.",
                &[CostDef::TapSource],
                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_amount(2)),
            ),
            AbilityDef::activated(
                "{3}{C}, {T}: Draw a card.",
                // The {C} has to be colourless specifically, which is what
                // makes this pair with the mana ability above rather than
                // with any three lands.
                &[CostDef::Mana(mana_cost!("{3}{C}")), CostDef::TapSource],
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
        ],
    ),
);

// MH3 208 — Writhing Chrysalis
pub(in crate::card::sets) static WRITHING_CHRYSALIS: CardRecord = CardRecord::new(
    "Writhing Chrysalis",
    "f54dbeb1-51f8-40e2-912a-ec25457de5a2",
    "Domenico Cava",
    // Four mana for three bodies and a sacrifice engine to feed on them,
    // which is what makes the small stats beside the point.
    CardRules::new_creature(mana_cost!("{2}{R}{G}"), &["Eldrazi", "Drone"], 2, 3).with_abilities(
        &[
            abilities::devoid(),
            AbilityDef::triggered(
                "When you cast this spell, create two 0/1 colorless Eldrazi Spawn creature tokens \
             with \"Sacrifice this token: Add {C}.\"",
                // A cast trigger, so the Spawn arrive while this is still on the
                // stack and can help pay for whatever follows it.
                TriggerEventDef::spell_cast(ObjectPredicateDef::Source),
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(ELDRAZI_SPAWN_TOKEN)).with_amount(2),
                ),
            ),
            abilities::reach(),
            AbilityDef::triggered(
                "Whenever you sacrifice another Eldrazi, put a +1/+1 counter on this creature.",
                TriggerEventDef::Sacrificed {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Eldrazi")),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    player: PlayerRelation::You,
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
        ],
    ),
);

// MH3 209 — Disruptor Flute
pub(in crate::card::sets) static DISRUPTOR_FLUTE: CardRecord = CardRecord::new(
    "Disruptor Flute",
    "5cad8671-4761-4014-a8a3-af45627e6e79",
    "Xavier Ribeiro",
CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        abilities::flash(),
        AbilityDef::as_enters(
            "As this artifact enters, choose a card name.",
            crate::card::ReplacementEffectDef::BindOutput {
                binding: Binding!("disruptor_flute_name"),
                effect: &abilities::choose_card_name_as_enters(
                    crate::card::CardNameSetDef::AllCardNames,
                ),
            },
        ),
        abilities::spell_cost_increase_for_name(
            "Spells with the chosen name cost {3} more to cast.",
            crate::card::CardNameDef::Binding(Binding!("disruptor_flute_name")),
            PlayerRelation::Any,
            mana_cost!("{3}"),
        ),
        abilities::cannot_activate_nonmana_abilities_with_name(
            "Activated abilities of sources with the chosen name can't be activated unless they're mana abilities.",
            crate::card::CardNameDef::Binding(Binding!("disruptor_flute_name")),
        ),
    ]),
);

// MH3 212 — Vexing Bauble
// Audit: unsupported — Cast triggers cannot inspect whether no mana was spent on the triggering spell.
pub(in crate::card::sets) static VEXING_BAUBLE_212: CardRecord = CardRecord::new(
    "Vexing Bauble",
    "29f11089-658f-42e6-aeb0-09b512ad2479",
    "Tony Foti",
    crate::card::CardRules::unsupported(),
);

// MH3 217 — Bountiful Landscape
pub(in crate::card::sets) static BOUNTIFUL_LANDSCAPE: CardRecord = CardRecord::new(
    "Bountiful Landscape",
    "b277752b-430a-4f09-8a98-b72f813dd52e",
    "Mark Poole",
// A land that taps for nothing useful and fetches a tapped basic, which
    // is worth a slot only because it is also a cycling card and because
    // what it finds is a land drop somebody else paid for.
    CardRules::new_land(&[]).with_abilities(&landscape_abilities(
        "{T}, Sacrifice this land: Search your library for a basic Forest, Island, or Mountain card, \
         put it onto the battlefield tapped, then shuffle.",
        // A basic one, not merely a card with the type: the tri-fetch cycle names
        // three basics and finds nothing else, which is what separates it from the
        // fetchlands that read "a Mountain or Plains card".
        ObjectPredicateDef::All(&[
            ObjectPredicateDef::Supertype(CardSupertype::Basic),
            ObjectPredicateDef::HasAnyBasicLandType(&[
                BasicLandType::Forest,
                BasicLandType::Island,
                BasicLandType::Mountain,
            ]),
        ]),
        &abilities::cycling!(
            "Cycling {G}{U}{R} ({G}{U}{R}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{G}{U}{R}"))],
        ),
    )),
);

// MH3 218 — Contaminated Landscape
pub(in crate::card::sets) static CONTAMINATED_LANDSCAPE: CardRecord = CardRecord::new(
    "Contaminated Landscape",
    "e2312c49-1627-47ad-8113-78a999a97d8d",
    "Donato Giancola",
// Colourless mana now, a tapped basic later, or a card when the deck
    // has enough colours to pay for the cycling.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        abilities::landscape_fetch(
            "{T}, Sacrifice this land: Search your library for a basic Plains, Island, or Swamp card, put it onto the battlefield tapped, then shuffle.",
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::Supertype(CardSupertype::Basic),
                ObjectPredicateDef::HasAnyBasicLandType(&[
                    BasicLandType::Plains,
                    BasicLandType::Island,
                    BasicLandType::Swamp,
                ]),
            ]),
        ),
        abilities::cycling!(
            "Cycling {W}{U}{B} ({W}{U}{B}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{W}{U}{B}"))],
        ),
    ]),
);

// MH3 219 — Deceptive Landscape
pub(in crate::card::sets) static DECEPTIVE_LANDSCAPE: CardRecord = CardRecord::new(
    "Deceptive Landscape",
    "2ae6828e-ff19-45db-8b59-61616353491f",
    "Erikas Perl",
// The white-black-green Landscape; only the three types and the cycling
    // cost below are its own.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        abilities::landscape_fetch(
            "{T}, Sacrifice this land: Search your library for a basic Plains, Swamp, or Forest card, put it onto the battlefield tapped, then shuffle.",
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::Supertype(CardSupertype::Basic),
                ObjectPredicateDef::HasAnyBasicLandType(&[
                    BasicLandType::Plains,
                    BasicLandType::Swamp,
                    BasicLandType::Forest,
                ]),
            ]),
        ),
        abilities::cycling!(
            "Cycling {W}{B}{G} ({W}{B}{G}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{W}{B}{G}"))],
        ),
    ]),
);

// MH3 221 — Foreboding Landscape
pub(in crate::card::sets) static FOREBODING_LANDSCAPE: CardRecord = CardRecord::new(
    "Foreboding Landscape",
    "57fb0fa7-0c5c-4a75-9461-c51403c30282",
    "Erikas Perl",
// The black-green-blue Landscape; only the three types and the cycling
    // cost below are its own.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        abilities::landscape_fetch(
            "{T}, Sacrifice this land: Search your library for a basic Swamp, Forest, or Island card, put it onto the battlefield tapped, then shuffle.",
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::Supertype(CardSupertype::Basic),
                ObjectPredicateDef::HasAnyBasicLandType(&[
                    BasicLandType::Swamp,
                    BasicLandType::Forest,
                    BasicLandType::Island,
                ]),
            ]),
        ),
        abilities::cycling!(
            "Cycling {B}{G}{U} ({B}{G}{U}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{B}{G}{U}"))],
        ),
    ]),
);

// MH3 223 — Perilous Landscape
pub(in crate::card::sets) static PERILOUS_LANDSCAPE: CardRecord = CardRecord::new(
    "Perilous Landscape",
    "4b0bd07e-cf80-4d64-af29-f4cec6632b3e",
    "Alayna Danner",
// The blue-red-white Landscape; only the three types and the cycling
    // cost below are its own.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        abilities::landscape_fetch(
            "{T}, Sacrifice this land: Search your library for a basic Island, Mountain, or Plains card, put it onto the battlefield tapped, then shuffle.",
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::Supertype(CardSupertype::Basic),
                ObjectPredicateDef::HasAnyBasicLandType(&[
                    BasicLandType::Island,
                    BasicLandType::Mountain,
                    BasicLandType::Plains,
                ]),
            ]),
        ),
        abilities::cycling!(
            "Cycling {U}{R}{W} ({U}{R}{W}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{U}{R}{W}"))],
        ),
    ]),
);

// MH3 225 — Seething Landscape
pub(in crate::card::sets) static SEETHING_LANDSCAPE: CardRecord = CardRecord::new(
    "Seething Landscape",
    "661fc907-7003-45c6-820c-9616e9a71c30",
    "Piotr Dura",
    // The Grixis member of the cycle: colourless mana now, a tapped basic
    // later, or a card when the deck has enough colours for the cycling.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        abilities::landscape_fetch(
            "{T}, Sacrifice this land: Search your library for a basic Island, Swamp, or \
             Mountain card, put it onto the battlefield tapped, then shuffle.",
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::Supertype(CardSupertype::Basic),
                ObjectPredicateDef::HasAnyBasicLandType(&[
                    BasicLandType::Island,
                    BasicLandType::Swamp,
                    BasicLandType::Mountain,
                ]),
            ]),
        ),
        abilities::cycling!(
            "Cycling {U}{B}{R} ({U}{B}{R}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{U}{B}{R}"))],
        ),
    ]),
);

// MH3 226 — Shattered Landscape
pub(in crate::card::sets) static SHATTERED_LANDSCAPE: CardRecord = CardRecord::new(
    "Shattered Landscape",
    "b3da28c7-6e92-439d-a163-91682d4f11dc",
    "Erikas Perl",
    // The Mardu member of the cycle: colourless mana now, a tapped basic
    // later, or a card when the deck has enough colours for the cycling.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        abilities::landscape_fetch(
            "{T}, Sacrifice this land: Search your library for a basic Mountain, Plains, or \
             Swamp card, put it onto the battlefield tapped, then shuffle.",
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::Supertype(CardSupertype::Basic),
                ObjectPredicateDef::HasAnyBasicLandType(&[
                    BasicLandType::Mountain,
                    BasicLandType::Plains,
                    BasicLandType::Swamp,
                ]),
            ]),
        ),
        abilities::cycling!(
            "Cycling {R}{W}{B} ({R}{W}{B}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{R}{W}{B}"))],
        ),
    ]),
);

// MH3 227 — Sheltering Landscape
pub(in crate::card::sets) static SHELTERING_LANDSCAPE: CardRecord = CardRecord::new(
    "Sheltering Landscape",
    "0fe070f4-8877-4280-b8fd-869f3ac34ab6",
    "Erikas Perl",
    // The same bargain as its Temur cousin: a colourless tap nobody wants, a
    // tapped basic when you have the land drop, and a card when you do not.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{T}, Sacrifice this land: Search your library for a basic Mountain, Forest, or Plains \
             card, put it onto the battlefield tapped, then shuffle.",
            &LANDSCAPE_FETCH_COST,
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                // The Naya half of the same cycle, and the same shape: three basics, a
                // tapped land, and a cycling cost nobody pays for the mana.
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Supertype(CardSupertype::Basic),
                    ObjectPredicateDef::HasAnyBasicLandType(&[
                        BasicLandType::Mountain,
                        BasicLandType::Forest,
                        BasicLandType::Plains,
                    ]),
                ]),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: false,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: true,
                attachment: None,
                binding: None,
                then: None,
            },
        ),
        abilities::cycling!(
            "Cycling {R}{G}{W} ({R}{G}{W}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{R}{G}{W}"))],
        ),
    ]),
);

// MH3 228 — Shifting Woodland
pub(in crate::card::sets) static SHIFTING_WOODLAND: CardRecord = CardRecord::new(
    "Shifting Woodland",
    "059164e1-894d-4586-9800-e60d6fbd6eb6",
    "Josu Hernaiz",
    // A Forest that turns into the best thing you have already lost, once
    // the graveyard is deep enough to be worth reading.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped_unless_you_control(
            "This land enters tapped unless you control a Forest.",
            ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
        ),
        AbilityDef::activated_mana(
            "{T}: Add {G}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
        ),
        // No "except it has this ability" clause, unlike Thespian's Stage: the
        // copy replaces every copiable value, so while it is a creature it is
        // not a land, taps for nothing, and cannot do this again.
        AbilityDef::activated_with_targets(
            "Delirium — {2}{G}{G}: This land becomes a copy of target permanent card in your \
             graveyard until end of turn. Activate only if there are four or more card types among \
             cards in your graveyard.",
            &[CostDef::Mana(mana_cost!("{2}{G}{G}"))],
            // "Target permanent card in your graveyard": the five permanent types, in
            // your own graveyard rather than either.
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::BecomeCopyOf {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                copier: None,
                exceptions: CopyExceptionsDef::NONE,
                duration: Some(ResolvedEffectDurationDef::UntilEndOfTurn),
            },
        )
        .with_activation_condition(
            &// Delirium, as an activation restriction rather than a trigger condition:
            // the ability is not offered at all while the graveyard is short of four
            // card types.
            TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                left: ValueDef::CardTypesAmongGraveyards(PlayerRelation::You),
                comparison: ComparisonDef::GreaterOrEqual,
                right: ValueDef::Constant(4),
            }),
        ),
    ]),
);

// MH3 231 — Tranquil Landscape
pub(in crate::card::sets) static TRANQUIL_LANDSCAPE: CardRecord = CardRecord::new(
    "Tranquil Landscape",
    "113f48b9-a972-4e2c-af95-05ab078e01f2",
    "Randy Gallegos",
// The green-white-blue Landscape; only the three types and the cycling
    // cost below are its own.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        abilities::landscape_fetch(
            "{T}, Sacrifice this land: Search your library for a basic Forest, Plains, or Island card, put it onto the battlefield tapped, then shuffle.",
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::Supertype(CardSupertype::Basic),
                ObjectPredicateDef::HasAnyBasicLandType(&[
                    BasicLandType::Forest,
                    BasicLandType::Plains,
                    BasicLandType::Island,
                ]),
            ]),
        ),
        abilities::cycling!(
            "Cycling {G}{W}{U} ({G}{W}{U}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{G}{W}{U}"))],
        ),
    ]),
);

// MH3 232 — Twisted Landscape
pub(in crate::card::sets) static TWISTED_LANDSCAPE: CardRecord = CardRecord::new(
    "Twisted Landscape",
    "d0e3e7b3-7ba9-47a2-b46c-a40bffb445e2",
    "Piotr Dura",
// The land drop it finds is the point; cycling is what it does on the
    // turns the deck already has enough of them.
    CardRules::new_land(&[]).with_abilities(&landscape_abilities(
        "{T}, Sacrifice this land: Search your library for a basic Swamp, Mountain, or Forest card, \
         put it onto the battlefield tapped, then shuffle.",
        // The Jund member: the same land with the other three basics on it.
        ObjectPredicateDef::All(&[
            ObjectPredicateDef::Supertype(CardSupertype::Basic),
            ObjectPredicateDef::HasAnyBasicLandType(&[
                BasicLandType::Swamp,
                BasicLandType::Mountain,
                BasicLandType::Forest,
            ]),
        ]),
        &abilities::cycling!(
            "Cycling {B}{R}{G} ({B}{R}{G}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{B}{R}{G}"))],
        ),
    )),
);

// MH3 233 — Ugin's Labyrinth
// Audit: unsupported — Mana amounts and overrides cannot inspect this source's linked-exile set. CountMatchingObjects accepts zone queries but has no exiled-with-this-source predicate, and CountObjects over LinkedExiles is not evaluated by the mana planner.
pub(in crate::card::sets) static UGIN_S_LABYRINTH_233: CardRecord = CardRecord::new(
    "Ugin's Labyrinth",
    "020e1348-1a35-4cc8-bad6-9fbddfa79277",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// MH3 234 — Urza's Cave
pub(in crate::card::sets) static URZA_S_CAVE_234: CardRecord = CardRecord::new(
    "Urza's Cave",
    "926916ed-2f22-4ba9-9427-194886ad6c1e",
    "Mark Poole",
    CardRules::new_land(&["Urza's", "Cave"]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated("{3}, {T}, Sacrifice this land: Search your library for a land card, put it onto the battlefield tapped, then shuffle.", &[CostDef::Mana(mana_cost!("{3}")), CostDef::TapSource, CostDef::SacrificeSource], EffectDef::SearchZone { player: EffectRecipientDef::Controller, source: ZoneKind::Library, object: ObjectPredicateDef::HasType(CardType::Land), minimum: 0, maximum: ValueDef::Constant(1), reveal: false, destination: ZoneKind::Battlefield, placement: ZonePlacement::Top, shuffle: true, enters_tapped: true, attachment: None, binding: None, then: None }),
    ]),
);

// MH3 237 — Ajani, Nacatl Pariah // Ajani, Nacatl Avenger
const CAT_WARRIOR_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Cat", "Warrior"], &[ManaColor::White], 2, 1).with_art(
        CardArt::new("ce5c5bcf-1fdd-4d73-a92b-223292da00ca", "Ben Wootten"),
    );

pub(in crate::card::sets) static AJANI_NACATL_PARIAH: CardRecord = CardRecord::new_dfc(
    "Ajani, Nacatl Pariah // Ajani, Nacatl Avenger",
    "0d16e8e0-31b2-4389-afd6-783c501f6fa0",
    "Chris Rallis",
    &[
        (
            "Ajani, Nacatl Pariah",
            const {
                CardRules::new_creature(mana_cost!("{1}{W}"), &["Cat", "Warrior"], 1, 2)
                    .with_supertype(CardSupertype::Legendary)
                    .with_abilities(&const { [
                        abilities::enters_trigger(
                            "When Ajani enters, create a 2/1 white Cat Warrior creature token.",
                            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                                CAT_WARRIOR_TOKEN,
                            ))),
                        ),
                        // One trigger per Cat rather than one per batch. Several Cats dying at
                        // once fire it several times, and every firing after the first finds
                        // Ajani already exiled and returned as a new object, so it has nothing
                        // left to turn over.
                        AbilityDef::triggered(
                            "Whenever one or more other Cats you control die, you may exile Ajani, then return him to the battlefield transformed under his owner's control.",
                            TriggerEventDef::zone_changed(
                                // The Cats that matter are the other ones: Ajani dying alongside them does
                                // not turn him over, and neither does his own death.
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Cat")),
                                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                ]),
                                Some(ZoneKind::Battlefield),
                                Some(ZoneKind::Graveyard),
                            ),
                            EffectDef::May {
                                player: EffectRecipientDef::Controller,
                                // "Exile Ajani, then return him to the battlefield transformed." One
                                // resolution: the exile links him to himself and the return brings him
                                // straight back on the other face, under his owner's control.
                                effect: &EffectDef::Sequence(&[
                                    EffectDef::ExileLinkedToSource {
                                        until_source_leaves: false,
                                        object: EffectRecipientDef::Source,
                                        face_down: false,
                                        then: None,
                                    },
                                    EffectDef::ReturnLinkedExiles {
                                        object: ObjectPredicateDef::Any,
                                        counters: None,
                                        zone: ZoneKind::Battlefield,
                                        grant: None,
                                        controller: None,
                                        transformed: true,
                                    },
                                ]),
                            },
                        ),
                    ] })
            },
        ),
        (
            "Ajani, Nacatl Avenger",
            const {
                CardRules::new_planeswalker_without_mana_cost(&["Ajani"])
                    .with_supertype(CardSupertype::Legendary)
                    .with_starting_loyalty(3)
                    .with_abilities(&const { [
                        AbilityDef::activated(
                            "+2: Put a +1/+1 counter on each Cat you control.",
                            &[CostDef::Loyalty(2)],
                            EffectDef::AddCounters {
                                object: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Cat")),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::You,
                                ))),
                                kind: CounterKind::PlusOnePlusOne,
                                amount: ValueDef::Constant(1),
                            },
                        ),
                        AbilityDef::activated_with_targets(
                            "0: Create a 2/1 white Cat Warrior creature token. When you do, if you control a red permanent other than Ajani, he deals damage equal to the number of creatures you control to any target.",
                            &[CostDef::Loyalty(0)],
                            &const { [AbilityTargetDef::exactly_one(
                                AbilityTargetPredicate::AnyTarget,
                            )] },
                            // The reflexive "when you do" is folded into this resolution: the token is
                            // made, and then the damage happens if the condition holds. What that
                            // costs is the separate window between the two and the chance to decline
                            // the damage; the target is named as the ability is activated instead of
                            // after the token appears, and there is always a legal one because a
                            // player is a legal target.
                            EffectDef::Sequence(&const { [
                                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                                    CAT_WARRIOR_TOKEN,
                                ))),
                                EffectDef::IfCondition {
                                    // "If you control a red permanent other than Ajani." Ajani himself is
                                    // white, so the clause is about a second permanent rather than about him.
                                    condition: &TriggerConditionDef::ObjectCount {
                                        query: ObjectQueryDef::matching(
                                            ObjectPredicateDef::All(&[
                                                ObjectPredicateDef::Color(ManaColor::Red),
                                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                            ]),
                                            &[ZoneKind::Battlefield],
                                            PlayerRelation::You,
                                        ),
                                        comparison: ComparisonDef::GreaterOrEqual,
                                        amount: 1,
                                    },
                                    then: &EffectDef::damage(
                                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                        ValueDef::CountMatchingObjects(
                                            &const {
                                                ObjectQueryDef::matching(
                                                    ObjectPredicateDef::HasType(CardType::Creature),
                                                    &[ZoneKind::Battlefield],
                                                    PlayerRelation::You,
                                                )
                                            },
                                        ),
                                    ),
                                },
                            ] }),
                        ),
                        AbilityDef::activated(
                            "−4: Each opponent chooses an artifact, a creature, an enchantment, and a planeswalker from among the nonland permanents they control, then sacrifices the rest.",
                            &[CostDef::Loyalty(-4)],
                            EffectDef::ChooseForEachPlayer(ChooseForEachPlayerDef {
                                player: EffectRecipientDef::Opponent,
                                candidates: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                                zone: ZoneKind::Battlefield,
                                // The four roles the ultimate lets each opponent fill. Order is printed
                                // order, which is also APNAP choice order within one player's selection.
                                selection: PerPlayerSelectionDef::OneOfEach(&[
                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::HasType(CardType::Enchantment),
                                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                                ]),
                                visibility: ChoiceVisibilityDef::Public,
                                chosen: Binding!("ugin_spared_permanents"),
                                unchosen: Binding!("ugin_sacrificed_permanents"),
                                then: &const { EffectDef::sacrifice(
                                    EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!(
                                        "ugin_sacrificed_permanents"
                                    ))),
                                ) },
                            }),
                        ),
                    ] })
            },
        ),
    ],
);

// MH3 238 — Razorgrass Ambush // Razorgrass Field
pub(in crate::card::sets) static RAZORGRASS_AMBUSH_RAZORGRASS_FIELD_238: CardRecord =
    CardRecord::new_mdfc(
        "Razorgrass Ambush // Razorgrass Field",
        "57065dca-f90e-4184-bbc4-95d726a4160b",
        "Cristi Balanescu",
        &[("Razorgrass Ambush", CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets("Razorgrass Ambush deals 3 damage to target attacking or blocking creature.", &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::AttackingOrBlocking]))], EffectDef::damage(EffectRecipientDef::Target(TargetIndex::PRIMARY), ValueDef::Constant(3))))), ("Razorgrass Field", CardRules::new_land(&[]).with_abilities(&[AbilityDef::replacement("As this land enters, you may pay 3 life. If you don't, it enters tapped.", ReplacementEffectDef::PayOr { payment: EffectPaymentDef::new(PlayerSetDef::Related(PlayerRelation::You), &[CostDef::PayLife(3)]), if_paid: &[], if_declined: &[ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped)] }), abilities::tap_for(ManaColor::White)]))],
    );

// MH3 239 — Witch Enchanter // Witch-Blessed Meadow
pub(in crate::card::sets) static WITCH_ENCHANTER: CardRecord = CardRecord::new_mdfc(
    "Witch Enchanter // Witch-Blessed Meadow",
    "62061e7c-cf19-4f03-b8fa-2bdba62d6b0b",
    "Tyler Walpole",
    &[
        (
            "Witch Enchanter",
            const {
                CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Warlock"], 2, 2)
                .with_abilities(&const { [abilities::enters_trigger_with_targets(
                    "When this creature enters, destroy target artifact or enchantment an opponent controls.",
                    // "Target artifact or enchantment an opponent controls": two types and a
                    // controller, which together are the whole restriction.
                    &const { [AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Enchantment),
                            ]),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                        ]),
                    )] },
                    EffectDef::destroy_target(TargetIndex::PRIMARY),
                )] })
            },
        ),
        (
            "Witch-Blessed Meadow",
            const {
                CardRules::new_land(&[]).with_abilities(&const { [
                AbilityDef::replacement(
                    "As this land enters, you may pay 3 life. If you don't, it enters tapped.",
                    ReplacementEffectDef::PayOr {
                        payment: EffectPaymentDef::new(
                            PlayerSetDef::Related(PlayerRelation::You),
                            &[CostDef::PayLife(3)],
                        ),
                        if_paid: &[],
                        // Declining is what taps it, so the paid branch does nothing and the
                        // declined branch is the whole of the cost.
                        if_declined: &[ReplacementEffectDef::ModifyBattlefieldEntry(
                                BattlefieldEntryModificationDef::Tapped,
                            )],
                    },
                ),
                AbilityDef::activated_mana(
                    "{T}: Add {W}.",
                    &[CostDef::TapSource],
                    EffectDef::AddMana(AddManaEffectDef::one(ManaColor::White)),
                ),
            ] })
            },
        ),
    ],
);

// MH3 240 — Hydroelectric Specimen // Hydroelectric Laboratory
pub(in crate::card::sets) static HYDROELECTRIC_SPECIMEN_HYDROELECTRI_240: CardRecord =
    CardRecord::new_mdfc(
        "Hydroelectric Specimen // Hydroelectric Laboratory",
        "8689ecd7-e9a6-458b-99d2-6dbaca527f00",
        "Raoul Vitale",
        &[("Hydroelectric Specimen", CardRules::new_creature(mana_cost!("{2}{U}"), &["Weird"], 1, 4).with_abilities(&[abilities::flash(), abilities::enters_trigger_with_targets("When this creature enters, you may change the target of target instant or sorcery spell with a single target to this creature.", &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::All(&[ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Instant), ObjectPredicateDef::HasType(CardType::Sorcery)]), ObjectPredicateDef::DeclaredTargetCount { minimum: 1, maximum: 1 }]), zones: &[ZoneKind::Stack], controller: None, owner: None })], EffectDef::May { player: EffectRecipientDef::Controller, effect: &EffectDef::ChangeStackTargets(&crate::card::ChangeStackTargetsDef { object: EffectRecipientDef::Target(TargetIndex::PRIMARY), chooser: PlayerRefDef::EffectController, change: crate::card::StackTargetChangeDef::ReplaceOneWith(EffectRecipientDef::Source) }) })])), ("Hydroelectric Laboratory", CardRules::new_land(&[]).with_abilities(&[AbilityDef::replacement("As this land enters, you may pay 3 life. If you don't, it enters tapped.", ReplacementEffectDef::PayOr { payment: EffectPaymentDef::new(PlayerSetDef::Related(PlayerRelation::You), &[CostDef::PayLife(3)]), if_paid: &[], if_declined: &[ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped)] }), abilities::tap_for(ManaColor::Blue)]))],
    );

// MH3 241 — Sink into Stupor // Soporific Springs
pub(in crate::card::sets) static SINK_INTO_STUPOR: CardRecord = CardRecord::new_mdfc(
    "Sink into Stupor // Soporific Springs",
    "5358b87a-1a29-426d-b165-40c97da2c14d",
    "Peter Polach",
    &[
        (
            "Sink into Stupor",
            const {
                CardRules::new_instant(mana_cost!("{1}{U}{U}")).with_ability(AbilityDef::spell_with_targets(
                "Return target spell or nonland permanent an opponent controls to its owner's hand.",
                &const { [AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::AnyOf(&[
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::Spell,
                                zones: &[ZoneKind::Stack],
                                controller: Some(PlayerRelation::Opponent),
                                owner: None,
                            },
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                                zones: &[ZoneKind::Battlefield],
                                controller: Some(PlayerRelation::Opponent),
                                owner: None,
                            },
                        ]),
                    )] },
                // Returning a spell is not countering it: one that cannot be countered is
                // answered all the same, and its controller keeps the card.
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            ))
            },
        ),
        (
            "Soporific Springs",
            const {
                CardRules::new_land(&[]).with_abilities(&const { [
                AbilityDef::replacement(
                    "As this land enters, you may pay 3 life. If you don't, it enters tapped.",
                    ReplacementEffectDef::PayOr {
                        payment: EffectPaymentDef::new(
                            PlayerSetDef::Related(PlayerRelation::You),
                            &[CostDef::PayLife(3)],
                        ),
                        if_paid: &[],
                        if_declined: &[ReplacementEffectDef::ModifyBattlefieldEntry(
                                BattlefieldEntryModificationDef::Tapped,
                            )],
                    },
                ),
                AbilityDef::activated_mana(
                    "{T}: Add {U}.",
                    &[CostDef::TapSource],
                    EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue)),
                ),
            ] })
            },
        ),
    ],
);

// MH3 243 — Boggart Trawler // Boggart Bog
pub(in crate::card::sets) static BOGGART_TRAWLER_BOGGART_BOG_243: CardRecord = CardRecord::new_mdfc(
    "Boggart Trawler // Boggart Bog",
    "d0d484a6-5610-4f1d-95ec-eda273c255e4",
    "Randy Gallegos",
    &[
        (
            "Boggart Trawler",
            CardRules::new_creature(mana_cost!("{2}{B}"), &["Goblin"], 3, 1).with_ability(
                abilities::enters_trigger_with_targets(
                    "When this creature enters, exile target player's graveyard.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Player(PlayerRelation::Any),
                    )],
                    EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::owned_by(
                            ObjectPredicateDef::Any,
                            &[ZoneKind::Graveyard],
                            PlayerSetDef::LegalTargets(TargetIndex::PRIMARY),
                        ))),
                        ZoneKind::Exile,
                        ZonePlacement::Top,
                    ),
                ),
            ),
        ),
        (
            "Boggart Bog",
            CardRules::new_land(&[]).with_abilities(&[
                AbilityDef::replacement(
                    "As this land enters, you may pay 3 life. If you don't, it enters tapped.",
                    ReplacementEffectDef::PayOr {
                        payment: EffectPaymentDef::new(
                            PlayerSetDef::Related(PlayerRelation::You),
                            &[CostDef::PayLife(3)],
                        ),
                        if_paid: &[],
                        if_declined: &[ReplacementEffectDef::ModifyBattlefieldEntry(
                            BattlefieldEntryModificationDef::Tapped,
                        )],
                    },
                ),
                abilities::tap_for(ManaColor::Black),
            ]),
        ),
    ],
);

// MH3 244 — Fell the Profane // Fell Mire
pub(in crate::card::sets) static FELL_THE_PROFANE_FELL_MIRE_244: CardRecord = CardRecord::new_mdfc(
    "Fell the Profane // Fell Mire",
    "a3cb782d-c459-468d-9779-9b5669abc337",
    "Yeong-Hao Han",
    &[
        (
            "Fell the Profane",
            CardRules::new_instant(mana_cost!("{2}{B}{B}")).with_ability(
                AbilityDef::spell_with_targets(
                    "Destroy target creature or planeswalker.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                        ]),
                    )],
                    EffectDef::destroy_target(TargetIndex::PRIMARY),
                ),
            ),
        ),
        (
            "Fell Mire",
            CardRules::new_land(&[]).with_abilities(&[
                AbilityDef::replacement(
                    "As this land enters, you may pay 3 life. If you don't, it enters tapped.",
                    ReplacementEffectDef::PayOr {
                        payment: EffectPaymentDef::new(
                            PlayerSetDef::Related(PlayerRelation::You),
                            &[CostDef::PayLife(3)],
                        ),
                        if_paid: &[],
                        if_declined: &[ReplacementEffectDef::ModifyBattlefieldEntry(
                            BattlefieldEntryModificationDef::Tapped,
                        )],
                    },
                ),
                abilities::tap_for(ManaColor::Black),
            ]),
        ),
    ],
);

// MH3 246 — Pinnacle Monk // Mystic Peak
pub(in crate::card::sets) static PINNACLE_MONK_MYSTIC_PEAK_246: CardRecord = CardRecord::new_mdfc(
    "Pinnacle Monk // Mystic Peak",
    "24d4f26e-7f96-4b38-867e-4fac819b2679",
    "Jason A. Engle",
    &[("Pinnacle Monk", CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Djinn", "Monk"], 2, 2).with_abilities(&[abilities::prowess(), abilities::enters_trigger_with_targets("When this creature enters, return target instant or sorcery card from your graveyard to your hand.", &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Instant), ObjectPredicateDef::HasType(CardType::Sorcery)]), zones: &[ZoneKind::Graveyard], controller: None, owner: Some(PlayerRelation::You) })], EffectDef::move_to_zone(EffectRecipientDef::Target(TargetIndex::PRIMARY), ZoneKind::Hand, ZonePlacement::Top))])), ("Mystic Peak", CardRules::new_land(&[]).with_abilities(&[AbilityDef::replacement("As this land enters, you may pay 3 life. If you don't, it enters tapped.", ReplacementEffectDef::PayOr { payment: EffectPaymentDef::new(PlayerSetDef::Related(PlayerRelation::You), &[CostDef::PayLife(3)]), if_paid: &[], if_declined: &[ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped)] }), abilities::tap_for(ManaColor::Red)]))],
);

// MH3 248 — Sundering Eruption // Volcanic Fissure
pub(in crate::card::sets) static SUNDERING_ERUPTION_VOLCANIC_FISSURE_248: CardRecord =
    CardRecord::new_mdfc(
        "Sundering Eruption // Volcanic Fissure",
        "50686ac7-346c-43d1-bdaa-28d46a12ad93",
        "Yohann Schepacz",
        &[("Sundering Eruption", CardRules::new_sorcery(mana_cost!("{2}{R}")).with_ability(AbilityDef::spell_with_targets("Destroy target land. Its controller may search their library for a basic land card, put it onto the battlefield tapped, then shuffle. Creatures without flying can't block this turn.", &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Land))], EffectDef::Sequence(&[EffectDef::Destroy { object: EffectRecipientDef::Target(TargetIndex::PRIMARY), then: None }, EffectDef::May { player: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY), effect: &EffectDef::SearchZone { player: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY), source: ZoneKind::Library, object: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Land), ObjectPredicateDef::Supertype(CardSupertype::Basic)]), minimum: 0, maximum: ValueDef::Constant(1), reveal: false, destination: ZoneKind::Battlefield, placement: ZonePlacement::Top, shuffle: true, enters_tapped: true, attachment: None, binding: None, then: None } }, EffectDef::Apply { recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(KeywordAbility::Flying))]), &[ZoneKind::Battlefield], PlayerRelation::Any), effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK), duration: ResolvedEffectDurationDef::UntilEndOfTurn }])))), ("Volcanic Fissure", CardRules::new_land(&[]).with_abilities(&[AbilityDef::replacement("As this land enters, you may pay 3 life. If you don't, it enters tapped.", ReplacementEffectDef::PayOr { payment: EffectPaymentDef::new(PlayerSetDef::Related(PlayerRelation::You), &[CostDef::PayLife(3)]), if_paid: &[], if_declined: &[ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped)] }), abilities::tap_for(ManaColor::Red)]))],
    );

// MH3 249 — Bridgeworks Battle // Tanglespan Bridgeworks
pub(in crate::card::sets) static BRIDGEWORKS_BATTLE_TANGLESPAN_BRIDGEWORKS_249: CardRecord =
    CardRecord::new_mdfc(
        "Bridgeworks Battle // Tanglespan Bridgeworks",
        "ebef3db0-2b58-4581-a79c-fbca9a059e63",
        "Ron Spears",
        &[("Bridgeworks Battle", CardRules::new_sorcery(mana_cost!("{2}{G}")).with_ability(AbilityDef::spell_with_targets("Target creature you control gets +2/+2 until end of turn. It fights up to one target creature you don't control.", &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::HasType(CardType::Creature), zones: &[ZoneKind::Battlefield], controller: Some(PlayerRelation::You), owner: None }), AbilityTargetDef::up_to(AbilityTargetPredicate::Object { object: ObjectPredicateDef::HasType(CardType::Creature), zones: &[ZoneKind::Battlefield], controller: Some(PlayerRelation::Opponent), owner: None }, 1)], EffectDef::Sequence(&[EffectDef::Apply { recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY), effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)), duration: ResolvedEffectDurationDef::UntilEndOfTurn }, EffectDef::Fight { first: ObjectRefDef::Target(TargetIndex::PRIMARY), second: ObjectRefDef::Target(TargetIndex(1)), excess: None }])))), ("Tanglespan Bridgeworks", CardRules::new_land(&[]).with_abilities(&[AbilityDef::replacement("As this land enters, you may pay 3 life. If you don't, it enters tapped.", ReplacementEffectDef::PayOr { payment: EffectPaymentDef::new(PlayerSetDef::Related(PlayerRelation::You), &[CostDef::PayLife(3)]), if_paid: &[], if_declined: &[ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped)] }), abilities::tap_for(ManaColor::Green)]))],
    );

// MH3 250 — Disciple of Freyalise // Garden of Freyalise
pub(in crate::card::sets) static DISCIPLE_OF_FREYALISE_GARDEN_OF_FREYALISE_250: CardRecord =
    CardRecord::new_mdfc(
        "Disciple of Freyalise // Garden of Freyalise",
        "a8e9ea5a-5e10-4b77-baef-0352ff035483",
        "Valera Lutfullina",
        &[("Disciple of Freyalise", CardRules::new_creature(mana_cost!("{3}{G}{G}{G}"), &["Elf", "Druid"], 3, 3).with_ability(abilities::enters_trigger("When this creature enters, you may sacrifice another creature. If you do, you gain X life and draw X cards, where X is that creature's power.", EffectDef::SacrificeOfChoice { player: EffectRecipientDef::Controller, object: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Not(&ObjectPredicateDef::Source)]), count: ValueDef::Constant(1), then: Some(&EffectDef::Sequence(&[EffectDef::GainLife { recipient: EffectRecipientDef::Controller, amount: ValueDef::TriggerEventAmount }, EffectDef::DrawCards { recipient: EffectRecipientDef::Controller, amount: ValueDef::TriggerEventAmount }])), amount: SacrificedAmountDef::Power, otherwise: None, optional: true }))), ("Garden of Freyalise", CardRules::new_land(&[]).with_abilities(&[AbilityDef::replacement("As this land enters, you may pay 3 life. If you don't, it enters tapped.", ReplacementEffectDef::PayOr { payment: EffectPaymentDef::new(PlayerSetDef::Related(PlayerRelation::You), &[CostDef::PayLife(3)]), if_paid: &[], if_declined: &[ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped)] }), abilities::tap_for(ManaColor::Green)]))],
    );

// MH3 261 — Waterlogged Teachings // Inundated Archive
pub(in crate::card::sets) static WATERLOGGED_TEACHINGS_INUNDATED_ARCHIVE_261: CardRecord =
    CardRecord::new_mdfc(
        "Waterlogged Teachings // Inundated Archive",
        "060f9675-4921-4cbb-bae2-54c85c679fd4",
        "Douglas Shuler",
        &[("Waterlogged Teachings", CardRules::new_instant(mana_cost!("{3}{U/B}")).with_ability(AbilityDef::spell("Search your library for an instant card or a card with flash, reveal it, put it into your hand, then shuffle.", EffectDef::SearchZone { player: EffectRecipientDef::Controller, source: ZoneKind::Library, object: ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Instant), ObjectPredicateDef::HasKeyword(KeywordAbility::Flash)]), minimum: 0, maximum: ValueDef::Constant(1), reveal: true, destination: ZoneKind::Hand, placement: ZonePlacement::Top, shuffle: true, enters_tapped: false, attachment: None, binding: None, then: None }))), ("Inundated Archive", CardRules::new_land(&[]).with_abilities(&[abilities::enters_tapped(CardType::Land), AbilityDef::activated_mana("{T}: Add {U} or {B}.", &[CostDef::TapSource], EffectDef::AddMana(AddManaEffectDef::choice(&[ManaColor::Blue, ManaColor::Black])))]))],
    );

// MH3 284 — Annoyed Altisaur (reprint)
const ANNOYED_ALTISAUR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2020::commander_legends::ANNOYED_ALTISAUR,
    "4aa9354d-3496-47f4-81c9-aead15efb8bb",
    "Lars Grant-West",
);

// MH3 286 — Priest of Titania (reprint)
const PRIEST_OF_TITANIA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1998::urzas_saga::PRIEST_OF_TITANIA,
    "eb11921b-1b28-483f-a707-4de21a6daa31",
    "Rebecca Guay",
);

// MH3 320 — Echoes of Eternity
// Audit: unsupported — AdditionalTriggerDef only doubles triggers caused by battlefield entry. It cannot double arbitrary triggers of colorless spells or permanents, including spell-cast and upkeep triggers.
pub(in crate::card::sets) static ECHOES_OF_ETERNITY_320: CardRecord = CardRecord::new(
    "Echoes of Eternity",
    "ae70f03f-cf60-418b-98e3-bc868e739656",
    "Clint Lockwood",
    crate::card::CardRules::unsupported(),
);

// MH3 334 — Party Thrasher
// Audit: unsupported — The engine does not grant convoke to spells conditionally on their being cast from exile. The cast-cost scanner handles intrinsic payment keywords, not this zone-dependent continuous grant.
pub(in crate::card::sets) static PARTY_THRASHER_334: CardRecord = CardRecord::new(
    "Party Thrasher",
    "52bb8272-e60f-4aa1-8f98-6110715a78fa",
    "Ina Wong",
    crate::card::CardRules::unsupported(),
);

// MH3 335 — Powerbalance
// Audit: unsupported — Immediate free-cast offers accept only graveyard or exile cards. They cannot cast the revealed top card directly from the library during this trigger; exiling it first changes the printed behavior.
pub(in crate::card::sets) static POWERBALANCE_335: CardRecord = CardRecord::new(
    "Powerbalance",
    "8a64a5c4-ebae-472b-8f90-dcdd8ab8bc26",
    "Leanna Crossan",
    crate::card::CardRules::unsupported(),
);

// MH3 350 — Archway of Innovation
// Audit: unsupported — There is no consumable permission granting improvise to only the next spell cast this turn. The next-cast lifetime currently applies to cast-timing permissions, not payment keywords.
pub(in crate::card::sets) static ARCHWAY_OF_INNOVATION_350: CardRecord = CardRecord::new(
    "Archway of Innovation",
    "472905ac-1eb9-4951-8180-b8c35fbab3d7",
    "Sam Burley",
    crate::card::CardRules::unsupported(),
);

// MH3 351 — Arena of Glory
pub(in crate::card::sets) static ARENA_OF_GLORY: CardRecord = CardRecord::new(
    "Arena of Glory",
    "3d7d07bb-b875-4a6d-8b87-4187e823af75",
    "Piotr Dura",
    // A red source that costs nothing to play and turns one creature a game
    // into a surprise, which is what a haste land is for.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::check_land_enters(
            "This land enters tapped unless you control a Mountain.",
            &[BasicLandType::Mountain],
        ),
        AbilityDef::activated_mana(
            "{T}: Add {R}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
        ),
        AbilityDef::activated_mana(
            "{R}, {T}, Exert this land: Add {R}{R}. If that mana is spent on a creature spell, it \
             gains haste until end of turn.",
            // {R} in, {R}{R} out, and one untap step owed: the land pays for the haste
            // out of next turn rather than out of this one.
            &[
                CostDef::Mana(mana_cost!("{R}")),
                CostDef::TapSource,
                CostDef::ExertSource,
            ],
            EffectDef::AddMana(
                AddManaEffectDef::one(ManaColor::Red)
                    .with_amount(2)
                    // The rider asks what the mana paid for rather than restricting what it may
                    // pay for: this mana casts anything, and only a creature gets anything out
                    // of it.
                    .with_spend_effects(&[ManaSpendEffectDef::ApplyToPaidSpellMatching {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        effect: AppliedEffectDef::add_ability(&abilities::haste()),
                    }]),
            ),
        ),
    ]),
);

// MH3 377 — Nadu, Winged Wisdom
/// One card off the top, sorted by whether it is a land: the land goes to
/// the battlefield and anything else goes to the hand, so nothing is left
/// for the player to decide.
const NADU_LAND: Binding = Binding!("nadu_land");
const NADU_NONLAND: Binding = Binding!("nadu_nonland");
pub(in crate::card::sets) static NADU_WINGED_WISDOM: CardRecord = CardRecord::new(
    "Nadu, Winged Wisdom",
    "8281df8a-2fde-454a-813c-d9f86bb35d36",
    "Gossip Goblin",
// Three mana for a 3/4 flier that turns every targeting spell you own
    // into a card, twice per creature per turn -- and Nadu is a creature you
    // control, so pointing something at him counts too.
    CardRules::new_creature(mana_cost!("{1}{G}{U}"), &["Bird", "Wizard"], 3, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::static_ability(
                "Creatures you control have \"Whenever this creature becomes the target of a spell or \
                 ability, reveal the top card of your library. If it's a land card, put it onto the \
                 battlefield. Otherwise, put it into your hand. This ability triggers only twice each \
                 turn.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    // The granted ability, carried by each creature rather than by Nadu: the
                    // cap is on one creature's copy of it, so every creature you control has
                    // two of these a turn.
                    effect: AppliedEffectDef::add_ability(&AbilityDef::triggered(
                        "Whenever this creature becomes the target of a spell or ability, reveal the top card of \
                         your library. If it's a land card, put it onto the battlefield. Otherwise, put it into your \
                         hand. This ability triggers only twice each turn.",
                        TriggerEventDef::becomes_targeted(
                            ObjectPredicateDef::Any,
                        ),
                        abilities::bind_top_cards_then(
                            PlayerRefDef::EffectController,
                            ValueDef::Constant(1),
                            &EffectDef::Sequence(&[
                                    EffectDef::RevealObjects(RevealObjectsDef {
                                        input: ObjectSetDef::Binding(ParentBinding),
                                        then: &EffectDef::None,
                                    }),
                                    EffectDef::ClassifyObjects(ClassifyObjectsDef {
                                            input: ObjectSetDef::Binding(ParentBinding),
                                            object: ObjectPredicateDef::HasType(CardType::Land),
                                            matching: NADU_LAND,
                                            remainder: NADU_NONLAND,
                                            then: &EffectDef::Sequence(&[
                                                    EffectDef::MoveObjects(MoveObjectsDef {
                                                        input: ObjectSetDef::Binding(NADU_LAND),
                                                        from: Some(ZoneKind::Library),
                                                        zone: ZoneKind::Battlefield,
                                                        placement: ZonePlacement::Top,
                                                        moved: None,
                                                        then: &EffectDef::None,
                                                    }),
                                                    EffectDef::MoveObjects(MoveObjectsDef {
                                                            input: ObjectSetDef::Binding(
                                                                NADU_NONLAND,
                                                            ),
                                                            from: Some(ZoneKind::Library),
                                                            zone: ZoneKind::Hand,
                                                            placement: ZonePlacement::Top,
                                                            moved: None,
                                                            then: &EffectDef::None,
                                                    }),
                                                ]),
                                    }),
                                ]),
                        ),
                    )
                    .triggering_at_most(2)),
                },
            ),
        ]),
);

// MH3 383 — Ulamog, the Defiler
// Audit: unsupported — The entry-value interpreter cannot compute AggregateObjectValues over exiled cards. Its greatest-exiled-mana-value entry counter count cannot be supplied to AddCountersValue.
pub(in crate::card::sets) static ULAMOG_THE_DEFILER_383: CardRecord = CardRecord::new(
    "Ulamog, the Defiler",
    "339f83ca-4f46-4246-be23-5ca4add31d81",
    "Vincent Proce",
    crate::card::CardRules::unsupported(),
);

// MH3 387 — Null Elemental Blast
pub(in crate::card::sets) static NULL_ELEMENTAL_BLAST_387: CardRecord = CardRecord::new(
    "Null Elemental Blast",
    "7114c9c8-5370-42e0-8aaf-dc05e2422a76",
    "Milivoj Ćeran",
    CardRules::new_instant(mana_cost!("{C}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Counter target multicolored spell.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Spell,
                            ObjectPredicateDef::Not(&ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::ColorCount(0),
                                ObjectPredicateDef::ColorCount(1),
                            ])),
                        ]),
                        zones: &[ZoneKind::Stack],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::counter_target(TargetIndex::PRIMARY),
            ),
            AbilityDef::spell_with_targets(
                "Destroy target multicolored permanent.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::ColorCount(0),
                        ObjectPredicateDef::ColorCount(1),
                    ])),
                )],
                EffectDef::destroy_target(TargetIndex::PRIMARY),
            ),
        ],
    )),
);

// MH3 400 — Flare of Denial
pub(in crate::card::sets) static FLARE_OF_DENIAL_400: CardRecord = CardRecord::new(
    "Flare of Denial",
    "0149c119-83ea-46f5-9e22-33a674ddddb6",
    "Jason A. Engle",
    CardRules::new_instant(mana_cost!("{1}{U}{U}")).with_abilities(&[
AbilityDef::alternative_cast(&[CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Not(&ObjectPredicateDef::Token), ObjectPredicateDef::Color(ManaColor::Blue)]))], AlternativeCastKindDef::AlternativeCost, Some("You may sacrifice a nontoken blue creature rather than pay this spell's mana cost."), EffectDef::None),
AbilityDef::spell_with_targets("Counter target spell.", &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::All(&[ObjectPredicateDef::Spell, ObjectPredicateDef::Any]), zones: &[ZoneKind::Stack], controller: None, owner: None })], EffectDef::counter_target(TargetIndex::PRIMARY))
]),
);

// MH3 409 — Grim Servant
pub(in crate::card::sets) static GRIM_SERVANT_409: CardRecord = CardRecord::new(
    "Grim Servant",
    "77251806-c2b6-448c-a95e-a1943ca0bfd8",
    "David Astruga",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Zombie", "Warlock"], 3, 2).with_abilities(&[
abilities::menace(),
abilities::enters_trigger("When this creature enters, search your library for a card with mana value less than or equal to your devotion to black, reveal it, put it into your hand, then shuffle. You lose 3 life. (Each {B} in the mana costs of permanents you control counts toward your devotion to black.)", EffectDef::Sequence(&[EffectDef::SearchZone { player: EffectRecipientDef::Controller, source: ZoneKind::Library, object: ObjectPredicateDef::ManaValueAtMostValue(ValueDef::DevotionTo(ManaColor::Black)), minimum: 0, maximum: ValueDef::Constant(1), reveal: true, destination: ZoneKind::Hand, placement: ZonePlacement::Top, shuffle: true, enters_tapped: false, attachment: None, binding: None, then: None }, EffectDef::LoseLife { recipient: EffectRecipientDef::Controller, amount: ValueDef::Constant(3) }]))
]),
);

// MH3 410 — Marionette Apprentice
pub(in crate::card::sets) static MARIONETTE_APPRENTICE_410: CardRecord = CardRecord::new(
    "Marionette Apprentice",
    "22b5a3dd-0b5a-434e-afee-a83b0279fd15",
    "Steve Ellis",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Artificer"], 1, 2).with_abilities(&[
abilities::enters_trigger("Fabricate 1 (When this creature enters, put a +1/+1 counter on it or create a 1/1 colorless Servo artifact creature token.)", EffectDef::ChooseEffect { player: EffectRecipientDef::Controller, choices: &[EffectChoiceDef { label: "Put a +1/+1 counter on this creature", effect: EffectDef::AddCounters { object: EffectRecipientDef::Source, kind: CounterKind::PlusOnePlusOne, amount: ValueDef::Constant(1) } }, EffectChoiceDef { label: "Create a Servo", effect: EffectDef::CreateToken(crate::card::CreateTokenDef::new(crate::card::TokenDef::Literal(crate::card::TokenCharacteristics::artifact_creature(&["Servo"], &[], 1, 1)))) }] }),
AbilityDef::triggered("Whenever another creature or artifact you control is put into a graveyard from the battlefield, each opponent loses 1 life.", TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::HasType(CardType::Artifact)]), ObjectPredicateDef::Not(&ObjectPredicateDef::Source), ObjectPredicateDef::ControlledBy(PlayerRelation::You)]), Some(ZoneKind::Battlefield), Some(ZoneKind::Graveyard)), EffectDef::LoseLife { recipient: EffectRecipientDef::Opponent, amount: ValueDef::Constant(1) })
]),
);

// MH3 411 — Necrodominance
// Audit: unsupported — Player rules can modify maximum hand size or remove the limit, but cannot set the base maximum to five. A minus-two modifier is not equivalent when another effect sets that maximum.
pub(in crate::card::sets) static NECRODOMINANCE_411: CardRecord = CardRecord::new(
    "Necrodominance",
    "f810a2d7-efbe-4ea1-83d1-d594a8eaf88b",
    "Robin Olausson",
    crate::card::CardRules::unsupported(),
);

// MH3 414 — Warren Soultrader
pub(in crate::card::sets) static WARREN_SOULTRADER_414: CardRecord = CardRecord::new(
    "Warren Soultrader",
    "17fd4d15-413f-41c5-b3e0-71bbb52851bc",
    "Pete Venters",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Zombie", "Goblin", "Wizard"], 3, 3).with_abilities(&[
AbilityDef::activated("Pay 1 life, Sacrifice another creature: Create a Treasure token. (It's an artifact with \"{T}, Sacrifice this token: Add one mana of any color.\")", &[CostDef::PayLife(1), CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Not(&ObjectPredicateDef::Source)]))], EffectDef::CreateToken(crate::card::CreateTokenDef::new(crate::card::TokenDef::Literal(crate::card::tokens::treasure()))))
]),
);

// MH3 416 — Flare of Duplication
pub(in crate::card::sets) static FLARE_OF_DUPLICATION_416: CardRecord = CardRecord::new(
    "Flare of Duplication",
    "170483c2-4e50-4cc8-9481-3330057d91bb",
    "Olivier Bernard",
    CardRules::new_instant(mana_cost!("{1}{R}{R}")).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                ObjectPredicateDef::Color(ManaColor::Red),
            ]))],
            AlternativeCastKindDef::AlternativeCost,
            Some(
                "You may sacrifice a nontoken red creature rather than pay this spell's mana cost.",
            ),
            EffectDef::None,
        ),
        AbilityDef::spell_with_targets(
            "Copy target instant or sorcery spell. You may choose new targets for the copy.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Spell,
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Instant),
                            ObjectPredicateDef::HasType(CardType::Sorcery),
                        ]),
                    ]),
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::CopyStackObject(&CopyStackObjectDef {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                controller: PlayerRefDef::EffectController,
                count: ValueDef::Constant(1),
                retarget: true,
                colors: None,
            }),
        ),
    ]),
);

// MH3 421 — Unstable Amulet
// Audit: unsupported — The exile-play permission has no lifetime ending when this same permanent next exiles a card. This-turn and while-exiled permissions both authorize the wrong plays.
pub(in crate::card::sets) static UNSTABLE_AMULET_421: CardRecord = CardRecord::new(
    "Unstable Amulet",
    "25f01df9-c4fa-4598-84c1-217bde6b1841",
    "José Parodi",
    crate::card::CardRules::unsupported(),
);

// MH3 427 — Monstrous Vortex
// Audit: unsupported — There is no discover procedure. Cascade uses a strict mana-value bound and lacks discover's choice to put the matched card into hand, so substituting cascade changes the card.
pub(in crate::card::sets) static MONSTROUS_VORTEX_427: CardRecord = CardRecord::new(
    "Monstrous Vortex",
    "0970efb6-427f-4d5b-9b66-eda4b91015bd",
    "Deruchenko Alexander",
    crate::card::CardRules::unsupported(),
);

// MH3 443 — Tamiyo, Inquisitive Student // Tamiyo, Seasoned Scholar
pub(in crate::card::sets) static TAMIYO_INQUISITIVE_STUDENT: CardRecord = CardRecord::new_dfc(
    "Tamiyo, Inquisitive Student // Tamiyo, Seasoned Scholar",
    "1b234fee-a2b6-4661-9f98-4da6fc26aebc",
    "Evyn Fong",
    &[
        (
            "Tamiyo, Inquisitive Student",
            const {
                CardRules::new_creature(mana_cost!("{U}"), &["Moonfolk", "Wizard"], 0, 3)
                    .with_supertype(CardSupertype::Legendary)
                    .with_abilities(&const { [
                        abilities::flying(),
                        AbilityDef::triggered(
                            "Whenever Tamiyo attacks, investigate. (Create a Clue token. It's an artifact with \"{2}, \
                             Sacrifice this token: Draw a card.\")",
                            TriggerEventDef::Attacks(AttackEventMatcherDef::any(ObjectPredicateDef::Source)),
                            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(CLUE_TOKEN))),
                        ),
                        // The third card of the turn, counted over the whole turn rather than
                        // any one step: her own attack Clue and the draw step are usually two
                        // of the three.
                        AbilityDef::triggered(
                            "When you draw your third card in a turn, exile Tamiyo, then return her to the \
                             battlefield transformed under her owner's control.",
                            TriggerEventDef::DrewCard(DrawEventMatcherDef::nth_each_turn(PlayerRelation::You, 3)),
                            // Exile and return, which is how a permanent turns over into a new object
                            // rather than merely flipping: the Tamiyo that comes back has no counters,
                            // no summoning history, and a fresh set of loyalty.
                            EffectDef::Sequence(&[
                                EffectDef::ExileLinkedToSource {
                                    until_source_leaves: false,
                                    object: EffectRecipientDef::Source,
                                    face_down: false,
                                    then: None,
                                },
                                EffectDef::ReturnLinkedExiles {
                                    object: ObjectPredicateDef::Any,
                                    counters: None,
                                    zone: ZoneKind::Battlefield,
                                    grant: None,
                                    controller: None,
                                    transformed: true,
                                },
                            ]),
                        ),
                    ] })
            },
        ),
        (
            "Tamiyo, Seasoned Scholar",
            const {
                CardRules::new_planeswalker_without_mana_cost(&["Tamiyo"])
                .with_supertype(CardSupertype::Legendary)
                // The back face has no mana cost to read a colour off, and
                // prints a colour indicator instead: she is blue on both
                // sides.
                .printed_colors(&[ManaColor::Blue])
                .with_starting_loyalty(2)
                .with_abilities(&const { [
                    AbilityDef::activated(
                        "+2: Until your next turn, whenever a creature attacks you or a planeswalker you \
                         control, it gets -1/-0 until end of turn.",
                        &[CostDef::Loyalty(2)],
                        EffectDef::InstallTrigger(InstalledTriggerDef {
                            // The attackers her plus ability shrinks. It is installed on resolution and
                            // watches until her controller's next turn, so it catches the attack it was
                            // played to blunt.
                            ability: &const { AbilityDef::triggered(
                                "Whenever a creature attacks you or a planeswalker you control, it gets -1/-0 until end of \
                                 turn.",
                                TriggerEventDef::Attacks(AttackEventMatcherDef::attacking(
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    PlayerRelation::You,
                                )),
                                EffectDef::Apply {
                                    recipient: EffectRecipientDef::TriggeringObject,
                                    effect: AppliedEffectDef::modify_power_toughness(
                                        ValueDef::Constant(-1),
                                        ValueDef::Constant(0),
                                    ),
                                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                                },
                            ) },
                            lifetime: InstalledTriggerLifetimeDef::UntilNextTurn(PlayerRefDef::EffectController),
                        }),
                    ),
                    AbilityDef::activated_with_targets(
                        "−3: Return target instant or sorcery card from your graveyard to your hand. If it's a \
                         green card, add one mana of any color.",
                        &[CostDef::Loyalty(-3)],
                        &const { [AbilityTargetDef::exactly_one(
                                AbilityTargetPredicate::Object {
                                    object: ObjectPredicateDef::AnyOf(&[
                                        ObjectPredicateDef::HasType(CardType::Instant),
                                        ObjectPredicateDef::HasType(CardType::Sorcery),
                                    ]),
                                    zones: &[ZoneKind::Graveyard],
                                    controller: None,
                                    owner: Some(PlayerRelation::You),
                                },
                            )] },
                        EffectDef::Sequence(&const { [
                            EffectDef::move_to_zone(
                                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                ZoneKind::Hand,
                                ZonePlacement::Top,
                            ),
                            EffectDef::AddMana(
                                AddManaEffectDef::any_color()
                                    // "If it's a green card, add one mana of any color." One mana when the card
                                    // returned was green and none otherwise, which is the whole rider: an
                                    // amount rather than a branch, read off the target the clause already has.
                                    .with_variable_amount(ValueDef::IfTargetMatches(&TargetConditionDef {
                                        slot: TargetIndex::PRIMARY,
                                        object: ObjectPredicateDef::Color(ManaColor::Green),
                                        then: ValueDef::Constant(1),
                                        otherwise: ValueDef::Constant(0),
                                    })),
                            ),
                        ] }),
                    ),
                    AbilityDef::activated(
                        "−7: Draw cards equal to half the number of cards in your library, rounded up. You get \
                         an emblem with \"You have no maximum hand size.\"",
                        &[CostDef::Loyalty(-7)],
                        EffectDef::Sequence(&const { [
                            EffectDef::DrawCards {
                                recipient: EffectRecipientDef::Controller,
                                amount: ValueDef::Halved(&HalvedValueDef::new(ValueDef::LibrarySize(PlayerRelation::You), RoundingDef::Up)),
                            },
                            EffectDef::CreateEmblem {
                                emblem: EmblemCharacteristics::new("Tamiyo, Seasoned Scholar emblem", &const { [AbilityDef::static_ability(
                                        "You have no maximum hand size.",
                                        EffectDef::StaticApply {
                                            recipient: EffectRecipientDef::Controller,
                                            effect: AppliedEffectDef::Rule(AppliedRuleDef::PlayerRule(
                                                crate::card::PlayerRuleDef::NoMaximumHandSize,
                                            )),
                                        },
                                    )] }),
                            },
                        ] }),
                    ),
                ] })
            },
        ),
    ],
);

// MH3 444 — Sorin of House Markov // Sorin, Ravenous Neonate
pub(in crate::card::sets) static SORIN_OF_HOUSE_MARKOV: CardRecord = CardRecord::new_dfc(
    "Sorin of House Markov // Sorin, Ravenous Neonate",
    "0347bf13-1ccb-4d4d-a5f2-68181d494b85",
    "Livia Prima",
    &[
        (
            "Sorin of House Markov",
            const {
                CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Noble"], 1, 4)
                .with_supertype(CardSupertype::Legendary)
                .with_abilities(&const { [
                    abilities::lifelink(),
                    abilities::extort(),
                    AbilityDef::triggered_if(
                        "At the beginning of each of your postcombat main phases, if you gained 3 or more life \
                         this turn, exile Sorin, then return him to the battlefield transformed under his \
                         owner's control.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::PostcombatMain,
                            player: PlayerRelation::You,
                        },
                        &// Three life in a turn, counted as a running total: gaining it and losing
                            // it again still turns him over, because what the clause reads is the
                            // gaining rather than where the life total ended up.
                            TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                                left: ValueDef::LifeGainedThisTurn(PlayerRelation::You),
                                comparison: ComparisonDef::GreaterOrEqual,
                                right: ValueDef::Constant(3),
                            }),
                        // The same exile-and-return Ajani uses: one resolution, so he is gone and
                        // back before anything else happens, and he comes back a new object with
                        // his printed loyalty.
                        EffectDef::Sequence(&[
                            EffectDef::ExileLinkedToSource {
                                until_source_leaves: false,
                                object: EffectRecipientDef::Source,
                                face_down: false,
                                then: None,
                            },
                            EffectDef::ReturnLinkedExiles {
                                object: ObjectPredicateDef::Any,
                                counters: None,
                                zone: ZoneKind::Battlefield,
                                grant: None,
                                controller: None,
                                transformed: true,
                            },
                        ]),
                    ),
                ] })
            },
        ),
        (
            "Sorin, Ravenous Neonate",
            const {
                // The back face has no mana cost, so its colours come from the printed
                // indicator. They matter to his own ultimate: he is a white permanent,
                // and the clause has to say "other than Sorin" precisely because of it.
                CardRules::new_planeswalker_without_mana_cost(&["Sorin"])
                .with_supertype(CardSupertype::Legendary)
                .with_starting_loyalty(3)
                .printed_colors(&[ManaColor::White, ManaColor::Black])
                .with_abilities(&const { [
                    abilities::extort(),
                    AbilityDef::activated(
                        "+2: Create a Food token.",
                        &[CostDef::Loyalty(2)],
                        EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(FOOD_TOKEN))),
                    ),
                    // The same tally the front face reads to turn over, spent here as
                    // damage: the lifelink body he arrived as is what loads this.
                    AbilityDef::activated_with_targets(
                        "\u{2212}1: Sorin deals damage equal to the amount of life you gained this turn to any \
                         target.",
                        &[CostDef::Loyalty(-1)],
                        &const { [AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::AnyTarget,
                        )] },
                        EffectDef::damage(
                            EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            ValueDef::LifeGainedThisTurn(PlayerRelation::You),
                        ),
                    ),
                    AbilityDef::activated_with_targets(
                        "\u{2212}6: Gain control of target creature. It becomes a Vampire in addition to its \
                         other types. Put a lifelink counter on it if you control a white permanent other than \
                         that creature or Sorin.",
                        &[CostDef::Loyalty(-6)],
                        &const { [AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::HasType(CardType::Creature),
                        )] },
                        EffectDef::Sequence(&const { [
                            EffectDef::gain_control(
                                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                PlayerRefDef::EffectController,
                                ControlDurationDef::Indefinitely,
                            ),
                            EffectDef::Apply {
                                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                // "It becomes a Vampire in addition to its other types": added rather than
                                // set, and with no duration, so what it was stays and the Vampire sticks.
                                effect: AppliedEffectDef::Characteristic(CharacteristicOperationDef::CreatureTypes(
                                        SetOperationDef::Add(CreatureTypeSetDef::named(&["Vampire"])),
                                    )),
                                duration: ResolvedEffectDurationDef::Permanent,
                            },
                            EffectDef::IfCondition {
                                // "A white permanent other than that creature or Sorin." The source is
                                // Sorin; the creature is the one this ability just took, which is why the
                                // query has to leave the target out rather than merely counting what you
                                // control.
                                condition: &TriggerConditionDef::ObjectCount {
                                    query: ObjectQueryDef::matching(
                                        ObjectPredicateDef::All(&[
                                            ObjectPredicateDef::Color(ManaColor::White),
                                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                        ]),
                                        &[ZoneKind::Battlefield],
                                        PlayerRelation::You,
                                    )
                                    .excluding_target(TargetIndex::PRIMARY),
                                    comparison: ComparisonDef::GreaterOrEqual,
                                    amount: 1,
                                },
                                then: &EffectDef::AddCounters {
                                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                    kind: CounterKind::Lifelink,
                                    amount: ValueDef::Constant(1),
                                },
                            },
                        ] }),
                    ),
                ] })
            },
        ),
    ],
);

// MH3 445 — Ral, Monsoon Mage // Ral, Leyline Prodigy
// Audit: unsupported — The back face's +1 must install a temporary instant/sorcery cost reduction until your next turn. ModifyCost is a static operation; the resolving-effect and ongoing-effect procedures cannot install that duration-scoped static discount.
pub(in crate::card::sets) static RAL_MONSOON_MAGE_RAL_LEYLINE_PRODIGY_445: CardRecord =
    CardRecord::new(
        "Ral, Monsoon Mage // Ral, Leyline Prodigy",
        "0a7344ed-f94d-4983-a654-3896cf2e2396",
        "Borja Pindado",
        crate::card::CardRules::unsupported(),
    );

// MH3 448 — Guide of Souls
pub(in crate::card::sets) static GUIDE_OF_SOULS: CardRecord = CardRecord::new(
    "Guide of Souls",
    "298de33f-cb39-47c5-9579-54d91eb34414",
    "Ryan Valle",
// A one-mana body that turns every other creature into a life and an
    // energy, and then spends the energy making one of them an Angel.
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Cleric"], 1, 2)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever another creature you control enters, you gain 1 life and get {E} (an energy \
                 counter).",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                // One life and one energy per creature, which is what makes the three-
                // energy payment a matter of a turn or two rather than a deck built for it.
                EffectDef::Sequence(&[
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::AddPlayerCounters {
                        recipient: EffectRecipientDef::Controller,
                        kind: CounterKind::named("energy"),
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
            // As with Inti, the target is declared when the attack trigger goes on
            // the stack rather than when the energy is paid, which is the one place
            // this differs from the printed reflexive trigger. "Whenever you attack"
            // guarantees an attacking creature, so there is always something to name.
            AbilityDef::triggered_with_targets(
                "Whenever you attack, you may pay {E}{E}{E}. When you do, put two +1/+1 counters and a \
                 flying counter on target attacking creature. It becomes an Angel in addition to its \
                 other types.",
                TriggerEventDef::attack_declared(ObjectPredicateDef::Any, 1, None),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Attacking,
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::PayOr(PayOrDef::optional(
                    &[CostDef::Energy(3)], // All three stick: the counters and the type are permanent, so the
                    // creature is still a flying Angel next turn.
                    &EffectDef::Sequence(&[
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(2),
                        },
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            kind: CounterKind::Flying,
                            amount: ValueDef::Constant(1),
                        },
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            effect: AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Angel"])),
                            duration: ResolvedEffectDurationDef::Permanent,
                        },
                    ]),
                )),
            ),
        ]),
);

// MH3 450 — Dreamtide Whale
pub(in crate::card::sets) static DREAMTIDE_WHALE_450: CardRecord = CardRecord::new(
    "Dreamtide Whale",
    "966e2066-ef45-4882-a420-247115a319b9",
    "Ron Spears",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Whale"], 7, 5).with_abilities(&[
AbilityDef::as_enters("Vanishing 2 (This creature enters with two time counters on it. At the beginning of your upkeep, remove a time counter from it. When the last is removed, sacrifice it.)", ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::AddCounters { kind: CounterKind::named("time"), amount: 2 })),
AbilityDef::triggered_if("At the beginning of your upkeep, if this creature has a time counter on it, remove a time counter from it.", TriggerEventDef::StepBegins { step: TurnStepDef::Upkeep, player: PlayerRelation::You }, &TriggerConditionDef::SourceCounters { kind: CounterKind::named("time"), comparison: ComparisonDef::Greater, amount: 0 }, EffectDef::RemoveCounters { object: EffectRecipientDef::Source, kind: CounterKind::named("time"), amount: ValueDef::Constant(1) }),
AbilityDef::triggered("When the last time counter is removed from this creature, sacrifice it.", TriggerEventDef::LastCounterRemoved { object: ObjectPredicateDef::Source, kind: CounterKind::named("time") }, EffectDef::sacrifice(EffectRecipientDef::Source)),
AbilityDef::triggered("Whenever a player casts their second spell each turn, proliferate. (Choose any number of permanents and/or players, then give each another counter of each kind already there.)", TriggerEventDef::While { event: &TriggerEventDef::spell_cast(ObjectPredicateDef::Any), condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef { left: ValueDef::CountSpellsCastThisTurn(&SpellCastQueryDef { spell: ObjectPredicateDef::Any, player: PlayerRelation::EventPlayer }), comparison: ComparisonDef::Equal, right: ValueDef::Constant(2) }) }, EffectDef::Proliferate)
]),
);

// MH3 452 — Crabomination
pub(in crate::card::sets) static CRABOMINATION: CardRecord = CardRecord::new(
    "Crabomination",
    "b6ac511f-6c28-45f9-968b-9ac72872641b",
    "Nicholas Gregory",
CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Crab", "Demon"], 5, 5).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{5}{B}{B}")), // The reduction the keyword applies is generic only, so a big
            // enough artifact still leaves both black pips owed.
            CostDef::sacrifice(
                ObjectPredicateDef::HasType(CardType::Artifact),
                CostQuantityDef::Fixed(1),
            )],
            AlternativeCastKindDef::Emerge,
            None,
            EffectDef::None,
        ),
        abilities::enters_trigger_with_targets(
            "When this creature enters, target opponent exiles the top card of their library, a card at random from their graveyard, and a card at random from their hand. You may cast a spell from among cards exiled this way without paying its mana cost.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                PlayerRelation::Opponent,
            ))],
            // One pile out of three zones, and one permission over the whole
            // pile: the free cast is spent on whichever of the three is
            // worth having.
            EffectDef::ExileOneFromEachZone(&PileExileDef {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zones: &[
                    ZonePickDef::top(ZoneKind::Library),
                    // A library has an order to read from; a hand and a
                    // graveyard do not, which is why the card says "at
                    // random" for those two.
                    ZonePickDef::at_random(ZoneKind::Graveyard),
                    ZonePickDef::at_random(ZoneKind::Hand),
                ],
                permission: Some(ExiledCastPermissionDef::FreeWhileResolving),
            }),
        ),
    ]),
);

// MH3 455 — Ripples of Undeath
pub(in crate::card::sets) static RIPPLES_OF_UNDEATH_455: CardRecord = CardRecord::new(
    "Ripples of Undeath",
    "0136a022-6b16-4b33-a817-946ded4e9dd5",
    "Ben Wootten",
    CardRules::new_enchantment(mana_cost!("{1}{B}")).with_abilities(&[
AbilityDef::triggered("At the beginning of your first main phase, mill three cards. Then you may pay {1} and 3 life. If you do, put a card from among those cards into your hand.", TriggerEventDef::StepBegins { step: TurnStepDef::PrecombatMain, player: PlayerRelation::You }, EffectDef::BindObjects(BindObjectsDef { source: ObjectCollectionSourceDef::TopCards { player: PlayerRefDef::EffectController, count: ValueDef::Constant(3) }, binding: Binding!("ripples_top"), then: &EffectDef::Sequence(&[EffectDef::Mill { player: EffectRecipientDef::Controller, amount: ValueDef::Constant(3) }, EffectDef::PayOr(PayOrDef::optional(&[CostDef::Mana(mana_cost!("{1}")), CostDef::PayLife(3)], &EffectDef::Choose(ChooseDef { chooser: PlayerRefDef::EffectController, candidates: ObjectSetDef::ZoneChangeSuccessorsOfBinding(Binding!("ripples_top")), exclude: None, minimum: 1, maximum: 1, binding: ObjectChoiceBindingDef::Objects(Binding!("ripples_return")), unchosen: None, visibility: ChoiceVisibilityDef::Private, then: &EffectDef::move_to_zone(EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("ripples_return"))), ZoneKind::Hand, ZonePlacement::Top) })))]) }))
]),
);

// MH3 457 — Detective's Phoenix (alternate printing)
const DETECTIVES_PHOENIX_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DETECTIVES_PHOENIX,
    1,
    "db16e02b-1b7f-4976-b9eb-41350337616c",
    "Deruchenko Alexander",
);

// MH3 460 — Wight of the Reliquary
/// Your own graveyard, which is what makes the sacrifice cost pay twice: the
/// creature it eats is a land and a point of power both.
static RELIQUARY_CREATURE_CARDS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Graveyard],
    PlayerRelation::You,
);

pub(in crate::card::sets) static WIGHT_OF_THE_RELIQUARY: CardRecord = CardRecord::new(
    "Wight of the Reliquary",
    "915715f7-5487-47aa-ada5-de1bce282164",
    "Scott Murphy",
    // Two mana for a body that grows with the graveyard it is filling, and
    // turns every spare creature into whatever land the deck needs.
    CardRules::new_creature(mana_cost!("{B}{G}"), &["Zombie", "Knight"], 2, 2).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::static_ability(
            "This creature gets +1/+1 for each creature card in your graveyard.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::CountMatchingObjects(&RELIQUARY_CREATURE_CARDS),
                    ValueDef::CountMatchingObjects(&RELIQUARY_CREATURE_CARDS),
                ),
            },
        ),
        AbilityDef::activated(
            "{T}, Sacrifice another creature: Search your library for a land card, put it onto the \
                 battlefield tapped, then shuffle.",
            &[
                CostDef::TapSource,
                CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::HasType(CardType::Land),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: false,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: true,
                attachment: None,
                binding: None,
                then: None,
            },
        ),
    ]),
);

// MH3 462 — Winter Moon
pub(in crate::card::sets) static WINTER_MOON_462: CardRecord = CardRecord::new(
    "Winter Moon",
    "1ea94321-7311-4543-bdc0-23938a8904c3",
    "Drew Baker",
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[AbilityDef::static_ability(
        "Players can't untap more than one nonbasic land during their untap steps.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::EachPlayer,
            effect: AppliedEffectDef::Rule(AppliedRuleDef::UntapAtMostOne(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(CardSupertype::Basic)),
                ]),
            )),
        },
    )]),
);

// MH3 474 — Herigast, Erupting Nullkite
// Audit: unsupported — The engine can author an individual emerge cost but cannot grant emerge using each creature spell's own mana cost to all of a player's creature spells.
pub(in crate::card::sets) static HERIGAST_ERUPTING_NULLKITE_474: CardRecord = CardRecord::new(
    "Herigast, Erupting Nullkite",
    "72a86d7d-a7a4-4a26-b92a-0518af2d9646",
    "Lucas Graciano",
    crate::card::CardRules::unsupported(),
);

// MH3 484 — Six (alternate printing)
const SIX_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SIX,
    1,
    "8f3070e7-dce0-4121-bbef-c4357f8265ab",
    "Andrew Mar",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &DEVOURER_OF_DESTINY,
    &GLARING_FLESHRAKER_7,
    &KOZILEK_THE_BROKEN_REALITY_10,
    &KOZILEK_S_COMMAND_11,
    &AERIE_AUXILIARY,
    &DOG_UMBRA,
    &FLARE_OF_FORTITUDE_26,
    &MANDIBULAR_KITE,
    &OCELOT_PRIDE,
    &PHELIA_EXUBERANT_SHEPHERD,
    &STATIC_PRISON,
    &THRABEN_CHARM,
    &AMPHIBIAN_DOWNPOUR_51,
    &BRAINSURGE,
    &CONSIGN_TO_MEMORY_54,
    &HARBINGER_OF_THE_SEAS_63,
    &SERUM_VISIONARY,
    &STRIX_SERENADE_71,
    &VOLATILE_STORMDRAKE_79,
    &ACCURSED_MARAUDER,
    &EMPEROR_OF_BONES,
    &NETHERGOYF,
    &RETROFITTED_TRANSMOGRANT,
    &SCURRILOUS_SENTRY,
    &WITHER_AND_BLOOM,
    &AMPED_RAPTOR,
    &DETECTIVES_PHOENIX,
    &GALVANIC_DISCHARGE,
    &GHOSTFIRE_SLICE_123,
    &GLIMPSE_THE_IMPOSSIBLE_124,
    &MOLTEN_GATEKEEPER,
    &SIEGE_SMASH_136,
    &BASKING_BROODSCALE,
    &COLLECTIVE_RESISTANCE,
    &COLOSSAL_DREADMASK,
    &ELDRAZI_REPURPOSER,
    &EVOLUTION_WITNESS,
    &FANATIC_OF_RHONAS,
    &HORRIFIC_ASSAULT,
    &MALEVOLENT_RUMBLE,
    &NYXBORN_HYDRA,
    &SIX,
    &SOWING_MYCOSPAWN,
    &SPRINGHEART_NANTUKO,
    &TEMPERAMENTAL_OOZEWAGG,
    &CONDUIT_GOBLIN,
    &EXPANDING_OOZE,
    &FAITHFUL_WATCHDOG,
    &OBSTINATE_GARGOYLE_195,
    &PHLAGE_TITAN_OF_FIRES_FURY,
    &PSYCHIC_FROG,
    &SNAPPING_VOIDCRAW,
    &WRITHING_CHRYSALIS,
    &DISRUPTOR_FLUTE,
    &VEXING_BAUBLE_212,
    &BOUNTIFUL_LANDSCAPE,
    &CONTAMINATED_LANDSCAPE,
    &DECEPTIVE_LANDSCAPE,
    &FOREBODING_LANDSCAPE,
    &PERILOUS_LANDSCAPE,
    &SEETHING_LANDSCAPE,
    &SHATTERED_LANDSCAPE,
    &SHELTERING_LANDSCAPE,
    &SHIFTING_WOODLAND,
    &TRANQUIL_LANDSCAPE,
    &TWISTED_LANDSCAPE,
    &UGIN_S_LABYRINTH_233,
    &URZA_S_CAVE_234,
    &AJANI_NACATL_PARIAH,
    &RAZORGRASS_AMBUSH_RAZORGRASS_FIELD_238,
    &WITCH_ENCHANTER,
    &HYDROELECTRIC_SPECIMEN_HYDROELECTRI_240,
    &SINK_INTO_STUPOR,
    &BOGGART_TRAWLER_BOGGART_BOG_243,
    &FELL_THE_PROFANE_FELL_MIRE_244,
    &PINNACLE_MONK_MYSTIC_PEAK_246,
    &SUNDERING_ERUPTION_VOLCANIC_FISSURE_248,
    &BRIDGEWORKS_BATTLE_TANGLESPAN_BRIDGEWORKS_249,
    &DISCIPLE_OF_FREYALISE_GARDEN_OF_FREYALISE_250,
    &WATERLOGGED_TEACHINGS_INUNDATED_ARCHIVE_261,
    &ECHOES_OF_ETERNITY_320,
    &PARTY_THRASHER_334,
    &POWERBALANCE_335,
    &ARCHWAY_OF_INNOVATION_350,
    &ARENA_OF_GLORY,
    &NADU_WINGED_WISDOM,
    &ULAMOG_THE_DEFILER_383,
    &NULL_ELEMENTAL_BLAST_387,
    &FLARE_OF_DENIAL_400,
    &GRIM_SERVANT_409,
    &MARIONETTE_APPRENTICE_410,
    &NECRODOMINANCE_411,
    &WARREN_SOULTRADER_414,
    &FLARE_OF_DUPLICATION_416,
    &UNSTABLE_AMULET_421,
    &MONSTROUS_VORTEX_427,
    &TAMIYO_INQUISITIVE_STUDENT,
    &SORIN_OF_HOUSE_MARKOV,
    &RAL_MONSOON_MAGE_RAL_LEYLINE_PRODIGY_445,
    &GUIDE_OF_SOULS,
    &DREAMTIDE_WHALE_450,
    &CRABOMINATION,
    &RIPPLES_OF_UNDEATH_455,
    &WIGHT_OF_THE_RELIQUARY,
    &WINTER_MOON_462,
    &HERIGAST_ERUPTING_NULLKITE_474,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    ANNOYED_ALTISAUR_REPRINT,
    PRIEST_OF_TITANIA_REPRINT,
    DETECTIVES_PHOENIX_ALTERNATE_1,
    SIX_ALTERNATE_1,
];
