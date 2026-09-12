//! Dominaria United cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::ParentBinding;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BattlefieldArrivalDef;
use crate::card::BindObjectsDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseCardsFromCollectionDef;
use crate::card::ChooseDef;
use crate::card::CollectionInspectionDef;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::CreatureTypeSetDef;
use crate::card::DrawEventMatcherDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::GraveyardPlayPermissionDef;
use crate::card::IfNoObjectsDef;
use crate::card::InstalledTriggerDef;
use crate::card::ManaColor;
use crate::card::MoveObjectsDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetCountConditionDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectSetPredicateDef;
use crate::card::PlayActionMatcherDef;
use crate::card::PlayRestrictionDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::RandomizeObjectOrderDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "DMU",
    slug: "dominaria-united",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// DMU 2 — Anointed Peacekeeper
pub(in crate::card::sets) static ANOINTED_PEACEKEEPER: CardRecord = CardRecord::new(
    "Anointed Peacekeeper",
    "5b8127b5-3a65-411a-84bc-54e5c1be1477",
    "Tia Masic",
CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Cleric"], 3, 3).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::as_enters(
            "As this creature enters, look at an opponent's hand, then choose any card name.",
            crate::card::ReplacementEffectDef::Sequence(&[
                crate::card::ReplacementEffectDef::LookAtHand(PlayerRelation::Opponent),
                crate::card::ReplacementEffectDef::BindOutput {
                    binding: crate::Binding!("anointed_peacekeeper_name"),
                    effect: &abilities::choose_card_name_as_enters(
                        crate::card::CardNameSetDef::AllCardNames,
                    ),
                },
            ]),
        ),
        abilities::spell_cost_increase_for_name(
            "Spells your opponents cast with the chosen name cost {2} more to cast.",
            crate::card::CardNameDef::Binding(crate::Binding!("anointed_peacekeeper_name")),
            PlayerRelation::Opponent,
            mana_cost!("{2}"),
        ),
        abilities::ability_cost_increase_for_name(
            "Activated abilities of sources with the chosen name cost {2} more to activate unless they're mana abilities.",
            crate::card::CardNameDef::Binding(crate::Binding!("anointed_peacekeeper_name")),
            mana_cost!("{2}"),
        ),
    ]),
);

// DMU 19 — Guardian of New Benalia
// Audit: unsupported — There is no enlist declaration-time tap choice, including the no-summoning-sickness restriction, or enlist event carrying the enlisted creature's power.
pub(in crate::card::sets) static GUARDIAN_OF_NEW_BENALIA_19: CardRecord = CardRecord::new(
    "Guardian of New Benalia",
    "43da76ee-fec3-4b2e-915d-10cf8d518d2c",
    "Ernanda Souza",
    crate::card::CardRules::unsupported(),
);

// DMU 24 — Leyline Binding
pub(in crate::card::sets) static LEYLINE_BINDING: CardRecord = CardRecord::new(
    "Leyline Binding",
    "3c3ac3dd-35db-447f-8674-37b4680a1ef7",
    "Cristi Balanescu",
// Six mana on paper and one in a deck with every basic land type, cast
    // at instant speed: the whole card is the mana base it asks for.
    CardRules::new_enchantment(mana_cost!("{5}{W}")).with_abilities(&[
        abilities::flash(),
        AbilityDef::static_ability(
            "Domain — This spell costs {1} less to cast for each basic land type among lands you \
             control.",
            EffectDef::ReduceGenericCostBy(ValueDef::BasicLandTypesControlled(PlayerRelation::You)),
        )
        .with_source_zones(&[ZoneKind::Hand]),
        abilities::enters_trigger_with_targets(
            "When this enchantment enters, exile target nonland permanent an opponent controls \
             until this enchantment leaves the battlefield.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::ExileLinkedToSource {
                    until_source_leaves: true,
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    face_down: false,
                    then: None,
                },
                // "Until this enchantment leaves the battlefield" is one printed clause, so
                // the return rides on a delayed trigger rather than appearing as a second
                // ability the card does not print.
                EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                    "When this enchantment leaves the battlefield, return the exiled card to the battlefield \
                     under its owner's control.",
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
            ]),
        ),
    ]),
);

