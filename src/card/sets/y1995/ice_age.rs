//! Ice Age cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef,
    AppliedRuleDef, CardArt, CardRules, CardSet, CardType, EffectDef, EffectRecipientDef,
    InstalledTriggerDef, ManaColor, ObjectPredicateDef, PlayerRelation, ResolvedEffectDurationDef,
    TopCardSelectionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement,
    abilities, cards,
};
use crate::{TargetIndex, mana_cost};

// ICE 61 — Brainstorm
pub(in crate::card::sets) static BRAINSTORM: CardRecord = CardRecord::new(
    cards::BRAINSTORM,
    "Brainstorm",
    CardArt::new("8d42d7aa-7f53-4cfc-842a-086aab2448d1", "Christopher Rush"),
    CardSet::IceAge,
    // One mana, no card advantage, and the best blue card in the format:
    // what it buys is the top of the library, and a fetchland turns the two
    // cards put back into two cards nobody has to draw.
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell(
        "Draw three cards, then put two cards from your hand on top of your library in any \
             order.",
        abilities::brainstorm(),
    )),
);

// ICE 72 — Hydroblast
pub(in crate::card::sets) static HYDROBLAST: CardRecord = CardRecord::new(
    cards::HYDROBLAST,
    "Hydroblast",
    CardArt::new("f62716f0-fde2-49ef-b8a4-c1b03f451194", "Kaja Foglio"),
    CardSet::IceAge,
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::choose_one_spell(
        "Choose one —\n• Counter target spell if it's red.\n• Destroy target permanent if it's red.",
        &[
            AbilityDef::counter_target(
                "Counter target spell if it's red",
                &AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Color(ManaColor::Red)),
            ),
            AbilityDef::destroy_target(
                "Destroy target permanent if it's red",
                &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Color(
                    ManaColor::Red,
                )),
                true,
            ),
        ],
    )),
);

/// Three cards named in the order they go back, so the whole arrangement is
/// one decision rather than three. Every inspected card is selected, which is
/// what makes the choice an ordering rather than a filter.
static PORTENT_LOOK: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::Constant(3),
    object: None,
    minimum: 3,
    maximum: 3,
    select_all_matching: false,
    reveal_selected: false,
    selected_zone: ZoneKind::Library,
    selected_placement: ZonePlacement::Top,
    rest_zone: ZoneKind::Library,
    rest_placement: ZonePlacement::Top,
    selected_order_follows_choice: true,
    then: Some(&PORTENT_SHUFFLE_AND_DRAW),
    selected_face_down: None,
};

/// The shuffle comes after the look and is the caster's call: having seen the
/// three, you decide whether to leave them arranged or wash them away. The
/// draw is delayed a turn, which is the price the card pays for costing one.
static PORTENT_SHUFFLE_AND_DRAW: EffectDef = EffectDef::Sequence(&[
    EffectDef::May {
        player: EffectRecipientDef::Controller,
        effect: &EffectDef::ShuffleLibrary {
            player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    },
    EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
        "At the beginning of the next turn's upkeep, draw a card.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::Any,
        },
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    ))),
]);

static PORTENT_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Any),
)];

// ICE 90 — Portent
pub(in crate::card::sets) static PORTENT: CardRecord = CardRecord::new(
    cards::PORTENT,
    "Portent",
    CardArt::new("e040be83-3fb5-4da5-ba7a-4923b8854b74", "Liz Danforth"),
    CardSet::IceAge,
    CardRules::new_sorcery(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Look at the top three cards of target player's library, then put them back in any order. You may have that player shuffle.\nDraw a card at the beginning of the next turn's upkeep.",
        &PORTENT_TARGET,
        EffectDef::LookAtTopAndSelect {
            player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            looker: EffectRecipientDef::Controller,
            selection: &PORTENT_LOOK,
        },
    )),
);

