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
    AbilityCostDef, AbilityDef, AbilityOperationDef, AbilityProcedureDef, AbilityTargetDef,
    AbilityTargetPredicate, ActivatedAbilityDef, ActivationTimingDef, AddManaEffectDef,
    AlternativeCastAbilityDef, AlternativeCastKindDef, AppliedEffectDef, AppliedRuleDef,
    BandingQuality, BasicLandType, BattlefieldEntryModificationDef, CREATURE_TYPES, CardBehavior,
    CardCatalog, CardChoiceSourceDef, CardDefinition, CardEffectStatus, CardPart, CardRules,
    CardSet, CardStructure, CardSupertype, CardType, CardTypeSet, CharacteristicContext,
    CharacteristicOperationDef, ColorChoiceOperationDef, ColorSet, ComparisonDef, ConditionDef,
    ControlDurationDef, CounterKind, CreatureTypeSetDef, DamageEventMatcherDef, DamageKindDef,
    DamageRecipientMatcherDef, DamageSourceGroupDef, DamageSourceMatcherDef, DeclarativeAbilityDef,
    DiscardSelectionDef, DividedTotal, DoubleFacedKind, EffectDef, EffectPaymentCostDef,
    EffectPaymentDef, EffectRecipientDef, EffectRecipientSetDef, FaceDownCharacteristics,
    HybridPair, KeywordAbility, ManaCost, ManaRestrictionDef, ManaSelectionDef, ManaSpendEffectDef,
    ObjectCountConditionDef, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef,
    OptionalAdditionalCostKindDef, PlayActionKind, PlayOptionDef, PlayRestriction, PlayerRefDef,
    PlayerRelation, PlayerSetDef, PowerToughnessOperationDef, ProtectedCreatureType, QuantifierDef,
    ReplacementChoiceDef, ReplacementConditionDef, ReplacementEffectDef, ReplacementEventDef,
    ResolvedEffectDurationDef, SacrificedAmountDef, SetOperationDef, SpellResolutionDestinationDef,
    StackTargetKindDef, TapPurposeDef, TargetPredicate, TargetSlotDef, TokenCharacteristics,
    TopCardSelectionDef, TriggerConditionDef, TriggerEventDef, TurnKindDef, TurnPhaseDef,
    TurnStepDef, ValueDef, ZoneKind, ZoneMoveCauseDef, ZonePlacement, abilities,
    applicable_part_ids,
};
use crate::casting::{CastChoices, CastSignature, CostConfiguration, TargetSelection};
use crate::deck::Deck;
use crate::ids::{
    AbilityId, AdditionalCostId, AlternativeCostId, CardDefinitionId, CardPartId, GameObjectId,
    GrantId, ModeId, PhysicalCardId, PlayOptionId, PlayerId, TargetIndex, TargetSlotId,
};
use crate::rng::ReplayRng;
#[cfg(test)]
use crate::rules;
use crate::{AttackDefender, Format};

mod ability_actions;
mod ability_layers;
mod ability_targeting;
mod activation;
mod activation_sacrifice;
mod activation_state;
mod api;
mod attachments;
mod banding;
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
mod crime;
mod damage;
mod decision;
mod decision_doomsday;
mod decision_keep_one_per_type;
mod decision_offers;
mod decision_permanent_choice;
mod decision_piles;
mod decision_resolution;
mod decision_search;
mod decision_search_resolution;
mod decision_state;
mod declarative_effects;
mod effect_support;
mod effect_values;
mod endure;
mod entry_replacements;
mod error;
mod event;
mod exert;
mod exile_permission;
mod explore;
mod face_down;
mod foretell;
mod land_type_layers;
mod land_type_substitution;
mod legacy_resolution;
mod lifecycle;
mod mana;
mod mana_planning;
mod mana_runtime;
mod mana_state;
mod monarch;
mod ninjutsu;
mod observation;
mod phasing;
mod play_permissions;
mod prevention_state;
mod procedure_state;
mod proliferate;
mod prospective_x;
mod replacement_state;
mod rooms;
mod sacrifice_to_total;
mod stack_resolution;
mod stack_rules;
mod state_based;
mod state_checkpoint;
mod static_animation;
mod targeting;
mod tokens;
mod trigger_capture;
mod trigger_placement;
mod trigger_state;
mod turn;
mod vote;
mod zones;