// DMU 28 — Prayer of Binding
// Audit: unsupported — Needs an exile-until-source-leaves duration with immediate return when that duration ends (CR 610.3); an ordinary leaves trigger returns the card later through the stack.
pub(in crate::card::sets) static PRAYER_OF_BINDING: CardRecord = CardRecord::new(
    "Prayer of Binding",
    "322f90b6-6b49-458d-9d5b-b601bfdd0af8",
    "Wylie Beckert",
    CardRules::unsupported(),
);

// DMU 29 — Resolute Reinforcements
pub(in crate::card::sets) static RESOLUTE_REINFORCEMENTS: CardRecord = CardRecord::new(
    "Resolute Reinforcements",
    "3e11ad33-b9d7-43ef-840a-61955683b599",
    "Billy Christian",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Soldier"], 1, 1).with_abilities(&[
        abilities::flash(),
        abilities::enters_trigger(
            "When this creature enters, create a 1/1 white Soldier \
             creature token.",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Soldier"], &[ManaColor::White], 1, 1),
            ))),
        ),
    ]),
);

// DMU 35 — Take Up the Shield
pub(in crate::card::sets) static TAKE_UP_THE_SHIELD: CardRecord = CardRecord::new(
    "Take Up the Shield",
    "851e842e-a497-4c36-90ee-8d64f806c378",
    "Manuel Castañón",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Put a +1/+1 counter on target creature. It gains lifelink and \
         indestructible until end of turn. (Damage and effects that \
         say \"destroy\" don't destroy it.)",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_ability(&abilities::lifelink()),
                    AppliedEffectDef::add_ability(&abilities::indestructible()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )]),
);

// DMU 57 — Micromancer
pub(in crate::card::sets) static MICROMANCER: CardRecord = CardRecord::new(
    "Micromancer",
    "b21203c8-a935-4ce0-a742-148587e32145",
    "Ernanda Souza",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Human", "Wizard"], 3, 3).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, you may search your library for an \
             instant or sorcery card with mana value 1, reveal it, put it \
             into your hand, then shuffle.",
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Instant),
                            ObjectPredicateDef::HasType(CardType::Sorcery),
                        ]),
                        ObjectPredicateDef::ManaValueEqualTo(ValueDef::Constant(1)),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: true,
                    destination: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            },
        ),
    ]),
);

// DMU 64 — Shore Up
pub(in crate::card::sets) static SHORE_UP: CardRecord = CardRecord::new(
    "Shore Up",
    "9d933bf1-14f0-4150-a0d2-6b845b9624cf",
    "Mark Behm",
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature you control gets +1/+1 and gains hexproof \
         until end of turn. Untap it. (It can't be the target of \
         spells or abilities your opponents control.)",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    AppliedEffectDef::add_ability(&abilities::hexproof()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ]),
    )]),
);

// DMU 72 — Tolarian Terror
pub(in crate::card::sets) static TOLARIAN_TERROR: CardRecord = CardRecord::new(
    "Tolarian Terror",
    "42f01cba-43d4-46ad-b7a5-d7631b0e1347",
    "Vincent Christiaens",
// Seven mana on paper and two in practice, which is what makes ward the
    // relevant half: the deck that casts it cheaply is holding up counters.
    CardRules::new_creature(mana_cost!("{6}{U}"), &["Serpent"], 5, 5).with_abilities(&[
        AbilityDef::static_ability(
            "This spell costs {1} less to cast for each instant and sorcery card in your graveyard.",
            EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(
                &ObjectQueryDef::matching(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                ),
            )),
        )
        // Read from hand, where the cost is paid, rather than from the
        // battlefield the creature is heading to.
        .with_source_zones(&[ZoneKind::Hand]),
        abilities::ward(
            &[crate::CostDef::Mana(crate::ManaCost::new(2, 0))],
            "Ward {2} (Whenever this creature becomes the target of a spell or ability an opponent controls, counter it unless that player pays {2}.)",
        ),
    ]),
);

