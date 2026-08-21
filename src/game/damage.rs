use crate::card::{
    DamageEventMatcherDef, DamageKindDef, DamageLimitDef, DamageRecipientMatcherDef,
    DamageSourceGroupDef, DamageSourceMatcherDef, ObjectRefDef,
};

use super::prevention_state::{
    ResolvedDamagePrevention, ResolvedDamagePreventionCapacity, ResolvedDamagePreventionCoverage,
    ResolvedDamageRecipientMatcher, ResolvedDamageSourceMatcher,
};
use super::{
    AppliedRuleDef, CardType, CardTypeSet, CommittedTriggerEvent, ControlFlow, CounterKind, Game,
    GameObjectId, KeywordAbility, Permanent, PlayerId, RelationalSourceFilter, RetiredObject,
    StackObjectKind, Target, TriggerEventObject,
};

#[derive(Clone, Copy)]
struct ProspectiveDamage<'a> {
    source: Option<GameObjectId>,
    source_object: Option<&'a TriggerEventObject>,
    source_is_spell: bool,
    target: Option<Target>,
    recipient_object: Option<&'a TriggerEventObject>,
    combat: bool,
}

impl Game {
    pub(super) fn damage_target(&mut self, target: Option<Target>, amount: u16) -> u16 {
        self.damage_target_from(None, target, amount)
    }

    /// Apply resolved prevention in creation order. Consumable promises are
    /// spent before unlimited prevention, matching the engine's historical
    /// Reverse Damage-before-Safe Passage behavior. A matching event promise
    /// is spent even when half of one damage rounds down to zero.
    fn apply_resolved_damage_prevention(
        &mut self,
        event: ProspectiveDamage<'_>,
        amount: u16,
    ) -> u16 {
        if amount == 0 || event.target.is_none() {
            return amount;
        }

        // Detach the vector while matching so predicate evaluation can read
        // the rest of the game without allocating a parallel match bitmap on
        // every damage event.
        let mut preventions = std::mem::take(&mut self.damage_preventions);
        let mut left = amount;
        let mut gained_life = Vec::new();

        for prevention in &mut preventions {
            if left == 0 || !self.resolved_damage_prevention_matches(prevention, event) {
                continue;
            }
            let prevented = match &mut prevention.capacity {
                ResolvedDamagePreventionCapacity::Amount(remaining) => {
                    let prevented = Self::damage_covered(prevention.coverage, left).min(*remaining);
                    *remaining -= prevented;
                    prevented
                }
                ResolvedDamagePreventionCapacity::Events(remaining) => {
                    *remaining = remaining.saturating_sub(1);
                    Self::damage_covered(prevention.coverage, left)
                }
                ResolvedDamagePreventionCapacity::Unlimited => continue,
            };
            left -= prevented;
            if let Some(player) = prevention.gain_life
                && prevented > 0
            {
                gained_life.push((player, prevented));
            }
        }

        preventions.retain(|prevention| {
            !matches!(
                prevention.capacity,
                ResolvedDamagePreventionCapacity::Amount(0)
                    | ResolvedDamagePreventionCapacity::Events(0)
            )
        });

        if left > 0 {
            for prevention in &preventions {
                if left == 0 {
                    break;
                }
                if !matches!(
                    prevention.capacity,
                    ResolvedDamagePreventionCapacity::Unlimited
                ) || !self.resolved_damage_prevention_matches(prevention, event)
                {
                    continue;
                }
                let prevented = Self::damage_covered(prevention.coverage, left);
                left -= prevented;
                if let Some(player) = prevention.gain_life
                    && prevented > 0
                {
                    gained_life.push((player, prevented));
                }
            }
        }

        self.damage_preventions = preventions;
        for (player, prevented) in gained_life {
            self.gain_life(player, prevented);
        }
        left
    }

    const fn damage_covered(coverage: ResolvedDamagePreventionCoverage, amount: u16) -> u16 {
        match coverage {
            ResolvedDamagePreventionCoverage::All => amount,
            ResolvedDamagePreventionCoverage::HalfRoundedDown => amount / 2,
        }
    }

