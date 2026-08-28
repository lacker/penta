//! Final Fantasy cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::TargetIndex;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, ActivationTimingDef, AddManaEffectDef,
    AdditionalTriggerDef, AppliedEffectDef, AppliedRuleDef, BattlefieldEntryModificationDef,
    CardArt, CardRules, CardSet, CardSupertype, CardType, CharacteristicOperationDef, ConditionDef,
    CounterKind, CreatureTypeSetDef, DamageEventMatcherDef, DamageKindDef,
    DamageRecipientMatcherDef, DamageSourceMatcherDef, DrawEventMatcherDef, EffectDef,
    EffectRecipientDef, ManaColor, ObjectPredicateDef, ObjectRefDef, PlayActionMatcherDef,
    PlayRestrictionDef, PlayerRelation, PlayerSetDef, ReplacementEffectDef,
    ResolvedEffectDurationDef, SetOperationDef, TopOfLibraryCostDef, TriggerConditionDef,
    TriggerEventDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::mana_cost;

// FIN 91 — Cecil, Dark Knight // Cecil, Redeemed Paladin
/// The front half's payoff, and the reason the card is played: hitting hard
/// enough to halve your own life is what turns Cecil over. Untapping is part
/// of the same clause, so a Cecil that traded its attack for the transform
/// comes back ready to block.
static CECIL_TURNS_OVER: [EffectDef; 2] = [
    EffectDef::Untap {
        object: EffectRecipientDef::Source,
    },
    EffectDef::Transform {
        object: EffectRecipientDef::Source,
    },
];

static CECIL_TRANSFORM_CHECK: TriggerConditionDef =
    TriggerConditionDef::ControllerLifeAtMostHalfStartingLife;

static CECIL_TRANSFORMS: EffectDef = EffectDef::Sequence(&CECIL_TURNS_OVER);

/// "You lose that much life. Then if ..." is one clause resolving in order:
/// the life is lost first, so the very damage that cost it can be what brings
/// the total low enough to turn the card over.
static CECIL_DARKNESS: [EffectDef; 2] = [
    EffectDef::LoseLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::TriggerEventAmount,
    },
    EffectDef::IfCondition {
        condition: &CECIL_TRANSFORM_CHECK,
        then: &CECIL_TRANSFORMS,
    },
];

static CECIL_DARK_KNIGHT_ABILITIES: [AbilityDef; 2] = [
    abilities::deathtouch(),
    AbilityDef::triggered(
        "Darkness — Whenever Cecil deals damage, you lose that much life. Then if your life total is less than or equal to half your starting life total, untap Cecil and transform it.",
        TriggerEventDef::DamageDealt(DamageEventMatcherDef {
            kind: DamageKindDef::Any,
            source: DamageSourceMatcherDef::Object(ObjectRefDef::Source),
            recipient: DamageRecipientMatcherDef::Any,
        }),
        EffectDef::Sequence(&CECIL_DARKNESS),
    ),
];

const fn cecil_dark_knight_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{B}"), &["Human", "Knight"], 2, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&CECIL_DARK_KNIGHT_ABILITIES)
}

/// "Other attacking creatures" excludes Cecil and takes in the opponent's
/// too, on the rare turn both sides are attacking at once.
static OTHER_ATTACKING_CREATURES: EffectRecipientDef = EffectRecipientDef::matching_objects(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::Attacking,
        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
    ]),
    &[ZoneKind::Battlefield],
    PlayerRelation::Any,
);

static INDESTRUCTIBLE: AbilityDef = abilities::indestructible();

static CECIL_REDEEMED_PALADIN_ABILITIES: [AbilityDef; 2] = [
    abilities::lifelink(),
    AbilityDef::triggered(
        "Protect — Whenever Cecil attacks, other attacking creatures gain indestructible until end of turn.",
        TriggerEventDef::attacks(ObjectPredicateDef::Source),
        EffectDef::Apply {
            recipient: OTHER_ATTACKING_CREATURES,
            effect: AppliedEffectDef::add_ability(&INDESTRUCTIBLE),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    ),
];

/// The back face has no printed mana cost and is white, where the front is
/// black: transforming changes the colour it defends in.
const fn cecil_redeemed_paladin_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Human", "Knight"], 4, 4)
        .printed_colors(&[ManaColor::White])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&CECIL_REDEEMED_PALADIN_ABILITIES)
}

pub(in crate::card::sets) static CECIL_DARK_KNIGHT: CardRecord = CardRecord::new_dfc_with_legacy_id(
    2129,
    "Cecil, Dark Knight // Cecil, Redeemed Paladin",
    CardArt::new("026e7167-d665-43d0-a51e-8df2d68cdb5e", "Josu Hernaiz"),
    CardSet::FinalFantasy,
    &[
        ("Cecil, Dark Knight", cecil_dark_knight_rules()),
        ("Cecil, Redeemed Paladin", cecil_redeemed_paladin_rules()),
    ],
);

