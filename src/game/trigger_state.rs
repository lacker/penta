use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt;
use std::sync::{Arc, RwLock};

use crate::action::{AbilityOrigin, Target};
use crate::card::{
    AbilityTargetDef, CardSupertype, CardTypeSet, CounterKind, EffectDef, ModalSpellDef,
    TriggerConditionDef, TriggerEventDef, TurnStepDef, ZoneKind,
};
use crate::casting::TargetSelection;
use crate::ids::{Binding, GameObjectId, PlayerId};

use super::{CastSourceZone, ObjectCharacteristics, StackAbilityResolver, StackObjectKind};

/// The prospective draw an ordinary effect inside a replacement program may
/// resume. The applied sources are event-local CR 614.5 state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ReplacedDrawContinuation {
    pub(super) player: PlayerId,
    pub(super) applied: Vec<AbilitySourceRef>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct TriggerContext {
    pub(super) object: Option<GameObjectId>,
    /// The exact object created by the zone change that caused this trigger.
    /// This is event-local rather than inferred from the global successor
    /// graph, and becomes stale if that object moves again before resolution.
    pub(super) zone_change_result: Option<GameObjectId>,
    pub(super) object_controller: Option<PlayerId>,
    pub(super) event_player: Option<PlayerId>,
    pub(super) amount: Option<i32>,
    /// What the event's damage was dealt to, when it was dealt to an object.
    /// Kept apart from `object`, which for a damage event is the source that
    /// dealt it: "whenever this creature deals combat damage to a creature,
    /// exile that creature" names both, and they are never the same one.
    pub(super) damaged_object: Option<GameObjectId>,
    /// What a reflexive sacrifice clause gave up, kept apart from `object`
    /// the way `damaged_object` is: the event matches the source that
    /// sacrificed, while "that creature's power" and "if the sacrificed
    /// creature was a Hamster" both name what it took. Read through the
    /// retired objects, because it is in a graveyard by the time the
    /// reflexive ability goes on the stack.
    pub(super) sacrificed_object: Option<GameObjectId>,
    /// Where the triggering spell was cast from. `None` for every event that
    /// is not a spell cast, and retained by the trigger after the spell has
    /// left the stack.
    pub(super) cast_from_zone: Option<ZoneKind>,
}

impl TriggerContext {
    pub(super) const fn empty() -> Self {
        Self {
            object: None,
            zone_change_result: None,
            object_controller: None,
            event_player: None,
            amount: None,
            damaged_object: None,
            sacrificed_object: None,
            cast_from_zone: None,
        }
    }
}

/// State local to one declarative effect resolution. Trigger information is
/// kept separate and copyable because it is also captured by abilities before
/// they ever resolve; bindings belong only to a particular continuation of an
/// effect program.
#[derive(Debug, Eq, PartialEq)]
pub(super) struct EffectResolutionContext {
    pub(super) trigger: TriggerContext,
    pub(super) replaced_draw: Option<ReplacedDrawContinuation>,
    /// What a payment made during this resolution actually cost, for the
    /// branch that reads it back. "You may pay {X}" settles X here rather
    /// than at the cast, which is where an ordinary X lives.
    pub(super) paid_amount: Option<u16>,
    /// How many objects the previous step matched, for a follow-up that
    /// counts them: the land cards a discard actually took.
    pub(super) matched_count: Option<u16>,
    /// Distinct card types among those same matched objects.
    pub(super) matched_card_types: Option<u16>,
    /// What those same matched objects add up to in mana value, for a
    /// follow-up measured by what the step before it turned up.
    pub(super) matched_mana_value: Option<u16>,
    /// A card name chosen while this effect resolves, which the rest of the
    /// same resolution reads back. Cabal Therapy names one and then discards
    /// every copy of it.
    pub(super) chosen_name: Option<String>,
    /// A counter kind selected by a nested counter-choice effect.
    pub(super) chosen_counter: Option<CounterKind>,
    parent_object: Shared<Option<Target>>,
    parent_objects: Shared<Vec<Target>>,
    bindings: Shared<EffectBindings>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(super) struct EffectBindings {
    values: BTreeMap<String, EffectBindingValue>,
}

struct Shared<T>(Arc<RwLock<T>>);

impl<T> Shared<T> {
    fn new(value: T) -> Self {
        Self(Arc::new(RwLock::new(value)))
    }