// DMU 89 — Cut Down
pub(in crate::card::sets) static CUT_DOWN: CardRecord = CardRecord::new(
    "Cut Down",
    "753db072-5d6a-4f37-8f7d-255572ecd3bd",
    "Dominik Mayer",
    // One black mana answers most of what an aggressive deck plays and
    // nothing of what a big one does, which is the whole design.
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target creature with total power and toughness 5 or less.",
        // "Total power and toughness 5 or less" is read live, so a creature that
        // was in range stops being a legal target the moment anything pumps it.
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::TotalPowerAndToughnessAtMost(5),
            ]),
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            then: None,
        },
    )),
);

// DMU 102 — Pilfer
pub(in crate::card::sets) static PILFER: CardRecord = CardRecord::new(
    "Pilfer",
    "6d872c10-4126-4130-a74a-1331ed418ca8",
    "Pauline Voss",
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target opponent reveals their hand. You choose a nonland card \
         from it. That player discards that card.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Opponent),
        )],
        EffectDef::Sequence(&[
            EffectDef::RevealHand {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(crate::Binding!("discard")),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    &[ZoneKind::Hand],
                    PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                )),
                exclude: None,
                minimum: 1,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::discard_cards(EffectRecipientDef::objects(
                    ObjectSetDef::Binding(crate::Binding!("discard")),
                )),
            }),
        ]),
    )]),
);

// DMU 107 — Sheoldred, the Apocalypse
pub(in crate::card::sets) static SHEOLDRED_THE_APOCALYPSE: CardRecord = CardRecord::new(
    "Sheoldred, the Apocalypse",
    "d67be074-cdd4-41d9-ac89-0a0456c4e4b2",
    "Chris Rahn",
    // A four-mana 4/5 deathtouch would be playable on its own. The draw
    // clauses are what make it unanswerable: the opponent's own draw step
    // pays for it, every turn it survives.
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Phyrexian", "Praetor"], 4, 5)
        .with_supertype(CardSupertype::Legendary)
        // Two clauses rather than one symmetrical one, because they are not
        // symmetrical: yours gains and theirs loses, and a card that made both
        // players lose would read very differently.
        .with_abilities(&[
            abilities::deathtouch(),
            AbilityDef::triggered(
                "Whenever you draw a card, you gain 2 life.",
                TriggerEventDef::DrewCard(DrawEventMatcherDef::any(PlayerRelation::You)),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ),
            AbilityDef::triggered(
                "Whenever an opponent draws a card, they lose 2 life.",
                TriggerEventDef::DrewCard(DrawEventMatcherDef::any(PlayerRelation::Opponent)),
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::EventPlayer,
                    amount: ValueDef::Constant(2),
                },
            ),
        ]),
);

// DMU 122 — Electrostatic Infantry
pub(in crate::card::sets) static ELECTROSTATIC_INFANTRY_122: CardRecord = CardRecord::new(
    "Electrostatic Infantry",
    "5ed2d72f-f1cf-45a7-adf7-969f531721ce",
    "Kekai Kotaki",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Dwarf", "Wizard"], 1, 2).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered(
            "Whenever you cast an instant or sorcery spell, put a +1/+1 counter on this creature.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
            ])),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: crate::card::CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// DMU 137 — Lightning Strike (reprint)
const LIGHTNING_STRIKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2013::theros::LIGHTNING_STRIKE,
    "7d541125-bfb8-4f88-8bf3-ad7b6af7ad1d",
    "Marta Nael",
);

// DMU 155 — Bite Down
pub(in crate::card::sets) static BITE_DOWN: CardRecord = CardRecord::new(
    "Bite Down",
    "0eacd3de-b803-4322-8d88-d533761aa748",
    "Kitt Lapeña",
    CardRules::new_instant(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature you control deals damage equal to its power \
         to target creature or planeswalker you don't control.",
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
                controller: Some(PlayerRelation::Opponent),
                owner: None,
            }),
        ],
        EffectDef::damage_from(
            ObjectRefDef::Target(TargetIndex::PRIMARY),
            EffectRecipientDef::Target(TargetIndex(1)),
            ValueDef::TargetPower(TargetIndex::PRIMARY),
        ),
    )]),
);