use prevention_state::{
    RelationalSourceFilter, ResolvedDamagePrevention, ResolvedDamagePreventionCapacity,
    ResolvedDamagePreventionCoverage, ResolvedDamageRecipientMatcher, ResolvedDamageRedirect,
    ResolvedDamageSourceMatcher,
};

pub use decision::{
    DecisionKind, DecisionObservation, DecisionOption, DecisionOrderSemantics, DecisionPreference,
    DecisionVisibility, DecisionZone,
};
pub use error::GameError;
use event::TurnPhaseResume;
pub use event::{BattlefieldExit, GameEvent, GameResult, StackObjectKind, Step, WinReason};
pub use mana::{Mana, ManaPool, ManaSource};
pub use observation::{
    EmblemObservation, ObjectCharacteristics, PermanentObservation, PhysicalFaceObservation,
    PhysicalFaceSide, PlayerObservation, StackObservation, ZoneCard, ZoneError,
};

pub(crate) use card_runtime::{
    CardAbilityResolver, CardRuntime, PileChoice, PileChosen, PileSplit, PilesSeparated,
    ResolvedAbility,
};

use observation::{LastSeenHand, PublicCard};

use activation_sacrifice::SacrificeQuota;
use activation_state::{ActivationChoices, FrozenActivatedAbility, PendingActivation};
use casting_state::{
    CastCostContext, CastOffer, CastOfferCost, CastSourceZone, SelectedSpellPlan,
    cast_source_zone_from_label,
};
use characteristic_state::{
    BasicLandTypeChange, BattlefieldExitSnapshot, CharacteristicSource, CopiableAbility,
    CopiableCharacteristics, DoubleFacedCopiableCharacteristics, EffectiveAbility,
    LandTypeOperation, PermanentLastKnownInformation,
};
use combat_state::CombatDamageStage;
use continuous_state::{
    AbilityLayerOperation, AbilityLayerOperationKind, AppliedAttackRestriction,
    AppliedPlayRestriction, AppliedRuleEffect, ContinuousEffectExpiration,
    ContinuousEffectTimestamp, ResolvedAbilityOperation, ResolvedAttackRestriction,
    ResolvedContinuousEffect, ResolvedContinuousEffectKind, ResolvedPlayPermission,
    ResolvedPlayRestriction, ResolvedPowerToughnessOperation, StaticAppliedEffect,
    StaticEffectTraversal, TemporaryAbilityGrant,
};
use decision_state::{
    ApplicableBeginTurnReplacement, BalanceAction, BalancePhase, BalanceTask, CounteredSpellZone,
    DecisionContinuation, DeferredBeginTurnEffect, DiscardFollowUp, FORK_COPY_COLOR,
    PendingDecision, Pregame, ResolvedEffectPayment, SacrificeDeclined, SacrificeFollowup,
    SearchFollowUp, ZoneMoveCause,
};
use exile_permission::{ExilePlayCost, ExilePlayPermission};
use mana_state::{
    AppliedStackEffect, FlexibleManaSource, ManaAbilityActivation, ManaActivationChoices,
    ManaPaymentPurpose, ManaPlanOptions, ManaSourceOutput, ManaSourceOutputs, PaymentCapacity,
    PlannedManaActivation, PlannedPaymentKind,
};
use procedure_state::{DrawReplacement, PendingProcedure};
use replacement_state::{
    ApplicableReplacement, ApplicableZoneMoveReplacement, BattlefieldExitCompletion,
    EntryCompletion, FrozenZoneMoveReplacement, PendingBattlefieldEntry,
    PendingBattlefieldExitBatch, PendingBattlefieldExitMove, PendingEvent,
    PendingReplacementEffect, ReplaceableEvent, ReplacementEffectContext,
};
use trigger_state::{
    AbilitySourceRef, BattlefieldTriggerListener, CommittedTriggerEvent, EffectResolutionContext,
    InstalledTrigger, InstalledTriggerLifetime, PendingTrigger, TriggerCapture, TriggerContext,
    TriggerEventObject, TriggerPlacementBatch,
};

