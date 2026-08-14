use std::borrow::Cow;
use std::collections::{BTreeMap, VecDeque};
use std::fmt;
use std::ops::ControlFlow;

use crate::action::{
    AbilityOrigin, Action, ActionError, CombatDamageAssignment, ManaColor, Target,
};
#[cfg(test)]
use crate::card::AbilityPredicateDef;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityProcedureDef, AbilityTargetDef, AbilityTargetPredicate,
    ActivatedAbilityDef, ActivationTimingDef, AddManaEffectDef, AlternativeCastAbilityDef,
    AlternativeCastKindDef, AnimationDef, AppliedEffectDef, BasicLandType,
    BattlefieldEntryModificationDef, CREATURE_TYPES, CardBehavior, CardCatalog,
    CardChoiceSourceDef, CardDefinition, CardEffectStatus, CardPart, CardRules, CardSet,
    CardStructure, CardSupertype, CardType, CardTypeSet, CharacteristicContext, ColorSet,
    ComparisonDef, ConditionDef, CostDef, CounterKind, DeclarativeAbilityDef, DiscardSelectionDef,
    DividedTotal, DoubleFacedKind, EffectDef, EffectDurationDef, EffectRecipientDef, HybridPair,
    KeywordAbility, ManaCost, ManaRestrictionDef, ManaSelectionDef, ManaSpendEffectDef,
    ObjectPredicateDef, ObjectQueryDef, PaymentDef, PlayActionKind, PlayOptionDef, PlayRestriction,
    PlayerRelation, QuantifierDef, ReplacementConditionDef, ReplacementEffectDef,
    ReplacementEventDef, ShieldCoverageDef, TargetPredicate, TargetSlotDef, TopCardSelectionDef,
    TriggerConditionDef, TriggerEventDef, TurnKindDef, TurnStepDef, ValueDef, ZoneKind,
    ZoneMoveCauseDef, ZonePlacement, abilities, applicable_part_ids,
};
use crate::casting::{CastChoices, CastSignature, CostConfiguration, TargetSelection};
use crate::deck::Deck;
use crate::ids::{
    AbilityId, AdditionalCostId, AlternativeCostId, CardDefinitionId, CardPartId, ChoiceIndex,
    GameObjectId, GrantId, ModeId, PhysicalCardId, PlayOptionId, PlayerId, TargetIndex,
    TargetSlotId,
};
use crate::rng::ReplayRng;
#[cfg(test)]
use crate::rules;
use crate::{AttackDefender, Format};

mod ability_actions;
mod ability_layers;
mod ability_targeting;
mod activation;
mod activation_state;
mod api;
mod battlefield;
mod card_runtime;
mod casting;
mod casting_actions;
mod casting_state;
mod characteristic_state;
mod characteristics;
mod combat;
mod combat_state;
mod continuous_effects;
mod continuous_state;
mod control_changes;
mod creature_characteristics;
mod damage;
mod decision;
mod decision_offers;
mod decision_permanent_choice;
mod decision_piles;
mod decision_resolution;
mod decision_search;
mod decision_state;
mod declarative_effects;
mod effect_support;
mod effect_values;
mod entry_replacements;
mod error;
mod event;
mod land_type_layers;
mod legacy_resolution;
mod lifecycle;
mod mana;
mod mana_planning;
mod mana_runtime;
mod mana_state;
mod observation;
mod prevention_state;
mod procedure_state;
mod replacement_state;
mod stack_resolution;
mod stack_rules;
mod state_based;
mod state_checkpoint;
mod targeting;
mod trigger_capture;
mod trigger_placement;
mod trigger_state;
mod turn;
mod zones;

use prevention_state::RelationalDamagePrevention;

pub use decision::{
    DecisionKind, DecisionObservation, DecisionOption, DecisionOrderSemantics, DecisionPreference,
    DecisionVisibility, DecisionZone,
};
pub use error::GameError;
pub use event::{BattlefieldExit, GameEvent, GameResult, StackObjectKind, Step, WinReason};
pub use mana::{Mana, ManaPool, ManaSource};
pub use observation::{
    EmblemObservation, PermanentObservation, PlayerObservation, StackObservation, ZoneCard,
    ZoneError,
};

