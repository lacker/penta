#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct AbilitySourceRef {
    pub(super) object: GameObjectId,
    pub(super) ability: AbilityOrigin,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PendingTrigger {
    pub(super) id: u32,
    pub(super) source: AbilitySourceRef,
    pub(super) presentation: ObjectCharacteristics,
    pub(super) owner: PlayerId,
    pub(super) controller: PlayerId,
    pub(super) text: &'static str,
    pub(super) target_defs: Vec<AbilityTargetDef>,
    pub(super) targets: Vec<TargetSelection>,
    pub(super) effect: EffectDef,
    pub(super) resolver: StackAbilityResolver,
    pub(super) context: EffectResolutionContext,
    pub(super) condition: Option<&'static TriggerConditionDef>,
    /// "Choose one --": the modes still to be chosen as this trigger is put
    /// onto the stack. Cleared once one is, because what the trigger then
    /// carries is that mode's own effect and targets.
    pub(super) modes: Option<ModalSpellDef>,
    pub(super) x: u16,
}

/// The immutable declaration captured when one event matches one source
/// ability. The game assigns the ephemeral trigger ID when it accepts this
/// record into the pending-trigger queue.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct TriggerCapture {
    pub(super) source: AbilitySourceRef,
    pub(super) presentation: ObjectCharacteristics,
    pub(super) owner: PlayerId,
    pub(super) controller: PlayerId,
    pub(super) text: &'static str,
    pub(super) target_defs: Vec<AbilityTargetDef>,
    pub(super) targets: Vec<TargetSelection>,
    pub(super) effect: EffectDef,
    pub(super) resolver: StackAbilityResolver,
    pub(super) context: EffectResolutionContext,
    /// The intervening-if condition this trigger reads, checked both when the
    /// ability would go on the stack and again when it resolves.
    pub(super) condition: Option<&'static TriggerConditionDef>,
    /// "Choose one --": the modes still to be chosen as this trigger is put
    /// onto the stack. Cleared once one is, because what the trigger then
    /// carries is that mode's own effect and targets.
    pub(super) modes: Option<ModalSpellDef>,
    /// The X chosen for the installing ability. Installed triggers retain the
    /// same resolving context as the effect that created them.
    pub(super) x: u16,
}

/// How long a trigger installed outside every zone continues listening.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum InstalledTriggerLifetime {
    /// Consume the listener on the first matching event, before checking an
    /// intervening-if condition or putting its ability on the stack.
    Once,
    /// Stop listening when this player's frozen future turn begins.
    UntilTurn { player: PlayerId, turn: u32 },
    /// Stop listening when the frozen turn it was installed on ends. The
    /// turn number is the game's own count, so an extra turn ends this the
    /// same way an ordinary one does.
    ThisTurn { turn: u32 },
}

/// A triggered ability installed by a resolved effect. Everything needed to
/// construct its stack object is frozen here because its source may be gone
/// by the time an event matches.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct InstalledTrigger {
    pub(super) id: u32,
    pub(super) event: TriggerEventDef,
    pub(super) capture: TriggerCapture,
    pub(super) lifetime: InstalledTriggerLifetime,
}

/// One battlefield trigger listener frozen at the start of an atomic event.
/// A simultaneous zone change can remove the source before another object in
/// the same event is published, so listener discovery cannot consult the
/// incrementally-mutated battlefield.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct BattlefieldTriggerListener {
    pub(super) event: TriggerEventDef,
    pub(super) uses_stack: bool,
    /// "This ability triggers only once each turn", carried from the
    /// printed definition so the capture can count without rediscovering
    /// which ability it came from.
    pub(super) trigger_limit: Option<u8>,
    /// Identifies an effect-installed listener. Battlefield listeners have no
    /// ID because their source's zone presence determines their lifetime.
    pub(super) installed: Option<u32>,
    pub(super) capture: TriggerCapture,
}

#[derive(Clone, Debug)]
pub(super) struct TriggerPlacementBatch {
    pub(super) controller: PlayerId,
    pub(super) triggers: Vec<PendingTrigger>,
}