#[cfg(test)]
use lifecycle::backing_cards;
use mana_planning::{
    add_generic, add_mana_cost, configured_base_mana_cost, fold_restricted_x, mana_cost_value,
    pay_cost_with_generic_strategy, reduce_generic,
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
    /// Counters on a card outside the battlefield, such as suspend's time
    /// counters. Zone changes clear this along with every other object-local
    /// property.
    counters: [u16; CounterKind::COUNT],
}

impl CardInstance {
    const fn add_counters(&mut self, kind: CounterKind, amount: u16) {
        let index = kind.index();
        self.counters[index] = self.counters[index].saturating_add(amount);
    }

    #[cfg(test)]
    const fn counters(&self, kind: CounterKind) -> u16 {
        self.counters[kind.index()]
    }
}

/// The physical nature of an object that can exist on the battlefield or
/// stack. A token is explicit state, never inferred from a synthetic card
/// definition. Its authored characteristics live on the permanent or frozen
/// stack presentation that owns them.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ObjectKind {
    Card(CardDefinitionId),
    Token,
    /// An emblem is a command-zone game object with creator-owned
    /// characteristics, not a synthetic card definition.
    Emblem,
    /// A stack ability has an object ID but is neither a card nor a token.
    Ability,
}

impl ObjectKind {
    const fn card_definition(self) -> Option<CardDefinitionId> {
        match self {
            Self::Card(definition) => Some(definition),
            Self::Token | Self::Emblem | Self::Ability => None,
        }
    }

    const fn is_token(self) -> bool {
        matches!(self, Self::Token)
    }
}

impl PartialEq<CardDefinitionId> for ObjectKind {
    fn eq(&self, other: &CardDefinitionId) -> bool {
        matches!(self, Self::Card(definition) if definition == other)
    }
}

impl PartialEq<ObjectKind> for CardDefinitionId {
    fn eq(&self, other: &ObjectKind) -> bool {
        other == self
    }
}

/// An object on the battlefield or stack. Cards convert into this shell when
/// they enter either zone; tokens are minted directly without ever pretending
/// to be a [`CardInstance`].
#[derive(Clone, Debug, Eq, PartialEq)]
struct ObjectInstance {
    id: GameObjectId,
    definition: ObjectKind,
    owner: PlayerId,
    backing: ObjectBacking,
    characteristics: CharacteristicSource,
    counters: [u16; CounterKind::COUNT],
}

impl From<CardInstance> for ObjectInstance {
    fn from(card: CardInstance) -> Self {
        Self {
            id: card.id,
            definition: ObjectKind::Card(card.definition),
            owner: card.owner,
            backing: card.backing,
            characteristics: card.characteristics,
            counters: card.counters,
        }
    }
}

