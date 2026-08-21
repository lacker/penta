//! Invasion cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, CardArt, CardRules, CardSet, CardType, EffectDef,
    EffectRecipientDef, ManaColor, ObjectPredicateDef, ObjectRefDef, PlayerRelation,
    StackTargetKindDef, TopCardSelectionDef, TriggerConditionDef, TriggerEventDef, ValueDef,
    ZoneKind, ZonePlacement, abilities, cards,
};
use crate::{TargetIndex, mana_cost};

static FACT_OR_FICTION_PILE_MOVES: EffectDef = EffectDef::Sequence(&[
    EffectDef::MoveToZone {
        object: abilities::CHOSEN_PILE,
        zone: ZoneKind::Hand,
        placement: ZonePlacement::Top,
        arrival_effect: None,
        attachment: None,
        controller: None,
    },
    EffectDef::MoveToZone {
        object: abilities::UNCHOSEN_PILE,
        zone: ZoneKind::Graveyard,
        placement: ZonePlacement::Top,
        arrival_effect: None,
        attachment: None,
        controller: None,
    },
]);

// INV 57 — Fact or Fiction
pub(in crate::card::sets) static FACT_OR_FICTION: CardRecord = CardRecord::new(
    cards::FACT_OR_FICTION,
    "Fact or Fiction",
    CardArt::new(
        "7fd4d018-dcf3-4439-8445-02d66e44f7d3",
        "Terese Nielsen",
    ),
    CardSet::Invasion,
    CardRules::new_instant(mana_cost!("{3}{U}")).with_ability(AbilityDef::spell(
        "Reveal the top five cards of your library. An opponent separates those cards into two piles. Put one pile into your hand and the other into your graveyard.",
        abilities::split_top_of_library_into_piles(
            ValueDef::Constant(5),
            &FACT_OR_FICTION_PILE_MOVES,
        ),
    )),
);

static OPT_DRAW: EffectDef = EffectDef::DrawCards {
    recipient: EffectRecipientDef::Controller,
    amount: ValueDef::Constant(1),
};

static OPT_SELECTION: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::Constant(1),
    object: None,
    minimum: 0,
    maximum: 1,
    select_all_matching: false,
    reveal_selected: false,
    selected_zone: ZoneKind::Library,
    selected_placement: ZonePlacement::Bottom,
    rest_zone: ZoneKind::Library,
    rest_placement: ZonePlacement::Top,
    selected_order_follows_choice: false,
    then: Some(&OPT_DRAW),
    selected_face_down: None,
};

// INV 64 — Opt
pub(in crate::card::sets) static OPT: CardRecord = CardRecord::new(
    cards::OPT,
    "Opt",
    CardArt::new("958262ec-8e52-40cf-a9fd-a60e42643e15", "John Howe"),
    CardSet::Invasion,
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell(
        "Scry 1.\nDraw a card.",
        EffectDef::LookAtTopAndSelect {
            player: EffectRecipientDef::Controller,
            looker: EffectRecipientDef::Controller,
            selection: &OPT_SELECTION,
        },
    )),
);

/// Prohibit targets any spell and then asks how big it was, so a five-drop
/// can be named and simply survives. Both halves share the target; only the
/// ceiling moves.
static TARGET_SPELL: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::Spell,
        zones: &[ZoneKind::Stack],
        controller: None,
        owner: None,
    },
)];

static COUNTER_TARGET_SPELL: EffectDef = EffectDef::Counter {
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    zone: ZoneKind::Graveyard,
};

static SMALL_ENOUGH_TO_COUNTER: TriggerConditionDef = TriggerConditionDef::TargetMatches {
    slot: TargetIndex::PRIMARY,
    object: ObjectPredicateDef::ManaValueAtMost(2),
};

static BIG_ENOUGH_TO_COUNTER_KICKED: TriggerConditionDef = TriggerConditionDef::TargetMatches {
    slot: TargetIndex::PRIMARY,
    object: ObjectPredicateDef::ManaValueAtMost(4),
};

static PROHIBIT_UNKICKED: EffectDef = EffectDef::IfCondition {
    condition: &SMALL_ENOUGH_TO_COUNTER,
    then: &COUNTER_TARGET_SPELL,
};

static PROHIBIT_KICKED: EffectDef = EffectDef::IfCondition {
    condition: &BIG_ENOUGH_TO_COUNTER_KICKED,
    then: &COUNTER_TARGET_SPELL,
};

// INV 67 — Prohibit
pub(in crate::card::sets) static PROHIBIT: CardRecord = CardRecord::new(
    cards::PROHIBIT,
    "Prohibit",
    CardArt::new("0daa5458-2a97-40d0-b18d-2381a7a68ee1", "Adam Rex"),
    CardSet::Invasion,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Counter target spell if its mana value is 2 or less.",
            &TARGET_SPELL,
            PROHIBIT_UNKICKED,
        ),
        abilities::kicker(
            mana_cost!("{3}{U}"),
            "Counter target spell if its mana value is 4 or less.",
            &TARGET_SPELL,
            PROHIBIT_KICKED,
        ),
    ]),
);