pub(crate) use card_runtime::{
    CardAbilityResolver, CardRuntime, PileChoice, PileChosen, PileSplit, PilesSeparated,
    ResolvedAbility,
};

use observation::{LastSeenHand, PublicCard};

use activation_state::FrozenActivatedAbility;
use casting_state::{CastSourceZone, SelectedSpellPlan};
use characteristic_state::{
    BasicLandTypeChange, BattlefieldExitSnapshot, CharacteristicSource, CopiableAbility,
    CopiableCharacteristics, EffectiveAbility, LandTypeOperation, PermanentLastKnownInformation,
};
use combat_state::CombatDamageStage;
use continuous_state::{
    AbilityEffectExpiration, AbilityLayerOperation, AbilityLayerOperationKind,
    ContinuousEffectTimestamp, StaticAppliedEffect, StaticEffectTraversal, TemporaryAbilityGrant,
    TemporaryGrantedAbility, TemporaryRemovedAbilities,
};
use decision_state::{
    ApplicableBeginTurnReplacement, BalanceAction, BalancePhase, BalanceTask, CounteredSpellZone,
    DecisionContinuation, DeferredBeginTurnEffect, FORK_COPY_COLOR, PendingDecision, Pregame,
    SacrificeFollowup, ZoneMoveCause,
};
use mana_state::{
    AppliedStackEffect, FlexibleManaSource, ManaAbilityActivation, ManaPaymentPurpose,
    PlannedManaActivation,
};
use procedure_state::{DrawReplacement, PendingProcedure};
use replacement_state::{
    ApplicableReplacement, ApplicableZoneMoveReplacement, BattlefieldEntryReplacementEffect,
    BattlefieldExitCompletion, EntryCompletion, FrozenZoneMoveReplacement, PendingBattlefieldEntry,
    PendingBattlefieldExitBatch, PendingBattlefieldExitMove, PendingEvent,
    PendingReplacementEffect, ReplaceableEvent, ReplacementEffectContext,
};
use trigger_state::{
    AbilitySourceRef, BattlefieldTriggerListener, CommittedTriggerEvent, DelayedTrigger,
    FloatingTrigger, PendingTrigger, TriggerCapture, TriggerContext, TriggerEventObject,
    TriggerPlacementBatch,
};