    fn resolved_damage_prevention_matches(
        &self,
        prevention: &ResolvedDamagePrevention,
        event: ProspectiveDamage<'_>,
    ) -> bool {
        (!prevention.combat_only || event.combat)
            && self.resolved_damage_source_matches(
                prevention.source,
                event.source,
                event.source_object,
                event.source_is_spell,
            )
            && Self::resolved_damage_recipient_matches(
                prevention.recipient,
                event.target,
                event.recipient_object,
            )
    }

    fn resolved_damage_source_matches(
        &self,
        matcher: ResolvedDamageSourceMatcher,
        source: Option<GameObjectId>,
        source_object: Option<&TriggerEventObject>,
        source_is_spell: bool,
    ) -> bool {
        match matcher {
            ResolvedDamageSourceMatcher::Any => true,
            ResolvedDamageSourceMatcher::Exact(expected) => source == Some(expected),
            ResolvedDamageSourceMatcher::Except(excluded) => source != Some(excluded),
            ResolvedDamageSourceMatcher::Matching {
                predicate,
                relative_to,
            } => source_object.is_some_and(|source| {
                self.trigger_object_matches(predicate, source, relative_to, source_is_spell)
            }),
            ResolvedDamageSourceMatcher::Group(group) => {
                source.is_some_and(|source| self.damage_source_is_in_group(source, group))
            }
        }
    }

    fn resolved_damage_recipient_matches(
        matcher: ResolvedDamageRecipientMatcher,
        target: Option<Target>,
        recipient_object: Option<&TriggerEventObject>,
    ) -> bool {
        match matcher {
            ResolvedDamageRecipientMatcher::Any => target.is_some(),
            ResolvedDamageRecipientMatcher::Exact(expected) => target == Some(expected),
            ResolvedDamageRecipientMatcher::PlayerAndCreaturesControlledBy(player) => {
                match target {
                    Some(Target::Player(recipient)) => recipient == player,
                    Some(Target::Permanent(id)) => recipient_object.is_some_and(|recipient| {
                        recipient.id == id
                            && recipient.controller == player
                            && recipient.types.contains(CardType::Creature)
                    }),
                    Some(Target::Card(_) | Target::Spell(_)) | None => false,
                }
            }
        }
    }

    /// Static prevention is derived live. Both prospective participants are
    /// visited because an applied effect can describe damage either to or by
    /// its affected object. A departed damage source is still represented by
    /// its last-known characteristics when a live recipient's predicate asks
    /// what dealt the damage.
    fn static_damage_is_prevented(&self, event: ProspectiveDamage<'_>) -> bool {
        let target_permanent = event.target.and_then(|target| match target {
            Target::Permanent(id) => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == id),
            Target::Player(_) | Target::Card(_) | Target::Spell(_) => None,
        });
        if target_permanent
            .is_some_and(|affected| self.static_damage_is_prevented_on(affected, event))
        {
            return true;
        }