    fn with<R>(&self, read: impl FnOnce(&T) -> R) -> R {
        let bindings = self
            .0
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        read(&bindings)
    }

    fn with_mut<R>(&self, write: impl FnOnce(&mut T) -> R) -> R {
        let mut bindings = self
            .0
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        write(&mut bindings)
    }
}

impl<T: Clone> Shared<T> {
    fn snapshot(&self) -> T {
        self.with(Clone::clone)
    }
}

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<T: Default> Default for Shared<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Clone + fmt::Debug> fmt::Debug for Shared<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.snapshot().fmt(formatter)
    }
}

impl<T: Clone + PartialEq> PartialEq for Shared<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0) || self.snapshot() == other.snapshot()
    }
}

impl<T: Clone + Eq> Eq for Shared<T> {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum EffectBindingValue {
    Object(Option<Target>),
    Objects(Vec<Target>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum RuntimeBinding {
    Label(String),
    ParentBinding,
}

impl From<Binding> for RuntimeBinding {
    fn from(binding: Binding) -> Self {
        match binding.label() {
            Some(label) => Self::Label(label.to_owned()),
            None => Self::ParentBinding,
        }
    }
}

impl Clone for EffectResolutionContext {
    fn clone(&self) -> Self {
        self.clone_with_binding_state(
            Shared::new(self.parent_object.snapshot()),
            Shared::new(self.parent_objects.snapshot()),
            Shared::new(self.bindings.snapshot()),
        )
    }
}

impl EffectResolutionContext {
    fn clone_with_binding_state(
        &self,
        parent_object: Shared<Option<Target>>,
        parent_objects: Shared<Vec<Target>>,
        bindings: Shared<EffectBindings>,
    ) -> Self {
        Self {
            trigger: self.trigger,
            replaced_draw: self.replaced_draw.clone(),
            paid_amount: self.paid_amount,
            matched_count: self.matched_count,
            matched_card_types: self.matched_card_types,
            matched_mana_value: self.matched_mana_value,
            chosen_name: self.chosen_name.clone(),
            chosen_counter: self.chosen_counter,
            parent_object,
            parent_objects,
            bindings,
        }
    }

    /// Fork one branch of the same resolution. Labeled bindings and the
    /// current lexical parent remain shared across sequence suspension, while
    /// ordinary `Clone` takes an independent snapshot for stored rules state.
    pub(super) fn fork_resolution(&self) -> Self {
        self.clone_with_binding_state(
            self.parent_object.clone(),
            self.parent_objects.clone(),
            self.bindings.clone(),
        )
    }

    pub(super) fn new(trigger: TriggerContext) -> Self {
        Self {
            trigger,
            replaced_draw: None,
            paid_amount: None,
            matched_count: None,
            matched_card_types: None,
            matched_mana_value: None,
            chosen_name: None,
            chosen_counter: None,
            parent_object: Shared::default(),
            parent_objects: Shared::default(),
            bindings: Shared::default(),
        }
    }

    #[cfg(test)]
    pub(super) fn empty() -> Self {
        Self::new(TriggerContext::empty())
    }

    pub(super) fn single_object(&self, binding: Binding) -> Option<Target> {
        match binding.label() {
            Some(label) => self.single_object_label(label),
            None => self.parent_object.snapshot(),
        }
    }

    pub(super) fn single_object_label(&self, label: &str) -> Option<Target> {
        self.bindings
            .with(|bindings| match bindings.values.get(label) {
                Some(EffectBindingValue::Object(object)) => *object,
                Some(EffectBindingValue::Objects(_)) | None => None,
            })
    }

    pub(super) fn bind_single_object(&mut self, binding: Binding, object: Option<Target>) {
        match binding.label() {
            Some(label) => self.bind_single_object_label(label, object),
            None => self.parent_object = Shared::new(object),
        }
    }

    pub(super) fn bind_single_object_label(&mut self, label: &str, object: Option<Target>) {
        self.bindings.with_mut(|bindings| {
            bindings
                .values
                .insert(label.to_owned(), EffectBindingValue::Object(object));
        });
    }

    pub(super) fn bind_runtime_single_object(
        &mut self,
        binding: &RuntimeBinding,
        object: Option<Target>,
    ) {
        match binding {
            RuntimeBinding::Label(label) => self.bind_single_object_label(label, object),
            RuntimeBinding::ParentBinding => self.parent_object = Shared::new(object),
        }
    }

    pub(super) fn object_group(&self, binding: Binding) -> Vec<Target> {
        match binding.label() {
            Some(label) => self.object_group_label(label),
            None => self.parent_objects.snapshot(),
        }
    }

    pub(super) fn object_group_label(&self, label: &str) -> Vec<Target> {
        self.bindings
            .with(|bindings| match bindings.values.get(label) {
                Some(EffectBindingValue::Objects(objects)) => objects.clone(),
                Some(EffectBindingValue::Object(_)) | None => Vec::new(),
            })
    }

    pub(super) fn bind_object_group(&mut self, binding: Binding, objects: Vec<Target>) {
        match binding.label() {
            Some(label) => self.bind_object_group_label(label, objects),
            None => self.parent_objects = Shared::new(objects),
        }
    }

    pub(super) fn bind_object_group_label(&mut self, label: &str, objects: Vec<Target>) {
        self.bindings.with_mut(|bindings| {
            bindings
                .values
                .insert(label.to_owned(), EffectBindingValue::Objects(objects));
        });
    }

    pub(super) fn runtime_object_group(&self, binding: &RuntimeBinding) -> Vec<Target> {
        match binding {
            RuntimeBinding::Label(label) => self.object_group_label(label),
            RuntimeBinding::ParentBinding => self.parent_objects.snapshot(),
        }
    }

    pub(super) fn bind_runtime_object_group(
        &mut self,
        binding: &RuntimeBinding,
        objects: Vec<Target>,
    ) {
        match binding {
            RuntimeBinding::Label(label) => self.bind_object_group_label(label, objects),
            RuntimeBinding::ParentBinding => self.parent_objects = Shared::new(objects),
        }
    }

    pub(super) fn declare_binding_group_label(&mut self, label: &str) {
        self.bindings.with_mut(|bindings| {
            bindings
                .values
                .entry(label.to_owned())
                .or_insert_with(|| EffectBindingValue::Objects(Vec::new()));
        });
    }

    pub(super) fn bind_binding_group_label(&mut self, label: &str, objects: Vec<Target>) {
        self.bindings.with_mut(|bindings| {
            bindings
                .values
                .insert(label.to_owned(), EffectBindingValue::Objects(objects));
        });
    }

    /// Remove object incarnations consumed by a group action from every
    /// earlier binding. A later stage that needs the zone-change successors
    /// receives them through the action's explicit output binding; retaining
    /// the stale hidden-zone identities would make unrelated earlier groups
    /// part of every later decision and checkpoint.
    pub(super) fn consume_bound_objects(&mut self, objects: &[Target]) {
        self.bindings.with_mut(|bindings| {
            for binding in bindings.values.values_mut() {
                match binding {
                    EffectBindingValue::Object(object) => {
                        if object.is_some_and(|object| objects.contains(&object)) {
                            *object = None;
                        }
                    }
                    EffectBindingValue::Objects(group) => {
                        group.retain(|object| !objects.contains(object));
                    }
                }
            }
        });
        self.parent_object.with_mut(|parent| {
            if parent.is_some_and(|object| objects.contains(&object)) {
                *parent = None;
            }
        });
        self.parent_objects
            .with_mut(|parents| parents.retain(|object| !objects.contains(object)));
    }

    pub(super) fn bindings(&self) -> BTreeMap<String, EffectBindingValue> {
        self.bindings.snapshot().values
    }

    pub(super) fn parent_object(&self) -> Option<Target> {
        self.parent_object.snapshot()
    }

    pub(super) fn parent_objects(&self) -> Vec<Target> {
        self.parent_objects.snapshot()
    }

    pub(super) fn bound_targets(&self) -> Vec<Target> {
        let mut targets = self
            .parent_object
            .snapshot()
            .iter()
            .copied()
            .chain(self.parent_objects.snapshot())
            .collect::<Vec<_>>();
        targets.extend(self.bindings.snapshot().values.values().flat_map(
            |binding| match binding {
                EffectBindingValue::Object(object) => object.iter().copied().collect::<Vec<_>>(),
                EffectBindingValue::Objects(objects) => objects.clone(),
            },
        ));
        targets
    }

    pub(super) fn from_bindings(
        trigger: TriggerContext,
        parent_object: Option<Target>,
        parent_objects: Vec<Target>,
        values: BTreeMap<String, EffectBindingValue>,
    ) -> Self {
        Self {
            trigger,
            replaced_draw: None,
            paid_amount: None,
            matched_count: None,
            matched_card_types: None,
            matched_mana_value: None,
            chosen_name: None,
            chosen_counter: None,
            parent_object: Shared::new(parent_object),
            parent_objects: Shared::new(parent_objects),
            bindings: Shared::new(EffectBindings { values }),
        }
    }
}

impl From<TriggerContext> for EffectResolutionContext {
    fn from(trigger: TriggerContext) -> Self {
        Self::new(trigger)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub(super) struct TriggerEventObject {
    pub(super) id: GameObjectId,
    pub(super) token: bool,
    pub(super) types: CardTypeSet,
    pub(super) controller: PlayerId,
    pub(super) colors: [bool; 5],
    pub(super) subtypes: Cow<'static, [&'static str]>,
    pub(super) mana_value: u16,
    /// Current power where one exists: a battlefield creature reports what it
    /// is now, not what it was printed as.
    pub(super) power: Option<i16>,
    /// Current toughness, read the same way and with the same caveat.
    pub(super) toughness: Option<i16>,
    pub(super) supertypes: [bool; CardSupertype::COUNT],
    /// Whether this object is in combat. Cheap to carry and it cannot feed
    /// back into a characteristic, unlike a keyword or a static bonus.
    pub(super) attacking_or_blocking: bool,
    /// The object's keywords, as a bitmask over
    /// [`crate::card::KeywordAbility::simple_index`].
    ///
    /// Unlike power, this includes keywords a static continuous effect grants
    /// or removes: `Game::keyword_mask` stratifies the layer-6 walk rather than
    /// omitting it, so a predicate here and the combat rules give one answer.
    /// The one exception is the walk's own recipient matching, which reads the
    /// layer below itself; `Game::collect_ability_layer_operations` says why.
    pub(super) keywords: u64,
    /// Whether this creature is attacking, excluding a creature that is only
    /// blocking. Bloodrush and similar predicates need the narrower state.
    pub(super) attacking: bool,
    /// Whether the object is a tapped permanent. Cheap to carry, and like
    /// `attacking` it cannot feed back into a characteristic.
    pub(super) tapped: bool,
    /// Whether this creature attacked at any point this turn, which outlives
    /// combat and so is not the same question as `attacking`.
    pub(super) attacked_this_turn: bool,
    pub(super) saddled: bool,
    /// Whether it attacked during its controller's previous turn. Answered
    /// where the snapshot is built, because the turn count it is measured
    /// against belongs to the game rather than to the permanent.
    pub(super) attacked_during_controllers_last_turn: bool,
}

include!("trigger_state/committed_event.rs");
include!("trigger_state/types.rs");