// DMU 172 — Magnigoth Sentry
pub(in crate::card::sets) static MAGNIGOTH_SENTRY: CardRecord = CardRecord::new(
    "Magnigoth Sentry",
    "d939d4bc-b7e8-4ee8-b904-68f0bff0fde1",
    "Dave Kendall",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Treefolk"], 4, 4)
        .with_abilities(&[abilities::reach()]),
);

// DMU 177 — Silverback Elder
pub(in crate::card::sets) static SILVERBACK_ELDER_177: CardRecord = CardRecord::new(
    "Silverback Elder",
    "b987664f-0b74-4c0a-b306-14767a55559a",
    "Alexander Mokhov",
    CardRules::new_creature(mana_cost!("{2}{G}{G}{G}"), &["Ape", "Shaman"], 5, 7).with_abilities(&[
AbilityDef::modal_triggered("Whenever you cast a creature spell, choose one —\n• Destroy target artifact or enchantment.\n• Look at the top five cards of your library. You may put a land card from among them onto the battlefield tapped. Put the rest on the bottom of your library in a random order.\n• You gain 4 life.", TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::ControlledBy(PlayerRelation::You)])), &[AbilityDef::destroy_target("Destroy target artifact or enchantment.", &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::HasType(CardType::Enchantment)]))), AbilityDef::spell("Look at the top five cards of your library. You may put a land card from among them onto the battlefield tapped. Put the rest on the bottom of your library in a random order.", EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef { source: ObjectCollectionSourceDef::TopCards { player: PlayerRefDef::EffectController, count: ValueDef::Constant(5) }, actor: PlayerRefDef::EffectController, inspection: CollectionInspectionDef::Look, object: ObjectPredicateDef::HasType(CardType::Land), minimum: 0, maximum: 1, chosen: Binding!("elder_land"), remainder: Binding!("elder_rest"), then: &EffectDef::Sequence(&[EffectDef::WithBattlefieldArrival { arrival: BattlefieldArrivalDef { controller: None, counters: None, attachment: None, modifications: &[crate::card::BattlefieldEntryModificationDef::Tapped] }, effect: &EffectDef::move_to_zone(EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("elder_land"))), ZoneKind::Battlefield, ZonePlacement::Top) }, EffectDef::RandomizeObjectOrder(RandomizeObjectOrderDef { input: ObjectSetDef::Binding(Binding!("elder_rest")), randomized: Binding!("elder_ordered"), then: &EffectDef::MoveObjects(MoveObjectsDef { input: ObjectSetDef::Binding(Binding!("elder_ordered")), from: Some(ZoneKind::Library), zone: ZoneKind::Library, placement: ZonePlacement::Bottom, moved: None, then: &EffectDef::None }) })]) })), AbilityDef::spell("You gain 4 life.", EffectDef::GainLife { recipient: EffectRecipientDef::Controller, amount: ValueDef::Constant(4) })])
]),
);

// DMU 182 — Tail Swipe
// Audit: unsupported — Fight and the two opposing creature targets are representable, but the
// conditional pump needs cast-time own-main-phase provenance. SourceCastAtInstantSpeed does not
// distinguish a main-phase response from a sorcery-speed cast, so it cannot express this condition.
pub(in crate::card::sets) static TAIL_SWIPE_182: CardRecord = CardRecord::new(
    "Tail Swipe",
    "95a39b26-8c83-40ea-b492-036251366d73",
    "Ângelo Bortolini",
    crate::card::CardRules::unsupported(),
);

