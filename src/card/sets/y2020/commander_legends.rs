//! Commander Legends card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ConditionDef;
use crate::card::CopyStackObjectDef;
use crate::card::CostDef;
use crate::card::CreateTokenDef;
use crate::card::DeckConstructionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ExilePlayDurationDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::ReplacementAbilityDef;
use crate::card::ReplacementEffectDef;
use crate::card::ReplacementEventDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::card::tokens;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "CMR",
    slug: "commander-legends",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const TREASURE_TOKEN: TokenCharacteristics = tokens::treasure().with_art(CardArt::new(
    "284ec798-2725-4741-8748-578c259d0623",
    "Alayna Danner",
));

// CMR 3 — Akroma's Will
pub(in crate::card::sets) static AKROMA_S_WILL_3: CardRecord = CardRecord::new(
    "Akroma's Will",
    "c281997b-1566-4469-a14c-6645f81ab023",
    "Antonio José Manzanedo",
    CardRules::new_instant(mana_cost!("{3}{W}")).with_abilities(&[
AbilityDef::modal_spell("Choose one. If you control a commander as you cast this spell, you may choose both instead.", &[AbilityDef::spell("• Creatures you control gain flying, vigilance, and double strike until end of turn.", EffectDef::Apply { recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::You), effect: AppliedEffectDef::Composite(&[AppliedEffectDef::add_ability(&abilities::flying()), AppliedEffectDef::add_ability(&abilities::vigilance()), AppliedEffectDef::add_ability(&abilities::double_strike())]), duration: ResolvedEffectDurationDef::UntilEndOfTurn }), AbilityDef::spell("• Creatures you control gain lifelink, indestructible, and protection from each color until end of turn.", EffectDef::Apply { recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::You), effect: AppliedEffectDef::Composite(&[AppliedEffectDef::add_ability(&abilities::lifelink()), AppliedEffectDef::add_ability(&abilities::indestructible()), AppliedEffectDef::add_ability(&abilities::protection_from_color(ManaColor::White)), AppliedEffectDef::add_ability(&abilities::protection_from_color(ManaColor::Blue)), AppliedEffectDef::add_ability(&abilities::protection_from_color(ManaColor::Black)), AppliedEffectDef::add_ability(&abilities::protection_from_color(ManaColor::Red)), AppliedEffectDef::add_ability(&abilities::protection_from_color(ManaColor::Green))]), duration: ResolvedEffectDurationDef::UntilEndOfTurn })]).with_conditional_mode_maximum(ConditionDef::Exists(ObjectQueryDef::matching(ObjectPredicateDef::Commander, &[ZoneKind::Battlefield], PlayerRelation::You)), 2)
]),
);

// CMR 74 — Hullbreacher
pub(in crate::card::sets) static HULLBREACHER: CardRecord = CardRecord::new(
    "Hullbreacher",
    "4df8aabc-7fcb-4b7b-980b-18f499e6c170",
    "Sidharth Chaturvedi",
// Three mana at instant speed that turns their draw spell into your
    // mana, and a 3/2 body attached to it.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Merfolk", "Pirate"], 3, 2)
        .with_abilities(&[
            abilities::flash(),
            AbilityDef::defined_replacement(
                "If an opponent would draw a card except the first one they draw in each of their draw \
                 steps, instead you create a Treasure token. (It\'s an artifact with \"{T}, Sacrifice \
                 this token: Add one mana of any color.\")",
                // "Except the first one they draw in each of their draw steps": their
                // turn-based draw still happens, and everything after it does not.
                ReplacementAbilityDef::new()
                    .with_event(ReplacementEventDef::WouldDraw {
                        player: PlayerRelation::Opponent,
                        during_own_draw_step: false,
                        except_first_in_draw_step: true,
                    }),
                // The draw is replaced outright and the Treasure is the effect's
                // controller's, which is what makes this a tax on them rather than a gift:
                // the card they would have drawn stays in their library.
                ReplacementEffectDef::Sequence(&[
                    ReplacementEffectDef::ReplaceEventWithNothing,
                    ReplacementEffectDef::Perform(&EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN)))),
                ]),
            ),
        ]),
);

