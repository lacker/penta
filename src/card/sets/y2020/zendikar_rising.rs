//! Zendikar Rising cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef, AlternativeCastKindDef,
    CardArt, CardRules, CardSet, CardSupertype, CardType, ComparisonDef, ControlDurationDef,
    CounterKind, EffectDef, EffectRecipientDef, ManaColor, ObjectPredicateDef, ObjectQueryDef,
    ObjectRefDef, ObjectSetDef, PlayerRefDef, PlayerRelation, TokenStatsDef, TriggerConditionDef,
    TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, abilities,
};
use crate::{ObjectBindingIndex, ObjectSetBindingIndex, TargetIndex, mana_cost};

// ZNR 9 — Dauntless Unity
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DAUNTLESS_UNITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b12a4d17-68e6-4133-99fd-e501e24e6c6b"),
    "Dauntless Unity",
    crate::card::CardArt::new("b12a4d17-68e6-4133-99fd-e501e24e6c6b", "Josu Hernaiz"),
    crate::card::CardSet::ZendikarRising,
    crate::card::CardRules::unsupported(),
);

// ZNR 39 — Skyclave Apparition
/// Everything the exile clause excludes, in one predicate: a land is safe, a
/// token is safe, and anything expensive is safe. "You don't control" is the
/// controller half rather than part of the predicate.
static A_CHEAP_NONLAND_NONTOKEN: [AbilityTargetDef; 1] = [AbilityTargetDef::up_to(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
            ObjectPredicateDef::ManaValueAtMost(4),
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::Opponent),
        owner: None,
    },
    1,
)];

/// "Where X is the mana value of the exiled card": both halves read the same
/// card, which is the one the leave trigger just bound.
static ILLUSION_SIZE: TokenStatsDef = TokenStatsDef {
    power: ValueDef::ObjectManaValue(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
    toughness: ValueDef::ObjectManaValue(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
};

/// The token is the exiled card's owner's, not the Apparition controller's:
/// what they get back for the permanent that is not coming back.
static APPARITION_PAYS_FOR_IT: EffectDef =
    EffectDef::create_creature_token(&["Illusion"], &[ManaColor::Blue], 0, 0)
        .with_variable_token_stats(&ILLUSION_SIZE)
        .with_controller(PlayerRefDef::OwnerOf(ObjectRefDef::Binding(
            ObjectBindingIndex::PRIMARY,
        )));

/// One token per exiled card, which is one token: the exile clause is "up to
/// one target". Binding the pile is also what makes the clause do nothing at
/// all when nothing was exiled -- the Apparition that entered with no legal
/// target leaves without paying anybody.
static APPARITION_NAMES_WHAT_IT_TOOK: EffectDef = EffectDef::ForEachInBinding {
    objects: ObjectSetBindingIndex::PRIMARY,
    binding: ObjectBindingIndex::PRIMARY,
    effect: &APPARITION_PAYS_FOR_IT,
};

static APPARITION_LEAVES: EffectDef = EffectDef::BindMatching {
    objects: ObjectSetDef::LinkedExiles(ObjectPredicateDef::Any),
    binding: ObjectSetBindingIndex::PRIMARY,
    then: &APPARITION_NAMES_WHAT_IT_TOOK,
};

static SKYCLAVE_APPARITION_ABILITIES: [AbilityDef; 2] = [
    abilities::enters_trigger_with_targets(
        "When this creature enters, exile up to one target nonland, nontoken permanent you don't \
         control with mana value 4 or less.",
        &A_CHEAP_NONLAND_NONTOKEN,
        EffectDef::ExileLinkedToSource {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            then: None,
        },
    ),
    // Leaves, not dies: the card stays in exile whatever happened to the
    // Apparition, and the token is what its owner gets instead.
    AbilityDef::triggered(
        "When this creature leaves the battlefield, the exiled card's owner creates an X/X blue \
         Illusion creature token, where X is the mana value of the exiled card.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            Some(ZoneKind::Battlefield),
            None,
        ),
        APPARITION_LEAVES,
    ),
];

pub(in crate::card::sets) static SKYCLAVE_APPARITION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b83cfbaa-7890-4f6f-878b-4edb45677371"),
    "Skyclave Apparition",
    crate::card::CardArt::new("b83cfbaa-7890-4f6f-878b-4edb45677371", "Donato Giancola"),
    crate::card::CardSet::ZendikarRising,
    // Three mana for a body and an answer, and the answer is only undone by
    // killing the body -- which hands back an Illusion rather than the card.
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Kor", "Spirit"], 2, 2)
        .with_abilities(&SKYCLAVE_APPARITION_ABILITIES),
);

// ZNR 85 — Thieving Skydiver
/// "If it was kicked", asked as the arrival resolves. The kick is what the
/// whole card is: unkicked he is a 2/1 flier and nothing else happens.
static SKYDIVER_WAS_KICKED: TriggerConditionDef =
    TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked);