// DMU 183 — Tear Asunder
pub(in crate::card::sets) static TEAR_ASUNDER: CardRecord = CardRecord::new(
    "Tear Asunder",
    "629aa907-9533-4681-9bf2-9e56450a4cc2",
    "Dave Kendall",
// Two mana for the artifact or enchantment the deck was worried about,
    // or four for anything at all -- and exile rather than destruction,
    // which is what the extra mana is really paying for.
    CardRules::new_instant(mana_cost!("{1}{G}")).with_abilities(&[
        abilities::kicker(
            &[crate::CostDef::Mana(mana_cost!("{1}{B}"))],
        ),
        AbilityDef::spell_with_targets(
            "Exile target artifact or enchantment. If this spell was kicked, exile target nonland permanent instead.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::IfAdditionalCostPaid {
                    cost: crate::AdditionalCostIndex::PRIMARY,
                    // What four buys instead. "Instead" widens the one target.
                    if_paid: &AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                            CardType::Land,
                        )),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    // What two mana buys, on either side of the board.
                    otherwise: &AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                },
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Exile,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// DMU 196 — Balmor, Battlemage Captain
pub(in crate::card::sets) static BALMOR_BATTLEMAGE_CAPTAIN: CardRecord = CardRecord::new(
    "Balmor, Battlemage Captain",
    "959ba62e-bb3a-49ad-8b1b-e787e413e5d4",
    "Bram Sels",
    CardRules::new_creature(mana_cost!("{U}{R}"), &["Bird", "Wizard"], 1, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::triggered(
                "Whenever you cast an instant or sorcery spell, creatures you \
                 control get +1/+0 and gain trample until end of turn.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::add_ability(&abilities::trample()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// DMU 200 — Garna, Bloodfist of Keld
pub(in crate::card::sets) static GARNA_BLOODFIST_OF_KELD: CardRecord = CardRecord::new(
    "Garna, Bloodfist of Keld",
    "294c5f08-08e7-458f-8838-ff321dc5d9f2",
    "Andrey Kuzinskiy",
    CardRules::new_creature(mana_cost!("{1}{B}{R}{R}"), &["Human", "Berserker"], 4, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::triggered(
            "Whenever another creature you control dies, draw a card if it \
             was attacking. Otherwise, Garna deals 1 damage to each \
             opponent.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::BindObjects(BindObjectsDef {
                binding: crate::Binding!("dead"),
                source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::One(
                    ObjectRefDef::TriggeringObject,
                )),
                then: &EffectDef::IfElseCondition {
                    condition: &TriggerConditionDef::ObjectSetCount(&ObjectSetCountConditionDef {
                        objects: &ObjectSetDef::Binding(crate::Binding!("dead")),
                        predicate: ObjectSetPredicateDef::contains(&ObjectPredicateDef::Attacking),
                    }),
                    then: &abilities::draw_cards(ValueDef::Constant(1)),
                    otherwise: &EffectDef::damage(
                        EffectRecipientDef::Opponent,
                        ValueDef::Constant(1),
                    ),
                },
            }),
        )]),
);

// DMU 232 — Inscribed Tablet
pub(in crate::card::sets) static INSCRIBED_TABLET_232: CardRecord = CardRecord::new(
    "Inscribed Tablet",
    "699d8655-d250-4ab6-92c5-376979bbabc7",
    "Jarel Threat",
    CardRules::new_artifact(mana_cost!("{1}")).with_ability(AbilityDef::activated(
        "{1}, {T}, Sacrifice this artifact: Reveal the top five cards of your library. Put a land card from among them into your hand and the rest on the bottom of your library in a random order. If you didn't put a card into your hand this way, draw a card.",
        &[crate::card::CostDef::Mana(mana_cost!("{1}")), crate::card::CostDef::TapSource, crate::card::CostDef::SacrificeSource],
        EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
            source: ObjectCollectionSourceDef::TopCards { player: PlayerRefDef::EffectController, count: ValueDef::Constant(5) }, actor: PlayerRefDef::EffectController, inspection: CollectionInspectionDef::Reveal,
            object: ObjectPredicateDef::HasType(CardType::Land), minimum: 1, maximum: 1,
            chosen: crate::Binding!("inscribed_tablet_chosen"), remainder: crate::Binding!("inscribed_tablet_remainder"),
            then: &EffectDef::MoveObjects(MoveObjectsDef { input: ObjectSetDef::Binding(crate::Binding!("inscribed_tablet_chosen")), from: Some(ZoneKind::Library), zone: ZoneKind::Hand, placement: ZonePlacement::Top, moved: Some(crate::Binding!("inscribed_tablet_to_hand")), then: &EffectDef::RandomizeObjectOrder(RandomizeObjectOrderDef { input: ObjectSetDef::Binding(crate::Binding!("inscribed_tablet_remainder")), randomized: ParentBinding, then: &EffectDef::Sequence(&[EffectDef::MoveObjects(MoveObjectsDef { input: ObjectSetDef::Binding(ParentBinding), from: Some(ZoneKind::Library), zone: ZoneKind::Library, placement: ZonePlacement::Bottom, moved: None, then: &EffectDef::None }), EffectDef::IfNoObjects(IfNoObjectsDef { input: ObjectSetDef::Binding(crate::Binding!("inscribed_tablet_to_hand")), if_empty: &EffectDef::DrawCards { recipient: EffectRecipientDef::Controller, amount: ValueDef::Constant(1) }, otherwise: &EffectDef::None })]) }) }),
        }),
    )),
);