// CMR 79 — Malcolm, Keen-Eyed Navigator
// Audit: unsupported — Combat damage has a grouped player-damage event, but noncombat damage does not. This trigger must group simultaneous Pirate damage of either kind; a combat-only declaration would omit the noncombat half.
pub(in crate::card::sets) static MALCOLM_KEEN_EYED_NAVIGATOR_79: CardRecord = CardRecord::new(
    "Malcolm, Keen-Eyed Navigator",
    "bbc3bbda-a4bc-4302-a3fc-b1c89f0f5461",
    "Eric Deschamps",
    crate::card::CardRules::unsupported(),
);

// CMR 89 — Sakashima of a Thousand Faces
// Audit: unsupported — Needs the Partner deck-construction permission and an entry-copy exception that retains every other ability printed on the source.
pub(in crate::card::sets) static SAKASHIMA_OF_A_THOUSAND_FACES: CardRecord = CardRecord::new(
    "Sakashima of a Thousand Faces",
    "714c3a1f-7b30-4ed8-8f38-6176758741fb",
    "Jason A. Engle",
    CardRules::unsupported(),
);

// CMR 141 — Opposition Agent
// Audit: unsupported — Needs an opponent-search replacement that transfers control of the searched library and redirects the found card's destination.
pub(in crate::card::sets) static OPPOSITION_AGENT_141: CardRecord = CardRecord::new(
    "Opposition Agent",
    "086f97e9-8b62-44f3-b467-149c2ac5ca78",
    "Scott Murphy",
    crate::card::CardRules::unsupported(),
);

// CMR 172 — Dargo, the Shipwrecker
// Audit: unsupported — Casting costs cannot combine a variable additional sacrifice group with a two-mana discount per selected permanent. Turn history also has no count of other artifacts or creatures sacrificed this turn.
pub(in crate::card::sets) static DARGO_THE_SHIPWRECKER_172: CardRecord = CardRecord::new(
    "Dargo, the Shipwrecker",
    "5cd87cf8-4d5d-4aba-8dfa-800b1fb3799b",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

// CMR 183 — Hellkite Courser
// Audit: unsupported — Command-zone cards can be put onto the battlefield, but the ordinary battlefield-exit procedure ignores a Command destination. The mandatory delayed return therefore cannot move the commander back; replacing it with an optional commander return through exile or hand would change the printed effect.
pub(in crate::card::sets) static HELLKITE_COURSER_183: CardRecord = CardRecord::new(
    "Hellkite Courser",
    "db45122e-b5ef-487b-8ea9-59ea066d3c88",
    "Caio Monteiro",
    crate::card::CardRules::unsupported(),
);

// CMR 185 — Impulsive Pilferer
// Audit: unsupported — Needs Encore, including a graveyard activation, one token copy attacking each opponent, and delayed sacrifice.
pub(in crate::card::sets) static IMPULSIVE_PILFERER_185: CardRecord = CardRecord::new(
    "Impulsive Pilferer",
    "55ba9bea-5549-45cf-896c-501a1c81fd5a",
    "Jakub Kasper",
    crate::card::CardRules::unsupported(),
);

// CMR 187 — Jeska's Will
pub(in crate::card::sets) static JESKA_S_WILL_187: CardRecord = CardRecord::new(
    "Jeska's Will",
    "4e91d96d-cc69-439b-b876-a7d57039022c",
    "Izzy",
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_abilities(&[
AbilityDef::modal_spell("Choose one. If you control a commander as you cast this spell, you may choose both instead.", &[AbilityDef::spell_with_targets("• Add {R} for each card in target opponent's hand.", &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Opponent))], EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red).with_variable_amount(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(ObjectPredicateDef::Any, &[ZoneKind::Hand], PlayerRelation::Opponent))))), AbilityDef::spell("• Exile the top three cards of your library. You may play them this turn.", EffectDef::ExileTopOfLibraryToPlay { player: EffectRecipientDef::Controller, amount: ValueDef::Constant(3), free: false, face_down: false, duration: ExilePlayDurationDef::ThisTurn, spend_any_color: false, play_condition: None, cast_only: false })]).with_conditional_mode_maximum(ConditionDef::Exists(ObjectQueryDef::matching(ObjectPredicateDef::Commander, &[ZoneKind::Battlefield], PlayerRelation::You)), 2)
]),
);