impl ObjectInstance {
    fn into_card(self) -> Option<CardInstance> {
        Some(CardInstance {
            id: self.id,
            definition: self.definition.card_definition()?,
            owner: self.owner,
            backing: self.backing,
            characteristics: self.characteristics,
            counters: self.counters,
        })
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
#[allow(clippy::struct_excessive_bools)]
struct StackObject {
    id: GameObjectId,
    kind: StackObjectKind,
    card: ObjectInstance,
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
    /// Colours imposed on this object by a copy effect or a resolving
    /// characteristic effect, such as "except that the copy is red" or a
    /// Lace. The override lasts for this stack incarnation.
    colors: Option<ColorSet>,
    /// Flashback replaces every destination this physical card would use when
    /// leaving the stack. This is frozen at cast time because the permission
    /// lived on the previous graveyard object.
    cast_via_flashback: bool,
    /// Whether this spell was cast at a time a sorcery could not have been.
    /// Recorded as the cast happens, because nothing afterwards can tell.
    cast_at_instant_speed: bool,
    /// Whether this spell was cast from its controller's hand. "If you cast
    /// it from your hand" distinguishes the ordinary cast from one made off
    /// the top of a library or out of a graveyard, and by the time the
    /// permanent's own trigger resolves nothing else remembers.
    cast_from_zone: Option<CastSourceZone>,
    /// The copiable characteristics supplied by the rule that allowed this
    /// spell to be cast face down. The permanent it becomes keeps the same
    /// values, while only its controller may inspect the physical card.
    face_down: Option<FaceDownCharacteristics>,
    /// Which colours of mana actually paid for this spell, for the clauses
    /// that count them (CR 702.86a, converge). Payment-derived rather than
    /// part of the cast signature: a copy is never cast, so no mana was ever
    /// spent on it and its count is zero however the original was paid for.
    colors_of_mana_spent: ColorSet,
    /// How many Phyrexian mana symbols were paid with life. Like mana colors
    /// spent, this is a fact about the payment rather than a copiable cast
    /// characteristic; spell copies always clear it.
    phyrexian_symbols_paid_with_life: u16,
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
    presentation: ObjectCharacteristics,
    text: Option<&'static str>,
    target_defs: Vec<AbilityTargetDef>,
    targets: Vec<TargetSelection>,
    context: EffectResolutionContext,
    resolver: StackAbilityResolver,
    /// The intervening-if condition, re-read as this ability resolves.
    condition: Option<&'static TriggerConditionDef>,
    /// Selected declarative mode effects frozen in canonical printed order.
    /// Repeated modes remain repeated procedures.
    mode_effects: Vec<ScopedEffect>,
    /// Where a successfully resolving spell card goes. This is frozen as a
    /// property of the stack object because optional additional costs can
    /// change it independently of the ability whose instructions resolve.
    /// Activated and triggered abilities carry `None`.
    resolution_destination: Option<SpellResolutionDestinationDef>,
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
    /// A linked triggered ability whose resolution offers its source card
    /// for one exact alternative cost.
    CastOffer(AlternativeCastKindDef),
}

impl StackAbilityResolver {
    fn linked_cast_offer(ability: &AbilityDef) -> Option<Self> {
        match ability.definition {
            DeclarativeAbilityDef::AlternativeCast(alternative)
                if ability.is_executable()
                    && alternative.kind == AlternativeCastKindDef::Miracle =>
            {
                Some(Self::CastOffer(alternative.kind))
            }
            _ => None,
        }
    }
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

    fn presentation(&self) -> ObjectCharacteristics {
        self.ability.as_ref().map_or_else(
            || {
                ObjectCharacteristics::card(
                    self.card
                        .definition
                        .card_definition()
                        .expect("a spell object is backed by a card definition"),
                    self.signature
                        .as_ref()
                        .and_then(|signature| match signature.form() {
                            crate::card::SpellForm::Part(part) => Some(*part),
                            crate::card::SpellForm::Combined(parts) => parts.first().copied(),
                        })
                        .unwrap_or(CardPartId::PRIMARY),
                )
            },
            |ability| ability.presentation,
        )
    }

    fn targets(&self) -> Vec<Target> {
        self.iter_targets().copied().collect()
    }

    /// Targets announced for this spell or ability. Installed abilities may
    /// retain an earlier ability's selections as lexical references for
    /// resolution, but those are not targets of the triggered ability and
    /// must not be presented publicly as though they were chosen again.
    fn declared_targets(&self) -> Vec<Target> {
        if let Some(signature) = &self.signature {
            return signature.iter_targets().copied().collect();
        }
        self.ability
            .iter()
            .flat_map(|ability| {
                ability
                    .targets
                    .iter()
                    .take(ability.target_defs.len())
                    .flat_map(TargetSelection::targets)
            })
            .copied()
            .collect()
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

    /// How many colours paid for this object, which is zero for everything
    /// that was never cast.
    fn colors_spent_count(&self) -> u8 {
        self.colors_of_mana_spent
            .to_flags()
            .iter()
            .filter(|spent| **spent)
            .count()
            .try_into()
            .unwrap_or(u8::MAX)
    }
}

/// Which way an attachment goes as a permanent enters, with both objects
/// already resolved to the identities they have now.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ArrivalAttachment {
    SourceToArrival(GameObjectId),
    ArrivalToHost(GameObjectId),
}

/// How a card arrives when an effect puts it onto the battlefield.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct BattlefieldArrival {
    pub(super) controller: PlayerId,
    pub(super) tapped: bool,
    /// The attachment this permanent makes as it enters, in whichever
    /// direction. "Put target creature card onto the battlefield and attach
    /// this to it" cannot be two steps: what arrives is a new object, and by
    /// the time a following effect ran there would be nothing to name.
    pub(super) attachment: Option<ArrivalAttachment>,
    /// Whether a double-faced card arrives showing its back face. "Return
    /// him to the battlefield transformed" says which face enters, so it
    /// belongs to the arrival rather than to a transform afterwards: what
    /// enters is a new object, and a following effect would have nothing
    /// left to name.
    pub(super) transformed: bool,
    /// Copiable face-down values established before this permanent enters.
    /// `None` means it arrives face up.
    pub(super) face_down: Option<FaceDownCharacteristics>,
    /// Whether this mechanism lets an underlying creature card turn face up
    /// for its own mana cost, as Manifest and Cloak do.
    pub(super) turn_up_for_mana_cost: bool,
    /// Counters the permanent arrives carrying. They belong to the arrival
    /// because an enters trigger reading the permanent's power has to see
    /// them: the counters were on it as it entered, not put there after.
    pub(super) counters: Option<(CounterKind, u16)>,
}

impl BattlefieldArrival {
    pub(super) const fn under(controller: PlayerId) -> Self {
        Self {
            controller,
            tapped: false,
            transformed: false,
            attachment: None,
            face_down: None,
            turn_up_for_mana_cost: false,
            counters: None,
        }
    }