#[cfg(test)]
use lifecycle::backing_cards;
use mana_planning::{
    add_generic, add_mana_cost, configured_mana_cost, fold_restricted_x, mana_cost_value,
    pay_cost_with_orders, reduce_generic,
};
#[cfg(test)]
use mana_planning::{can_pay, pay_cost};
use targeting::{
    combinations, extra_target_cost, flatten_target_selections, mode_id_selections, one_or_none,
    positive_compositions, target_combinations,
};
use zones::{public_cards, remove_card};

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

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
struct Permanent {
    card: CardInstance,
    timestamp: ContinuousEffectTimestamp,
    /// The logical part currently supplying this permanent's printed
    /// characteristics. Transforming changes this without changing object ID.
    presented: CardPartId,
    controller: PlayerId,
    tapped: bool,
    entered_controller_turn: u32,
    damage: u16,
    /// Loyalty counters are distinct from marked creature damage and persist
    power_bonus: i16,
    toughness_bonus: i16,
    attacking: bool,
    attack_defender: Option<crate::AttackDefender>,
    emblem_source: Option<AbilityOrigin>,
    /// Whether a loyalty ability has already been activated this turn. CR
    /// 606.3 allows one per planeswalker per turn.
    activated_loyalty_this_turn: bool,
    /// Whether nothing may block this creature for the rest of the turn.
    /// Cleared in cleanup with the other until-end-of-turn state.
    unblockable_this_turn: bool,
    /// Whether combat damage to and from this permanent is prevented for the
    /// rest of the turn. Maze of Ith sets it; the creature stays an attacker
    /// so its attack triggers and its blockers are unaffected.
    combat_damage_prevented: bool,
    /// Whether combat damage from this permanent is prevented without also
    /// preventing combat damage to it. Kor Haven uses this narrower marker.
    combat_damage_dealt_by_prevented: bool,
    /// Who controls this permanent again once the turn ends, set while a
    /// control-changing effect holds it. Cleanup restores it.
    control_reverts_to: Option<PlayerId>,
    /// Whether a "can't be regenerated" effect is covering this permanent
    /// for the rest of the turn.
    cannot_regenerate_this_turn: bool,
    /// The permanent whose continued presence is holding this one's control
    /// change. When it leaves the battlefield or changes hands, control goes
    /// back to `control_reverts_to`.
    control_source: Option<GameObjectId>,
    /// Whether that holder also has to stay tapped to keep the change.
    control_requires_source_tapped: bool,
    /// Whether this attacker was blocked. A blocked creature stays blocked
    /// even if every blocker leaves, so this cannot be recomputed from the
    /// blockers still on the battlefield.
    blocked: bool,
    blocking: Option<GameObjectId>,
    chosen_player: Option<PlayerId>,
    chosen_creature_type: Option<String>,
    /// The card name a permanent named as it entered, for Pithing Needle.
    chosen_card_name: Option<String>,
    destroy_at_end: bool,
    temporary_keywords: Vec<KeywordAbility>,
    temporary_granted_abilities: Vec<TemporaryGrantedAbility>,
    temporary_removed_abilities: Vec<TemporaryRemovedAbilities>,
    /// The creature this permanent has become for the turn, if a manland's
    /// animation ability has resolved.
    animation: Option<&'static AnimationDef>,
    /// How many times each of this permanent's activated abilities has been
    /// activated this turn, for the cards that count their own activations.
    /// Cleared with the rest of the once-a-turn state.
    activations_this_turn: Vec<(AbilityOrigin, u8)>,
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
    attacked_this_turn: bool,
    /// How many times this creature has been declared as an attacker this
    /// turn. `attacked_this_turn` is already set by the time the attack
    /// triggers are captured, so a "first time each turn" trigger needs the
    /// count rather than the flag.
    attacks_this_turn: u8,
    /// Keywords granted until a named player's next upkeep, which outlive
    /// the cleanup that clears `temporary_keywords`. Erhnam Djinn's
    /// forestwalk is one.
    keywords_until_upkeep_of: Vec<(PlayerId, KeywordAbility)>,
    /// Sources that dealt damage to this permanent during the current turn.
    /// IDs deliberately refer to the damaging object incarnation so a later
    /// death trigger can use the live source or its retired LKI snapshot.
    damage_sources: Vec<GameObjectId>,
    /// Whether this permanent has dealt damage to an opponent of its
    /// controller this turn, by any means. Cleared in cleanup with the rest
    /// of the once-a-turn state.
    dealt_damage_to_opponent_this_turn: bool,
    /// Whether any damage still marked on this permanent came from a source
    /// with deathtouch. The source may leave before state-based actions are
    /// checked, so this is damage-event state rather than a live lookup.
    deathtouch_damage: bool,
    /// The permanent whose ability created this token, for the cards that
    /// later refer to "tokens created with this creature". A token that
    /// outlives its creator keeps pointing at an object ID nothing matches,
    /// which is what makes those tokens permanently orphaned.
    created_by: Option<GameObjectId>,
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
            timestamp: ContinuousEffectTimestamp(u64::from(card.id.0)),
            card,
            presented,
            controller,
            tapped: false,
            entered_controller_turn,
            damage: 0,
            power_bonus: 0,
            toughness_bonus: 0,
            attacking: false,
            attack_defender: None,
            emblem_source: None,
            activated_loyalty_this_turn: false,
            unblockable_this_turn: false,
            combat_damage_prevented: false,
            combat_damage_dealt_by_prevented: false,
            control_reverts_to: None,
            cannot_regenerate_this_turn: false,
            control_source: None,
            control_requires_source_tapped: false,
            blocked: false,
            blocking: None,
            chosen_player: None,
            chosen_creature_type: None,
            chosen_card_name: None,
            destroy_at_end: false,
            temporary_keywords: Vec::new(),
            temporary_granted_abilities: Vec::new(),
            temporary_removed_abilities: Vec::new(),
            animation: None,
            activations_this_turn: Vec::new(),
            counters: [0; CounterKind::COUNT],
            attached_to: None,
            exile_instead_of_dying: false,
            combat_damage_assignment: Vec::new(),
            copy_effect: None,
            copied_from: None,
            text_changes: Vec::new(),
            regeneration_shields: 0,
            attacked_this_turn: false,
            attacks_this_turn: 0,
            keywords_until_upkeep_of: Vec::new(),
            damage_sources: Vec::new(),
            dealt_damage_to_opponent_this_turn: false,
            deathtouch_damage: false,
            created_by: None,
        }
    }

    const fn counters(&self, kind: CounterKind) -> u16 {
        self.counters[kind.index()]
    }

    const fn add_counters(&mut self, kind: CounterKind, amount: u16) {
        let index = kind.index();
        self.counters[index] = self.counters[index].saturating_add(amount);
    }

    const fn remove_counters(&mut self, kind: CounterKind, amount: u16) {
        let index = kind.index();
        self.counters[index] = self.counters[index].saturating_sub(amount);
    }

    const fn set_counters(&mut self, kind: CounterKind, amount: u16) {
        self.counters[kind.index()] = amount;
    }
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
        mana_value: u16,
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
    /// Colours the copy effect that made this object imposed on it, for
    /// "except that the copy is red". Nothing else changes an object's
    /// colour on the stack.
    colors: Option<ColorSet>,
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
    mode_effects: Vec<ScopedEffect>,
    /// The X chosen when the ability was activated, so its effects read the
    /// same number the cost was paid for.
    x: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum StackAbilityResolver {
    Declarative(ScopedEffect),
    DeclarativeWithCustomFollowup {
        effect: ScopedEffect,
        behavior: CardBehavior,
    },
    Custom(CardBehavior),
    CardOwned(&'static CardAbilityResolver),
}

/// One authored effect together with the start of its clause-local target
/// range in an instantiated stack object's flattened target list.
///
/// An authored [`TargetIndex`] is deliberately local to its ability clause.
/// Modal branches can each name target zero; freezing a spell assigns every
/// selected branch a distinct base and resolution translates through it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ScopedEffect {
    effect: EffectDef,
    target_base: usize,
}