// CMR 189 — Krark, the Thumbless
pub(in crate::card::sets) static KRARK_THE_THUMBLESS_189: CardRecord = CardRecord::new(
    "Krark, the Thumbless",
    "06a981cd-1951-438e-95c9-68294795638e",
    "Mathias Kollros",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Wizard"], 2, 2).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::triggered("Whenever you cast an instant or sorcery spell, flip a coin. If you lose the flip, return that spell to its owner's hand. If you win the flip, copy that spell, and you may choose new targets for the copy.", TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Instant), ObjectPredicateDef::HasType(CardType::Sorcery)]), ObjectPredicateDef::ControlledBy(PlayerRelation::You)])), EffectDef::FlipCoin { on_win: &EffectDef::CopyStackObject(&CopyStackObjectDef { object: EffectRecipientDef::TriggeringObject, controller: PlayerRefDef::EffectController, count: ValueDef::Constant(1), retarget: true, colors: None }), on_loss: &EffectDef::move_to_zone(EffectRecipientDef::TriggeringObject, ZoneKind::Hand, ZonePlacement::Top) }),
AbilityDef::deck_construction("Partner", DeckConstructionDef::Partner, "Both commanders are designated before the game.")
]),
);

// CMR 197 — Rograkh, Son of Rohgahh
pub(in crate::card::sets) static ROGRAKH_SON_OF_ROHGAHH_197: CardRecord = CardRecord::new(
    "Rograkh, Son of Rohgahh",
    "a4fab67f-00c2-4125-9262-d21a29411797",
    "Chris Seaman",
    CardRules::new_creature(mana_cost!("{0}"), &["Kobold", "Warrior"], 0, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::first_strike(),
            abilities::menace(),
            abilities::trample(),
            AbilityDef::deck_construction(
                "Partner (You can have two commanders if both have partner.)",
                DeckConstructionDef::Partner,
                "A symmetric commander pairing permission.",
            ),
        ]),
);

// CMR 211 — Wheel of Misfortune
// Audit: unsupported — Needs simultaneous secret integer choices, highest and lowest comparison, and the resulting per-player damage, discard, and draw.
pub(in crate::card::sets) static WHEEL_OF_MISFORTUNE_211: CardRecord = CardRecord::new(
    "Wheel of Misfortune",
    "74177b51-a300-49d9-8ea7-557b19cf80c7",
    "J.P. Targete",
    crate::card::CardRules::unsupported(),
);

// CMR 216 — Annoyed Altisaur
pub(in crate::card::sets) static ANNOYED_ALTISAUR: CardRecord = CardRecord::new(
    "Annoyed Altisaur",
    "7536d618-0c98-45bb-913b-b8117b4acf87",
    "Lars Grant-West",
    // Seven mana with cascade attached, which is why a limited deck plays it
    // as two cards rather than as an expensive one.
    CardRules::new_creature(mana_cost!("{5}{G}{G}"), &["Dinosaur"], 6, 5).with_abilities(&[
        abilities::reach(),
        abilities::trample(),
        abilities::cascade(),
    ]),
);

// CMR 217 — Apex Devastator
pub(in crate::card::sets) static APEX_DEVASTATOR_217: CardRecord = CardRecord::new(
    "Apex Devastator",
    "8fa281e1-5c48-4bba-b8e9-88c6f5f53abb",
    "Svetlin Velinov",
    CardRules::new_creature(mana_cost!("{8}{G}{G}"), &["Chimera", "Hydra"], 10, 10).with_abilities(
        &[
            abilities::cascade(),
            abilities::cascade(),
            abilities::cascade(),
            abilities::cascade(),
        ],
    ),
);

// CMR 305 — Commander's Plate
// Audit: unsupported — Needs protection dynamically derived from colors outside the controller's commander color identity, plus commander-specific equip.
pub(in crate::card::sets) static COMMANDER_S_PLATE_305: CardRecord = CardRecord::new(
    "Commander's Plate",
    "19992dbd-7a6a-43d3-b1db-01716b2eed27",
    "Volkan Baǵa",
    crate::card::CardRules::unsupported(),
);

// CMR 354 — Rejuvenating Springs
pub(in crate::card::sets) static REJUVENATING_SPRINGS_354: CardRecord = CardRecord::new(
    "Rejuvenating Springs",
    "51e69910-0d90-48a0-af29-3cddaeec5151",
    "Alayna Danner",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "Tap: Add either printed color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Green,
                ManaColor::Blue,
            ])),
        ),
    ]),
);

