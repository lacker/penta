//! Weatherlight cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::cards;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, BattlefieldEntryModificationDef, CardArt, CardRules, CardSet,
    CardType, CounterKind, EffectDef, EffectPaymentDef, EffectRecipientDef, HalvedValueDef,
    ManaColor, ObjectPredicateDef, ObjectSetDef, PayOrDef, PlayActionMatcherDef,
    PlayRestrictionDef, PlayerRefDef, PlayerRelation, PlayerSetDef, ReplacementEffectDef,
    ResolvedEffectDurationDef, RoundingDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement,
};
use crate::{TargetIndex, mana_cost};

/// The tax names spells an opponent casts, so it never touches your own.
static OPPONENTS_ARTIFACTS_AND_ENCHANTMENTS: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasType(CardType::Artifact),
    ObjectPredicateDef::HasType(CardType::Enchantment),
]);

/// Both halves of the same lock, applied to the same player for the same
/// turn: no instants or sorceries, and no activations but mana abilities.
static ABEYANCE_LOCK: [AppliedEffectDef; 2] = [
    AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(PlayRestrictionDef::new(
        PlayActionMatcherDef::CastSpell,
        ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Instant),
            ObjectPredicateDef::HasType(CardType::Sorcery),
        ]),
    ))),
    AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(PlayRestrictionDef::new(
        PlayActionMatcherDef::ActivateNonManaAbility,
        ObjectPredicateDef::Any,
    ))),
];

// WTH 1 — Abeyance
pub(in crate::card::sets) static ABEYANCE: CardRecord = CardRecord::new(
    cards::ABEYANCE,
    "Abeyance",
    CardArt::new("efb452f0-c019-4409-bfb1-600a97d58fdd", "Thomas Gianni"),
    CardSet::Weatherlight,
    // A counterspell that replaces itself and stops the next one too: the
    // deck holding it is buying one turn without interaction.
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Until end of turn, target player can't cast instant or sorcery spells, and that player can't activate abilities that aren't mana abilities.\nDraw a card.",
        &ABEYANCE_TARGET,
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::target_players(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&ABEYANCE_LOCK),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

static ABEYANCE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Any),
)];

// WTH 7 — Aura of Silence
pub(in crate::card::sets) static AURA_OF_SILENCE: CardRecord = CardRecord::new(
    cards::AURA_OF_SILENCE,
    "Aura of Silence",
    CardArt::new(
        "57e6c366-b8c7-4f66-b8e1-82dc69c0081c",
        "D. Alexander Gregory",
    ),
    CardSet::Weatherlight,
    // It taxes while it sits and answers something on the way out, so the
    // opponent pays either way.
    CardRules::new_enchantment(mana_cost!("{1}{W}{W}")).with_abilities(&[
        AbilityDef::static_ability(
            "Artifact and enchantment spells your opponents cast cost {2} more to cast.",
            EffectDef::IncreaseMatchingSpellCostBy {
                spell: OPPONENTS_ARTIFACTS_AND_ENCHANTMENTS,
                caster: PlayerRelation::Opponent,
                amount: mana_cost!("{2}"),
            },
        ),
        AbilityDef::activated_with_targets(
            "Sacrifice this enchantment: Destroy target artifact or enchantment.",
            &[AbilityCostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                OPPONENTS_ARTIFACTS_AND_ENCHANTMENTS,
            )],
            EffectDef::destroy_target(TargetIndex::PRIMARY, true),
        ),
    ]),
);

/// The artifact has to belong to the player being attacked, which in a
/// two-player game is the only opponent there is.
static DEFENDERS_ARTIFACT: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Artifact),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::Opponent),
        owner: None,
    },
)];

/// Paying trades the hit for the artifact: the Vandal connects, and then
/// deals nothing because it spent the swing breaking something instead.
static VANDAL_TRADE: EffectDef = EffectDef::Sequence(&[
    EffectDef::Destroy {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        can_regenerate: true,
    },
    EffectDef::Apply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::Rule(AppliedRuleDef::AssignsNoCombatDamage),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
]);

/// Half the life you have, rounded up: at twenty that is ten, and the deck
/// casting this intends to win before losing the other ten.
static DOOMSDAY_LIFE: HalvedValueDef =
    HalvedValueDef::new(ValueDef::LifeTotal(PlayerRelation::You), RoundingDef::Up);

/// The search and the life are one clause resolving in order, and the order
/// matters: the five cards are chosen while the library still exists.
static DOOMSDAY_STEPS: [EffectDef; 2] = [
    EffectDef::SearchZonesAndExileRest {
        player: EffectRecipientDef::Controller,
        zones: &DOOMSDAY_ZONES,
        count: 5,
    },
    EffectDef::LoseLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Halved(&DOOMSDAY_LIFE),
    },
];

static DOOMSDAY_ZONES: [ZoneKind; 2] = [ZoneKind::Library, ZoneKind::Graveyard];

// WTH 66 — Doomsday
pub(in crate::card::sets) static DOOMSDAY: CardRecord = CardRecord::new(
    cards::DOOMSDAY,
    "Doomsday",
    CardArt::new("5b3c6d87-9383-450b-bba5-33435b6b0d08", "Adrian Smith"),
    CardSet::Weatherlight,
    // A five-card library you built yourself, and half your life for it. The
    // deck that plays it is not trying to survive the exile -- it is trying
    // to draw the five cards it just stacked and win on the spot.
    CardRules::new_sorcery(mana_cost!("{B}{B}{B}")).with_ability(AbilityDef::spell(
        "Search your library and graveyard for five cards and exile the rest. Put the chosen cards on top of your library in any order. You lose half your life, rounded up.",
        EffectDef::Sequence(&DOOMSDAY_STEPS),
    )),
);