// FIN 114 — Resentful Revelation
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RESENTFUL_REVELATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("945006ea-c6a1-4ee5-abb2-387c2b6d3123"),
    "Resentful Revelation",
    crate::card::CardArt::new("945006ea-c6a1-4ee5-abb2-387c2b6d3123", "Justyna Dura"),
    crate::card::CardSet::FinalFantasy,
    crate::card::CardRules::unsupported(),
);

// FIN 164 — Suplex
static A_CREATURE: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Creature),
)];

static AN_ARTIFACT: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Artifact),
)];

/// The second sentence is about the creature, not about the damage: it is
/// applied to the target whether or not three damage was enough, or arrived
/// at all, so the two clauses resolve in order rather than as one linked
/// effect. A creature that shrugs the three off is still exiled if
/// something else finishes it before the turn ends.
static SUPLEX_SLAMS: EffectDef = EffectDef::Sequence(&[
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: ValueDef::Constant(3),
    },
    EffectDef::Apply {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        effect: AppliedEffectDef::Rule(AppliedRuleDef::ExileInsteadOfDying),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
]);

static SUPLEX_MODES: [AbilityDef; 2] = [
    AbilityDef::spell_with_targets(
        "Suplex deals 3 damage to target creature. If that creature would die this turn, exile it \
         instead.",
        &A_CREATURE,
        SUPLEX_SLAMS,
    ),
    AbilityDef::spell_with_targets(
        "Exile target artifact.",
        &AN_ARTIFACT,
        EffectDef::MoveToZone {
            counters: None,
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Exile,
            placement: ZonePlacement::Top,
            arrival_effect: None,
            attachment: None,
            controller: None,
            tapped: false,
        },
    ),
];

pub(in crate::card::sets) static SUPLEX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f61693a2-7042-44e0-85ba-9bf12ab94e7e"),
    "Suplex",
    CardArt::new("f61693a2-7042-44e0-85ba-9bf12ab94e7e", "Fang Xinyu"),
    CardSet::FinalFantasy,
    // Three damage that answers a recursive creature for good, or the
    // artifact half when there is nothing to throw.
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_ability(AbilityDef::choose_one_spell(
        "Choose one —\n• Suplex deals 3 damage to target creature. If that creature would die \
         this turn, exile it instead.\n• Exile target artifact.",
        &SUPLEX_MODES,
    )),
);

// FIN 206 — Tifa Lockhart
/// A land you control, not any land: the opponent's fetchland does nothing
/// for her.
static A_LAND_YOU_CONTROL: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Land),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

/// Doubling is +X/+0 where X is her power as this resolves, so two landfalls
/// in a turn compound: the second reads the size the first left behind.
static TIFA_DOUBLES: [AbilityDef; 2] = [
    abilities::trample(),
    AbilityDef::triggered(
        "Landfall — Whenever a land you control enters, double Tifa Lockhart's power until end of turn.",
        TriggerEventDef::zone_changed(A_LAND_YOU_CONTROL, None, Some(ZoneKind::Battlefield)),
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::SourcePower,
                ValueDef::Constant(0),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    ),
];

pub(in crate::card::sets) static TIFA_LOCKHART: CardRecord = CardRecord::new_with_legacy_id(
    2146,
    "Tifa Lockhart",
    CardArt::new("fb781323-2746-405d-a9b2-e778c037a6e9", "Laurel Austin"),
    CardSet::FinalFantasy,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Monk"], 1, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&TIFA_DOUBLES),
);

// FIN 248 — Vivi Ornitier
/// "Add X mana in any combination of {U} and/or {R}" divides one amount
/// across two types, so the runtime offers the ability once per division.
/// Vivi enters with no power at all, so the first activation worth making
/// comes after a noncreature spell has grown it.
static VIVI_MANA: AddManaEffectDef =
    AddManaEffectDef::combination(&VIVI_COLORS, 0).with_variable_amount(ValueDef::SourcePower);

static VIVI_COLORS: [ManaColor; 2] = [ManaColor::Blue, ManaColor::Red];

static VIVI_COST: [AbilityCostDef; 1] = [AbilityCostDef::Mana(mana_cost!("{0}"))];

/// The counter and the damage are one clause, and the counter comes first --
/// so a Vivi that has just been cast at is already bigger by the time its own
/// mana ability is next offered.
static VIVI_PAYOFF: [EffectDef; 2] = [
    EffectDef::AddCounters {
        object: EffectRecipientDef::Source,
        kind: CounterKind::PlusOnePlusOne,
        amount: ValueDef::Constant(1),
    },
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Opponent,
        amount: ValueDef::Constant(1),
    },
];