/// "Target artifact with mana value X or less", where X is what his own cast
/// paid: the target is sized by the kick rather than by anything printed.
static AN_ARTIFACT_WORTH_X: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Artifact),
        ObjectPredicateDef::ManaValueAtMostValue(ValueDef::SourceCastX),
    ]),
)];

static THAT_ARTIFACT_IS_AN_EQUIPMENT: TriggerConditionDef = TriggerConditionDef::TargetMatches {
    slot: TargetIndex::PRIMARY,
    object: ObjectPredicateDef::Subtype("Equipment"),
};

static SKYDIVER_EQUIPS_ITSELF: EffectDef = EffectDef::AttachToSource {
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
};

static SKYDIVER_STEALS: [EffectDef; 2] = [
    EffectDef::GainControl {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        controller: PlayerRefDef::EffectController,
        duration: ControlDurationDef::Indefinitely,
    },
    EffectDef::IfCondition {
        condition: &THAT_ARTIFACT_IS_AN_EQUIPMENT,
        then: &SKYDIVER_EQUIPS_ITSELF,
    },
];

pub(in crate::card::sets) static THIEVING_SKYDIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ff84ea71-e477-44f7-a3f8-77fef708efeb"),
    "Thieving Skydiver",
    CardArt::new("ff84ea71-e477-44f7-a3f8-77fef708efeb", "Kieran Yanner"),
    CardSet::ZendikarRising,
    // Two mana for a flier, or two plus X for a flier that takes the best
    // artifact on the board with it -- a Mox on turn three, a Sword on turn
    // five, and the Sword comes down already attached.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Merfolk", "Rogue"], 2, 1).with_abilities(&[
        AbilityDef::alternative_cast(
            mana_cost!("{X}{1}{U}"),
            AlternativeCastKindDef::Kicked,
            Some(
                "Kicker {X}. X can't be 0. (You may pay an additional {X} as you cast this \
                 spell.)",
            ),
            EffectDef::None,
        )
        .with_alternative_minimum_x(1),
        abilities::flying(),
        AbilityDef::triggered_if_with_targets(
            "When this creature enters, if it was kicked, gain control of target artifact with \
             mana value X or less. If that artifact is an Equipment, attach it to this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &SKYDIVER_WAS_KICKED,
            &AN_ARTIFACT_WORTH_X,
            EffectDef::Sequence(&SKYDIVER_STEALS),
        ),
    ]),
);

// ZNR 94 — Bloodchief's Thirst
static CREATURE_OR_PLANESWALKER: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::HasType(CardType::Planeswalker),
]);

/// The mana-value bound is part of what may be targeted rather than something
/// checked on resolution, so an unkicked Thirst never points at anything
/// bigger in the first place.
static THIRST_SMALL_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::All(&[
        CREATURE_OR_PLANESWALKER,
        ObjectPredicateDef::ManaValueAtMost(2),
    ]),
)];

static THIRST_ANY_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    CREATURE_OR_PLANESWALKER,
)];

static THIRST_DESTROY: EffectDef = EffectDef::Destroy {
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    can_regenerate: true,
    then: None,
};

pub(in crate::card::sets) static BLOODCHIEFS_THIRST: CardRecord = CardRecord::new_with_legacy_id(
    2165,
    "Bloodchief's Thirst",
    CardArt::new("059e8447-6b1c-4651-a734-a8fea2cbf7b2", "Jason Rainville"),
    CardSet::ZendikarRising,
    // One black kills most of what an aggressive deck leads with; four kills
    // whatever is left, which is why the card is played over a cheaper
    // removal spell that can only do the first job.
    CardRules::new_sorcery(mana_cost!("{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Kicker {2}{B} (You may pay an additional {2}{B} as you cast this spell.)\nDestroy target creature or planeswalker with mana value 2 or less.",
            &THIRST_SMALL_TARGET,
            THIRST_DESTROY,
        ),
        abilities::kicker(
            mana_cost!("{3}{B}"),
            "Destroy target creature or planeswalker.",
            &THIRST_ANY_TARGET,
            THIRST_DESTROY,
        ),
    ]),
);

// ZNR 185 — Gnarlid Colony
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GNARLID_COLONY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7327289d-eed8-44b1-8495-7172e2b49d5f"),
    "Gnarlid Colony",
    crate::card::CardArt::new("7327289d-eed8-44b1-8495-7172e2b49d5f", "Izzy"),
    crate::card::CardSet::ZendikarRising,
    crate::card::CardRules::unsupported(),
);

// ZNR 232 — Omnath, Locus of Creation
/// A land arriving under its controller. Landfall watches the battlefield
/// rather than the land drop, so a land put onto the battlefield by a fetch
/// or a search counts the same way one played from hand does.
static A_LAND_YOU_CONTROL_ENTERING: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Land),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

static PLANESWALKERS_YOU_DO_NOT_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Planeswalker),
    &[ZoneKind::Battlefield],
    PlayerRelation::NotYou,
);