        event
            .source
            .and_then(|source| {
                self.battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == source)
            })
            .filter(|affected| {
                target_permanent.is_none_or(|target| target.card.id != affected.card.id)
            })
            .is_some_and(|affected| self.static_damage_is_prevented_on(affected, event))
    }

    fn static_damage_is_prevented_on(
        &self,
        affected: &Permanent,
        event: ProspectiveDamage<'_>,
    ) -> bool {
        self.visit_applied_rules(affected, |applied| {
            if matches!(applied.rule, AppliedRuleDef::PreventDamage(matcher)
            if self.static_damage_matcher_matches(
                matcher,
                applied.source,
                affected.card.id,
                event,
            )) {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        })
        .is_break()
    }

    fn static_damage_matcher_matches(
        &self,
        matcher: DamageEventMatcherDef,
        effect_source: GameObjectId,
        affected: GameObjectId,
        event: ProspectiveDamage<'_>,
    ) -> bool {
        (matcher.kind == DamageKindDef::Any || event.combat)
            && match matcher.source {
                DamageSourceMatcherDef::Any => true,
                DamageSourceMatcherDef::AffectedObject => event.source == Some(affected),
                DamageSourceMatcherDef::Object(reference) => self
                    .static_object_reference(reference, effect_source)
                    .is_some_and(|expected| event.source == Some(expected)),
                DamageSourceMatcherDef::Except(reference) => self
                    .static_object_reference(reference, effect_source)
                    .is_some_and(|excluded| event.source != Some(excluded)),
                DamageSourceMatcherDef::Matching(predicate) => {
                    event.source_object.is_some_and(|source| {
                        self.trigger_object_matches(
                            predicate,
                            source,
                            effect_source,
                            event.source_is_spell,
                        )
                    })
                }
                DamageSourceMatcherDef::Group(group) => event.source.is_some_and(|source| {
                    self.damage_source_is_in_group(source, Self::relational_source_filter(group))
                }),
            }
            && match matcher.recipient {
                DamageRecipientMatcherDef::Any => event.target.is_some(),
                DamageRecipientMatcherDef::AffectedObject => event
                    .recipient_object
                    .is_some_and(|recipient| recipient.id == affected),
                DamageRecipientMatcherDef::Recipients(recipients) => recipients
                    .object_reference()
                    .and_then(|reference| self.static_object_reference(reference, effect_source))
                    .is_some_and(|recipient| {
                        event
                            .recipient_object
                            .is_some_and(|object| object.id == recipient)
                    }),
                // Both exist for trigger matching rather than for a static
                // shield, and the validation above refuses either here.
                DamageRecipientMatcherDef::PlayerAndCreaturesControlledBy(_)
                | DamageRecipientMatcherDef::PlayerOrPlaneswalker => false,
            }
    }

    const fn relational_source_filter(group: DamageSourceGroupDef) -> RelationalSourceFilter {
        match group {
            DamageSourceGroupDef::CreaturesWithFlying => {
                RelationalSourceFilter::CreaturesWithFlying
            }
            DamageSourceGroupDef::AttackingCreaturesWithoutFlying => {
                RelationalSourceFilter::AttackingCreaturesWithoutFlying
            }
            DamageSourceGroupDef::Artifacts => RelationalSourceFilter::Artifacts,
            DamageSourceGroupDef::UnblockedCreatures => RelationalSourceFilter::UnblockedCreatures,
        }
    }

    /// Whether one source belongs to a named group. Membership is evaluated
    /// when damage would be dealt, so attacking and keyword state stay live.
    fn damage_source_is_in_group(
        &self,
        source: GameObjectId,
        group: RelationalSourceFilter,
    ) -> bool {
        let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
        else {
            return false;
        };
        if group == super::RelationalSourceFilter::Artifacts {
            return self
                .permanent_types(permanent)
                .is_some_and(|types| types.contains(CardType::Artifact));
        }
        if !self
            .permanent_types(permanent)
            .is_some_and(|types| types.contains(CardType::Creature))
        {
            return false;
        }
        let flying = self.permanent_has_executable_keyword(permanent, KeywordAbility::Flying);
        match group {
            RelationalSourceFilter::CreaturesWithFlying => flying,
            RelationalSourceFilter::AttackingCreaturesWithoutFlying => {
                permanent.attacking && !flying
            }
            super::RelationalSourceFilter::UnblockedCreatures => {
                permanent.attacking
                    && !self
                        .battlefield
                        .iter()
                        .any(|blocker| blocker.is_blocking(source))
            }
            // Not a creature question at all, so it is asked before the one
            // above rather than through it.
            super::RelationalSourceFilter::Artifacts => {
                unreachable!("handled before the type gate")
            }
        }
    }

    fn static_object_reference(
        &self,
        reference: ObjectRefDef,
        effect_source: GameObjectId,
    ) -> Option<GameObjectId> {
        match reference {
            ObjectRefDef::Source | ObjectRefDef::ResolvingObject => Some(effect_source),
            ObjectRefDef::AttachedToSource => {
                self.current_or_last_known_attached_host(effect_source)
            }
            ObjectRefDef::AbilityGrantSource
            | ObjectRefDef::Binding(_)
            | ObjectRefDef::Target(_)
            | ObjectRefDef::SourceOfTargetedStackObject(_)
            | ObjectRefDef::TriggeringObject => None,
        }
    }

    fn damage_source_is_spell(&self, source: GameObjectId) -> bool {
        self.stack
            .iter()
            .find(|object| object.id == source)
            .is_some_and(|object| object.kind == StackObjectKind::Spell)
            || matches!(
                self.retired_objects.get(&source),
                Some(RetiredObject::Stack(object)) if object.kind == StackObjectKind::Spell
            )
    }

    pub(super) fn damage_target_from(
        &mut self,
        source: Option<GameObjectId>,
        target: Option<Target>,
        amount: u16,
    ) -> u16 {
        self.damage_target_from_kind(source, target, amount, false)
    }

    /// Where damage actually lands. A permanent whose static effect redirects
    /// its controller's damage takes it instead, provided the source is in
    /// the group that effect names.
    pub(super) fn redirected_damage_target(
        &self,
        source: Option<GameObjectId>,
        target: Option<Target>,
    ) -> Option<Target> {
        let Some(Target::Player(player)) = target else {
            return target;
        };
        let Some(source) = source else {
            return target;
        };
        if let Some(destination) = self
            .damage_redirects
            .iter()
            .find(|redirect| redirect.player == player && redirect.source == source)
            .map(|redirect| redirect.destination)
        {
            return Some(Target::Permanent(destination));
        }
        for candidate in &self.battlefield {
            if candidate.controller != player {
                continue;
            }
            let mut redirects = false;
            let _ = self.visit_applied_rules(candidate, |applied| {
                if let AppliedRuleDef::RedirectPlayerDamageToThis(group) = applied.rule
                    && self.damage_source_is_in_group(source, Self::relational_source_filter(group))
                {
                    redirects = true;
                    ControlFlow::Break(())
                } else {
                    ControlFlow::Continue(())
                }
            });
            if redirects {
                return Some(Target::Permanent(candidate.card.id));
            }
        }
        target
    }

    #[allow(clippy::too_many_lines)]
    /// Records damage a player has been dealt this turn, in total and under
    /// each source group it belongs to. The groups are answered now rather
    /// than later: "damage dealt by unblocked creatures" stops being
    /// answerable once combat is over.
    fn record_damage_taken(&mut self, player: PlayerId, amount: u16, source: Option<GameObjectId>) {
        if amount == 0 {
            return;
        }
        self.damage_taken_this_turn[player.index()] =
            self.damage_taken_this_turn[player.index()].saturating_add(amount);
        let Some(source) = source else {
            return;
        };
        for group in crate::card::DamageSourceGroupDef::ALL {
            if self.damage_source_is_in_group(source, Self::relational_source_filter(group)) {
                let slot = &mut self.damage_taken_by_group_this_turn[player.index()][group.index()];
                *slot = slot.saturating_add(amount);
            }
        }
    }

    pub(super) fn damage_taken_this_turn(
        &self,
        player: PlayerId,
        group: Option<crate::card::DamageSourceGroupDef>,
    ) -> u16 {
        match group {
            None => self.damage_taken_this_turn[player.index()],
            Some(group) => self.damage_taken_by_group_this_turn[player.index()][group.index()],
        }
    }

    /// Damage landing on a player: the life change, the running total the
    /// turn keeps, and the note a source makes of having connected.
    fn deal_damage_to_player(
        &mut self,
        player: PlayerId,
        amount: u16,
        source: Option<GameObjectId>,
        combat: bool,
    ) {
        self.record_damage_taken(player, amount, source);
        self.deal_damage(player, amount);
        self.note_damage_dealt_by(source, amount);
        if combat {
            self.combat_damage_may_steal_the_crown(source, player, amount);
        }
        if amount > 0
            && let Some(damager) = source.and_then(|source| {
                self.battlefield
                    .iter_mut()
                    .find(|permanent| permanent.card.id == source)
            })
            && damager.controller != player
        {
            damager.dealt_damage_to_opponent_this_turn = true;
        }
    }

    /// Records that `source` dealt damage, whatever it landed on. A
    /// planeswalker taking loyalty loss counts, which is why this is called
    /// before that branch rather than beside the damage marks.
    fn note_damage_dealt_by(&mut self, source: Option<GameObjectId>, amount: u16) {
        if amount == 0 {
            return;
        }
        if let Some(damager) = source.and_then(|source| {
            self.battlefield
                .iter_mut()
                .find(|permanent| permanent.card.id == source)
        }) {
            damager.dealt_damage_this_turn = true;
        }
    }

    /// Damage landing on a permanent. A planeswalker loses loyalty instead of
    /// marking damage; everything else marks it and remembers what dealt it.
    /// Answers whether the damage was actually dealt to something.
    /// Whether this permanent is a creature, which is what decides between
    /// infect's counters and a planeswalker's ordinary loyalty loss.
    fn is_creature_permanent(&self, id: GameObjectId) -> bool {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .and_then(|permanent| self.permanent_types(permanent))
            .is_some_and(CardTypeSet::is_creature)
    }

    /// Infect damage to a creature is -1/-1 counters (CR 702.90b). Nothing
    /// is marked on the permanent, so it survives cleanup and shrinks the
    /// creature for good.
    fn deal_infect_damage_to_creature(
        &mut self,
        id: GameObjectId,
        amount: u16,
        source: Option<GameObjectId>,
    ) {
        self.note_damage_dealt_by(source, amount);
        if let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == id)
        {
            permanent.add_counters(CounterKind::MinusOneMinusOne, amount);
        }
    }

    fn deal_damage_to_permanent(
        &mut self,
        id: GameObjectId,
        amount: u16,
        source: Option<GameObjectId>,
        has_deathtouch: bool,
    ) -> bool {
        let Some(index) = self
            .battlefield
            .iter()
            .position(|permanent| permanent.card.id == id)
        else {
            return false;
        };
        self.note_damage_dealt_by(source, amount);
        if self
            .permanent_types(&self.battlefield[index])
            .is_some_and(|types| types.contains(CardType::Planeswalker))
        {
            let remaining = self.battlefield[index]
                .counters(CounterKind::Loyalty)
                .saturating_sub(amount);
            self.battlefield[index].set_counters(CounterKind::Loyalty, remaining);
            return true;
        }
        let permanent = &mut self.battlefield[index];
        permanent.damage = permanent.damage.saturating_add(amount);
        if amount > 0 {
            permanent.was_dealt_damage_this_turn = true;
            permanent.deathtouch_damage |= has_deathtouch;
            if let Some(source) = source
                && !permanent.damage_sources.contains(&source)
            {
                permanent.damage_sources.push(source);
            }
        }
        true
    }

    /// Caps a prospective damage event by every limiting rule that applies to
    /// its recipient. Limits compose by taking the smallest survivor, which
    /// is what two independent "instead" replacements do.
    fn apply_damage_limits(&self, event: ProspectiveDamage<'_>, amount: u16) -> u16 {
        let Some(Target::Player(player)) = event.target else {
            return amount;
        };
        let life = self.players[player.index()].life;
        let mut limited = amount;
        let _ = self.visit_player_damage_limits(player, |source, matcher, limit| {
            if self.static_damage_matcher_matches(matcher, source, source, event) {
                limited = limited.min(Self::damage_under_limit(limit, amount, life));
            }
            ControlFlow::Continue(())
        });
        limited
    }

    /// How much of `amount` a single limit lets through.
    const fn damage_under_limit(limit: DamageLimitDef, amount: u16, life: i16) -> u16 {
        match limit {
            DamageLimitDef::CapAt(cap) => {
                if amount > cap {
                    cap
                } else {
                    amount
                }
            }
            DamageLimitDef::LeaveAtLeastLife(floor) => {
                // How much can land before the floor is reached. A recipient
                // already at or below it takes nothing at all.
                let headroom = life.saturating_sub(floor);
                if headroom <= 0 {
                    0
                } else {
                    let headroom = headroom.cast_unsigned();
                    if headroom < amount { headroom } else { amount }
                }
            }
        }
    }

    pub(super) fn damage_target_from_kind(
        &mut self,
        source: Option<GameObjectId>,
        target: Option<Target>,
        amount: u16,
        combat: bool,
    ) -> u16 {
        // CR 614.9: redirection applies before the damage is dealt, so the
        // preventions below all answer the permanent it lands on
        // rather than the player it was aimed at.
        let target = self.redirected_damage_target(source, target);
        // Freeze both prospective participants once. Prevention follow-ups can
        // synchronously change life totals, and characteristic-defining
        // effects may depend on that state; every matcher and the eventual
        // committed event must nevertheless describe one damage event.
        let source_object = source.and_then(|source| self.damage_source_event_object(source));
        let source_is_spell = source.is_some_and(|source| self.damage_source_is_spell(source));
        let recipient_object = target.and_then(|target| match target {
            Target::Permanent(id) => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == id)
                .map(|permanent| self.trigger_event_object(permanent)),
            Target::Player(_) | Target::Card(_) | Target::Spell(_) => None,
        });
        let event = ProspectiveDamage {
            source,
            source_object: source_object.as_ref(),
            source_is_spell,
            target,
            recipient_object: recipient_object.as_ref(),
            combat,
        };
        // "Damage can't be prevented this turn" is not a prevention of its
        // own but a rule about every other one, so it is read here, ahead of
        // both the installed rules and the static ones. Damage limits are
        // not prevention (CR 615.1) and still apply.
        let preventable = !self.damage_cannot_be_prevented_this_turn;
        let amount = if preventable {
            self.apply_resolved_damage_prevention(event, amount)
        } else {
            amount
        };
        let amount = self.apply_damage_limits(event, amount);
        if amount == 0 {
            return 0;
        }
        // Protection prevents the damage it stops (CR 702.16e), so it is
        // part of what a "damage can't be prevented" turn switches off --
        // what protection does to targeting is untouched, and is decided
        // long before this.
        if preventable
            && (self.static_damage_is_prevented(event)
                || target.is_some_and(|target| match target {
                    Target::Permanent(id) => self
                        .battlefield
                        .iter()
                        .find(|permanent| permanent.card.id == id)
                        .is_some_and(|permanent| {
                            source.is_some_and(|source| {
                                self.is_protected_from_object(permanent, source)
                            })
                        }),
                    Target::Player(_) | Target::Card(_) | Target::Spell(_) => false,
                }))
        {
            return 0;
        }
        let source_has_keyword = |keyword: KeywordAbility| {
            source_object.as_ref().is_some_and(|source| {
                keyword
                    .simple_index()
                    .is_some_and(|index| source.keywords & (1 << index) != 0)
            })
        };
        let lifelink_controller = source_object.as_ref().and_then(|source| {
            source_has_keyword(KeywordAbility::Lifelink).then_some(source.controller)
        });
        let has_deathtouch = source_has_keyword(KeywordAbility::Deathtouch);
        // Infect changes what the damage does rather than how much of it
        // there is, so it is read here, after every prevention and limit has
        // settled the amount (CR 702.90a).
        let has_infect = source_has_keyword(KeywordAbility::Infect);
        let dealt_damage = match target {
            Some(Target::Player(player)) if has_infect => {
                self.add_poison_counters(player, amount);
                true
            }
            Some(Target::Player(player)) => {
                self.deal_damage_to_player(player, amount, source, combat);
                true
            }
            Some(Target::Permanent(id)) if has_infect && self.is_creature_permanent(id) => {
                self.deal_infect_damage_to_creature(id, amount, source);
                true
            }
            Some(Target::Permanent(id)) => {
                self.deal_damage_to_permanent(id, amount, source, has_deathtouch)
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
            && let Some(recipient) = target
        {
            let event = CommittedTriggerEvent::DamageDealt {
                source: source_object,
                source_is_spell,
                recipient,
                recipient_object,
                amount,
                combat,
            };
            self.capture_battlefield_triggers(&event);
        }
        if dealt_damage { amount } else { 0 }
    }

    pub(super) fn damage_source_event_object(
        &self,
        source: GameObjectId,
    ) -> Option<TriggerEventObject> {
        if let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
        {
            return Some(self.trigger_event_object(permanent));
        }
        if let Some(object) = self.stack.iter().find(|object| object.id == source) {
            return self.stack_trigger_event_object(object);
        }
        match self.retired_objects.get(&source) {
            Some(RetiredObject::Permanent { permanent, .. }) => {
                Some(self.trigger_event_object(permanent))
            }
            Some(RetiredObject::Stack(object)) => self.stack_trigger_event_object(object),
            Some(RetiredObject::Card(_)) | None => None,
        }
    }

    pub(super) fn damage_targets(&self) -> Vec<Target> {
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
}
