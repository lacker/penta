use std::borrow::Cow;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::ops::ControlFlow;

use crate::Format;
use crate::action::{
    AbilityOrigin, Action, ActionError, CombatDamageAssignment, ManaColor, Target,
};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityImplementationDef, AbilityTargetDef, AbilityTargetPredicate,
    ActivatedAbilityDef, AddManaEffectDef, AlternativeCastAbilityDef, AlternativeCastKindDef,
    AppliedEffectDef, BasicLandType, CREATURE_TYPES, CardBehavior, CardCatalog, CardDefinition,
    CardEffectStatus, CardPart, CardRules, CardSet, CardStructure, CardSupertype, CardType,
    CardTypeSet, CharacteristicContext, ComparisonDef, CounterKind, DeclarativeAbilityDef,
    DoubleFacedKind, EffectDef, EffectDurationDef, EffectRecipientDef, KeywordAbility, LandEntry,
    ManaCost, ManaRestrictionDef, ManaSelectionDef, ManaSpendEffectDef, ObjectPredicateDef,
    ObjectQueryDef, PlayActionKind, PlayOptionDef, PlayRestriction, PlayerRelation,
    ReplacementEventDef, SpellForm, TargetPredicate, TargetSlotDef, TriggerConditionDef,
    TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZoneMoveCauseDef, abilities,
    applicable_part_ids,
};
use crate::casting::{CastChoices, CastSignature, CostConfiguration, TargetSelection};
use crate::deck::{Deck, DeckError, ValidatedDeck};
use crate::ids::{
    AbilityId, AdditionalCostId, AlternativeCostId, CardDefinitionId, CardPartId, GameObjectId,
    GrantId, MeldRecipeId, ModeId, PhysicalCardId, PlayOptionId, PlayerId, TargetSlotId,
};
use crate::rng::ReplayRng;
#[cfg(test)]
use crate::rules;

mod decision;
mod event;
mod mana;
mod observation;

pub use decision::{
    DecisionKind, DecisionObservation, DecisionOption, DecisionOrderSemantics, DecisionPreference,
    DecisionVisibility, DecisionZone,
};
pub use event::{BattlefieldExit, GameEvent, GameResult, StackObjectKind, Step, WinReason};
pub use mana::{Mana, ManaPool, ManaSource};
pub use observation::{
    PermanentObservation, PlayerObservation, StackObservation, ZoneCard, ZoneError,
};

use observation::{LastSeenHand, PublicCard};

#[derive(Clone, Debug, Eq, PartialEq)]
struct PhysicalCard {
    id: PhysicalCardId,
    definition: CardDefinitionId,
    owner: PlayerId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ObjectBacking {
    Cards(Vec<PhysicalCardId>),
    None,
}

/// Which of the possible combat-damage steps is currently being processed.
///
/// The public protocol deliberately exposes both strike waves as
/// [`Step::CombatDamage`]. Keeping the first wave's participants here lets the
/// second wave follow the dynamic first strike/double strike eligibility rules
/// without changing that protocol.
#[derive(Clone, Debug, Eq, PartialEq)]
enum CombatDamageStage {
    NotStarted,
    Single,
    FirstStrike {
        strike_wave_combatants: Vec<GameObjectId>,
    },
    RegularAfterFirstStrike {
        strike_wave_combatants: Vec<GameObjectId>,
    },
}

/// Where this object's copiable characteristics come from. This deliberately
/// does not follow physical backing: a copy can have characteristics with no
/// card, while a future meld result can be backed by two cards without being
/// the printed definition of either one.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(dead_code)]
enum CharacteristicSource {
    Card(CardDefinitionId),
    Copy(CardDefinitionId),
    Ability(CardDefinitionId),
    Meld(MeldRecipeId),
}

/// Links an object that changed zones to the new object or objects created by
/// that move. Current mechanics create one object; the vector also supports a
/// future melded object separating into multiple cards.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ZoneChangeOutcome {
    pub previous: GameObjectId,
    pub created: Vec<GameObjectId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CardInstance {
    id: GameObjectId,
    definition: CardDefinitionId,
    owner: PlayerId,
    backing: ObjectBacking,
    characteristics: CharacteristicSource,
}

/// One indefinite text-changing effect in layer 3. These effects belong to
/// the object, are applied in timestamp order, and are deliberately excluded
/// from its copiable values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BasicLandTypeChange {
    from: BasicLandType,
    to: BasicLandType,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LandTypeOperation {
    SetTo(BasicLandType),
    Add(&'static [BasicLandType]),
}

/// An ability added as an exception while copying an object. Unlike an
/// ordinary granted ability, this becomes part of the resulting object's
/// copiable values and can therefore be copied again.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CopiableAbility {
    origin: AbilityOrigin,
    definition: AbilityDef,
}

/// The compact copiable-value snapshot needed by the copy effects currently
/// supported by the engine. The catalog source supplies all ordinary printed
/// characteristics; copy-process exceptions are frozen beside it.
#[derive(Clone, Debug, Eq, PartialEq)]
struct CopiableCharacteristics {
    base: (CardDefinitionId, CardPartId),
    added_types: CardTypeSet,
    added_abilities: Vec<CopiableAbility>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
struct Permanent {
    card: CardInstance,
    /// The logical part currently supplying this permanent's printed
    /// characteristics. Transforming changes this without changing object ID.
    presented: CardPartId,
    controller: PlayerId,
    tapped: bool,
    entered_controller_turn: u32,
    damage: u16,
    /// Loyalty counters are distinct from marked creature damage and persist
    /// across cleanup. Planeswalker abilities remain deferred, but supported
    /// damage spells can already target and remove a planeswalker correctly.
    loyalty: Option<i16>,
    power_bonus: i16,
    toughness_bonus: i16,
    attacking: bool,
    /// Whether nothing may block this creature for the rest of the turn.
    /// Cleared in cleanup with the other until-end-of-turn state.
    unblockable_this_turn: bool,
    /// Whether this attacker was blocked. A blocked creature stays blocked
    /// even if every blocker leaves, so this cannot be recomputed from the
    /// blockers still on the battlefield.
    blocked: bool,
    blocking: Option<GameObjectId>,
    chosen_player: Option<PlayerId>,
    chosen_creature_type: Option<String>,
    destroy_at_end: bool,
    temporary_keywords: Vec<KeywordAbility>,
    factory_animated: bool,
    dragon_whelp_activations: u8,
    /// Every kind of counter this permanent carries, indexed by
    /// [`CounterKind::index`]. Only +1/+1 counters have rules meaning on their
    /// own; the rest are markers the cards that place them interpret.
    counters: [u16; CounterKind::COUNT],
    /// What this Aura is attached to. `None` for everything that is not an
    /// Aura. State-based actions put it into its owner's graveyard if the
    /// referenced host leaves or stops being legal.
    attached_to: Option<GameObjectId>,
    /// Set by Pillar of Flame: if this creature would die this turn, it is
    /// exiled instead. The replacement outlives the damage itself, so it
    /// cannot be a property of the damage. Clears in cleanup.
    exile_instead_of_dying: bool,
    combat_damage_assignment: Vec<CombatDamageAssignment>,
    /// Values established by the most recent copy effect. This is a frozen
    /// snapshot rather than a live pointer to the target, so later changes to
    /// that object cannot leak through and copy chains preserve exceptions.
    copy_effect: Option<CopiableCharacteristics>,
    /// Whether this permanent entered as a copy. Transforming double-faced
    /// cards use this to distinguish their own back face from a copied one
    /// when determining mana value.
    copied_from: Option<(CardDefinitionId, CardPartId)>,
    /// Indefinite text changes applied to this object in timestamp order.
    text_changes: Vec<BasicLandTypeChange>,
    regeneration_shields: u8,
    berserked: bool,
    attacked_this_turn: bool,
    forestwalk_until_upkeep_of: Option<PlayerId>,
    /// Sources that dealt damage to this permanent during the current turn.
    /// IDs deliberately refer to the damaging object incarnation so a later
    /// death trigger can use the live source or its retired LKI snapshot.
    damage_sources: Vec<GameObjectId>,
    /// Whether any damage still marked on this permanent came from a source
    /// with deathtouch. The source may leave before state-based actions are
    /// checked, so this is damage-event state rather than a live lookup.
    deathtouch_damage: bool,
}

impl Permanent {
    /// A permanent as it arrives on the battlefield, before any card-specific
    /// adjustments. Three call sites used to spell out every field, which made
    /// adding one a three-place edit and gave a new entry path nothing to
    /// build on.
    fn entering(
        card: CardInstance,
        presented: CardPartId,
        controller: PlayerId,
        entered_controller_turn: u32,
    ) -> Self {
        Self {
            card,
            presented,
            controller,
            tapped: false,
            entered_controller_turn,
            damage: 0,
            loyalty: None,
            power_bonus: 0,
            toughness_bonus: 0,
            attacking: false,
            unblockable_this_turn: false,
            blocked: false,
            blocking: None,
            chosen_player: None,
            chosen_creature_type: None,
            destroy_at_end: false,
            temporary_keywords: Vec::new(),
            factory_animated: false,
            dragon_whelp_activations: 0,
            counters: [0; CounterKind::COUNT],
            attached_to: None,
            exile_instead_of_dying: false,
            combat_damage_assignment: Vec::new(),
            copy_effect: None,
            copied_from: None,
            text_changes: Vec::new(),
            regeneration_shields: 0,
            berserked: false,
            attacked_this_turn: false,
            forestwalk_until_upkeep_of: None,
            damage_sources: Vec::new(),
            deathtouch_damage: false,
        }
    }

    const fn counters(&self, kind: CounterKind) -> u16 {
        self.counters[kind.index()]
    }

    const fn set_counters(&mut self, kind: CounterKind, amount: u16) {
        self.counters[kind.index()] = amount;
    }

    const fn add_counters(&mut self, kind: CounterKind, amount: u16) {
        let index = kind.index();
        self.counters[index] = self.counters[index].saturating_add(amount);
    }

    const fn remove_counter(&mut self, kind: CounterKind) {
        let index = kind.index();
        self.counters[index] = self.counters[index].saturating_sub(1);
    }
}

/// An effect queued for the next time a step begins. Whatever queued it has
/// usually left by then, so the entry carries its own source and controller.
#[derive(Clone, Debug, Eq, PartialEq)]
struct DelayedTrigger {
    /// The object that queued this, kept whole so the effect resolves with
    /// the same source and controller it would have had at the time.
    object: Box<StackObject>,
    step: TurnStepDef,
    player: PlayerRelation,
    effect: &'static EffectDef,
}

/// A retired object incarnation retained for last-known-information queries.
/// Zone changes still create a new [`GameObjectId`]; this record deliberately
/// never follows the physical card into its new zone.
#[derive(Clone, Debug, Eq, PartialEq)]
enum RetiredObject {
    Card(CardInstance),
    Permanent {
        permanent: Box<Permanent>,
        power: Option<i16>,
        toughness: Option<i16>,
        keywords: Vec<KeywordAbility>,
    },
    Stack(Box<StackObject>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct StackObject {
    id: GameObjectId,
    kind: StackObjectKind,
    card: CardInstance,
    /// The permanent object whose ability this is. Spell objects have no
    /// source; their `card` is the stack incarnation itself.
    source: Option<GameObjectId>,
    /// The complete executable ability captured when this object was put on
    /// the stack. The origin remains useful provenance, but resolution never
    /// uses it to rediscover rules from a source that may since have changed.
    ability: Option<StackAbilityPayload>,
    controller: PlayerId,
    /// Present exactly for spell objects. This freezes form, modes, costs, X,
    /// and target-slot bindings for resolution and copy effects.
    signature: Option<CastSignature>,
    chosen_permanents: Vec<GameObjectId>,
    /// Effects carried by mana used to pay for this object. They are attached
    /// before the spell is finalized on the stack and retain their source.
    applied_effects: Vec<AppliedStackEffect>,
    /// Indefinite text changes applied while this object is on the stack.
    /// They transfer to a resolving permanent but are not copied by spell-copy
    /// effects.
    text_changes: Vec<BasicLandTypeChange>,
    /// Flashback replaces every destination this physical card would use when
    /// leaving the stack. This is frozen at cast time because the permission
    /// lived on the previous graveyard object.
    cast_via_flashback: bool,
    is_copy: bool,
}

/// The immutable rules payload of an activated or triggered ability on the
/// stack. `origin` describes where the ability came from; the remaining fields
/// are the authoritative frozen characteristics used for presentation,
/// target legality, and resolution.
#[derive(Clone, Debug, Eq, PartialEq)]
struct StackAbilityPayload {
    origin: AbilityOrigin,
    /// The complete activated or spell ability as it existed when this object
    /// was put on the stack. Copy effects that retain the resolving ability
    /// need its costs and targets as copiable values, not only its resolver;
    /// triggered payloads do not currently need this optional snapshot.
    definition: Option<Box<AbilityDef>>,
    presentation_definition: CardDefinitionId,
    text: Option<&'static str>,
    target_defs: Vec<AbilityTargetDef>,
    targets: Vec<TargetSelection>,
    context: TriggerContext,
    resolver: StackAbilityResolver,
    /// The intervening-if condition, re-read as this ability resolves.
    condition: Option<&'static TriggerConditionDef>,
    /// Selected declarative mode effects frozen in canonical printed order.
    /// Repeated modes remain repeated procedures.
    mode_effects: Vec<EffectDef>,
    /// The X chosen when the ability was activated, so its effects read the
    /// same number the cost was paid for.
    x: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum StackAbilityResolver {
    Declarative(EffectDef),
    DeclarativeWithCustomFollowup {
        effect: EffectDef,
        behavior: CardBehavior,
    },
    Custom(CardBehavior),
}

/// Ordered stack-zone storage. The top is the final element, while removal by
/// index permits effects such as a counterspell to remove any targeted stack
/// object without disturbing the relative order of objects above or below it.
/// Stack order is therefore structural and never inferred from object IDs.
#[derive(Clone, Debug, Default)]
struct GameStack {
    objects: Vec<StackObject>,
}

impl GameStack {
    fn push(&mut self, object: StackObject) {
        self.objects.push(object);
    }

    fn pop(&mut self) -> Option<StackObject> {
        self.objects.pop()
    }

    fn remove(&mut self, index: usize) -> StackObject {
        self.objects.remove(index)
    }

    #[cfg(test)]
    fn clear(&mut self) {
        self.objects.clear();
    }

    fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.objects.len()
    }

    #[cfg(test)]
    fn last(&self) -> Option<&StackObject> {
        self.objects.last()
    }

    fn iter(&self) -> std::slice::Iter<'_, StackObject> {
        self.objects.iter()
    }
}

impl std::ops::Index<usize> for GameStack {
    type Output = StackObject;

    fn index(&self, index: usize) -> &Self::Output {
        &self.objects[index]
    }
}

impl std::ops::IndexMut<usize> for GameStack {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.objects[index]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AppliedStackEffect {
    source: Option<ManaSource>,
    effect: crate::AppliedEffectDef,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TriggerContext {
    object: Option<GameObjectId>,
    object_controller: Option<PlayerId>,
    event_player: Option<PlayerId>,
    amount: Option<i32>,
}

impl TriggerContext {
    const fn empty() -> Self {
        Self {
            object: None,
            object_controller: None,
            event_player: None,
            amount: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TriggerEventObject {
    id: GameObjectId,
    types: CardTypeSet,
    controller: PlayerId,
    colors: [bool; 5],
    subtypes: Cow<'static, [&'static str]>,
    mana_value: u16,
    /// Current power where one exists: a battlefield creature reports what it
    /// is now, not what it was printed as.
    power: Option<i16>,
    supertypes: [bool; CardSupertype::COUNT],
    /// Whether this object is in combat. Cheap to carry and it cannot feed
    /// back into a characteristic, unlike a keyword or a static bonus.
    attacking_or_blocking: bool,
    /// Printed and temporary keywords, as a bitmask over
    /// [`KeywordAbility::simple_index`].
    ///
    /// A keyword granted by a static effect is deliberately missing, for the
    /// same reason power is: resolving static effects matches their sources
    /// against these characteristics, so reading a granted keyword back here
    /// would not terminate.
    keywords: u32,
    /// Whether this creature is attacking, excluding a creature that is only
    /// blocking. Bloodrush and similar predicates need the narrower state.
    attacking: bool,
}

/// The object or procedure a mana payment is paying for. Restrictions are
/// evaluated against this frozen purpose both while planning mana abilities
/// and when selecting the exact mana units to spend.
#[derive(Clone, Debug, Eq, PartialEq)]
enum ManaPaymentPurpose {
    Spell {
        object: GameObjectId,
        definition: CardDefinitionId,
        controller: PlayerId,
        form: SpellForm,
    },
    Ability {
        source: GameObjectId,
        /// Whether the ability taps its source to pay for itself. When it
        /// does, that source cannot also be tapped for mana, so it is barred
        /// from the payment rather than merely deprioritised.
        taps_source: bool,
    },
    Other,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum CommittedTriggerEvent {
    ZoneChanged {
        object: TriggerEventObject,
        from: ZoneKind,
        to: ZoneKind,
    },
    BecomesTapped {
        object: TriggerEventObject,
    },
    LifeGained {
        player: PlayerId,
        amount: u16,
    },
    Attacks {
        object: TriggerEventObject,
    },
    TappedForMana {
        object: TriggerEventObject,
    },
    DamageDealt {
        object: TriggerEventObject,
        amount: u16,
    },
    SpellCast {
        object: TriggerEventObject,
    },
    StepBegins {
        step: TurnStepDef,
        player: PlayerId,
    },
    DamagedCreatureDied {
        object: TriggerEventObject,
        source: GameObjectId,
    },
}

impl CommittedTriggerEvent {
    fn context(&self) -> TriggerContext {
        match self {
            Self::ZoneChanged { object, .. }
            | Self::BecomesTapped { object }
            | Self::Attacks { object }
            | Self::DamagedCreatureDied { object, .. } => TriggerContext {
                object: Some(object.id),
                object_controller: Some(object.controller),
                event_player: None,
                amount: None,
            },
            Self::DamageDealt { object, amount } => TriggerContext {
                object: Some(object.id),
                object_controller: Some(object.controller),
                event_player: None,
                amount: Some(i32::from(*amount)),
            },
            Self::LifeGained { player, amount } => TriggerContext {
                object: None,
                object_controller: None,
                event_player: Some(*player),
                amount: Some(i32::from(*amount)),
            },
            // The player who tapped a permanent for mana is its controller,
            // which is the same shape a cast spell has.
            Self::TappedForMana { object } | Self::SpellCast { object } => TriggerContext {
                object: Some(object.id),
                object_controller: Some(object.controller),
                event_player: Some(object.controller),
                amount: None,
            },
            Self::StepBegins { player, .. } => TriggerContext {
                object: None,
                object_controller: None,
                event_player: Some(*player),
                amount: None,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AbilitySourceRef {
    object: GameObjectId,
    ability: AbilityOrigin,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PendingTrigger {
    id: u32,
    source: AbilitySourceRef,
    definition: CardDefinitionId,
    owner: PlayerId,
    controller: PlayerId,
    text: &'static str,
    target_defs: &'static [AbilityTargetDef],
    targets: Vec<TargetSelection>,
    effect: EffectDef,
    resolver: StackAbilityResolver,
    context: TriggerContext,
    condition: Option<&'static TriggerConditionDef>,
}

/// The immutable declaration captured when one event matches one source
/// ability. The game assigns the ephemeral trigger ID when it accepts this
/// record into the pending-trigger queue.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TriggerCapture {
    source: AbilitySourceRef,
    definition: CardDefinitionId,
    owner: PlayerId,
    controller: PlayerId,
    text: &'static str,
    target_defs: &'static [AbilityTargetDef],
    effect: EffectDef,
    resolver: StackAbilityResolver,
    context: TriggerContext,
    /// The intervening-if condition this trigger reads, checked both when the
    /// ability would go on the stack and again when it resolves.
    condition: Option<&'static TriggerConditionDef>,
}

/// One battlefield trigger listener frozen at the start of an atomic event.
/// A simultaneous zone change can remove the source before another object in
/// the same event is published, so listener discovery cannot consult the
/// incrementally-mutated battlefield.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BattlefieldTriggerListener {
    event: TriggerEventDef,
    uses_stack: bool,
    capture: TriggerCapture,
}

#[derive(Clone, Debug)]
struct TriggerPlacementBatch {
    controller: PlayerId,
    triggers: Vec<PendingTrigger>,
}

impl StackObject {
    fn iter_targets(&self) -> impl Iterator<Item = &Target> {
        self.signature
            .iter()
            .flat_map(CastSignature::iter_targets)
            .chain(
                self.ability
                    .iter()
                    .filter(|_| self.signature.is_none())
                    .flat_map(|ability| ability.targets.iter())
                    .flat_map(TargetSelection::targets),
            )
    }

    fn ability_origin(&self) -> Option<AbilityOrigin> {
        self.ability.as_ref().map(|ability| ability.origin)
    }

    fn ability_text(&self) -> Option<&'static str> {
        self.ability.as_ref().and_then(|ability| ability.text)
    }

    fn presentation_definition(&self) -> CardDefinitionId {
        self.ability
            .as_ref()
            .map_or(self.card.definition, |ability| {
                ability.presentation_definition
            })
    }

    fn targets(&self) -> Vec<Target> {
        self.iter_targets().copied().collect()
    }

    fn first_target(&self) -> Option<Target> {
        self.iter_targets().next().copied()
    }

    fn target_count(&self) -> usize {
        self.iter_targets().count()
    }

    fn x(&self) -> u16 {
        self.signature.as_ref().map_or_else(
            || self.ability.as_ref().map_or(0, |ability| ability.x),
            CastSignature::x,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PlayerState {
    life: i16,
    library: Vec<CardInstance>,
    hand: Vec<CardInstance>,
    graveyard: Vec<CardInstance>,
    exile: Vec<CardInstance>,
    mana_pool: ManaPool,
    mana: Vec<Mana>,
    land_played_this_turn: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct EffectiveAbility {
    origin: AbilityOrigin,
    ability: AbilityDef,
}

/// An ability granted to one non-battlefield object until cleanup. The object
/// identity naturally makes the grant end if that card changes zones.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TemporaryAbilityGrant {
    object: GameObjectId,
    ability: &'static AbilityDef,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct StaticAppliedEffect {
    source: GameObjectId,
    source_definition: CardDefinitionId,
    source_part: CardPartId,
    source_ability: AbilityId,
    grant: Option<GrantId>,
    effect: AppliedEffectDef,
}

struct StaticEffectTraversal<'a> {
    source: &'a Permanent,
    source_definition: CardDefinitionId,
    source_part: CardPartId,
    source_ability: AbilityId,
    affected: &'a Permanent,
    next_grant: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PermanentLastKnownInformation {
    power: Option<i16>,
    toughness: Option<i16>,
    keywords: Vec<KeywordAbility>,
}

/// Characteristics and abilities frozen immediately before a permanent exits
/// the battlefield. Every member of a simultaneous exit batch is snapshotted
/// before any member is removed.
#[derive(Clone, Debug, Eq, PartialEq)]
struct BattlefieldExitSnapshot {
    object: TriggerEventObject,
    abilities: Vec<EffectiveAbility>,
    last_known: PermanentLastKnownInformation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FrozenActivatedAbility {
    origin: AbilityOrigin,
    definition: Option<Box<AbilityDef>>,
    presentation_definition: CardDefinitionId,
    text: Option<&'static str>,
    target_defs: &'static [AbilityTargetDef],
    resolver: StackAbilityResolver,
    /// The X chosen at activation, frozen alongside everything else the
    /// ability will resolve with.
    x: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CastSourceZone {
    Hand,
    Graveyard,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ManaAbilityActivation {
    source: GameObjectId,
    ability: AbilityOrigin,
    color: ManaColor,
    costs: crate::card::AbilityCostList,
    effect: AddManaEffectDef,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PlannedManaActivation {
    source: GameObjectId,
    ability: AbilityOrigin,
    color: ManaColor,
    production: ManaPool,
    benefits_payment: bool,
    flexibility: usize,
    order: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FlexibleManaSource {
    source: GameObjectId,
    outputs: Vec<(AbilityOrigin, ManaColor, ManaPool, bool)>,
    order: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Pregame {
    Mulligan(PlayerId),
    Bottom(PlayerId),
}

#[derive(Clone, Debug)]
struct PendingDecision {
    observation: DecisionObservation,
    continuation: DecisionContinuation,
}

#[derive(Clone, Copy, Debug)]
enum BalanceAction {
    Sacrifice,
    Discard,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BalancePhase {
    Lands,
    Hands,
    Creatures,
}

impl BalancePhase {
    const fn next(self) -> Option<Self> {
        match self {
            Self::Lands => Some(Self::Hands),
            Self::Hands => Some(Self::Creatures),
            Self::Creatures => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ZoneMoveCause {
    Rules,
    Cost,
    Effect { controller: PlayerId },
}

#[derive(Clone, Debug)]
struct BalanceTask {
    player: PlayerId,
    prompt: String,
    zone: DecisionZone,
    cards: Vec<CardInstance>,
    count: usize,
    action: BalanceAction,
    cause: ZoneMoveCause,
}

/// Where a countered spell ends up.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CounteredSpellZone {
    Graveyard,
    Exile,
}

#[derive(Clone, Debug)]
enum DecisionContinuation {
    Tutor,
    BasicLandTypeTextChange {
        target: Target,
    },
    OptionalManaPayment {
        player: PlayerId,
        cost: ManaCost,
        object: Box<StackObject>,
        context: TriggerContext,
        effect: &'static EffectDef,
    },
    ChainLightning {
        player: PlayerId,
        spell: StackObject,
        targets: Vec<Target>,
    },
    Fork {
        player: PlayerId,
        spell: StackObject,
        target_lists: Vec<Vec<TargetSelection>>,
    },
    ManaVault {
        player: PlayerId,
        permanent: GameObjectId,
    },
    RecallCost {
        player: PlayerId,
        card: GameObjectId,
        choices: CastChoices,
    },
    RecallReturn {
        player: PlayerId,
    },
    Duress {
        victim: PlayerId,
        cause: ZoneMoveCause,
    },
    /// An effect the controller was offered and may decline.
    OptionalEffect {
        object: Box<StackObject>,
        context: TriggerContext,
        effect: &'static EffectDef,
    },
    /// A sacrifice an effect demanded, chosen by the sacrificing player.
    SacrificeOfChoice,
    /// The spell's controller deciding whether to keep it alive.
    CounterUnlessPaid {
        spell: GameObjectId,
        player: PlayerId,
        cost: ManaCost,
        zone: CounteredSpellZone,
    },
    /// A discard an effect demanded, chosen by the discarding player.
    DiscardToEffect {
        player: PlayerId,
        cause: ZoneMoveCause,
    },
    /// Holds the revealed cards while the caster decides which to keep; they
    /// have already left the library, so the continuation must place them all.
    GrislySalvage {
        player: PlayerId,
        revealed: Vec<CardInstance>,
    },
    Balance {
        controller: PlayerId,
        phase: BalancePhase,
        task: BalanceTask,
        remaining: Vec<BalanceTask>,
    },
    TimeVault {
        permanent: GameObjectId,
        remaining: Vec<GameObjectId>,
    },
    SylvanSelect {
        player: PlayerId,
        candidates: Vec<GameObjectId>,
        choices_left: usize,
    },
    SylvanMode {
        player: PlayerId,
        card: GameObjectId,
        candidates: Vec<GameObjectId>,
        choices_left: usize,
    },
    ErhnamForestwalk {
        player: PlayerId,
        source: GameObjectId,
    },
    /// Sin Collector and Lifebane Zombie, holding the hand they exile from.
    ExileFromHand {
        victim: PlayerId,
    },
    /// Augur of Bolas holding the three cards it looked at; they have already
    /// left the library, so the continuation must place all of them.
    AugurOfBolas {
        player: PlayerId,
        revealed: Vec<CardInstance>,
    },
    /// A shock land asking whether to pay life to come in untapped.
    ShockLand {
        player: PlayerId,
        permanent: GameObjectId,
        life: u8,
    },
    ChooseCreatureType {
        player: PlayerId,
        permanent: Box<Permanent>,
        entry: LandEntry,
        choices: Vec<String>,
    },
    TriggerOrder {
        batch: TriggerPlacementBatch,
        remaining: Vec<TriggerPlacementBatch>,
    },
    TriggerPlacement {
        trigger: PendingTrigger,
        pending: Vec<PendingTrigger>,
        remaining: Vec<TriggerPlacementBatch>,
        candidates: Vec<Target>,
    },
}

#[derive(Clone, Debug)]
#[allow(clippy::struct_excessive_bools)]
pub struct Game {
    format: Format,
    seed: u64,
    rng: ReplayRng,
    catalog: CardCatalog,
    #[allow(dead_code)] // Reserved for backing validation and future meld actions.
    physical_cards: Vec<PhysicalCard>,
    players: [PlayerState; 2],
    battlefield: Vec<Permanent>,
    stack: GameStack,
    retired_objects: BTreeMap<GameObjectId, RetiredObject>,
    /// Abilities granted to non-battlefield object incarnations until cleanup.
    temporary_ability_grants: Vec<TemporaryAbilityGrant>,
    next_object_id: u32,
    turn: u32,
    turns_started: [u32; 2],
    active_player: PlayerId,
    priority: PlayerId,
    consecutive_passes: u8,
    step: Step,
    attackers_declared: bool,
    /// Whether a creature has died so far this turn, for morbid. Cleared as a
    /// turn begins rather than in cleanup, so a morbid spell cast during the
    /// end step still sees the creature that died in combat.
    creature_died_this_turn: bool,
    /// Cards exiled by an object that promises to bring them back, paired
    /// with whatever exiled them. Oblivion Ring is the shape.
    linked_exiles: Vec<(GameObjectId, GameObjectId)>,
    /// How many of each player's next sorceries may be cast as though they
    /// had flash. Quicken grants one, and the grant lapses with the turn.
    sorcery_flash_grants: [u8; 2],
    /// Effects waiting for a step to begin. Obzedat's return is one.
    delayed_triggers: Vec<DelayedTrigger>,
    blockers_declared: bool,
    untap_pending: bool,
    pregame: Option<Pregame>,
    mulligans: [u8; 2],
    cleanup_pending: bool,
    pending_decisions: Vec<PendingDecision>,
    next_decision_id: u32,
    pending_triggers: Vec<PendingTrigger>,
    next_trigger_id: u32,
    last_seen_hands: [LastSeenHand; 2],
    pending_combat_attackers: Vec<GameObjectId>,
    combat_damage_stage: CombatDamageStage,
    combat_blocked_attackers: Vec<GameObjectId>,
    extra_turns: Vec<PlayerId>,
    mana_drain_pending: [u16; 2],
    channel_active: [bool; 2],
    skipped_turns: [u16; 2],
    result: Option<GameResult>,
    events: Vec<GameEvent>,
}

impl Game {
    /// Creates a game, shuffles both decks, and draws opening hands.
    ///
    /// Player one takes the first turn and skips that turn's draw. Mulligans
    /// are not yet part of this constructor.
    ///
    /// # Errors
    ///
    /// Returns [`GameError`] if a deck references a card absent from the
    /// supplied catalog, card instance IDs are exhausted, or a deck cannot
    /// supply an opening hand.
    pub fn new(catalog: CardCatalog, decks: [Deck; 2], seed: u64) -> Result<Self, GameError> {
        Self::new_with_format(Format::OldSchool9394, catalog, decks, seed)
    }

    /// Creates a game using the construction and gameplay rules of `format`.
    ///
    /// # Errors
    ///
    /// Returns [`GameError`] if a deck is illegal in the selected format,
    /// references a card absent from the supplied catalog, exhausts card
    /// instance IDs, or cannot supply an opening hand.
    #[allow(clippy::too_many_lines)]
    pub fn new_with_format(
        format: Format,
        catalog: CardCatalog,
        decks: [Deck; 2],
        seed: u64,
    ) -> Result<Self, GameError> {
        let mut rng = ReplayRng::new(seed);
        let mut next_physical_id = 0_u32;
        let mut next_object_id = 0_u32;
        let mut physical_cards = Vec::new();
        let [deck_one, deck_two] = decks;
        let deck_one = deck_one
            .validate_for_format(&catalog, format)
            .map_err(|error| GameError::InvalidDeck {
                player: PlayerId::One,
                error,
            })?;
        let deck_two = deck_two
            .validate_for_format(&catalog, format)
            .map_err(|error| GameError::InvalidDeck {
                player: PlayerId::Two,
                error,
            })?;

        let format_rules = format.rules();

        let mut build_player =
            |player: PlayerId, deck: ValidatedDeck| -> Result<PlayerState, GameError> {
                let definitions = deck.into_main();
                let mut library = Vec::with_capacity(definitions.len());
                for definition in definitions {
                    let physical_id = PhysicalCardId(next_physical_id);
                    next_physical_id = next_physical_id
                        .checked_add(1)
                        .ok_or(GameError::TooManyCards)?;
                    let object_id = GameObjectId(next_object_id);
                    next_object_id = next_object_id
                        .checked_add(1)
                        .ok_or(GameError::TooManyCards)?;
                    physical_cards.push(PhysicalCard {
                        id: physical_id,
                        definition,
                        owner: player,
                    });
                    library.push(CardInstance {
                        id: object_id,
                        definition,
                        owner: player,
                        backing: ObjectBacking::Cards(vec![physical_id]),
                        characteristics: CharacteristicSource::Card(definition),
                    });
                }
                rng.shuffle(&mut library);
                let initial_hand = draw_opening_hand(&mut library, format_rules.opening_hand_size)?;
                let mut hand = Vec::with_capacity(initial_hand.len());
                for mut card in initial_hand {
                    card.id = GameObjectId(next_object_id);
                    next_object_id = next_object_id
                        .checked_add(1)
                        .ok_or(GameError::TooManyCards)?;
                    hand.push(card);
                }
                Ok(PlayerState {
                    life: i16::from(format_rules.starting_life),
                    library,
                    hand,
                    graveyard: Vec::new(),
                    exile: Vec::new(),
                    mana_pool: ManaPool::default(),
                    mana: Vec::new(),
                    land_played_this_turn: false,
                })
            };

        let players = [
            build_player(PlayerId::One, deck_one)?,
            build_player(PlayerId::Two, deck_two)?,
        ];

        Ok(Self {
            format,
            seed,
            rng,
            catalog,
            physical_cards,
            players,
            battlefield: Vec::new(),
            stack: GameStack::default(),
            retired_objects: BTreeMap::new(),
            temporary_ability_grants: Vec::new(),
            next_object_id,
            turn: 1,
            turns_started: [1, 0],
            active_player: PlayerId::One,
            priority: PlayerId::One,
            consecutive_passes: 0,
            step: Step::Upkeep,
            attackers_declared: false,
            creature_died_this_turn: false,
            linked_exiles: Vec::new(),
            sorcery_flash_grants: [0; 2],
            delayed_triggers: Vec::new(),
            blockers_declared: false,
            untap_pending: false,
            pregame: Some(Pregame::Mulligan(PlayerId::One)),
            mulligans: [0, 0],
            cleanup_pending: false,
            pending_decisions: Vec::new(),
            next_decision_id: 0,
            pending_triggers: Vec::new(),
            next_trigger_id: 0,
            last_seen_hands: [None, None],
            pending_combat_attackers: Vec::new(),
            combat_damage_stage: CombatDamageStage::NotStarted,
            combat_blocked_attackers: Vec::new(),
            extra_turns: Vec::new(),
            mana_drain_pending: [0, 0],
            channel_active: [false, false],
            skipped_turns: [0, 0],
            result: None,
            events: vec![GameEvent::GameStarted { seed }],
        })
    }

    #[must_use]
    pub const fn format(&self) -> Format {
        self.format
    }

    #[must_use]
    pub const fn seed(&self) -> u64 {
        self.seed
    }

    /// Returns the printed definition associated with one physical card.
    #[must_use]
    #[cfg(test)]
    fn physical_card_definition(&self, id: PhysicalCardId) -> Option<CardDefinitionId> {
        self.physical_cards
            .iter()
            .find(|card| card.id == id)
            .map(|card| card.definition)
    }

    /// Returns the owner of one physical card. Object control and copied
    /// characteristics are intentionally independent of this value.
    #[must_use]
    #[cfg(test)]
    fn physical_card_owner(&self, id: PhysicalCardId) -> Option<PlayerId> {
        self.physical_cards
            .iter()
            .find(|card| card.id == id)
            .map(|card| card.owner)
    }

    fn allocate_object_id(&mut self) -> GameObjectId {
        let id = GameObjectId(self.next_object_id);
        self.next_object_id = self
            .next_object_id
            .checked_add(1)
            .expect("game object IDs exhausted");
        id
    }

    fn zone_change_card(&mut self, mut card: CardInstance) -> (CardInstance, ZoneChangeOutcome) {
        let previous = card.id;
        self.retired_objects
            .entry(previous)
            .or_insert_with(|| RetiredObject::Card(card.clone()));
        card.id = self.allocate_object_id();
        let created = vec![card.id];
        (card, ZoneChangeOutcome { previous, created })
    }

    fn remove_battlefield_object(
        &mut self,
        index: usize,
        last_known: &PermanentLastKnownInformation,
    ) -> Permanent {
        let permanent = self.battlefield.remove(index);
        self.retired_objects.insert(
            permanent.card.id,
            RetiredObject::Permanent {
                permanent: Box::new(permanent.clone()),
                power: last_known.power,
                toughness: last_known.toughness,
                keywords: last_known.keywords.clone(),
            },
        );
        permanent
    }

    fn retire_stack_object(&mut self, object: &StackObject) {
        self.retired_objects
            .insert(object.id, RetiredObject::Stack(Box::new(object.clone())));
    }

    fn current_or_last_known_power(&self, object: GameObjectId) -> Option<i16> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == object)
            .and_then(|permanent| self.power(permanent))
            .or_else(|| match self.retired_objects.get(&object) {
                Some(RetiredObject::Permanent { power, .. }) => *power,
                Some(RetiredObject::Card(_) | RetiredObject::Stack(_)) | None => None,
            })
    }

    /// How many counters of one kind an object has, using last-known
    /// information once it has left the battlefield. An ability whose cost
    /// sacrificed its own source still reads the counters it had.
    fn current_or_last_known_counters(&self, object: GameObjectId, kind: CounterKind) -> u16 {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == object)
            .map_or_else(
                || match self.retired_objects.get(&object) {
                    Some(RetiredObject::Permanent { permanent, .. }) => permanent.counters(kind),
                    Some(RetiredObject::Card(_) | RetiredObject::Stack(_)) | None => 0,
                },
                |permanent| permanent.counters(kind),
            )
    }

    /// The values a predicate can read while matching, where the only context
    /// is the ability's source. Anything wider stays outside the boundary.
    fn value_from_source(&self, value: ValueDef, source: GameObjectId) -> Option<i32> {
        match value {
            ValueDef::Constant(amount) => Some(amount),
            ValueDef::CountersOnSource(kind) => {
                Some(i32::from(self.current_or_last_known_counters(source, kind)))
            }
            // A spell is its own source, so its chosen X is right there --
            // by way of the retired record, because a spell leaves the stack
            // before its effect runs. An activated ability's source is the
            // permanent instead, and its X is not reachable from a predicate.
            ValueDef::ChosenX => self
                .stack
                .iter()
                .find(|object| object.id == source)
                .map(|object| i32::from(object.x()))
                .or_else(|| match self.retired_objects.get(&source) {
                    Some(RetiredObject::Stack(object)) => Some(i32::from(object.x())),
                    Some(RetiredObject::Card(_) | RetiredObject::Permanent { .. }) | None => None,
                }),
            _ => None,
        }
    }

    fn current_or_last_known_toughness(&self, object: GameObjectId) -> Option<i16> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == object)
            .and_then(|permanent| self.toughness(permanent))
            .or_else(|| match self.retired_objects.get(&object) {
                Some(RetiredObject::Permanent { toughness, .. }) => *toughness,
                Some(RetiredObject::Card(_) | RetiredObject::Stack(_)) | None => None,
            })
    }

    fn current_or_last_known_controller(&self, object: GameObjectId) -> Option<PlayerId> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == object)
            .map(|permanent| permanent.controller)
            .or_else(|| {
                self.stack
                    .iter()
                    .find(|candidate| candidate.id == object)
                    .map(|candidate| candidate.controller)
            })
            .or_else(|| match self.retired_objects.get(&object) {
                Some(RetiredObject::Permanent { permanent, .. }) => Some(permanent.controller),
                Some(RetiredObject::Stack(stack)) => Some(stack.controller),
                Some(RetiredObject::Card(_)) | None => None,
            })
    }

    fn unbacked_object(
        &mut self,
        definition: CardDefinitionId,
        owner: PlayerId,
        characteristics: CharacteristicSource,
    ) -> CardInstance {
        CardInstance {
            id: self.allocate_object_id(),
            definition,
            owner,
            backing: ObjectBacking::None,
            characteristics,
        }
    }

    #[must_use]
    pub const fn result(&self) -> Option<GameResult> {
        self.result
    }

    /// Whether the first-strike combat-damage step has finished and the
    /// regular combat-damage step will begin after priority passes.
    #[must_use]
    pub fn regular_combat_damage_pending(&self) -> bool {
        self.result.is_none()
            && self.step == Step::CombatDamage
            && self.pending_combat_attackers.is_empty()
            && matches!(
                &self.combat_damage_stage,
                CombatDamageStage::FirstStrike { .. }
            )
    }

    /// Returns the player expected to make the engine's next decision.
    ///
    /// This may differ from the player with priority during pregame choices,
    /// turn-based actions such as declaring blockers, and other mandatory
    /// choices. Bot runners should observe this player and submit one of that
    /// observation's legal actions.
    #[must_use]
    pub fn decision_player(&self) -> Option<PlayerId> {
        if self.result.is_some() {
            return None;
        }
        if let Some(decision) = self.pending_decisions.first() {
            return Some(decision.observation.player);
        }
        if !self.pending_combat_attackers.is_empty() {
            return Some(self.active_player);
        }
        if let Some(pregame) = self.pregame {
            return Some(match pregame {
                Pregame::Mulligan(player) | Pregame::Bottom(player) => player,
            });
        }
        if self.cleanup_pending || self.untap_pending {
            return Some(self.active_player);
        }
        if self.step == Step::DeclareAttackers && !self.attackers_declared {
            return Some(self.active_player);
        }
        if self.step == Step::DeclareBlockers && !self.blockers_declared {
            return Some(self.active_player.opponent());
        }
        Some(self.priority)
    }

    /// Whether the game is still settling opening hands.
    ///
    /// The first turn has not begun during mulligans, so a client should not
    /// be describing a step or a turn yet.
    #[must_use]
    pub const fn in_pregame(&self) -> bool {
        self.pregame.is_some()
    }

    #[must_use]
    /// Returns the omniscient event trace.
    ///
    /// This is intended for replays and debugging. Give bots
    /// [`PlayerObservation`] rather than this event stream.
    pub fn events(&self) -> &[GameEvent] {
        &self.events
    }

    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn legal_actions(&self, player: PlayerId) -> Vec<Action> {
        if self.result.is_some() {
            return Vec::new();
        }

        let mut actions = vec![Action::Concede];
        if let Some(decision) = self.pending_decisions.first() {
            if decision.observation.player == player {
                // Bounded selections are represented by the decision observation rather
                // than by an eagerly-expanded Cartesian product. Callers submit the
                // selected option IDs through `ChooseDecision`; `apply` validates the
                // selection directly against this schema.
                actions.push(Action::ChooseDecision {
                    decision: decision.observation.id,
                    options: Vec::new(),
                });
                if decision.observation.cancellable {
                    actions.push(Action::CancelDecision {
                        decision: decision.observation.id,
                    });
                }
            }
            return actions;
        }
        if let Some(attacker) = self.pending_combat_attackers.first().copied() {
            if player == self.active_player {
                actions.extend(self.combat_assignment_actions(attacker));
            }
            return actions;
        }
        if let Some(pregame) = self.pregame {
            match pregame {
                Pregame::Mulligan(deciding) if player == deciding => {
                    actions.push(Action::KeepHand);
                    actions.push(Action::TakeMulligan);
                }
                Pregame::Bottom(deciding) if player == deciding => {
                    let count = usize::from(self.mulligans[player.index()])
                        .min(self.players[player.index()].hand.len());
                    actions.extend(
                        combinations(
                            &self.players[player.index()]
                                .hand
                                .iter()
                                .map(|card| card.id)
                                .collect::<Vec<_>>(),
                            count,
                        )
                        .into_iter()
                        .map(|cards| Action::BottomCards { cards }),
                    );
                }
                Pregame::Mulligan(_) | Pregame::Bottom(_) => {}
            }
            return actions;
        }
        if self.cleanup_pending {
            if player == self.active_player {
                let state = &self.players[player.index()];
                let count = state.hand.len().saturating_sub(7);
                actions.extend(
                    combinations(
                        &state.hand.iter().map(|card| card.id).collect::<Vec<_>>(),
                        count,
                    )
                    .into_iter()
                    .map(|cards| Action::DiscardCards { cards }),
                );
            }
            return actions;
        }
        if self.untap_pending {
            if player == self.active_player {
                actions.extend(self.untap_actions(player));
            }
            return actions;
        }
        if self.step == Step::DeclareAttackers && !self.attackers_declared {
            if player == self.active_player {
                let juggernaut_must_attack = self.battlefield.iter().any(|permanent| {
                    permanent.controller == player
                        && !permanent.tapped
                        && !permanent.attacking
                        && self.can_attack(permanent)
                        && self.effective_behavior(permanent) == Some(CardBehavior::Juggernaut)
                });
                if !juggernaut_must_attack {
                    actions.push(Action::FinishDeclaringAttackers);
                }
                actions.extend(self.attacker_actions(player));
            }
            return actions;
        }
        if self.step == Step::DeclareBlockers && !self.blockers_declared {
            if player == self.active_player.opponent() {
                actions.push(Action::FinishDeclaringBlockers);
                actions.extend(self.blocker_actions(player));
            }
            return actions;
        }
        if player != self.priority {
            return actions;
        }

        actions.push(Action::PassPriority);
        self.add_mana_actions(player, &mut actions);
        if self.channel_active[player.index()] && self.players[player.index()].life > 1 {
            actions.push(Action::PayLifeForMana);
        }
        self.add_land_actions(player, &mut actions);
        self.add_spell_actions(player, &mut actions);
        self.add_ability_actions(player, &mut actions);
        actions
    }

    /// Applies one engine-enumerated action for a player.
    ///
    /// # Errors
    ///
    /// Returns [`ActionError`] when the game is over or the action is not
    /// currently legal for that player.
    pub fn apply(&mut self, player: PlayerId, action: Action) -> Result<(), ActionError> {
        if self.result.is_some() {
            return Err(ActionError::GameAlreadyFinished);
        }
        if !self.is_legal_action(player, &action) {
            return Err(ActionError::NotLegal { player, action });
        }

        match action {
            Action::KeepHand => self.keep_hand(player),
            Action::TakeMulligan => self.take_mulligan(player),
            Action::BottomCards { cards } => self.bottom_cards(player, &cards),
            Action::DiscardCards { cards } => self.discard_cards(player, &cards),
            Action::ChooseDecision { decision, options } => {
                self.choose_decision(player, decision, &options);
            }
            Action::CancelDecision { decision } => self.cancel_decision(decision),
            Action::ChooseUntap { permanents } => self.choose_untap(player, &permanents),
            Action::PassPriority => self.pass_priority(player),
            Action::PlayLand { card, option } => self.play_land(player, card, option),
            Action::ActivateManaAbility {
                source,
                ability,
                color,
            } => {
                self.activate_mana_source(player, source, ability, color);
            }
            Action::PayLifeForMana => {
                self.players[player.index()].life -= 1;
                self.add_unrestricted_mana(player, ManaColor::Colorless, 1);
                self.consecutive_passes = 0;
            }
            Action::CastSpell {
                card,
                choices,
                sacrifices,
            } => self.cast_spell(player, card, choices, &sacrifices),
            Action::ActivateAbility {
                source,
                ability,
                targets,
                sacrifice,
                x,
            } => self.activate_ability(player, source, ability, targets, sacrifice, x),
            Action::DeclareAttacker { attacker } => self.declare_attacker(attacker),
            Action::FinishDeclaringAttackers => self.finish_declaring_attackers(),
            Action::DeclareBlocker { blocker, attacker } => {
                self.declare_blocker(blocker, attacker);
            }
            Action::FinishDeclaringBlockers => self.finish_declaring_blockers(),
            Action::AssignCombatDamage {
                attacker,
                assignments,
            } => self.assign_combat_damage(attacker, assignments),
            Action::Concede => self.finish(GameResult::Winner {
                winner: player.opponent(),
                reason: WinReason::OpponentConceded,
            }),
        }
        if self.result.is_none() {
            self.finish_rules_procedure();
        }
        Ok(())
    }

    /// Validates an action against the current state without mutating the game.
    ///
    /// Unlike [`legal_actions`], this also validates the option IDs supplied to
    /// a bounded [`Action::ChooseDecision`] selection without expanding every
    /// possible combination into a vector.
    #[must_use]
    pub fn is_legal_action(&self, player: PlayerId, action: &Action) -> bool {
        if let Action::ChooseDecision { decision, options } = action {
            let Some(pending) = self.pending_decisions.first() else {
                return false;
            };
            let observation = &pending.observation;
            if observation.player != player || observation.id != *decision {
                return false;
            }
            let available = observation
                .options
                .iter()
                .map(|option| option.id)
                .collect::<std::collections::HashSet<_>>();
            let unique = options
                .iter()
                .copied()
                .collect::<std::collections::HashSet<_>>();
            options.len() == unique.len()
                && options.len() >= observation.minimum
                && options.len() <= observation.maximum
                && options.iter().all(|option| available.contains(option))
        } else {
            self.legal_actions(player).contains(action)
        }
    }

    /// One card in a hand or library, as a simulation sees it.
    ///
    /// This is not redacted. [`Self::observe`] is the redacted view, and it is
    /// what anything talking to a client should use; a `Game` in your own
    /// process has no one to hide from.
    #[must_use]
    pub fn hand(&self, player: PlayerId) -> Vec<ZoneCard> {
        zone_cards(&self.players[player.index()].hand)
    }

    /// The player's library from the top down, so index zero is the next draw.
    #[must_use]
    pub fn library(&self, player: PlayerId) -> Vec<ZoneCard> {
        zone_cards(&self.players[player.index()].library)
    }

    /// Replaces a hand with exactly these cards, named by definition.
    ///
    /// The cards are built fresh, so this states what a hand *is* rather than
    /// moving objects around: to explore "their last card is either Lightning
    /// Bolt or Counterspell", set the same hand twice with a different last
    /// entry and play both out. Nothing is conserved, because a simulation
    /// exploring a world it cannot see has no reason to be.
    ///
    /// The new cards get new object IDs. Rewrite an opponent's zones rather
    /// than your own if you are holding IDs from an earlier observation.
    ///
    /// # Errors
    ///
    /// Returns [`ZoneError::UnknownCard`] if a definition is not in the
    /// catalog this game was built with.
    pub fn set_hand(
        &mut self,
        player: PlayerId,
        cards: &[CardDefinitionId],
    ) -> Result<(), ZoneError> {
        let built = self.build_zone(player, cards)?;
        self.players[player.index()].hand = built;
        Ok(())
    }

    /// Puts a permanent onto the battlefield under `player`, named by
    /// definition, and returns its object ID.
    ///
    /// This completes the simulation surface that [`Self::set_hand`] and
    /// [`Self::set_library`] start: those state what a hidden zone holds, and
    /// this states what is in play. It is how a caller reaches a board state
    /// directly instead of playing toward one.
    ///
    /// The permanent enters as though it resolved, raising the same
    /// zone-change event, so anything that triggers on entering sees it. It
    /// does not pay a cost, take a turn, or respect timing: setting up a board
    /// is not the same as playing to one, and the difference is the point.
    ///
    /// # Errors
    ///
    /// Returns [`ZoneError::UnknownCard`] when the definition is not in this
    /// game's catalog, and [`ZoneError::TooManyCards`] when the game has run
    /// out of object identifiers.
    ///
    /// # Panics
    ///
    /// Panics if the catalog yields no card for a definition it just
    /// validated, which would mean the catalog changed mid-call.
    pub fn put_onto_battlefield(
        &mut self,
        player: PlayerId,
        definition: CardDefinitionId,
    ) -> Result<GameObjectId, ZoneError> {
        let Some(card) = self.catalog.get(definition) else {
            return Err(ZoneError::UnknownCard(definition));
        };
        let presented = card.primary_part_id();
        let starting_loyalty = card
            .part(presented)
            .and_then(|part| part.rules.starting_loyalty())
            .map(|loyalty| i16::try_from(loyalty).unwrap_or(i16::MAX));
        let built = self.build_zone(player, &[definition])?;
        let card = built
            .into_iter()
            .next()
            .expect("build_zone returns one card for one definition");
        let id = card.id;
        let mut permanent =
            Permanent::entering(card, presented, player, self.turns_started[player.index()]);
        permanent.loyalty = starting_loyalty;
        self.battlefield.push(permanent);
        let entered = self
            .battlefield
            .last()
            .expect("the permanent just pushed is on the battlefield");
        let entered_event = self.trigger_event_object(entered);
        self.capture_battlefield_triggers(&CommittedTriggerEvent::ZoneChanged {
            object: entered_event,
            from: ZoneKind::Stack,
            to: ZoneKind::Battlefield,
        });
        Ok(id)
    }

    /// Puts a card into a player's graveyard directly, as a simulation and
    /// test entry point. Nothing died and nothing resolved, so no trigger sees
    /// this.
    ///
    /// # Errors
    ///
    /// Returns [`ZoneError::UnknownCard`] when the definition is not cataloged.
    ///
    /// # Panics
    ///
    /// Panics if building one card from one definition yields no card.
    pub fn put_into_graveyard(
        &mut self,
        player: PlayerId,
        definition: CardDefinitionId,
    ) -> Result<GameObjectId, ZoneError> {
        let built = self.build_zone(player, &[definition])?;
        let card = built
            .into_iter()
            .next()
            .expect("build_zone returns one card for one definition");
        let id = card.id;
        self.players[player.index()].graveyard.push(card);
        Ok(id)
    }

    /// Replaces a library with exactly these cards, top card first. Behaves
    /// like [`Self::set_hand`] in every other respect.
    ///
    /// # Errors
    ///
    /// Returns [`ZoneError::UnknownCard`] under the same conditions as
    /// [`Self::set_hand`].
    pub fn set_library(
        &mut self,
        player: PlayerId,
        cards: &[CardDefinitionId],
    ) -> Result<(), ZoneError> {
        let built = self.build_zone(player, cards)?;
        self.players[player.index()].library = built;
        Ok(())
    }

    /// Mints fresh instances owned by `player`, one per definition.
    fn build_zone(
        &mut self,
        player: PlayerId,
        cards: &[CardDefinitionId],
    ) -> Result<Vec<CardInstance>, ZoneError> {
        if let Some(unknown) = cards
            .iter()
            .find(|definition| self.catalog.get(**definition).is_none())
        {
            return Err(ZoneError::UnknownCard(*unknown));
        }

        cards
            .iter()
            .map(|definition| {
                let id = GameObjectId(self.next_object_id);
                self.next_object_id = self
                    .next_object_id
                    .checked_add(1)
                    .ok_or(ZoneError::TooManyCards)?;
                Ok(CardInstance {
                    id,
                    definition: *definition,
                    owner: player,
                    // A card conjured for a hypothetical has no physical
                    // provenance, which only meld and copy effects consult.
                    backing: ObjectBacking::None,
                    characteristics: CharacteristicSource::Card(*definition),
                })
            })
            .collect()
    }

    #[must_use]
    pub fn observe(&self, viewer: PlayerId) -> PlayerObservation {
        let player = &self.players[viewer.index()];
        let opponent = &self.players[viewer.opponent().index()];
        PlayerObservation {
            viewer,
            turn: self.turn,
            active_turn: self.turns_started[self.active_player.index()],
            active_player: self.active_player,
            priority: self.priority,
            step: self.step,
            regular_combat_damage_pending: self.regular_combat_damage_pending(),
            life_totals: [self.players[0].life, self.players[1].life],
            mana_pools: [self.players[0].mana_pool, self.players[1].mana_pool],
            hand: player
                .hand
                .iter()
                .map(|card| (card.id, card.definition))
                .collect(),
            opponent_hand_size: opponent.hand.len(),
            last_seen_hand: self.last_seen_hands[viewer.index()].clone(),
            library_sizes: [self.players[0].library.len(), self.players[1].library.len()],
            graveyards: [
                public_cards(&self.players[0].graveyard),
                public_cards(&self.players[1].graveyard),
            ],
            exiles: [
                public_cards(&self.players[0].exile),
                public_cards(&self.players[1].exile),
            ],
            battlefield: self
                .battlefield
                .iter()
                .map(|permanent| PermanentObservation {
                    id: permanent.card.id,
                    definition: permanent.card.definition,
                    presented: permanent.presented,
                    controller: permanent.controller,
                    chosen_creature_type: permanent.chosen_creature_type.clone(),
                    tapped: permanent.tapped,
                    power: self.power_ignoring_static_effects(permanent),
                    toughness: self.toughness(permanent),
                    damage: permanent.damage,
                    attacking: permanent.attacking,
                    blocking: permanent.blocking,
                    flying: self.has_flying(permanent),
                    can_attack: self.can_attack(permanent),
                    entered_this_turn: self.turns_started[permanent.controller.index()]
                        == permanent.entered_controller_turn,
                })
                .collect(),
            stack: self
                .stack
                .iter()
                .map(|object| StackObservation {
                    id: object.id,
                    kind: object.kind,
                    source: object.source,
                    ability: object.ability_origin(),
                    ability_text: object.ability_text().map(str::to_owned),
                    definition: object.presentation_definition(),
                    controller: object.controller,
                    counterable: self.can_be_countered(object),
                    signature: object.signature.clone(),
                    targets: object.targets(),
                    chosen_permanents: object.chosen_permanents.clone(),
                    x: object.x(),
                })
                .collect(),
            decision: self.pending_decisions.first().and_then(|decision| {
                (decision.observation.visibility == DecisionVisibility::Public
                    || decision.observation.player == viewer)
                    .then(|| decision.observation.clone())
            }),
            result: self.result,
            legal_actions: self.legal_actions(viewer),
        }
    }

    fn add_mana_actions(&self, player: PlayerId, actions: &mut Vec<Action>) {
        for permanent in self
            .battlefield
            .iter()
            .filter(|permanent| permanent.controller == player)
        {
            actions.extend(self.mana_ability_activations(permanent).into_iter().map(
                |activation| Action::ActivateManaAbility {
                    source: activation.source,
                    ability: activation.ability,
                    color: activation.color,
                },
            ));
        }
    }

    fn keep_hand(&mut self, player: PlayerId) {
        if self.mulligans[player.index()] > 0 {
            self.pregame = Some(Pregame::Bottom(player));
        } else {
            self.advance_pregame(player);
        }
    }

    fn take_mulligan(&mut self, player: PlayerId) {
        let hand = std::mem::take(&mut self.players[player.index()].hand);
        for card in hand {
            let (card, _zone_change) = self.zone_change_card(card);
            self.players[player.index()].library.push(card);
        }
        self.rng.shuffle(&mut self.players[player.index()].library);
        let initial_hand = draw_opening_hand(
            &mut self.players[player.index()].library,
            self.format.rules().opening_hand_size,
        )
        .expect("a validated deck always contains at least seven cards");
        for card in initial_hand {
            let (card, _zone_change) = self.zone_change_card(card);
            self.players[player.index()].hand.push(card);
        }
        self.mulligans[player.index()] = self.mulligans[player.index()].saturating_add(1);
    }

    fn bottom_cards(&mut self, player: PlayerId, cards: &[GameObjectId]) {
        for id in cards.iter().rev() {
            if let Some(card) = remove_card(&mut self.players[player.index()].hand, *id) {
                let (card, _zone_change) = self.zone_change_card(card);
                self.players[player.index()].library.insert(0, card);
            }
        }
        self.advance_pregame(player);
    }

    fn advance_pregame(&mut self, player: PlayerId) {
        if player == PlayerId::One {
            self.pregame = Some(Pregame::Mulligan(PlayerId::Two));
            self.priority = PlayerId::Two;
        } else {
            self.pregame = None;
            self.priority = PlayerId::One;
        }
    }

    fn discard_cards(&mut self, player: PlayerId, cards: &[GameObjectId]) {
        self.discard_cards_with_cause(player, cards, ZoneMoveCause::Rules);
        self.cleanup_pending = false;
        self.complete_cleanup();
        if self.result.is_none() {
            self.priority = self.active_player;
            self.events.push(GameEvent::StepChanged {
                turn: self.turn,
                active_player: self.active_player,
                step: self.step,
            });
        }
    }

    fn zone_move_replacement_destination(
        &self,
        card: &CardInstance,
        from: ZoneKind,
        to: ZoneKind,
        actual_cause: ZoneMoveCause,
    ) -> Option<ZoneKind> {
        let characteristic_context = match from {
            ZoneKind::Library => CharacteristicContext::Library,
            ZoneKind::Hand => CharacteristicContext::Hand,
            ZoneKind::Graveyard => CharacteristicContext::Graveyard,
            ZoneKind::Exile => CharacteristicContext::Exile,
            ZoneKind::Command => CharacteristicContext::Command,
            ZoneKind::Battlefield | ZoneKind::Stack => return None,
        };
        let replacement_controller = card.owner;
        let definition = self.catalog.get(card.definition)?;
        let parts = applicable_part_ids(definition, &characteristic_context).ok()?;
        for part in parts {
            let Some(part) = definition.part(part) else {
                continue;
            };
            for ability in part.rules.ability_clauses() {
                let DeclarativeAbilityDef::Replacement(replacement) = ability.definition else {
                    continue;
                };
                let ReplacementEventDef::WouldMove {
                    from: event_from,
                    to: event_to,
                    cause,
                } = replacement.event
                else {
                    continue;
                };
                let cause_matches = match cause {
                    ZoneMoveCauseDef::Any => true,
                    ZoneMoveCauseDef::EffectControlledBy(relation) => {
                        let ZoneMoveCause::Effect { controller } = actual_cause else {
                            continue;
                        };
                        self.player_relation_matches(
                            controller,
                            relation,
                            replacement_controller,
                            TriggerContext::empty(),
                        )
                    }
                };
                if event_from == from
                    && event_to == to
                    && cause_matches
                    && ability.implementation.is_executable()
                    && replacement.source_zones.contains(&from)
                    && let EffectDef::MoveToZone {
                        object: EffectRecipientDef::Source,
                        zone,
                    } = ability.effect
                {
                    return Some(zone);
                }
            }
        }
        None
    }

    fn put_card_onto_battlefield_from(
        &mut self,
        card: CardInstance,
        from: ZoneKind,
        controller: PlayerId,
    ) -> CardInstance {
        let definition = self
            .catalog
            .get(card.definition)
            .expect("a card in hand remains cataloged");
        let presented = applicable_part_ids(definition, &CharacteristicContext::Hand)
            .ok()
            .and_then(|parts| parts.first().copied())
            .unwrap_or(CardPartId::PRIMARY);
        let starting_loyalty = definition
            .part(presented)
            .and_then(|part| part.rules.starting_loyalty())
            .map(|loyalty| i16::try_from(loyalty).unwrap_or(i16::MAX));
        let (card, _zone_change) = self.zone_change_card(card);
        let entered_card = card.clone();
        let mut permanent = Permanent::entering(
            card,
            presented,
            controller,
            self.turns_started[controller.index()],
        );
        permanent.loyalty = starting_loyalty;
        self.battlefield.push(permanent);
        let entered = self
            .battlefield
            .last()
            .expect("the replacement put a permanent onto the battlefield");
        let entered_event = self.trigger_event_object(entered);
        self.capture_battlefield_triggers(&CommittedTriggerEvent::ZoneChanged {
            object: entered_event,
            from,
            to: ZoneKind::Battlefield,
        });
        self.apply_legend_rule();
        entered_card
    }

    /// Moves a card between non-stack zones after applying replacement
    /// abilities printed on that card. The replacement is selected before the
    /// old object leaves its source zone, so its source-zone characteristics
    /// remain available while matching the proposed move.
    fn move_card_from_nonbattlefield_zone(
        &mut self,
        id: GameObjectId,
        expected_from: ZoneKind,
        requested_to: ZoneKind,
        cause: ZoneMoveCause,
    ) -> Option<(CardInstance, ZoneKind)> {
        let (from, card) = self
            .card_in_nonbattlefield_zone(id)
            .map(|(zone, card)| (zone, card.clone()))?;
        if from != expected_from {
            return None;
        }
        let destination = self
            .zone_move_replacement_destination(&card, from, requested_to, cause)
            .unwrap_or(requested_to);
        if matches!(destination, ZoneKind::Stack | ZoneKind::Command) {
            return None;
        }

        let owner = card.owner;
        let cards = match from {
            ZoneKind::Library => &mut self.players[owner.index()].library,
            ZoneKind::Hand => &mut self.players[owner.index()].hand,
            ZoneKind::Graveyard => &mut self.players[owner.index()].graveyard,
            ZoneKind::Exile => &mut self.players[owner.index()].exile,
            ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => return None,
        };
        let card = remove_card(cards, id)?;
        let card = if destination == ZoneKind::Battlefield {
            self.put_card_onto_battlefield_from(card, from, owner)
        } else {
            let (card, _zone_change) = self.zone_change_card(card);
            match destination {
                ZoneKind::Library => self.players[owner.index()].library.push(card.clone()),
                ZoneKind::Hand => self.players[owner.index()].hand.push(card.clone()),
                ZoneKind::Graveyard => self.players[owner.index()].graveyard.push(card.clone()),
                ZoneKind::Exile => self.players[owner.index()].exile.push(card.clone()),
                ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => {
                    unreachable!("unsupported destinations returned before removing the card")
                }
            }
            card
        };
        Some((card, destination))
    }

    fn discard_cards_with_cause(
        &mut self,
        player: PlayerId,
        cards: &[GameObjectId],
        cause: ZoneMoveCause,
    ) {
        let mut discarded = Vec::new();
        for id in cards {
            if !self.players[player.index()]
                .hand
                .iter()
                .any(|card| card.id == *id)
            {
                continue;
            }
            let Some((card, _destination)) = self.move_card_from_nonbattlefield_zone(
                *id,
                ZoneKind::Hand,
                ZoneKind::Graveyard,
                cause,
            ) else {
                continue;
            };
            let definition = card.definition;
            discarded.push((card.id, definition));
        }
        if !discarded.is_empty() {
            self.events.push(GameEvent::CardsDiscarded {
                player,
                cards: discarded,
            });
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn queue_decision(
        &mut self,
        player: PlayerId,
        prompt: impl Into<String>,
        visibility: DecisionVisibility,
        preference: DecisionPreference,
        bounds: std::ops::RangeInclusive<usize>,
        cancellable: bool,
        options: Vec<DecisionOption>,
        continuation: DecisionContinuation,
    ) {
        // A player can only choose from what is there. Asking for a minimum
        // the options cannot supply leaves no legal `ChooseDecision`, because
        // `is_legal` requires at least `minimum` of them — and when the
        // decision is also not cancellable, the game has no legal action at
        // all and deadlocks. Demonic Tutor did exactly that on an empty
        // library. Magic resolves as much of an effect as it can, so lower the
        // requirement to what exists and let the continuation take it from
        // there; each one already handles being handed nothing.
        let minimum = (*bounds.start()).min(options.len());

        let id = self.next_decision_id;
        self.next_decision_id = self.next_decision_id.saturating_add(1);
        self.pending_decisions.push(PendingDecision {
            observation: DecisionObservation {
                id,
                player,
                kind: DecisionKind::Choice,
                order_semantics: None,
                prompt: prompt.into(),
                visibility,
                preference,
                minimum,
                maximum: (*bounds.end()).max(minimum),
                cancellable,
                options,
            },
            continuation,
        });
    }

    fn queue_basic_land_type_text_change(&mut self, player: PlayerId, target: Target) {
        let options = BasicLandType::ALL
            .into_iter()
            .flat_map(|from| {
                BasicLandType::ALL
                    .into_iter()
                    .filter(move |to| from != *to)
                    .map(move |to| DecisionOption {
                        id: u32::try_from(from.index() * BasicLandType::ALL.len() + to.index())
                            .expect("the basic-land-type choice id fits u32"),
                        label: format!("{} → {}", from.subtype(), to.subtype()),
                        card: None,
                        ability_text: None,
                        zone: DecisionZone::None,
                    })
            })
            .collect();
        self.queue_decision(
            player,
            "Replace one basic land type with another",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::BasicLandTypeTextChange { target },
        );
    }

    /// Finishes an atomic rules procedure before a player can receive
    /// priority. Mana abilities invoked while casting resolve inside the
    /// procedure, while ordinary triggers collected by them wait here.
    fn finish_rules_procedure(&mut self) {
        // A decision can be one step in a still-resolving spell or turn-based
        // procedure. Neither state-based actions nor trigger placement happen
        // in the middle of that procedure: for example, a creature dealt
        // lethal damage by Chain Lightning can still activate a mana ability
        // when its controller is asked whether to pay for the copy. Drain the
        // continuation chain before reaching either priority-boundary check.
        if !self.pending_decisions.is_empty() {
            return;
        }

        self.check_state_based_actions();
        if self.result.is_none() {
            self.begin_trigger_placement();
        }
    }

    fn capture_trigger(&mut self, capture: &TriggerCapture) {
        // Rule 603.4: an intervening-if condition is checked as the ability
        // would trigger. Failing it means the ability never triggers at all,
        // so nothing reaches the stack and nothing is reported.
        if let Some(condition) = capture.condition
            && !self.trigger_condition_holds(
                condition,
                capture.source.object,
                capture.controller,
                capture.context,
            )
        {
            return;
        }
        let id = self.next_trigger_id;
        self.next_trigger_id = self.next_trigger_id.saturating_add(1);
        self.pending_triggers.push(PendingTrigger {
            id,
            source: capture.source,
            definition: capture.definition,
            owner: capture.owner,
            controller: capture.controller,
            text: capture.text,
            target_defs: capture.target_defs,
            targets: Vec::new(),
            effect: capture.effect,
            resolver: capture.resolver,
            context: capture.context,
            condition: capture.condition,
        });
        self.events.push(GameEvent::AbilityTriggered {
            player: capture.controller,
            trigger: id,
            source: capture.source.object,
            definition: capture.definition,
        });
    }

    const fn ability_presentation_definition(
        origin: AbilityOrigin,
        fallback: CardDefinitionId,
    ) -> CardDefinitionId {
        match origin {
            AbilityOrigin::Printed { definition, .. } => definition,
            AbilityOrigin::IntrinsicBasicLand(_) | AbilityOrigin::Granted { .. } => fallback,
        }
    }

    fn capture_battlefield_triggers(&mut self, event: &CommittedTriggerEvent) {
        let listeners = self.battlefield_trigger_listeners();
        self.capture_battlefield_triggers_from_snapshot(&listeners, event);
    }

    fn battlefield_trigger_listeners(&self) -> Vec<BattlefieldTriggerListener> {
        let mut listeners = Vec::new();
        for permanent in &self.battlefield {
            self.for_each_effective_ability(permanent, |effective| {
                let ability = effective.ability;
                // Fully declarative and fully implemented custom triggers use
                // the shared stack. Partial custom clauses may still describe
                // a legacy path that executes elsewhere (Mana Vault today), so
                // admitting them here would manufacture a duplicate trigger.
                if !matches!(
                    ability.implementation,
                    AbilityImplementationDef::Definition
                        | AbilityImplementationDef::CustomFull { .. }
                ) {
                    return;
                }
                let (definition, uses_stack) = match ability.definition {
                    DeclarativeAbilityDef::TriggeredMana(definition) => (definition, false),
                    DeclarativeAbilityDef::Triggered(definition) => (definition, true),
                    DeclarativeAbilityDef::Spell(_)
                    | DeclarativeAbilityDef::ActivatedMana(_)
                    | DeclarativeAbilityDef::Activated(_)
                    | DeclarativeAbilityDef::Static(_)
                    | DeclarativeAbilityDef::Replacement(_)
                    | DeclarativeAbilityDef::AlternativeCast(_)
                    | DeclarativeAbilityDef::SpecialAction(_)
                    | DeclarativeAbilityDef::Keyword(_)
                    | DeclarativeAbilityDef::Legacy => return,
                };
                if !definition.source_zones.contains(&ZoneKind::Battlefield) {
                    return;
                }
                let source = AbilitySourceRef {
                    object: permanent.card.id,
                    ability: effective.origin,
                };
                listeners.push(BattlefieldTriggerListener {
                    event: definition.event,
                    uses_stack,
                    capture: TriggerCapture {
                        source,
                        definition: Self::ability_presentation_definition(
                            effective.origin,
                            Self::effective_rules_source(permanent).0,
                        ),
                        owner: permanent.card.owner,
                        controller: permanent.controller,
                        text: ability.text,
                        target_defs: definition.targets,
                        effect: ability.effect,
                        resolver: Self::ability_resolver(&ability),
                        context: TriggerContext::empty(),
                        condition: definition.condition,
                    },
                });
            });
        }
        listeners
    }

    fn capture_battlefield_triggers_from_snapshot(
        &mut self,
        listeners: &[BattlefieldTriggerListener],
        event: &CommittedTriggerEvent,
    ) {
        let mana_triggers = listeners
            .iter()
            .copied()
            .filter(|listener| {
                !listener.uses_stack
                    && self.trigger_event_matches(
                        listener.event,
                        event,
                        listener.capture.source.object,
                    )
            })
            .collect::<Vec<_>>();
        for listener in mana_triggers {
            self.resolve_triggered_mana_effect(
                listener.capture.source,
                listener.capture.controller,
                listener.capture.effect,
            );
        }

        let stack_triggers = listeners
            .iter()
            .copied()
            .filter(|listener| {
                listener.uses_stack
                    && self.trigger_event_matches(
                        listener.event,
                        event,
                        listener.capture.source.object,
                    )
            })
            .collect::<Vec<_>>();
        for listener in stack_triggers {
            self.capture_trigger(&TriggerCapture {
                context: event.context(),
                ..listener.capture
            });
        }
    }

    fn resolve_triggered_mana_effect(
        &mut self,
        source: AbilitySourceRef,
        controller: PlayerId,
        effect: EffectDef,
    ) {
        match effect {
            EffectDef::Sequence(effects) => {
                for effect in effects {
                    self.resolve_triggered_mana_effect(source, controller, *effect);
                }
            }
            EffectDef::AddMana(AddManaEffectDef {
                mana: ManaSelectionDef::One(kind),
                amount,
                restrictions,
                spend_effects,
            }) => {
                let mana = Mana::from_ability(
                    kind,
                    ManaSource {
                        object: source.object,
                        ability: source.ability,
                    },
                    restrictions,
                    spend_effects,
                );
                self.add_mana(controller, std::iter::repeat_n(mana, usize::from(amount)));
            }
            EffectDef::None
            | EffectDef::AddMana(AddManaEffectDef {
                mana: ManaSelectionDef::Choice(_),
                ..
            })
            | EffectDef::DealDamage { .. }
            | EffectDef::GainLife { .. }
            | EffectDef::DrawCards { .. }
            | EffectDef::DiscardCards { .. }
            | EffectDef::LoseLife { .. }
            | EffectDef::Tap { .. }
            | EffectDef::Untap { .. }
            | EffectDef::Destroy { .. }
            | EffectDef::Sacrifice { .. }
            | EffectDef::SacrificeOfChoice { .. }
            | EffectDef::Counter { .. }
            | EffectDef::CounterUnlessPaid { .. }
            | EffectDef::AddCounters { .. }
            | EffectDef::ChangeTextBasicLandType { .. }
            | EffectDef::BecomeCopyOf { .. }
            | EffectDef::OptionalManaPayment { .. }
            | EffectDef::May(_)
            | EffectDef::EntersTapped
            | EffectDef::CannotBeForcedToSacrifice
            | EffectDef::GrantFlashToNextSorcery
            | EffectDef::ExileLinkedToSource { .. }
            | EffectDef::ReturnLinkedExiles { .. }
            | EffectDef::MakeUnblockableThisTurn { .. }
            | EffectDef::AtNextStep { .. }
            | EffectDef::ReduceGenericCostBy(_)
            | EffectDef::MultiplyEventAmount(_)
            | EffectDef::MoveToZone { .. }
            | EffectDef::Attach { .. }
            | EffectDef::CreateToken { .. }
            | EffectDef::ChooseCreatureType { .. }
            | EffectDef::Apply { .. }
            | EffectDef::Special(_) => {
                // Choice-bearing and non-mana primitives need a dedicated
                // immediate procedure before a supported card can use them.
            }
        }
    }

    fn capture_custom_source_triggers(
        &mut self,
        source: &Permanent,
        abilities: &[EffectiveAbility],
        event: &CommittedTriggerEvent,
    ) {
        let triggers = abilities
            .iter()
            .filter_map(|effective| match effective.ability.definition {
                DeclarativeAbilityDef::Triggered(definition)
                    if matches!(
                        effective.ability.implementation,
                        AbilityImplementationDef::CustomPartial { .. }
                    ) && effective.ability.implementation.custom_behavior().is_some()
                        && definition.source_zones.contains(&ZoneKind::Battlefield)
                        && self.trigger_event_matches(definition.event, event, source.card.id) =>
                {
                    Some((
                        effective.origin,
                        effective.ability.text,
                        definition.targets,
                        effective.ability.effect,
                        Self::ability_resolver(&effective.ability),
                    ))
                }
                DeclarativeAbilityDef::Spell(_)
                | DeclarativeAbilityDef::ActivatedMana(_)
                | DeclarativeAbilityDef::TriggeredMana(_)
                | DeclarativeAbilityDef::Activated(_)
                | DeclarativeAbilityDef::Triggered(_)
                | DeclarativeAbilityDef::Static(_)
                | DeclarativeAbilityDef::Replacement(_)
                | DeclarativeAbilityDef::AlternativeCast(_)
                | DeclarativeAbilityDef::SpecialAction(_)
                | DeclarativeAbilityDef::Keyword(_)
                | DeclarativeAbilityDef::Legacy => None,
            })
            .collect::<Vec<_>>();
        for (ability, text, targets, effect, resolver) in triggers {
            self.capture_trigger(&TriggerCapture {
                source: AbilitySourceRef {
                    object: source.card.id,
                    ability,
                },
                definition: Self::ability_presentation_definition(
                    ability,
                    Self::effective_rules_source(source).0,
                ),
                owner: source.card.owner,
                controller: source.controller,
                text,
                target_defs: targets,
                effect,
                resolver,
                context: event.context(),
                // A legacy custom trigger states its own condition inside its
                // behavior rather than declaring one here.
                condition: None,
            });
        }
    }

    const fn ability_resolver(ability: &AbilityDef) -> StackAbilityResolver {
        match ability.implementation.custom_behavior() {
            Some(behavior) => StackAbilityResolver::Custom(behavior),
            None => StackAbilityResolver::Declarative(ability.effect),
        }
    }

    fn freeze_activated_ability(
        &self,
        permanent: &Permanent,
        origin: AbilityOrigin,
    ) -> FrozenActivatedAbility {
        let effective =
            self.find_effective_ability(permanent, |effective| effective.origin == origin);
        let fallback_definition = Self::effective_rules_source(permanent).0;
        let presentation_definition =
            Self::ability_presentation_definition(origin, fallback_definition);
        let text = effective.map(|effective| effective.ability.text);
        let definition = effective.map(|effective| Box::new(effective.ability));
        let (target_defs, resolver) = effective.map_or(
            (&[][..], StackAbilityResolver::Declarative(EffectDef::None)),
            |effective| {
                let target_defs = match effective.ability.definition {
                    DeclarativeAbilityDef::Activated(definition) => definition.targets,
                    DeclarativeAbilityDef::Spell(_)
                    | DeclarativeAbilityDef::ActivatedMana(_)
                    | DeclarativeAbilityDef::TriggeredMana(_)
                    | DeclarativeAbilityDef::Triggered(_)
                    | DeclarativeAbilityDef::Static(_)
                    | DeclarativeAbilityDef::Replacement(_)
                    | DeclarativeAbilityDef::AlternativeCast(_)
                    | DeclarativeAbilityDef::SpecialAction(_)
                    | DeclarativeAbilityDef::Keyword(_)
                    | DeclarativeAbilityDef::Legacy => &[],
                };
                (target_defs, Self::ability_resolver(&effective.ability))
            },
        );
        FrozenActivatedAbility {
            origin,
            definition,
            presentation_definition,
            text,
            target_defs,
            resolver,
            // Filled in by the activation, which is where X is chosen.
            x: 0,
        }
    }

    fn trigger_event_matches(
        &self,
        definition: TriggerEventDef,
        event: &CommittedTriggerEvent,
        source: GameObjectId,
    ) -> bool {
        match (definition, event) {
            (
                TriggerEventDef::ZoneChanged {
                    object: predicate,
                    from,
                    to,
                },
                CommittedTriggerEvent::ZoneChanged {
                    object,
                    from: actual_from,
                    to: actual_to,
                },
            ) => {
                from.is_none_or(|expected| expected == *actual_from)
                    && to.is_none_or(|expected| expected == *actual_to)
                    && self.trigger_object_matches(predicate, object, source, false)
            }
            (
                TriggerEventDef::BecomesTapped(predicate),
                CommittedTriggerEvent::BecomesTapped { object },
            )
            | (
                TriggerEventDef::TappedForMana(predicate),
                CommittedTriggerEvent::TappedForMana { object },
            ) => self.trigger_object_matches(predicate, object, source, false),
            (TriggerEventDef::Attacks(predicate), CommittedTriggerEvent::Attacks { object }) => {
                self.trigger_object_matches(predicate, object, source, false)
            }

            (
                TriggerEventDef::DamageDealt {
                    source: _,
                    recipient: EffectRecipientDef::Source,
                },
                CommittedTriggerEvent::DamageDealt { object, .. },
            ) => object.id == source,
            (
                TriggerEventDef::LifeGained(relation),
                CommittedTriggerEvent::LifeGained { player, .. },
            ) => {
                let controller = self
                    .current_or_last_known_controller(source)
                    .unwrap_or(*player);
                self.player_relation_matches(*player, relation, controller, event.context())
            }
            (
                TriggerEventDef::SpellCast(predicate),
                CommittedTriggerEvent::SpellCast { object },
            ) => self.trigger_object_matches(predicate, object, source, true),
            (
                TriggerEventDef::StepBegins { step, player },
                CommittedTriggerEvent::StepBegins {
                    step: actual_step,
                    player: actual_player,
                },
            ) => {
                let controller = self
                    .current_or_last_known_controller(source)
                    .unwrap_or(*actual_player);
                step == *actual_step
                    && self.player_relation_matches(
                        *actual_player,
                        player,
                        controller,
                        event.context(),
                    )
            }
            (
                TriggerEventDef::DamagedCreatureDied,
                CommittedTriggerEvent::DamagedCreatureDied {
                    source: actual_source,
                    ..
                },
            ) => source == *actual_source,
            _ => false,
        }
    }

    /// Who controls an object, whether it is still on the battlefield or has
    /// left and is only remembered.
    fn controller_of_object(&self, object: GameObjectId) -> Option<PlayerId> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == object)
            .map(|permanent| permanent.controller)
            .or_else(|| match self.retired_objects.get(&object) {
                Some(RetiredObject::Permanent { permanent, .. }) => Some(permanent.controller),
                Some(RetiredObject::Stack(object)) => Some(object.controller),
                Some(RetiredObject::Card(_)) | None => None,
            })
    }

    /// Whether `object` satisfies `predicate`. `source` is the ability's own
    /// object, which is what a controller relation is measured against.
    fn trigger_object_matches(
        &self,
        predicate: ObjectPredicateDef,
        object: &TriggerEventObject,
        source: GameObjectId,
        is_spell: bool,
    ) -> bool {
        match predicate {
            ObjectPredicateDef::Any => true,
            ObjectPredicateDef::Source => object.id == source,
            ObjectPredicateDef::HasType(card_type) => object.types.contains(card_type),
            ObjectPredicateDef::Spell => is_spell,
            ObjectPredicateDef::NoncreatureSpell => {
                is_spell && !object.types.contains(CardType::Creature)
            }
            ObjectPredicateDef::Color(color) => color
                .color_index()
                .is_some_and(|index| object.colors[index]),
            ObjectPredicateDef::Subtype(subtype) => object.subtypes.contains(&subtype),
            ObjectPredicateDef::ManaValueAtMost(limit) => object.mana_value <= u16::from(limit),
            ObjectPredicateDef::ManaValueEqualTo(value) => self
                .value_from_source(value, source)
                .is_some_and(|value| value == i32::from(object.mana_value)),
            ObjectPredicateDef::ManaValueAtMostValue(value) => self
                .value_from_source(value, source)
                .is_some_and(|value| i32::from(object.mana_value) <= value),
            ObjectPredicateDef::PowerAtLeast(minimum) => {
                object.power.is_some_and(|power| power >= minimum)
            }
            ObjectPredicateDef::Supertype(supertype) => object.supertypes[supertype.index()],
            ObjectPredicateDef::AttackingOrBlocking => object.attacking_or_blocking,
            ObjectPredicateDef::SharesNameWithSource => {
                let name = self.object_card_name(object.id);
                name.is_some() && name == self.object_card_name(source)
            }
            ObjectPredicateDef::HasKeyword(keyword) => keyword
                .simple_index()
                .is_some_and(|index| object.keywords & (1 << index) != 0),
            ObjectPredicateDef::ControlledBy(relation) => {
                self.controller_of_object(source).is_some_and(|controller| {
                    self.player_relation_matches(
                        object.controller,
                        relation,
                        controller,
                        TriggerContext::empty(),
                    )
                })
            }
            ObjectPredicateDef::Attacking => {
                object.types.contains(CardType::Creature) && object.attacking
            }
            ObjectPredicateDef::All(predicates) => predicates
                .iter()
                .all(|predicate| self.trigger_object_matches(*predicate, object, source, is_spell)),
            ObjectPredicateDef::AnyOf(predicates) => predicates
                .iter()
                .any(|predicate| self.trigger_object_matches(*predicate, object, source, is_spell)),
            ObjectPredicateDef::Not(predicate) => {
                !self.trigger_object_matches(*predicate, object, source, is_spell)
            }
            ObjectPredicateDef::Special(_) => false,
        }
    }

    fn ability_targets_matching(
        &self,
        predicate: AbilityTargetPredicate,
        controller: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
    ) -> Vec<Target> {
        match predicate {
            AbilityTargetPredicate::AnyTarget => {
                let mut targets =
                    vec![Target::Player(PlayerId::One), Target::Player(PlayerId::Two)];
                targets.extend(
                    self.battlefield
                        .iter()
                        .filter(|permanent| {
                            (self.power(permanent).is_some()
                                || self
                                    .permanent_types(permanent)
                                    .is_some_and(|types| types.contains(CardType::Planeswalker)))
                                && self.permanent_can_be_targeted_by(permanent, controller, source)
                        })
                        .map(|permanent| Target::Permanent(permanent.card.id)),
                );
                targets
            }
            AbilityTargetPredicate::Player(relation) => [PlayerId::One, PlayerId::Two]
                .into_iter()
                .filter(|player| {
                    self.player_relation_matches(*player, relation, controller, context)
                })
                .map(Target::Player)
                .collect(),
            AbilityTargetPredicate::Object { .. } => {
                self.ability_object_targets_matching(predicate, controller, source, context)
            }
        }
    }

    fn ability_object_targets_matching(
        &self,
        predicate: AbilityTargetPredicate,
        controller: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
    ) -> Vec<Target> {
        let AbilityTargetPredicate::Object {
            object,
            zones,
            controller: controller_relation,
            owner: owner_relation,
        } = predicate
        else {
            unreachable!("object-target matching requires an object predicate")
        };
        let mut targets = Vec::new();
        if zones.contains(&ZoneKind::Battlefield) {
            targets.extend(self.battlefield.iter().filter_map(|permanent| {
                let characteristics = self.trigger_event_object(permanent);
                (controller_relation.is_none_or(|relation| {
                    self.player_relation_matches(
                        permanent.controller,
                        relation,
                        controller,
                        context,
                    )
                }) && owner_relation.is_none_or(|relation| {
                    self.player_relation_matches(
                        permanent.card.owner,
                        relation,
                        controller,
                        context,
                    )
                }) && self.permanent_can_be_targeted_by(permanent, controller, source)
                    && self.trigger_object_matches(object, &characteristics, source, false))
                .then_some(Target::Permanent(permanent.card.id))
            }));
        }
        if zones.contains(&ZoneKind::Stack) {
            targets.extend(self.stack.iter().filter_map(|stack_object| {
                let characteristics = self.stack_trigger_event_object(stack_object)?;
                (stack_object.kind == StackObjectKind::Spell
                    && controller_relation.is_none_or(|relation| {
                        self.player_relation_matches(
                            stack_object.controller,
                            relation,
                            controller,
                            context,
                        )
                    })
                    && owner_relation.is_none_or(|relation| {
                        self.player_relation_matches(
                            stack_object.card.owner,
                            relation,
                            controller,
                            context,
                        )
                    })
                    && self.trigger_object_matches(object, &characteristics, source, true))
                .then_some(Target::Spell(stack_object.id))
            }));
        }
        for zone in [
            ZoneKind::Library,
            ZoneKind::Hand,
            ZoneKind::Graveyard,
            ZoneKind::Exile,
            ZoneKind::Command,
        ] {
            if !zones.contains(&zone) || controller_relation.is_some() {
                continue;
            }
            targets.extend(self.cards_in_zone(zone).filter_map(|card| {
                (owner_relation.is_none_or(|relation| {
                    self.player_relation_matches(card.owner, relation, controller, context)
                }) && self.card_object_matches(object, card, zone, source))
                .then_some(Target::Card(card.id))
            }));
        }
        targets
    }

    fn cards_in_zone(&self, zone: ZoneKind) -> impl Iterator<Item = &CardInstance> {
        self.players.iter().flat_map(move |player| match zone {
            ZoneKind::Library => player.library.iter(),
            ZoneKind::Hand => player.hand.iter(),
            ZoneKind::Graveyard => player.graveyard.iter(),
            ZoneKind::Exile => player.exile.iter(),
            ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => [].iter(),
        })
    }

    fn card_in_nonbattlefield_zone(&self, id: GameObjectId) -> Option<(ZoneKind, &CardInstance)> {
        [
            ZoneKind::Library,
            ZoneKind::Hand,
            ZoneKind::Graveyard,
            ZoneKind::Exile,
        ]
        .into_iter()
        .find_map(|zone| {
            self.cards_in_zone(zone)
                .find(|card| card.id == id)
                .map(|card| (zone, card))
        })
    }

    fn card_object_matches(
        &self,
        predicate: ObjectPredicateDef,
        card: &CardInstance,
        zone: ZoneKind,
        source: GameObjectId,
    ) -> bool {
        let context = match zone {
            ZoneKind::Library => CharacteristicContext::Library,
            ZoneKind::Hand => CharacteristicContext::Hand,
            ZoneKind::Graveyard => CharacteristicContext::Graveyard,
            ZoneKind::Exile => CharacteristicContext::Exile,
            ZoneKind::Command => CharacteristicContext::Command,
            ZoneKind::Battlefield | ZoneKind::Stack => return false,
        };
        let Some(object) =
            self.printed_trigger_event_object(card.id, card.definition, card.owner, &context)
        else {
            return false;
        };
        self.trigger_object_matches(predicate, &object, source, false)
    }

    fn player_relation_matches(
        &self,
        player: PlayerId,
        relation: PlayerRelation,
        controller: PlayerId,
        context: TriggerContext,
    ) -> bool {
        match relation {
            PlayerRelation::Any => true,
            PlayerRelation::You => player == controller,
            PlayerRelation::NotYou => player != controller,
            PlayerRelation::Opponent => player == controller.opponent(),
            PlayerRelation::ActivePlayer => player == self.active_player,
            PlayerRelation::NonactivePlayer => player == self.active_player.opponent(),
            PlayerRelation::EventPlayer => context.event_player == Some(player),
        }
    }

    fn begin_trigger_placement(&mut self) {
        if self.pending_triggers.is_empty() {
            return;
        }
        let triggers = std::mem::take(&mut self.pending_triggers);
        let mut batches = Vec::new();
        for controller in [self.active_player, self.active_player.opponent()] {
            let controlled = triggers
                .iter()
                .filter(|trigger| trigger.controller == controller)
                .cloned()
                .collect::<Vec<_>>();
            if !controlled.is_empty() {
                batches.push(TriggerPlacementBatch {
                    controller,
                    triggers: controlled,
                });
            }
        }
        self.continue_trigger_placement(batches);
    }

    fn continue_trigger_placement(&mut self, mut batches: Vec<TriggerPlacementBatch>) {
        let Some(batch) = (!batches.is_empty()).then(|| batches.remove(0)) else {
            // APNAP determines only how simultaneous triggers are placed. The
            // player who was about to receive priority before placement keeps
            // it afterward (for example, the nonactive player who tapped City
            // of Brass after the active player passed).
            self.consecutive_passes = 0;
            return;
        };
        if batch.triggers.len() == 1 {
            self.place_trigger_sequence(batch.triggers, batches);
        } else {
            self.queue_trigger_order_decision(batch, batches);
        }
    }

    fn place_trigger_sequence(
        &mut self,
        mut triggers: Vec<PendingTrigger>,
        remaining: Vec<TriggerPlacementBatch>,
    ) {
        while !triggers.is_empty() {
            let trigger = triggers.remove(0);
            if trigger.targets.len() < trigger.target_defs.len() {
                self.queue_trigger_target_decision(trigger, triggers, remaining);
                return;
            }
            self.put_trigger_on_stack(trigger);
        }
        self.continue_trigger_placement(remaining);
    }

    fn queue_trigger_target_decision(
        &mut self,
        mut trigger: PendingTrigger,
        pending: Vec<PendingTrigger>,
        remaining: Vec<TriggerPlacementBatch>,
    ) {
        let target = trigger.target_defs[trigger.targets.len()];
        let candidates = self.ability_targets_matching(
            target.predicate,
            trigger.controller,
            trigger.source.object,
            trigger.context,
        );
        if candidates.len() < usize::from(target.minimum) {
            // A triggered ability with no legal choice for a required target
            // is removed from the stack as the placement procedure completes.
            self.place_trigger_sequence(pending, remaining);
            return;
        }
        if candidates.is_empty() && target.minimum == 0 {
            trigger
                .targets
                .push(TargetSelection::new(target.id, Vec::new()));
            let mut continued = vec![trigger];
            continued.extend(pending);
            self.place_trigger_sequence(continued, remaining);
            return;
        }

        let options = candidates
            .iter()
            .enumerate()
            .map(|(index, candidate)| DecisionOption {
                id: u32::try_from(index).unwrap_or(u32::MAX),
                label: self.target_label(trigger.controller, *candidate),
                card: self.target_card(*candidate),
                ability_text: None,
                zone: match candidate {
                    Target::Player(_) => DecisionZone::None,
                    Target::Card(id) => self.card_in_nonbattlefield_zone(*id).map_or(
                        DecisionZone::None,
                        |(zone, _)| match zone {
                            ZoneKind::Library => DecisionZone::Library,
                            ZoneKind::Hand => DecisionZone::Hand,
                            ZoneKind::Graveyard => DecisionZone::Graveyard,
                            ZoneKind::Exile => DecisionZone::Exile,
                            ZoneKind::Command => DecisionZone::Command,
                            ZoneKind::Battlefield | ZoneKind::Stack => DecisionZone::None,
                        },
                    ),
                    Target::Permanent(_) => DecisionZone::Battlefield,
                    Target::Spell(_) => DecisionZone::Stack,
                },
            })
            .collect::<Vec<_>>();
        let source_name = self
            .catalog
            .get(trigger.definition)
            .map_or("Triggered ability", |card| card.name.as_str());
        let target_effect = match trigger.effect {
            EffectDef::May(effect) => *effect,
            effect => effect,
        };
        let preference = if matches!(
            target_effect,
            EffectDef::ExileLinkedToSource {
                object: EffectRecipientDef::Target(slot),
            } if slot == target.id
        ) {
            DecisionPreference::LinkedExileTargets
        } else {
            DecisionPreference::Neutral
        };
        let id = self.next_decision_id;
        self.next_decision_id = self.next_decision_id.saturating_add(1);
        self.pending_decisions.insert(
            0,
            PendingDecision {
                observation: DecisionObservation {
                    id,
                    player: trigger.controller,
                    kind: DecisionKind::TriggerPlacement,
                    order_semantics: None,
                    prompt: format!("{source_name}: choose {}", target.label),
                    visibility: DecisionVisibility::Public,
                    preference,
                    minimum: usize::from(target.minimum),
                    maximum: usize::from(target.maximum).min(options.len()),
                    cancellable: false,
                    options,
                },
                continuation: DecisionContinuation::TriggerPlacement {
                    trigger,
                    pending,
                    remaining,
                    candidates,
                },
            },
        );
    }

    fn queue_trigger_order_decision(
        &mut self,
        batch: TriggerPlacementBatch,
        remaining: Vec<TriggerPlacementBatch>,
    ) {
        let options = batch
            .triggers
            .iter()
            .map(|trigger| {
                let name = self
                    .catalog
                    .get(trigger.definition)
                    .map_or("Triggered ability", |card| card.name.as_str());
                DecisionOption {
                    id: trigger.id,
                    label: format!("{name} triggered ability"),
                    card: Some((trigger.source.object, trigger.definition)),
                    ability_text: Some(trigger.text.into()),
                    zone: DecisionZone::Battlefield,
                }
            })
            .collect::<Vec<_>>();
        let count = options.len();
        let id = self.next_decision_id;
        self.next_decision_id = self.next_decision_id.saturating_add(1);
        // Trigger placement precedes any older legacy prompt that was queued
        // while the enclosing event was being processed.
        self.pending_decisions.insert(
            0,
            PendingDecision {
                observation: DecisionObservation {
                    id,
                    player: batch.controller,
                    kind: DecisionKind::TriggerOrder,
                    order_semantics: Some(DecisionOrderSemantics::Resolution),
                    prompt: "Choose triggered ability resolution order".into(),
                    visibility: DecisionVisibility::Public,
                    preference: DecisionPreference::Neutral,
                    minimum: count,
                    maximum: count,
                    cancellable: false,
                    options,
                },
                continuation: DecisionContinuation::TriggerOrder { batch, remaining },
            },
        );
    }

    fn complete_trigger_order(
        &mut self,
        batch: &TriggerPlacementBatch,
        remaining: Vec<TriggerPlacementBatch>,
        resolution_order: &[u32],
    ) {
        // The last object pushed is the first to resolve, so consume the
        // player-facing resolution order in reverse.
        let push_order = resolution_order
            .iter()
            .rev()
            .map(|trigger_id| {
                batch
                    .triggers
                    .iter()
                    .find(|trigger| trigger.id == *trigger_id)
                    .expect("validated trigger order contains each pending trigger")
                    .clone()
            })
            .collect();
        self.place_trigger_sequence(push_order, remaining);
    }

    fn target_card(&self, target: Target) -> Option<(GameObjectId, CardDefinitionId)> {
        match target {
            Target::Player(_) => None,
            Target::Card(id) => self
                .card_in_nonbattlefield_zone(id)
                .map(|(_, card)| (id, card.definition)),
            Target::Permanent(id) => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == id)
                .map(|permanent| (id, permanent.card.definition)),
            Target::Spell(id) => self
                .stack
                .iter()
                .find(|object| object.id == id)
                .map(|object| (id, object.card.definition)),
        }
    }

    fn put_trigger_on_stack(&mut self, trigger: PendingTrigger) {
        let card = self.unbacked_object(
            trigger.definition,
            trigger.owner,
            CharacteristicSource::Ability(trigger.definition),
        );
        let object = card.id;
        self.stack.push(StackObject {
            id: object,
            kind: StackObjectKind::TriggeredAbility,
            card,
            source: Some(trigger.source.object),
            ability: Some(StackAbilityPayload {
                origin: trigger.source.ability,
                definition: None,
                presentation_definition: trigger.definition,
                text: Some(trigger.text),
                target_defs: trigger.target_defs.to_vec(),
                targets: trigger.targets,
                context: trigger.context,
                resolver: trigger.resolver,
                condition: trigger.condition,
                mode_effects: Vec::new(),
                x: 0,
            }),
            controller: trigger.controller,
            signature: None,
            chosen_permanents: Vec::new(),
            applied_effects: Vec::new(),
            text_changes: Vec::new(),
            cast_via_flashback: false,
            is_copy: false,
        });
        self.events.push(GameEvent::TriggeredAbilityPutOnStack {
            player: trigger.controller,
            trigger: trigger.id,
            object,
            source: trigger.source.object,
            definition: trigger.definition,
        });
    }

    fn queue_optional_mana_payment(
        &mut self,
        player: PlayerId,
        cost: ManaCost,
        object: &StackObject,
        context: TriggerContext,
        effect: &'static EffectDef,
    ) {
        let mut options = vec![DecisionOption {
            id: 0,
            label: "Decline".into(),
            card: None,
            ability_text: None,
            zone: DecisionZone::None,
        }];
        if self.can_pay_cost(player, cost, 0) {
            options.push(DecisionOption {
                id: 1,
                label: "Pay the cost".into(),
                card: None,
                ability_text: None,
                zone: DecisionZone::None,
            });
        }
        self.queue_decision(
            player,
            object
                .ability_text()
                .unwrap_or("Pay the optional mana cost?"),
            DecisionVisibility::Private,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::OptionalManaPayment {
                player,
                cost,
                object: Box::new(object.clone()),
                context,
                effect,
            },
        );
    }

    /// Offers the spell's own controller the chance to keep it. A controller
    /// who cannot pay is not asked; the spell is simply countered.
    fn queue_counter_unless_paid(
        &mut self,
        spell: GameObjectId,
        amount: u16,
        zone: CounteredSpellZone,
    ) {
        let Some(controller) = self
            .stack
            .iter()
            .find(|object| object.id == spell)
            .map(|object| object.controller)
        else {
            return;
        };
        let cost = ManaCost::new(amount, 0);
        if !self.can_pay_cost(controller, cost, 0) {
            self.counter_spell_into(spell, zone);
            return;
        }
        self.queue_decision(
            controller,
            format!("Pay {amount} or your spell is countered"),
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            vec![
                DecisionOption {
                    id: 0,
                    label: "Let it be countered".into(),
                    card: None,
                    ability_text: None,
                    zone: DecisionZone::None,
                },
                DecisionOption {
                    id: 1,
                    label: "Pay the cost".into(),
                    card: None,
                    ability_text: None,
                    zone: DecisionZone::None,
                },
            ],
            DecisionContinuation::CounterUnlessPaid {
                spell,
                player: controller,
                cost,
                zone,
            },
        );
    }

    /// Offers an effect its controller may decline, resolving it only on a
    /// yes. Declining is always available, which is what "may" means.
    fn queue_optional_effect(
        &mut self,
        player: PlayerId,
        object: &StackObject,
        context: TriggerContext,
        effect: &'static EffectDef,
    ) {
        self.queue_decision(
            player,
            object.ability_text().unwrap_or("Use this optional effect?"),
            DecisionVisibility::Public,
            DecisionPreference::PreferOption(1),
            1..=1,
            false,
            vec![
                DecisionOption {
                    id: 0,
                    label: "Decline".into(),
                    card: None,
                    ability_text: None,
                    zone: DecisionZone::None,
                },
                DecisionOption {
                    id: 1,
                    label: "Do it".into(),
                    card: None,
                    ability_text: None,
                    zone: DecisionZone::None,
                },
            ],
            DecisionContinuation::OptionalEffect {
                object: Box::new(object.clone()),
                context,
                effect,
            },
        );
    }

    fn target_label(&self, viewer: PlayerId, target: Target) -> String {
        match target {
            Target::Player(player) if player == viewer => "you".into(),
            Target::Player(_) => "your opponent".into(),
            Target::Card(id) => self
                .card_in_nonbattlefield_zone(id)
                .and_then(|(_, card)| self.catalog.get(card.definition))
                .map_or_else(|| "that card".into(), |card| card.name.clone()),
            Target::Permanent(id) => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == id)
                .and_then(|permanent| self.catalog.get(permanent.card.definition))
                .map_or_else(|| "that permanent".into(), |card| card.name.clone()),
            Target::Spell(id) => self
                .stack
                .iter()
                .find(|object| object.id == id)
                .and_then(|object| self.catalog.get(object.card.definition))
                .map_or_else(|| "that spell".into(), |card| card.name.clone()),
        }
    }

    fn queue_chain_lightning_decision(&mut self, player: PlayerId, spell: StackObject) {
        // Without RR to spend there is nothing to decide, and a prompt whose
        // only answer is "no" is worse than no prompt at all.
        if !self.can_pay_cost(player, ManaCost::new(0, 2), 0) {
            return;
        }
        let mut targets = self.damage_targets();
        if let Some(target) = spell.first_target()
            && !targets.contains(&target)
        {
            targets.push(target);
        }
        let mut options = vec![DecisionOption {
            id: 0,
            label: "Don't copy Chain Lightning".into(),
            card: None,
            ability_text: None,
            zone: DecisionZone::None,
        }];
        options.extend(
            targets
                .iter()
                .enumerate()
                .map(|(index, target)| DecisionOption {
                    id: u32::try_from(index + 1).unwrap_or(u32::MAX),
                    label: format!(
                        "Copy Chain Lightning → {}",
                        self.target_label(player, *target)
                    ),
                    card: None,
                    ability_text: None,
                    zone: DecisionZone::None,
                }),
        );
        self.queue_decision(
            player,
            "Copy Chain Lightning?",
            DecisionVisibility::Private,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::ChainLightning {
                player,
                spell,
                targets,
            },
        );
    }

    fn queue_fork_decision(&mut self, player: PlayerId, spell: StackObject) {
        let target_lists = self.copy_target_choices(&spell, player);
        if spell
            .signature
            .as_ref()
            .is_some_and(|signature| signature.targets().is_empty())
        {
            self.push_copy(spell, player, Vec::new());
            return;
        }
        let original_targets = spell.targets();
        let options = target_lists
            .iter()
            .enumerate()
            .map(|(index, targets)| DecisionOption {
                id: u32::try_from(index).unwrap_or(u32::MAX),
                label: if flatten_target_selections(targets) == original_targets {
                    "Keep original targets".into()
                } else {
                    let labels = flatten_target_selections(targets)
                        .iter()
                        .map(|target| self.target_label(player, *target))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("Copy with targets {labels}")
                },
                card: None,
                ability_text: None,
                zone: DecisionZone::None,
            })
            .collect();
        self.queue_decision(
            player,
            "Choose targets for Fork's copy",
            DecisionVisibility::Private,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::Fork {
                player,
                spell,
                target_lists,
            },
        );
    }

    #[allow(clippy::too_many_lines)]
    fn copy_target_choices(
        &self,
        spell: &StackObject,
        player: PlayerId,
    ) -> Vec<Vec<TargetSelection>> {
        let Some(signature) = &spell.signature else {
            return Vec::new();
        };
        if signature.targets().is_empty() {
            return vec![Vec::new()];
        }
        let Some(definition) = self.catalog.get(spell.card.definition) else {
            return vec![signature.targets().to_vec()];
        };
        let Some(option) = definition.play_option(signature.play_option()) else {
            return vec![signature.targets().to_vec()];
        };
        let declarative_slots = spell
            .ability
            .as_ref()
            .map(|ability| ability.target_defs.clone())
            .filter(|slots| !slots.is_empty())
            .or_else(|| {
                Self::spell_ability(definition, option).and_then(|(_, ability)| {
                    let DeclarativeAbilityDef::Spell(spell) = ability.definition else {
                        return None;
                    };
                    Self::selected_spell_target_defs(spell, signature.modes())
                        .filter(|targets| !targets.is_empty())
                })
            });
        if let Some(slots) = declarative_slots {
            let context = spell
                .ability
                .as_ref()
                .map_or_else(TriggerContext::empty, |ability| ability.context);
            let mut choices = vec![Vec::new()];
            for original in signature.targets() {
                let Some(slot) = slots.iter().find(|slot| slot.id == original.slot()) else {
                    return vec![signature.targets().to_vec()];
                };
                let mut replacements = target_combinations(
                    &self.ability_targets_matching(slot.predicate, player, spell.id, context),
                    original.targets().len(),
                )
                .into_iter()
                .map(|targets| TargetSelection::new(slot.id, targets))
                .collect::<Vec<_>>();
                // Copy effects may keep the original target even if it has
                // since become illegal; normal resolution will then apply
                // the usual target-legality rules to the copy.
                replacements.push(original.clone());
                replacements.sort_unstable_by_key(|selection| selection.targets().to_vec());
                replacements.dedup();
                let mut combined = Vec::new();
                for prefix in &choices {
                    for replacement in &replacements {
                        let mut selected = prefix.clone();
                        selected.push(replacement.clone());
                        combined.push(selected);
                    }
                }
                choices = combined;
            }
            return choices;
        }
        let slots = Self::target_slots_for(option, signature.modes());
        if Self::uses_legacy_behavior_targets(definition, option) {
            let Some(behavior) = Self::play_option_behavior(definition, option) else {
                return vec![signature.targets().to_vec()];
            };
            let mut choices = self
                .legal_target_lists(
                    behavior,
                    signature.x(),
                    player,
                    Some(signature.iter_targets().count()),
                )
                .into_iter()
                .map(|targets| {
                    if targets.is_empty() {
                        Vec::new()
                    } else {
                        vec![TargetSelection::new(TargetSlotId(0), targets)]
                    }
                })
                .collect::<Vec<_>>();
            choices.push(signature.targets().to_vec());
            choices.sort_unstable_by_key(|targets| flatten_target_selections(targets));
            choices.dedup();
            return choices;
        }

        let mut choices = vec![Vec::new()];
        for original in signature.targets() {
            let Some(slot) = slots.iter().find(|slot| slot.id == original.slot()) else {
                return vec![signature.targets().to_vec()];
            };
            let mut replacements = target_combinations(
                &self.targets_matching(slot.predicate),
                original.targets().len(),
            )
            .into_iter()
            .map(|targets| TargetSelection::new(slot.id, targets))
            .collect::<Vec<_>>();
            replacements.push(original.clone());
            replacements.sort_unstable_by_key(|selection| selection.targets().to_vec());
            replacements.dedup();
            let mut combined = Vec::new();
            for prefix in &choices {
                for replacement in &replacements {
                    let mut selected = prefix.clone();
                    selected.push(replacement.clone());
                    combined.push(selected);
                }
            }
            choices = combined;
        }
        choices
    }

    fn queue_mana_vault_decision(&mut self, player: PlayerId, permanent: GameObjectId) {
        let mut options = vec![DecisionOption {
            id: 0,
            label: "Leave Mana Vault tapped".into(),
            card: None,
            ability_text: None,
            zone: DecisionZone::None,
        }];
        if self.can_pay_cost(player, ManaCost::new(4, 0), 0) {
            options.push(DecisionOption {
                id: 1,
                label: "Pay 4 to untap Mana Vault".into(),
                card: None,
                ability_text: None,
                zone: DecisionZone::None,
            });
        }
        self.queue_decision(
            player,
            "Mana Vault would remain tapped",
            DecisionVisibility::Private,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::ManaVault { player, permanent },
        );
    }

    fn queue_erhnam_decision(&mut self, player: PlayerId, source: GameObjectId) {
        let options = self
            .battlefield
            .iter()
            .filter(|permanent| {
                permanent.controller == player.opponent() && self.power(permanent).is_some()
            })
            .map(|permanent| {
                let name = self
                    .catalog
                    .get(permanent.card.definition)
                    .map_or("that creature", |card| card.name.as_str());
                DecisionOption {
                    id: permanent.card.id.0,
                    label: format!("Give {name} forestwalk"),
                    card: Some((permanent.card.id, permanent.card.definition)),
                    ability_text: None,
                    zone: DecisionZone::Battlefield,
                }
            })
            .collect::<Vec<_>>();
        if options.is_empty() {
            return;
        }
        self.queue_decision(
            player,
            "Erhnam Djinn: choose a creature for forestwalk",
            DecisionVisibility::Private,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::ErhnamForestwalk { player, source },
        );
    }

    fn push_copy(
        &mut self,
        mut spell: StackObject,
        player: PlayerId,
        targets: Vec<TargetSelection>,
    ) {
        let definition = spell.card.definition;
        let card = self.unbacked_object(definition, player, CharacteristicSource::Copy(definition));
        spell.id = card.id;
        spell.card = card;
        spell.source = None;
        spell.controller = player;
        if let Some(ability) = &mut spell.ability {
            ability.targets.clone_from(&targets);
        }
        spell.signature = spell.signature.as_ref().map(|signature| {
            signature
                .copy_with_targets(targets)
                .expect("copy replacement retains target slots and cardinality")
        });
        // Effects attached by mana spent on the original spell are not
        // copiable values. The copy keeps printed static abilities through
        // its definition, but it was not paid for with that mana.
        spell.applied_effects.clear();
        // Text-changing effects are not copiable values.
        spell.text_changes.clear();
        spell.is_copy = true;
        self.stack.push(spell);
    }

    fn push_activated_ability(
        &mut self,
        source: GameObjectId,
        source_card: &CardInstance,
        controller: PlayerId,
        frozen: FrozenActivatedAbility,
        targets: Vec<TargetSelection>,
        chosen_permanents: Vec<GameObjectId>,
    ) -> GameObjectId {
        let event_chosen_permanents = chosen_permanents.clone();
        let card = self.unbacked_object(
            frozen.presentation_definition,
            source_card.owner,
            CharacteristicSource::Ability(frozen.presentation_definition),
        );
        let id = card.id;
        self.stack.push(StackObject {
            id,
            kind: StackObjectKind::ActivatedAbility,
            card,
            source: Some(source),
            ability: Some(StackAbilityPayload {
                origin: frozen.origin,
                definition: frozen.definition,
                presentation_definition: frozen.presentation_definition,
                text: frozen.text,
                target_defs: frozen.target_defs.to_vec(),
                targets,
                context: TriggerContext::empty(),
                resolver: frozen.resolver,
                // Only a triggered ability carries an intervening-if.
                condition: None,
                mode_effects: Vec::new(),
                x: frozen.x,
            }),
            controller,
            signature: None,
            chosen_permanents,
            applied_effects: Vec::new(),
            text_changes: Vec::new(),
            cast_via_flashback: false,
            is_copy: false,
        });
        self.events.push(GameEvent::AbilityActivated {
            player: controller,
            object: id,
            source,
            definition: frozen.presentation_definition,
            chosen_permanents: event_chosen_permanents,
        });
        id
    }

    fn card_decision_options(
        &self,
        cards: &[CardInstance],
        zone: DecisionZone,
    ) -> Vec<DecisionOption> {
        cards
            .iter()
            .enumerate()
            .map(|(index, card)| DecisionOption {
                id: u32::try_from(index).unwrap_or(u32::MAX),
                label: self.catalog.get(card.definition).map_or_else(
                    || "Unknown card".into(),
                    |definition| definition.name.clone(),
                ),
                card: Some((card.id, card.definition)),
                ability_text: None,
                zone,
            })
            .collect()
    }

    fn queue_balance_task(
        &mut self,
        controller: PlayerId,
        phase: BalancePhase,
        task: BalanceTask,
        remaining: Vec<BalanceTask>,
    ) {
        let options = self.card_decision_options(&task.cards, task.zone);
        self.queue_decision(
            task.player,
            task.prompt.clone(),
            if task.zone == DecisionZone::Hand {
                DecisionVisibility::Private
            } else {
                DecisionVisibility::Public
            },
            DecisionPreference::LowerCardValue,
            task.count..=task.count,
            false,
            options,
            DecisionContinuation::Balance {
                controller,
                phase,
                task,
                remaining,
            },
        );
    }

    /// Asks `player` to pick the cards an effect makes them discard. A hand
    /// too small to cover the demand simply goes away in full, and an empty
    /// hand raises no decision at all.
    fn queue_effect_discard(&mut self, player: PlayerId, amount: i32, cause: ZoneMoveCause) {
        let hand = self.players[player.index()].hand.clone();
        let count = usize::try_from(amount).unwrap_or(0).min(hand.len());
        if count == 0 {
            return;
        }
        if count == hand.len() {
            let cards = hand.iter().map(|card| card.id).collect::<Vec<_>>();
            self.discard_cards_with_cause(player, &cards, cause);
            return;
        }
        let options = self.card_decision_options(&hand, DecisionZone::Hand);
        self.queue_decision(
            player,
            format!("Choose {count} card(s) to discard"),
            DecisionVisibility::Private,
            DecisionPreference::LowerCardValue,
            count..=count,
            false,
            options,
            DecisionContinuation::DiscardToEffect { player, cause },
        );
    }

    /// Whether a spell or ability an opponent of `player` controls can make
    /// them sacrifice a permanent. Sigarda says it cannot.
    fn can_be_forced_to_sacrifice(&self, player: PlayerId, caused_by: PlayerId) -> bool {
        if caused_by == player {
            return true;
        }
        !self.battlefield.iter().any(|permanent| {
            permanent.controller == player
                && self
                    .find_effective_ability(permanent, |effective| {
                        effective.ability.implementation.is_executable()
                            && matches!(
                                effective.ability.definition,
                                DeclarativeAbilityDef::Static(_)
                            )
                            && effective.ability.effect == EffectDef::CannotBeForcedToSacrifice
                    })
                    .is_some()
        })
    }

    /// Asks `player` which matching permanent they control to sacrifice. With
    /// nothing matching there is no choice and no sacrifice; with exactly one
    /// there is nothing to ask.
    fn queue_chosen_sacrifice(
        &mut self,
        player: PlayerId,
        predicate: ObjectPredicateDef,
        source: GameObjectId,
    ) {
        let candidates = self
            .battlefield
            .iter()
            .filter(|permanent| permanent.controller == player)
            .filter(|permanent| {
                self.trigger_object_matches(
                    predicate,
                    &self.trigger_event_object(permanent),
                    source,
                    false,
                )
            })
            .map(|permanent| permanent.card.clone())
            .collect::<Vec<_>>();
        match candidates.as_slice() {
            [] => {}
            [only] => self.destroy_permanents(&[only.id], false),
            _ => {
                let options = self.card_decision_options(&candidates, DecisionZone::Battlefield);
                self.queue_decision(
                    player,
                    "Choose a permanent to sacrifice",
                    DecisionVisibility::Public,
                    DecisionPreference::LowerCardValue,
                    1..=1,
                    false,
                    options,
                    DecisionContinuation::SacrificeOfChoice,
                );
            }
        }
    }

    fn queue_time_vault_decision(&mut self, permanent: GameObjectId, remaining: Vec<GameObjectId>) {
        let card = self
            .battlefield
            .iter()
            .find(|candidate| candidate.card.id == permanent)
            .map(|permanent| (permanent.card.id, permanent.card.definition));
        self.queue_decision(
            self.active_player,
            "Time Vault would remain tapped",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            vec![
                DecisionOption {
                    id: 0,
                    label: "Leave Time Vault tapped".into(),
                    card,
                    ability_text: None,
                    zone: DecisionZone::Battlefield,
                },
                DecisionOption {
                    id: 1,
                    label: "Untap Time Vault and skip your next turn".into(),
                    card,
                    ability_text: None,
                    zone: DecisionZone::Battlefield,
                },
            ],
            DecisionContinuation::TimeVault {
                permanent,
                remaining,
            },
        );
    }

    fn finish_untap_choices(&mut self) {
        let mut vaults = self
            .battlefield
            .iter()
            .filter(|permanent| {
                permanent.controller == self.active_player
                    && permanent.tapped
                    && self.effective_behavior(permanent) == Some(CardBehavior::TimeVault)
            })
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>();
        if vaults.is_empty() {
            self.handle_upkeep_triggers();
        } else {
            let first = vaults.remove(0);
            self.queue_time_vault_decision(first, vaults);
        }
    }

    fn queue_sylvan_select(
        &mut self,
        player: PlayerId,
        candidates: Vec<GameObjectId>,
        choices_left: usize,
    ) {
        let cards = self.players[player.index()]
            .hand
            .iter()
            .filter(|card| candidates.contains(&card.id))
            .cloned()
            .collect::<Vec<_>>();
        let options = self.card_decision_options(&cards, DecisionZone::DrawnThisStep);
        self.queue_decision(
            player,
            format!("Choose a card drawn this step ({choices_left} remaining)"),
            DecisionVisibility::Private,
            DecisionPreference::LowerCardValue,
            1..=1,
            false,
            options,
            DecisionContinuation::SylvanSelect {
                player,
                candidates,
                choices_left,
            },
        );
    }

    fn queue_sylvan_mode(
        &mut self,
        player: PlayerId,
        card: GameObjectId,
        candidates: Vec<GameObjectId>,
        choices_left: usize,
    ) {
        let card_info = self.players[player.index()]
            .hand
            .iter()
            .find(|candidate| candidate.id == card)
            .map(|card| (card.id, card.definition));
        let card_name = card_info
            .and_then(|(_, definition)| self.catalog.get(definition))
            .map_or("this card", |card| card.name.as_str());
        let mut options = vec![DecisionOption {
            id: 0,
            label: format!("Put {card_name} back on top"),
            card: card_info,
            ability_text: None,
            zone: DecisionZone::DrawnThisStep,
        }];
        if self.players[player.index()].life >= 4 {
            options.push(DecisionOption {
                id: 1,
                label: format!("Pay 4 life to keep {card_name}"),
                card: card_info,
                ability_text: None,
                zone: DecisionZone::DrawnThisStep,
            });
        }
        self.queue_decision(
            player,
            format!("Keep {card_name}?"),
            DecisionVisibility::Private,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::SylvanMode {
                player,
                card,
                candidates,
                choices_left,
            },
        );
    }

    #[allow(clippy::too_many_lines)]
    fn choose_decision(&mut self, player: PlayerId, decision: u32, options: &[u32]) {
        let pending = self.pending_decisions.remove(0);
        debug_assert_eq!(pending.observation.id, decision);
        match pending.continuation {
            DecisionContinuation::BasicLandTypeTextChange { target } => {
                let Some(option) = options.first().copied() else {
                    return;
                };
                let width = u32::try_from(BasicLandType::ALL.len())
                    .expect("the basic-land-type count fits u32");
                let Some(from) = usize::try_from(option / width)
                    .ok()
                    .and_then(BasicLandType::from_index)
                else {
                    return;
                };
                let Some(to) = usize::try_from(option % width)
                    .ok()
                    .and_then(BasicLandType::from_index)
                else {
                    return;
                };
                if from == to {
                    return;
                }
                let change = BasicLandTypeChange { from, to };
                match target {
                    Target::Permanent(id) => {
                        if let Some(permanent) = self
                            .battlefield
                            .iter_mut()
                            .find(|permanent| permanent.card.id == id)
                        {
                            permanent.text_changes.push(change);
                        }
                    }
                    Target::Spell(id) => {
                        if let Some(index) = self.stack.iter().position(|spell| spell.id == id) {
                            self.stack[index].text_changes.push(change);
                        }
                    }
                    Target::Player(_) | Target::Card(_) => {}
                }
            }
            DecisionContinuation::ExileFromHand { victim } => {
                let Some((card, _)) = pending
                    .observation
                    .options
                    .iter()
                    .find(|option| options.contains(&option.id))
                    .and_then(|option| option.card)
                else {
                    return;
                };
                if let Some(card) = remove_card(&mut self.players[victim.index()].hand, card) {
                    let (card, _zone_change) = self.zone_change_card(card);
                    self.players[victim.index()].exile.push(card);
                }
            }
            DecisionContinuation::AugurOfBolas { player, revealed } => {
                let kept = pending
                    .observation
                    .options
                    .iter()
                    .find(|option| options.contains(&option.id))
                    .and_then(|option| option.card)
                    .map(|(card, _)| card);
                let (to_hand, to_bottom): (Vec<_>, Vec<_>) =
                    revealed.into_iter().partition(|card| Some(card.id) == kept);
                for card in to_hand {
                    let (card, _zone_change) = self.zone_change_card(card);
                    self.players[player.index()].hand.push(card);
                }
                // "In any order" -- printed order is as good as any, and the
                // rest of the library is already unknown to everyone.
                for card in to_bottom {
                    let (card, _zone_change) = self.zone_change_card(card);
                    self.players[player.index()].library.push(card);
                }
            }
            DecisionContinuation::ShockLand {
                player,
                permanent,
                life,
            } => {
                if options.contains(&1) {
                    if let Some(land) = self
                        .battlefield
                        .iter_mut()
                        .find(|candidate| candidate.card.id == permanent)
                    {
                        land.tapped = false;
                    }
                    self.lose_life(player, u16::from(life));
                    self.check_life_totals();
                }
            }
            DecisionContinuation::ChooseCreatureType {
                player,
                mut permanent,
                entry,
                choices,
            } => {
                let Some(selected) = options
                    .first()
                    .and_then(|option| usize::try_from(*option).ok())
                    .and_then(|index| choices.get(index))
                    .cloned()
                else {
                    return;
                };
                permanent.chosen_creature_type = Some(selected);
                self.finish_land_entry(player, *permanent, entry);
            }
            DecisionContinuation::OptionalManaPayment {
                player,
                cost,
                object,
                context,
                effect,
            } => {
                if options.contains(&1) {
                    self.activate_mana_for_cost(player, cost, 0);
                    let _ = self.pay_player_cost(player, cost, 0);
                    self.resolve_effect_def(*effect, &object, context);
                }
            }
            DecisionContinuation::ChainLightning {
                player,
                spell,
                targets,
            } => {
                if let Some(option) = options.first().copied()
                    && option > 0
                    && let Some(target) = targets.get(usize::try_from(option - 1).unwrap_or(0))
                {
                    let cost = ManaCost::new(0, 2);
                    self.activate_mana_for_cost(player, cost, 0);
                    let _ = self.pay_player_cost(player, cost, 0);
                    let replacements = spell
                        .signature
                        .as_ref()
                        .and_then(|signature| signature.targets().first())
                        .map(|selection| vec![TargetSelection::single(selection.slot(), *target)])
                        .unwrap_or_default();
                    self.push_copy(spell, player, replacements);
                }
            }
            DecisionContinuation::Fork {
                player,
                spell,
                target_lists,
            } => {
                if let Some(option) = options.first().copied()
                    && let Some(targets) = target_lists.get(usize::try_from(option).unwrap_or(0))
                {
                    self.push_copy(spell, player, targets.clone());
                }
            }
            DecisionContinuation::ManaVault { player, permanent } => {
                let cost = ManaCost::new(4, 0);
                // Multiple tapped Mana Vaults queue their upkeep decisions at
                // once.  Paying for an earlier vault can make a later
                // decision's previously-offered payment option stale.
                if options.contains(&1) && self.can_pay_cost(player, cost, 0) {
                    self.activate_mana_for_cost(player, cost, 0);
                    let _ = self.pay_player_cost(player, cost, 0);
                    if let Some(vault) = self
                        .battlefield
                        .iter_mut()
                        .find(|candidate| candidate.card.id == permanent)
                    {
                        vault.tapped = false;
                    }
                }
            }
            DecisionContinuation::GrislySalvage { player, revealed } => {
                let kept = pending
                    .observation
                    .options
                    .iter()
                    .find(|option| options.contains(&option.id))
                    .and_then(|option| option.card)
                    .map(|(card, _)| card);
                let (to_hand, to_graveyard): (Vec<_>, Vec<_>) =
                    revealed.into_iter().partition(|card| Some(card.id) == kept);
                for card in to_hand {
                    let (card, _zone_change) = self.zone_change_card(card);
                    self.players[player.index()].hand.push(card);
                }
                self.bury_cards(player, to_graveyard);
            }
            DecisionContinuation::CounterUnlessPaid {
                spell,
                player,
                cost,
                zone,
            } => {
                if options.contains(&1) {
                    self.activate_mana_for_cost(player, cost, 0);
                    let _ = self.pay_player_cost(player, cost, 0);
                } else {
                    self.counter_spell_into(spell, zone);
                }
            }
            DecisionContinuation::OptionalEffect {
                object,
                context,
                effect,
            } => {
                if options.contains(&1) {
                    self.resolve_effect_def(*effect, &object, context);
                }
            }
            DecisionContinuation::SacrificeOfChoice => {
                let sacrificed = pending
                    .observation
                    .options
                    .iter()
                    .filter(|option| options.contains(&option.id))
                    .filter_map(|option| option.card.map(|(card, _)| card))
                    .collect::<Vec<_>>();
                self.destroy_permanents(&sacrificed, false);
            }
            DecisionContinuation::DiscardToEffect { player, cause } => {
                let discarded = pending
                    .observation
                    .options
                    .iter()
                    .filter(|option| options.contains(&option.id))
                    .filter_map(|option| option.card.map(|(card, _)| card))
                    .collect::<Vec<_>>();
                self.discard_cards_with_cause(player, &discarded, cause);
            }
            DecisionContinuation::Duress { victim, cause } => {
                let Some(option) = pending
                    .observation
                    .options
                    .iter()
                    .find(|option| options.contains(&option.id))
                else {
                    return;
                };
                let Some((card, _)) = option.card else {
                    return;
                };
                self.discard_cards_with_cause(victim, &[card], cause);
            }
            DecisionContinuation::Tutor => {
                let found = pending
                    .observation
                    .options
                    .iter()
                    .find(|option| options.contains(&option.id))
                    .and_then(|option| option.card);
                if let Some((card, _)) = found
                    && let Some(card) = remove_card(&mut self.players[player.index()].library, card)
                {
                    let (card, _zone_change) = self.zone_change_card(card);
                    self.players[player.index()].hand.push(card);
                }
                // The card says "then shuffle", and a search that finds
                // nothing still searched. Skipping this would hand a player
                // their library order for free: tutor, fail to find, and the
                // top of the deck is whatever it already was.
                self.rng.shuffle(&mut self.players[player.index()].library);
            }
            DecisionContinuation::RecallCost {
                player,
                card,
                choices,
            } => {
                let discarded = pending
                    .observation
                    .options
                    .iter()
                    .filter(|option| options.contains(&option.id))
                    .filter_map(|option| option.card.map(|(card, _)| card))
                    .collect::<Vec<_>>();
                self.discard_cards_with_cause(player, &discarded, ZoneMoveCause::Cost);
                self.finish_cast_spell(player, card, &choices, &[]);
            }
            DecisionContinuation::RecallReturn { player } => {
                for option in &pending.observation.options {
                    if options.contains(&option.id)
                        && let Some((card, _)) = option.card
                        && let Some(card) =
                            remove_card(&mut self.players[player.index()].graveyard, card)
                    {
                        let (card, _zone_change) = self.zone_change_card(card);
                        self.players[player.index()].hand.push(card);
                    }
                }
            }
            DecisionContinuation::Balance {
                controller,
                phase,
                task,
                mut remaining,
            } => {
                let mut discards = Vec::new();
                let mut sacrifices = Vec::new();
                for option in &pending.observation.options {
                    if !options.contains(&option.id) {
                        continue;
                    }
                    let Some((card, _)) = option.card else {
                        continue;
                    };
                    match task.action {
                        BalanceAction::Sacrifice => sacrifices.push(card),
                        BalanceAction::Discard => discards.push(card),
                    }
                }
                self.destroy_permanents(&sacrifices, false);
                self.discard_cards_with_cause(task.player, &discards, task.cause);
                if !remaining.is_empty() {
                    let next = remaining.remove(0);
                    self.queue_balance_task(controller, phase, next, remaining);
                } else if let Some(next) = phase.next() {
                    self.queue_balance_phase(controller, next);
                }
            }
            DecisionContinuation::TimeVault {
                permanent,
                mut remaining,
            } => {
                if options.contains(&1) {
                    if let Some(vault) = self
                        .battlefield
                        .iter_mut()
                        .find(|candidate| candidate.card.id == permanent)
                    {
                        vault.tapped = false;
                    }
                    self.skipped_turns[player.index()] =
                        self.skipped_turns[player.index()].saturating_add(1);
                }
                if remaining.is_empty() {
                    self.handle_upkeep_triggers();
                } else {
                    let next = remaining.remove(0);
                    self.queue_time_vault_decision(next, remaining);
                }
            }
            DecisionContinuation::SylvanSelect {
                player,
                mut candidates,
                choices_left,
            } => {
                let selected = pending
                    .observation
                    .options
                    .iter()
                    .find(|option| options.contains(&option.id))
                    .and_then(|option| option.card)
                    .map(|(card, _)| card);
                if let Some(card) = selected {
                    candidates.retain(|candidate| *candidate != card);
                    self.queue_sylvan_mode(player, card, candidates, choices_left);
                }
            }
            DecisionContinuation::SylvanMode {
                player,
                card,
                candidates,
                choices_left,
            } => {
                if options.contains(&1) {
                    self.players[player.index()].life -= 4;
                    self.check_life_totals();
                } else if let Some(card) = remove_card(&mut self.players[player.index()].hand, card)
                {
                    let (card, _zone_change) = self.zone_change_card(card);
                    self.players[player.index()].library.push(card);
                }
                if choices_left > 1 && self.result.is_none() {
                    self.queue_sylvan_select(player, candidates, choices_left - 1);
                }
            }
            DecisionContinuation::ErhnamForestwalk { player, source } => {
                let Some(target) = pending
                    .observation
                    .options
                    .iter()
                    .find(|option| options.contains(&option.id))
                    .and_then(|option| option.card)
                    .map(|(card, _)| card)
                else {
                    return;
                };
                let can_grant = self
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == target)
                    .is_some_and(|permanent| {
                        permanent.controller == player.opponent() && self.power(permanent).is_some()
                    });
                if can_grant
                    && let Some(permanent) = self
                        .battlefield
                        .iter_mut()
                        .find(|permanent| permanent.card.id == target)
                {
                    permanent.forestwalk_until_upkeep_of = Some(player);
                    self.events.push(GameEvent::ErhnamForestwalkGranted {
                        player,
                        source,
                        target,
                    });
                }
            }
            DecisionContinuation::TriggerOrder { batch, remaining } => {
                self.complete_trigger_order(&batch, remaining, options);
            }
            DecisionContinuation::TriggerPlacement {
                mut trigger,
                pending,
                remaining,
                candidates,
            } => {
                let target = trigger.target_defs[trigger.targets.len()];
                let selected = options
                    .iter()
                    .filter_map(|option| {
                        usize::try_from(*option)
                            .ok()
                            .and_then(|index| candidates.get(index))
                            .copied()
                    })
                    .collect();
                trigger
                    .targets
                    .push(TargetSelection::new(target.id, selected));
                let mut continued = vec![trigger];
                continued.extend(pending);
                self.place_trigger_sequence(continued, remaining);
            }
        }
    }

    fn cancel_decision(&mut self, decision: u32) {
        debug_assert_eq!(self.pending_decisions[0].observation.id, decision);
        self.pending_decisions.remove(0);
    }

    fn add_land_actions(&self, player: PlayerId, actions: &mut Vec<Action>) {
        let state = &self.players[player.index()];
        if player != self.active_player
            || !self.step.is_main()
            || !self.stack.is_empty()
            || state.land_played_this_turn
        {
            return;
        }
        for card in &state.hand {
            let Some(definition) = self.catalog.get(card.definition) else {
                continue;
            };
            actions.extend(
                definition
                    .play_options
                    .iter()
                    .filter(|option| option.action == PlayActionKind::PlayLand)
                    .filter(|option| match &option.form {
                        crate::card::SpellForm::Part(part) => definition
                            .part(*part)
                            .is_some_and(|part| part.rules.has_type(CardType::Land)),
                        crate::card::SpellForm::Combined(_) => false,
                    })
                    .map(|option| Action::PlayLand {
                        card: card.id,
                        option: option.id,
                    }),
            );
        }
    }

    #[allow(clippy::too_many_lines)]
    fn add_spell_actions(&self, player: PlayerId, actions: &mut Vec<Action>) {
        let state = &self.players[player.index()];
        for (card, source_zone) in state
            .hand
            .iter()
            .map(|card| (card, CastSourceZone::Hand))
            .chain(
                state
                    .graveyard
                    .iter()
                    .map(|card| (card, CastSourceZone::Graveyard)),
            )
        {
            let Some(definition) = self.catalog.get(card.definition) else {
                continue;
            };
            for option in definition
                .play_options
                .iter()
                .filter(|option| option.action == PlayActionKind::CastSpell)
            {
                if source_zone == CastSourceZone::Graveyard
                    && option.restriction == PlayRestriction::FromHandOnly
                {
                    continue;
                }
                // A declarative card intentionally has no custom behavior.
                // `Unsupported` is only a local neutral value for the legacy
                // helpers below; it is not stored as part of that card's rules.
                let behavior = Self::play_option_behavior(definition, option)
                    .unwrap_or(CardBehavior::Unsupported);
                let Some(types) = Self::play_option_types(definition, option) else {
                    continue;
                };
                // Metadata-only creatures retain baseline casting/combat. A
                // metadata-only noncreature spell or modal branch must not be
                // exposed as a legal action that would silently do nothing.
                if option.effect_status == CardEffectStatus::MetadataOnly && !types.is_creature() {
                    continue;
                }
                let part_has_flash = match &option.form {
                    crate::card::SpellForm::Part(part) => {
                        definition.part(*part).is_some_and(|part| {
                            part.rules.has_executable_keyword(KeywordAbility::Flash)
                        })
                    }
                    crate::card::SpellForm::Combined(parts) => parts.iter().any(|part| {
                        definition.part(*part).is_some_and(|part| {
                            part.rules.has_executable_keyword(KeywordAbility::Flash)
                        })
                    }),
                };
                // A granted flash covers the next sorcery whenever it is
                // cast, so it only matters when the timing would refuse.
                let granted_flash = types.contains(CardType::Sorcery)
                    && self.sorcery_flash_grants[player.index()] > 0;
                if !types.contains(CardType::Instant)
                    && !part_has_flash
                    && !granted_flash
                    && (player != self.active_player
                        || !self.step.is_main()
                        || !self.stack.is_empty())
                {
                    continue;
                }
                let payment_purpose = ManaPaymentPurpose::Spell {
                    object: card.id,
                    definition: card.definition,
                    controller: player,
                    form: option.form.clone(),
                };

                for modes in Self::implemented_mode_selections(option) {
                    let declared_slots = Self::target_slots_for(option, &modes);
                    let _ = self.visit_cost_configurations(
                        definition,
                        card.id,
                        option,
                        source_zone,
                        |costs| {
                            let alternative_kind =
                                self.selected_alternative_kind(definition, option, card.id, &costs);
                            if alternative_kind == Some(AlternativeCastKindDef::Overload)
                                && !modes.is_empty()
                            {
                                return ControlFlow::Continue(());
                            }
                            let Some(cost) =
                                self.configured_cast_mana_cost(card.id, option, &costs)
                            else {
                                return ControlFlow::Continue(());
                            };
                            let max_x = if cost.variable_x {
                                self.maximum_x_for(player, cost, &payment_purpose)
                            } else {
                                0
                            };
                            for x in 0..=max_x {
                                if behavior == CardBehavior::Recall
                                    && usize::from(x)
                                        > state.hand.len().saturating_sub(usize::from(
                                            source_zone == CastSourceZone::Hand,
                                        ))
                                {
                                    continue;
                                }
                                let target_choices = if alternative_kind
                                    == Some(AlternativeCastKindDef::Overload)
                                {
                                    vec![Vec::new()]
                                } else if let Some((_, ability)) =
                                    Self::spell_ability(definition, option)
                                {
                                    let DeclarativeAbilityDef::Spell(spell) = ability.definition
                                    else {
                                        unreachable!("spell_ability returns a spell clause")
                                    };
                                    let Some(targets) =
                                        Self::selected_spell_target_defs(spell, &modes)
                                    else {
                                        continue;
                                    };
                                    self.legal_ability_target_selections(
                                        &targets,
                                        player,
                                        card.id,
                                        TriggerContext::empty(),
                                    )
                                } else if Self::uses_legacy_behavior_targets(definition, option) {
                                    self.legacy_target_selections(behavior, x, player)
                                } else {
                                    self.legal_target_selections(&declared_slots)
                                };
                                for targets in &target_choices {
                                    let target_count = targets
                                        .iter()
                                        .map(|selection| selection.targets().len())
                                        .sum();
                                    let payable_cost = reduce_generic(
                                        add_generic(
                                            cost,
                                            fireball_extra_cost(behavior, target_count),
                                        ),
                                        self.spell_cost_reduction(definition.id, player),
                                    );
                                    if !self.can_pay_cost_for(
                                        player,
                                        payable_cost,
                                        x,
                                        &payment_purpose,
                                    ) {
                                        continue;
                                    }
                                    let sacrifice_choices = if behavior
                                        == CardBehavior::GoblinGrenade
                                    {
                                        self.battlefield
                                            .iter()
                                            .filter(|permanent| {
                                                permanent.controller == player
                                                    && self.effective_rules(permanent).is_some_and(
                                                        |rules| rules.has_subtype("Goblin"),
                                                    )
                                            })
                                            .map(|permanent| vec![permanent.card.id])
                                            .collect()
                                    } else {
                                        vec![Vec::new()]
                                    };
                                    for sacrifices in sacrifice_choices {
                                        actions.push(Action::CastSpell {
                                            card: card.id,
                                            choices: CastChoices::new(option.id)
                                                .with_modes(modes.clone())
                                                .with_costs(costs.clone())
                                                .with_x(x)
                                                .with_targets(targets.clone()),
                                            sacrifices,
                                        });
                                    }
                                }
                            }
                            ControlFlow::Continue(())
                        },
                    );
                }
            }
        }
    }

    fn play_option_types(
        definition: &CardDefinition,
        option: &PlayOptionDef,
    ) -> Option<CardTypeSet> {
        match &option.form {
            crate::card::SpellForm::Part(part) => {
                definition.part(*part).map(|part| part.rules.types())
            }
            crate::card::SpellForm::Combined(parts) => {
                let mut combined = CardTypeSet::empty();
                let mut found = false;
                for part in parts {
                    combined = combined.union(definition.part(*part)?.rules.types());
                    found = true;
                }
                found.then_some(combined)
            }
        }
    }

    fn play_option_behavior(
        definition: &CardDefinition,
        option: &PlayOptionDef,
    ) -> Option<CardBehavior> {
        let first = match &option.form {
            crate::card::SpellForm::Part(part) => *part,
            crate::card::SpellForm::Combined(parts) => *parts.first()?,
        };
        definition
            .part(first)
            .and_then(|part| part.rules.special_behavior())
    }

    fn spell_ability(
        definition: &CardDefinition,
        option: &PlayOptionDef,
    ) -> Option<(AbilityOrigin, AbilityDef)> {
        let crate::card::SpellForm::Part(part_id) = &option.form else {
            return None;
        };
        let part_id = *part_id;
        let part = definition.part(part_id)?;
        part.rules
            .indexed_abilities()
            .find(|attached| {
                attached.definition.implementation.is_executable()
                    && matches!(
                        attached.definition.definition,
                        DeclarativeAbilityDef::Spell(_)
                    )
            })
            .map(|attached| {
                (
                    AbilityOrigin::Printed {
                        definition: definition.id,
                        part: part_id,
                        ability: attached.id,
                    },
                    attached.definition,
                )
            })
    }

    fn selected_spell_target_defs(
        spell: crate::card::SpellAbilityDef,
        selected_modes: &[ModeId],
    ) -> Option<Vec<AbilityTargetDef>> {
        let mut targets = spell.targets().to_vec();
        if spell.modal().is_none() {
            return selected_modes.is_empty().then_some(targets);
        }
        for selected in selected_modes {
            let mode = spell.mode(*selected)?;
            let DeclarativeAbilityDef::Spell(mode_spell) = mode.definition else {
                return None;
            };
            targets.extend_from_slice(mode_spell.targets());
        }
        Some(targets)
    }

    fn selected_spell_mode_effects(
        spell: crate::card::SpellAbilityDef,
        selected_modes: &[ModeId],
    ) -> Option<Vec<EffectDef>> {
        if spell.modal().is_none() {
            return selected_modes.is_empty().then_some(Vec::new());
        }
        let mut selected = selected_modes.to_vec();
        selected.sort_by_key(|mode| mode.index());
        selected
            .into_iter()
            .map(|mode| {
                let mode = spell.mode(mode)?;
                (mode.implementation == AbilityImplementationDef::Definition).then_some(mode.effect)
            })
            .collect()
    }

    fn alternative_cast_clause(
        definition: &CardDefinition,
        option: &PlayOptionDef,
        alternative: AlternativeCostId,
    ) -> Option<(AbilityOrigin, AbilityDef, AlternativeCastKindDef)> {
        let parts: &[CardPartId] = match &option.form {
            crate::card::SpellForm::Part(part) => std::slice::from_ref(part),
            crate::card::SpellForm::Combined(parts) => parts,
        };
        parts.iter().find_map(|part_id| {
            definition
                .part(*part_id)?
                .rules
                .indexed_abilities()
                .find_map(|attached| {
                    let DeclarativeAbilityDef::AlternativeCast(alternative_cast) =
                        attached.definition.definition
                    else {
                        return None;
                    };
                    (attached.alternative_cost_id() == Some(alternative)).then_some((
                        AbilityOrigin::Printed {
                            definition: definition.id,
                            part: *part_id,
                            ability: attached.id,
                        },
                        attached.definition,
                        alternative_cast.kind,
                    ))
                })
        })
    }

    fn alternative_cast_ability(
        definition: &CardDefinition,
        option: &PlayOptionDef,
        alternative: AlternativeCostId,
    ) -> Option<(AbilityOrigin, AbilityDef, AlternativeCastKindDef)> {
        Self::alternative_cast_clause(definition, option, alternative)
            .filter(|(_, ability, _)| ability.implementation.is_executable())
    }

    fn selected_alternative_kind(
        &self,
        definition: &CardDefinition,
        option: &PlayOptionDef,
        card: GameObjectId,
        costs: &CostConfiguration,
    ) -> Option<AlternativeCastKindDef> {
        let selected = costs.alternative()?;
        if Some(selected) == Self::temporary_alternative_cost_id(option)
            && self.granted_flashback(card, option).is_some()
        {
            return Some(AlternativeCastKindDef::Flashback);
        }
        Self::alternative_cast_ability(definition, option, selected).map(|(_, _, kind)| kind)
    }

    fn temporary_alternative_cost_id(option: &PlayOptionDef) -> Option<AlternativeCostId> {
        (u8::MIN..=u8::MAX)
            .rev()
            .map(AlternativeCostId)
            .find(|candidate| {
                option
                    .alternative_costs
                    .iter()
                    .all(|cost| cost.id != *candidate)
            })
    }

    fn granted_flashback(
        &self,
        card: GameObjectId,
        option: &PlayOptionDef,
    ) -> Option<(AlternativeCastAbilityDef, ManaCost)> {
        self.temporary_ability_grants
            .iter()
            .filter(|grant| grant.object == card)
            .find_map(|grant| {
                if !grant.ability.implementation.is_executable() {
                    return None;
                }
                let DeclarativeAbilityDef::AlternativeCast(alternative) = grant.ability.definition
                else {
                    return None;
                };
                (alternative.kind == AlternativeCastKindDef::Flashback)
                    .then(|| alternative.mana_cost.resolve(option.mana_cost))
                    .flatten()
                    .map(|mana_cost| (alternative, mana_cost))
            })
    }
    fn spell_custom_followup(
        definition: &CardDefinition,
        option: &PlayOptionDef,
        primary: AbilityId,
    ) -> Option<CardBehavior> {
        let crate::card::SpellForm::Part(part_id) = &option.form else {
            return None;
        };
        definition
            .part(*part_id)?
            .rules
            .indexed_abilities()
            .find_map(|attached| {
                (attached.id != primary)
                    .then(|| attached.definition.implementation.custom_behavior())
                    .flatten()
            })
    }

    fn frozen_spell_payload(
        &self,
        definition_id: CardDefinitionId,
        signature: &CastSignature,
    ) -> Option<StackAbilityPayload> {
        let definition = self.catalog.get(definition_id)?;
        let option = definition.play_option(signature.play_option())?;
        if let Some(selected) = signature.costs().alternative()
            && let Some((origin, ability, AlternativeCastKindDef::Overload)) =
                Self::alternative_cast_ability(definition, option, selected)
        {
            let DeclarativeAbilityDef::AlternativeCast(alternative_cast) = ability.definition
            else {
                unreachable!("alternative_cast_ability returns an alternative-cast clause")
            };
            return Some(StackAbilityPayload {
                origin,
                definition: Some(Box::new(ability)),
                presentation_definition: definition_id,
                text: alternative_cast.stack_text.or(Some(ability.text)),
                target_defs: Vec::new(),
                targets: signature.targets().to_vec(),
                context: TriggerContext::empty(),
                resolver: StackAbilityResolver::Declarative(ability.effect),
                condition: None,
                mode_effects: Vec::new(),
                x: signature.x(),
            });
        }
        let (origin, ability) = Self::spell_ability(definition, option)?;
        let AbilityOrigin::Printed {
            ability: ability_id,
            ..
        } = origin
        else {
            unreachable!("a printed spell clause has a printed origin")
        };
        let DeclarativeAbilityDef::Spell(spell) = ability.definition else {
            unreachable!("spell_ability returns a spell clause")
        };
        let followup = Self::spell_custom_followup(definition, option, ability_id);
        let target_defs = Self::selected_spell_target_defs(spell, signature.modes())
            .expect("validated modes select declared spell targets");
        let mode_effects = Self::selected_spell_mode_effects(spell, signature.modes())
            .expect("validated modes select declared spell branches");
        Some(StackAbilityPayload {
            origin,
            definition: Some(Box::new(ability)),
            presentation_definition: definition_id,
            text: Some(ability.text),
            target_defs,
            targets: signature.targets().to_vec(),
            context: TriggerContext::empty(),
            condition: None,
            resolver: followup.map_or_else(
                || Self::ability_resolver(&ability),
                |behavior| StackAbilityResolver::DeclarativeWithCustomFollowup {
                    effect: ability.effect,
                    behavior,
                },
            ),
            mode_effects,
            x: signature.x(),
        })
    }

    fn uses_legacy_behavior_targets(definition: &CardDefinition, option: &PlayOptionDef) -> bool {
        matches!(
            (&definition.structure, &option.form),
            (
                crate::card::CardStructure::Single { main },
                crate::card::SpellForm::Part(part),
            ) if main == part
        ) && definition.play_options.len() == 1
            && option.id == PlayOptionId::DEFAULT
            && option.modes.is_none()
            && option.targets.is_empty()
            && Self::spell_ability(definition, option).is_none()
    }

    fn implemented_mode_selections(option: &PlayOptionDef) -> Vec<Vec<ModeId>> {
        let Some(mode_set) = &option.modes else {
            return vec![Vec::new()];
        };
        let implemented = mode_set
            .modes
            .iter()
            .filter(|mode| mode.effect_status == CardEffectStatus::Implemented)
            .map(|mode| mode.id)
            .collect::<Vec<_>>();
        let mut implemented = implemented;
        implemented.sort_unstable();
        mode_id_selections(
            &implemented,
            usize::from(mode_set.minimum),
            usize::from(mode_set.maximum),
            mode_set.may_repeat,
        )
    }

    fn target_slots_for(option: &PlayOptionDef, modes: &[ModeId]) -> Vec<TargetSlotDef> {
        let mut slots = option.targets.clone();
        if let Some(mode_set) = &option.modes {
            for mode in modes {
                if let Some(mode) = mode_set
                    .modes
                    .iter()
                    .find(|candidate| candidate.id == *mode)
                {
                    slots.extend(mode.targets.clone());
                }
            }
        }
        slots
    }

    fn visit_cost_configurations(
        &self,
        definition: &CardDefinition,
        card: GameObjectId,
        option: &PlayOptionDef,
        source_zone: CastSourceZone,
        mut visitor: impl FnMut(CostConfiguration) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        let mut selected_additional = Vec::with_capacity(option.additional_costs.len());
        if source_zone == CastSourceZone::Hand
            && Self::visit_additional_cost_configurations(
                option,
                None,
                option.additional_costs.len(),
                &mut selected_additional,
                &mut visitor,
            )
            .is_break()
        {
            return ControlFlow::Break(());
        }
        for cost in &option.alternative_costs {
            let kind = match Self::alternative_cast_clause(definition, option, cost.id) {
                Some((_, ability, kind)) if ability.implementation.is_executable() => Some(kind),
                Some(_) => continue,
                None => None,
            };
            let available = match (source_zone, kind) {
                (CastSourceZone::Hand, Some(AlternativeCastKindDef::Flashback))
                | (CastSourceZone::Graveyard, Some(AlternativeCastKindDef::Overload) | None) => {
                    false
                }
                (CastSourceZone::Hand, Some(AlternativeCastKindDef::Overload) | None)
                | (CastSourceZone::Graveyard, Some(AlternativeCastKindDef::Flashback)) => true,
            };
            if available
                && Self::visit_additional_cost_configurations(
                    option,
                    Some(cost.id),
                    option.additional_costs.len(),
                    &mut selected_additional,
                    &mut visitor,
                )
                .is_break()
            {
                return ControlFlow::Break(());
            }
        }
        if source_zone == CastSourceZone::Graveyard
            && self.granted_flashback(card, option).is_some()
            && let Some(granted) = Self::temporary_alternative_cost_id(option)
            && Self::visit_additional_cost_configurations(
                option,
                Some(granted),
                option.additional_costs.len(),
                &mut selected_additional,
                &mut visitor,
            )
            .is_break()
        {
            return ControlFlow::Break(());
        }

        ControlFlow::Continue(())
    }

    fn visit_additional_cost_configurations(
        option: &PlayOptionDef,
        alternative: Option<AlternativeCostId>,
        remaining: usize,
        selected_reversed: &mut Vec<AdditionalCostId>,
        visitor: &mut impl FnMut(CostConfiguration) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        let Some(index) = remaining.checked_sub(1) else {
            let additional = selected_reversed.iter().rev().copied().collect();
            return visitor(CostConfiguration::new(alternative, additional));
        };

        if Self::visit_additional_cost_configurations(
            option,
            alternative,
            index,
            selected_reversed,
            visitor,
        )
        .is_break()
        {
            return ControlFlow::Break(());
        }
        selected_reversed.push(option.additional_costs[index].id);
        let result = Self::visit_additional_cost_configurations(
            option,
            alternative,
            index,
            selected_reversed,
            visitor,
        );
        selected_reversed.pop();
        result
    }

    fn configured_cast_mana_cost(
        &self,
        card: GameObjectId,
        option: &PlayOptionDef,
        configuration: &CostConfiguration,
    ) -> Option<ManaCost> {
        let granted = Self::temporary_alternative_cost_id(option);
        let granted_flashback = (configuration.alternative().is_some()
            && configuration.alternative() == granted)
            .then(|| self.granted_flashback(card, option))
            .flatten();
        let mut cost = granted_flashback.map_or_else(
            || configured_mana_cost(option, configuration),
            |(_, mana_cost)| Some(mana_cost),
        )?;
        // `configured_mana_cost` already included additional costs for every
        // printed alternative and the normal cost. Runtime-granted
        // alternatives need them folded in here.
        if granted_flashback.is_some() {
            for selected in configuration.additional() {
                let additional = option
                    .additional_costs
                    .iter()
                    .find(|candidate| candidate.id == *selected)?;
                if let Some(mana) = additional.mana_cost {
                    cost = add_mana_cost(cost, mana);
                }
            }
        }
        Some(cost)
    }

    fn legacy_target_selections(
        &self,
        behavior: CardBehavior,
        x: u16,
        player: PlayerId,
    ) -> Vec<Vec<TargetSelection>> {
        self.legal_target_lists(behavior, x, player, None)
            .into_iter()
            .map(|targets| {
                if targets.is_empty() {
                    Vec::new()
                } else {
                    vec![TargetSelection::new(TargetSlotId(0), targets)]
                }
            })
            .collect()
    }

    fn legal_target_selections(&self, slots: &[TargetSlotDef]) -> Vec<Vec<TargetSelection>> {
        let mut selections = vec![Vec::new()];
        for slot in slots {
            let candidates = self.targets_matching(slot.predicate);
            let mut choices = Vec::new();
            for count in slot.minimum..=slot.maximum {
                choices.extend(
                    target_combinations(&candidates, usize::from(count))
                        .into_iter()
                        .map(|targets| TargetSelection::new(slot.id, targets)),
                );
            }
            let mut combined = Vec::new();
            for prefix in &selections {
                for choice in &choices {
                    let mut selected = prefix.clone();
                    selected.push(choice.clone());
                    combined.push(selected);
                }
            }
            selections = combined;
        }
        selections
    }

    fn legal_ability_target_selections(
        &self,
        slots: &[AbilityTargetDef],
        controller: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
    ) -> Vec<Vec<TargetSelection>> {
        let mut selections = vec![Vec::new()];
        for slot in slots {
            let candidates =
                self.ability_targets_matching(slot.predicate, controller, source, context);
            let mut choices = Vec::new();
            for count in slot.minimum..=slot.maximum {
                choices.extend(
                    target_combinations(&candidates, usize::from(count))
                        .into_iter()
                        .map(|targets| TargetSelection::new(slot.id, targets)),
                );
            }
            let mut combined = Vec::new();
            for prefix in &selections {
                for choice in &choices {
                    let mut selected = prefix.clone();
                    selected.push(choice.clone());
                    combined.push(selected);
                }
            }
            selections = combined;
        }
        selections
    }

    fn targets_matching(&self, predicate: TargetPredicate) -> Vec<Target> {
        match predicate {
            TargetPredicate::AnyTarget => self.damage_targets(),
            TargetPredicate::Player => {
                vec![Target::Player(PlayerId::One), Target::Player(PlayerId::Two)]
            }
            TargetPredicate::Permanent => self
                .battlefield
                .iter()
                .map(|permanent| Target::Permanent(permanent.card.id))
                .collect(),
            TargetPredicate::CreaturePermanent => self
                .battlefield
                .iter()
                .filter(|permanent| self.power(permanent).is_some())
                .map(|permanent| Target::Permanent(permanent.card.id))
                .collect(),
            TargetPredicate::Spell => self
                .stack
                .iter()
                .filter(|object| object.kind == StackObjectKind::Spell)
                .map(|object| Target::Spell(object.id))
                .collect(),
            TargetPredicate::NoncreatureSpell => self
                .stack
                .iter()
                .filter(|object| {
                    object.kind == StackObjectKind::Spell
                        && self
                            .stack_spell_types(object)
                            .is_some_and(|types| !types.is_creature())
                })
                .map(|object| Target::Spell(object.id))
                .collect(),
        }
    }

    fn stack_spell_types(&self, object: &StackObject) -> Option<CardTypeSet> {
        let definition = self.catalog.get(object.card.definition)?;
        let signature = object.signature.as_ref()?;
        let option = definition.play_option(signature.play_option())?;
        Self::play_option_types(definition, option)
    }

    fn stack_spell_enters_tapped(&self, object: &StackObject) -> bool {
        let Some(definition) = self.catalog.get(object.card.definition) else {
            return false;
        };
        let Some(signature) = &object.signature else {
            return false;
        };
        let crate::card::SpellForm::Part(part) = signature.form() else {
            return false;
        };
        definition.part(*part).is_some_and(|part| {
            part.rules.ability_clauses().iter().any(|ability| {
                ability.implementation.is_executable()
                    && matches!(ability.definition, DeclarativeAbilityDef::Replacement(_))
                    && ability.effect == EffectDef::EntersTapped
            })
        })
    }

    fn stack_trigger_event_object(&self, object: &StackObject) -> Option<TriggerEventObject> {
        let signature = object.signature.as_ref()?;
        self.printed_trigger_event_object(
            object.id,
            object.card.definition,
            object.controller,
            &CharacteristicContext::Stack {
                form: signature.form().clone(),
            },
        )
    }

    fn printed_trigger_event_object(
        &self,
        id: GameObjectId,
        definition: CardDefinitionId,
        controller: PlayerId,
        context: &CharacteristicContext,
    ) -> Option<TriggerEventObject> {
        let definition = self.catalog.get(definition)?;
        let parts = applicable_part_ids(definition, context).ok()?;
        let mut types = CardTypeSet::empty();
        let mut colors = [false; 5];
        let mut subtypes = Vec::new();
        let mut mana_value = 0;
        let mut power = None;
        let mut supertypes = [false; CardSupertype::COUNT];
        let mut keywords = 0;
        for part in parts {
            let part = definition.part(part)?;
            types = types.union(part.rules.types());
            for ability in part.rules.ability_clauses() {
                if ability.implementation.is_executable()
                    && let DeclarativeAbilityDef::Keyword(keyword) = ability.definition
                    && let Some(index) = keyword.simple_index()
                {
                    keywords |= 1 << index;
                }
            }
            for (combined, present) in colors.iter_mut().zip(part.rules.colors()) {
                *combined |= present;
            }
            for subtype in part.rules.subtypes() {
                if !subtypes.contains(subtype) {
                    subtypes.push(*subtype);
                }
            }
            mana_value += part.rules.mana_cost().map_or(0, ManaCost::mana_value);
            if let Some(stats) = part.rules.creature_stats() {
                power = Some(stats.power);
            }
            for supertype in CardSupertype::ALL {
                supertypes[supertype.index()] |= part.rules.has_supertype(supertype);
            }
        }
        Some(TriggerEventObject {
            id,
            types,
            controller,
            colors,
            subtypes: Cow::Owned(subtypes),
            // A card or a spell is nowhere near combat.
            attacking_or_blocking: false,
            keywords,
            mana_value,
            power,
            supertypes,
            attacking: false,
        })
    }

    /// Every legal target list, with hexproof and protection applied once at
    /// the end rather than in each of the several dozen per-card filters
    /// below. Doing it here is not just tidier: protection used to be spelled
    /// out arm by arm, and the arms that forgot -- Terror among them -- were
    /// simply wrong.
    fn legal_target_lists(
        &self,
        behavior: CardBehavior,
        x: u16,
        player: PlayerId,
        exact_count: Option<usize>,
    ) -> Vec<Vec<Target>> {
        self.printed_target_lists(behavior, x, player, exact_count)
            .into_iter()
            .filter(|choice| {
                choice.iter().all(|target| match target {
                    Target::Permanent(id) => self
                        .battlefield
                        .iter()
                        .find(|permanent| permanent.card.id == *id)
                        .is_none_or(|permanent| {
                            // Hexproof stops opponents only; you can always
                            // target your own. Protection stops everyone,
                            // including the permanent's own controller.
                            (permanent.controller == player || !self.has_hexproof(permanent))
                                && !self
                                    .is_protected_from_colors(permanent, behavior.rules().colors())
                        }),
                    Target::Player(_) | Target::Card(_) | Target::Spell(_) => true,
                })
            })
            .collect()
    }

    #[allow(clippy::too_many_lines)]
    fn printed_target_lists(
        &self,
        behavior: CardBehavior,
        x: u16,
        player: PlayerId,
        exact_count: Option<usize>,
    ) -> Vec<Vec<Target>> {
        match behavior {
            CardBehavior::Duress => vec![vec![Target::Player(player.opponent())]],
            CardBehavior::LightningBolt
            | CardBehavior::ChainLightning
            | CardBehavior::PillarOfFlame
            | CardBehavior::GoblinGrenade
            | CardBehavior::DrainLife
            | CardBehavior::WarleadersHelix => self
                .damage_targets()
                .into_iter()
                .map(|target| vec![target])
                .collect(),
            CardBehavior::Fireball => {
                let targets = self.damage_targets();
                let counts: Vec<_> =
                    exact_count.map_or_else(|| (1..=targets.len()).collect(), |count| vec![count]);
                counts
                    .into_iter()
                    .flat_map(|count| target_combinations(&targets, count))
                    .collect()
            }
            CardBehavior::DivineOffering => self
                .battlefield
                .iter()
                .filter(|permanent| self.is_artifact_permanent(permanent))
                .map(|permanent| vec![Target::Permanent(permanent.card.id)])
                .collect(),
            CardBehavior::CopyArtifact => self
                .battlefield
                .iter()
                .filter(|permanent| self.is_artifact_permanent(permanent))
                .map(|permanent| vec![Target::Permanent(permanent.card.id)])
                .collect(),
            CardBehavior::DustToDust => {
                let artifacts: Vec<_> = self
                    .battlefield
                    .iter()
                    .filter(|permanent| self.is_artifact_permanent(permanent))
                    .map(|permanent| Target::Permanent(permanent.card.id))
                    .collect();
                target_combinations(&artifacts, 2)
            }
            CardBehavior::SwordsToPlowshares => self
                .battlefield
                .iter()
                .filter(|permanent| self.power(permanent).is_some())
                .map(|permanent| vec![Target::Permanent(permanent.card.id)])
                .collect(),
            CardBehavior::Putrefy => self
                .battlefield
                .iter()
                .filter(|permanent| {
                    self.power(permanent).is_some() || self.is_artifact_permanent(permanent)
                })
                .map(|permanent| vec![Target::Permanent(permanent.card.id)])
                .collect(),
            CardBehavior::UltimatePrice => self
                .battlefield
                .iter()
                .filter(|permanent| {
                    self.power(permanent).is_some()
                        && self.effective_rules(permanent).is_some_and(|rules| {
                            rules.colors().iter().filter(|on| **on).count() == 1
                        })
                })
                .map(|permanent| vec![Target::Permanent(permanent.card.id)])
                .collect(),
            CardBehavior::DoomBlade => self
                .battlefield
                .iter()
                .filter(|permanent| {
                    self.power(permanent).is_some()
                        && !self
                            .effective_rules(permanent)
                            .is_some_and(|rules| rules.colors()[2])
                })
                .map(|permanent| vec![Target::Permanent(permanent.card.id)])
                .collect(),
            CardBehavior::Terror => self
                .battlefield
                .iter()
                .filter(|permanent| {
                    self.power(permanent).is_some()
                        && !self.is_artifact_permanent(permanent)
                        && !self
                            .effective_rules(permanent)
                            .is_some_and(|rules| rules.colors()[2])
                })
                .map(|permanent| vec![Target::Permanent(permanent.card.id)])
                .collect(),
            CardBehavior::GiantGrowth | CardBehavior::Berserk => self
                .battlefield
                .iter()
                .filter(|permanent| {
                    permanent.controller == player && self.power(permanent).is_some()
                })
                .map(|permanent| vec![Target::Permanent(permanent.card.id)])
                .collect(),
            CardBehavior::HurkylsRecall => vec![
                vec![Target::Player(PlayerId::One)],
                vec![Target::Player(PlayerId::Two)],
            ],
            CardBehavior::Detonate => self
                .battlefield
                .iter()
                .filter(|permanent| {
                    self.is_artifact_permanent(permanent)
                        && self.permanent_mana_value(permanent) == x
                })
                .map(|permanent| vec![Target::Permanent(permanent.card.id)])
                .collect(),
            CardBehavior::Fork => self
                .stack
                .iter()
                .filter(|object| {
                    object.kind == StackObjectKind::Spell
                        && self.stack_spell_types(object).is_some_and(|types| {
                            types.contains(CardType::Instant) || types.contains(CardType::Sorcery)
                        })
                })
                .map(|object| vec![Target::Spell(object.id)])
                .collect(),
            // Both read the spell's kind off its chosen play option, so a
            // split or modal card counts as whatever it was actually cast as.
            CardBehavior::Negate | CardBehavior::EssenceScatter | CardBehavior::Dispel => self
                .stack
                .iter()
                .filter(|object| {
                    object.kind == StackObjectKind::Spell
                        && self
                            .stack_spell_types(object)
                            .is_some_and(|types| match behavior {
                                CardBehavior::EssenceScatter => types.is_creature(),
                                CardBehavior::Dispel => types.contains(CardType::Instant),
                                _ => !types.is_creature(),
                            })
                })
                .map(|object| vec![Target::Spell(object.id)])
                .collect(),
            CardBehavior::ManaDrain | CardBehavior::Dissipate => self
                .stack
                .iter()
                .filter(|object| object.kind == StackObjectKind::Spell)
                .map(|object| vec![Target::Spell(object.id)])
                .collect(),
            _ => vec![Vec::new()],
        }
    }

    #[allow(clippy::too_many_lines)]
    fn add_ability_actions(&self, player: PlayerId, actions: &mut Vec<Action>) {
        for permanent in self
            .battlefield
            .iter()
            .filter(|permanent| permanent.controller == player)
        {
            let mut has_declarative_activation = false;
            let mut has_custom_activation = false;
            let (definition, part) = Self::effective_rules_source(permanent);
            let fallback_origin = AbilityOrigin::Printed {
                definition,
                part,
                ability: AbilityId::PRIMARY,
            };
            let mut fallback_target_slot = TargetSlotId(0);
            let mut activated = [None; 2];
            let mut activated_count = 0;
            let mut last_activated_origin = None;
            self.for_each_effective_ability(permanent, |effective| {
                let ability = effective.ability;
                let DeclarativeAbilityDef::Activated(definition) = ability.definition else {
                    return;
                };
                // Copy-process exceptions can retain an activated ability
                // whose structural origin is already present in the copied
                // values. Actions identify an ability by that origin, so a
                // consecutive repeat is externally indistinguishable and
                // would resolve through the first matching ability anyway.
                if last_activated_origin == Some(effective.origin) {
                    return;
                }
                last_activated_origin = Some(effective.origin);
                let target_slot = definition
                    .targets
                    .first()
                    .map_or(TargetSlotId(0), |target| target.id);
                if effective.origin == fallback_origin {
                    fallback_target_slot = target_slot;
                }
                if ability.implementation.is_executable() {
                    if let Some(slot) = activated.get_mut(activated_count) {
                        *slot = Some((effective.origin, target_slot));
                    }
                    activated_count += 1;
                }
                if ability.implementation.is_executable()
                    && ability.implementation != AbilityImplementationDef::Definition
                {
                    has_custom_activation = true;
                }
                if ability.implementation != AbilityImplementationDef::Definition
                    || !definition.source_zones.contains(&ZoneKind::Battlefield)
                {
                    return;
                }
                has_declarative_activation = true;
                let taps_source = definition.costs.contains(&AbilityCostDef::TapSource);
                // The same purpose the payment will use, so an ability that
                // taps its own source is never offered on mana only that
                // source could have made.
                let payment_purpose = ManaPaymentPurpose::Ability {
                    source: permanent.card.id,
                    taps_source,
                };
                if (taps_source && (permanent.tapped || !self.can_use_tap_ability(permanent)))
                    || definition.costs.iter().any(|cost| match cost {
                        AbilityCostDef::Mana(cost) => {
                            !self.can_pay_cost_for(player, *cost, 0, &payment_purpose)
                        }
                        AbilityCostDef::PayLife(amount) => {
                            self.players[player.index()].life
                                < i16::try_from(*amount).unwrap_or(i16::MAX)
                        }
                        AbilityCostDef::TapSource
                        | AbilityCostDef::SacrificeSource
                        | AbilityCostDef::SacrificePermanent { .. } => false,
                        AbilityCostDef::UntapSource
                        | AbilityCostDef::DiscardSource
                        | AbilityCostDef::DiscardCards(_)
                        | AbilityCostDef::ExileSource
                        | AbilityCostDef::Special(_) => true,
                    })
                {
                    return;
                }
                let mut sacrifice_costs = definition.costs.iter().filter_map(|cost| match cost {
                    AbilityCostDef::SacrificePermanent { object, controller } => {
                        Some((*object, *controller))
                    }
                    _ => None,
                });
                let sacrifice_cost = sacrifice_costs.next();
                if sacrifice_costs.next().is_some() {
                    return;
                }
                let sacrifice_choices = sacrifice_cost.map_or_else(
                    || vec![None],
                    |(predicate, relation)| {
                        self.battlefield
                            .iter()
                            .filter(|candidate| {
                                self.player_relation_matches(
                                    candidate.controller,
                                    relation,
                                    player,
                                    TriggerContext::empty(),
                                ) && self.trigger_object_matches(
                                    predicate,
                                    &self.trigger_event_object(candidate),
                                    permanent.card.id,
                                    false,
                                )
                            })
                            .map(|candidate| Some(candidate.card.id))
                            .collect()
                    },
                );
                if sacrifice_choices.is_empty() {
                    return;
                }
                // A variable cost offers one activation per affordable X.
                // Zero is always among them, and the affordability check above
                // already proved the rest of the cost is payable there.
                let max_x = definition
                    .costs
                    .iter()
                    .find_map(|cost| match cost {
                        AbilityCostDef::Mana(cost) if cost.variable_x => {
                            Some(self.maximum_x_for(player, *cost, &payment_purpose))
                        }
                        _ => None,
                    })
                    .unwrap_or(0);
                for selections in self.legal_ability_target_selections(
                    definition.targets,
                    player,
                    permanent.card.id,
                    TriggerContext::empty(),
                ) {
                    for sacrifice in &sacrifice_choices {
                        for x in 0..=max_x {
                            actions.push(Action::ActivateAbility {
                                source: permanent.card.id,
                                ability: effective.origin,
                                targets: selections.clone(),
                                sacrifice: *sacrifice,
                                x,
                            });
                        }
                    }
                }
            });
            if has_declarative_activation && !has_custom_activation {
                continue;
            }
            let (ability, ability_target_slot) =
                activated[0].unwrap_or((fallback_origin, fallback_target_slot));
            let (secondary_ability, secondary_target_slot) =
                activated[1].unwrap_or((fallback_origin, fallback_target_slot));
            match self.effective_behavior(permanent) {
                Some(CardBehavior::Atog) => {
                    actions.extend(
                        self.battlefield
                            .iter()
                            .filter(|candidate| {
                                candidate.controller == player
                                    && self.is_artifact_permanent(candidate)
                            })
                            .map(|candidate| Action::ActivateAbility {
                                source: permanent.card.id,
                                ability,
                                targets: Vec::new(),
                                sacrifice: Some(candidate.card.id),
                                x: 0,
                            }),
                    );
                }
                Some(CardBehavior::GlassesOfUrza) if !permanent.tapped => {
                    for target in [PlayerId::One, PlayerId::Two] {
                        actions.push(Action::ActivateAbility {
                            source: permanent.card.id,
                            ability,
                            targets: vec![TargetSelection::single(
                                ability_target_slot,
                                Target::Player(target),
                            )],
                            sacrifice: None,
                            x: 0,
                        });
                    }
                }
                Some(CardBehavior::IcyManipulator)
                    if !permanent.tapped
                        && self.can_use_tap_ability(permanent)
                        && self.can_pay_cost(player, ManaCost::new(1, 0), 0) =>
                {
                    actions.extend(self.battlefield.iter().map(|candidate| {
                        Action::ActivateAbility {
                            source: permanent.card.id,
                            ability,
                            targets: vec![TargetSelection::single(
                                ability_target_slot,
                                Target::Permanent(candidate.card.id),
                            )],
                            sacrifice: None,
                            x: 0,
                        }
                    }));
                }
                Some(CardBehavior::Pendelhaven)
                    if !permanent.tapped && self.can_use_tap_ability(permanent) =>
                {
                    actions.extend(
                        self.battlefield
                            .iter()
                            .filter(|candidate| {
                                self.power(candidate) == Some(1)
                                    && self.toughness(candidate) == Some(1)
                            })
                            .map(|candidate| Action::ActivateAbility {
                                source: permanent.card.id,
                                ability,
                                targets: vec![TargetSelection::single(
                                    ability_target_slot,
                                    Target::Permanent(candidate.card.id),
                                )],
                                sacrifice: None,
                                x: 0,
                            }),
                    );
                }
                Some(CardBehavior::SedgeTroll)
                    if self.can_pay_cost(player, ManaCost::colored(0, 0, 0, 1, 0, 0), 0) =>
                {
                    actions.push(Action::ActivateAbility {
                        source: permanent.card.id,
                        ability,
                        targets: Vec::new(),
                        sacrifice: None,
                        x: 0,
                    });
                }
                Some(CardBehavior::StoneGiant)
                    if !permanent.tapped && self.can_use_tap_ability(permanent) =>
                {
                    let power = self.power(permanent).unwrap_or(0);
                    actions.extend(
                        self.battlefield
                            .iter()
                            .filter(|candidate| {
                                candidate.controller == player
                                    && self.toughness(candidate).is_some_and(|value| value < power)
                            })
                            .map(|candidate| Action::ActivateAbility {
                                source: permanent.card.id,
                                ability,
                                targets: vec![TargetSelection::single(
                                    ability_target_slot,
                                    Target::Permanent(candidate.card.id),
                                )],
                                sacrifice: None,
                                x: 0,
                            }),
                    );
                }
                Some(CardBehavior::DragonWhelp)
                    if self.can_pay_cost(player, ManaCost::new(0, 1), 0) =>
                {
                    actions.push(Action::ActivateAbility {
                        source: permanent.card.id,
                        ability,
                        targets: Vec::new(),
                        sacrifice: None,
                        x: 0,
                    });
                }
                Some(CardBehavior::MishrasFactory)
                    if self.can_pay_cost(player, ManaCost::new(1, 0), 0) =>
                {
                    actions.push(Action::ActivateAbility {
                        source: permanent.card.id,
                        ability,
                        targets: Vec::new(),
                        sacrifice: None,
                        x: 0,
                    });
                    if !permanent.tapped && self.can_use_tap_ability(permanent) {
                        actions.extend(
                            self.battlefield
                                .iter()
                                .filter(|candidate| {
                                    candidate.controller == player && candidate.factory_animated
                                })
                                .map(|candidate| Action::ActivateAbility {
                                    source: permanent.card.id,
                                    ability: secondary_ability,
                                    targets: vec![TargetSelection::single(
                                        secondary_target_slot,
                                        Target::Permanent(candidate.card.id),
                                    )],
                                    sacrifice: None,
                                    x: 0,
                                }),
                        );
                    }
                }
                Some(CardBehavior::MishrasFactory)
                    if !permanent.tapped && self.can_use_tap_ability(permanent) =>
                {
                    actions.extend(
                        self.battlefield
                            .iter()
                            .filter(|candidate| {
                                candidate.controller == player && candidate.factory_animated
                            })
                            .map(|candidate| Action::ActivateAbility {
                                source: permanent.card.id,
                                ability: secondary_ability,
                                targets: vec![TargetSelection::single(
                                    secondary_target_slot,
                                    Target::Permanent(candidate.card.id),
                                )],
                                sacrifice: None,
                                x: 0,
                            }),
                    );
                }
                Some(CardBehavior::ChaosOrb)
                    if !permanent.tapped && self.can_pay_cost(player, ManaCost::new(1, 0), 0) =>
                {
                    actions.extend(
                        self.battlefield
                            .iter()
                            .filter(|candidate| candidate.card.id != permanent.card.id)
                            .map(|candidate| Action::ActivateAbility {
                                source: permanent.card.id,
                                ability,
                                targets: vec![TargetSelection::single(
                                    ability_target_slot,
                                    Target::Permanent(candidate.card.id),
                                )],
                                sacrifice: None,
                                x: 0,
                            }),
                    );
                }
                Some(CardBehavior::OrcishMechanics)
                    if !permanent.tapped && self.can_use_tap_ability(permanent) =>
                {
                    for sacrificed in self.battlefield.iter().filter(|candidate| {
                        candidate.controller == player
                            && candidate.card.id != permanent.card.id
                            && self.is_artifact_permanent(candidate)
                    }) {
                        actions.extend(self.damage_targets().into_iter().map(|target| {
                            Action::ActivateAbility {
                                source: permanent.card.id,
                                ability,
                                targets: vec![TargetSelection::single(ability_target_slot, target)],
                                sacrifice: Some(sacrificed.card.id),
                                x: 0,
                            }
                        }));
                    }
                }
                Some(CardBehavior::Triskelion)
                    if permanent.counters(CounterKind::PlusOnePlusOne) > 0 =>
                {
                    actions.extend(self.damage_targets().into_iter().map(|target| {
                        Action::ActivateAbility {
                            source: permanent.card.id,
                            ability,
                            targets: vec![TargetSelection::single(ability_target_slot, target)],
                            sacrifice: None,
                            x: 0,
                        }
                    }));
                }
                Some(CardBehavior::LibraryOfAlexandria)
                    if !permanent.tapped
                        && self.can_use_tap_ability(permanent)
                        && self.players[player.index()].hand.len() == 7 =>
                {
                    actions.push(Action::ActivateAbility {
                        source: permanent.card.id,
                        ability,
                        targets: Vec::new(),
                        sacrifice: None,
                        x: 0,
                    });
                }
                Some(CardBehavior::MazeOfIth)
                    if !permanent.tapped && self.can_use_tap_ability(permanent) =>
                {
                    actions.extend(
                        self.battlefield
                            .iter()
                            .filter(|candidate| candidate.attacking)
                            .map(|candidate| Action::ActivateAbility {
                                source: permanent.card.id,
                                ability,
                                targets: vec![TargetSelection::single(
                                    ability_target_slot,
                                    Target::Permanent(candidate.card.id),
                                )],
                                sacrifice: None,
                                x: 0,
                            }),
                    );
                }
                Some(CardBehavior::NevinyrralsDisk)
                    if !permanent.tapped
                        && self.can_use_tap_ability(permanent)
                        && self.can_pay_cost(player, ManaCost::new(1, 0), 0) =>
                {
                    actions.push(Action::ActivateAbility {
                        source: permanent.card.id,
                        ability,
                        targets: Vec::new(),
                        sacrifice: None,
                        x: 0,
                    });
                }
                Some(CardBehavior::IcatianJavelineers)
                    if !permanent.tapped
                        && self.can_use_tap_ability(permanent)
                        && permanent.counters(CounterKind::Javelin) > 0 =>
                {
                    actions.extend(self.damage_targets().into_iter().map(|target| {
                        Action::ActivateAbility {
                            source: permanent.card.id,
                            ability,
                            targets: vec![TargetSelection::single(ability_target_slot, target)],
                            sacrifice: None,
                            x: 0,
                        }
                    }));
                }
                Some(CardBehavior::TimeVault)
                    if !permanent.tapped && self.can_use_tap_ability(permanent) =>
                {
                    actions.push(Action::ActivateAbility {
                        source: permanent.card.id,
                        ability,
                        targets: Vec::new(),
                        sacrifice: None,
                        x: 0,
                    });
                }
                _ => {}
            }
        }
        self.add_hand_ability_actions(player, actions);
    }

    fn visit_printed_card_abilities(
        &self,
        card: &CardInstance,
        context: &CharacteristicContext,
        mut visitor: impl FnMut(EffectiveAbility) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        let Some(definition) = self.catalog.get(card.definition) else {
            return ControlFlow::Continue(());
        };
        let Ok(parts) = applicable_part_ids(definition, context) else {
            return ControlFlow::Continue(());
        };
        for part in parts {
            let Some(part_definition) = definition.part(part) else {
                continue;
            };
            for attached in part_definition.rules.indexed_abilities() {
                if visitor(EffectiveAbility {
                    origin: AbilityOrigin::Printed {
                        definition: definition.id,
                        part,
                        ability: attached.id,
                    },
                    ability: attached.definition,
                })
                .is_break()
                {
                    return ControlFlow::Break(());
                }
            }
        }
        ControlFlow::Continue(())
    }

    fn for_each_printed_card_ability(
        &self,
        card: &CardInstance,
        context: &CharacteristicContext,
        mut visitor: impl FnMut(EffectiveAbility),
    ) {
        let result = self.visit_printed_card_abilities(card, context, |effective| {
            visitor(effective);
            ControlFlow::Continue(())
        });
        debug_assert!(result.is_continue());
    }

    fn find_printed_card_ability(
        &self,
        card: &CardInstance,
        context: &CharacteristicContext,
        mut predicate: impl FnMut(EffectiveAbility) -> bool,
    ) -> Option<EffectiveAbility> {
        let mut found = None;
        let _ = self.visit_printed_card_abilities(card, context, |effective| {
            if predicate(effective) {
                found = Some(effective);
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        });
        found
    }

    fn add_hand_ability_actions(&self, player: PlayerId, actions: &mut Vec<Action>) {
        for card in &self.players[player.index()].hand {
            self.for_each_printed_card_ability(card, &CharacteristicContext::Hand, |effective| {
                let ability = effective.ability;
                let DeclarativeAbilityDef::Activated(definition) = ability.definition else {
                    return;
                };
                if ability.implementation != AbilityImplementationDef::Definition
                    || !definition.source_zones.contains(&ZoneKind::Hand)
                {
                    return;
                }
                let mut mana_cost = ManaCost::default();
                let mut supported = true;
                for cost in definition.costs.as_slice() {
                    match cost {
                        AbilityCostDef::Mana(cost) => {
                            mana_cost = add_mana_cost(mana_cost, *cost);
                        }
                        AbilityCostDef::DiscardSource => {}
                        AbilityCostDef::TapSource
                        | AbilityCostDef::UntapSource
                        | AbilityCostDef::SacrificeSource
                        | AbilityCostDef::PayLife(_)
                        | AbilityCostDef::DiscardCards(_)
                        | AbilityCostDef::SacrificePermanent { .. }
                        | AbilityCostDef::ExileSource
                        | AbilityCostDef::Special(_) => supported = false,
                    }
                }
                let payment_purpose = ManaPaymentPurpose::Ability {
                    source: card.id,
                    taps_source: false,
                };
                if !supported || !self.can_pay_cost_for(player, mana_cost, 0, &payment_purpose) {
                    return;
                }
                let max_x = if mana_cost.variable_x {
                    self.maximum_x_for(player, mana_cost, &payment_purpose)
                } else {
                    0
                };
                for targets in self.legal_ability_target_selections(
                    definition.targets,
                    player,
                    card.id,
                    TriggerContext::empty(),
                ) {
                    for x in 0..=max_x {
                        actions.push(Action::ActivateAbility {
                            source: card.id,
                            ability: effective.origin,
                            targets: targets.clone(),
                            sacrifice: None,
                            x,
                        });
                    }
                }
            });
        }
    }

    fn behavior(&self, definition: CardDefinitionId) -> Option<CardBehavior> {
        self.catalog
            .get(definition)
            .and_then(|card| card.rules.special_behavior())
    }

    fn permanent_mana_value(&self, permanent: &Permanent) -> u16 {
        // A transforming double-faced permanent keeps the mana value of its
        // front face while its back face is up. A permanent merely copying a
        // back face is not itself that transforming double-faced card, so its
        // copied characteristics continue through the ordinary path below.
        if permanent.copied_from.is_none()
            && let Some(definition) = self.catalog.get(permanent.card.definition)
            && let CardStructure::DoubleFaced {
                front,
                kind: DoubleFacedKind::Transforming,
                ..
            } = &definition.structure
        {
            return definition
                .part(*front)
                .map_or(0, |part| part.rules.printed_mana_cost().mana_value());
        }
        self.effective_rules(permanent)
            .map_or(0, |rules| rules.printed_mana_cost().mana_value())
    }

    fn stack_spell_mana_value(&self, object: &StackObject) -> u16 {
        let Some(definition) = self.catalog.get(object.card.definition) else {
            return 0;
        };
        let Some(signature) = &object.signature else {
            return 0;
        };
        match signature.form() {
            crate::card::SpellForm::Part(part) => definition
                .part(*part)
                .and_then(CardPart::mana_cost)
                .map_or(0, mana_cost_value),
            crate::card::SpellForm::Combined(parts) => parts
                .iter()
                .filter_map(|part| definition.part(*part).and_then(CardPart::mana_cost))
                .map(mana_cost_value)
                .fold(0, u16::saturating_add),
        }
    }

    fn play_land(&mut self, player: PlayerId, card_id: GameObjectId, option_id: PlayOptionId) {
        let definition_id = self.players[player.index()]
            .hand
            .iter()
            .find(|card| card.id == card_id)
            .map(|card| card.definition)
            .expect("legal land action references a card in hand");
        let definition = self
            .catalog
            .get(definition_id)
            .expect("legal land action references a cataloged card");
        let option = definition
            .play_option(option_id)
            .filter(|option| option.action == PlayActionKind::PlayLand)
            .expect("legal land action references a land play option");
        let presented = match &option.form {
            crate::card::SpellForm::Part(part) => *part,
            crate::card::SpellForm::Combined(_) => {
                unreachable!("a land play option presents exactly one card part")
            }
        };
        let land_rules = definition
            .part(presented)
            .filter(|part| part.rules.has_type(CardType::Land))
            .map(|part| part.rules)
            .expect("land play option references a land part");
        let entry = land_rules.land_entry_procedure();
        let card = remove_card(&mut self.players[player.index()].hand, card_id)
            .expect("legal land action references a card in hand");
        let tapped = match entry {
            LandEntry::Untapped => false,
            // A shock land arrives tapped and the decision below untaps it if
            // the player pays. Printed, the payment is a replacement and the
            // land never enters tapped at all, but nobody gets priority in
            // between and nothing here triggers on entering tapped, so the
            // only difference is which way round the two events read.
            LandEntry::Tapped | LandEntry::PayLifeOrTapped(_) => true,
            LandEntry::TappedUnlessControlsLandType(types) => {
                !self.controls_any_land_type(player, types)
            }
        };
        let (card, _zone_change) = self.zone_change_card(card);
        let permanent_id = card.id;
        self.players[player.index()].land_played_this_turn = true;
        let permanent = Permanent {
            card,
            presented,
            controller: player,
            tapped,
            entered_controller_turn: self.turns_started[player.index()],
            damage: 0,
            loyalty: None,
            power_bonus: 0,
            toughness_bonus: 0,
            attacking: false,
            unblockable_this_turn: false,
            blocked: false,
            blocking: None,
            chosen_player: None,
            chosen_creature_type: None,
            destroy_at_end: false,
            temporary_keywords: Vec::new(),
            factory_animated: false,
            dragon_whelp_activations: 0,
            counters: [0; CounterKind::COUNT],
            attached_to: None,
            exile_instead_of_dying: false,
            combat_damage_assignment: Vec::new(),
            copy_effect: None,
            copied_from: None,
            text_changes: Vec::new(),
            regeneration_shields: 0,
            berserked: false,
            attacked_this_turn: false,
            forestwalk_until_upkeep_of: None,
            damage_sources: Vec::new(),
            deathtouch_damage: false,
        };
        self.consecutive_passes = 0;
        self.events.push(GameEvent::LandPlayed {
            player,
            card: permanent_id,
            definition: definition_id,
        });

        // "As this enters" choices are replacement effects: the choice is
        // part of the entry procedure, so neither the permanent nor an ETB
        // snapshot exists until the player has made it.
        if self.permanent_chooses_creature_type(&permanent) {
            self.queue_creature_type_choice(player, permanent, entry);
        } else {
            self.finish_land_entry(player, permanent, entry);
        }
    }

    fn finish_land_entry(&mut self, player: PlayerId, permanent: Permanent, entry: LandEntry) {
        let permanent_id = permanent.card.id;
        self.battlefield.push(permanent);
        let entered = self
            .battlefield
            .last()
            .expect("the played land is on the battlefield");
        let entered_event = self.trigger_event_object(entered);
        self.capture_battlefield_triggers(&CommittedTriggerEvent::ZoneChanged {
            object: entered_event,
            from: ZoneKind::Hand,
            to: ZoneKind::Battlefield,
        });
        // A second legendary land can arrive this way without the stack ever
        // being involved, so the legend rule has to run here too.
        self.apply_legend_rule();
        if let LandEntry::PayLifeOrTapped(life) = entry
            && self
                .battlefield
                .iter()
                .any(|candidate| candidate.card.id == permanent_id)
        {
            self.queue_shock_land_decision(player, permanent_id, life);
        }
    }

    fn permanent_chooses_creature_type(&self, permanent: &Permanent) -> bool {
        self.find_effective_ability(permanent, |effective| {
            effective.ability.implementation.is_executable()
                && matches!(
                    effective.ability.definition,
                    DeclarativeAbilityDef::Replacement(definition)
                        if definition.event == ReplacementEventDef::EntersBattlefield
                )
                && effective.ability.effect
                    == EffectDef::ChooseCreatureType {
                        object: EffectRecipientDef::Source,
                    }
        })
        .is_some()
    }

    fn creature_type_choices(&self, player: PlayerId) -> Vec<String> {
        let mut counts = CREATURE_TYPES
            .iter()
            .map(|creature_type| ((*creature_type).into(), 0))
            .collect::<BTreeMap<String, usize>>();
        for card in &self.players[player.index()].hand {
            let Some(definition) = self.catalog.get(card.definition) else {
                continue;
            };
            for part in &definition.parts {
                if part.rules.has_type(CardType::Creature) {
                    for subtype in part.rules.subtypes() {
                        if let Some(count) = counts.get_mut(*subtype) {
                            *count += 1;
                        }
                    }
                }
            }
        }
        let mut choices = counts.into_iter().collect::<Vec<_>>();
        choices.sort_by(|(left_name, left_count), (right_name, right_count)| {
            right_count
                .cmp(left_count)
                .then_with(|| left_name.cmp(right_name))
        });
        choices.into_iter().map(|(name, _)| name).collect()
    }

    fn queue_creature_type_choice(
        &mut self,
        player: PlayerId,
        permanent: Permanent,
        entry: LandEntry,
    ) {
        let mut choices = self.creature_type_choices(player);
        // The complete game vocabulary always contains creature types. Keep a
        // legal fallback so a deliberately tiny test catalog cannot strand
        // the land in an unfinished entry procedure.
        if choices.is_empty() {
            choices.push("Human".into());
        }
        let options = choices
            .iter()
            .enumerate()
            .map(|(index, creature_type)| DecisionOption {
                id: u32::try_from(index).unwrap_or(u32::MAX),
                label: creature_type.clone(),
                card: None,
                ability_text: None,
                zone: DecisionZone::None,
            })
            .collect();
        self.queue_decision(
            player,
            "Choose a creature type",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::ChooseCreatureType {
                player,
                permanent: Box::new(permanent),
                entry,
                choices,
            },
        );
    }

    /// Shock lands: pay the life to have it enter untapped, or leave it tapped.
    fn queue_shock_land_decision(&mut self, player: PlayerId, permanent: GameObjectId, life: u8) {
        let name = self
            .battlefield
            .iter()
            .find(|candidate| candidate.card.id == permanent)
            .and_then(|candidate| self.catalog.get(candidate.card.definition))
            .map_or_else(|| "the land".to_string(), |card| card.name.clone());
        // You may pay life down to zero, but not more life than you have.
        if self.players[player.index()].life < i16::from(life) {
            return;
        }
        let options = vec![
            DecisionOption {
                id: 0,
                label: format!("Leave {name} tapped"),
                card: None,
                ability_text: None,
                zone: DecisionZone::None,
            },
            DecisionOption {
                id: 1,
                label: format!("Pay {life} life for {name} to enter untapped"),
                card: None,
                ability_text: None,
                zone: DecisionZone::None,
            },
        ];
        self.queue_decision(
            player,
            format!("Pay {life} life for {name}?"),
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::ShockLand {
                player,
                permanent,
                life,
            },
        );
    }

    fn activate_mana_source(
        &mut self,
        player: PlayerId,
        source: GameObjectId,
        ability: AbilityOrigin,
        color: ManaColor,
    ) {
        let activation = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
            .and_then(|permanent| self.mana_ability_activation(permanent, ability, color))
            .expect("legal mana action references a mana source");
        let produced_mana = Self::mana_for_activation(activation);
        for cost in activation.costs.as_slice() {
            match cost {
                AbilityCostDef::TapSource => {
                    // Captured before the tap so the land's own characteristics
                    // are the ones a watcher sees, and only here: a mana
                    // ability with no tap cost never taps anything for mana.
                    let tapped_for_mana = self
                        .battlefield
                        .iter()
                        .find(|permanent| permanent.card.id == source)
                        .map(|permanent| CommittedTriggerEvent::TappedForMana {
                            object: self.trigger_event_object(permanent),
                        });
                    let _ = self.tap_permanent(source);
                    if let Some(event) = tapped_for_mana {
                        self.capture_battlefield_triggers(&event);
                    }
                }
                AbilityCostDef::SacrificeSource => self.sacrifice_permanent(source),
                AbilityCostDef::PayLife(amount) => {
                    self.players[player.index()].life -= i16::try_from(*amount).unwrap_or(i16::MAX);
                }
                AbilityCostDef::Mana(_)
                | AbilityCostDef::DiscardSource
                | AbilityCostDef::UntapSource
                | AbilityCostDef::DiscardCards(_)
                | AbilityCostDef::SacrificePermanent { .. }
                | AbilityCostDef::ExileSource
                | AbilityCostDef::Special(_) => {
                    unreachable!("unsupported mana-ability costs are not enumerated")
                }
            }
        }
        self.add_mana(player, produced_mana);
        self.consecutive_passes = 0;
        self.events.push(GameEvent::ManaAdded { player, source });
    }

    /// Whether the chosen modes suit the play option: the right number, in
    /// ascending order, without repeats unless the card allows them, and all
    /// of them actually executable.
    fn mode_selection_is_valid(option: &PlayOptionDef, choices: &CastChoices) -> bool {
        match &option.modes {
            None => choices.modes().is_empty(),
            Some(mode_set) => {
                let count = choices.modes().len();
                if count < usize::from(mode_set.minimum) || count > usize::from(mode_set.maximum) {
                    return false;
                }
                if !mode_set.may_repeat {
                    let unique = choices
                        .modes()
                        .iter()
                        .copied()
                        .collect::<std::collections::HashSet<_>>();
                    if unique.len() != count {
                        return false;
                    }
                }
                if choices.modes().windows(2).any(|pair| pair[0] > pair[1]) {
                    return false;
                }
                choices.modes().iter().all(|selected| {
                    mode_set.modes.iter().any(|mode| {
                        mode.id == *selected && mode.effect_status == CardEffectStatus::Implemented
                    })
                })
            }
        }
    }

    /// Whether the chosen targets fill a declarative spell clause's slots and
    /// every one of them is legal right now.
    fn spell_target_selection_is_valid(
        &self,
        target_defs: &[AbilityTargetDef],
        choices: &CastChoices,
        player: PlayerId,
        card_id: GameObjectId,
    ) -> bool {
        target_defs
            .iter()
            .zip(choices.targets())
            .all(|(slot, selection)| {
                let count = selection.targets().len();
                let legal = self.ability_targets_matching(
                    slot.predicate,
                    player,
                    card_id,
                    TriggerContext::empty(),
                );
                slot.id == selection.slot()
                    && count >= usize::from(slot.minimum)
                    && count <= usize::from(slot.maximum)
                    && selection
                        .targets()
                        .iter()
                        .all(|target| legal.contains(target))
            })
    }

    /// Whether the chosen targets fill the play option's own declared slots,
    /// used by cards whose targeting comes from the option rather than from a
    /// declarative spell clause.
    fn declared_slot_selection_is_valid(
        &self,
        declared_slots: &[TargetSlotDef],
        choices: &CastChoices,
    ) -> bool {
        if declared_slots.len() != choices.targets().len() {
            return false;
        }
        declared_slots
            .iter()
            .zip(choices.targets())
            .all(|(slot, selection)| {
                let count = selection.targets().len();
                slot.id == selection.slot()
                    && count >= usize::from(slot.minimum)
                    && count <= usize::from(slot.maximum)
                    && selection
                        .targets()
                        .iter()
                        .all(|target| self.target_matches(slot.predicate, *target))
            })
    }

    #[allow(clippy::too_many_lines)]
    fn validated_cast_signature(
        &self,
        player: PlayerId,
        card_id: GameObjectId,
        choices: &CastChoices,
    ) -> Option<(CastSignature, ManaCost, CardBehavior, CastSourceZone)> {
        let state = &self.players[player.index()];
        let (card, source_zone) = state
            .hand
            .iter()
            .find(|card| card.id == card_id)
            .map(|card| (card, CastSourceZone::Hand))
            .or_else(|| {
                state
                    .graveyard
                    .iter()
                    .find(|card| card.id == card_id)
                    .map(|card| (card, CastSourceZone::Graveyard))
            })?;
        let definition = self.catalog.get(card.definition)?;
        let option = definition
            .play_option(choices.play_option())
            .filter(|option| option.action == PlayActionKind::CastSpell)?;
        if source_zone == CastSourceZone::Graveyard
            && option.restriction == PlayRestriction::FromHandOnly
        {
            return None;
        }
        let behavior =
            Self::play_option_behavior(definition, option).unwrap_or(CardBehavior::Unsupported);
        let types = Self::play_option_types(definition, option)?;
        if option.effect_status == CardEffectStatus::MetadataOnly && !types.is_creature() {
            return None;
        }

        if !Self::mode_selection_is_valid(option, choices) {
            return None;
        }

        if !self
            .visit_cost_configurations(definition, card_id, option, source_zone, |costs| {
                if &costs == choices.costs() {
                    ControlFlow::Break(())
                } else {
                    ControlFlow::Continue(())
                }
            })
            .is_break()
        {
            return None;
        }
        let alternative_kind =
            self.selected_alternative_kind(definition, option, card_id, choices.costs());
        if alternative_kind == Some(AlternativeCastKindDef::Overload) && !choices.modes().is_empty()
        {
            return None;
        }
        let mut cost = self.configured_cast_mana_cost(card_id, option, choices.costs())?;
        if !cost.variable_x && choices.x() != 0 {
            return None;
        }

        let declared_slots = Self::target_slots_for(option, choices.modes());
        if alternative_kind == Some(AlternativeCastKindDef::Overload) {
            if !choices.targets().is_empty() {
                return None;
            }
        } else if let Some((_, ability)) = Self::spell_ability(definition, option) {
            let DeclarativeAbilityDef::Spell(spell) = ability.definition else {
                unreachable!("spell_ability returns a spell clause")
            };
            let target_defs = Self::selected_spell_target_defs(spell, choices.modes())?;
            if target_defs.len() != choices.targets().len() {
                return None;
            }
            if !self.spell_target_selection_is_valid(&target_defs, choices, player, card_id) {
                return None;
            }
        } else if Self::uses_legacy_behavior_targets(definition, option) {
            let flat_targets = choices.iter_targets().copied().collect::<Vec<_>>();
            let has_legacy_shape = if flat_targets.is_empty() {
                choices.targets().is_empty()
            } else {
                matches!(choices.targets(), [selection]
                    if selection.slot() == TargetSlotId(0)
                        && selection.targets() == flat_targets)
            };
            if !has_legacy_shape
                || !self
                    .legal_target_lists(behavior, choices.x(), player, None)
                    .contains(&flat_targets)
            {
                return None;
            }
            cost = add_generic(cost, fireball_extra_cost(behavior, flat_targets.len()));
        } else if !self.declared_slot_selection_is_valid(&declared_slots, choices) {
            return None;
        }
        cost = reduce_generic(cost, self.spell_cost_reduction(definition.id, player));
        let payment_purpose = ManaPaymentPurpose::Spell {
            object: card_id,
            definition: definition.id,
            controller: player,
            form: option.form.clone(),
        };
        if cost.variable_x && choices.x() > self.maximum_x_for(player, cost, &payment_purpose) {
            return None;
        }
        if !self.can_pay_cost_for(player, cost, choices.x(), &payment_purpose) {
            return None;
        }

        Some((
            CastSignature::from_validated_choices(option.form.clone(), choices.clone()),
            cost,
            behavior,
            source_zone,
        ))
    }

    fn target_matches(&self, predicate: TargetPredicate, target: Target) -> bool {
        self.targets_matching(predicate).contains(&target)
    }

    fn cast_spell(
        &mut self,
        player: PlayerId,
        card_id: GameObjectId,
        choices: CastChoices,
        sacrifices: &[GameObjectId],
    ) {
        let (_, _, behavior, _) = self
            .validated_cast_signature(player, card_id, &choices)
            .expect("legal cast action has valid structured choices");
        if behavior == CardBehavior::Recall && choices.x() > 0 {
            let eligible = self.players[player.index()]
                .hand
                .iter()
                .filter(|card| card.id != card_id)
                .cloned()
                .collect::<Vec<_>>();
            let options = self.card_decision_options(&eligible, DecisionZone::Hand);
            self.queue_decision(
                player,
                format!("Discard {} card(s) to cast Recall", choices.x()),
                DecisionVisibility::Private,
                DecisionPreference::LowerCardValue,
                usize::from(choices.x())..=usize::from(choices.x()),
                true,
                options,
                DecisionContinuation::RecallCost {
                    player,
                    card: card_id,
                    choices,
                },
            );
            return;
        }
        self.finish_cast_spell(player, card_id, &choices, sacrifices);
    }

    fn finish_cast_spell(
        &mut self,
        player: PlayerId,
        card_id: GameObjectId,
        choices: &CastChoices,
        sacrifices: &[GameObjectId],
    ) {
        let (signature, cost, _behavior, source_zone) = self
            .validated_cast_signature(player, card_id, choices)
            .expect("validated casting choices remain valid while paying costs");
        let targets = signature.iter_targets().copied().collect::<Vec<_>>();
        let x = signature.x();
        let cast_via_flashback = self.players[player.index()]
            .hand
            .iter()
            .chain(&self.players[player.index()].graveyard)
            .find(|card| card.id == card_id)
            .and_then(|card| self.catalog.get(card.definition))
            .and_then(|definition| {
                definition
                    .play_option(signature.play_option())
                    .map(|option| (definition, option))
            })
            .and_then(|(definition, option)| {
                self.selected_alternative_kind(definition, option, card_id, signature.costs())
            })
            == Some(AlternativeCastKindDef::Flashback);
        let card = match source_zone {
            CastSourceZone::Hand => remove_card(&mut self.players[player.index()].hand, card_id),
            CastSourceZone::Graveyard => {
                remove_card(&mut self.players[player.index()].graveyard, card_id)
            }
        }
        .expect("legal cast action references a card in its validated source zone");
        // Every outstanding grant applies to the same next sorcery, whatever
        // its timing, so consume them together based on the form actually cast.
        let cast_is_sorcery = self
            .catalog
            .get(card.definition)
            .and_then(|definition| {
                let option = definition.play_option(signature.play_option())?;
                Self::play_option_types(definition, option)
            })
            .is_some_and(|types| types.contains(CardType::Sorcery));
        if cast_is_sorcery {
            self.sorcery_flash_grants[player.index()] = 0;
        }
        // A spell is first proposed on the stack, then mana abilities may be
        // activated and costs are paid. The operation cannot fail after the
        // validated signature above, so keeping the provisional object local
        // gives mana spend riders a concrete destination without exposing a
        // half-paid spell to priority or trigger placement.
        let (card, _zone_change) = self.zone_change_card(card);
        let stack_id = card.id;
        let definition = card.definition;
        let frozen_spell_ability = self.frozen_spell_payload(definition, &signature);
        let mut stack_object = StackObject {
            id: stack_id,
            kind: StackObjectKind::Spell,
            card,
            source: None,
            ability: frozen_spell_ability,
            controller: player,
            signature: Some(signature),
            chosen_permanents: Vec::new(),
            applied_effects: Vec::new(),
            text_changes: Vec::new(),
            cast_via_flashback,
            is_copy: false,
        };
        let payment_purpose = ManaPaymentPurpose::Spell {
            object: stack_id,
            definition,
            controller: player,
            form: stack_object
                .signature
                .as_ref()
                .expect("a spell has a cast signature")
                .form()
                .clone(),
        };
        self.activate_mana_for_cost_avoiding_for(player, cost, x, None, &payment_purpose);
        let spent_mana = self.pay_player_cost_for(player, cost, x, &payment_purpose);
        Self::apply_spent_mana_to_spell(&mut stack_object, &spent_mana);
        for sacrificed in sacrifices {
            self.sacrifice_permanent(*sacrificed);
        }
        let cast_event = self
            .stack_trigger_event_object(&stack_object)
            .expect("a cast spell has locked characteristics");
        self.stack.push(stack_object);
        self.consecutive_passes = 0;
        self.events.push(GameEvent::SpellCast {
            player,
            card: stack_id,
            definition,
            targets,
        });
        self.capture_battlefield_triggers(&CommittedTriggerEvent::SpellCast { object: cast_event });
    }

    fn pass_priority(&mut self, _player: PlayerId) {
        self.consecutive_passes += 1;
        if self.consecutive_passes == 1 {
            self.priority = self.priority.opponent();
            return;
        }

        self.consecutive_passes = 0;
        if self.stack.is_empty() {
            self.advance_step();
        } else {
            self.resolve_stack_top();
            if self.result.is_none() {
                self.priority = self.active_player;
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    fn resolve_stack_top(&mut self) {
        let object = self
            .stack
            .pop()
            .expect("resolution is requested only for a nonempty stack");
        self.retire_stack_object(&object);
        let definition = object.card.definition;
        match object.kind {
            StackObjectKind::ActivatedAbility => {
                let source = object
                    .source
                    .expect("activated abilities remember their source");
                let event = if self.resolve_stack_ability(&object) {
                    GameEvent::AbilityResolved {
                        object: object.id,
                        source,
                        definition,
                    }
                } else {
                    GameEvent::AbilityFizzled {
                        object: object.id,
                        source,
                        definition,
                    }
                };
                self.events.push(event);
                return;
            }
            StackObjectKind::TriggeredAbility => {
                let source = object
                    .source
                    .expect("triggered abilities remember their source");
                let event = if self.resolve_stack_ability(&object) {
                    GameEvent::TriggeredAbilityResolved {
                        object: object.id,
                        source,
                        definition,
                    }
                } else {
                    GameEvent::TriggeredAbilityFizzled {
                        object: object.id,
                        source,
                        definition,
                    }
                };
                self.events.push(event);
                return;
            }
            StackObjectKind::Spell => {}
        }
        let behavior = self
            .behavior(definition)
            .unwrap_or(CardBehavior::Unsupported);
        let spell_types = self
            .stack_spell_types(&object)
            .unwrap_or_else(|| behavior.types());
        let aura_host = self.aura_host_for(&object);
        let aura_fizzles =
            spell_types.is_permanent() && aura_host.is_some() && self.spell_fizzles(&object);
        if spell_types.is_permanent() && !aura_fizzles {
            let chosen_player = match object.first_target() {
                Some(Target::Player(player)) => Some(player),
                // "Choose an opponent" has exactly one answer with two players,
                // so the card is cast without asking and the opponent is implied.
                _ if behavior == CardBehavior::BlackVise => Some(object.controller.opponent()),
                _ => None,
            };
            let copy_effect = if behavior == CardBehavior::CopyArtifact {
                object.first_target().and_then(|target| match target {
                    Target::Permanent(id) => self
                        .battlefield
                        .iter()
                        .find(|permanent| permanent.card.id == id)
                        .filter(|permanent| self.is_artifact_permanent(permanent))
                        .map(|permanent| {
                            let mut copy = Self::copiable_characteristics(permanent);
                            copy.added_types = copy.added_types.with(CardType::Enchantment);
                            copy
                        }),
                    Target::Player(_) | Target::Card(_) | Target::Spell(_) => None,
                })
            } else {
                None
            };
            let presented = object
                .signature
                .as_ref()
                .and_then(|signature| match signature.form() {
                    crate::card::SpellForm::Part(part) => Some(*part),
                    crate::card::SpellForm::Combined(parts) => parts.first().copied(),
                })
                .unwrap_or(CardPartId::PRIMARY);
            let starting_loyalty = self
                .catalog
                .get(object.card.definition)
                .and_then(|definition| definition.part(presented))
                .and_then(|part| part.rules.starting_loyalty())
                .map(|loyalty| i16::try_from(loyalty).unwrap_or(i16::MAX));
            let (permanent_card, _zone_change) = self.zone_change_card(object.card.clone());
            self.battlefield.push(Permanent {
                card: permanent_card,
                presented,
                controller: object.controller,
                tapped: self.stack_spell_enters_tapped(&object)
                    || matches!(behavior, CardBehavior::TimeVault),
                entered_controller_turn: self.turns_started[object.controller.index()],
                damage: 0,
                loyalty: starting_loyalty,
                power_bonus: 0,
                toughness_bonus: 0,
                attacking: false,
                unblockable_this_turn: false,
                blocked: false,
                blocking: None,
                chosen_player,
                chosen_creature_type: None,
                destroy_at_end: false,
                temporary_keywords: Vec::new(),
                factory_animated: false,
                dragon_whelp_activations: 0,
                counters: {
                    let mut counters = [0; CounterKind::COUNT];
                    counters[CounterKind::PlusOnePlusOne.index()] = match behavior {
                        CardBehavior::Triskelion | CardBehavior::Tetravus => 3,
                        _ => 0,
                    };
                    counters[CounterKind::Javelin.index()] =
                        u16::from(behavior == CardBehavior::IcatianJavelineers);
                    counters
                },
                attached_to: None,
                exile_instead_of_dying: false,
                combat_damage_assignment: Vec::new(),
                copy_effect: copy_effect.clone(),
                copied_from: None,
                text_changes: object.text_changes.clone(),
                regeneration_shields: 0,
                berserked: false,
                attacked_this_turn: false,
                forestwalk_until_upkeep_of: None,
                damage_sources: Vec::new(),
                deathtouch_damage: false,
            });
            if let Some(copy) = copy_effect {
                let copied_behavior = self
                    .catalog
                    .get(copy.base.0)
                    .and_then(|definition| definition.part(copy.base.1))
                    .and_then(|part| part.rules.special_behavior());
                let copied_from = copy.base;
                if let Some(permanent) = self.battlefield.last_mut() {
                    permanent.copied_from = Some(copied_from);
                    if copied_behavior == Some(CardBehavior::Tetravus) {
                        permanent.set_counters(CounterKind::PlusOnePlusOne, 3);
                    }
                }
            }
            // An Aura enters attached to what its spell targeted. This runs
            // before the entry event so anything watching sees it already
            // attached.
            if let Some(host) = aura_host
                && let Some(permanent) = self.battlefield.last_mut()
            {
                permanent.attached_to = Some(host);
            }
            let entered = self
                .battlefield
                .last()
                .expect("the resolving permanent spell just entered");
            let entered_event = self.trigger_event_object(entered);
            self.capture_battlefield_triggers(&CommittedTriggerEvent::ZoneChanged {
                object: entered_event,
                from: ZoneKind::Stack,
                to: ZoneKind::Battlefield,
            });
        } else if aura_fizzles || self.spell_fizzles(&object) {
            // 608.2b: a spell whose targets are all illegal on resolution does
            // nothing at all — a second Counterspell aimed at the same target
            // arrives to find it gone and goes to the graveyard spent.
            self.events.push(GameEvent::SpellFizzled {
                card: object.id,
                definition,
            });
        } else if object.ability.is_some() {
            let _ = self.resolve_stack_ability(&object);
        } else {
            self.resolve_spell_effect(&object, behavior);
        }
        let card_id = object.id;
        if (!spell_types.is_permanent() || aura_fizzles) && !object.is_copy {
            let owner = object.card.owner;
            // A flashback spell exiles itself instead of returning to the
            // graveyard it was cast from, which is what keeps it from being
            // flashed back again.
            let (card, _zone_change) = self.zone_change_card(object.card);
            if object.cast_via_flashback || behavior == CardBehavior::Recall {
                self.players[owner.index()].exile.push(card);
            } else {
                self.players[owner.index()].graveyard.push(card);
            }
        }
        self.events.push(GameEvent::SpellResolved {
            card: card_id,
            definition,
        });
    }

    /// Sin Collector and Lifebane Zombie reveal the targeted player's hand,
    /// then choose and exile one card matching the source's printed filter.
    fn queue_reveal_and_exile(
        &mut self,
        controller: PlayerId,
        victim: PlayerId,
        behavior: CardBehavior,
    ) {
        self.last_seen_hands[controller.index()] =
            Some((victim, public_cards(&self.players[victim.index()].hand)));
        let eligible = self.players[victim.index()]
            .hand
            .iter()
            .filter(|card| {
                self.catalog
                    .get(card.definition)
                    .is_some_and(|definition| match behavior {
                        CardBehavior::LifebaneZombie => {
                            let colors = definition.rules.colors();
                            definition.rules.has_type(CardType::Creature)
                                && (colors[0] || colors[4])
                        }
                        CardBehavior::SinCollector => {
                            definition.rules.has_type(CardType::Instant)
                                || definition.rules.has_type(CardType::Sorcery)
                        }
                        _ => false,
                    })
            })
            .cloned()
            .collect::<Vec<_>>();
        if eligible.is_empty() {
            // The hand is revealed and holds nothing this card can take. A
            // prompt with no options is answerable, but there is nothing to
            // ask.
            return;
        }
        let options = self.card_decision_options(&eligible, DecisionZone::Hand);
        let prompt = if behavior == CardBehavior::LifebaneZombie {
            "Exile a green or white creature card from their hand"
        } else {
            "Exile an instant or sorcery card from their hand"
        };
        // The hand is revealed, so the choice is public rather than hidden.
        self.queue_decision(
            controller,
            prompt,
            DecisionVisibility::Public,
            DecisionPreference::HigherCardValue,
            1..=1,
            false,
            options,
            DecisionContinuation::ExileFromHand { victim },
        );
    }

    fn resolve_stack_ability(&mut self, object: &StackObject) -> bool {
        if self.stack_ability_fizzles(object) {
            return false;
        }
        // Rule 603.4's second look. A condition that has stopped holding
        // since the ability triggered makes it do nothing at all, which is
        // reported the same way an ability with no legal target is.
        if let Some(ability) = object.ability.as_ref()
            && let Some(condition) = ability.condition
            && !self.trigger_condition_holds(
                condition,
                object.source.unwrap_or(object.id),
                object.controller,
                ability.context,
            )
        {
            return false;
        }
        let (resolver, context, mode_effects) = object
            .ability
            .as_ref()
            .map(|ability| {
                (
                    ability.resolver,
                    ability.context,
                    ability.mode_effects.as_slice(),
                )
            })
            .expect("ability stack objects freeze their complete payload");
        match resolver {
            StackAbilityResolver::Declarative(effect) => {
                self.resolve_effect_def(effect, object, context);
                for effect in mode_effects {
                    self.resolve_effect_def(*effect, object, context);
                }
            }
            StackAbilityResolver::DeclarativeWithCustomFollowup { effect, behavior } => {
                self.resolve_effect_def(effect, object, context);
                for effect in mode_effects {
                    self.resolve_effect_def(*effect, object, context);
                }
                self.resolve_custom_spell_followup(object, behavior);
            }
            StackAbilityResolver::Custom(behavior) => match object.kind {
                StackObjectKind::Spell => self.resolve_spell_effect(object, behavior),
                StackObjectKind::ActivatedAbility => {
                    self.resolve_custom_activated_ability(object, behavior);
                }
                StackObjectKind::TriggeredAbility => {
                    self.resolve_custom_triggered_ability(object, behavior);
                }
            },
        }
        true
    }

    fn resolve_custom_triggered_ability(&mut self, object: &StackObject, behavior: CardBehavior) {
        if matches!(
            behavior,
            CardBehavior::SinCollector | CardBehavior::LifebaneZombie
        ) {
            if let Some(Target::Player(victim)) = self.first_legal_ability_target(object) {
                self.queue_reveal_and_exile(object.controller, victim, behavior);
            }
            return;
        }
        if behavior == CardBehavior::AugurOfBolas {
            let controller = object.controller;
            let revealed = self.take_top_of_library(controller, 3);
            let eligible = revealed
                .iter()
                .filter(|card| {
                    self.catalog.get(card.definition).is_some_and(|definition| {
                        definition.rules.has_type(CardType::Instant)
                            || definition.rules.has_type(CardType::Sorcery)
                    })
                })
                .cloned()
                .collect::<Vec<_>>();
            let options = self.card_decision_options(&eligible, DecisionZone::Library);
            // "You may reveal": taking nothing is a real choice, so the minimum
            // is zero even when something qualifies.
            self.queue_decision(
                controller,
                "Put an instant or sorcery card into your hand",
                DecisionVisibility::Public,
                DecisionPreference::HigherCardValue,
                0..=1,
                false,
                options,
                DecisionContinuation::AugurOfBolas {
                    player: controller,
                    revealed,
                },
            );
        }
    }

    fn resolve_custom_spell_followup(&mut self, object: &StackObject, behavior: CardBehavior) {
        if behavior == CardBehavior::ChainLightning {
            let deciding = match object.first_target() {
                Some(Target::Player(player)) => Some(player),
                Some(Target::Permanent(id)) => self.permanent_controller(id),
                Some(Target::Card(_) | Target::Spell(_)) | None => None,
            };
            if let Some(player) = deciding {
                self.queue_chain_lightning_decision(player, object.clone());
            }
        }
    }

    fn stack_ability_fizzles(&self, object: &StackObject) -> bool {
        let Some(ability) = &object.ability else {
            return false;
        };
        let mut had_target = false;
        let mut has_legal_target = false;
        for selection in &ability.targets {
            let Some(definition) = ability
                .target_defs
                .iter()
                .find(|definition| definition.id == selection.slot())
            else {
                continue;
            };
            for target in selection.targets() {
                had_target = true;
                has_legal_target |=
                    self.stack_ability_target_is_legal(object, definition.id, *target);
            }
        }
        had_target && !has_legal_target
    }

    #[allow(clippy::too_many_lines)]
    fn resolve_effect_def(
        &mut self,
        effect: EffectDef,
        object: &StackObject,
        context: TriggerContext,
    ) {
        match effect {
            EffectDef::Sequence(effects) => {
                for effect in effects {
                    self.resolve_effect_def(*effect, object, context);
                }
            }
            EffectDef::AddMana(AddManaEffectDef {
                mana: ManaSelectionDef::One(kind),
                amount,
                restrictions,
                spend_effects,
            }) => {
                let color = kind;
                let source = object
                    .source
                    .zip(object.ability_origin())
                    .map(|(object, ability)| ManaSource { object, ability });
                let mana = Mana {
                    color,
                    source,
                    restrictions,
                    spend_effects,
                };
                self.add_mana(
                    object.controller,
                    std::iter::repeat_n(mana, usize::from(amount)),
                );
            }
            EffectDef::DealDamage { recipient, amount } => {
                let amount = self
                    .effect_value(amount, object, context)
                    .max(0)
                    .try_into()
                    .unwrap_or(u16::MAX);
                for target in self.effect_recipients(recipient, object, context) {
                    self.damage_target_from(
                        object.source.or(Some(object.id)),
                        Some(target),
                        amount,
                    );
                }
            }
            EffectDef::GainLife { recipient, amount } => {
                let amount = self
                    .effect_value(amount, object, context)
                    .max(0)
                    .try_into()
                    .unwrap_or(u16::MAX);
                for target in self.effect_recipients(recipient, object, context) {
                    if let Target::Player(player) = target {
                        self.gain_life(player, amount);
                    }
                }
            }
            EffectDef::DrawCards { recipient, amount } => {
                let amount = self
                    .effect_value(amount, object, context)
                    .max(0)
                    .try_into()
                    .unwrap_or(u16::MAX);
                for target in self.effect_recipients(recipient, object, context) {
                    if let Target::Player(player) = target {
                        self.draw_cards(player, amount);
                    }
                }
            }
            EffectDef::DiscardCards { recipient, amount } => {
                let amount = self.effect_value(amount, object, context).max(0);
                let cause = ZoneMoveCause::Effect {
                    controller: object.controller,
                };
                for target in self.effect_recipients(recipient, object, context) {
                    if let Target::Player(player) = target {
                        self.queue_effect_discard(player, amount, cause);
                    }
                }
            }
            EffectDef::LoseLife { recipient, amount } => {
                let amount = self
                    .effect_value(amount, object, context)
                    .max(0)
                    .try_into()
                    .unwrap_or(u16::MAX);
                for target in self.effect_recipients(recipient, object, context) {
                    if let Target::Player(player) = target {
                        self.lose_life(player, amount);
                    }
                }
            }
            EffectDef::Tap { object: recipient } => {
                for target in self.effect_recipients(recipient, object, context) {
                    if let Target::Permanent(permanent) = target {
                        let _ = self.tap_permanent(permanent);
                    }
                }
            }
            EffectDef::CreateToken { token, count } => {
                for _ in 0..self.effect_value(count, object, context).max(0) {
                    self.create_token(object.controller, token);
                }
            }
            EffectDef::Untap { object: recipient } => {
                for target in self.effect_recipients(recipient, object, context) {
                    if let Target::Permanent(id) = target
                        && let Some(permanent) = self
                            .battlefield
                            .iter_mut()
                            .find(|candidate| candidate.card.id == id)
                    {
                        permanent.tapped = false;
                    }
                }
            }
            EffectDef::Destroy {
                object: recipient,
                can_regenerate,
            } => {
                let permanents = self
                    .effect_recipients(recipient, object, context)
                    .into_iter()
                    .filter_map(|target| match target {
                        Target::Permanent(permanent) => Some(permanent),
                        Target::Player(_) | Target::Card(_) | Target::Spell(_) => None,
                    })
                    .collect::<Vec<_>>();
                self.destroy_permanents(&permanents, can_regenerate);
            }
            EffectDef::Sacrifice { object: recipient } => {
                let permanents = self
                    .effect_recipients(recipient, object, context)
                    .into_iter()
                    .filter_map(|target| match target {
                        Target::Permanent(permanent) => Some(permanent),
                        Target::Player(_) | Target::Card(_) | Target::Spell(_) => None,
                    })
                    .filter(|permanent| {
                        self.permanent_controller(*permanent)
                            .is_none_or(|controller| {
                                self.can_be_forced_to_sacrifice(controller, object.controller)
                            })
                    })
                    .collect::<Vec<_>>();
                self.destroy_permanents(&permanents, false);
            }
            EffectDef::SacrificeOfChoice {
                player: recipient,
                object: predicate,
            } => {
                let source = object.source.unwrap_or(object.id);
                for target in self.effect_recipients(recipient, object, context) {
                    if let Target::Player(player) = target
                        && self.can_be_forced_to_sacrifice(player, object.controller)
                    {
                        self.queue_chosen_sacrifice(player, predicate, source);
                    }
                }
            }
            EffectDef::GrantFlashToNextSorcery => {
                let grants = &mut self.sorcery_flash_grants[object.controller.index()];
                *grants = grants.saturating_add(1);
            }
            EffectDef::May(inner) => {
                self.queue_optional_effect(object.controller, object, context, inner);
            }
            EffectDef::ExileLinkedToSource { object: recipient } => {
                let source = object.source.unwrap_or(object.id);
                for target in self.effect_recipients(recipient, object, context) {
                    let exiled = match target {
                        Target::Permanent(id) => self.exile_permanent_returning_card(id),
                        Target::Card(id) => self.exile_card_returning_card(id),
                        Target::Player(_) | Target::Spell(_) => None,
                    };
                    if let Some(exiled) = exiled {
                        self.linked_exiles.push((source, exiled));
                    }
                }
            }
            EffectDef::ReturnLinkedExiles { zone, grant } => {
                let source = object.source.unwrap_or(object.id);
                let returning = self
                    .linked_exiles
                    .iter()
                    .filter(|(exiled_by, _)| *exiled_by == source)
                    .map(|(_, card)| *card)
                    .collect::<Vec<_>>();
                self.linked_exiles
                    .retain(|(exiled_by, _)| *exiled_by != source);
                for card in returning {
                    self.return_exiled_card(card, zone, grant);
                }
            }
            EffectDef::MakeUnblockableThisTurn { object: recipient } => {
                for target in self.effect_recipients(recipient, object, context) {
                    if let Target::Permanent(id) = target
                        && let Some(permanent) = self
                            .battlefield
                            .iter_mut()
                            .find(|permanent| permanent.card.id == id)
                    {
                        permanent.unblockable_this_turn = true;
                    }
                }
            }
            EffectDef::AtNextStep {
                step,
                player,
                effect,
            } => {
                self.delayed_triggers.push(DelayedTrigger {
                    object: Box::new(object.clone()),
                    step,
                    player,
                    effect,
                });
            }
            EffectDef::Counter { object: recipient } => {
                for target in self.effect_recipients(recipient, object, context) {
                    if let Target::Spell(spell) = target {
                        self.counter_spell(spell);
                    }
                }
            }
            EffectDef::CounterUnlessPaid {
                object: recipient,
                amount,
                zone,
            } => {
                let amount = self
                    .effect_value(amount, object, context)
                    .max(0)
                    .try_into()
                    .unwrap_or(u16::MAX);
                let zone = if zone == ZoneKind::Exile {
                    CounteredSpellZone::Exile
                } else {
                    CounteredSpellZone::Graveyard
                };
                for target in self.effect_recipients(recipient, object, context) {
                    if let Target::Spell(spell) = target {
                        self.queue_counter_unless_paid(spell, amount, zone);
                    }
                }
            }
            EffectDef::AddCounters {
                object: recipient,
                kind,
                amount,
            } => {
                let amount = self
                    .effect_value(amount, object, context)
                    .max(0)
                    .try_into()
                    .unwrap_or(u16::MAX);
                for target in self.effect_recipients(recipient, object, context) {
                    if let Target::Permanent(permanent) = target
                        && let Some(permanent) = self
                            .battlefield
                            .iter_mut()
                            .find(|candidate| candidate.card.id == permanent)
                    {
                        permanent.add_counters(kind, amount);
                    }
                }
            }
            EffectDef::ChangeTextBasicLandType { object: recipient } => {
                if let Some(target) = self
                    .effect_recipients(recipient, object, context)
                    .into_iter()
                    .next()
                {
                    self.queue_basic_land_type_text_change(object.controller, target);
                }
            }
            EffectDef::BecomeCopyOf {
                object: recipient,
                retain_source_ability,
            } => {
                let Some(Target::Permanent(target)) = self
                    .effect_recipients(recipient, object, context)
                    .into_iter()
                    .next()
                else {
                    return;
                };
                let Some(mut copy) = self
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == target)
                    .map(Self::copiable_characteristics)
                else {
                    return;
                };
                if retain_source_ability
                    && let Some(payload) = &object.ability
                    && let Some(definition) = payload.definition.as_deref()
                {
                    copy.added_abilities.push(CopiableAbility {
                        origin: payload.origin,
                        definition: *definition,
                    });
                }
                if let Some(source) = object.source
                    && let Some(permanent) = self
                        .battlefield
                        .iter_mut()
                        .find(|permanent| permanent.card.id == source)
                {
                    permanent.copy_effect = Some(copy);
                }
            }
            EffectDef::OptionalManaPayment { cost, effect } => {
                self.queue_optional_mana_payment(object.controller, cost, object, context, effect);
            }
            EffectDef::Apply {
                recipient,
                effect,
                duration,
            } => self.resolve_applied_effect(recipient, effect, duration, object, context),
            EffectDef::MoveToZone {
                object: recipient,
                zone,
            } => {
                for target in self.effect_recipients(recipient, object, context) {
                    self.move_target_to_zone(
                        target,
                        zone,
                        ZoneMoveCause::Effect {
                            controller: object.controller,
                        },
                    );
                }
            }
            // An Aura attaches as its spell becomes a permanent, which is
            // handled where the permanent enters rather than here.
            EffectDef::Attach { .. }
            | EffectDef::None
            | EffectDef::AddMana(AddManaEffectDef {
                mana: ManaSelectionDef::Choice(_),
                ..
            })
            | EffectDef::EntersTapped
            | EffectDef::CannotBeForcedToSacrifice
            | EffectDef::ReduceGenericCostBy(_)
            | EffectDef::MultiplyEventAmount(_)
            | EffectDef::ChooseCreatureType { .. }
            | EffectDef::Special(_) => {
                // Choice-bearing mana and the remaining declarative effect
                // families are execution seams until a supported card needs
                // their concrete rules procedure.
            }
        }
    }

    fn effect_value(&self, value: ValueDef, object: &StackObject, context: TriggerContext) -> i32 {
        match value {
            ValueDef::Constant(value) => value,
            ValueDef::ChosenX => i32::from(object.x()),
            ValueDef::SourcePower => object
                .source
                .and_then(|source| self.current_or_last_known_power(source))
                .map_or(0, i32::from),
            ValueDef::SourceToughness => object
                .source
                .and_then(|source| self.current_or_last_known_toughness(source))
                .map_or(0, i32::from),
            ValueDef::TriggerEventAmount => context.amount.unwrap_or(0),
            ValueDef::CountersOnSource(kind) => object.source.map_or(0, |source| {
                i32::from(self.current_or_last_known_counters(source, kind))
            }),
            ValueDef::CardsInHandAbove { player, threshold } => {
                let player = [PlayerId::One, PlayerId::Two]
                    .into_iter()
                    .find(|candidate| {
                        self.player_relation_matches(*candidate, player, object.controller, context)
                    })
                    .unwrap_or(object.controller);
                i32::try_from(
                    self.players[player.index()]
                        .hand
                        .len()
                        .saturating_sub(usize::from(threshold)),
                )
                .unwrap_or(i32::MAX)
            }
            ValueDef::CountMatchingObjects(query) => {
                let recipient = EffectRecipientDef::MatchingObjects {
                    object: query.object,
                    zones: query.zones,
                    controller: query.controller,
                };
                i32::try_from(self.effect_recipients(recipient, object, context).len())
                    .unwrap_or(i32::MAX)
            }
            ValueDef::AnyMatchingObject(query) => i32::from(self.any_battlefield_object_matches(
                query,
                object.source.unwrap_or(object.id),
                object.controller,
            )),
            ValueDef::IfTargetMatches(condition) => {
                let source = object.source.unwrap_or(object.id);
                let matched = self
                    .effect_recipients(EffectRecipientDef::Target(condition.slot), object, context)
                    .into_iter()
                    .any(|target| match target {
                        Target::Card(id) => {
                            self.card_in_nonbattlefield_zone(id)
                                .is_some_and(|(zone, card)| {
                                    self.card_object_matches(condition.object, card, zone, source)
                                })
                        }
                        Target::Permanent(id) => self
                            .battlefield
                            .iter()
                            .find(|permanent| permanent.card.id == id)
                            .is_some_and(|permanent| {
                                self.trigger_object_matches(
                                    condition.object,
                                    &self.trigger_event_object(permanent),
                                    source,
                                    false,
                                )
                            }),
                        Target::Player(_) | Target::Spell(_) => false,
                    });
                let chosen = if matched {
                    condition.then
                } else {
                    condition.otherwise
                };
                self.effect_value(chosen, object, context)
            }
            ValueDef::IfMatchingObjectCount(condition) => {
                let count = self.effect_value(
                    ValueDef::CountMatchingObjects(&condition.query),
                    object,
                    context,
                );
                let chosen = if count == i32::from(condition.equals) {
                    condition.then
                } else {
                    condition.otherwise
                };
                self.effect_value(chosen, object, context)
            }
            ValueDef::IfCreatureDiedThisTurn(branches) => {
                let chosen = if self.creature_died_this_turn {
                    branches.then
                } else {
                    branches.otherwise
                };
                self.effect_value(chosen, object, context)
            }
            ValueDef::Negate(inner) => self.effect_value(*inner, object, context).saturating_neg(),
        }
    }

    /// Moves one object to a zone. Only the moves a supported card actually
    /// makes are handled; the rest stay seams rather than guesses.
    fn move_target_to_zone(&mut self, target: Target, zone: ZoneKind, cause: ZoneMoveCause) {
        if let Target::Permanent(id) = target {
            // Leaving the battlefield has its own procedure: last-known
            // information, exit events, and the triggers watching for them.
            match zone {
                ZoneKind::Exile => self.exile_permanent(id),
                ZoneKind::Hand => self.return_permanent_to_hand(id),
                ZoneKind::Graveyard => self.destroy_permanent_without_regeneration(id),
                ZoneKind::Library => self.return_permanent_to_library_top(id),
                ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => {}
            }
            return;
        }
        let Target::Card(id) = target else {
            return;
        };
        let Some(from) = self
            .card_in_nonbattlefield_zone(id)
            .map(|(from, _card)| from)
        else {
            return;
        };
        let _ = self.move_card_from_nonbattlefield_zone(id, from, zone, cause);
    }

    fn resolve_applied_effect(
        &mut self,
        recipient: EffectRecipientDef,
        effect: AppliedEffectDef,
        duration: EffectDurationDef,
        object: &StackObject,
        context: TriggerContext,
    ) {
        for target in self.effect_recipients(recipient, object, context) {
            self.apply_applied_effect_component(target, effect, object, context);
        }
        // Current supported Apply effects all last until cleanup. Keeping the
        // duration explicit here makes unsupported permanent/granted effects
        // visible rather than silently changing their lifetime.
        debug_assert!(matches!(
            duration,
            EffectDurationDef::UntilEndOfTurn | EffectDurationDef::Permanent
        ));
    }

    fn apply_applied_effect_component(
        &mut self,
        target: Target,
        effect: AppliedEffectDef,
        object: &StackObject,
        context: TriggerContext,
    ) {
        match effect {
            AppliedEffectDef::Composite(effects) => {
                for effect in effects {
                    self.apply_applied_effect_component(target, *effect, object, context);
                }
            }
            AppliedEffectDef::GrantAbility(ability) => match target {
                Target::Card(target) => {
                    let grant = TemporaryAbilityGrant {
                        object: target,
                        ability,
                    };
                    if self.card_in_nonbattlefield_zone(target).is_some()
                        && !self.temporary_ability_grants.contains(&grant)
                    {
                        self.temporary_ability_grants.push(grant);
                    }
                }
                Target::Permanent(target) => {
                    if let DeclarativeAbilityDef::Keyword(keyword) = ability.definition
                        && let Some(permanent) = self
                            .battlefield
                            .iter_mut()
                            .find(|permanent| permanent.card.id == target)
                        && !permanent.temporary_keywords.contains(&keyword)
                    {
                        permanent.temporary_keywords.push(keyword);
                    }
                }
                Target::Player(_) | Target::Spell(_) => {}
            },
            AppliedEffectDef::ModifyPowerToughness { power, toughness } => {
                let Target::Permanent(target) = target else {
                    return;
                };
                let power = i16::try_from(
                    self.effect_value(power, object, context)
                        .clamp(i32::from(i16::MIN), i32::from(i16::MAX)),
                )
                .expect("the effect value was clamped to i16");
                let toughness = i16::try_from(
                    self.effect_value(toughness, object, context)
                        .clamp(i32::from(i16::MIN), i32::from(i16::MAX)),
                )
                .expect("the effect value was clamped to i16");
                if let Some(permanent) = self
                    .battlefield
                    .iter_mut()
                    .find(|permanent| permanent.card.id == target)
                {
                    permanent.power_bonus = permanent.power_bonus.saturating_add(power);
                    permanent.toughness_bonus = permanent.toughness_bonus.saturating_add(toughness);
                }
            }
            AppliedEffectDef::CannotBeCountered
            | AppliedEffectDef::CannotBeBlockedBy(_)
            | AppliedEffectDef::AddLandTypes(_)
            | AppliedEffectDef::Special(_) => {}
        }
    }

    fn live_object_target(&self, object: GameObjectId) -> Option<Target> {
        if self
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == object)
        {
            return Some(Target::Permanent(object));
        }
        if self.stack.iter().any(|candidate| candidate.id == object) {
            return Some(Target::Spell(object));
        }
        self.card_in_nonbattlefield_zone(object)
            .is_some()
            .then_some(Target::Card(object))
    }

    fn effect_recipients(
        &self,
        recipient: EffectRecipientDef,
        object: &StackObject,
        context: TriggerContext,
    ) -> Vec<Target> {
        if let EffectRecipientDef::Target(slot) = recipient {
            let selections = object
                .signature
                .as_ref()
                .map(CastSignature::targets)
                .or_else(|| {
                    object
                        .ability
                        .as_ref()
                        .map(|ability| ability.targets.as_slice())
                });
            return selections
                .and_then(|selections| selections.iter().find(|selection| selection.slot() == slot))
                .into_iter()
                .flat_map(TargetSelection::targets)
                .copied()
                .filter(|target| self.stack_ability_target_is_legal(object, slot, *target))
                .collect();
        }

        if let EffectRecipientDef::ObjectsSharingNameWithTarget(slot) = recipient {
            return self.objects_sharing_name_with_target(slot, object, context);
        }

        let EffectRecipientDef::MatchingObjects {
            object: predicate,
            zones,
            controller,
        } = recipient
        else {
            return match recipient {
                EffectRecipientDef::Source => object.source.map(Target::Permanent),
                EffectRecipientDef::AttachedPermanent => object
                    .source
                    .and_then(|source| self.attached_host(source))
                    .map(Target::Permanent),
                EffectRecipientDef::Controller => Some(Target::Player(object.controller)),
                EffectRecipientDef::Opponent => Some(Target::Player(object.controller.opponent())),
                EffectRecipientDef::TriggeringObject => context
                    .object
                    .and_then(|object| self.live_object_target(object)),
                EffectRecipientDef::ControllerOfTriggeringObject => context
                    .object
                    .and_then(|object| self.current_or_last_known_controller(object))
                    .or(context.object_controller)
                    .map(Target::Player),
                EffectRecipientDef::EventPlayer => context.event_player.map(Target::Player),
                EffectRecipientDef::Target(_)
                | EffectRecipientDef::MatchingObjects { .. }
                | EffectRecipientDef::ObjectsSharingNameWithTarget(_) => {
                    unreachable!("target, matching, and shared-name recipients returned above")
                }
            }
            .into_iter()
            .collect();
        };

        self.matching_objects(
            predicate,
            zones,
            controller,
            object.source.unwrap_or(object.id),
            object.controller,
            context,
        )
    }

    /// Whether a trigger's intervening-if condition holds right now. Rule
    /// 603.4 asks this when the ability would trigger and again as it
    /// resolves, so both call sites read the same board.
    fn trigger_condition_holds(
        &self,
        condition: &TriggerConditionDef,
        source: GameObjectId,
        controller: PlayerId,
        context: TriggerContext,
    ) -> bool {
        let TriggerConditionDef::ObjectCount {
            query,
            comparison,
            amount,
        } = condition;
        let count = self
            .matching_objects(
                query.object,
                query.zones,
                query.controller,
                source,
                controller,
                context,
            )
            .len();
        let amount = usize::from(*amount);
        match comparison {
            ComparisonDef::AtLeast => count >= amount,
            ComparisonDef::AtMost => count <= amount,
            ComparisonDef::Exactly => count == amount,
        }
    }

    /// Every object a zone-scoped query matches. A trigger's intervening-if
    /// condition is read before any stack object exists, so this takes the
    /// ability's source and controller directly rather than a resolving
    /// object.
    fn matching_objects(
        &self,
        predicate: ObjectPredicateDef,
        zones: &'static [ZoneKind],
        controller: PlayerRelation,
        source: GameObjectId,
        ability_controller: PlayerId,
        context: TriggerContext,
    ) -> Vec<Target> {
        let mut recipients = Vec::new();
        if zones.contains(&ZoneKind::Battlefield) {
            recipients.extend(self.battlefield.iter().filter_map(|permanent| {
                let characteristics = self.trigger_event_object(permanent);
                (self.player_relation_matches(
                    permanent.controller,
                    controller,
                    ability_controller,
                    context,
                ) && self.trigger_object_matches(predicate, &characteristics, source, false))
                .then_some(Target::Permanent(permanent.card.id))
            }));
        }
        if zones.contains(&ZoneKind::Stack) {
            recipients.extend(self.stack.iter().filter_map(|candidate| {
                let characteristics = self.stack_trigger_event_object(candidate)?;
                (candidate.kind == StackObjectKind::Spell
                    && self.player_relation_matches(
                        candidate.controller,
                        controller,
                        ability_controller,
                        context,
                    )
                    && self.trigger_object_matches(predicate, &characteristics, source, true))
                .then_some(Target::Spell(candidate.id))
            }));
        }
        // The same card zones the target enumerator understands. Without this
        // a sweep over graveyards matched nothing and the clause was inert.
        for zone in [
            ZoneKind::Library,
            ZoneKind::Hand,
            ZoneKind::Graveyard,
            ZoneKind::Exile,
            ZoneKind::Command,
        ] {
            if !zones.contains(&zone) {
                continue;
            }
            recipients.extend(self.cards_in_zone(zone).filter_map(|card| {
                (self.player_relation_matches(card.owner, controller, ability_controller, context)
                    && self.card_object_matches(predicate, card, zone, source))
                .then_some(Target::Card(card.id))
            }));
        }
        recipients
    }

    fn stack_ability_target_is_legal(
        &self,
        object: &StackObject,
        slot: TargetSlotId,
        target: Target,
    ) -> bool {
        let source = object.source.unwrap_or(object.id);
        let Some(ability) = &object.ability else {
            return true;
        };
        let Some(definition) = ability
            .target_defs
            .iter()
            .find(|definition| definition.id == slot)
        else {
            // Legacy custom actions can carry targets without a declarative
            // target slot. Their historic resolver remains authoritative.
            return true;
        };
        if Self::ability_target_uses_custom_predicate(definition.predicate) {
            // Custom activated handlers offered these targets before the
            // shared predicate vocabulary could express their full legality.
            // Preserve their prior zone-presence check until the named
            // predicate itself is migrated; treating `Special` as no matches
            // would incorrectly counter every such ability on resolution.
            return match target {
                Target::Player(_) => true,
                Target::Card(id) => self.card_in_nonbattlefield_zone(id).is_some(),
                Target::Permanent(id) => self
                    .battlefield
                    .iter()
                    .any(|permanent| permanent.card.id == id),
                Target::Spell(id) => self.stack.iter().any(|candidate| candidate.id == id),
            };
        }
        self.ability_targets_matching(
            definition.predicate,
            object.controller,
            source,
            ability.context,
        )
        .contains(&target)
    }

    fn ability_target_uses_custom_predicate(predicate: AbilityTargetPredicate) -> bool {
        match predicate {
            AbilityTargetPredicate::AnyTarget | AbilityTargetPredicate::Player(_) => false,
            AbilityTargetPredicate::Object { object, .. } => {
                Self::object_predicate_uses_custom_predicate(object)
            }
        }
    }

    fn object_predicate_uses_custom_predicate(predicate: ObjectPredicateDef) -> bool {
        match predicate {
            ObjectPredicateDef::Special(_) => true,
            ObjectPredicateDef::All(predicates) | ObjectPredicateDef::AnyOf(predicates) => {
                predicates
                    .iter()
                    .any(|predicate| Self::object_predicate_uses_custom_predicate(*predicate))
            }
            ObjectPredicateDef::Not(predicate) => {
                Self::object_predicate_uses_custom_predicate(*predicate)
            }
            ObjectPredicateDef::Any
            | ObjectPredicateDef::Source
            | ObjectPredicateDef::HasType(_)
            | ObjectPredicateDef::Spell
            | ObjectPredicateDef::NoncreatureSpell
            | ObjectPredicateDef::Color(_)
            | ObjectPredicateDef::Subtype(_)
            | ObjectPredicateDef::ManaValueAtMost(_)
            | ObjectPredicateDef::ManaValueEqualTo(_)
            | ObjectPredicateDef::ManaValueAtMostValue(_)
            | ObjectPredicateDef::PowerAtLeast(_)
            | ObjectPredicateDef::ControlledBy(_)
            | ObjectPredicateDef::Supertype(_)
            | ObjectPredicateDef::SharesNameWithSource
            | ObjectPredicateDef::AttackingOrBlocking
            | ObjectPredicateDef::Attacking
            | ObjectPredicateDef::HasKeyword(_) => false,
        }
    }

    fn first_legal_ability_target(&self, object: &StackObject) -> Option<Target> {
        object.ability.as_ref().and_then(|ability| {
            ability.targets.iter().find_map(|selection| {
                selection.targets().iter().copied().find(|target| {
                    self.stack_ability_target_is_legal(object, selection.slot(), *target)
                })
            })
        })
    }

    fn resolve_custom_activated_ability(&mut self, object: &StackObject, behavior: CardBehavior) {
        match behavior {
            CardBehavior::ChaosOrb
                if self
                    .battlefield
                    .iter()
                    .any(|permanent| Some(permanent.card.id) == object.source) =>
            {
                if let Some(chosen) = object.chosen_permanents.first().copied() {
                    self.destroy_permanent(chosen);
                }
                self.destroy_permanent(object.source.expect("ability has a source"));
            }
            CardBehavior::OrcishMechanics => {
                self.damage_target(self.first_legal_ability_target(object), 2);
            }
            CardBehavior::IcyManipulator => {
                if let Some(Target::Permanent(target)) = self.first_legal_ability_target(object) {
                    let _ = self.tap_permanent(target);
                }
            }
            CardBehavior::Pendelhaven => {
                if let Some(Target::Permanent(target)) = self.first_legal_ability_target(object)
                    && let Some(permanent) = self
                        .battlefield
                        .iter_mut()
                        .find(|permanent| permanent.card.id == target)
                {
                    permanent.power_bonus += 1;
                    permanent.toughness_bonus += 2;
                }
            }
            CardBehavior::SedgeTroll => {
                if let Some(permanent) = self
                    .battlefield
                    .iter_mut()
                    .find(|permanent| Some(permanent.card.id) == object.source)
                {
                    permanent.regeneration_shields =
                        permanent.regeneration_shields.saturating_add(1);
                }
            }
            CardBehavior::Triskelion | CardBehavior::IcatianJavelineers => {
                self.damage_target(self.first_legal_ability_target(object), 1);
            }
            CardBehavior::LibraryOfAlexandria => {
                self.draw_cards(object.controller, 1);
            }
            CardBehavior::MazeOfIth => {
                if let Some(Target::Permanent(target)) = self.first_legal_ability_target(object)
                    && let Some(creature) = self
                        .battlefield
                        .iter_mut()
                        .find(|permanent| permanent.card.id == target)
                {
                    creature.tapped = false;
                    creature.attacking = false;
                    creature.blocked = false;
                    creature.combat_damage_assignment.clear();
                }
            }
            CardBehavior::NevinyrralsDisk => {
                let doomed = self
                    .battlefield
                    .iter()
                    .filter(|permanent| {
                        self.permanent_types(permanent).is_some_and(|types| {
                            types.contains(CardType::Creature)
                                || types.contains(CardType::Artifact)
                                || types.contains(CardType::Enchantment)
                        })
                    })
                    .map(|permanent| permanent.card.id)
                    .collect::<Vec<_>>();
                self.destroy_permanents(&doomed, true);
            }
            CardBehavior::TimeVault => self.extra_turns.push(object.controller),
            _ => {}
        }
    }

    #[allow(clippy::too_many_lines)]
    fn resolve_spell_effect(&mut self, object: &StackObject, behavior: CardBehavior) {
        match behavior {
            CardBehavior::SphinxsRevelation => {
                let player = object.controller;
                self.gain_life(player, object.x());
                self.draw_cards(player, object.x());
            }
            CardBehavior::ManaDrain => {
                if let Some(Target::Spell(target)) = object.first_target() {
                    let drained = self
                        .stack
                        .iter()
                        .find(|candidate| candidate.id == target)
                        .map_or(0, |candidate| self.stack_spell_mana_value(candidate));
                    self.counter_spell(target);
                    self.mana_drain_pending[object.controller.index()] =
                        self.mana_drain_pending[object.controller.index()].saturating_add(drained);
                }
            }
            CardBehavior::LightningBolt => {
                self.damage_target(object.first_target(), 3);
            }
            CardBehavior::PillarOfFlame => {
                self.damage_target(object.first_target(), 2);
                if let Some(Target::Permanent(target)) = object.first_target()
                    && let Some(permanent) = self
                        .battlefield
                        .iter_mut()
                        .find(|permanent| permanent.card.id == target)
                {
                    permanent.exile_instead_of_dying = true;
                }
            }
            CardBehavior::WarleadersHelix => {
                self.damage_target(object.first_target(), 4);
                self.gain_life(object.controller, 4);
            }
            CardBehavior::GiantGrowth => {
                if let Some(Target::Permanent(target)) = object.first_target()
                    && let Some(permanent) = self
                        .battlefield
                        .iter_mut()
                        .find(|permanent| permanent.card.id == target)
                {
                    permanent.power_bonus += 3;
                    permanent.toughness_bonus += 3;
                }
            }
            CardBehavior::Berserk => {
                if let Some(Target::Permanent(target)) = object.first_target() {
                    let current_power = self
                        .battlefield
                        .iter()
                        .find(|permanent| permanent.card.id == target)
                        .and_then(|permanent| self.power(permanent))
                        .unwrap_or(0)
                        .max(0);
                    if let Some(permanent) = self
                        .battlefield
                        .iter_mut()
                        .find(|permanent| permanent.card.id == target)
                    {
                        permanent.power_bonus += current_power;
                        if !permanent
                            .temporary_keywords
                            .contains(&KeywordAbility::Trample)
                        {
                            permanent.temporary_keywords.push(KeywordAbility::Trample);
                        }
                        permanent.berserked = true;
                    }
                }
            }
            CardBehavior::GoblinGrenade => {
                self.damage_target(object.first_target(), 5);
            }
            CardBehavior::ChainLightning => {
                let deciding = match object.first_target() {
                    Some(Target::Player(player)) => Some(player),
                    Some(Target::Permanent(id)) => self.permanent_controller(id),
                    Some(Target::Card(_) | Target::Spell(_)) | None => None,
                };
                self.damage_target(object.first_target(), 3);
                if let Some(player) = deciding {
                    self.queue_chain_lightning_decision(player, object.clone());
                }
            }
            CardBehavior::Fireball => {
                let divisor = u16::try_from(object.target_count()).unwrap_or(u16::MAX);
                let amount = object.x().checked_div(divisor).unwrap_or(0);
                for target in object.targets() {
                    self.damage_target(Some(target), amount);
                }
            }
            CardBehavior::DrainLife => {
                self.damage_target(object.first_target(), object.x());
                self.gain_life(object.controller, object.x());
            }
            CardBehavior::Earthquake => {
                for player in [PlayerId::One, PlayerId::Two] {
                    self.deal_damage(player, object.x());
                }
                let targets = self
                    .battlefield
                    .iter()
                    .filter(|permanent| {
                        self.power(permanent).is_some() && !self.has_flying(permanent)
                    })
                    .map(|permanent| permanent.card.id)
                    .collect::<Vec<_>>();
                for target in targets {
                    self.damage_target(Some(Target::Permanent(target)), object.x());
                }
            }
            // Doom Blade belongs here rather than with Terror: it says
            // nothing about regeneration, so the ordinary destroy applies.
            CardBehavior::DoomBlade | CardBehavior::UltimatePrice => {
                if let Some(Target::Permanent(target)) = object.first_target() {
                    self.destroy_permanent(target);
                }
            }
            CardBehavior::Terror | CardBehavior::Putrefy => {
                if let Some(Target::Permanent(target)) = object.first_target() {
                    self.destroy_permanent_without_regeneration(target);
                }
            }
            CardBehavior::DustToDust => {
                for target in object.iter_targets().filter_map(|target| match target {
                    Target::Permanent(id) => Some(*id),
                    Target::Player(_) | Target::Card(_) | Target::Spell(_) => None,
                }) {
                    self.exile_permanent(target);
                }
            }
            CardBehavior::HurkylsRecall => {
                if let Some(Target::Player(player)) = object.first_target() {
                    let artifacts: Vec<_> = self
                        .battlefield
                        .iter()
                        .filter(|permanent| {
                            permanent.controller == player && self.is_artifact_permanent(permanent)
                        })
                        .map(|permanent| permanent.card.id)
                        .collect();
                    for artifact in artifacts {
                        self.return_permanent_to_hand(artifact);
                    }
                }
            }
            CardBehavior::DivineOffering => {
                if let Some(Target::Permanent(target)) = object.first_target()
                    && let Some(permanent) = self
                        .battlefield
                        .iter()
                        .find(|permanent| permanent.card.id == target)
                {
                    let life = self.permanent_mana_value(permanent);
                    self.destroy_permanent(target);
                    self.gain_life(object.controller, life);
                }
            }
            CardBehavior::SwordsToPlowshares => {
                if let Some(Target::Permanent(target)) = object.first_target()
                    && let Some(index) = self.battlefield.iter().position(|permanent| {
                        permanent.card.id == target
                            && !self.is_protected_from_colors(permanent, behavior.rules().colors())
                    })
                {
                    let controller = self.battlefield[index].controller;
                    let life = self.power(&self.battlefield[index]).unwrap_or(0).max(0);
                    self.exile_permanent(target);
                    self.players[controller.index()].life += life;
                }
            }
            CardBehavior::Negate | CardBehavior::EssenceScatter | CardBehavior::Dispel => {
                if let Some(Target::Spell(target)) = object.first_target() {
                    self.counter_spell(target);
                }
            }
            CardBehavior::Dissipate => {
                if let Some(Target::Spell(target)) = object.first_target() {
                    self.counter_spell_into(target, CounteredSpellZone::Exile);
                }
            }
            CardBehavior::Detonate => {
                if let Some(Target::Permanent(target)) = object.first_target()
                    && let Some(controller) = self.permanent_controller(target)
                {
                    self.destroy_permanent(target);
                    self.deal_damage(controller, object.x());
                }
            }
            CardBehavior::Fork => {
                if let Some(Target::Spell(target)) = object.first_target()
                    && let Some(original) =
                        self.stack.iter().find(|item| item.id == target).cloned()
                {
                    self.queue_fork_decision(object.controller, original);
                }
            }
            CardBehavior::WheelOfFortune => self.resolve_wheel_of_fortune(object.controller),
            CardBehavior::Timetwister => self.resolve_timetwister(),
            CardBehavior::TimeWalk => self.extra_turns.push(object.controller),
            CardBehavior::Channel => self.channel_active[object.controller.index()] = true,
            CardBehavior::DemonicTutor => {
                let options = self.players[object.controller.index()]
                    .library
                    .iter()
                    .enumerate()
                    .map(|(index, card)| DecisionOption {
                        id: u32::try_from(index).unwrap_or(u32::MAX),
                        label: self
                            .catalog
                            .get(card.definition)
                            .map_or_else(|| "Unknown card".into(), |card| card.name.clone()),
                        card: Some((card.id, card.definition)),
                        ability_text: None,
                        zone: DecisionZone::Library,
                    })
                    .collect();
                self.queue_decision(
                    object.controller,
                    "Choose a card to put into your hand, or fail to find",
                    DecisionVisibility::Private,
                    DecisionPreference::HigherCardValue,
                    // Searching a hidden zone never obliges the searcher to
                    // find (CR 701.19c), so the minimum is zero even with a
                    // full library. Failing to find is not cancelling: the
                    // spell resolved and the search happened, which is why
                    // the shuffle below runs either way.
                    0..=1,
                    false,
                    options,
                    DecisionContinuation::Tutor,
                );
            }
            CardBehavior::Duress => {
                if let Some(Target::Player(victim)) = object.first_target() {
                    let eligible = self.players[victim.index()]
                        .hand
                        .iter()
                        .filter(|card| {
                            self.catalog.get(card.definition).is_some_and(|definition| {
                                !definition.rules.has_type(CardType::Creature)
                                    && !definition.rules.has_type(CardType::Land)
                            })
                        })
                        .cloned()
                        .collect::<Vec<_>>();
                    let options = self.card_decision_options(&eligible, DecisionZone::Hand);
                    // The hand is revealed, so the caster picking from it is
                    // public information rather than a hidden choice.
                    self.queue_decision(
                        object.controller,
                        "Choose a card for them to discard",
                        DecisionVisibility::Public,
                        DecisionPreference::HigherCardValue,
                        1..=1,
                        false,
                        options,
                        DecisionContinuation::Duress {
                            victim,
                            cause: ZoneMoveCause::Effect {
                                controller: object.controller,
                            },
                        },
                    );
                }
            }
            CardBehavior::Mulch => {
                let player = object.controller;
                let revealed = self.take_top_of_library(player, 4);
                let (lands, rest): (Vec<_>, Vec<_>) = revealed.into_iter().partition(|card| {
                    self.catalog
                        .get(card.definition)
                        .is_some_and(|definition| definition.rules.has_type(CardType::Land))
                });
                for card in lands {
                    let (card, _zone_change) = self.zone_change_card(card);
                    self.players[player.index()].hand.push(card);
                }
                self.bury_cards(player, rest);
            }
            CardBehavior::GrislySalvage => {
                let player = object.controller;
                let revealed = self.take_top_of_library(player, 5);
                let eligible = revealed
                    .iter()
                    .filter(|card| {
                        self.catalog.get(card.definition).is_some_and(|definition| {
                            definition.rules.has_type(CardType::Creature)
                                || definition.rules.has_type(CardType::Land)
                        })
                    })
                    .cloned()
                    .collect::<Vec<_>>();
                let options = self.card_decision_options(&eligible, DecisionZone::Library);
                // "You may put ... into your hand": taking nothing is a real
                // choice, so the minimum is zero even when something qualifies.
                self.queue_decision(
                    player,
                    "Put a creature or land card into your hand",
                    DecisionVisibility::Public,
                    DecisionPreference::HigherCardValue,
                    0..=1,
                    false,
                    options,
                    DecisionContinuation::GrislySalvage { player, revealed },
                );
            }
            CardBehavior::HymnToTourach => self.discard_random(
                object.controller.opponent(),
                2,
                ZoneMoveCause::Effect {
                    controller: object.controller,
                },
            ),
            CardBehavior::MindTwist => {
                self.discard_random(
                    object.controller.opponent(),
                    object.x(),
                    ZoneMoveCause::Effect {
                        controller: object.controller,
                    },
                );
            }
            CardBehavior::Balance => self.resolve_balance(object.controller),
            CardBehavior::Regrowth => {
                if let Some(card) = self.players[object.controller.index()].graveyard.pop() {
                    let (card, _zone_change) = self.zone_change_card(card);
                    self.players[object.controller.index()].hand.push(card);
                }
            }
            CardBehavior::Recall => {
                let options = self.card_decision_options(
                    &self.players[object.controller.index()].graveyard,
                    DecisionZone::Graveyard,
                );
                let count = usize::from(object.x()).min(options.len());
                self.queue_decision(
                    object.controller,
                    format!("Return {count} card(s) from your graveyard"),
                    DecisionVisibility::Private,
                    DecisionPreference::HigherCardValue,
                    count..=count,
                    false,
                    options,
                    DecisionContinuation::RecallReturn {
                        player: object.controller,
                    },
                );
            }
            _ => {}
        }
    }

    /// Lifts the top `count` cards off a library, fewer if it is short.
    /// Revealing them is informational only; nothing yet keys off having seen
    /// a card, so the mechanical effect is where they end up.
    fn take_top_of_library(&mut self, player: PlayerId, count: usize) -> Vec<CardInstance> {
        let library = &mut self.players[player.index()].library;
        let taken = count.min(library.len());
        library.drain(..taken).collect()
    }

    /// Sends cards to their owner's graveyard in the order given.
    fn bury_cards(&mut self, player: PlayerId, cards: Vec<CardInstance>) {
        for card in cards {
            let (card, _zone_change) = self.zone_change_card(card);
            self.players[player.index()].graveyard.push(card);
        }
    }

    fn discard_random(&mut self, player: PlayerId, count: u16, cause: ZoneMoveCause) {
        self.rng.shuffle(&mut self.players[player.index()].hand);
        let hand_count = u16::try_from(self.players[player.index()].hand.len()).unwrap_or(u16::MAX);
        let discard_count = count.min(hand_count);
        let discarded = self.players[player.index()]
            .hand
            .iter()
            .rev()
            .take(usize::from(discard_count))
            .map(|card| card.id)
            .collect::<Vec<_>>();
        self.discard_cards_with_cause(player, &discarded, cause);
    }

    fn resolve_balance(&mut self, controller: PlayerId) {
        self.queue_balance_phase(controller, BalancePhase::Lands);
    }

    fn queue_balance_phase(&mut self, controller: PlayerId, phase: BalancePhase) {
        let mut tasks = self.balance_tasks(controller, phase);
        if tasks.is_empty() {
            if let Some(next) = phase.next() {
                self.queue_balance_phase(controller, next);
            }
            return;
        }
        let first = tasks.remove(0);
        self.queue_balance_task(controller, phase, first, tasks);
    }

    fn balance_tasks(&self, controller: PlayerId, phase: BalancePhase) -> Vec<BalanceTask> {
        let mut tasks = Vec::new();
        if phase == BalancePhase::Hands {
            let keep = self.players[0].hand.len().min(self.players[1].hand.len());
            for player in [self.active_player, self.active_player.opponent()] {
                let count = self.players[player.index()].hand.len().saturating_sub(keep);
                if count > 0 {
                    tasks.push(BalanceTask {
                        player,
                        prompt: format!("Choose {count} card(s) to discard to Balance"),
                        zone: DecisionZone::Hand,
                        cards: self.players[player.index()].hand.clone(),
                        count,
                        action: BalanceAction::Discard,
                        cause: ZoneMoveCause::Effect { controller },
                    });
                }
            }
            return tasks;
        }

        let card_type = match phase {
            BalancePhase::Lands => CardType::Land,
            BalancePhase::Creatures => CardType::Creature,
            BalancePhase::Hands => unreachable!("the hand phase returned above"),
        };
        let counts = [self.active_player, self.active_player.opponent()].map(|player| {
            self.battlefield
                .iter()
                .filter(|permanent| {
                    permanent.controller == player
                        && if card_type == CardType::Creature {
                            self.power(permanent).is_some()
                        } else {
                            self.permanent_types(permanent)
                                .is_some_and(|types| types.contains(CardType::Land))
                        }
                })
                .count()
        });
        let keep = counts[0].min(counts[1]);
        for player in [self.active_player, self.active_player.opponent()] {
            let cards = self
                .battlefield
                .iter()
                .filter(|permanent| {
                    permanent.controller == player
                        && if card_type == CardType::Creature {
                            self.power(permanent).is_some()
                        } else {
                            self.permanent_types(permanent)
                                .is_some_and(|types| types.contains(CardType::Land))
                        }
                })
                .map(|permanent| permanent.card.clone())
                .collect::<Vec<_>>();
            let count = cards.len().saturating_sub(keep);
            if count > 0 {
                tasks.push(BalanceTask {
                    player,
                    prompt: format!(
                        "Choose {count} {} to sacrifice to Balance",
                        if card_type == CardType::Land {
                            "land(s)"
                        } else {
                            "creature(s)"
                        }
                    ),
                    zone: DecisionZone::Battlefield,
                    cards,
                    count,
                    action: BalanceAction::Sacrifice,
                    cause: ZoneMoveCause::Effect { controller },
                });
            }
        }
        tasks
    }

    fn resolve_timetwister(&mut self) {
        for player in [PlayerId::One, PlayerId::Two] {
            let hand = std::mem::take(&mut self.players[player.index()].hand);
            let graveyard = std::mem::take(&mut self.players[player.index()].graveyard);
            for card in hand.into_iter().chain(graveyard) {
                let (card, _zone_change) = self.zone_change_card(card);
                self.players[player.index()].library.push(card);
            }
            self.rng.shuffle(&mut self.players[player.index()].library);
        }
        for player in [PlayerId::One, PlayerId::Two] {
            self.draw_cards(player, 7);
        }
    }

    fn resolve_wheel_of_fortune(&mut self, controller: PlayerId) {
        for player in [PlayerId::One, PlayerId::Two] {
            let hand = self.players[player.index()]
                .hand
                .iter()
                .map(|card| card.id)
                .collect::<Vec<_>>();
            self.discard_cards_with_cause(player, &hand, ZoneMoveCause::Effect { controller });
        }
        let can_draw = [
            self.players[0].library.len() >= 7,
            self.players[1].library.len() >= 7,
        ];
        match can_draw {
            [false, false] => {
                self.finish(GameResult::Draw);
                return;
            }
            [false, true] => {
                self.finish(GameResult::Winner {
                    winner: PlayerId::Two,
                    reason: WinReason::OpponentTriedToDrawFromEmptyLibrary,
                });
                return;
            }
            [true, false] => {
                self.finish(GameResult::Winner {
                    winner: PlayerId::One,
                    reason: WinReason::OpponentTriedToDrawFromEmptyLibrary,
                });
                return;
            }
            [true, true] => {}
        }
        for player in [PlayerId::One, PlayerId::Two] {
            for _ in 0..7 {
                let _ = self.draw_card(player);
            }
        }
    }

    fn damage_target(&mut self, target: Option<Target>, amount: u16) {
        self.damage_target_from(None, target, amount);
    }

    fn damage_target_from(
        &mut self,
        source: Option<GameObjectId>,
        target: Option<Target>,
        amount: u16,
    ) {
        let source_colors = source.map_or([false; 5], |source| self.object_colors(source));
        let lifelink_controller = source.and_then(|source| {
            self.source_controller_with_keyword(source, KeywordAbility::Lifelink)
        });
        let has_deathtouch = source.is_some_and(|source| {
            self.source_controller_with_keyword(source, KeywordAbility::Deathtouch)
                .is_some()
        });
        let dealt_damage = match target {
            Some(Target::Player(player)) => {
                self.deal_damage(player, amount);
                true
            }
            Some(Target::Permanent(id)) => {
                if let Some(index) = self
                    .battlefield
                    .iter()
                    .position(|permanent| permanent.card.id == id)
                {
                    if self.is_protected_from_colors(&self.battlefield[index], source_colors) {
                        return;
                    }
                    if self
                        .permanent_types(&self.battlefield[index])
                        .is_some_and(|types| types.contains(CardType::Planeswalker))
                    {
                        let loyalty_loss = i16::try_from(amount).unwrap_or(i16::MAX);
                        if let Some(loyalty) = &mut self.battlefield[index].loyalty {
                            *loyalty = loyalty.saturating_sub(loyalty_loss);
                            true
                        } else {
                            false
                        }
                    } else {
                        let permanent = &mut self.battlefield[index];
                        permanent.damage = permanent.damage.saturating_add(amount);
                        if amount > 0 {
                            permanent.deathtouch_damage |= has_deathtouch;
                            if let Some(source) = source
                                && !permanent.damage_sources.contains(&source)
                            {
                                permanent.damage_sources.push(source);
                            }
                        }
                        true
                    }
                } else {
                    false
                }
            }
            Some(Target::Card(_) | Target::Spell(_)) | None => false,
        };
        if dealt_damage
            && amount > 0
            && let Some(controller) = lifelink_controller
        {
            self.gain_life(controller, amount);
        }
        if dealt_damage
            && amount > 0
            && let Some(Target::Permanent(id)) = target
            && let Some(damaged) = self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == id)
        {
            let event = CommittedTriggerEvent::DamageDealt {
                object: self.trigger_event_object(damaged),
                amount,
            };
            self.capture_battlefield_triggers(&event);
        }
    }

    fn damage_targets(&self) -> Vec<Target> {
        let mut targets = vec![Target::Player(PlayerId::One), Target::Player(PlayerId::Two)];
        targets.extend(
            self.battlefield
                .iter()
                .filter(|permanent| {
                    self.power(permanent).is_some()
                        || self
                            .permanent_types(permanent)
                            .is_some_and(|types| types.contains(CardType::Planeswalker))
                })
                .map(|permanent| Target::Permanent(permanent.card.id)),
        );
        targets
    }

    fn count_behavior(&self, behavior: CardBehavior) -> u16 {
        let blood_moon_active = self.blood_moon_active();
        u16::try_from(
            self.battlefield
                .iter()
                .filter(|permanent| {
                    self.effective_behavior_with_blood_moon(permanent, blood_moon_active)
                        == Some(behavior)
                })
                .count(),
        )
        .unwrap_or(u16::MAX)
    }

    fn blood_moon_active(&self) -> bool {
        self.battlefield
            .iter()
            .any(|permanent| self.copiable_behavior(permanent) == Some(CardBehavior::BloodMoon))
    }

    fn is_nonbasic_land(&self, permanent: &Permanent) -> bool {
        self.permanent_types(permanent)
            .is_some_and(|types| types.contains(CardType::Land))
            && self
                .effective_rules(permanent)
                .is_some_and(|rules| !rules.has_supertype(CardSupertype::Basic))
    }

    fn is_artifact_permanent(&self, permanent: &Permanent) -> bool {
        self.permanent_types(permanent)
            .is_some_and(|types| types.contains(CardType::Artifact))
    }

    /// Resolves the printed rules currently supplying baseline permanent
    /// characteristics. A copy's copiable rules take precedence over the
    /// physical card's presented part.
    fn effective_rules<'a>(&'a self, permanent: &Permanent) -> Option<&'a CardRules> {
        let (definition, part) = Self::effective_rules_source(permanent);
        self.catalog
            .get(definition)?
            .part(part)
            .map(|part| &part.rules)
    }

    fn effective_rules_source(permanent: &Permanent) -> (CardDefinitionId, CardPartId) {
        permanent
            .copy_effect
            .as_ref()
            .map_or((permanent.card.definition, permanent.presented), |copy| {
                copy.base
            })
    }

    fn land_type_operations(
        &self,
        permanent: &Permanent,
    ) -> Vec<(GameObjectId, LandTypeOperation)> {
        let mut operations = Vec::new();
        let target_is_nonbasic = self.is_nonbasic_land(permanent);
        let mut blood_moon_active = None;
        for source in &self.battlefield {
            if target_is_nonbasic && self.copiable_behavior(source) == Some(CardBehavior::BloodMoon)
            {
                operations.push((
                    source.card.id,
                    LandTypeOperation::SetTo(BasicLandType::Mountain),
                ));
            }

            // The only modeled additive operation applies to the permanent
            // this Aura is attached to. Avoid walking every unrelated
            // permanent's static rules for every characteristic query.
            if source.attached_to != Some(permanent.card.id) {
                continue;
            }
            if self.is_nonbasic_land(source)
                && *blood_moon_active.get_or_insert_with(|| self.blood_moon_active())
            {
                continue;
            }
            let Some(rules) = self.effective_rules(source) else {
                continue;
            };
            for ability in rules
                .ability_clauses()
                .iter()
                .copied()
                .chain(
                    source
                        .copy_effect
                        .iter()
                        .flat_map(|copy| copy.added_abilities.iter())
                        .map(|ability| ability.definition),
                )
                .filter(|ability| {
                    ability.implementation.is_executable()
                        && matches!(ability.definition, DeclarativeAbilityDef::Static(_))
                })
            {
                Self::collect_land_type_operations(
                    ability.effect,
                    source,
                    permanent,
                    &mut operations,
                );
            }
        }
        operations.sort_by_key(|(source, _)| *source);
        operations
    }

    fn collect_land_type_operations(
        effect: EffectDef,
        source: &Permanent,
        affected: &Permanent,
        operations: &mut Vec<(GameObjectId, LandTypeOperation)>,
    ) {
        match effect {
            EffectDef::Sequence(effects) => {
                for effect in effects {
                    Self::collect_land_type_operations(*effect, source, affected, operations);
                }
            }
            EffectDef::Apply {
                recipient: EffectRecipientDef::AttachedPermanent,
                effect: AppliedEffectDef::AddLandTypes(types),
                duration:
                    EffectDurationDef::WhileSourceRemainsInZone
                    | EffectDurationDef::UntilSourceLeavesZone,
            } if source.attached_to == Some(affected.card.id) => {
                operations.push((source.card.id, LandTypeOperation::Add(types)));
            }
            _ => {}
        }
    }

    fn copiable_characteristics(permanent: &Permanent) -> CopiableCharacteristics {
        permanent
            .copy_effect
            .clone()
            .unwrap_or_else(|| CopiableCharacteristics {
                base: (permanent.card.definition, permanent.presented),
                added_types: CardTypeSet::empty(),
                added_abilities: Vec::new(),
            })
    }

    fn copiable_behavior(&self, permanent: &Permanent) -> Option<CardBehavior> {
        self.effective_rules(permanent)
            .and_then(CardRules::special_behavior)
    }

    fn trigger_event_object(&self, permanent: &Permanent) -> TriggerEventObject {
        let rules = self
            .effective_rules(permanent)
            .expect("a battlefield object has effective rules");
        TriggerEventObject {
            id: permanent.card.id,
            types: self
                .permanent_types(permanent)
                .expect("a battlefield object has effective types"),
            controller: permanent.controller,
            attacking_or_blocking: permanent.attacking || permanent.blocking.is_some(),
            colors: rules.colors(),
            subtypes: self.effective_subtypes(permanent),
            mana_value: self.permanent_mana_value(permanent),
            power: self.power_ignoring_static_effects(permanent),
            keywords: Self::keyword_mask_ignoring_static_effects(permanent, rules),
            supertypes: {
                let mut supertypes = [false; CardSupertype::COUNT];
                for supertype in CardSupertype::ALL {
                    supertypes[supertype.index()] = rules.has_supertype(supertype);
                }
                supertypes
            },
            attacking: permanent.attacking,
        }
    }

    /// The keywords an object presents without consulting static effects. See
    /// [`TriggerEventObject::keywords`] for why granted ones stay out.
    fn keyword_mask_ignoring_static_effects(permanent: &Permanent, rules: &CardRules) -> u32 {
        let mut mask = 0;
        let mut set = |keyword: KeywordAbility| {
            if let Some(index) = keyword.simple_index() {
                mask |= 1 << index;
            }
        };
        for keyword in &permanent.temporary_keywords {
            set(*keyword);
        }
        for ability in rules.ability_clauses() {
            if ability.implementation.is_executable()
                && let DeclarativeAbilityDef::Keyword(keyword) = ability.definition
            {
                set(keyword);
            }
        }
        mask
    }

    fn battlefield_exit_snapshot(&self, permanent: &Permanent) -> BattlefieldExitSnapshot {
        let abilities = self.effective_abilities(permanent);
        let mut keywords = permanent.temporary_keywords.clone();
        for effective in &abilities {
            if effective.ability.implementation.is_executable()
                && let DeclarativeAbilityDef::Keyword(ability) = effective.ability.definition
                && !keywords.contains(&ability)
            {
                keywords.push(ability);
            }
        }
        BattlefieldExitSnapshot {
            object: self.trigger_event_object(permanent),
            abilities,
            last_known: PermanentLastKnownInformation {
                power: self.power_ignoring_static_effects(permanent),
                toughness: self.toughness(permanent),
                keywords,
            },
        }
    }

    /// Ordered subtypes after the continuous effects currently modeled by the
    /// engine. Layer-3 text changes apply to the copied/printed line first;
    /// timestamp-ordered layer-4 Set/Add operations then model Blood Moon and
    /// Aura-granted basic land types. Nonland subtypes such as Dryad survive.
    fn effective_subtypes(&self, permanent: &Permanent) -> Cow<'static, [&'static str]> {
        fn is_land_subtype(subtype: &str) -> bool {
            BasicLandType::from_subtype(subtype).is_some() || subtype == "Gate"
        }

        let Some(rules) = self.effective_rules(permanent) else {
            return Cow::Borrowed(&[]);
        };
        let operations = self.land_type_operations(permanent);
        if permanent.text_changes.is_empty() && operations.is_empty() {
            return Cow::Borrowed(rules.subtypes());
        }

        let mut subtypes = rules.subtypes().to_vec();
        for change in &permanent.text_changes {
            for subtype in &mut subtypes {
                if BasicLandType::from_subtype(subtype) == Some(change.from) {
                    *subtype = change.to.subtype();
                }
            }
        }

        let mut seen = [false; BasicLandType::ALL.len()];
        subtypes.retain(|subtype| {
            let Some(land_type) = BasicLandType::from_subtype(subtype) else {
                return true;
            };
            let keep = !seen[land_type.index()];
            seen[land_type.index()] = true;
            keep
        });

        for (_, operation) in operations {
            match operation {
                LandTypeOperation::SetTo(land_type) => {
                    let insertion = subtypes
                        .iter()
                        .position(|subtype| is_land_subtype(subtype))
                        .unwrap_or(0);
                    subtypes.retain(|subtype| !is_land_subtype(subtype));
                    subtypes.insert(insertion.min(subtypes.len()), land_type.subtype());
                }
                LandTypeOperation::Add(types) => {
                    let mut insertion = subtypes
                        .iter()
                        .position(|subtype| !is_land_subtype(subtype))
                        .unwrap_or(subtypes.len());
                    for land_type in types {
                        if subtypes
                            .iter()
                            .any(|subtype| BasicLandType::from_subtype(subtype) == Some(*land_type))
                        {
                            continue;
                        }
                        subtypes.insert(insertion, land_type.subtype());
                        insertion += 1;
                    }
                }
            }
        }
        Cow::Owned(subtypes)
    }

    /// Basic land subtypes in effective type-line order, with duplicate types
    /// collapsed before the rules grant one intrinsic ability for each type.
    fn visit_effective_basic_land_types(
        &self,
        permanent: &Permanent,
        mut visitor: impl FnMut(BasicLandType) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        if !self
            .permanent_types(permanent)
            .is_some_and(|types| types.contains(CardType::Land))
        {
            return ControlFlow::Continue(());
        }

        let mut present = [false; BasicLandType::ALL.len()];
        for subtype in self.effective_subtypes(permanent).iter() {
            let Some(land_type) = BasicLandType::from_subtype(subtype) else {
                continue;
            };
            if present[land_type.index()] {
                continue;
            }
            present[land_type.index()] = true;
            if visitor(land_type).is_break() {
                return ControlFlow::Break(());
            }
        }
        ControlFlow::Continue(())
    }

    /// Basic land subtypes after the continuous effects currently modeled by
    /// the engine.
    fn effective_land_types(&self, permanent: &Permanent) -> [bool; 5] {
        let mut types = [false; 5];
        let result = self.visit_effective_basic_land_types(permanent, |land_type| {
            types[land_type.index()] = true;
            ControlFlow::Continue(())
        });
        debug_assert!(result.is_continue());
        types
    }

    /// Abilities the object currently has. Surviving printed abilities retain
    /// printed order, followed by intrinsic basic-land-type abilities in
    /// effective type-line order and then abilities granted by other objects.
    fn visit_effective_abilities(
        &self,
        permanent: &Permanent,
        mut visitor: impl FnMut(EffectiveAbility) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        let blood_moon_applies = self.is_nonbasic_land(permanent) && self.blood_moon_active();
        if !blood_moon_applies && let Some(rules) = self.effective_rules(permanent) {
            let (definition, part) = Self::effective_rules_source(permanent);
            for attached in rules.indexed_abilities() {
                if visitor(EffectiveAbility {
                    origin: AbilityOrigin::Printed {
                        definition,
                        part,
                        ability: attached.id,
                    },
                    ability: attached.definition,
                })
                .is_break()
                {
                    return ControlFlow::Break(());
                }
            }
            if let Some(copy) = &permanent.copy_effect {
                for added in &copy.added_abilities {
                    if visitor(EffectiveAbility {
                        origin: added.origin,
                        ability: added.definition,
                    })
                    .is_break()
                    {
                        return ControlFlow::Break(());
                    }
                }
            }
        }

        if self
            .visit_effective_basic_land_types(permanent, |land_type| {
                visitor(EffectiveAbility {
                    origin: AbilityOrigin::IntrinsicBasicLand(land_type),
                    ability: abilities::tap_for(land_type.mana_color()),
                })
            })
            .is_break()
        {
            return ControlFlow::Break(());
        }
        self.visit_static_applied_effects(permanent, |applied| match applied.effect {
            AppliedEffectDef::GrantAbility(ability) => visitor(EffectiveAbility {
                origin: AbilityOrigin::Granted {
                    source: applied.source,
                    source_definition: applied.source_definition,
                    source_part: applied.source_part,
                    source_ability: applied.source_ability,
                    grant: applied
                        .grant
                        .expect("a granted ability has a structural grant identity"),
                },
                ability: *ability,
            }),
            AppliedEffectDef::CannotBeCountered
            | AppliedEffectDef::CannotBeBlockedBy(_)
            | AppliedEffectDef::AddLandTypes(_)
            | AppliedEffectDef::Composite(_)
            | AppliedEffectDef::ModifyPowerToughness { .. }
            | AppliedEffectDef::Special(_) => ControlFlow::Continue(()),
        })
    }

    fn for_each_effective_ability(
        &self,
        permanent: &Permanent,
        mut visitor: impl FnMut(EffectiveAbility),
    ) {
        let result = self.visit_effective_abilities(permanent, |effective| {
            visitor(effective);
            ControlFlow::Continue(())
        });
        debug_assert!(result.is_continue());
    }

    fn find_effective_ability(
        &self,
        permanent: &Permanent,
        mut predicate: impl FnMut(EffectiveAbility) -> bool,
    ) -> Option<EffectiveAbility> {
        let mut found = None;
        let _ = self.visit_effective_abilities(permanent, |effective| {
            if predicate(effective) {
                found = Some(effective);
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        });
        found
    }

    fn effective_abilities(&self, permanent: &Permanent) -> Vec<EffectiveAbility> {
        let mut abilities = Vec::new();
        self.for_each_effective_ability(permanent, |effective| abilities.push(effective));
        abilities
    }

    fn visit_static_applied_effects(
        &self,
        affected: &Permanent,
        mut visitor: impl FnMut(StaticAppliedEffect) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        let mut blood_moon_active = None;
        for source in &self.battlefield {
            let Some(rules) = self.effective_rules(source) else {
                continue;
            };
            let (source_definition, source_part) = Self::effective_rules_source(source);
            let supplies_static_effect = rules.ability_clauses().iter().any(|ability| {
                ability.implementation.is_executable()
                    && matches!(ability.definition, DeclarativeAbilityDef::Static(_))
            });
            if !supplies_static_effect {
                continue;
            }
            if rules.has_type(CardType::Land)
                && !rules.has_supertype(CardSupertype::Basic)
                && *blood_moon_active.get_or_insert_with(|| self.blood_moon_active())
            {
                continue;
            }
            for attached in rules.indexed_abilities() {
                if !attached.definition.implementation.is_executable()
                    || !matches!(
                        attached.definition.definition,
                        DeclarativeAbilityDef::Static(_)
                    )
                {
                    continue;
                }
                let mut traversal = StaticEffectTraversal {
                    source,
                    source_definition,
                    source_part,
                    source_ability: attached.id,
                    affected,
                    next_grant: 0,
                };
                if self
                    .visit_static_effect(attached.definition.effect, &mut traversal, &mut visitor)
                    .is_break()
                {
                    return ControlFlow::Break(());
                }
            }
        }
        ControlFlow::Continue(())
    }

    fn visit_static_effect(
        &self,
        effect: EffectDef,
        traversal: &mut StaticEffectTraversal<'_>,
        visitor: &mut impl FnMut(StaticAppliedEffect) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        match effect {
            EffectDef::Sequence(effects) => {
                for effect in effects {
                    if self
                        .visit_static_effect(*effect, traversal, visitor)
                        .is_break()
                    {
                        return ControlFlow::Break(());
                    }
                }
                ControlFlow::Continue(())
            }
            EffectDef::Apply {
                recipient,
                effect,
                duration,
            } => {
                // Traverse the whole applied-effect structure even when this
                // recipient does not match. Grant IDs identify structural
                // grant sites, so later grants must not be renumbered by
                // which permanent happens to be queried.
                let include_effect = matches!(
                    duration,
                    EffectDurationDef::WhileSourceRemainsInZone
                        | EffectDurationDef::UntilSourceLeavesZone
                ) && self.static_recipient_matches(
                    recipient,
                    traversal.source,
                    traversal.affected,
                );
                Self::visit_static_applied_effect_components(
                    effect,
                    traversal,
                    include_effect,
                    visitor,
                )
            }
            _ => ControlFlow::Continue(()),
        }
    }

    fn visit_static_applied_effect_components(
        effect: AppliedEffectDef,
        traversal: &mut StaticEffectTraversal<'_>,
        include_effect: bool,
        visitor: &mut impl FnMut(StaticAppliedEffect) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        match effect {
            AppliedEffectDef::Composite(effects) => {
                for effect in effects {
                    if Self::visit_static_applied_effect_components(
                        *effect,
                        traversal,
                        include_effect,
                        visitor,
                    )
                    .is_break()
                    {
                        return ControlFlow::Break(());
                    }
                }
                ControlFlow::Continue(())
            }
            AppliedEffectDef::CannotBeCountered
            | AppliedEffectDef::CannotBeBlockedBy(_)
            | AppliedEffectDef::AddLandTypes(_)
            | AppliedEffectDef::ModifyPowerToughness { .. }
            | AppliedEffectDef::GrantAbility(_)
            | AppliedEffectDef::Special(_) => {
                let grant = if matches!(effect, AppliedEffectDef::GrantAbility(_)) {
                    let grant = GrantId::from_index(traversal.next_grant)
                        .expect("one static ability contains at most 256 grant sites");
                    traversal.next_grant += 1;
                    Some(grant)
                } else {
                    None
                };
                if include_effect {
                    visitor(StaticAppliedEffect {
                        source: traversal.source.card.id,
                        source_definition: traversal.source_definition,
                        source_part: traversal.source_part,
                        source_ability: traversal.source_ability,
                        grant,
                        effect,
                    })
                } else {
                    ControlFlow::Continue(())
                }
            }
        }
    }

    /// Whether this permanent has the shared Aura attachment spell effect.
    fn is_aura_permanent(&self, permanent: &Permanent) -> bool {
        self.effective_rules(permanent).is_some_and(|rules| {
            rules.ability_clauses().iter().any(|ability| {
                ability.implementation.is_executable()
                    && matches!(ability.effect, EffectDef::Attach { .. })
            })
        })
    }

    /// Whether an Aura may stay attached to `host`: the host has to still be
    /// on the battlefield and still satisfy what the Aura enchants.
    fn is_legal_aura_host(&self, aura: &Permanent, host: GameObjectId) -> bool {
        let Some(host) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == host)
        else {
            return false;
        };
        let Some(rules) = self.effective_rules(aura) else {
            return false;
        };
        let aura_colors = rules.colors();
        let mut target_slots = rules
            .ability_clauses()
            .iter()
            .filter(|ability| {
                ability.implementation.is_executable()
                    && matches!(ability.effect, EffectDef::Attach { .. })
            })
            .flat_map(|ability| match ability.definition {
                DeclarativeAbilityDef::Spell(spell) => spell.targets(),
                _ => &[],
            })
            .peekable();
        if target_slots.peek().is_none() {
            return false;
        }
        target_slots.all(|slot| match slot.predicate {
            AbilityTargetPredicate::Object {
                object,
                zones,
                controller,
                owner,
            } => {
                zones.contains(&ZoneKind::Battlefield)
                        && controller.is_none_or(|relation| {
                            self.player_relation_matches(
                                host.controller,
                                relation,
                                aura.controller,
                                TriggerContext::empty(),
                            )
                        })
                        && owner.is_none_or(|relation| {
                            self.player_relation_matches(
                                host.card.owner,
                                relation,
                                aura.controller,
                                TriggerContext::empty(),
                            )
                        })
                        && self.trigger_object_matches(
                            object,
                            &self.trigger_event_object(host),
                            aura.card.id,
                            false,
                        )
                        // Hexproof only constrains targeting. Protection also
                        // makes an existing attachment illegal.
                        && !self.is_protected_from_colors(host, aura_colors)
            }
            AbilityTargetPredicate::AnyTarget | AbilityTargetPredicate::Player(_) => false,
        })
    }

    /// The permanent an Aura spell targeted, read off its own spell clause.
    fn aura_host_for(&self, object: &StackObject) -> Option<GameObjectId> {
        let definition = self.catalog.get(object.card.definition)?;
        let signature = object.signature.as_ref()?;
        let option = definition.play_option(signature.play_option())?;
        let (_, ability) = Self::spell_ability(definition, option)?;
        if !matches!(ability.effect, EffectDef::Attach { .. }) {
            return None;
        }
        signature
            .targets()
            .iter()
            .flat_map(TargetSelection::targets)
            .find_map(|target| match target {
                Target::Permanent(id) => Some(*id),
                _ => None,
            })
    }

    /// Whether a definition is a token rather than a printed card.
    fn is_token(&self, definition: CardDefinitionId) -> bool {
        self.catalog
            .get(definition)
            .is_some_and(|card| card.debut_set == CardSet::Token)
    }

    /// Puts one token onto the battlefield under `controller`.
    ///
    /// A token is a real permanent built from a catalog definition that no
    /// format allows, so it can be looked up and rendered like any other card
    /// while never being deck-legal.
    fn create_token(&mut self, controller: PlayerId, token: CardDefinitionId) {
        let Some(definition) = self.catalog.get(token) else {
            return;
        };
        let presented = definition.primary_part_id();
        // A token has no physical card behind it, which is exactly what an
        // unbacked object is.
        let card = self.unbacked_object(token, controller, CharacteristicSource::Card(token));
        let permanent = Permanent::entering(
            card,
            presented,
            controller,
            self.turns_started[controller.index()],
        );
        self.battlefield.push(permanent);
        let entered = self
            .battlefield
            .last()
            .expect("the token just created is on the battlefield");
        let entered_event = self.trigger_event_object(entered);
        self.capture_battlefield_triggers(&CommittedTriggerEvent::ZoneChanged {
            object: entered_event,
            from: ZoneKind::Stack,
            to: ZoneKind::Battlefield,
        });
    }

    /// What an Aura is attached to, if it is on the battlefield and attached.
    fn attached_host(&self, aura: GameObjectId) -> Option<GameObjectId> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == aura)
            .and_then(|permanent| permanent.attached_to)
    }

    fn static_recipient_matches(
        &self,
        recipient: EffectRecipientDef,
        source: &Permanent,
        affected: &Permanent,
    ) -> bool {
        match recipient {
            EffectRecipientDef::Source => source.card.id == affected.card.id,
            EffectRecipientDef::AttachedPermanent => source.attached_to == Some(affected.card.id),

            EffectRecipientDef::MatchingObjects {
                object,
                zones,
                controller,
            } => {
                zones.contains(&ZoneKind::Battlefield)
                    && self.player_relation_matches(
                        affected.controller,
                        controller,
                        source.controller,
                        TriggerContext::empty(),
                    )
                    && self.trigger_object_matches(
                        object,
                        &self.trigger_event_object(affected),
                        source.card.id,
                        false,
                    )
            }
            // None of these name a permanent a static effect could apply to;
            // a static effect has no chosen target either.
            EffectRecipientDef::Controller
            | EffectRecipientDef::Opponent
            | EffectRecipientDef::Target(_)
            | EffectRecipientDef::ObjectsSharingNameWithTarget(_)
            | EffectRecipientDef::TriggeringObject
            | EffectRecipientDef::ControllerOfTriggeringObject
            | EffectRecipientDef::EventPlayer => false,
        }
    }

    /// Resolves an ordinary activated clause by its printed order. Legacy
    /// aggregate definitions fall back to their historic primary identity;
    /// migrated multi-ability cards retain the exact clause chosen by the
    /// action (for example, Factory's animate and pump abilities).
    #[cfg(test)]
    fn activated_ability_origin(&self, permanent: &Permanent, index: usize) -> AbilityOrigin {
        let mut activated_index = 0;
        self.find_effective_ability(permanent, |effective| {
            if !effective.ability.implementation.is_executable()
                || !matches!(
                    effective.ability.definition,
                    DeclarativeAbilityDef::Activated(_)
                )
            {
                return false;
            }
            let matches = activated_index == index;
            activated_index += 1;
            matches
        })
        .map_or(
            AbilityOrigin::Printed {
                definition: Self::effective_rules_source(permanent).0,
                part: Self::effective_rules_source(permanent).1,
                ability: AbilityId::PRIMARY,
            },
            |effective| effective.origin,
        )
    }

    fn permanent_types(&self, permanent: &Permanent) -> Option<CardTypeSet> {
        let mut types = self.effective_rules(permanent)?.types();
        if let Some(copy) = &permanent.copy_effect {
            types = types.union(copy.added_types);
        }
        if permanent.factory_animated {
            types = types.with(CardType::Artifact).with(CardType::Creature);
        }
        Some(types)
    }

    fn effective_behavior(&self, permanent: &Permanent) -> Option<CardBehavior> {
        if self.is_nonbasic_land(permanent) && self.blood_moon_active() {
            None
        } else {
            self.copiable_behavior(permanent)
        }
    }

    fn effective_behavior_with_blood_moon(
        &self,
        permanent: &Permanent,
        blood_moon_active: bool,
    ) -> Option<CardBehavior> {
        if blood_moon_active && self.is_nonbasic_land(permanent) {
            None
        } else {
            self.copiable_behavior(permanent)
        }
    }

    fn is_protected_from_colors(&self, permanent: &Permanent, source_colors: [bool; 5]) -> bool {
        [
            ManaColor::White,
            ManaColor::Blue,
            ManaColor::Black,
            ManaColor::Red,
            ManaColor::Green,
        ]
        .into_iter()
        .any(|color| {
            source_colors[color
                .color_index()
                .expect("the iteration contains only colors")]
                && self.permanent_has_executable_keyword(
                    permanent,
                    KeywordAbility::ProtectionFrom(color),
                )
        })
    }

    fn permanent_can_be_targeted_by(
        &self,
        permanent: &Permanent,
        controller: PlayerId,
        source: GameObjectId,
    ) -> bool {
        !(self.is_protected_from_colors(permanent, self.object_colors(source))
            || permanent.controller != controller
                && self.permanent_has_executable_keyword(permanent, KeywordAbility::Hexproof))
    }

    fn object_colors(&self, object: GameObjectId) -> [bool; 5] {
        if let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == object)
        {
            return self
                .effective_rules(permanent)
                .map_or([false; 5], CardRules::colors);
        }
        if let Some(stack) = self.stack.iter().find(|stack| stack.id == object) {
            return self
                .stack_trigger_event_object(stack)
                .map_or([false; 5], |event| event.colors);
        }
        if let Some(retired) = self.retired_objects.get(&object) {
            return match retired {
                RetiredObject::Permanent { permanent, .. } => self
                    .effective_rules(permanent)
                    .map_or([false; 5], CardRules::colors),
                RetiredObject::Stack(stack) => self
                    .stack_trigger_event_object(stack)
                    .map_or([false; 5], |event| event.colors),
                RetiredObject::Card(card) => self
                    .catalog
                    .get(card.definition)
                    .map_or([false; 5], |definition| definition.rules.colors()),
            };
        }
        self.players
            .iter()
            .flat_map(|player| player.hand.iter())
            .find(|card| card.id == object)
            .and_then(|card| self.catalog.get(card.definition))
            .map_or([false; 5], |definition| definition.rules.colors())
    }

    fn combat_is_protected(&self, blocker: &Permanent, attacker: &Permanent) -> bool {
        let blocker_colors = self
            .effective_rules(blocker)
            .map_or([false; 5], CardRules::colors);
        self.is_protected_from_colors(attacker, blocker_colors)
    }

    fn mana_ability_activations(&self, permanent: &Permanent) -> Vec<ManaAbilityActivation> {
        let mut activations = Vec::new();
        self.for_each_effective_ability(permanent, |effective| {
            let ability = effective.ability;
            if !ability.implementation.is_executable() {
                return;
            }
            let DeclarativeAbilityDef::ActivatedMana(definition) = ability.definition else {
                return;
            };
            let taps_source = definition.costs.contains(&AbilityCostDef::TapSource);
            let sacrifices_source = definition.costs.contains(&AbilityCostDef::SacrificeSource);
            if !definition.source_zones.contains(&ZoneKind::Battlefield)
                || !(taps_source || sacrifices_source)
                || (taps_source && (permanent.tapped || !self.can_use_tap_ability(permanent)))
                || definition.costs.iter().any(|cost| {
                    !matches!(
                        cost,
                        AbilityCostDef::TapSource
                            | AbilityCostDef::SacrificeSource
                            | AbilityCostDef::PayLife(_)
                    )
                })
                || definition.costs.iter().any(|cost| {
                    matches!(cost, AbilityCostDef::PayLife(amount) if self.players[permanent.controller.index()].life < i16::try_from(*amount).unwrap_or(i16::MAX))
                })
            {
                return;
            }
            match ability.effect {
                EffectDef::AddMana(effect) => {
                    let mut add_activation = |color| {
                        activations.push(ManaAbilityActivation {
                            source: permanent.card.id,
                            ability: effective.origin,
                            color,
                            costs: definition.costs,
                            effect,
                        });
                    };
                    match effect.mana {
                        ManaSelectionDef::One(color) => {
                            add_activation(color);
                        }
                        ManaSelectionDef::Choice(colors) => {
                            for color in colors {
                                add_activation(*color);
                            }
                        }
                    }
                }
                EffectDef::Special(_)
                    if self.effective_behavior(permanent) == Some(CardBehavior::FellwarStone) =>
                {
                    let mut visiting = Vec::new();
                    let colors = self.fellwar_stone_colors(permanent, &mut visiting);
                    activations.extend(colors.into_iter().map(|color| ManaAbilityActivation {
                        source: permanent.card.id,
                        ability: effective.origin,
                        color,
                        costs: definition.costs,
                        effect: AddManaEffectDef::one(color),
                    }));
                }
                EffectDef::None
                | EffectDef::Sequence(_)
                | EffectDef::DealDamage { .. }
                | EffectDef::GainLife { .. }
                | EffectDef::DrawCards { .. }
                | EffectDef::DiscardCards { .. }
                | EffectDef::LoseLife { .. }
                | EffectDef::Tap { .. }
                | EffectDef::Untap { .. }
                | EffectDef::Attach { .. }
                | EffectDef::CreateToken { .. }
                | EffectDef::Destroy { .. }
                | EffectDef::Sacrifice { .. }
                | EffectDef::SacrificeOfChoice { .. }
                | EffectDef::Counter { .. }
                | EffectDef::CounterUnlessPaid { .. }
                | EffectDef::AddCounters { .. }
                | EffectDef::ChangeTextBasicLandType { .. }
                | EffectDef::BecomeCopyOf { .. }
                | EffectDef::OptionalManaPayment { .. }
                | EffectDef::May(_)
                | EffectDef::EntersTapped
                | EffectDef::CannotBeForcedToSacrifice
                | EffectDef::GrantFlashToNextSorcery
                | EffectDef::ExileLinkedToSource { .. }
                | EffectDef::ReturnLinkedExiles { .. }
            | EffectDef::MakeUnblockableThisTurn { .. }
            | EffectDef::AtNextStep { .. }
                | EffectDef::ReduceGenericCostBy(_)
                | EffectDef::MultiplyEventAmount(_)
                | EffectDef::MoveToZone { .. }
                | EffectDef::ChooseCreatureType { .. }
                | EffectDef::Apply { .. }
                | EffectDef::Special(_) => {}
            }
        });
        activations
    }

    fn fellwar_stone_colors(
        &self,
        permanent: &Permanent,
        visiting: &mut Vec<GameObjectId>,
    ) -> Vec<ManaColor> {
        if visiting.contains(&permanent.card.id) {
            return Vec::new();
        }
        visiting.push(permanent.card.id);
        let mut colors = self
            .battlefield
            .iter()
            .filter(|candidate| {
                candidate.controller == permanent.controller.opponent()
                    && self
                        .permanent_types(candidate)
                        .is_some_and(|types| types.contains(CardType::Land))
            })
            .flat_map(|candidate| self.colors_permanent_could_produce(candidate, visiting))
            .filter(|color| *color != ManaColor::Colorless)
            .collect::<Vec<_>>();
        visiting.pop();
        colors.sort_unstable();
        colors.dedup();
        colors
    }

    fn colors_permanent_could_produce(
        &self,
        permanent: &Permanent,
        visiting: &mut Vec<GameObjectId>,
    ) -> Vec<ManaColor> {
        if visiting.contains(&permanent.card.id) {
            return Vec::new();
        }
        visiting.push(permanent.card.id);
        let mut colors = Vec::new();
        self.for_each_effective_ability(permanent, |effective| {
            if !effective.ability.implementation.is_executable()
                || !matches!(
                    effective.ability.definition,
                    DeclarativeAbilityDef::ActivatedMana(_)
                )
            {
                return;
            }
            match effective.ability.effect {
                EffectDef::AddMana(effect) => match effect.mana {
                    ManaSelectionDef::One(kind) => colors.push(kind),
                    ManaSelectionDef::Choice(kinds) => {
                        colors.extend_from_slice(kinds);
                    }
                },
                EffectDef::Special(_)
                    if self.effective_behavior(permanent) == Some(CardBehavior::FellwarStone) =>
                {
                    colors.extend(self.fellwar_stone_colors(permanent, visiting));
                }
                EffectDef::None
                | EffectDef::Sequence(_)
                | EffectDef::DealDamage { .. }
                | EffectDef::GainLife { .. }
                | EffectDef::DrawCards { .. }
                | EffectDef::DiscardCards { .. }
                | EffectDef::LoseLife { .. }
                | EffectDef::Tap { .. }
                | EffectDef::Untap { .. }
                | EffectDef::Attach { .. }
                | EffectDef::CreateToken { .. }
                | EffectDef::Destroy { .. }
                | EffectDef::Sacrifice { .. }
                | EffectDef::SacrificeOfChoice { .. }
                | EffectDef::Counter { .. }
                | EffectDef::CounterUnlessPaid { .. }
                | EffectDef::AddCounters { .. }
                | EffectDef::ChangeTextBasicLandType { .. }
                | EffectDef::BecomeCopyOf { .. }
                | EffectDef::OptionalManaPayment { .. }
                | EffectDef::May(_)
                | EffectDef::EntersTapped
                | EffectDef::CannotBeForcedToSacrifice
                | EffectDef::GrantFlashToNextSorcery
                | EffectDef::ExileLinkedToSource { .. }
                | EffectDef::ReturnLinkedExiles { .. }
                | EffectDef::MakeUnblockableThisTurn { .. }
                | EffectDef::AtNextStep { .. }
                | EffectDef::ReduceGenericCostBy(_)
                | EffectDef::MultiplyEventAmount(_)
                | EffectDef::MoveToZone { .. }
                | EffectDef::ChooseCreatureType { .. }
                | EffectDef::Apply { .. }
                | EffectDef::Special(_) => {}
            }
        });
        visiting.pop();
        colors.sort_unstable();
        colors.dedup();
        colors
    }

    fn mana_ability_activation(
        &self,
        permanent: &Permanent,
        ability: AbilityOrigin,
        color: ManaColor,
    ) -> Option<ManaAbilityActivation> {
        self.mana_ability_activations(permanent)
            .into_iter()
            .find(|activation| activation.ability == ability && activation.color == color)
    }

    fn mana_production(activation: ManaAbilityActivation) -> ManaPool {
        let mut pool = ManaPool::default();
        pool.add_color(activation.color, activation.effect.amount);
        pool
    }

    fn mana_for_activation(activation: ManaAbilityActivation) -> Vec<Mana> {
        let mana = Mana::from_ability(
            activation.color,
            ManaSource {
                object: activation.source,
                ability: activation.ability,
            },
            activation.effect.restrictions,
            activation.effect.spend_effects,
        );
        vec![mana; usize::from(activation.effect.amount)]
    }

    fn add_mana(&mut self, player: PlayerId, mana: impl IntoIterator<Item = Mana>) {
        for mana in mana {
            self.players[player.index()]
                .mana_pool
                .add_color(mana.color, 1);
            self.players[player.index()].mana.push(mana);
        }
    }

    fn add_unrestricted_mana(&mut self, player: PlayerId, color: ManaColor, amount: u16) {
        self.add_mana(
            player,
            std::iter::repeat_n(Mana::unrestricted(color), usize::from(amount)),
        );
    }

    fn payment_object(&self, purpose: &ManaPaymentPurpose) -> Option<(TriggerEventObject, bool)> {
        match purpose {
            ManaPaymentPurpose::Spell {
                object,
                definition,
                controller,
                form,
            } => self
                .printed_trigger_event_object(
                    *object,
                    *definition,
                    *controller,
                    &CharacteristicContext::Stack { form: form.clone() },
                )
                .map(|object| (object, true)),
            ManaPaymentPurpose::Ability { source, .. } => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == *source)
                .map(|permanent| (self.trigger_event_object(permanent), false))
                .or_else(|| match self.retired_objects.get(source) {
                    Some(RetiredObject::Permanent { permanent, .. }) => {
                        Some((self.trigger_event_object(permanent), false))
                    }
                    Some(RetiredObject::Card(_) | RetiredObject::Stack(_)) | None => None,
                })
                .or_else(|| {
                    let (zone, card) = self.card_in_nonbattlefield_zone(*source)?;
                    let context = match zone {
                        ZoneKind::Library => CharacteristicContext::Library,
                        ZoneKind::Hand => CharacteristicContext::Hand,
                        ZoneKind::Graveyard => CharacteristicContext::Graveyard,
                        ZoneKind::Exile => CharacteristicContext::Exile,
                        ZoneKind::Command => CharacteristicContext::Command,
                        ZoneKind::Battlefield | ZoneKind::Stack => return None,
                    };
                    self.printed_trigger_event_object(
                        card.id,
                        card.definition,
                        card.owner,
                        &context,
                    )
                    .map(|object| (object, false))
                }),
            ManaPaymentPurpose::Other => None,
        }
    }

    fn chosen_creature_type_for_mana_source(&self, source: GameObjectId) -> Option<&str> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
            .and_then(|permanent| permanent.chosen_creature_type.as_deref())
            .or_else(|| match self.retired_objects.get(&source) {
                Some(RetiredObject::Permanent { permanent, .. }) => {
                    permanent.chosen_creature_type.as_deref()
                }
                Some(RetiredObject::Card(_) | RetiredObject::Stack(_)) | None => None,
            })
    }

    fn mana_can_pay_for(&self, mana: Mana, purpose: &ManaPaymentPurpose) -> bool {
        mana.restrictions
            .iter()
            .all(|restriction| match restriction {
                ManaRestrictionDef::CastSpell(predicate) => self
                    .payment_object(purpose)
                    .is_some_and(|(object, is_spell)| {
                        is_spell
                            && self.trigger_object_matches(*predicate, &object, object.id, true)
                    }),
                ManaRestrictionDef::CastCreatureSpellOfChosenType => {
                    let Some(source) = mana.source else {
                        return false;
                    };
                    let Some(chosen) = self.chosen_creature_type_for_mana_source(source.object)
                    else {
                        return false;
                    };
                    self.payment_object(purpose)
                        .is_some_and(|(object, is_spell)| {
                            is_spell
                                && object.types.contains(CardType::Creature)
                                && object.subtypes.contains(&chosen)
                        })
                }
                ManaRestrictionDef::ActivateAbility(predicate) => self
                    .payment_object(purpose)
                    .is_some_and(|(object, is_spell)| {
                        !is_spell
                            && self.trigger_object_matches(*predicate, &object, object.id, false)
                    }),
                ManaRestrictionDef::Special(_) => false,
            })
    }

    fn mana_has_spend_effect_for(mana: Mana, purpose: &ManaPaymentPurpose) -> bool {
        mana.spend_effects.iter().any(|effect| {
            matches!(
                (purpose, effect),
                (
                    ManaPaymentPurpose::Spell { .. },
                    ManaSpendEffectDef::ApplyToPaidSpell(_)
                ) | (
                    ManaPaymentPurpose::Ability { .. },
                    ManaSpendEffectDef::ApplyToPaidAbility(_)
                )
            )
        })
    }

    fn eligible_mana_pool(&self, player: PlayerId, purpose: &ManaPaymentPurpose) -> ManaPool {
        let aggregate = self.players[player.index()].mana_pool;
        let mut eligible = ManaPool::default();
        let mut tracked = ManaPool::default();
        for mana in &self.players[player.index()].mana {
            if tracked.amount(mana.color) >= aggregate.amount(mana.color) {
                continue;
            }
            tracked.add_color(mana.color, 1);
            if self.mana_can_pay_for(*mana, purpose) {
                eligible.add_color(mana.color, 1);
            }
        }
        // Compatibility callers and tests may still write aggregate pools
        // directly. Any units without per-mana records are unrestricted.
        for color in [
            ManaColor::White,
            ManaColor::Blue,
            ManaColor::Black,
            ManaColor::Red,
            ManaColor::Green,
            ManaColor::Colorless,
        ] {
            eligible.add_color(
                color,
                aggregate
                    .amount(color)
                    .saturating_sub(tracked.amount(color)),
            );
        }
        eligible
    }

    fn pay_player_cost_for(
        &mut self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        purpose: &ManaPaymentPurpose,
    ) -> Vec<Mana> {
        self.reconcile_mana(player);
        let before = self.eligible_mana_pool(player, purpose);
        let mut after = before;
        let has_eligible_spend_effect = |color| {
            self.players[player.index()].mana.iter().any(|mana| {
                mana.color == color
                    && self.mana_can_pay_for(*mana, purpose)
                    && Self::mana_has_spend_effect_for(*mana, purpose)
            })
        };
        let mut hybrid_order = [ManaColor::Red, ManaColor::White];
        hybrid_order.sort_by_key(|color| !has_eligible_spend_effect(*color));
        let mut generic_order = [
            ManaColor::Colorless,
            ManaColor::Green,
            ManaColor::Black,
            ManaColor::Red,
            ManaColor::White,
            ManaColor::Blue,
        ];
        generic_order.sort_by_key(|color| !has_eligible_spend_effect(*color));
        pay_cost_with_orders(&mut after, cost, x, &hybrid_order, &generic_order);
        let mut spent = Vec::new();
        for color in [
            ManaColor::White,
            ManaColor::Blue,
            ManaColor::Black,
            ManaColor::Red,
            ManaColor::Green,
            ManaColor::Colorless,
        ] {
            let count = before.amount(color).saturating_sub(after.amount(color));
            for _ in 0..count {
                let index = self.players[player.index()]
                    .mana
                    .iter()
                    .enumerate()
                    .filter(|(_, mana)| {
                        mana.color == color && self.mana_can_pay_for(**mana, purpose)
                    })
                    .max_by_key(|(_, mana)| {
                        (
                            Self::mana_has_spend_effect_for(**mana, purpose),
                            !mana.restrictions.is_empty(),
                        )
                    })
                    .map(|(index, _)| index);
                if let Some(index) = index {
                    spent.push(self.players[player.index()].mana.remove(index));
                }
                self.players[player.index()]
                    .mana_pool
                    .remove_color(color, 1);
            }
        }
        spent
    }

    fn pay_player_cost(&mut self, player: PlayerId, cost: ManaCost, x: u16) -> Vec<Mana> {
        self.pay_player_cost_for(player, cost, x, &ManaPaymentPurpose::Other)
    }

    fn apply_spent_mana_to_spell(object: &mut StackObject, spent: &[Mana]) {
        for mana in spent {
            for spend_effect in mana.spend_effects {
                if let ManaSpendEffectDef::ApplyToPaidSpell(effect) = *spend_effect {
                    object.applied_effects.push(AppliedStackEffect {
                        source: mana.source,
                        effect,
                    });
                }
            }
        }
    }

    /// Tests and compatibility callers can still construct aggregate pools
    /// directly. Trim any now-impossible annotations before authoritative
    /// payment so that those writes cannot leave stale spend riders behind.
    fn reconcile_mana(&mut self, player: PlayerId) {
        for color in [
            ManaColor::White,
            ManaColor::Blue,
            ManaColor::Black,
            ManaColor::Red,
            ManaColor::Green,
            ManaColor::Colorless,
        ] {
            let allowed = usize::from(self.players[player.index()].mana_pool.amount(color));
            let mut retained = 0;
            self.players[player.index()].mana.retain(|mana| {
                if mana.color != color {
                    true
                } else if retained < allowed {
                    retained += 1;
                    true
                } else {
                    false
                }
            });
        }
    }

    fn can_pay_cost(&self, player: PlayerId, cost: ManaCost, x: u16) -> bool {
        self.can_pay_cost_for(player, cost, x, &ManaPaymentPurpose::Other)
    }

    fn can_pay_cost_for(
        &self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        purpose: &ManaPaymentPurpose,
    ) -> bool {
        self.assigned_mana_activations_for(player, cost, x, purpose)
            .is_some()
    }

    /// Returns the mana sources the engine's default payment policy would tap
    /// for an action. This is a read-only preview for clients; applying the
    /// action still performs the authoritative payment and validation.
    #[must_use]
    pub fn mana_sources_for_action(&self, player: PlayerId, action: &Action) -> Vec<GameObjectId> {
        let Some((cost, x, avoid, purpose)) = self.mana_requirement(player, action) else {
            return Vec::new();
        };
        self.plan_mana_sources(player, cost, x, avoid, &purpose)
    }

    #[allow(clippy::too_many_lines)]
    fn mana_requirement(
        &self,
        player: PlayerId,
        action: &Action,
    ) -> Option<(ManaCost, u16, Option<GameObjectId>, ManaPaymentPurpose)> {
        match action {
            Action::CastSpell { card, choices, .. } => {
                let definition = self
                    .players
                    .iter()
                    .flat_map(|player| player.hand.iter().chain(&player.graveyard))
                    .find(|candidate| candidate.id == *card)
                    .and_then(|candidate| self.catalog.get(candidate.definition))?;
                let option = definition.play_option(choices.play_option())?;
                let behavior = Self::play_option_behavior(definition, option)
                    .unwrap_or(CardBehavior::Unsupported);
                let cost = self.configured_cast_mana_cost(*card, option, choices.costs())?;
                Some((
                    reduce_generic(
                        add_generic(
                            cost,
                            fireball_extra_cost(behavior, choices.iter_targets().count()),
                        ),
                        self.spell_cost_reduction(definition.id, player),
                    ),
                    choices.x(),
                    None,
                    ManaPaymentPurpose::Spell {
                        object: *card,
                        definition: definition.id,
                        controller: player,
                        form: option.form.clone(),
                    },
                ))
            }
            Action::ActivateAbility {
                source,
                ability,
                targets,
                x,
                ..
            } => self.ability_mana_requirement(player, *source, *ability, targets, *x),
            _ => None,
        }
    }

    /// The mana half of an activation cost, and how the payment should treat
    /// the ability's own source.
    fn ability_mana_requirement(
        &self,
        player: PlayerId,
        source: GameObjectId,
        ability: AbilityOrigin,
        targets: &[TargetSelection],
        x: u16,
    ) -> Option<(ManaCost, u16, Option<GameObjectId>, ManaPaymentPurpose)> {
        if let Some(card) = self.players[player.index()]
            .hand
            .iter()
            .find(|card| card.id == source)
            && let Some(definition) = self
                .find_printed_card_ability(card, &CharacteristicContext::Hand, |effective| {
                    effective.origin == ability
                        && effective.ability.implementation == AbilityImplementationDef::Definition
                })
                .and_then(|effective| match effective.ability.definition {
                    DeclarativeAbilityDef::Activated(definition)
                        if definition.source_zones.contains(&ZoneKind::Hand) =>
                    {
                        Some(definition)
                    }
                    _ => None,
                })
        {
            return Self::activated_ability_mana_cost(definition).map(|cost| {
                (
                    cost,
                    x,
                    None,
                    ManaPaymentPurpose::Ability {
                        source,
                        taps_source: false,
                    },
                )
            });
        }

        let permanent = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)?;
        if let Some(definition) = self
            .find_effective_ability(permanent, |effective| effective.origin == ability)
            .and_then(|effective| match effective.ability.definition {
                DeclarativeAbilityDef::Activated(definition) => Some(definition),
                DeclarativeAbilityDef::Spell(_)
                | DeclarativeAbilityDef::ActivatedMana(_)
                | DeclarativeAbilityDef::TriggeredMana(_)
                | DeclarativeAbilityDef::Triggered(_)
                | DeclarativeAbilityDef::Static(_)
                | DeclarativeAbilityDef::Replacement(_)
                | DeclarativeAbilityDef::AlternativeCast(_)
                | DeclarativeAbilityDef::SpecialAction(_)
                | DeclarativeAbilityDef::Keyword(_)
                | DeclarativeAbilityDef::Legacy => None,
            })
        {
            let cost = Self::activated_ability_mana_cost(definition);
            let taps_source = definition.costs.contains(&AbilityCostDef::TapSource);
            return cost.map(|cost| {
                (
                    cost,
                    x,
                    (taps_source
                        || self.effective_behavior(permanent)
                            == Some(CardBehavior::MishrasFactory))
                    .then_some(source),
                    ManaPaymentPurpose::Ability {
                        source,
                        taps_source,
                    },
                )
            });
        }

        let behavior = self.effective_behavior(permanent)?;
        let cost = match behavior {
            CardBehavior::MishrasFactory
                if targets
                    .iter()
                    .all(|selection| selection.targets().is_empty()) =>
            {
                ManaCost::new(1, 0)
            }
            CardBehavior::ChaosOrb
            | CardBehavior::NevinyrralsDisk
            | CardBehavior::IcyManipulator => ManaCost::new(1, 0),
            CardBehavior::SedgeTroll => ManaCost::colored(0, 0, 0, 1, 0, 0),
            _ => return None,
        };
        Some((
            cost,
            0,
            (behavior == CardBehavior::MishrasFactory).then_some(source),
            // Mishra's Factory animates without tapping, so tapping it for the
            // mana is legal even if it is rarely wanted.
            ManaPaymentPurpose::Ability {
                source,
                taps_source: false,
            },
        ))
    }

    fn activated_ability_mana_cost(definition: ActivatedAbilityDef) -> Option<ManaCost> {
        let mut cost = ManaCost::default();
        let mut has_mana_cost = false;
        for ability_cost in definition.costs.as_slice() {
            if let AbilityCostDef::Mana(mana) = ability_cost {
                cost = add_mana_cost(cost, *mana);
                has_mana_cost = true;
            }
        }
        has_mana_cost.then_some(cost)
    }

    fn plan_mana_sources(
        &self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        avoid: Option<GameObjectId>,
        purpose: &ManaPaymentPurpose,
    ) -> Vec<GameObjectId> {
        self.plan_mana_activations_for(player, cost, x, avoid, purpose)
            .unwrap_or_default()
            .into_iter()
            .map(|activation| activation.source)
            .collect()
    }

    fn assigned_mana_activations_for(
        &self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        purpose: &ManaPaymentPurpose,
    ) -> Option<Vec<PlannedManaActivation>> {
        let mut pool = self.eligible_mana_pool(player, purpose);
        let mut assigned = Vec::new();
        let mut flexible = Vec::new();
        // An ability that taps its source as a cost cannot also tap it for
        // mana, so that source is not a candidate at all.
        let barred = match purpose {
            ManaPaymentPurpose::Ability {
                source,
                taps_source: true,
            } => Some(*source),
            _ => None,
        };
        for (order, permanent) in self
            .battlefield
            .iter()
            .filter(|permanent| permanent.controller == player)
            .filter(|permanent| Some(permanent.card.id) != barred)
            .enumerate()
        {
            let mut activations = self
                .mana_ability_activations(permanent)
                .into_iter()
                .filter(|activation| {
                    Self::mana_for_activation(*activation)
                        .first()
                        .is_some_and(|mana| self.mana_can_pay_for(*mana, purpose))
                })
                .collect::<Vec<_>>();
            // When several outputs are legal, prefer one whose spend rider
            // benefits this payment. Players can still manually choose a
            // different mana ability before casting.
            activations.sort_by_key(|activation| {
                let benefits_payment = Self::mana_for_activation(*activation)
                    .first()
                    .is_some_and(|mana| Self::mana_has_spend_effect_for(*mana, purpose));
                let pays_colored_symbol = mana_cost_amount(cost, activation.color) > 0
                    || cost.white_red_hybrid > 0
                        && matches!(activation.color, ManaColor::White | ManaColor::Red);
                (!benefits_payment, !pays_colored_symbol)
            });
            let outputs = activations
                .into_iter()
                .map(|activation| {
                    let benefits_payment = Self::mana_for_activation(activation)
                        .first()
                        .is_some_and(|mana| Self::mana_has_spend_effect_for(*mana, purpose));
                    (
                        activation.ability,
                        activation.color,
                        Self::mana_production(activation),
                        benefits_payment,
                    )
                })
                .collect::<Vec<_>>();
            match outputs.as_slice() {
                [] => {}
                [(ability, color, production, benefits_payment)] => {
                    pool.add(*production);
                    assigned.push(PlannedManaActivation {
                        source: permanent.card.id,
                        ability: *ability,
                        color: *color,
                        production: *production,
                        benefits_payment: *benefits_payment,
                        flexibility: 1,
                        order,
                    });
                }
                _ => flexible.push(FlexibleManaSource {
                    source: permanent.card.id,
                    outputs,
                    order,
                }),
            }
        }

        let mut flexible_assignment = Vec::new();
        if !assign_flexible_mana_outputs(&flexible, 0, pool, cost, x, &mut flexible_assignment) {
            return None;
        }
        assigned.extend(flexible_assignment);
        Some(assigned)
    }

    fn plan_mana_activations_for(
        &self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        avoid: Option<GameObjectId>,
        purpose: &ManaPaymentPurpose,
    ) -> Option<Vec<PlannedManaActivation>> {
        let mut available = self.assigned_mana_activations_for(player, cost, x, purpose)?;
        let mut pool = self.eligible_mana_pool(player, purpose);
        let mut selected = Vec::new();

        for color in colored_mana() {
            let required = mana_cost_amount(cost, color);
            while pool.amount(color) < required {
                let index = available
                    .iter()
                    .enumerate()
                    .filter(|(_, activation)| activation.color == color)
                    .min_by_key(|(_, activation)| {
                        (
                            Some(activation.source) == avoid,
                            !activation.benefits_payment,
                            activation.flexibility,
                            activation.production.total(),
                            activation.order,
                        )
                    })
                    .map(|(index, _)| index)?;
                let activation = available.remove(index);
                pool.add(activation.production);
                selected.push(activation);
            }
        }

        while available_white_red_hybrid(pool, cost) < cost.white_red_hybrid {
            let index = available
                .iter()
                .enumerate()
                .filter(|(_, activation)| {
                    matches!(activation.color, ManaColor::White | ManaColor::Red)
                })
                .min_by_key(|(_, activation)| {
                    (
                        Some(activation.source) == avoid,
                        !activation.benefits_payment,
                        activation.flexibility,
                        activation.production.total(),
                        activation.order,
                    )
                })
                .map(|(index, _)| index)?;
            let activation = available.remove(index);
            pool.add(activation.production);
            selected.push(activation);
        }

        let required_total = colored_cost_total(cost)
            .saturating_add(cost.generic)
            .saturating_add(x.saturating_mul(cost.x_multiplier));
        while pool.total() < required_total {
            let index = available
                .iter()
                .enumerate()
                .min_by_key(|(_, activation)| {
                    (
                        Some(activation.source) == avoid,
                        !activation.benefits_payment,
                        activation.color != ManaColor::Colorless,
                        activation.production.total(),
                        activation.order,
                    )
                })
                .map(|(index, _)| index)?;
            let activation = available.remove(index);
            pool.add(activation.production);
            selected.push(activation);
        }

        debug_assert!(can_pay(pool, cost, x));
        Some(selected)
    }

    /// How much generic mana this card's own static clauses take off its
    /// cost. Read from the hand, which is where casting reads it.
    fn spell_cost_reduction(&self, definition: CardDefinitionId, player: PlayerId) -> u16 {
        let Some(card) = self.catalog.get(definition) else {
            return 0;
        };
        card.rules
            .ability_clauses()
            .iter()
            .filter(|ability| ability.implementation.is_executable())
            .filter_map(|ability| match ability.effect {
                EffectDef::ReduceGenericCostBy(value) => Some(value),
                _ => None,
            })
            .map(|value| self.cost_reduction_value(value, player))
            .fold(0, u16::saturating_add)
    }

    /// The values a cost reduction can read. There is no resolving object
    /// while a cost is being worked out, so only board counts are available.
    fn cost_reduction_value(&self, value: ValueDef, player: PlayerId) -> u16 {
        match value {
            ValueDef::Constant(amount) => u16::try_from(amount.max(0)).unwrap_or(u16::MAX),
            ValueDef::CountMatchingObjects(query) if query.zones == [ZoneKind::Battlefield] => {
                u16::try_from(
                    self.battlefield
                        .iter()
                        .filter(|permanent| {
                            self.player_relation_matches(
                                permanent.controller,
                                query.controller,
                                player,
                                TriggerContext::empty(),
                            ) && self.trigger_object_matches(
                                query.object,
                                &self.trigger_event_object(permanent),
                                permanent.card.id,
                                false,
                            )
                        })
                        .count(),
                )
                .unwrap_or(u16::MAX)
            }
            _ => 0,
        }
    }

    fn maximum_x_for(&self, player: PlayerId, cost: ManaCost, purpose: &ManaPaymentPurpose) -> u16 {
        let maximum = self.players[player.index()]
            .mana_pool
            .total()
            .saturating_add(
                self.battlefield
                    .iter()
                    .filter(|permanent| permanent.controller == player)
                    .filter_map(|permanent| {
                        self.mana_ability_activations(permanent)
                            .into_iter()
                            .map(Self::mana_production)
                            .max_by_key(|production| production.total())
                    })
                    .map(ManaPool::total)
                    .sum(),
            );
        // The upper bound is only a search ceiling; can_pay_cost_for is
        // what rules each X in or out, including the barred source.
        (0..=maximum)
            .rev()
            .find(|x| self.can_pay_cost_for(player, cost, *x, purpose))
            .unwrap_or(0)
    }

    fn activate_mana_for_cost(&mut self, player: PlayerId, cost: ManaCost, x: u16) {
        self.activate_mana_for_cost_avoiding(player, cost, x, None);
    }

    fn activate_mana_for_cost_avoiding(
        &mut self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        avoid: Option<GameObjectId>,
    ) {
        self.activate_mana_for_cost_avoiding_for(
            player,
            cost,
            x,
            avoid,
            &ManaPaymentPurpose::Other,
        );
    }

    fn activate_mana_for_cost_avoiding_for(
        &mut self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        avoid: Option<GameObjectId>,
        purpose: &ManaPaymentPurpose,
    ) {
        let plan = self
            .plan_mana_activations_for(player, cost, x, avoid, purpose)
            .expect("a legal payment has a complete mana activation plan");
        for activation in plan {
            self.activate_mana_source(
                player,
                activation.source,
                activation.ability,
                activation.color,
            );
        }
    }

    fn base_stats(&self, permanent: &Permanent) -> Option<crate::CreatureStats> {
        // Once Factory's animation ability resolves, removing its printed
        // abilities does not end the continuous animation effect. In
        // particular, Blood Moon changes its land subtype and abilities but
        // leaves the active artifact-creature types and 2/2 base stats intact.
        if permanent.factory_animated {
            Some(crate::CreatureStats {
                power: 2,
                toughness: 2,
            })
        } else {
            self.effective_rules(permanent)
                .and_then(CardRules::creature_stats)
        }
    }

    fn controls_land_type(&self, player: PlayerId, land_type: BasicLandType) -> bool {
        self.battlefield.iter().any(|permanent| {
            permanent.controller == player
                && self.effective_land_types(permanent)[land_type.index()]
        })
    }

    fn controls_any_land_type(&self, player: PlayerId, types: [bool; 5]) -> bool {
        self.battlefield.iter().any(|permanent| {
            if permanent.controller != player
                || !self
                    .permanent_types(permanent)
                    .is_some_and(|card_types| card_types.contains(CardType::Land))
            {
                return false;
            }
            self.effective_land_types(permanent)
                .into_iter()
                .zip(types)
                .any(|(present, wanted)| present && wanted)
        })
    }

    fn crusade_bonus(&self, permanent: &Permanent) -> i16 {
        if !self
            .effective_rules(permanent)
            .is_some_and(|rules| rules.colors()[0])
        {
            return 0;
        }
        i16::try_from(self.count_behavior(CardBehavior::Crusade)).unwrap_or(i16::MAX)
    }

    fn plus_one_counter_bonus(permanent: &Permanent) -> i16 {
        i16::try_from(permanent.counters(CounterKind::PlusOnePlusOne)).unwrap_or(i16::MAX)
    }

    /// Whether any permanent on the battlefield matches, which is what an "as
    /// long as you control a ..." clause asks. `controller` is whoever the
    /// query's player relation is measured against.
    fn any_battlefield_object_matches(
        &self,
        query: &ObjectQueryDef,
        source: GameObjectId,
        controller: PlayerId,
    ) -> bool {
        self.battlefield.iter().any(|permanent| {
            self.player_relation_matches(
                permanent.controller,
                query.controller,
                controller,
                TriggerContext::empty(),
            ) && self.trigger_object_matches(
                query.object,
                &self.trigger_event_object(permanent),
                source,
                false,
            )
        })
    }

    fn static_power_toughness_bonus(&self, permanent: &Permanent) -> (i16, i16) {
        let mut total = (0_i16, 0_i16);
        let result = self.visit_static_applied_effects(permanent, |applied| {
            if let AppliedEffectDef::ModifyPowerToughness { power, toughness } = applied.effect {
                // A static bonus is measured from its own source's controller,
                // not from whoever it is being applied to.
                let controller = self
                    .controller_of_object(applied.source)
                    .unwrap_or(permanent.controller);
                let bonus = |value: ValueDef| -> i16 {
                    let amount = match value {
                        ValueDef::Constant(amount) => amount,
                        ValueDef::AnyMatchingObject(query) => i32::from(
                            self.any_battlefield_object_matches(query, applied.source, controller),
                        ),
                        // Everything else stays a seam; the boundary test
                        // rejects a card that reaches for one.
                        _ => 0,
                    };
                    i16::try_from(amount.clamp(i32::from(i16::MIN), i32::from(i16::MAX)))
                        .expect("the static bonus was clamped to i16")
                };
                total = (
                    total.0.saturating_add(bonus(power)),
                    total.1.saturating_add(bonus(toughness)),
                );
            }
            ControlFlow::Continue(())
        });
        debug_assert!(result.is_continue());
        total
    }

    /// Power without continuous static bonuses.
    ///
    /// Characteristics handed to a predicate cannot use full `power`: static
    /// effects are resolved by matching each source against the affected
    /// permanent's characteristics, so asking for power there would re-enter
    /// this computation forever. A `PowerAtLeast` predicate therefore sees
    /// counters and until-end-of-turn pumps but not a Crusade-style static.
    fn power_ignoring_static_effects(&self, permanent: &Permanent) -> Option<i16> {
        self.power_parts(permanent, 0)
    }

    fn power(&self, permanent: &Permanent) -> Option<i16> {
        let (static_power, _) = self.static_power_toughness_bonus(permanent);
        self.power_parts(permanent, static_power)
    }

    fn power_parts(&self, permanent: &Permanent, static_power: i16) -> Option<i16> {
        self.base_stats(permanent).map(|stats| {
            let conditional_bonus = match self.effective_behavior(permanent) {
                Some(CardBehavior::KirdApe)
                    if self.controls_land_type(permanent.controller, BasicLandType::Forest) =>
                {
                    1
                }
                Some(CardBehavior::SedgeTroll)
                    if self.controls_land_type(permanent.controller, BasicLandType::Swamp) =>
                {
                    1
                }
                _ => 0,
            };
            let ascended = if self.blood_baron_has_ascended(permanent) {
                6
            } else {
                0
            };
            stats.power
                + ascended
                + permanent.power_bonus
                + self.crusade_bonus(permanent)
                + static_power
                + conditional_bonus
                + Self::plus_one_counter_bonus(permanent)
        })
    }

    fn toughness(&self, permanent: &Permanent) -> Option<i16> {
        self.base_stats(permanent).map(|stats| {
            let (_, static_toughness) = self.static_power_toughness_bonus(permanent);
            let conditional_bonus = match self.effective_behavior(permanent) {
                Some(CardBehavior::KirdApe)
                    if self.controls_land_type(permanent.controller, BasicLandType::Forest) =>
                {
                    2
                }
                Some(CardBehavior::SedgeTroll)
                    if self.controls_land_type(permanent.controller, BasicLandType::Swamp) =>
                {
                    1
                }
                _ => 0,
            };
            let ascended = if self.blood_baron_has_ascended(permanent) {
                6
            } else {
                0
            };
            stats.toughness
                + ascended
                + permanent.toughness_bonus
                + self.crusade_bonus(permanent)
                + static_toughness
                + conditional_bonus
                + Self::plus_one_counter_bonus(permanent)
        })
    }

    fn has_flying(&self, permanent: &Permanent) -> bool {
        self.permanent_has_executable_keyword(permanent, KeywordAbility::Flying)
            || self.blood_baron_has_ascended(permanent)
    }

    /// Blood Baron of Vizkopa's condition: 30 or more life for its controller
    /// and 10 or less for the opponent. While it holds the Baron is +6/+6 and
    /// flies.
    fn blood_baron_has_ascended(&self, permanent: &Permanent) -> bool {
        self.effective_behavior(permanent) == Some(CardBehavior::BloodBaronOfVizkopa)
            && self.players[permanent.controller.index()].life >= 30
            && self.players[permanent.controller.opponent().index()].life <= 10
    }

    fn has_trample(&self, permanent: &Permanent) -> bool {
        self.permanent_has_executable_keyword(permanent, KeywordAbility::Trample)
    }

    fn has_undying(&self, permanent: &Permanent) -> bool {
        self.permanent_has_executable_keyword(permanent, KeywordAbility::Undying)
    }

    fn has_hexproof(&self, permanent: &Permanent) -> bool {
        self.permanent_has_executable_keyword(permanent, KeywordAbility::Hexproof)
    }

    fn has_mountainwalk(&self, permanent: &Permanent) -> bool {
        self.permanent_has_executable_keyword(permanent, KeywordAbility::Mountainwalk)
    }

    fn permanent_has_executable_keyword(
        &self,
        permanent: &Permanent,
        expected: KeywordAbility,
    ) -> bool {
        permanent.temporary_keywords.contains(&expected)
            || self
                .find_effective_ability(permanent, |effective| {
                    effective.ability.implementation.is_executable()
                        && matches!(
                            effective.ability.definition,
                            DeclarativeAbilityDef::Keyword(actual) if actual == expected
                        )
                })
                .is_some()
    }

    fn source_controller_with_keyword(
        &self,
        source: GameObjectId,
        expected: KeywordAbility,
    ) -> Option<PlayerId> {
        if let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
        {
            return self
                .permanent_has_executable_keyword(permanent, expected)
                .then_some(permanent.controller);
        }
        match self.retired_objects.get(&source) {
            Some(RetiredObject::Permanent {
                permanent,
                keywords,
                ..
            }) if keywords.contains(&expected) => Some(permanent.controller),
            Some(
                RetiredObject::Permanent { .. } | RetiredObject::Card(_) | RetiredObject::Stack(_),
            )
            | None => None,
        }
    }

    fn has_forestwalk(permanent: &Permanent) -> bool {
        permanent.forestwalk_until_upkeep_of.is_some()
    }

    fn controls_mountain(&self, player: PlayerId) -> bool {
        self.controls_land_type(player, BasicLandType::Mountain)
    }

    fn controls_forest(&self, player: PlayerId) -> bool {
        self.controls_land_type(player, BasicLandType::Forest)
    }

    fn can_use_tap_ability(&self, permanent: &Permanent) -> bool {
        self.base_stats(permanent).is_none_or(|_| {
            self.permanent_has_executable_keyword(permanent, KeywordAbility::Haste)
                || self.turns_started[permanent.controller.index()]
                    > permanent.entered_controller_turn
        })
    }

    #[allow(clippy::too_many_lines)]
    fn activate_ability(
        &mut self,
        player: PlayerId,
        source: GameObjectId,
        ability: AbilityOrigin,
        targets: Vec<TargetSelection>,
        sacrifice: Option<GameObjectId>,
        x: u16,
    ) {
        if let Some(source_card) = self.players[player.index()]
            .hand
            .iter()
            .find(|card| card.id == source)
            .cloned()
        {
            let Some(effective) = self.find_printed_card_ability(
                &source_card,
                &CharacteristicContext::Hand,
                |effective| effective.origin == ability,
            ) else {
                return;
            };
            let DeclarativeAbilityDef::Activated(definition) = effective.ability.definition else {
                return;
            };
            if effective.ability.implementation != AbilityImplementationDef::Definition
                || !definition.source_zones.contains(&ZoneKind::Hand)
            {
                return;
            }
            let frozen = FrozenActivatedAbility {
                origin: effective.origin,
                definition: Some(Box::new(effective.ability)),
                presentation_definition: Self::ability_presentation_definition(
                    effective.origin,
                    source_card.definition,
                ),
                text: Some(effective.ability.text),
                target_defs: definition.targets,
                resolver: Self::ability_resolver(&effective.ability),
                x,
            };
            let payment_purpose = ManaPaymentPurpose::Ability {
                source,
                taps_source: false,
            };
            for cost in definition.costs.as_slice() {
                match cost {
                    AbilityCostDef::Mana(cost) => {
                        self.activate_mana_for_cost_avoiding_for(
                            player,
                            *cost,
                            x,
                            None,
                            &payment_purpose,
                        );
                        let _ = self.pay_player_cost_for(player, *cost, x, &payment_purpose);
                    }
                    AbilityCostDef::DiscardSource => {
                        let discarded = remove_card(&mut self.players[player.index()].hand, source)
                            .expect("a legal hand activation still has its source");
                        let definition = discarded.definition;
                        let (discarded, _zone_change) = self.zone_change_card(discarded);
                        let discarded_id = discarded.id;
                        self.players[player.index()].graveyard.push(discarded);
                        self.events.push(GameEvent::CardsDiscarded {
                            player,
                            cards: vec![(discarded_id, definition)],
                        });
                    }
                    AbilityCostDef::TapSource
                    | AbilityCostDef::UntapSource
                    | AbilityCostDef::SacrificeSource
                    | AbilityCostDef::PayLife(_)
                    | AbilityCostDef::DiscardCards(_)
                    | AbilityCostDef::SacrificePermanent { .. }
                    | AbilityCostDef::ExileSource
                    | AbilityCostDef::Special(_) => {
                        unreachable!("unsupported hand-zone costs are not offered")
                    }
                }
            }
            let chosen_permanents = targets
                .iter()
                .flat_map(TargetSelection::targets)
                .filter_map(|target| match target {
                    Target::Permanent(permanent) => Some(*permanent),
                    Target::Player(_) | Target::Card(_) | Target::Spell(_) => None,
                })
                .collect();
            self.push_activated_ability(
                source,
                &source_card,
                player,
                frozen,
                targets,
                chosen_permanents,
            );
            self.consecutive_passes = 0;
            self.check_state_based_actions();
            return;
        }
        let Some(source_permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
        else {
            return;
        };
        let source_card = source_permanent.card.clone();
        let mut frozen_ability = self.freeze_activated_ability(source_permanent, ability);
        frozen_ability.x = x;
        // `apply` validated these exact ordered slot selections against a
        // generated legal action. Freeze both their slot identity and values
        // before any activation cost can move or change the source.
        let frozen_targets = targets;
        let target = frozen_targets
            .iter()
            .flat_map(TargetSelection::targets)
            .next()
            .copied();
        let declarative = self
            .find_effective_ability(source_permanent, |effective| effective.origin == ability)
            .map(|effective| effective.ability)
            .filter(|ability| {
                ability.implementation == AbilityImplementationDef::Definition
                    && matches!(ability.definition, DeclarativeAbilityDef::Activated(_))
            });
        let behavior = self.effective_behavior(source_permanent);
        if let Some(ability_def) = declarative {
            let DeclarativeAbilityDef::Activated(definition) = ability_def.definition else {
                unreachable!("the declarative activation filter checked its category")
            };
            let taps_source = definition.costs.contains(&AbilityCostDef::TapSource);
            for cost in definition.costs.as_slice() {
                match cost {
                    AbilityCostDef::Mana(cost) => {
                        self.activate_mana_for_cost_avoiding_for(
                            player,
                            *cost,
                            x,
                            taps_source.then_some(source),
                            &ManaPaymentPurpose::Ability {
                                source,
                                taps_source,
                            },
                        );
                        let _ = self.pay_player_cost(player, *cost, x);
                    }
                    AbilityCostDef::TapSource => {
                        let _ = self.tap_permanent(source);
                    }
                    AbilityCostDef::SacrificeSource => self.sacrifice_permanent(source),
                    AbilityCostDef::DiscardSource => {
                        unreachable!("a battlefield source cannot discard itself")
                    }
                    AbilityCostDef::PayLife(amount) => {
                        self.players[player.index()].life -=
                            i16::try_from(*amount).unwrap_or(i16::MAX);
                    }
                    AbilityCostDef::SacrificePermanent { .. } => {
                        self.sacrifice_permanent(
                            sacrifice.expect("a legal activation chose the sacrificed permanent"),
                        );
                    }
                    AbilityCostDef::UntapSource
                    | AbilityCostDef::DiscardCards(_)
                    | AbilityCostDef::ExileSource
                    | AbilityCostDef::Special(_) => {
                        unreachable!("unsupported costs are not offered as legal actions")
                    }
                }
            }
            let mut chosen_permanents = frozen_targets
                .iter()
                .flat_map(TargetSelection::targets)
                .filter_map(|target| match target {
                    Target::Permanent(permanent) => Some(*permanent),
                    Target::Player(_) | Target::Card(_) | Target::Spell(_) => None,
                })
                .collect::<Vec<_>>();
            if let Some(sacrificed) = sacrifice
                && !chosen_permanents.contains(&sacrificed)
            {
                chosen_permanents.push(sacrificed);
            }
            self.push_activated_ability(
                source,
                &source_card,
                player,
                frozen_ability,
                frozen_targets,
                chosen_permanents,
            );
            self.consecutive_passes = 0;
            self.check_state_based_actions();
            return;
        }
        match behavior {
            Some(CardBehavior::Atog) => {
                if let Some(sacrificed) = sacrifice {
                    self.sacrifice_permanent(sacrificed);
                    if let Some(atog) = self
                        .battlefield
                        .iter_mut()
                        .find(|permanent| permanent.card.id == source)
                    {
                        atog.power_bonus += 2;
                        atog.toughness_bonus += 2;
                    }
                }
            }
            Some(CardBehavior::GlassesOfUrza) => {
                let _ = self.tap_permanent(source);
                if let Some(Target::Player(target)) = target {
                    self.last_seen_hands[player.index()] =
                        Some((target, public_cards(&self.players[target.index()].hand)));
                }
            }
            Some(CardBehavior::IcyManipulator | CardBehavior::Pendelhaven) => {
                let cost = if behavior == Some(CardBehavior::IcyManipulator) {
                    ManaCost::new(1, 0)
                } else {
                    ManaCost::new(0, 0)
                };
                if cost.generic > 0 {
                    self.activate_mana_for_cost(player, cost, 0);
                    let _ = self.pay_player_cost(player, cost, 0);
                }
                let card = self
                    .tap_permanent(source)
                    .expect("legal tap ability has a source");
                self.push_activated_ability(
                    source,
                    &card,
                    player,
                    frozen_ability,
                    frozen_targets,
                    Vec::new(),
                );
            }
            Some(CardBehavior::SedgeTroll) => {
                let cost = ManaCost::colored(0, 0, 0, 1, 0, 0);
                self.activate_mana_for_cost(player, cost, 0);
                let _ = self.pay_player_cost(player, cost, 0);
                let card = self
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == source)
                    .map(|permanent| permanent.card.clone())
                    .expect("legal Sedge Troll activation has a source");
                self.push_activated_ability(
                    source,
                    &card,
                    player,
                    frozen_ability,
                    Vec::new(),
                    Vec::new(),
                );
            }
            Some(CardBehavior::StoneGiant) => {
                let _ = self.tap_permanent(source);
                if let Some(Target::Permanent(target)) = target
                    && let Some(creature) = self
                        .battlefield
                        .iter_mut()
                        .find(|permanent| permanent.card.id == target)
                {
                    if !creature
                        .temporary_keywords
                        .contains(&KeywordAbility::Flying)
                    {
                        creature.temporary_keywords.push(KeywordAbility::Flying);
                    }
                    creature.destroy_at_end = true;
                }
            }
            Some(CardBehavior::DragonWhelp) => {
                let cost = ManaCost::new(0, 1);
                self.activate_mana_for_cost(player, cost, 0);
                let _ = self.pay_player_cost(player, cost, 0);
                if let Some(permanent) = self
                    .battlefield
                    .iter_mut()
                    .find(|permanent| permanent.card.id == source)
                {
                    permanent.power_bonus += 1;
                    permanent.dragon_whelp_activations =
                        permanent.dragon_whelp_activations.saturating_add(1);
                    if permanent.dragon_whelp_activations >= 4 {
                        permanent.destroy_at_end = true;
                    }
                }
            }
            Some(CardBehavior::MishrasFactory) => {
                if let Some(Target::Permanent(target)) = target {
                    let _ = self.tap_permanent(source);
                    if let Some(worker) = self
                        .battlefield
                        .iter_mut()
                        .find(|permanent| permanent.card.id == target)
                    {
                        worker.power_bonus += 1;
                        worker.toughness_bonus += 1;
                    }
                } else {
                    let cost = ManaCost::new(1, 0);
                    self.activate_mana_for_cost_avoiding(player, cost, 0, Some(source));
                    let _ = self.pay_player_cost(player, cost, 0);
                    if let Some(permanent) = self
                        .battlefield
                        .iter_mut()
                        .find(|permanent| permanent.card.id == source)
                    {
                        permanent.factory_animated = true;
                    }
                }
            }
            Some(CardBehavior::ChaosOrb) => {
                let cost = ManaCost::new(1, 0);
                self.activate_mana_for_cost(player, cost, 0);
                let _ = self.pay_player_cost(player, cost, 0);
                let card = self
                    .tap_permanent(source)
                    .expect("legal Chaos Orb activation has a source");
                let chosen_permanents = match target {
                    Some(Target::Permanent(chosen)) => vec![chosen],
                    Some(Target::Player(_) | Target::Card(_) | Target::Spell(_)) | None => {
                        Vec::new()
                    }
                };
                self.push_activated_ability(
                    source,
                    &card,
                    player,
                    frozen_ability,
                    frozen_targets,
                    chosen_permanents,
                );
            }
            Some(CardBehavior::OrcishMechanics) => {
                let card = self
                    .tap_permanent(source)
                    .expect("legal Orcish Mechanics activation has a source");
                if let Some(sacrificed) = sacrifice {
                    self.sacrifice_permanent(sacrificed);
                }
                let chosen_permanents: Vec<_> = sacrifice.into_iter().collect();
                self.push_activated_ability(
                    source,
                    &card,
                    player,
                    frozen_ability,
                    frozen_targets,
                    chosen_permanents,
                );
            }
            Some(CardBehavior::Triskelion) => {
                let card = self
                    .battlefield
                    .iter_mut()
                    .find(|permanent| permanent.card.id == source)
                    .map(|permanent| {
                        permanent.remove_counter(CounterKind::PlusOnePlusOne);
                        permanent.card.clone()
                    })
                    .expect("legal Triskelion activation has a source");
                self.push_activated_ability(
                    source,
                    &card,
                    player,
                    frozen_ability,
                    frozen_targets,
                    Vec::new(),
                );
            }
            Some(
                behavior @ (CardBehavior::LibraryOfAlexandria
                | CardBehavior::MazeOfIth
                | CardBehavior::NevinyrralsDisk
                | CardBehavior::IcatianJavelineers
                | CardBehavior::TimeVault),
            ) => {
                if behavior == CardBehavior::NevinyrralsDisk {
                    let cost = ManaCost::new(1, 0);
                    self.activate_mana_for_cost(player, cost, 0);
                    let _ = self.pay_player_cost(player, cost, 0);
                }
                let card = self
                    .tap_permanent(source)
                    .expect("legal activation has a source");
                if behavior == CardBehavior::IcatianJavelineers {
                    self.battlefield
                        .iter_mut()
                        .find(|permanent| permanent.card.id == source)
                        .expect("the activation source remains on the battlefield")
                        .remove_counter(CounterKind::Javelin);
                }
                self.push_activated_ability(
                    source,
                    &card,
                    player,
                    frozen_ability,
                    frozen_targets,
                    Vec::new(),
                );
            }
            _ => {}
        }
        self.consecutive_passes = 0;
        self.check_state_based_actions();
    }

    fn attacker_actions(&self, player: PlayerId) -> Vec<Action> {
        self.battlefield
            .iter()
            .filter(|permanent| {
                permanent.controller == player
                    && !permanent.tapped
                    && !permanent.attacking
                    && self.power(permanent).is_some()
                    && self.can_attack(permanent)
            })
            .map(|permanent| Action::DeclareAttacker {
                attacker: permanent.card.id,
            })
            .collect()
    }

    fn can_attack(&self, permanent: &Permanent) -> bool {
        if self.permanent_has_executable_keyword(permanent, KeywordAbility::Defender) {
            return false;
        }
        if self.count_behavior(CardBehavior::Moat) > 0 && !self.has_flying(permanent) {
            return false;
        }
        self.base_stats(permanent).is_some_and(|_| {
            self.permanent_has_executable_keyword(permanent, KeywordAbility::Haste)
                || self.turns_started[permanent.controller.index()]
                    > permanent.entered_controller_turn
        })
    }

    fn declare_attacker(&mut self, attacker: GameObjectId) {
        let vigilance = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == attacker)
            .is_some_and(|permanent| {
                self.permanent_has_executable_keyword(permanent, KeywordAbility::Vigilance)
            });
        if let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == attacker)
        {
            permanent.attacking = true;
            permanent.attacked_this_turn = true;
        }
        if !vigilance {
            let _ = self.tap_permanent(attacker);
        }
    }

    fn finish_declaring_attackers(&mut self) {
        self.attackers_declared = true;
        self.priority = self.active_player;
        self.consecutive_passes = 0;
        let attackers = self
            .battlefield
            .iter()
            .filter(|permanent| permanent.controller == self.active_player && permanent.attacking)
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>();
        if attackers.is_empty() {
            return;
        }
        self.events.push(GameEvent::AttackDeclared {
            player: self.active_player,
            attackers: attackers.clone(),
        });
        // CR 508.2: the whole declaration happens at once, so every attacker
        // is already attacking by the time any of these triggers is captured.
        let events = attackers
            .iter()
            .filter_map(|attacker| {
                self.battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == *attacker)
                    .map(|permanent| CommittedTriggerEvent::Attacks {
                        object: self.trigger_event_object(permanent),
                    })
            })
            .collect::<Vec<_>>();
        for event in &events {
            self.capture_battlefield_triggers(event);
        }
    }

    /// Whether a static effect on `attacker` forbids `blocker` from blocking
    /// it, as Juggernaut forbids Walls.
    fn blocking_is_prevented(&self, attacker: &Permanent, blocker: &Permanent) -> bool {
        let characteristics = self.trigger_event_object(blocker);
        let mut prevented = false;
        let result = self.visit_static_applied_effects(attacker, |applied| {
            if let AppliedEffectDef::CannotBeBlockedBy(predicate) = applied.effect
                && self.trigger_object_matches(predicate, &characteristics, applied.source, false)
            {
                prevented = true;
                return ControlFlow::Break(());
            }
            ControlFlow::Continue(())
        });
        debug_assert!(result.is_continue() || prevented);
        prevented
    }

    fn blocker_actions(&self, player: PlayerId) -> Vec<Action> {
        let blockers: Vec<_> = self
            .battlefield
            .iter()
            .filter(|permanent| {
                permanent.controller == player
                    && !permanent.tapped
                    && permanent.blocking.is_none()
                    && self.power(permanent).is_some()
            })
            .map(|permanent| permanent.card.id)
            .collect();
        let attackers: Vec<_> = self
            .battlefield
            .iter()
            .filter(|permanent| permanent.attacking)
            .map(|permanent| {
                (
                    permanent.card.id,
                    self.has_flying(permanent),
                    (self.has_mountainwalk(permanent)
                        && self.controls_mountain(permanent.controller.opponent()))
                        || (Self::has_forestwalk(permanent)
                            && self.controls_forest(permanent.controller.opponent())),
                    self.power(permanent).unwrap_or(0),
                )
            })
            .collect();
        blockers
            .into_iter()
            .flat_map(|blocker| {
                let blocker_permanent = self
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == blocker)
                    .expect("blocker is on the battlefield");
                let blocker_can_block_flying = self.has_flying(blocker_permanent)
                    || self
                        .permanent_has_executable_keyword(blocker_permanent, KeywordAbility::Reach);
                let ironclaw =
                    self.effective_behavior(blocker_permanent) == Some(CardBehavior::IronclawOrcs);
                attackers
                    .iter()
                    .filter_map(move |(attacker, flying, unblockable, power)| {
                        let attacker_permanent = self
                            .battlefield
                            .iter()
                            .find(|permanent| permanent.card.id == *attacker)
                            .expect("attacker is on the battlefield");
                        let pixies = self
                            .battlefield
                            .iter()
                            .find(|permanent| permanent.card.id == *attacker)
                            .is_some_and(|permanent| {
                                self.effective_behavior(permanent)
                                    == Some(CardBehavior::ArgothianPixies)
                            });
                        let intimidate = self.permanent_has_executable_keyword(
                            attacker_permanent,
                            KeywordAbility::Intimidate,
                        );
                        let shares_color = self
                            .effective_rules(attacker_permanent)
                            .zip(self.effective_rules(blocker_permanent))
                            .is_some_and(|(attacker, blocker)| {
                                attacker
                                    .colors()
                                    .into_iter()
                                    .zip(blocker.colors())
                                    .any(|(attacker, blocker)| attacker && blocker)
                            });
                        let can_block = !(*unblockable
                            || attacker_permanent.unblockable_this_turn
                            || self.blocking_is_prevented(attacker_permanent, blocker_permanent)
                            || *flying && !blocker_can_block_flying
                            || intimidate
                                && !self.is_artifact_permanent(blocker_permanent)
                                && !shares_color
                            || ironclaw && *power >= 2
                            || pixies && self.is_artifact_permanent(blocker_permanent)
                            || self.combat_is_protected(blocker_permanent, attacker_permanent));
                        can_block.then_some(Action::DeclareBlocker {
                            blocker,
                            attacker: *attacker,
                        })
                    })
            })
            .collect()
    }

    fn declare_blocker(&mut self, blocker: GameObjectId, attacker: GameObjectId) {
        if let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == blocker)
        {
            permanent.blocking = Some(attacker);
        }
        if !self.combat_blocked_attackers.contains(&attacker) {
            self.combat_blocked_attackers.push(attacker);
        }
    }

    fn finish_declaring_blockers(&mut self) {
        self.blockers_declared = true;
        self.priority = self.active_player;
        self.consecutive_passes = 0;
        let blocked = self
            .battlefield
            .iter()
            .filter_map(|permanent| permanent.blocking)
            .collect::<Vec<_>>();
        for permanent in &mut self.battlefield {
            permanent.blocked = blocked.contains(&permanent.card.id);
        }
        let assignments = self
            .battlefield
            .iter()
            .filter_map(|permanent| {
                permanent
                    .blocking
                    .map(|attacker| (permanent.card.id, attacker))
            })
            .collect::<Vec<_>>();
        if !assignments.is_empty() {
            self.events.push(GameEvent::BlockDeclared {
                player: self.active_player.opponent(),
                assignments,
            });
        }
    }

    fn start_combat_damage(&mut self) {
        // Tests and a few internal procedures can construct combat directly,
        // so also capture live blocking relationships here. During an ordinary
        // game, `declare_blocker` recorded them before either player received
        // priority and they therefore survive a blocker leaving the field.
        let newly_blocked = self
            .battlefield
            .iter()
            .filter_map(|permanent| permanent.blocking)
            .collect::<Vec<_>>();
        for attacker in newly_blocked {
            if !self.combat_blocked_attackers.contains(&attacker) {
                self.combat_blocked_attackers.push(attacker);
            }
        }

        let strike_wave_combatants = self
            .battlefield
            .iter()
            .filter(|permanent| permanent.attacking || permanent.blocking.is_some())
            .filter(|permanent| {
                self.permanent_has_executable_keyword(permanent, KeywordAbility::FirstStrike)
                    || self
                        .permanent_has_executable_keyword(permanent, KeywordAbility::DoubleStrike)
            })
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>();
        self.combat_damage_stage = if strike_wave_combatants.is_empty() {
            CombatDamageStage::Single
        } else {
            CombatDamageStage::FirstStrike {
                strike_wave_combatants,
            }
        };
        self.begin_combat_damage_assignment();
    }

    fn begin_regular_combat_damage_after_first_strike(&mut self) {
        let CombatDamageStage::FirstStrike {
            strike_wave_combatants,
        } = &self.combat_damage_stage
        else {
            return;
        };
        self.combat_damage_stage = CombatDamageStage::RegularAfterFirstStrike {
            strike_wave_combatants: strike_wave_combatants.clone(),
        };
        self.begin_combat_damage_assignment();
    }

    fn deals_damage_in_current_combat_step(&self, permanent: &Permanent) -> bool {
        match &self.combat_damage_stage {
            CombatDamageStage::NotStarted | CombatDamageStage::Single => true,
            CombatDamageStage::FirstStrike {
                strike_wave_combatants,
            } => strike_wave_combatants.contains(&permanent.card.id),
            CombatDamageStage::RegularAfterFirstStrike {
                strike_wave_combatants,
            } => {
                !strike_wave_combatants.contains(&permanent.card.id)
                    || self
                        .permanent_has_executable_keyword(permanent, KeywordAbility::DoubleStrike)
            }
        }
    }

    fn begin_combat_damage_assignment(&mut self) {
        for permanent in &mut self.battlefield {
            permanent.combat_damage_assignment.clear();
        }
        self.pending_combat_attackers = self
            .battlefield
            .iter()
            .filter(|attacker| {
                attacker.attacking && self.deals_damage_in_current_combat_step(attacker)
            })
            // A single blocker leaves nothing worth deciding: it takes lethal
            // and, with trample, the rest spills over. Only a real split
            // between several blockers is worth asking about.
            .filter(|attacker| {
                self.battlefield
                    .iter()
                    .filter(|blocker| blocker.blocking == Some(attacker.card.id))
                    .count()
                    > 1
            })
            .map(|attacker| attacker.card.id)
            .collect();
        if self.pending_combat_attackers.is_empty() {
            self.deal_combat_damage();
        }
    }

    fn combat_assignment_actions(&self, attacker_id: GameObjectId) -> Vec<Action> {
        let Some(attacker) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == attacker_id)
        else {
            return Vec::new();
        };
        let power = self.power(attacker).unwrap_or(0).max(0).cast_unsigned();
        let trample = self.has_trample(attacker);
        let mut recipients: Vec<_> = self
            .battlefield
            .iter()
            .filter(|permanent| permanent.blocking == Some(attacker_id))
            .map(|permanent| Target::Permanent(permanent.card.id))
            .collect();
        recipients.sort_unstable();
        if trample {
            recipients.push(Target::Player(self.active_player.opponent()));
        }

        damage_distributions(recipients.len(), power)
            .into_iter()
            .filter(|amounts| {
                let blockers = || {
                    recipients
                        .iter()
                        .zip(amounts)
                        .filter_map(|(target, amount)| match target {
                            Target::Permanent(id) => Some((*id, *amount)),
                            Target::Player(_) | Target::Card(_) | Target::Spell(_) => None,
                        })
                };
                // 510.1c: damage is assigned in an order, and a blocker only
                // gets any once every blocker ahead of it has lethal. Whatever
                // order the player picks, that leaves at most one blocker
                // holding a non-lethal share.
                if blockers()
                    .filter(|(id, amount)| {
                        *amount > 0 && *amount < self.lethal_damage_from(*id, attacker_id)
                    })
                    .count()
                    > 1
                {
                    return false;
                }
                // 510.1d: trample only spills once every blocker has lethal.
                if !trample || amounts.last().copied().unwrap_or(0) == 0 {
                    return true;
                }
                blockers().all(|(id, amount)| amount >= self.lethal_damage_from(id, attacker_id))
            })
            .map(|amounts| Action::AssignCombatDamage {
                attacker: attacker_id,
                assignments: recipients
                    .iter()
                    .copied()
                    .zip(amounts)
                    .map(|(recipient, amount)| CombatDamageAssignment { recipient, amount })
                    .collect(),
            })
            .collect()
    }

    /// How an unassigned attacker spreads its damage: enough to kill each
    /// blocker in turn, then the remainder over the top if it tramples and
    /// onto the last blocker if it does not.
    fn default_damage_split(
        &self,
        attacker_id: GameObjectId,
        blockers: &[GameObjectId],
    ) -> Vec<(Target, u16)> {
        let Some(attacker) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == attacker_id)
        else {
            return Vec::new();
        };
        let mut remaining = self.power(attacker).unwrap_or(0).max(0).cast_unsigned();
        let trample = self.has_trample(attacker);
        let mut split = Vec::with_capacity(blockers.len() + 1);
        for blocker in blockers {
            let amount = self
                .lethal_damage_from(*blocker, attacker_id)
                .min(remaining);
            remaining -= amount;
            split.push((Target::Permanent(*blocker), amount));
        }
        if remaining > 0 {
            if trample {
                split.push((Target::Player(self.active_player.opponent()), remaining));
            } else if let Some(last) = split.last_mut() {
                last.1 += remaining;
            }
        }
        split
    }

    fn lethal_damage(&self, permanent_id: GameObjectId) -> u16 {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == permanent_id)
            .map_or(0, |permanent| {
                self.toughness(permanent)
                    .unwrap_or(0)
                    .max(0)
                    .cast_unsigned()
                    .saturating_sub(permanent.damage)
            })
    }

    fn lethal_damage_from(&self, permanent_id: GameObjectId, source: GameObjectId) -> u16 {
        let ordinary = self.lethal_damage(permanent_id);
        if ordinary > 0
            && self
                .source_controller_with_keyword(source, KeywordAbility::Deathtouch)
                .is_some()
        {
            1
        } else {
            ordinary
        }
    }

    fn assign_combat_damage(
        &mut self,
        attacker: GameObjectId,
        assignments: Vec<CombatDamageAssignment>,
    ) {
        if let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == attacker)
        {
            permanent.combat_damage_assignment = assignments;
        }
        self.pending_combat_attackers.remove(0);
        if self.pending_combat_attackers.is_empty() {
            self.deal_combat_damage();
        }
    }

    fn deal_combat_damage(&mut self) {
        let attackers: Vec<_> = self
            .battlefield
            .iter()
            .filter(|permanent| permanent.attacking)
            .map(|permanent| permanent.card.id)
            .collect();
        for attacker_id in attackers {
            let Some(attacker_index) = self
                .battlefield
                .iter()
                .position(|permanent| permanent.card.id == attacker_id)
            else {
                continue;
            };
            let power = self
                .power(&self.battlefield[attacker_index])
                .unwrap_or(0)
                .max(0)
                .cast_unsigned();
            let attacker_deals_damage =
                self.deals_damage_in_current_combat_step(&self.battlefield[attacker_index]);
            let blockers: Vec<_> = self
                .battlefield
                .iter()
                .filter(|permanent| permanent.blocking == Some(attacker_id))
                .map(|permanent| permanent.card.id)
                .collect();
            if attacker_deals_damage && blockers.is_empty() {
                let was_blocked = self.combat_blocked_attackers.contains(&attacker_id);
                if was_blocked && !self.has_trample(&self.battlefield[attacker_index]) {
                    continue;
                }
                self.damage_target_from(
                    Some(attacker_id),
                    Some(Target::Player(self.active_player.opponent())),
                    power,
                );
                match self.effective_behavior(&self.battlefield[attacker_index]) {
                    Some(CardBehavior::HypnoticSpecter) => {
                        self.discard_random(
                            self.active_player.opponent(),
                            1,
                            ZoneMoveCause::Effect {
                                controller: self.active_player,
                            },
                        );
                    }
                    Some(CardBehavior::WhirlingDervish) => {
                        self.battlefield[attacker_index]
                            .add_counters(CounterKind::PlusOnePlusOne, 1);
                    }
                    _ => {}
                }
            } else if !blockers.is_empty() {
                let assignments = self.battlefield[attacker_index]
                    .combat_damage_assignment
                    .clone();
                if attacker_deals_damage {
                    if assignments.is_empty() {
                        for (recipient, amount) in self.default_damage_split(attacker_id, &blockers)
                        {
                            self.damage_target_from(Some(attacker_id), Some(recipient), amount);
                        }
                    } else {
                        for assignment in assignments {
                            self.damage_target_from(
                                Some(attacker_id),
                                Some(assignment.recipient),
                                assignment.amount,
                            );
                        }
                    }
                }
                let return_damage = blockers
                    .iter()
                    .filter_map(|id| {
                        self.battlefield
                            .iter()
                            .find(|permanent| permanent.card.id == *id)
                            .filter(|permanent| self.deals_damage_in_current_combat_step(permanent))
                            .and_then(|permanent| self.power(permanent))
                            .map(|power| (*id, power.max(0).cast_unsigned()))
                    })
                    .collect::<Vec<_>>();
                for (blocker, amount) in return_damage {
                    self.damage_target_from(
                        Some(blocker),
                        Some(Target::Permanent(attacker_id)),
                        amount,
                    );
                }
            }
        }
        self.check_state_based_actions();
    }

    /// Every battlefield permanent whose printed name matches the chosen
    /// target's, the target included.
    fn objects_sharing_name_with_target(
        &self,
        slot: TargetSlotId,
        object: &StackObject,
        context: TriggerContext,
    ) -> Vec<Target> {
        let Some(name) = self
            .effect_recipients(EffectRecipientDef::Target(slot), object, context)
            .into_iter()
            .find_map(|target| match target {
                Target::Permanent(id) => self.permanent_card_name(id),
                _ => None,
            })
        else {
            return Vec::new();
        };
        self.battlefield
            .iter()
            .filter(|permanent| self.permanent_card_name(permanent.card.id) == Some(name))
            .map(|permanent| Target::Permanent(permanent.card.id))
            .collect()
    }

    /// The printed name of any object the engine can still find, wherever it
    /// is. Used by the cards that speak about names rather than identity.
    fn object_card_name(&self, id: GameObjectId) -> Option<&str> {
        self.permanent_card_name(id).or_else(|| {
            self.card_in_nonbattlefield_zone(id)
                .and_then(|(_, card)| self.catalog.get(card.definition))
                .map(|card| card.name.as_str())
        })
    }

    /// The copiable name a permanent presents, for the cards that gather
    /// everything sharing a name.
    fn permanent_card_name(&self, id: GameObjectId) -> Option<&str> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .and_then(|permanent| self.catalog.get(Self::effective_rules_source(permanent).0))
            .map(|card| card.name.as_str())
    }

    fn permanent_controller(&self, id: GameObjectId) -> Option<PlayerId> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .map(|permanent| permanent.controller)
    }

    /// Commits the untapped-to-tapped transition in one place so triggered
    /// abilities observe mana costs, activated-ability costs, combat, and
    /// resolving tap effects through the same event path.
    fn tap_permanent(&mut self, id: GameObjectId) -> Option<CardInstance> {
        let (card, event, was_tapped) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .map(|permanent| {
                (
                    permanent.card.clone(),
                    self.trigger_event_object(permanent),
                    permanent.tapped,
                )
            })?;
        if !was_tapped {
            self.battlefield
                .iter_mut()
                .find(|permanent| permanent.card.id == id)
                .expect("the observed permanent remains on the battlefield")
                .tapped = true;
            self.capture_battlefield_triggers(&CommittedTriggerEvent::BecomesTapped {
                object: event,
            });
        }
        Some(card)
    }

    fn destroy_permanent(&mut self, id: GameObjectId) {
        self.destroy_permanents(&[id], true);
    }

    fn destroy_permanent_without_regeneration(&mut self, id: GameObjectId) {
        self.destroy_permanents(&[id], false);
    }

    fn sacrifice_permanent(&mut self, id: GameObjectId) {
        self.destroy_permanent_without_regeneration(id);
    }

    fn destroy_permanents(&mut self, ids: &[GameObjectId], can_regenerate: bool) {
        let mut seen = Vec::new();
        let mut doomed = Vec::new();
        for &id in ids {
            if seen.contains(&id) {
                continue;
            }
            seen.push(id);
            let Some(permanent) = self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == id)
            else {
                continue;
            };
            if can_regenerate && permanent.regeneration_shields > 0 {
                self.regenerate_permanent(id);
            } else {
                doomed.push(id);
            }
        }
        self.move_permanents_to_graveyard(&doomed);
    }

    fn regenerate_permanent(&mut self, id: GameObjectId) {
        let Some(index) = self
            .battlefield
            .iter()
            .position(|permanent| permanent.card.id == id)
        else {
            return;
        };
        {
            let permanent = &mut self.battlefield[index];
            permanent.regeneration_shields -= 1;
            permanent.damage = 0;
            permanent.damage_sources.clear();
            permanent.deathtouch_damage = false;
            permanent.attacking = false;
            permanent.blocked = false;
            permanent.blocking = None;
            permanent.combat_damage_assignment.clear();
        }
        let _ = self.tap_permanent(id);
        for other in &mut self.battlefield {
            if other.card.id != id && other.blocking == Some(id) {
                other.blocking = None;
            }
        }
    }

    /// Moves one simultaneous batch to graveyards. Listener declarations and
    /// last-known characteristics are frozen before any member leaves, then all
    /// old object incarnations are retired before the individual zone-change
    /// events are published.
    fn move_permanents_to_graveyard(&mut self, ids: &[GameObjectId]) {
        let listeners = self.battlefield_trigger_listeners();
        let mut seen = Vec::new();
        let exits = ids
            .iter()
            .filter_map(|id| {
                if seen.contains(id) {
                    return None;
                }
                seen.push(*id);
                self.battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == *id)
                    .map(|permanent| {
                        (
                            *id,
                            self.battlefield_exit_snapshot(permanent),
                            permanent.damage_sources.clone(),
                            permanent.exile_instead_of_dying,
                            self.has_undying(permanent)
                                && permanent.counters(CounterKind::PlusOnePlusOne) == 0,
                            permanent.presented,
                        )
                    })
            })
            .collect::<Vec<_>>();

        self.creature_died_this_turn |=
            exits.iter().any(|(_, snapshot, _, exile_instead, _, _)| {
                !exile_instead && snapshot.object.types.is_creature()
            });
        for (_, snapshot, damage_sources, exile_instead, _, _) in &exits {
            if *exile_instead {
                continue;
            }
            for &source in damage_sources {
                self.capture_battlefield_triggers_from_snapshot(
                    &listeners,
                    &CommittedTriggerEvent::DamagedCreatureDied {
                        object: snapshot.object.clone(),
                        source,
                    },
                );
            }
        }

        let mut removed = Vec::new();
        for (id, snapshot, _, exile_instead, undying, presented) in exits {
            let index = self
                .battlefield
                .iter()
                .position(|permanent| permanent.card.id == id)
                .expect("a snapshotted battlefield object remains until its batch exits");
            let permanent = self.remove_battlefield_object(index, &snapshot.last_known);
            removed.push((permanent, snapshot, exile_instead, undying, presented));
        }

        for (permanent, snapshot, exile_instead, undying, presented) in removed {
            let (to, destination) = if exile_instead {
                (ZoneKind::Exile, BattlefieldExit::Exile)
            } else {
                (ZoneKind::Graveyard, BattlefieldExit::Graveyard)
            };
            let event = CommittedTriggerEvent::ZoneChanged {
                object: snapshot.object,
                from: ZoneKind::Battlefield,
                to,
            };
            self.capture_battlefield_triggers_from_snapshot(&listeners, &event);
            self.capture_custom_source_triggers(&permanent, &snapshot.abilities, &event);
            self.record_battlefield_exit(&permanent, destination);
            // 111.7: a token that leaves the battlefield ceases to exist. The
            // exit and everything watching for it still happened.
            if self.is_token(permanent.card.definition) {
                continue;
            }
            let owner = permanent.card.owner;
            let (card, _zone_change) = self.zone_change_card(permanent.card);
            if exile_instead {
                self.players[owner.index()].exile.push(card);
                continue;
            }
            self.players[owner.index()].graveyard.push(card);

            // Undying observes the creature as it died, then returns the card
            // from the graveyard as a fresh object under its owner's control.
            if undying {
                self.return_top_graveyard_card_with_undying(owner, presented);
            }
        }
    }

    fn return_top_graveyard_card_with_undying(&mut self, owner: PlayerId, presented: CardPartId) {
        let Some(card) = self.players[owner.index()].graveyard.pop() else {
            return;
        };
        let (card, _zone_change) = self.zone_change_card(card);
        self.battlefield.push(Permanent {
            card,
            presented,
            controller: owner,
            tapped: false,
            entered_controller_turn: self.turns_started[owner.index()],
            damage: 0,
            loyalty: None,
            power_bonus: 0,
            toughness_bonus: 0,
            attacking: false,
            unblockable_this_turn: false,
            blocked: false,
            blocking: None,
            chosen_player: None,
            chosen_creature_type: None,
            destroy_at_end: false,
            temporary_keywords: Vec::new(),
            factory_animated: false,
            dragon_whelp_activations: 0,
            counters: {
                let mut counters = [0; CounterKind::COUNT];
                counters[CounterKind::PlusOnePlusOne.index()] = 1;
                counters
            },
            attached_to: None,
            exile_instead_of_dying: false,
            combat_damage_assignment: Vec::new(),
            copy_effect: None,
            copied_from: None,
            text_changes: Vec::new(),
            regeneration_shields: 0,
            berserked: false,
            attacked_this_turn: false,
            forestwalk_until_upkeep_of: None,
            damage_sources: Vec::new(),
            deathtouch_damage: false,
        });
    }

    fn record_battlefield_exit(&mut self, permanent: &Permanent, destination: BattlefieldExit) {
        self.events.push(GameEvent::PermanentLeftBattlefield {
            controller: permanent.controller,
            card: permanent.card.id,
            definition: permanent.card.definition,
            destination,
        });
    }

    fn exile_permanent(&mut self, id: GameObjectId) {
        let listeners = self.battlefield_trigger_listeners();
        let Some(index) = self
            .battlefield
            .iter()
            .position(|permanent| permanent.card.id == id)
        else {
            return;
        };
        let snapshot = self.battlefield_exit_snapshot(&self.battlefield[index]);
        let permanent = self.remove_battlefield_object(index, &snapshot.last_known);
        let event = CommittedTriggerEvent::ZoneChanged {
            object: snapshot.object,
            from: ZoneKind::Battlefield,
            to: ZoneKind::Exile,
        };
        self.capture_battlefield_triggers_from_snapshot(&listeners, &event);
        self.capture_custom_source_triggers(&permanent, &snapshot.abilities, &event);
        self.record_battlefield_exit(&permanent, BattlefieldExit::Exile);
        if self.is_token(permanent.card.definition) {
            return;
        }
        let owner = permanent.card.owner;
        let (card, _zone_change) = self.zone_change_card(permanent.card);
        self.players[owner.index()].exile.push(card);
    }

    /// Exiles a permanent and reports the object it became in exile, so the
    /// clause that promised to return it can remember which card that is.
    fn exile_permanent_returning_card(&mut self, id: GameObjectId) -> Option<GameObjectId> {
        let owner = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .map(|permanent| permanent.card.owner)?;
        let before = self.players[owner.index()].exile.len();
        self.exile_permanent(id);
        self.players[owner.index()]
            .exile
            .get(before)
            .map(|card| card.id)
    }

    /// Exiles a card from wherever it is outside the battlefield, reporting
    /// the object it became so the link can be recorded.
    fn exile_card_returning_card(&mut self, id: GameObjectId) -> Option<GameObjectId> {
        let (zone, owner) = self
            .card_in_nonbattlefield_zone(id)
            .map(|(zone, card)| (zone, card.owner))?;
        if zone == ZoneKind::Exile {
            return None;
        }
        let card = self.take_card_from_zone(owner, zone, id)?;
        let (card, _zone_change) = self.zone_change_card(card);
        let exiled = card.id;
        self.players[owner.index()].exile.push(card);
        Some(exiled)
    }

    /// Removes a card from one of a player's non-battlefield zones.
    fn take_card_from_zone(
        &mut self,
        owner: PlayerId,
        zone: ZoneKind,
        id: GameObjectId,
    ) -> Option<CardInstance> {
        let state = &mut self.players[owner.index()];
        let cards = match zone {
            ZoneKind::Library => &mut state.library,
            ZoneKind::Hand => &mut state.hand,
            ZoneKind::Graveyard => &mut state.graveyard,
            ZoneKind::Exile => &mut state.exile,
            ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => return None,
        };
        remove_card(cards, id)
    }

    /// Brings a linked exile back. A card that is no longer in exile has
    /// moved on, and nothing follows it.
    fn return_exiled_card(
        &mut self,
        id: GameObjectId,
        zone: ZoneKind,
        grant: Option<KeywordAbility>,
    ) {
        let Some(owner) = [PlayerId::One, PlayerId::Two].into_iter().find(|player| {
            self.players[player.index()]
                .exile
                .iter()
                .any(|card| card.id == id)
        }) else {
            return;
        };
        let Some(card) = remove_card(&mut self.players[owner.index()].exile, id) else {
            return;
        };
        if zone == ZoneKind::Battlefield {
            let entered = self.put_card_onto_battlefield_from(card, ZoneKind::Exile, owner);
            if let Some(keyword) = grant
                && let Some(permanent) = self
                    .battlefield
                    .iter_mut()
                    .find(|permanent| permanent.card.id == entered.id)
            {
                permanent.temporary_keywords.push(keyword);
            }
        } else {
            let (card, _zone_change) = self.zone_change_card(card);
            self.players[owner.index()].hand.push(card);
        }
    }

    /// Raises the start-of-step event and resolves whatever was waiting for
    /// it. The upkeep has its own richer path and calls both itself.
    fn begin_step_triggers(&mut self) {
        if self.step == Step::Upkeep {
            return;
        }
        let step = Self::turn_step_def(self.step);
        self.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
            step,
            player: self.active_player,
        });
        self.fire_delayed_triggers(step);
    }

    /// Resolves the effects that were waiting for this step.
    ///
    /// A real delayed trigger goes on the stack and can be responded to. This
    /// resolves at the step boundary instead, which no card here can tell
    /// apart, and keeps the queue from needing a listener of its own.
    fn fire_delayed_triggers(&mut self, step: TurnStepDef) {
        let active = self.active_player;
        let mut due = Vec::new();
        let mut waiting = Vec::new();
        for delayed in std::mem::take(&mut self.delayed_triggers) {
            let is_due = delayed.step == step
                && self.player_relation_matches(
                    active,
                    delayed.player,
                    delayed.object.controller,
                    TriggerContext::empty(),
                );
            if is_due {
                due.push(delayed);
            } else {
                waiting.push(delayed);
            }
        }
        self.delayed_triggers = waiting;
        for delayed in due {
            self.resolve_effect_def(*delayed.effect, &delayed.object, TriggerContext::empty());
        }
    }

    fn return_permanent_to_hand(&mut self, id: GameObjectId) {
        let listeners = self.battlefield_trigger_listeners();
        let Some(index) = self
            .battlefield
            .iter()
            .position(|permanent| permanent.card.id == id)
        else {
            return;
        };
        let snapshot = self.battlefield_exit_snapshot(&self.battlefield[index]);
        let permanent = self.remove_battlefield_object(index, &snapshot.last_known);
        let event = CommittedTriggerEvent::ZoneChanged {
            object: snapshot.object,
            from: ZoneKind::Battlefield,
            to: ZoneKind::Hand,
        };
        self.capture_battlefield_triggers_from_snapshot(&listeners, &event);
        self.capture_custom_source_triggers(&permanent, &snapshot.abilities, &event);
        self.record_battlefield_exit(&permanent, BattlefieldExit::Hand);
        if self.is_token(permanent.card.definition) {
            return;
        }
        let owner = permanent.card.owner;
        let (card, _zone_change) = self.zone_change_card(permanent.card);
        self.players[owner.index()].hand.push(card);
    }

    /// Puts a permanent on top of its owner's library. The exit is the same
    /// procedure a bounce uses; only the destination differs.
    fn return_permanent_to_library_top(&mut self, id: GameObjectId) {
        let listeners = self.battlefield_trigger_listeners();
        let Some(index) = self
            .battlefield
            .iter()
            .position(|permanent| permanent.card.id == id)
        else {
            return;
        };
        let snapshot = self.battlefield_exit_snapshot(&self.battlefield[index]);
        let permanent = self.remove_battlefield_object(index, &snapshot.last_known);
        let event = CommittedTriggerEvent::ZoneChanged {
            object: snapshot.object,
            from: ZoneKind::Battlefield,
            to: ZoneKind::Library,
        };
        self.capture_battlefield_triggers_from_snapshot(&listeners, &event);
        self.capture_custom_source_triggers(&permanent, &snapshot.abilities, &event);
        self.record_battlefield_exit(&permanent, BattlefieldExit::LibraryTop);
        if self.is_token(permanent.card.definition) {
            return;
        }
        let owner = permanent.card.owner;
        let (card, _zone_change) = self.zone_change_card(permanent.card);
        self.players[owner.index()].library.push(card);
    }

    /// True when a spell had targets and every one of them is now illegal.
    fn spell_fizzles(&self, object: &StackObject) -> bool {
        if object.target_count() == 0 {
            return false;
        }
        if object.ability.is_some() {
            return self.stack_ability_fizzles(object);
        }
        if let Some(signature) = &object.signature
            && let Some(definition) = self.catalog.get(object.card.definition)
            && let Some(option) = definition.play_option(signature.play_option())
        {
            let slots = Self::target_slots_for(option, signature.modes());
            if !slots.is_empty() || option.modes.is_some() || !option.targets.is_empty() {
                return signature
                    .targets()
                    .iter()
                    .zip(slots)
                    .flat_map(|(selection, slot)| {
                        selection
                            .targets()
                            .iter()
                            .map(move |target| (slot.predicate, *target))
                    })
                    .all(|(predicate, target)| !self.target_matches(predicate, target));
            }
        }
        object.iter_targets().all(|target| match target {
            Target::Player(_) => false,
            Target::Card(id) => self.card_in_nonbattlefield_zone(*id).is_none(),
            Target::Permanent(id) => !self
                .battlefield
                .iter()
                .any(|permanent| permanent.card.id == *id),
            Target::Spell(id) => !self.stack.iter().any(|candidate| candidate.id == *id),
        })
    }

    fn effect_applies_to_source(
        effect: EffectDef,
        expected: AppliedEffectDef,
        duration: EffectDurationDef,
    ) -> bool {
        match effect {
            EffectDef::Sequence(effects) => effects
                .iter()
                .any(|effect| Self::effect_applies_to_source(*effect, expected, duration)),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect,
                duration: actual_duration,
            } => Self::applied_effect_contains(effect, expected) && actual_duration == duration,
            EffectDef::None
            | EffectDef::AddMana(_)
            | EffectDef::DealDamage { .. }
            | EffectDef::GainLife { .. }
            | EffectDef::DrawCards { .. }
            | EffectDef::DiscardCards { .. }
            | EffectDef::LoseLife { .. }
            | EffectDef::Tap { .. }
            | EffectDef::Untap { .. }
            | EffectDef::Attach { .. }
            | EffectDef::Destroy { .. }
            | EffectDef::Sacrifice { .. }
            | EffectDef::SacrificeOfChoice { .. }
            | EffectDef::Counter { .. }
            | EffectDef::CounterUnlessPaid { .. }
            | EffectDef::AddCounters { .. }
            | EffectDef::ChangeTextBasicLandType { .. }
            | EffectDef::BecomeCopyOf { .. }
            | EffectDef::OptionalManaPayment { .. }
            | EffectDef::May(_)
            | EffectDef::EntersTapped
            | EffectDef::CannotBeForcedToSacrifice
            | EffectDef::GrantFlashToNextSorcery
            | EffectDef::ExileLinkedToSource { .. }
            | EffectDef::ReturnLinkedExiles { .. }
            | EffectDef::MakeUnblockableThisTurn { .. }
            | EffectDef::AtNextStep { .. }
            | EffectDef::ReduceGenericCostBy(_)
            | EffectDef::MultiplyEventAmount(_)
            | EffectDef::MoveToZone { .. }
            | EffectDef::CreateToken { .. }
            | EffectDef::ChooseCreatureType { .. }
            | EffectDef::Apply { .. }
            | EffectDef::Special(_) => false,
        }
    }

    fn applied_effect_contains(effect: AppliedEffectDef, expected: AppliedEffectDef) -> bool {
        effect == expected
            || matches!(
                effect,
                AppliedEffectDef::Composite(effects)
                    if effects
                        .iter()
                        .any(|effect| Self::applied_effect_contains(*effect, expected))
            )
    }

    fn stack_spell_has_static_effect(
        &self,
        object: &StackObject,
        expected: AppliedEffectDef,
    ) -> bool {
        let Some(signature) = &object.signature else {
            return false;
        };
        let Some(definition) = self.catalog.get(object.card.definition) else {
            return false;
        };
        let Ok(parts) = applicable_part_ids(
            definition,
            &CharacteristicContext::Stack {
                form: signature.form().clone(),
            },
        ) else {
            return false;
        };
        parts.into_iter().any(|part| {
            definition.part(part).is_some_and(|part| {
                part.rules.ability_clauses().iter().any(|ability| {
                    ability.implementation.is_executable()
                        && matches!(
                            ability.definition,
                            DeclarativeAbilityDef::Static(definition)
                                if definition.source_zones.contains(&ZoneKind::Stack)
                        )
                        && Self::effect_applies_to_source(
                            ability.effect,
                            expected,
                            EffectDurationDef::WhileSourceRemainsInZone,
                        )
                })
            })
        })
    }

    /// Whether a spell on the stack can be countered at all. Printed static
    /// abilities and effects carried by mana converge here; neither changes
    /// whether the spell is a legal target.
    fn can_be_countered(&self, object: &StackObject) -> bool {
        !self.stack_spell_has_static_effect(object, AppliedEffectDef::CannotBeCountered)
            && !object.applied_effects.iter().any(|applied| {
                Self::applied_effect_contains(applied.effect, AppliedEffectDef::CannotBeCountered)
            })
    }

    fn counter_spell(&mut self, id: GameObjectId) {
        self.counter_spell_into(id, CounteredSpellZone::Graveyard);
    }

    /// A countered spell normally goes to its owner's graveyard, but several
    /// cards exile it instead so it cannot be rebought.
    fn counter_spell_into(&mut self, id: GameObjectId, zone: CounteredSpellZone) {
        let Some(index) = self.stack.iter().position(|object| object.id == id) else {
            return;
        };
        // "Can't be countered" is not "can't be targeted": a Counterspell may
        // legally target Supreme Verdict, resolve, and accomplish nothing. So
        // this is the only place that checks, and the target lists do not.
        if !self.can_be_countered(&self.stack[index]) {
            return;
        }
        let object = self.stack.remove(index);
        self.retire_stack_object(&object);
        if object.kind == StackObjectKind::Spell && !object.is_copy {
            let owner = object.card.owner;
            let (card, _zone_change) = self.zone_change_card(object.card);
            match if object.cast_via_flashback {
                CounteredSpellZone::Exile
            } else {
                zone
            } {
                CounteredSpellZone::Graveyard => self.players[owner.index()].graveyard.push(card),
                CounteredSpellZone::Exile => self.players[owner.index()].exile.push(card),
            }
        }
    }

    fn check_state_based_actions(&mut self) {
        loop {
            let battlefield_len = self.battlefield.len();
            let mut regenerate = Vec::new();
            let mut die = Vec::new();
            for permanent in &self.battlefield {
                // 704.5m: an Aura attached to nothing, or to something that is
                // no longer a legal host, is put into its owner's graveyard.
                if self.is_aura_permanent(permanent)
                    && permanent
                        .attached_to
                        .is_none_or(|host| !self.is_legal_aura_host(permanent, host))
                {
                    die.push(permanent.card.id);
                    continue;
                }
                if permanent.loyalty.is_some_and(|loyalty| loyalty <= 0) {
                    die.push(permanent.card.id);
                    continue;
                }
                let Some(toughness) = self.toughness(permanent) else {
                    continue;
                };
                let zero_toughness = toughness <= 0;
                let lethal_damage = i32::from(permanent.damage) >= i32::from(toughness)
                    || (permanent.damage > 0 && permanent.deathtouch_damage);
                if !zero_toughness && !lethal_damage {
                    continue;
                }
                if !zero_toughness && permanent.regeneration_shields > 0 {
                    regenerate.push(permanent.card.id);
                } else {
                    die.push(permanent.card.id);
                }
            }
            for id in regenerate {
                self.regenerate_permanent(id);
            }
            self.move_permanents_to_graveyard(&die);
            self.apply_legend_rule();
            if self.battlefield.len() == battlefield_len {
                break;
            }
        }
        self.check_life_totals();
    }

    /// The legend rule as a state-based action: a player controlling two or
    /// more legendary permanents with the same name keeps one and puts the
    /// rest into the graveyard. The rules let the controller choose; with
    /// identical names the copies differ only in tap and damage state, so the
    /// strictly best one — untapped over tapped, then newest — is kept
    /// without asking.
    fn apply_legend_rule(&mut self) {
        loop {
            let mut extra: Option<GameObjectId> = None;
            'search: for permanent in &self.battlefield {
                if !self
                    .effective_rules(permanent)
                    .is_some_and(|rules| rules.has_supertype(CardSupertype::Legendary))
                {
                    continue;
                }
                let name_source = Self::effective_rules_source(permanent);
                for other in &self.battlefield {
                    if other.card.id == permanent.card.id
                        || other.controller != permanent.controller
                        || Self::effective_rules_source(other) != name_source
                    {
                        continue;
                    }
                    let permanent_wins = (!permanent.tapped && other.tapped)
                        || (permanent.tapped == other.tapped
                            && permanent.card.id.0 > other.card.id.0);
                    extra = Some(if permanent_wins {
                        other.card.id
                    } else {
                        permanent.card.id
                    });
                    break 'search;
                }
            }
            let Some(extra) = extra else {
                return;
            };
            self.destroy_permanent_without_regeneration(extra);
        }
    }

    fn untap_actions(&self, player: PlayerId) -> Vec<Action> {
        let lands: Vec<_> = self
            .battlefield
            .iter()
            .filter(|permanent| {
                permanent.controller == player
                    && permanent.tapped
                    && self
                        .permanent_types(permanent)
                        .is_some_and(|types| types.contains(CardType::Land))
            })
            .map(|permanent| permanent.card.id)
            .collect();
        let creatures: Vec<_> = self
            .battlefield
            .iter()
            .filter(|permanent| {
                permanent.controller == player
                    && permanent.tapped
                    && self.power(permanent).is_some()
            })
            .map(|permanent| permanent.card.id)
            .collect();
        let land_choices = if self.winter_orb_active() {
            one_or_none(&lands)
        } else {
            vec![lands]
        };
        let creature_choices = if self.count_behavior(CardBehavior::Smoke) > 0 {
            one_or_none(&creatures)
        } else {
            vec![creatures]
        };
        let mut actions = Vec::new();
        for land in &land_choices {
            for creature in &creature_choices {
                let mut permanents = land.clone();
                permanents.extend(creature);
                permanents.sort_unstable();
                permanents.dedup();
                actions.push(Action::ChooseUntap { permanents });
            }
        }
        actions
    }

    fn choose_untap(&mut self, player: PlayerId, selected: &[GameObjectId]) {
        for permanent in &mut self.battlefield {
            if permanent.controller == player && selected.contains(&permanent.card.id) {
                permanent.tapped = false;
            }
        }
        self.untap_pending = false;
        self.priority = self.active_player;
        self.finish_untap_choices();
    }

    /// Commits every life gain in one place so replacement and triggered
    /// abilities observe spells, lifelink, and card-specific drains through
    /// the same event path. Gaining nothing is not a life-gain event.
    fn gain_life(&mut self, player: PlayerId, amount: u16) {
        if amount == 0 {
            return;
        }
        let amount = amount.saturating_mul(self.life_gain_multiplier(player));
        self.players[player.index()].life = self.players[player.index()]
            .life
            .saturating_add(i16::try_from(amount).unwrap_or(i16::MAX));
        self.capture_battlefield_triggers(&CommittedTriggerEvent::LifeGained { player, amount });
    }

    /// How much a life gain is scaled by the replacement effects on the
    /// battlefield. CR 616.1 lets the affected player order these, but the
    /// order of pure multipliers cannot change their product.
    fn life_gain_multiplier(&self, player: PlayerId) -> u16 {
        let mut multiplier = 1u16;
        for permanent in &self.battlefield {
            self.for_each_effective_ability(permanent, |effective| {
                let ability = effective.ability;
                let DeclarativeAbilityDef::Replacement(definition) = ability.definition else {
                    return;
                };
                let ReplacementEventDef::WouldGainLife(relation) = definition.event else {
                    return;
                };
                let EffectDef::MultiplyEventAmount(factor) = ability.effect else {
                    return;
                };
                if ability.implementation.is_executable()
                    && self.player_relation_matches(
                        player,
                        relation,
                        permanent.controller,
                        TriggerContext::empty(),
                    )
                {
                    multiplier = multiplier.saturating_mul(u16::from(factor));
                }
            });
        }
        multiplier
    }

    /// Life loss that is not damage: no source deals it, nothing that
    /// triggers on damage sees it, and prevention does not apply.
    fn lose_life(&mut self, player: PlayerId, amount: u16) {
        let amount_as_i16 = i16::try_from(amount).unwrap_or(i16::MAX);
        self.players[player.index()].life -= amount_as_i16;
        self.events.push(GameEvent::LifeLost { player, amount });
    }

    fn deal_damage(&mut self, player: PlayerId, amount: u16) {
        let amount_as_i16 = i16::try_from(amount).unwrap_or(i16::MAX);
        self.players[player.index()].life -= amount_as_i16;
        self.events.push(GameEvent::DamageDealt { player, amount });
    }

    const fn turn_step_def(step: Step) -> TurnStepDef {
        match step {
            Step::Upkeep => TurnStepDef::Upkeep,
            Step::Draw => TurnStepDef::Draw,
            Step::PrecombatMain => TurnStepDef::PrecombatMain,
            Step::BeginningOfCombat => TurnStepDef::BeginningOfCombat,
            Step::DeclareAttackers => TurnStepDef::DeclareAttackers,
            Step::DeclareBlockers => TurnStepDef::DeclareBlockers,
            Step::CombatDamage => TurnStepDef::CombatDamage,
            Step::EndOfCombat => TurnStepDef::EndOfCombat,
            Step::PostcombatMain => TurnStepDef::PostcombatMain,
            Step::End => TurnStepDef::End,
            Step::Cleanup => TurnStepDef::Cleanup,
        }
    }

    fn advance_step(&mut self) {
        if self.step.ends_phase() || self.format.rules().mana_empties_at_end_of_step {
            self.empty_mana_pools();
            if self.result.is_some() {
                return;
            }
        }

        match self.step {
            Step::Upkeep => {
                self.step = Step::Draw;
                let vault_damage = u16::try_from(
                    self.battlefield
                        .iter()
                        .filter(|permanent| {
                            permanent.controller == self.active_player
                                && permanent.tapped
                                && self.effective_behavior(permanent)
                                    == Some(CardBehavior::ManaVault)
                        })
                        .count(),
                )
                .unwrap_or(u16::MAX);
                if vault_damage > 0 {
                    self.deal_damage(self.active_player, vault_damage);
                    self.check_life_totals();
                    if self.result.is_some() {
                        return;
                    }
                }
                if !(self.turn == 1 && self.active_player == PlayerId::One) {
                    let mut drawn = self
                        .draw_card(self.active_player)
                        .into_iter()
                        .collect::<Vec<_>>();
                    if self.battlefield.iter().any(|permanent| {
                        permanent.controller == self.active_player
                            && self.effective_behavior(permanent)
                                == Some(CardBehavior::SylvanLibrary)
                    }) && self.result.is_none()
                    {
                        if let Some(card) = self.draw_card(self.active_player) {
                            drawn.push(card);
                        }
                        if let Some(card) = self.draw_card(self.active_player) {
                            drawn.push(card);
                        }
                        if drawn.len() >= 2 && self.result.is_none() {
                            self.queue_sylvan_select(self.active_player, drawn, 2);
                        }
                    }
                }
            }
            Step::Draw => {
                self.step = Step::PrecombatMain;
                let amount =
                    std::mem::take(&mut self.mana_drain_pending[self.active_player.index()]);
                self.add_unrestricted_mana(self.active_player, ManaColor::Colorless, amount);
            }
            Step::PrecombatMain => self.step = Step::BeginningOfCombat,
            Step::BeginningOfCombat => {
                self.step = Step::DeclareAttackers;
                self.attackers_declared = false;
            }
            Step::DeclareAttackers => {
                self.step = Step::DeclareBlockers;
                self.blockers_declared = false;
            }
            Step::DeclareBlockers => {
                self.step = Step::CombatDamage;
                self.start_combat_damage();
            }
            Step::CombatDamage => self.advance_combat_damage_step(),
            Step::EndOfCombat => {
                self.clear_combat();
                self.step = Step::PostcombatMain;
            }
            Step::PostcombatMain => {
                self.step = Step::End;
                self.handle_end_step();
            }
            Step::End => {
                self.step = Step::Cleanup;
                self.cleanup();
            }
            Step::Cleanup => self.start_next_turn(),
        }

        if self.result.is_none() {
            self.begin_step_triggers();
            self.priority = self.active_player;
            self.events.push(GameEvent::StepChanged {
                turn: self.turn,
                active_player: self.active_player,
                step: self.step,
            });
        }
    }

    fn advance_combat_damage_step(&mut self) {
        if matches!(
            &self.combat_damage_stage,
            CombatDamageStage::FirstStrike { .. }
        ) {
            self.begin_regular_combat_damage_after_first_strike();
        } else {
            self.step = Step::EndOfCombat;
        }
    }

    fn start_next_turn(&mut self) {
        self.turn += 1;
        let mut next_player = self
            .extra_turns
            .pop()
            .unwrap_or_else(|| self.active_player.opponent());
        while self.skipped_turns[next_player.index()] > 0 {
            self.skipped_turns[next_player.index()] -= 1;
            let skipped = next_player;
            next_player = self.extra_turns.pop().unwrap_or_else(|| skipped.opponent());
        }
        self.active_player = next_player;
        self.turns_started[self.active_player.index()] += 1;
        self.creature_died_this_turn = false;
        self.sorcery_flash_grants = [0; 2];
        self.step = Step::Upkeep;
        self.players[self.active_player.index()].land_played_this_turn = false;
        for permanent in &mut self.battlefield {
            if permanent.forestwalk_until_upkeep_of == Some(self.active_player) {
                permanent.forestwalk_until_upkeep_of = None;
            }
        }
        let winter_orb = self.winter_orb_active();
        let smoke = self.count_behavior(CardBehavior::Smoke) > 0;
        let restricted_lands: Vec<_> = self
            .battlefield
            .iter()
            .filter(|permanent| {
                self.permanent_types(permanent)
                    .is_some_and(|types| types.contains(CardType::Land))
            })
            .map(|permanent| permanent.card.id)
            .collect();
        let restricted_creatures: Vec<_> = self
            .battlefield
            .iter()
            .filter(|permanent| self.power(permanent).is_some())
            .map(|permanent| permanent.card.id)
            .collect();
        let mana_vaults: Vec<_> = self
            .battlefield
            .iter()
            .filter(|permanent| {
                matches!(
                    self.effective_behavior(permanent),
                    Some(CardBehavior::ManaVault | CardBehavior::TimeVault)
                )
            })
            .map(|permanent| permanent.card.id)
            .collect();
        self.untap_pending = false;
        for permanent in &mut self.battlefield {
            if permanent.controller == self.active_player {
                let restricted = (winter_orb && restricted_lands.contains(&permanent.card.id))
                    || (smoke && restricted_creatures.contains(&permanent.card.id));
                if restricted && permanent.tapped {
                    self.untap_pending = true;
                } else if !mana_vaults.contains(&permanent.card.id) {
                    permanent.tapped = false;
                }
            }
        }
        if !self.untap_pending {
            self.finish_untap_choices();
        }
    }

    #[allow(clippy::too_many_lines)]
    fn handle_upkeep_triggers(&mut self) {
        let player = self.active_player;
        self.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
            step: TurnStepDef::Upkeep,
            player,
        });
        self.fire_delayed_triggers(TurnStepDef::Upkeep);
        let vise_damage: u16 = self
            .battlefield
            .iter()
            .filter(|permanent| {
                self.effective_behavior(permanent) == Some(CardBehavior::BlackVise)
                    && permanent.chosen_player == Some(player)
            })
            .map(|_| {
                u16::try_from(self.players[player.index()].hand.len().saturating_sub(4))
                    .unwrap_or(u16::MAX)
            })
            .sum();
        if vise_damage > 0 {
            self.deal_damage(player, vise_damage);
        }
        if self.count_behavior(CardBehavior::TheAbyss) > 0 {
            let target = self
                .battlefield
                .iter()
                .filter(|permanent| {
                    self.power(permanent).is_some() && !self.is_artifact_permanent(permanent)
                })
                .map(|permanent| permanent.card.id)
                .min();
            if let Some(target) = target {
                self.destroy_permanent(target);
            }
        }
        let erhnams = self
            .battlefield
            .iter()
            .filter(|permanent| {
                permanent.controller == player
                    && self.effective_behavior(permanent) == Some(CardBehavior::ErhnamDjinn)
            })
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>();
        for source in erhnams {
            self.queue_erhnam_decision(player, source);
        }
        if self.count_behavior(CardBehavior::CityInABottle) > 0 {
            let doomed: Vec<_> = self
                .battlefield
                .iter()
                .filter(|permanent| {
                    self.behavior(permanent.card.definition) != Some(CardBehavior::CityInABottle)
                        && self
                            .catalog
                            .get(permanent.card.definition)
                            .is_some_and(|card| card.debut_set == CardSet::ArabianNights)
                })
                .map(|permanent| permanent.card.id)
                .collect();
            for permanent in doomed {
                self.destroy_permanent(permanent);
            }
        }
        let tapped_vaults: Vec<_> = self
            .battlefield
            .iter()
            .filter(|permanent| {
                permanent.controller == player
                    && permanent.tapped
                    && self.effective_behavior(permanent) == Some(CardBehavior::ManaVault)
            })
            .map(|permanent| permanent.card.id)
            .collect();
        for permanent in tapped_vaults {
            self.queue_mana_vault_decision(player, permanent);
        }
        self.check_life_totals();
    }

    fn handle_end_step(&mut self) {
        let doomed: Vec<_> = self
            .battlefield
            .iter()
            .filter(|permanent| {
                permanent.destroy_at_end || permanent.berserked && permanent.attacked_this_turn
            })
            .map(|permanent| permanent.card.id)
            .collect();
        for id in doomed {
            self.destroy_permanent(id);
        }
    }

    fn cleanup(&mut self) {
        if self.players[self.active_player.index()].hand.len() > 7 {
            self.cleanup_pending = true;
        } else {
            self.complete_cleanup();
        }
    }

    fn complete_cleanup(&mut self) {
        self.channel_active[self.active_player.index()] = false;
        self.finish_cleanup();
        self.empty_mana_pools();
        if self.result.is_none() {
            self.start_next_turn();
        }
    }

    fn finish_cleanup(&mut self) {
        self.temporary_ability_grants.clear();
        for permanent in &mut self.battlefield {
            permanent.damage = 0;
            permanent.exile_instead_of_dying = false;
            permanent.damage_sources.clear();
            permanent.deathtouch_damage = false;
            permanent.power_bonus = 0;
            permanent.toughness_bonus = 0;
            permanent.temporary_keywords.clear();
            permanent.unblockable_this_turn = false;
            permanent.destroy_at_end = false;
            permanent.factory_animated = false;
            permanent.dragon_whelp_activations = 0;
            permanent.regeneration_shields = 0;
            permanent.berserked = false;
            permanent.attacked_this_turn = false;
        }
    }

    fn clear_combat(&mut self) {
        for permanent in &mut self.battlefield {
            permanent.attacking = false;
            permanent.blocked = false;
            permanent.blocking = None;
            permanent.combat_damage_assignment.clear();
        }
        self.pending_combat_attackers.clear();
        self.combat_damage_stage = CombatDamageStage::NotStarted;
        self.combat_blocked_attackers.clear();
    }

    fn winter_orb_active(&self) -> bool {
        self.battlefield.iter().any(|permanent| {
            !permanent.tapped && self.effective_behavior(permanent) == Some(CardBehavior::WinterOrb)
        })
    }

    fn draw_card(&mut self, player: PlayerId) -> Option<GameObjectId> {
        let Some(card) = self.players[player.index()].library.pop() else {
            self.finish(GameResult::Winner {
                winner: player.opponent(),
                reason: WinReason::OpponentTriedToDrawFromEmptyLibrary,
            });
            return None;
        };
        let (card, _zone_change) = self.zone_change_card(card);
        let card_id = card.id;
        self.players[player.index()].hand.push(card);
        self.events.push(GameEvent::CardDrawn {
            player,
            card: card_id,
        });
        Some(card_id)
    }

    fn draw_cards(&mut self, player: PlayerId, count: u16) {
        for _ in 0..count {
            if self.result.is_some() {
                break;
            }
            let _ = self.draw_card(player);
        }
    }

    fn empty_mana_pools(&mut self) {
        let mana_burn = self.format.rules().mana_burn;
        for player in [PlayerId::One, PlayerId::Two] {
            let amount = self.players[player.index()].mana_pool.total();
            self.players[player.index()].mana_pool = ManaPool::default();
            self.players[player.index()].mana.clear();
            if mana_burn && amount > 0 {
                let amount_as_i16 = i16::try_from(amount).unwrap_or(i16::MAX);
                self.players[player.index()].life -= amount_as_i16;
                self.events.push(GameEvent::ManaBurn { player, amount });
            }
        }
        self.check_life_totals();
    }

    fn check_life_totals(&mut self) {
        let one_lost = self.players[0].life <= 0;
        let two_lost = self.players[1].life <= 0;
        match (one_lost, two_lost) {
            (true, true) => self.finish(GameResult::Draw),
            (true, false) => self.finish(GameResult::Winner {
                winner: PlayerId::Two,
                reason: WinReason::OpponentLostAllLife,
            }),
            (false, true) => self.finish(GameResult::Winner {
                winner: PlayerId::One,
                reason: WinReason::OpponentLostAllLife,
            }),
            (false, false) => {}
        }
    }

    fn finish(&mut self, result: GameResult) {
        self.result = Some(result);
        self.events.push(GameEvent::GameEnded { result });
    }
}

/// Projects internal card instances into the public, unredacted zone view.
fn zone_cards(cards: &[CardInstance]) -> Vec<ZoneCard> {
    cards
        .iter()
        .map(|card| ZoneCard {
            object: card.id,
            definition: card.definition,
        })
        .collect()
}

fn remove_card(cards: &mut Vec<CardInstance>, id: GameObjectId) -> Option<CardInstance> {
    cards
        .iter()
        .position(|card| card.id == id)
        .map(|index| cards.remove(index))
}

fn public_cards(cards: &[CardInstance]) -> Vec<PublicCard> {
    cards
        .iter()
        .map(|card| (card.id, card.definition))
        .collect()
}

fn flatten_target_selections(selections: &[TargetSelection]) -> Vec<Target> {
    selections
        .iter()
        .flat_map(TargetSelection::targets)
        .copied()
        .collect()
}

#[cfg(test)]
fn backing_cards(backing: &ObjectBacking) -> Vec<PhysicalCardId> {
    match backing {
        ObjectBacking::Cards(cards) => cards.clone(),
        ObjectBacking::None => Vec::new(),
    }
}

fn draw_opening_hand(
    library: &mut Vec<CardInstance>,
    opening_hand_size: usize,
) -> Result<Vec<CardInstance>, GameError> {
    if library.len() < opening_hand_size {
        return Err(GameError::NotEnoughCardsForOpeningHand);
    }
    let split_at = library.len() - opening_hand_size;
    Ok(library.split_off(split_at))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GameError {
    InvalidDeck { player: PlayerId, error: DeckError },
    TooManyCards,
    NotEnoughCardsForOpeningHand,
}

impl fmt::Display for GameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDeck { player, error } => {
                write!(formatter, "invalid deck for {player}: {error}")
            }
            Self::TooManyCards => formatter.write_str("game contains too many card instances"),
            Self::NotEnoughCardsForOpeningHand => {
                formatter.write_str("deck cannot provide a seven-card opening hand")
            }
        }
    }
}

impl Error for GameError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidDeck { error, .. } => Some(error),
            Self::TooManyCards | Self::NotEnoughCardsForOpeningHand => None,
        }
    }
}

fn can_pay(pool: ManaPool, cost: ManaCost, x: u16) -> bool {
    pool.white >= cost.white
        && pool.blue >= cost.blue
        && pool.black >= cost.black
        && pool.red >= cost.red
        && pool.green >= cost.green
        && available_white_red_hybrid(pool, cost) >= cost.white_red_hybrid
        && pool.total()
            >= colored_cost_total(cost)
                .saturating_add(cost.generic)
                .saturating_add(x.saturating_mul(cost.x_multiplier))
}

fn assign_flexible_mana_outputs(
    sources: &[FlexibleManaSource],
    index: usize,
    pool: ManaPool,
    cost: ManaCost,
    x: u16,
    assignment: &mut Vec<PlannedManaActivation>,
) -> bool {
    let Some(source) = sources.get(index) else {
        return can_pay(pool, cost, x);
    };
    for (ability, color, output, benefits_payment) in &source.outputs {
        let mut next = pool;
        next.add(*output);
        assignment.push(PlannedManaActivation {
            source: source.source,
            ability: *ability,
            color: *color,
            production: *output,
            benefits_payment: *benefits_payment,
            flexibility: source.outputs.len(),
            order: source.order,
        });
        if assign_flexible_mana_outputs(sources, index + 1, next, cost, x, assignment) {
            return true;
        }
        assignment.pop();
    }
    false
}

#[cfg(test)]
fn pay_cost(pool: &mut ManaPool, cost: ManaCost, x: u16) {
    pay_cost_with_orders(
        pool,
        cost,
        x,
        &[ManaColor::Red, ManaColor::White],
        &[
            ManaColor::Colorless,
            ManaColor::Green,
            ManaColor::Black,
            ManaColor::Red,
            ManaColor::White,
            ManaColor::Blue,
        ],
    );
}

fn pay_cost_with_orders(
    pool: &mut ManaPool,
    cost: ManaCost,
    x: u16,
    hybrid_order: &[ManaColor],
    generic_order: &[ManaColor],
) {
    for color in colored_mana() {
        pool.remove_color(color, mana_cost_amount(cost, color));
    }
    let mut hybrid_remaining = cost.white_red_hybrid;
    for color in hybrid_order {
        debug_assert!(matches!(color, ManaColor::Red | ManaColor::White));
        let spent = pool.amount(*color).min(hybrid_remaining);
        pool.remove_color(*color, spent);
        hybrid_remaining -= spent;
        if hybrid_remaining == 0 {
            break;
        }
    }
    debug_assert_eq!(hybrid_remaining, 0);
    pay_generic_in_order(
        pool,
        cost.generic
            .saturating_add(x.saturating_mul(cost.x_multiplier)),
        generic_order,
    );
}

fn add_generic(mut cost: ManaCost, additional: u16) -> ManaCost {
    cost.generic = cost.generic.saturating_add(additional);
    cost
}

/// A cost reduction only ever removes generic mana, and never takes a cost
/// below its colored requirements (CR 601.2f).
fn reduce_generic(mut cost: ManaCost, reduction: u16) -> ManaCost {
    cost.generic = cost.generic.saturating_sub(reduction);
    cost
}

fn add_mana_cost(mut cost: ManaCost, additional: ManaCost) -> ManaCost {
    cost.generic = cost.generic.saturating_add(additional.generic);
    cost.white = cost.white.saturating_add(additional.white);
    cost.blue = cost.blue.saturating_add(additional.blue);
    cost.black = cost.black.saturating_add(additional.black);
    cost.red = cost.red.saturating_add(additional.red);
    cost.green = cost.green.saturating_add(additional.green);
    cost.white_red_hybrid = cost
        .white_red_hybrid
        .saturating_add(additional.white_red_hybrid);
    cost.variable_x |= additional.variable_x;
    cost.x_multiplier = cost.x_multiplier.saturating_add(additional.x_multiplier);
    cost
}

fn configured_mana_cost(
    option: &PlayOptionDef,
    configuration: &CostConfiguration,
) -> Option<ManaCost> {
    let mut cost = if let Some(selected) = configuration.alternative() {
        option
            .alternative_costs
            .iter()
            .find(|candidate| candidate.id == selected)
            .map(|candidate| candidate.mana_cost)?
    } else {
        option.mana_cost?
    };
    for selected in configuration.additional() {
        let additional = option
            .additional_costs
            .iter()
            .find(|candidate| candidate.id == *selected)?;
        if let Some(mana) = additional.mana_cost {
            cost = add_mana_cost(cost, mana);
        }
    }
    Some(cost)
}

fn mode_id_selections(
    modes: &[ModeId],
    minimum: usize,
    maximum: usize,
    may_repeat: bool,
) -> Vec<Vec<ModeId>> {
    (minimum..=maximum)
        .flat_map(|count| {
            if may_repeat {
                repeated_mode_selections(modes, count)
            } else {
                mode_combinations(modes, count)
            }
        })
        .collect()
}

fn mode_combinations(modes: &[ModeId], count: usize) -> Vec<Vec<ModeId>> {
    if count == 0 {
        return vec![Vec::new()];
    }
    if modes.len() < count {
        return Vec::new();
    }
    let mut result = Vec::new();
    for (index, mode) in modes.iter().enumerate() {
        for mut tail in mode_combinations(&modes[index + 1..], count - 1) {
            let mut choice = vec![*mode];
            choice.append(&mut tail);
            result.push(choice);
        }
    }
    result
}

fn repeated_mode_selections(modes: &[ModeId], count: usize) -> Vec<Vec<ModeId>> {
    if count == 0 {
        return vec![Vec::new()];
    }
    let mut result = Vec::new();
    for (index, mode) in modes.iter().enumerate() {
        for mut tail in repeated_mode_selections(&modes[index..], count - 1) {
            let mut choice = vec![*mode];
            choice.append(&mut tail);
            result.push(choice);
        }
    }
    result
}

fn fireball_extra_cost(behavior: CardBehavior, target_count: usize) -> u16 {
    if behavior == CardBehavior::Fireball {
        u16::try_from(target_count.saturating_sub(1)).unwrap_or(u16::MAX)
    } else {
        0
    }
}

fn pay_generic_in_order(pool: &mut ManaPool, amount: u16, order: &[ManaColor]) {
    let mut remaining = amount;
    for color in order {
        let spent = pool.amount(*color).min(remaining);
        pool.remove_color(*color, spent);
        remaining -= spent;
        if remaining == 0 {
            break;
        }
    }
    debug_assert_eq!(remaining, 0);
}

fn colored_mana() -> Vec<ManaColor> {
    vec![
        ManaColor::White,
        ManaColor::Blue,
        ManaColor::Black,
        ManaColor::Red,
        ManaColor::Green,
    ]
}

const fn mana_cost_amount(cost: ManaCost, color: ManaColor) -> u16 {
    match color {
        ManaColor::White => cost.white,
        ManaColor::Blue => cost.blue,
        ManaColor::Black => cost.black,
        ManaColor::Red => cost.red,
        ManaColor::Green => cost.green,
        ManaColor::Colorless => 0,
    }
}

const fn colored_cost_total(cost: ManaCost) -> u16 {
    cost.white + cost.blue + cost.black + cost.red + cost.green + cost.white_red_hybrid
}

const fn mana_cost_value(cost: ManaCost) -> u16 {
    cost.generic.saturating_add(colored_cost_total(cost))
}

const fn available_white_red_hybrid(pool: ManaPool, cost: ManaCost) -> u16 {
    pool.white
        .saturating_sub(cost.white)
        .saturating_add(pool.red.saturating_sub(cost.red))
}

fn one_or_none(values: &[GameObjectId]) -> Vec<Vec<GameObjectId>> {
    std::iter::once(Vec::new())
        .chain(values.iter().map(|value| vec![*value]))
        .collect()
}

fn combinations(values: &[GameObjectId], count: usize) -> Vec<Vec<GameObjectId>> {
    if count == 0 {
        return vec![Vec::new()];
    }
    if values.len() < count {
        return Vec::new();
    }
    let mut result = Vec::new();
    for (index, value) in values.iter().enumerate() {
        for mut tail in combinations(&values[index + 1..], count - 1) {
            let mut choice = vec![*value];
            choice.append(&mut tail);
            result.push(choice);
        }
    }
    result
}

fn target_combinations(values: &[Target], count: usize) -> Vec<Vec<Target>> {
    if count == 0 {
        return vec![Vec::new()];
    }
    if values.len() < count {
        return Vec::new();
    }
    let mut result = Vec::new();
    for (index, value) in values.iter().enumerate() {
        for mut tail in target_combinations(&values[index + 1..], count - 1) {
            let mut choice = vec![*value];
            choice.append(&mut tail);
            result.push(choice);
        }
    }
    result
}

fn damage_distributions(recipient_count: usize, total: u16) -> Vec<Vec<u16>> {
    if recipient_count == 0 {
        return (total == 0).then_some(Vec::new()).into_iter().collect();
    }
    let mut result = Vec::new();
    for amount in 0..=total {
        for mut tail in damage_distributions(recipient_count - 1, total - amount) {
            let mut distribution = vec![amount];
            distribution.append(&mut tail);
            result.push(distribution);
        }
    }
    result
}

#[cfg(test)]
mod tests;