    /// Puts a card onto the battlefield with mechanism-owned face-down values
    /// and the mechanism's mana-cost turn-up permission.
    pub(super) const fn face_down_under(
        controller: PlayerId,
        face_down: FaceDownCharacteristics,
        turn_up_for_mana_cost: bool,
    ) -> Self {
        Self {
            controller,
            tapped: false,
            transformed: false,
            attachment: None,
            face_down: Some(face_down),
            turn_up_for_mana_cost,
            counters: None,
        }
    }

    pub(super) const fn tapped_under(controller: PlayerId) -> Self {
        Self {
            controller,
            tapped: true,
            transformed: false,
            attachment: None,
            face_down: None,
            turn_up_for_mana_cost: false,
            counters: None,
        }
    }

    pub(super) const fn transformed_under(controller: PlayerId) -> Self {
        Self {
            controller,
            tapped: false,
            transformed: true,
            attachment: None,
            face_down: None,
            turn_up_for_mana_cost: false,
            counters: None,
        }
    }

    /// The resolving source attaches to what arrives.
    pub(super) const fn attaching(mut self, source: GameObjectId) -> Self {
        self.attachment = Some(ArrivalAttachment::SourceToArrival(source));
        self
    }

    /// What arrives attaches to a permanent already on the battlefield.
    pub(super) const fn attached_to(mut self, host: GameObjectId) -> Self {
        self.attachment = Some(ArrivalAttachment::ArrivalToHost(host));
        self
    }