static VIVI_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::activated_mana(
        "{0}: Add X mana in any combination of {U} and/or {R}, where X is this creature's power. Activate only during your turn and only once each turn.",
        &VIVI_COST,
        EffectDef::AddMana(VIVI_MANA),
    )
    .with_activation_timing(ActivationTimingDef::YourTurn)
    .activations_each_turn(1),
    AbilityDef::triggered(
        "Whenever you cast a noncreature spell, put a +1/+1 counter on this creature and it deals 1 damage to each opponent.",
        TriggerEventDef::SpellCast(ObjectPredicateDef::All(&[
            ObjectPredicateDef::NoncreatureSpell,
            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
        ])),
        EffectDef::Sequence(&VIVI_PAYOFF),
    ),
];

pub(in crate::card::sets) static VIVI_ORNITIER: CardRecord = CardRecord::new_with_legacy_id(
    2162,
    "Vivi Ornitier",
    CardArt::new("ecc1027a-8c07-44a0-bdde-fa2844cff694", "Toni Infante"),
    CardSet::FinalFantasy,
    CardRules::new_creature(mana_cost!("{1}{U}{R}"), &["Wizard"], 0, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&VIVI_ABILITIES),
);

// FIN 289 — Starting Town
static TOWN_PAYS_A_LIFE: [AbilityCostDef; 2] =
    [AbilityCostDef::TapSource, AbilityCostDef::PayLife(1)];

static STARTING_TOWN_ABILITIES: [AbilityDef; 3] = [
    // "Your first, second, or third turn of the game" counts the turns you
    // have taken rather than the turn number: on the draw, your third turn
    // is the game's sixth, and the Town still comes in untapped.
    AbilityDef::as_enters(
        "This land enters tapped unless it's your first, second, or third turn of the game.",
        ReplacementEffectDef::Conditional {
            condition: ConditionDef::ControllerTurnsTakenAtMost(3),
            if_true: &[],
            if_false: &TOWN_ENTERS_TAPPED,
        },
    ),
    abilities::tap_for(ManaColor::Colorless),
    AbilityDef::activated_mana(
        "{T}, Pay 1 life: Add one mana of any color.",
        &TOWN_PAYS_A_LIFE,
        EffectDef::AddMana(AddManaEffectDef::any_color()),
    ),
];

static TOWN_ENTERS_TAPPED: [ReplacementEffectDef; 1] =
    [ReplacementEffectDef::ModifyBattlefieldEntry(
        BattlefieldEntryModificationDef::Tapped,
    )];

pub(in crate::card::sets) static STARTING_TOWN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fc7d1912-7e27-49ef-bd98-375d975a42b0"),
    "Starting Town",
    CardArt::new("fc7d1912-7e27-49ef-bd98-375d975a42b0", "Hristo D. Chukov"),
    CardSet::FinalFantasy,
    // A City of Brass for the turns that matter and a tapped land after
    // them, which is the trade a deck makes for fixing it only needs early.
    CardRules::new_land(&["Town"]).with_abilities(&STARTING_TOWN_ABILITIES),
);

// FIN 551c — Traveling Chocobo
/// Two permissions rather than one: the printed sentence names two kinds of
/// play, and the restriction each carries is a single action and a single
/// predicate. Lands cost nothing beyond the land drop; a Bird pays its own
/// mana cost, since nothing here says otherwise.
static CHOCOBO_PLAYS_LANDS: PlayRestrictionDef = PlayRestrictionDef::new(
    PlayActionMatcherDef::PlayLand,
    ObjectPredicateDef::HasType(CardType::Land),
);

static CHOCOBO_CASTS_BIRDS: PlayRestrictionDef = PlayRestrictionDef::new(
    PlayActionMatcherDef::CastSpell,
    ObjectPredicateDef::Subtype("Bird"),
);

static CHOCOBO_TOP_OF_LIBRARY: [AppliedEffectDef; 2] = [
    AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromTopOfLibrary {
        restriction: CHOCOBO_PLAYS_LANDS,
        cost: TopOfLibraryCostDef::Printed,
    }),
    AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromTopOfLibrary {
        restriction: CHOCOBO_CASTS_BIRDS,
        cost: TopOfLibraryCostDef::Printed,
    }),
];

/// A land or a Bird, and yours either way.
static CHOCOBO_ARRIVAL: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::AnyOf(&[
        ObjectPredicateDef::HasType(CardType::Land),
        ObjectPredicateDef::Subtype("Bird"),
    ]),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

static CHOCOBO_DOUBLES_TRIGGERS: AdditionalTriggerDef = AdditionalTriggerDef {
    entering: CHOCOBO_ARRIVAL,
    permanent: ObjectPredicateDef::ControlledBy(PlayerRelation::You),
};