// ICE 194 — Incinerate
pub(in crate::card::sets) static INCINERATE: CardRecord = CardRecord::new(
    cards::INCINERATE,
    "Incinerate",
    CardArt::new("9c3f00af-010d-4485-b8b7-47400d99c496", "Mark Poole"),
    CardSet::IceAge,
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Incinerate deals 3 damage to any target. A creature dealt damage this way can't be regenerated this turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamageAndApply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
                applied: AppliedEffectDef::Rule(AppliedRuleDef::CannotRegenerate),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ICE 213 — Pyroblast
pub(in crate::card::sets) static PYROBLAST: CardRecord = CardRecord::new(
    cards::PYROBLAST,
    "Pyroblast",
    CardArt::new("c342cac5-08ae-4428-9c2c-f6c5904e54d2", "Kaja Foglio"),
    CardSet::IceAge,
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::choose_one_spell(
        "Choose one —\n• Counter target spell if it's blue.\n• Destroy target permanent if it's blue.",
        &[
            AbilityDef::counter_target(
                "Counter target spell if it's blue",
                &AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Color(ManaColor::Blue)),
            ),
            AbilityDef::destroy_target(
                "Destroy target permanent if it's blue",
                &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Color(
                    ManaColor::Blue,
                )),
                true,
            ),
        ],
    )),
);

// ICE 350 — Zuran Orb
pub(in crate::card::sets) static ZURAN_ORB: CardRecord = CardRecord::new(
    cards::ZURAN_ORB,
    "Zuran Orb",
    CardArt::new("3a9d1082-a862-45d4-9e5e-392e879fead6", "Sandra Everingham"),
    CardSet::IceAge,
    CardRules::new_artifact(mana_cost!("{0}")).with_ability(AbilityDef::activated(
        "Sacrifice a land: You gain 2 life.",
        &[AbilityCostDef::SacrificePermanent {
            object: ObjectPredicateDef::HasType(CardType::Land),
            controller: PlayerRelation::You,
        }],
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(2),
        },
    )),
);

// ICE 351 — Adarkar Wastes
pub(in crate::card::sets) static ADARKAR_WASTES: CardRecord = CardRecord::new(
    cards::ADARKAR_WASTES,
    "Adarkar Wastes",
    CardArt::new("09dd9023-f7ee-4e99-8821-7059deb83730", "Mike Raabe"),
    CardSet::IceAge,
    CardRules::new_land(&[]).with_abilities(&abilities::pain_land(
        "{T}: Add {W} or {U}. This land deals 1 damage to you.",
        &[ManaColor::White, ManaColor::Blue],
    )),
);

// ICE 356 — Karplusan Forest
pub(in crate::card::sets) static KARPLUSAN_FOREST: CardRecord = CardRecord::new(
    cards::KARPLUSAN_FOREST,
    "Karplusan Forest",
    CardArt::new("ba6f1263-d598-49fb-b5f8-09f11822ebd0", "Nicola Leonard"),
    CardSet::IceAge,
    CardRules::new_land(&[]).with_abilities(&abilities::pain_land(
        "{T}: Add {R} or {G}. This land deals 1 damage to you.",
        &[ManaColor::Red, ManaColor::Green],
    )),
);

// ICE 362 — Underground River
pub(in crate::card::sets) static UNDERGROUND_RIVER: CardRecord = CardRecord::new(
    cards::UNDERGROUND_RIVER,
    "Underground River",
    CardArt::new("92369d7e-5e5a-46f9-bb31-c57d62410283", "NéNé Thomas"),
    CardSet::IceAge,
    CardRules::new_land(&[]).with_abilities(&abilities::pain_land(
        "{T}: Add {U} or {B}. This land deals 1 damage to you.",
        &[ManaColor::Blue, ManaColor::Black],
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &BRAINSTORM,
    &HYDROBLAST,
    &PORTENT,
    &INCINERATE,
    &PYROBLAST,
    &ZURAN_ORB,
    &ADARKAR_WASTES,
    &KARPLUSAN_FOREST,
    &UNDERGROUND_RIVER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