// CMR 356 — Spectator Seating
pub(in crate::card::sets) static SPECTATOR_SEATING_356: CardRecord = CardRecord::new(
    "Spectator Seating",
    "2f6f1453-fe93-4a29-965c-5f867a81e8b3",
    "Ravenna Tran",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "Tap: Add either printed color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Red,
                ManaColor::White,
            ])),
        ),
    ]),
);

// CMR 360 — Vault of Champions
pub(in crate::card::sets) static VAULT_OF_CHAMPIONS_360: CardRecord = CardRecord::new(
    "Vault of Champions",
    "0e144ae1-500d-4485-b476-5783b14380d9",
    "Cliff Childs",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "Tap: Add either printed color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Black,
            ])),
        ),
    ]),
);

// CMR 361 — War Room
// Audit: unsupported — Needs the number of colors in the controller's commanders' color identity as a life-payment value.
pub(in crate::card::sets) static WAR_ROOM_361: CardRecord = CardRecord::new(
    "War Room",
    "48d6ce7c-5dc8-449b-acbd-db259ae687ed",
    "Milivoj Ćeran",
    crate::card::CardRules::unsupported(),
);

// CMR 573 — Kediss, Emberclaw Familiar
// In the two-player engine, the damaged opponent has no other opponents to receive this damage.
pub(in crate::card::sets) static KEDISS_EMBERCLAW_FAMILIAR_573: CardRecord = CardRecord::new(
    "Kediss, Emberclaw Familiar",
    "23766fa0-e673-46c7-a29f-3fb140844b1c",
    "Jesper Ejsing",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Elemental", "Lizard"], 1, 1).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::triggered("Whenever a commander you control deals combat damage to an opponent, it deals that much damage to each other opponent.", TriggerEventDef::combat_damage_to_related_player(ObjectPredicateDef::All(&[ObjectPredicateDef::Commander, ObjectPredicateDef::ControlledBy(PlayerRelation::You)]), PlayerRelation::Opponent), EffectDef::None),
AbilityDef::deck_construction("Partner", DeckConstructionDef::Partner, "Both commanders are designated before the game.")
]),
);

// CMR 669 — Port Razer
// Audit: unsupported — Needs per-turn memory of which players this creature attacked, together with an additional-combat insertion.
pub(in crate::card::sets) static PORT_RAZER_669: CardRecord = CardRecord::new(
    "Port Razer",
    "77222431-9db0-4bc8-80be-7bfc7c48bc5e",
    "Craig J Spearing",
    crate::card::CardRules::unsupported(),
);

// CMR 713 — Training Center
pub(in crate::card::sets) static TRAINING_CENTER_713: CardRecord = CardRecord::new(
    "Training Center",
    "15a0efa5-6559-446b-a4d9-47585f6e94fb",
    "Daniel Ljunggren",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "Tap: Add either printed color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[ManaColor::Blue, ManaColor::Red])),
        ),
    ]),
);

// CMR 714 — Undergrowth Stadium
pub(in crate::card::sets) static UNDERGROWTH_STADIUM_714: CardRecord = CardRecord::new(
    "Undergrowth Stadium",
    "35bd7a70-0cd1-48f2-96b6-7869c003a8c7",
    "Yeong-Hao Han",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "Tap: Add either printed color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Green,
            ])),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AKROMA_S_WILL_3,
    &HULLBREACHER,
    &MALCOLM_KEEN_EYED_NAVIGATOR_79,
    &SAKASHIMA_OF_A_THOUSAND_FACES,
    &OPPOSITION_AGENT_141,
    &DARGO_THE_SHIPWRECKER_172,
    &HELLKITE_COURSER_183,
    &IMPULSIVE_PILFERER_185,
    &JESKA_S_WILL_187,
    &KRARK_THE_THUMBLESS_189,
    &ROGRAKH_SON_OF_ROHGAHH_197,
    &WHEEL_OF_MISFORTUNE_211,
    &ANNOYED_ALTISAUR,
    &APEX_DEVASTATOR_217,
    &COMMANDER_S_PLATE_305,
    &REJUVENATING_SPRINGS_354,
    &SPECTATOR_SEATING_356,
    &VAULT_OF_CHAMPIONS_360,
    &WAR_ROOM_361,
    &KEDISS_EMBERCLAW_FAMILIAR_573,
    &PORT_RAZER_669,
    &TRAINING_CENTER_713,
    &UNDERGROWTH_STADIUM_714,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