// DMU 236 — Relic of Legends
// Audit: unsupported — The mana-ability planner rejects a selected-creature tap cost. TapSource cannot represent tapping a legendary creature while leaving the Relic untapped.
pub(in crate::card::sets) static RELIC_OF_LEGENDS_236: CardRecord = CardRecord::new(
    "Relic of Legends",
    "64a2809e-c441-416c-90ff-6fb1e246dff3",
    "Titus Lunter",
    crate::card::CardRules::unsupported(),
);

// DMU 246 — Crystal Grotto
pub(in crate::card::sets) static CRYSTAL_GROTTO: CardRecord = CardRecord::new(
    "Crystal Grotto",
    "bd250c9d-c65f-4293-a6b0-007fac634d3d",
    "Piotr Dura",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_trigger(
            "When this land enters, scry 1.",
            abilities::scry(ValueDef::Constant(1)),
        ),
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{1}, {T}: Add one mana of any color.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// DMU 252 — Plaza of Heroes
// Audit: unsupported — ManaTypeSource cannot derive the available colors from legendary permanents. The legendary-spell restriction alone does not implement the separate unrestricted color-producing ability.
pub(in crate::card::sets) static PLAZA_OF_HEROES_252: CardRecord = CardRecord::new(
    "Plaza of Heroes",
    "a2cfcf67-f83c-43af-9e2d-5513fcdde835",
    "Gabor Szikszai",
    crate::card::CardRules::unsupported(),
);

// DMU 282 — Serra Redeemer
pub(in crate::card::sets) static SERRA_REDEEMER: CardRecord = CardRecord::new(
    "Serra Redeemer",
    "a8b9cb5c-29f2-46ed-803e-c2170955217c",
    "Joshua Raphael",
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Angel", "Soldier"], 2, 4).with_abilities(
        &[
            abilities::flying(),
            AbilityDef::triggered(
                "Whenever another creature you control with power 2 or less \
                 enters, put two +1/+1 counters on that creature.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ObjectPredicateDef::PowerLessThan(ValueDef::Constant(3)),
                        ]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::TriggeringObject,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(2),
                },
            ),
        ],
    ),
);

// DMU 329 — Braids, Arisen Nightmare
// Audit: unsupported — The sacrifice continuation lacks a predicate comparing a candidate permanent's card types with the sacrificed permanent's last-known type set.
pub(in crate::card::sets) static BRAIDS_ARISEN_NIGHTMARE_329: CardRecord = CardRecord::new(
    "Braids, Arisen Nightmare",
    "1e20d56c-20df-4fe9-a329-df5768a180af",
    "Dibujante Nocturno",
    crate::card::CardRules::unsupported(),
);