impl ScopedEffect {
    const fn primary(effect: EffectDef) -> Self {
        Self {
            effect,
            target_base: 0,
        }
    }

    const fn with_effect(self, effect: EffectDef) -> Self {
        Self {
            effect,
            target_base: self.target_base,
        }
    }

    fn target_slot(self, target: TargetIndex) -> TargetSlotId {
        TargetSlotId::from_index(self.target_base + target.index())
            .expect("validated target composition fits the runtime slot space")
    }
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
    /// Whether this player has tried to draw from an empty library since
    /// state-based actions were last checked.
    tried_to_draw_from_empty_library: bool,
    hand: Vec<CardInstance>,
    graveyard: Vec<CardInstance>,
    exile: Vec<CardInstance>,
    /// Cards the player brought in their sideboard. Outside the game is not
    /// a zone, so ordinary zone queries and observations never walk this
    /// collection.
    outside_game: Vec<CardInstance>,
    mana_pool: ManaPool,
    mana: Vec<Mana>,
    land_played_this_turn: bool,
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
    next_continuous_effect_timestamp: u64,
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
    /// Combat phases still owed this turn, added by an effect rather than by
    /// the ordinary turn structure.
    additional_combat_phases: u8,
    /// Whether each player has been stopped from casting noncreature spells
    /// for the rest of the turn.
    noncreature_casts_locked: [bool; 2],
    /// Emblems, which are objects with abilities and no zone. They are kept
    /// beside the battlefield rather than on it: only the static-effect walk
    /// reads them, and nothing can target, tap, or destroy one.
    emblems: Vec<Permanent>,
    /// How many spells each player has cast this turn, and how many they cast
    /// during the turn before. The werewolves ask about the turn that just
    /// ended, which is only knowable if it was counted while it happened.
    spells_cast_this_turn: [u16; 2],
    spells_cast_last_turn: [u16; 2],
    /// How many cards each player has drawn this turn. Miracle asks whether a
    /// draw was the first one.
    cards_drawn_this_turn: [u16; 2],
    /// The cards each player drew this turn, in draw order. Sylvan Library
    /// chooses among them, and only a card still in hand can be chosen.
    drawn_this_turn: [Vec<GameObjectId>; 2],
    /// Set while a resumable all-player draw instruction is in progress.
    /// Empty-library loss itself is settled at the next state-based-action
    /// check using the flags stored on each player.
    defer_empty_library_loss: bool,
    /// One-shot draw replacements, in creation order for each player.
    draw_replacements: [VecDeque<DrawReplacement>; 2],
    /// Prevention rules that inspect control or source identity when damage
    /// would be dealt rather than freezing a set of permanents at resolution.
    relational_damage_preventions: Vec<RelationalDamagePrevention>,
    /// The revealed card a miracle cost may currently be paid for. The window
    /// belongs to one card and closes as soon as its controller does anything
    /// else.
    miracle_window: Option<GameObjectId>,
    /// Effects waiting for a step to begin. Obzedat's return is one.
    delayed_triggers: Vec<DelayedTrigger>,
    /// Triggered abilities listening from nowhere until their controller's
    /// next turn. Jace's first ability installs one.
    floating_triggers: Vec<FloatingTrigger>,
    blockers_declared: bool,
    untap_pending: bool,
    pregame: Option<Pregame>,
    mulligans: [u8; 2],
    cleanup_pending: bool,
    pending_decisions: Vec<PendingDecision>,
    next_decision_id: u32,
    pending_events: VecDeque<PendingEvent>,
    pending_procedures: VecDeque<PendingProcedure>,
    pending_triggers: Vec<PendingTrigger>,
    next_trigger_id: u32,
    last_seen_hands: [LastSeenHand; 2],
    pending_combat_attackers: Vec<GameObjectId>,
    combat_damage_stage: CombatDamageStage,
    combat_blocked_attackers: Vec<GameObjectId>,
    /// The next ordinary turn in the two-player rotation. Extra turns sit in
    /// front of this anchor without changing it.
    next_regular_player: PlayerId,
    extra_turns: Vec<PlayerId>,
    channel_active: [bool; 2],
    /// A Fog: all combat damage this turn is prevented. This is game state
    /// rather than permanent state because it outlives any particular
    /// creature, and applies to combatants that were not on the battlefield
    /// when it resolved.
    all_combat_damage_prevented: bool,
    /// Prevention shields waiting for damage this turn. Each is spent as the
    /// damage it covers is dealt, and whatever is left is discarded in
    /// cleanup, so a shield never outlives the turn that made it.
    prevention_shields: Vec<PreventionShield>,
    result: Option<GameResult>,
    events: Vec<GameEvent>,
}

/// One "prevent the next N damage" promise. `remaining` of `None` is the
/// "prevent all damage" form, which is never spent and simply lasts the turn.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PreventionShield {
    recipient: Target,
    remaining: Option<u16>,
    /// The one source this shield answers, for "the next time a source of
    /// your choice would deal damage". `None` covers every source, which is
    /// what an ordinary "prevent the next N damage" shield does. A shield
    /// naming a source is spent by the first damage that source deals however
    /// much it prevents, so it is removed on use even without a remaining
    /// count.
    source: Option<GameObjectId>,
    /// How much of a covered hit this shield stops. Only a shield naming a
    /// source uses anything but the whole of it.
    coverage: ShieldCoverageDef,
    /// Whether the recipient's controller gains life equal to what this
    /// shield actually prevented, which Reverse Damage does and an ordinary
    /// shield does not.
    gain_life: bool,
}

#[cfg(test)]
mod planeswalker_combat_tests;
#[cfg(test)]
mod planeswalker_tests;
#[cfg(test)]
mod tests;