    /// What arrives carries these counters.
    pub(super) const fn with_counters(mut self, counters: Option<(CounterKind, u16)>) -> Self {
        self.counters = counters;
        self
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
    /// Poison counters this player has been given. Ten of them is a loss,
    /// checked as a state-based action alongside life and library.
    poison: u16,
    /// Energy counters this player has (CR 122.1a). Unlike poison it is a
    /// resource rather than a clock: it is spent, no amount of it wins or
    /// loses anything, and it persists between turns.
    energy: u16,
}

#[derive(Clone, Debug)]
#[allow(clippy::struct_excessive_bools)]
pub struct Game {
    /// The battlefield object the most recent entry committed, so a move can
    /// hand back the identity the permanent actually got rather than the one
    /// the card had in the zone it came from. Consumed immediately by that
    /// move; never observed anywhere else.
    arrived: Option<GameObjectId>,
    /// The X a cast is being considered at, while its targets are being
    /// enumerated or checked. A spell is not on the stack yet at that point,
    /// so its chosen X is not readable from the object -- but which
    /// creatures are legal targets can depend on it, and the enumerator
    /// already walks one X at a time.
    prospective_x: prospective_x::ProspectiveX,
    /// What each retired object became when it changed zones. A trigger
    /// captured on the battlefield names the object that was there, and
    /// "return it to its owner's hand" has to reach the card that object is
    /// now -- which is a different identity, allocated as it moved.
    successors: std::collections::HashMap<GameObjectId, GameObjectId>,
    format: Format,
    seed: u64,
    rng: ReplayRng,
    catalog: CardCatalog,
    #[allow(dead_code)] // Reserved for backing validation and future meld actions.
    physical_cards: Vec<PhysicalCard>,
    players: [PlayerState; 2],
    battlefield: Vec<Permanent>,
    /// Permanents that are phased out. A phased-out permanent is treated as
    /// though it does not exist (CR 702.25), so it is held here rather than
    /// left on the battlefield behind a flag: every walk over the
    /// battlefield is then correct without knowing about phasing at all.
    /// Phasing is not a zone change, so nothing about the object is
    /// disturbed while it waits.
    ///
    /// A checkpoint taken while anything is phased out is refused: a
    /// phased-out permanent is absent from every observation, so the
    /// reconstruction has nothing to rebuild it from. No registered deck can
    /// phase anything today, and carrying these is what the deck that can
    /// will need first.
    phased_out: Vec<Permanent>,
    stack: GameStack,
    retired_objects: BTreeMap<GameObjectId, RetiredObject>,
    /// Abilities granted to non-battlefield object incarnations until cleanup.
    temporary_ability_grants: Vec<TemporaryAbilityGrant>,
    next_object_id: u32,
    next_continuous_effect_timestamp: u64,
    turn: u32,
    turns_started: [u32; 2],
    /// Damage each player has been dealt this turn, in total and by the
    /// named source groups. Accumulated as the damage is dealt, since a
    /// group such as "unblocked creatures" is only answerable then.
    damage_taken_this_turn: [u16; 2],
    damage_taken_by_group_this_turn: [[u16; DamageSourceGroupDef::COUNT]; 2],
    active_player: PlayerId,
    priority: PlayerId,
    consecutive_passes: u8,
    step: Step,
    attackers_declared: bool,
    /// Whether a creature has died so far this turn, for morbid. Cleared as a
    /// turn begins rather than in cleanup, so a morbid spell cast during the
    /// end step still sees the creature that died in combat.
    creature_died_this_turn: bool,
    /// How many creatures have died this turn. The flag above answers "did
    /// one die"; a card that scales with the count needs the number, and one
    /// death has to move both together.
    creatures_died_this_turn: u16,
    /// Cards exiled by an object that promises to bring them back, paired
    /// with whatever exiled them. Oblivion Ring is the shape.
    linked_exiles: Vec<(GameObjectId, GameObjectId)>,
    /// Which players have had a permanent leave the battlefield from under
    /// their control this turn, which is what revolt asks (CR 702.121a). The
    /// board afterwards cannot tell: a permanent that left and was replaced
    /// leaves a battlefield that looks untouched.
    permanent_left_battlefield_this_turn: [bool; 2],
    /// Whether a card has left each player's graveyard this turn. "If a card
    /// left your graveyard this turn" is a fact about the turn rather than
    /// about any card, and by the time an end step asks, the card it is
    /// about is somewhere else entirely.
    card_left_graveyard_this_turn: [bool; 2],
    /// Which players have the city's blessing (CR 702.131a). It is gained for
    /// the rest of the game, so this only ever turns on.
    citys_blessing: [bool; 2],
    /// How much life each player has gained this turn. "If you gained life
    /// this turn" is a fact about what happened rather than about the life
    /// total, which a loss in between would hide.
    life_gained_this_turn: [u16; 2],
    /// The monarch, if anyone is (CR 720). There is at most one, they draw
    /// a card at the beginning of their end step, and a creature that deals
    /// combat damage to them hands the crown to its controller.
    monarch: Option<PlayerId>,
    /// Who the creature a ninjutsu cost just returned was attacking. Read
    /// as the cost is paid and consumed as the ability resolves, because by
    /// then the creature is in hand and cannot be asked.
    ninjutsu_returned_defender: Option<AttackDefender>,
    /// Cards in exile somebody has been given permission to play from
    /// there. Object ids are allocated per zone change and never reused, so
    /// an entry is dropped when the card is played and cannot otherwise be
    /// mistaken for a later object.
    exile_play_permissions: Vec<ExilePlayPermission>,
    /// Whether "damage can't be prevented this turn" is in force. Stomp
    /// prints it, and it is a rule about the whole turn rather than about
    /// any one damage event, so it is read where damage is dealt rather
    /// than installed as a prevention of its own.
    damage_cannot_be_prevented_this_turn: bool,
    /// How many of each player's next sorceries may be cast as though they
    /// had flash. Quicken grants one, and the grant lapses with the turn.
    sorcery_flash_grants: [u8; 2],
    /// Additional major phases that will happen after the current phase. New
    /// sequences are prepended, matching the newest-first ordering rule for
    /// multiple effects that add phases after the same boundary.
    turn_phase_queue: VecDeque<TurnPhaseDef>,
    /// The ordinary continuation displaced when the first queued phase starts.
    /// It stays frozen while nested phase schedules prepend more work.
    turn_phase_resume: Option<TurnPhaseResume>,
    /// Resolving play prohibitions in creation/component order. Static
    /// prohibitions remain source-derived from battlefield abilities.
    resolved_play_restrictions: Vec<ResolvedPlayRestriction>,
    /// Resolving restrictions on which creatures may attack a player.
    /// Creature-scoped printed restrictions remain derived from abilities.
    resolved_attack_restrictions: Vec<ResolvedAttackRestriction>,
    /// Resolving play permissions, the mirror of the prohibitions above.
    /// "You may cast spells from your graveyard this turn" is aimed at a
    /// player and lasts no longer than the turn, so nothing on the
    /// battlefield can be asked for it afterwards.
    resolved_play_permissions: Vec<ResolvedPlayPermission>,
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
    /// Whether each player has already taken the one draw their own draw
    /// step spares from Orcish Bowmasters. Reset as that step begins rather
    /// than at the turn's start, so "each of their draw steps" stays true of
    /// a turn that somehow holds more than one.
    draw_step_draw_taken: [bool; 2],
    /// The cards each player drew this turn, in draw order. Sylvan Library
    /// chooses among them, and only a card still in hand can be chosen.
    drawn_this_turn: [Vec<GameObjectId>; 2],
    /// Set while a resumable all-player draw instruction is in progress.
    /// Empty-library loss itself is settled at the next state-based-action
    /// check using the flags stored on each player.
    defer_empty_library_loss: bool,
    /// One-shot draw replacements, in creation order for each player.
    draw_replacements: [VecDeque<DrawReplacement>; 2],
    /// Resolved damage-prevention rules in creation order. Static prevention
    /// is derived live from the ability that creates it and is not stored.
    damage_preventions: Vec<ResolvedDamagePrevention>,
    /// Resolved damage-redirection replacements in creation order. These are
    /// applied before prevention and remain separate from prevention state.
    damage_redirects: Vec<ResolvedDamageRedirect>,
    /// Triggered abilities installed by resolved effects and listening from
    /// outside every zone.
    installed_triggers: Vec<InstalledTrigger>,
    next_installed_trigger_id: u32,
    blockers_declared: bool,
    untap_pending: bool,
    pregame: Option<Pregame>,
    mulligans: [u8; 2],
    cleanup_pending: bool,
    pending_decisions: Vec<PendingDecision>,
    /// What to run when the discard now being answered finishes. One discard
    /// effect is in flight at a time, so this is a slot rather than a queue.
    pending_discard_follow_up: Option<DiscardFollowUp>,
    next_decision_id: u32,
    pending_events: VecDeque<PendingEvent>,
    pending_procedures: VecDeque<PendingProcedure>,
    pending_triggers: Vec<PendingTrigger>,
    next_trigger_id: u32,
    last_seen_hands: [LastSeenHand; 2],
    /// Creatures still owing a combat damage assignment this step. Attackers
    /// dividing among their blockers, and blockers dividing among the
    /// attackers they block, wait in the same queue.
    pending_combat_assignments: Vec<GameObjectId>,
    combat_damage_stage: CombatDamageStage,
    combat_blocked_attackers: Vec<GameObjectId>,
    /// The next ordinary turn in the two-player rotation. Extra turns sit in
    /// front of this anchor without changing it.
    next_regular_player: PlayerId,
    extra_turns: Vec<PlayerId>,
    channel_active: [bool; 2],
    result: Option<GameResult>,
    events: Vec<GameEvent>,
}

include!("permanent.rs");
include!("game_stack.rs");

#[cfg(test)]
mod planeswalker_combat_tests;
#[cfg(test)]
mod planeswalker_tests;
#[cfg(test)]
mod tests;