// DMU 339 — Ertai Resurrected
pub(in crate::card::sets) static ERTAI_RESURRECTED: CardRecord = CardRecord::new(
    "Ertai Resurrected",
    "2c46a2ca-27fd-44d4-80d0-7c83ed0a564e",
    "Justin Hernandez & Alexis Hernandez",
// A flash body that answers something on the way in, and pays for the
    // privilege with the card its victim's controller draws.
    CardRules::new_creature(
        mana_cost!("{2}{U}{B}"),
        &["Phyrexian", "Human", "Wizard"],
        3,
        2,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::flash(),
        AbilityDef::modal_triggered_up_to_one(
            "When this creature enters, choose up to one —\n• Counter target spell, activated \
             ability, or triggered ability. Its controller draws a card.\n• Destroy another \
             target creature or planeswalker. Its controller draws a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            // Both modes pay the same compensation, and both read it off the target
            // after that target is gone: the countered or destroyed object is retired
            // with its controller recorded, which is what "its controller" wants.
            &[
                AbilityDef::spell_with_targets(
                    "Counter target spell, activated ability, or triggered ability. Its controller draws a \
                     card.",
                    // "Spell, activated ability, or triggered ability" is every stack object
                    // there is: mana abilities never use the stack, so the wider slot needs no
                    // clause excluding them.
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::Any,
                            zones: &[ZoneKind::Stack],
                            controller: None,
                            owner: None,
                        },
                    )],
                    EffectDef::Sequence(&[
                        EffectDef::Counter {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            zone: ZoneKind::Graveyard,
                            placement: ZonePlacement::Top,
                        },
                        EffectDef::DrawCards {
                            recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                            amount: ValueDef::Constant(1),
                        },
                    ]),
                ),
                AbilityDef::spell_with_targets(
                    "Destroy another target creature or planeswalker. Its controller draws a card.",
                    // "Another" is the exclusion; Ertai himself has just arrived, so without it
                    // he would be a legal answer to his own trigger.
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::All(&[
                                ObjectPredicateDef::AnyOf(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                                ]),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            zones: &[ZoneKind::Battlefield],
                            controller: None,
                            owner: None,
                        },
                    )],
                    EffectDef::Sequence(&[
                        EffectDef::Destroy {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            then: None,
                        },
                        EffectDef::DrawCards {
                            recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                            amount: ValueDef::Constant(1),
                        },
                    ]),
                ),
            ],
        ),
    ]),
);

// DMU 387 — Leyline Binding (alternate printing)
const LEYLINE_BINDING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LEYLINE_BINDING,
    1,
    "32da8479-7d0e-4eb1-b18c-66eb170e31a5",
    "Cristi Balanescu",
);

// DMU 388 — Serra Paragon
pub(in crate::card::sets) static SERRA_PARAGON: CardRecord = CardRecord::new(
    "Serra Paragon",
    "69284b53-f712-418c-94a0-4e5638117256",
    "Heonhwa",
// Four mana for a 3/4 flier that buys back a land or a cheap permanent
    // every turn it lives, and pays two life for each one on its way out.
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Angel"], 3, 4)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::static_ability(
                "Once during each of your turns, you may play a land from your graveyard or cast a \
                 permanent spell with mana value 3 or less from your graveyard. If you do, it gains \
                 \"When this permanent is put into a graveyard from the battlefield, exile it and you \
                 gain 2 life.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Controller,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromGraveyard(
                        // "A land ... or a permanent spell with mana value 3 or less": one
                        // permission rather than two, because the once-each-turn bound is on the
                        // pair. Any play action, since which one it is follows from the card --
                        // nothing but a land is ever played as a land, and nothing but a spell is
                        // ever cast.
                        GraveyardPlayPermissionDef::once_each_of_your_turns(PlayRestrictionDef::new(
                            PlayActionMatcherDef::Any,
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Land),
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::Not(&ObjectPredicateDef::AnyOf(&[
                                        ObjectPredicateDef::HasType(CardType::Instant),
                                        ObjectPredicateDef::HasType(CardType::Sorcery),
                                    ])),
                                    ObjectPredicateDef::ManaValueAtMost(3),
                                ]),
                            ]),
                        ))
                            // What the permanent gains, and what makes the Paragon a value engine
                            // rather than a recursion loop: the card leaves for good, and the two life
                            // are the consolation.
                            .granting(&AppliedEffectDef::add_ability(&abilities::dies_trigger(
                                "When this permanent is put into a graveyard from the battlefield, exile it and you gain 2 \
                                 life.",
                                EffectDef::Sequence(&[
                                    EffectDef::move_to_zone(
                                        EffectRecipientDef::TriggeringZoneChangeResult,
                                        ZoneKind::Exile,
                                        ZonePlacement::Top,
                                    ),
                                    EffectDef::GainLife {
                                        recipient: EffectRecipientDef::Controller,
                                        amount: ValueDef::Constant(2),
                                    },
                                ]),
                            ))),
                    )),
                },
            ),
        ]),
);