const fn omnath_resolution(amount: u8) -> TriggerConditionDef {
    TriggerConditionDef::SourceResolutionsThisTurn {
        comparison: ComparisonDef::Equal,
        amount,
    }
}

/// The count includes the resolution asking, so the first time reads one.
static OMNATH_FIRST_TIME: TriggerConditionDef = omnath_resolution(1);

static OMNATH_SECOND_TIME: TriggerConditionDef = omnath_resolution(2);

static OMNATH_THIRD_TIME: TriggerConditionDef = omnath_resolution(3);

static OMNATH_GAINS_FOUR: EffectDef = EffectDef::GainLife {
    recipient: EffectRecipientDef::Controller,
    amount: ValueDef::Constant(4),
};

/// Four mana of four colours is four separate additions: what the pool ends
/// up holding is the same either way, and one `AddMana` names a run of like
/// units plus at most one other.
static OMNATH_ADDS_FOUR_COLORS: [EffectDef; 4] = [
    EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
    EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
    EffectDef::AddMana(AddManaEffectDef::one(ManaColor::White)),
    EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue)),
];

static OMNATH_ADDS_MANA: EffectDef = EffectDef::Sequence(&OMNATH_ADDS_FOUR_COLORS);

static OMNATH_BURNS: [EffectDef; 2] = [
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Opponent,
        amount: ValueDef::Constant(4),
    },
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
            PLANESWALKERS_YOU_DO_NOT_CONTROL,
        )),
        amount: ValueDef::Constant(4),
    },
];

static OMNATH_BURNS_EVERYTHING: EffectDef = EffectDef::Sequence(&OMNATH_BURNS);

/// Three exclusive branches on one count, so a fourth land does nothing at
/// all rather than repeating the third.
static OMNATH_LANDFALL: [EffectDef; 3] = [
    EffectDef::IfCondition {
        condition: &OMNATH_FIRST_TIME,
        then: &OMNATH_GAINS_FOUR,
    },
    EffectDef::IfCondition {
        condition: &OMNATH_SECOND_TIME,
        then: &OMNATH_ADDS_MANA,
    },
    EffectDef::IfCondition {
        condition: &OMNATH_THIRD_TIME,
        then: &OMNATH_BURNS_EVERYTHING,
    },
];

static OMNATH_ABILITIES: [AbilityDef; 2] = [
    abilities::enters_trigger(
        "When Omnath enters, draw a card.",
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    ),
    AbilityDef::triggered(
        "Landfall — Whenever a land you control enters, you gain 4 life if this is the first time \
         this ability has resolved this turn. If it's the second time, add {R}{G}{W}{U}. If it's \
         the third time, Omnath deals 4 damage to each opponent and each planeswalker you don't \
         control.",
        TriggerEventDef::zone_changed(
            A_LAND_YOU_CONTROL_ENTERING,
            None,
            Some(ZoneKind::Battlefield),
        ),
        EffectDef::Sequence(&OMNATH_LANDFALL),
    ),
];

pub(in crate::card::sets) static OMNATH_LOCUS_OF_CREATION: CardRecord =
    CardRecord::new_with_legacy_id(
        2264,
        "Omnath, Locus of Creation",
        CardArt::new("4e4fb50c-a81f-44d3-93c5-fa9a0b37f617", "Chris Rahn"),
        CardSet::ZendikarRising,
        // Four colours for a 4/4 that replaces itself, and a deck full of
        // fetchlands turns the third land of a turn into eight damage.
        CardRules::new_creature(mana_cost!("{R}{G}{W}{U}"), &["Elemental"], 4, 4)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&OMNATH_ABILITIES),
    );

// ZNR 319 — Luminarch Aspirant
/// "Target creature you control" -- including herself, which is what makes
/// an unanswered Aspirant a clock rather than a lord.
static A_CREATURE_YOU_CONTROL: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
    ]),
)];

pub(in crate::card::sets) static LUMINARCH_ASPIRANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ebe9427d-068f-487c-9263-b40366a164bc"),
    "Luminarch Aspirant",
    CardArt::new("ebe9427d-068f-487c-9263-b40366a164bc", "Mads Ahm"),
    CardSet::ZendikarRising,
    // Two mana that adds a counter every turn it survives, before attackers
    // are declared -- so the counter is already on whatever is about to
    // attack or block.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Cleric"], 1, 1).with_ability(
        AbilityDef::triggered_with_targets(
            "At the beginning of combat on your turn, put a +1/+1 counter on target creature you \
             control.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            &A_CREATURE_YOU_CONTROL,
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// ZNR 335 — Thieving Skydiver (alternate printing)

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &DAUNTLESS_UNITY,
    &SKYCLAVE_APPARITION,
    &THIEVING_SKYDIVER,
    &BLOODCHIEFS_THIRST,
    &GNARLID_COLONY,
    &OMNATH_LOCUS_OF_CREATION,
    &LUMINARCH_ASPIRANT,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::alternate(&THIEVING_SKYDIVER, 1), // ZNR 335
];