static TRAVELING_CHOCOBO_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::static_ability(
        "You may look at the top card of your library any time.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::You)),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::MayLookAtTopOfLibrary),
        },
    ),
    AbilityDef::static_ability(
        "You may play lands and cast Bird spells from the top of your library.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::You)),
            effect: AppliedEffectDef::Composite(&CHOCOBO_TOP_OF_LIBRARY),
        },
    ),
    // The Chocobo itself is a Bird, so a second one doubles the first one's
    // arrival trigger -- and two of them double everything twice.
    AbilityDef::static_ability(
        "If a land or Bird you control entering the battlefield causes a triggered ability of a \
         permanent you control to trigger, that ability triggers an additional time.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::You)),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::TriggersAnAdditionalTime(
                &CHOCOBO_DOUBLES_TRIGGERS,
            )),
        },
    ),
];

pub(in crate::card::sets) static TRAVELING_CHOCOBO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("156cfd45-1556-4804-becf-039cfff7de3d"),
    "Traveling Chocobo",
    crate::card::CardArt::new("156cfd45-1556-4804-becf-039cfff7de3d", "Toni Infante"),
    crate::card::CardSet::FinalFantasy,
    // Three mana for a body, a land engine, and a Panharmonicon that only
    // reads lands and its own kind -- which in a deck built for it is most
    // of what enters.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Bird"], 3, 2)
        .with_abilities(&TRAVELING_CHOCOBO_ABILITIES),
);

// FIN 581 — Astrologian's Planisphere
/// Two events, one clause, one counter each: a noncreature spell, and the
/// third card of the turn however it was drawn. The Hero's own draw step
/// counts toward the third, which is why the card wants a turn with two
/// cantrips in it rather than a big draw spell.
static PLANISPHERE_TRIGGERS: [TriggerEventDef; 2] = [
    TriggerEventDef::SpellCast(ObjectPredicateDef::All(&[
        ObjectPredicateDef::NoncreatureSpell,
        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
    ])),
    TriggerEventDef::DrewCard(DrawEventMatcherDef::nth_each_turn(PlayerRelation::You, 3)),
];

/// Granted to the equipped creature, so "this creature" is the creature
/// rather than the Equipment: the counter goes where the ability lives.
static PLANISPHERE_GRANTED: AbilityDef = AbilityDef::triggered(
    "Whenever you cast a noncreature spell and whenever you draw your third card each turn, put \
     a +1/+1 counter on this creature.",
    TriggerEventDef::AnyOf(&PLANISPHERE_TRIGGERS),
    EffectDef::AddCounters {
        object: EffectRecipientDef::Source,
        kind: CounterKind::PlusOnePlusOne,
        amount: ValueDef::Constant(1),
    },
);

static PLANISPHERE_GRANT: [AppliedEffectDef; 2] = [
    AppliedEffectDef::Characteristic(CharacteristicOperationDef::CreatureTypes(
        SetOperationDef::Add(CreatureTypeSetDef::named(&["Wizard"])),
    )),
    AppliedEffectDef::add_ability(&PLANISPHERE_GRANTED),
];

static PLANISPHERE_EQUIP: [AbilityCostDef; 1] = [AbilityCostDef::Mana(mana_cost!("{2}"))];

static ASTROLOGIAN_S_PLANISPHERE_ABILITIES: [AbilityDef; 3] = [
    abilities::job_select(),
    AbilityDef::static_ability(
        "Equipped creature is a Wizard in addition to its other types and has \"Whenever you \
         cast a noncreature spell and whenever you draw your third card each turn, put a +1/+1 \
         counter on this creature.\"",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::AttachedPermanent,
            effect: AppliedEffectDef::Composite(&PLANISPHERE_GRANT),
        },
    ),
    // The flavour name in front of the cost is the whole of what "Diana —"
    // adds: it is an ordinary equip ability underneath.
    abilities::equip(&PLANISPHERE_EQUIP, "Diana — Equip {2}"),
];

pub(in crate::card::sets) static ASTROLOGIAN_S_PLANISPHERE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a0f6e2d7-58b5-4a7d-8c42-e25185cd173f"),
    "Astrologian's Planisphere",
    crate::card::CardArt::new("a0f6e2d7-58b5-4a7d-8c42-e25185cd173f", "Josephine Chang"),
    crate::card::CardSet::FinalFantasy,
    // Two mana for a 1/1 that grows on the turns a blue deck was having
    // anyway, and an Equipment left over when it dies.
    CardRules::new_artifact(mana_cost!("{1}{U}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&ASTROLOGIAN_S_PLANISPHERE_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &CECIL_DARK_KNIGHT,
    &RESENTFUL_REVELATION,
    &SUPLEX,
    &TIFA_LOCKHART,
    &VIVI_ORNITIER,
    &STARTING_TOWN,
    &TRAVELING_CHOCOBO,
    &ASTROLOGIAN_S_PLANISPHERE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