// DMU 409 — Rundvelt Hordemaster
// Audit: unsupported — The top-library exile permission has no card-characteristic filter. The arbitrary-card exile permission lasts only this turn, so neither can grant the Goblin-only permission through the end of your next turn.
pub(in crate::card::sets) static RUNDVELT_HORDEMASTER_409: CardRecord = CardRecord::new(
    "Rundvelt Hordemaster",
    "060d14a4-e903-4c89-9c3a-baa91f125c4e",
    "Bruno Biazotto",
    crate::card::CardRules::unsupported(),
);

// DMU 416 — Llanowar Loamspeaker
pub(in crate::card::sets) static LLANOWAR_LOAMSPEAKER_416: CardRecord = CardRecord::new(
    "Llanowar Loamspeaker",
    "5fdb1dfd-6394-414f-959e-9f129a3ab1a1",
    "Zara Alfonso",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Elf", "Druid"], 1, 3).with_abilities(&[
AbilityDef::activated_mana("{T}: Add one mana of any color.", &[CostDef::TapSource], EffectDef::AddMana(AddManaEffectDef::any_color())),
AbilityDef::activated_with_targets("{T}: Target land you control becomes a 3/3 Elemental creature with haste until end of turn. It's still a land. Activate only as a sorcery.", &[CostDef::TapSource], &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::HasType(CardType::Land), zones: &[ZoneKind::Battlefield], controller: Some(PlayerRelation::You), owner: None })], EffectDef::Apply { recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY), effect: AppliedEffectDef::Composite(&[AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)), AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Elemental"])), AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(3), ValueDef::Constant(3)), AppliedEffectDef::add_ability(&abilities::haste())]), duration: ResolvedEffectDurationDef::UntilEndOfTurn }).with_activation_timing(ActivationTimingDef::SorcerySpeed)
]),
);

// DMU 422 — Thran Portal
// Audit: unsupported — Chosen basic land types are supported, but there is no activation-cost modifier adding a life payment to every mana ability this land acquires.
pub(in crate::card::sets) static THRAN_PORTAL_422: CardRecord = CardRecord::new(
    "Thran Portal",
    "eba2995e-f255-46da-abcf-9a6f3996edb1",
    "Sarah Finnigan",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANOINTED_PEACEKEEPER,
    &GUARDIAN_OF_NEW_BENALIA_19,
    &LEYLINE_BINDING,
    &PRAYER_OF_BINDING,
    &RESOLUTE_REINFORCEMENTS,
    &TAKE_UP_THE_SHIELD,
    &MICROMANCER,
    &SHORE_UP,
    &TOLARIAN_TERROR,
    &CUT_DOWN,
    &PILFER,
    &SHEOLDRED_THE_APOCALYPSE,
    &ELECTROSTATIC_INFANTRY_122,
    &BITE_DOWN,
    &MAGNIGOTH_SENTRY,
    &SILVERBACK_ELDER_177,
    &TAIL_SWIPE_182,
    &TEAR_ASUNDER,
    &BALMOR_BATTLEMAGE_CAPTAIN,
    &GARNA_BLOODFIST_OF_KELD,
    &INSCRIBED_TABLET_232,
    &RELIC_OF_LEGENDS_236,
    &CRYSTAL_GROTTO,
    &PLAZA_OF_HEROES_252,
    &SERRA_REDEEMER,
    &BRAIDS_ARISEN_NIGHTMARE_329,
    &ERTAI_RESURRECTED,
    &SERRA_PARAGON,
    &RUNDVELT_HORDEMASTER_409,
    &LLANOWAR_LOAMSPEAKER_416,
    &THRAN_PORTAL_422,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] =
    &[LIGHTNING_STRIKE_REPRINT, LEYLINE_BINDING_ALTERNATE_1];