// WTH 105 — Goblin Vandal
pub(in crate::card::sets) static GOBLIN_VANDAL: CardRecord = CardRecord::new(
    cards::GOBLIN_VANDAL,
    "Goblin Vandal",
    CardArt::new("b7ad3b81-f706-4b33-b1ec-7600182a5232", "Franz Vohwinkel"),
    CardSet::Weatherlight,
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin", "Rogue"], 1, 1).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks and isn't blocked, you may pay {R}. If you do, destroy target artifact defending player controls and this creature assigns no combat damage this turn.",
            TriggerEventDef::AttacksAndIsNotBlocked {
                attacker: ObjectPredicateDef::Source,
            },
            &DEFENDERS_ARTIFACT,
            EffectDef::PayOr(PayOrDef::optional(
                EffectPaymentDef::mana(
                    PlayerSetDef::Related(PlayerRelation::You),
                    mana_cost!("{R}"),
                ),
                &VANDAL_TRADE,
            )),
        ),
    ),
);

/// Any card in any graveyard, which is what the sacrifice mode reaches. The
/// tap mode needs no target beyond the player, because a graveyard has only
/// one bottom card.
static A_CARD_IN_A_GRAVEYARD: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::Any,
        zones: &[ZoneKind::Graveyard],
        controller: None,
        owner: None,
    },
)];

static A_PLAYER: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Any),
)];

static FURNACE_EXILE_AND_DRAW: EffectDef = EffectDef::Sequence(&[
    EffectDef::MoveToZone {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        zone: ZoneKind::Exile,
        controller: None,
        placement: ZonePlacement::Top,
        arrival_effect: None,
        attachment: None,
    },
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
]);

// WTH 153 — Mind Stone
pub(in crate::card::sets) static MIND_STONE: CardRecord = CardRecord::new(
    cards::MIND_STONE,
    "Mind Stone",
    CardArt::new("162e81d3-6cd4-4cb8-8ed8-cfbd8d34ca71", "Adam Rex"),
    CardSet::Weatherlight,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {C}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
        AbilityDef::activated(
            "{1}, {T}, Sacrifice this artifact: Draw a card.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
            ],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// WTH 154 — Null Rod
pub(in crate::card::sets) static NULL_ROD: CardRecord = CardRecord::new(
    cards::NULL_ROD,
    "Null Rod",
    CardArt::new("bc45f2cb-c256-4a0f-879a-c7db5b1a0b94", "Anson Maddocks"),
    CardSet::Weatherlight,
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::static_ability(
        "Activated abilities of artifacts can't be activated.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Artifact),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotActivateAbilities),
        },
    )),
);

// WTH 155 — Phyrexian Furnace
pub(in crate::card::sets) static PHYREXIAN_FURNACE: CardRecord = CardRecord::new(
    cards::PHYREXIAN_FURNACE,
    "Phyrexian Furnace",
    CardArt::new("e98bca31-a1f4-4d9e-bbb8-fd9b6f4d2b91", "George Pratt"),
    CardSet::Weatherlight,
    // The tap mode eats a graveyard from the bottom, one card a turn; the
    // sacrifice mode answers the one card that actually mattered.
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: Exile the bottom card of target player's graveyard.",
            &[AbilityCostDef::TapSource],
            &A_PLAYER,
            EffectDef::MoveToZone {
                object: EffectRecipientDef::objects(ObjectSetDef::BottomOfGraveyard(
                    PlayerRefDef::Target(TargetIndex::PRIMARY),
                )),
                zone: ZoneKind::Exile,
                controller: None,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
            },
        ),
        AbilityDef::activated_with_targets(
            "{1}, Sacrifice this artifact: Exile target card from a graveyard. Draw a card.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::SacrificeSource,
            ],
            &A_CARD_IN_A_GRAVEYARD,
            FURNACE_EXILE_AND_DRAW,
        ),
    ]),
);

// WTH 164 — Gemstone Mine
pub(in crate::card::sets) static GEMSTONE_MINE: CardRecord = CardRecord::new(
    cards::GEMSTONE_MINE,
    "Gemstone Mine",
    CardArt::new("09507f7f-c58f-4f57-b878-b39811a5b619", "Brom"),
    CardSet::Weatherlight,
    // Three activations of perfect mana, and then nothing: the deck that
    // plays four of these is buying the first three turns, not the tenth.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters with three mining counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::Mining,
                    amount: 3,
                },
            ),
        ),
        AbilityDef::activated_mana(
            "{T}, Remove a mining counter from this land: Add one mana of any color. If there are no mining counters on this land, sacrifice it.",
            &GEMSTONE_MINE_COSTS,
            EffectDef::AddMana(
                AddManaEffectDef::any_color().sacrificing_source_when_out_of(CounterKind::Mining),
            ),
        ),
    ]),
);

static GEMSTONE_MINE_COSTS: [AbilityCostDef; 2] = [
    AbilityCostDef::TapSource,
    AbilityCostDef::RemoveCountersFromSource {
        kind: CounterKind::Mining,
        amount: 1,
    },
];

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ABEYANCE,
    &AURA_OF_SILENCE,
    &DOOMSDAY,
    &GOBLIN_VANDAL,
    &MIND_STONE,
    &NULL_ROD,
    &PHYREXIAN_FURNACE,
    &GEMSTONE_MINE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