static TARGET_ARTIFACT: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Artifact),
)];

static DESTROY_TARGET_ARTIFACT: EffectDef = EffectDef::Destroy {
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    can_regenerate: true,
};

static ARTIFACT_SMALL_ENOUGH: TriggerConditionDef = TriggerConditionDef::TargetMatches {
    slot: TargetIndex::PRIMARY,
    object: ObjectPredicateDef::ManaValueAtMost(2),
};

static ARTIFACT_SMALL_ENOUGH_KICKED: TriggerConditionDef = TriggerConditionDef::TargetMatches {
    slot: TargetIndex::PRIMARY,
    object: ObjectPredicateDef::ManaValueAtMost(5),
};

static OVERLOAD_UNKICKED: EffectDef = EffectDef::IfCondition {
    condition: &ARTIFACT_SMALL_ENOUGH,
    then: &DESTROY_TARGET_ARTIFACT,
};

static OVERLOAD_KICKED: EffectDef = EffectDef::IfCondition {
    condition: &ARTIFACT_SMALL_ENOUGH_KICKED,
    then: &DESTROY_TARGET_ARTIFACT,
};

/// A land you control, read off what the spell or ability already targets.
static A_LAND_YOU_CONTROL: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Land),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

static RESPONSE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::StackObject {
        object: ObjectPredicateDef::TargetsObjectMatching(&A_LAND_YOU_CONTROL),
        controller: Some(PlayerRelation::Opponent),
        kind: StackTargetKindDef::SpellOrAbility,
    },
)];

/// The destroy follows the counter rather than preceding it: the countered
/// ability is retired with its source recorded, so the permanent is still
/// findable afterwards, and a spell -- which has no such source -- leaves
/// nothing to destroy.
static RESPONSE_EFFECT: EffectDef = EffectDef::Sequence(&[
    EffectDef::Counter {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        zone: ZoneKind::Graveyard,
    },
    EffectDef::Destroy {
        object: EffectRecipientDef::object(ObjectRefDef::SourceOfTargetedStackObject(
            TargetIndex::PRIMARY,
        )),
        can_regenerate: true,
    },
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(2),
    },
]);

// INV 78 — Teferi's Response
pub(in crate::card::sets) static TEFERIS_RESPONSE: CardRecord = CardRecord::new(
    cards::TEFERIS_RESPONSE,
    "Teferi's Response",
    CardArt::new("f3bb2df8-8b6e-4f7c-9e9a-6c8b0f4b8e2d", "Scott Bailey"),
    CardSet::Invasion,
    // The answer to Wasteland and Dust Bowl: the land lives, the thing that
    // came for it dies, and two cards make the exchange worth a card.
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell or ability an opponent controls that targets a land you control. If a permanent's ability is countered this way, destroy that permanent.\nDraw two cards.",
        &RESPONSE_TARGET,
        RESPONSE_EFFECT,
    )),
);

// INV 157 — Overload
pub(in crate::card::sets) static OVERLOAD: CardRecord = CardRecord::new(
    cards::OVERLOAD,
    "Overload",
    CardArt::new("c91fca91-7296-422e-b251-d571b710ff71", "Gary Ruddell"),
    CardSet::Invasion,
    // One mana answers a Lotus Petal or a Cursed Scroll; three answers most
    // of what a Premodern deck actually plays.
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Destroy target artifact if its mana value is 2 or less.",
            &TARGET_ARTIFACT,
            OVERLOAD_UNKICKED,
        ),
        abilities::kicker(
            mana_cost!("{2}{R}"),
            "Destroy target artifact if its mana value is 5 or less.",
            &TARGET_ARTIFACT,
            OVERLOAD_KICKED,
        ),
    ]),
);

// INV 317 — Tsabo's Web
pub(in crate::card::sets) static TSABOS_WEB: CardRecord = CardRecord::new(
    cards::TSABOS_WEB,
    "Tsabo's Web",
    CardArt::new("0dee69f8-cceb-41b9-a0ee-6b2ac9f4bad9", "Carl Critchlow"),
    CardSet::Invasion,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::triggered(
            "When this artifact enters, draw a card.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::static_ability(
            "Each land with an activated ability that isn't a mana ability doesn't untap during its controller's untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasNonManaActivatedAbility,
                    ]), &[ZoneKind::Battlefield], PlayerRelation::Any),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
    ]),
);

// INV 321 — Coastal Tower
pub(in crate::card::sets) static COASTAL_TOWER: CardRecord = CardRecord::new(
    cards::COASTAL_TOWER,
    "Coastal Tower",
    CardArt::new("d115dbff-e35b-495f-a1e3-19651895927e", "Don Hazeltine"),
    CardSet::Invasion,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::activated_mana(
            "{T}: Add {W} or {U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Blue,
            ])),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &FACT_OR_FICTION,
    &OPT,
    &PROHIBIT,
    &TEFERIS_RESPONSE,
    &OVERLOAD,
    &TSABOS_WEB,
    &COASTAL_TOWER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
