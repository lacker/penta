// Assigning flexible payment-source outputs.
//
// Included textually into `mana_planning.rs`, so the imports here are the
// parent module's. A permanent's outputs are alternatives: in particular,
// tapping a source for mana and tapping it for Convoke or Improvise cannot
// both be chosen from the same source.

#[derive(Clone, Copy)]
struct ManaPlanningRequest<'a> {
    player: PlayerId,
    cost: ManaCost,
    x: u16,
    options: ManaPlanOptions,
    purpose: &'a ManaPaymentPurpose,
    reserved: &'a [GameObjectId],
    life_available: u16,
}

#[allow(dead_code)]
struct ManaPlanSelection {
    available: Vec<PlannedManaActivation>,
    selected: Vec<PlannedManaActivation>,
    pool: PaymentCapacity,
    life_spent: u16,
    avoid: Option<GameObjectId>,
}

#[allow(dead_code)]
impl ManaPlanSelection {
    fn new(
        available: Vec<PlannedManaActivation>,
        mana: ManaPool,
        avoid: Option<GameObjectId>,
    ) -> Self {
        Self {
            available,
            selected: Vec::new(),
            pool: PaymentCapacity::from_mana(mana),
            life_spent: 0,
            avoid,
        }
    }

    fn select(&mut self, index: usize) {
        let activation = self.available.remove(index);
        self.pool.add_planned(&activation);
        self.life_spent = self.life_spent.saturating_add(activation.life_payment);
        self.selected.push(activation);
    }

    const fn life_mana_capacity(&self, life_mana_enabled: bool, life_available: u16) -> u16 {
        if life_mana_enabled {
            life_available
                .saturating_sub(self.life_spent)
        } else {
            0
        }
    }

    fn pay_colored(&mut self, cost: ManaCost) -> Option<()> {
        for color in colored_mana() {
            let required = mana_cost_amount(cost, color);
            while self.pool.amount(color) < required {
                let index = self
                    .available
                    .iter()
                    .enumerate()
                    // Read from what the activation makes rather than from
                    // the colour that labels it: an ability making two
                    // unlike mana pays for either.
                    .filter(|(_, activation)| activation.payment_amount(color) > 0)
                    .min_by_key(|(_, activation)| {
                        (
                            Some(activation.source) == self.avoid,
                            !activation.benefits_payment,
                            activation.flexibility,
                            activation.payment_total(),
                            activation.order,
                        )
                    })
                    .map(|(index, _)| index)?;
                self.select(index);
            }
        }
        Some(())
    }

    fn pay_hybrid(&mut self, cost: ManaCost) -> Option<()> {
        // Hybrid pairs must be assigned together. In particular, one white
        // capacity cannot independently satisfy both `{W/U}` and `{W/B}`.
        while !can_cover_hybrid_cost(self.pool.mana, cost) {
            let covered = maximum_hybrid_payment(
                mana_available_for_hybrid(self.pool.mana, cost),
                cost,
                &|_| false,
            )
            .total;
            let index = self
                .available
                .iter()
                .enumerate()
                .filter(|(_, activation)| {
                    let mut next = self.pool;
                    next.add_planned(activation);
                    maximum_hybrid_payment(
                        mana_available_for_hybrid(next.mana, cost),
                        cost,
                        &|_| false,
                    )
                    .total
                        > covered
                })
                .min_by_key(|(_, activation)| {
                    (
                        Some(activation.source) == self.avoid,
                        !activation.benefits_payment,
                        activation.flexibility,
                        activation.payment_total(),
                        activation.order,
                    )
                })
                .map(|(index, _)| index)?;
            self.select(index);
        }
        Some(())
    }

    fn pay_colorless(
        &mut self,
        cost: ManaCost,
        life_mana_enabled: bool,
        life_available: u16,
    ) -> Option<()> {
        // Direct generic contributions can never pay a true colorless symbol.
        // Repeatable life mana can, but shares the remaining life budget with
        // every selected PayLife mana ability.
        while self
            .pool
            .amount(ManaColor::Colorless)
            .saturating_add(self.life_mana_capacity(life_mana_enabled, life_available))
            < cost.colorless
        {
            let index = self
                .available
                .iter()
                .enumerate()
                .filter(|(_, payment)| payment.payment_amount(ManaColor::Colorless) > 0)
                .min_by_key(|(_, payment)| {
                    (
                        Some(payment.source) == self.avoid,
                        !payment.benefits_payment,
                        payment.flexibility,
                        payment.payment_total(),
                        payment.order,
                    )
                })
                .map(|(index, _)| index)?;
            self.select(index);
        }
        Some(())
    }

    fn pay_total(
        &mut self,
        required_total: u16,
        life_mana_enabled: bool,
        life_available: u16,
    ) -> Option<()> {
        while self
            .pool
            .total()
            .saturating_add(self.life_mana_capacity(life_mana_enabled, life_available))
            < required_total
        {
            let index = self
                .available
                .iter()
                .enumerate()
                .min_by_key(|(_, activation)| {
                    (
                        Some(activation.source) == self.avoid,
                        !activation.benefits_payment,
                        activation.production.amount(ManaColor::Colorless) == 0,
                        activation.payment_total(),
                        activation.order,
                    )
                })
                .map(|(index, _)| index)?;
            self.select(index);
        }
        Some(())
    }
}

impl Game {
    #[allow(dead_code)]
    pub(super) fn assigned_mana_activations_for(
        &self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        purpose: &ManaPaymentPurpose,
    ) -> Option<Vec<PlannedManaActivation>> {
        let life_available =
            u16::try_from(self.players[player.index()].life.max(0)).unwrap_or(u16::MAX);
        self.assigned_mana_activations(ManaPlanningRequest {
            player,
            cost,
            x,
            options: ManaPlanOptions::default(),
            purpose,
            reserved: &[],
            life_available,
        })
    }

    /// Plans against an explicit life budget. Spell-casting callers use the
    /// life left after the spell's own and its casting permission's life
    /// costs, so mana abilities cannot commit the same life a second time.
    pub(super) fn assigned_mana_activations_for_reserving_with_life(
        &self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        purpose: &ManaPaymentPurpose,
        reserved: &[GameObjectId],
        life_available: u16,
    ) -> Option<Vec<PlannedManaActivation>> {
        self.assigned_mana_activations(ManaPlanningRequest {
            player,
            cost,
            x,
            options: ManaPlanOptions::default(),
            purpose,
            reserved,
            life_available,
        })
    }

    fn assigned_mana_activations(
        &self,
        request: ManaPlanningRequest<'_>,
    ) -> Option<Vec<PlannedManaActivation>> {
        let (cost, x) = self.restrict_x(request.cost, request.x, request.purpose);
        let mana = self.eligible_mana_pool(request.player, request.purpose);
        let starting_pool = PaymentCapacity::from_mana(mana);
        let contributions = self.payment_contributions(request.purpose);
        let life_mana_enabled = self
            .repeatable_colorless_life_mana_activation(request.player)
            .is_some();
        // An ability that taps its source as a cost cannot also tap it for
        // mana, so that source is not a candidate at all.
        let barred = match request.purpose {
            ManaPaymentPurpose::Ability {
                source,
                taps_source: true,
                ..
            } => Some(*source),
            _ => None,
        };
        let sources = self.flexible_mana_sources(request, cost, contributions, barred);
        if !contributions.any()
            && sources
                .iter()
                .flat_map(|source| &source.outputs)
                .all(|output| output.kind.cost_object().is_none())
        {
            return assign_independent_mana_sources(
                &sources,
                starting_pool,
                cost,
                x,
                life_mana_enabled,
                request.options.avoid,
                request.life_available,
            );
        }
        let mut search = PaymentAssignmentSearch::new(
            &sources,
            cost,
            x,
            life_mana_enabled,
            request.options.avoid,
        );
        let found = if contributions.any() {
            search.assign_contributions(0, starting_pool, request.life_available)
        } else {
            search.assign_flexible(0, starting_pool, request.life_available)
        };
        found.then_some(search.best_assignment.unwrap_or_default())
    }

    fn flexible_mana_sources(
        &self,
        request: ManaPlanningRequest<'_>,
        cost: ManaCost,
        contributions: ManaContributionKinds,
        barred: Option<GameObjectId>,
    ) -> Vec<FlexibleManaSource> {
        let mut sources = Vec::new();
        for (order, permanent) in self
            .battlefield
            .iter()
            .filter(|permanent| permanent.controller == request.player)
            .filter(|permanent| Some(permanent.card.id) != barred)
            .enumerate()
        {
            let activations = self.eligible_payment_activations(permanent, request, cost);
            let mana_outputs = Self::planned_outputs(&activations, request.purpose);
            let mut outputs = mana_outputs.clone();
            if contributions.any() {
                let contribution_outputs =
                    self.permanent_contribution_outputs(permanent, contributions);
                let combined = Self::mana_and_contribution_outputs(
                    permanent,
                    &activations,
                    &mana_outputs,
                    &contribution_outputs,
                );
                // Prefer a harmless direct contribution; combined outputs come last
                // because they are only needed when this source contributes twice.
                outputs = contribution_outputs;
                outputs.extend(mana_outputs.iter().cloned());
                outputs.extend(combined);
            }
            if !outputs.is_empty() {
                sources.push(FlexibleManaSource {
                    source: permanent.card.id,
                    outputs,
                    order,
                });
            }
            self.append_repeatable_costed_mana_sources(
                &mut sources,
                permanent,
                &activations,
                &mana_outputs,
                order,
            );
        }
        let spell = match request.purpose {
            ManaPaymentPurpose::Spell { object, .. } => Some(*object),
            _ => None,
        };
        for activation in self
            .hand_mana_ability_activations(request.player)
            .into_iter()
            .filter(|activation| Some(activation.source) != spell)
            .filter(|activation| !Self::activation_consumes_reserved(activation, request.reserved))
        {
            let outputs = Self::planned_outputs(core::slice::from_ref(&activation), request.purpose);
            if let Some(existing) = sources
                .iter_mut()
                .find(|source| source.source == activation.source)
            {
                existing.outputs.extend(outputs);
            } else {
                sources.push(FlexibleManaSource {
                    source: activation.source,
                    outputs,
                    order: sources.len(),
                });
            }
        }
        if contributions.delve {
            let start = sources.len();
            for (offset, card) in self.players[request.player.index()]
                .graveyard
                .iter()
                .filter(|card| Some(card.id) != spell)
                .filter(|card| !request.reserved.contains(&card.id))
                .enumerate()
            {
                sources.push(FlexibleManaSource {
                    source: card.id,
                    outputs: vec![ManaSourceOutput {
                        kind: PlannedPaymentKind::Contribution(ManaContributionKind::Delve),
                        production: ManaPool::default(),
                        colored_contribution: ManaPool::default(),
                        generic_payment: 1,
                        life_payment: 0,
                        benefits_payment: false,
                    }],
                    order: start.saturating_add(offset),
                });
            }
        }
        sources
    }

    fn eligible_payment_activations(
        &self,
        permanent: &Permanent,
        request: ManaPlanningRequest<'_>,
        cost: ManaCost,
    ) -> Vec<ManaAbilityActivation> {
        let mut activations = self
            .mana_ability_activations(permanent)
            .into_iter()
            .filter(|activation| self.mana_activation_is_eligible(permanent, activation, request))
            .collect::<Vec<_>>();
        // When several outputs are legal, prefer one whose spend rider
        // benefits this payment. Players can still manually choose a
        // different mana ability before casting.
        activations.sort_by_key(|activation| {
            let benefits_payment = Self::mana_for_activation(activation)
                .first()
                .is_some_and(|mana| Self::mana_has_spend_effect_for(*mana, request.purpose));
            let production = Self::mana_production(activation);
            let pays_colored_symbol = colored_mana().into_iter().any(|color| {
                production.amount(color) > 0
                    && (mana_cost_amount(cost, color) > 0 || hybrid_pays_with(cost, color))
            });
            (!benefits_payment, !pays_colored_symbol)
        });
        activations
    }

    fn mana_activation_is_eligible(
        &self,
        permanent: &Permanent,
        activation: &ManaAbilityActivation,
        request: ManaPlanningRequest<'_>,
    ) -> bool {
        let preserves_tap_cost_payer = request.options.tap_cost_payer.is_none_or(|payer| {
            Self::mana_activation_preserves_tap_payment(permanent, activation, payer)
        });
        let preserves_required_source = !matches!(
            request.purpose,
            ManaPaymentPurpose::Ability {
                source,
                leaves_source: true,
                ..
            } if *source == activation.source
        ) || !activation.costs.iter().any(|cost| {
            matches!(
                cost,
                CostDef::SacrificeSource
                    | CostDef::ExileSource
                    | CostDef::ReturnSourceToHand
            )
        });
        // An activation that itself costs mana is left to the player. The
        // plan adds each source's production to a running pool, and one that
        // also spends from that pool would be counted as free.
        let costs_mana = activation
            .costs
            .iter()
            .any(|cost| matches!(cost, CostDef::Mana(_)));
        let consumes_reserved = Self::activation_consumes_reserved(activation, request.reserved);
        let consumes_spell = matches!(
            request.purpose,
            ManaPaymentPurpose::Spell { object, .. }
                if activation.cost_object == Some(*object)
        );
        Self::mana_for_activation(activation)
            .first()
            .is_some_and(|mana| self.mana_can_pay_for(*mana, request.purpose))
            // "Activate only as an instant": paying for a spell is not a
            // moment an instant could be cast, so the planner leaves it to
            // the player and their priority.
            && !activation.only_as_instant
            && preserves_tap_cost_payer
            && preserves_required_source
            && !costs_mana
            && !consumes_reserved
            && !consumes_spell
    }

    fn activation_consumes_reserved(
        activation: &ManaAbilityActivation,
        reserved: &[GameObjectId],
    ) -> bool {
        activation
            .cost_object
            .is_some_and(|object| reserved.contains(&object))
            || (reserved.contains(&activation.source)
                && activation.costs.iter().any(|cost| {
                    matches!(
                        cost,
                        CostDef::SacrificeSource
                            | CostDef::ExileSource
                            | CostDef::ReturnSourceToHand
                    )
                }))
            // A multi-object sacrifice has no concrete object IDs in the mana
            // activation. Conservatively leave it to manual activation when
            // the spell has reserved an object for another cost.
            || (!reserved.is_empty()
                && activation
                    .costs
                    .iter()
                    .any(|cost| matches!(cost, CostDef::SacrificePermanents { .. })))
    }
}

fn can_cover_payment(capacity: PaymentCapacity, cost: ManaCost, x: u16) -> bool {
    if capacity.generic == 0 {
        return can_pay(capacity.mana, cost, x);
    }
    let mut mana_only = cost;
    mana_only.generic = cost
        .generic
        .saturating_add(x.saturating_mul(cost.x_multiplier))
        .saturating_sub(capacity.generic);
    mana_only.variable_x = false;
    can_pay(capacity.mana, mana_only, 0)
}

fn with_life_mana_capacity(
    mut capacity: PaymentCapacity,
    life_mana_enabled: bool,
    life_available: u16,
) -> PaymentCapacity {
    if life_mana_enabled {
        capacity
            .mana
            .add_color(ManaColor::Colorless, life_available);
    }
    capacity
}

fn payment_specific_cap(cost: ManaCost, color: ManaColor) -> u16 {
    let flexible = FlexibleManaSymbol::ALL
        .into_iter()
        .filter(|symbol| symbol.mana_options().contains(&color))
        .map(|symbol| cost.flexible_count(symbol))
        .fold(0_u16, u16::saturating_add);
    mana_cost_amount(cost, color).saturating_add(flexible)
}

/// Collapses mana that cannot possibly pay a specific symbol into generic-only
/// capacity. This makes interchangeable five-color lands share a bounded set
/// of planner states instead of branching once per color on every land.
fn normalized_payment_capacity(
    capacity: PaymentCapacity,
    cost: ManaCost,
    x: u16,
) -> PaymentCapacity {
    let generic_required = cost
        .generic
        .saturating_add(x.saturating_mul(cost.x_multiplier));
    let mut normalized = PaymentCapacity {
        mana: ManaPool::default(),
        generic: capacity.generic.min(generic_required),
    };
    for color in ManaColor::ALL {
        let amount = capacity.amount(color);
        let specific = amount.min(payment_specific_cap(cost, color));
        normalized.mana.add_color(color, specific);
        normalized.generic = normalized
            .generic
            .saturating_add(amount.saturating_sub(specific))
            .min(generic_required);
    }
    normalized
}

fn planned_payment(
    source: &FlexibleManaSource,
    output: ManaSourceOutput,
) -> PlannedManaActivation {
    PlannedManaActivation {
        source: source.source,
        kind: output.kind,
        production: output.production,
        colored_contribution: output.colored_contribution,
        generic_payment: output.generic_payment,
        life_payment: output.life_payment,
        benefits_payment: output.benefits_payment,
        flexibility: source.outputs.len(),
        order: source.order,
    }
}

/// Exact dynamic assignment for independent sources. Equivalent payment
/// capacities retain only their best-ranked plan, so boards with many lands
/// offering the same colors remain polynomial in the size of the cost.
#[allow(clippy::too_many_arguments)]
fn assign_independent_mana_sources(
    sources: &[FlexibleManaSource],
    starting_pool: PaymentCapacity,
    cost: ManaCost,
    x: u16,
    life_mana_enabled: bool,
    avoid: Option<GameObjectId>,
    life_available: u16,
) -> Option<Vec<PlannedManaActivation>> {
    let starting_pool = normalized_payment_capacity(starting_pool, cost, x);
    // Ordered, not hashed. Two plans can reach the same payment capacity with
    // the same rank -- tapping either of two lands that make the same colour
    // is the ordinary case -- and both the retention test below and the
    // `min_by` at the end keep whichever they met first. Hash iteration order
    // is not stable even within one process, because `RandomState` takes a
    // fresh key per map, so the same seeded game replayed twice in a row
    // could tap a painland once and a basic the next time and end a point of
    // life apart. That is a determinism break the replay, checkpoint, and bot
    // versioning contracts all rest on.
    let mut states = BTreeMap::from([(
        (starting_pool.mana, starting_pool.generic, 0_u16),
        Vec::<PlannedManaActivation>::new(),
    )]);

    for source in sources {
        let mut next: BTreeMap<(ManaPool, u16, u16), Vec<PlannedManaActivation>> =
            BTreeMap::new();
        for ((mana, generic, life_spent), plan) in states {
            for output in &source.outputs {
                let next_life = life_spent.saturating_add(output.life_payment);
                if next_life > life_available {
                    continue;
                }
                let mut capacity = PaymentCapacity { mana, generic };
                capacity.add_output(output);
                capacity = normalized_payment_capacity(capacity, cost, x);
                let mut candidate = plan.clone();
                candidate.push(planned_payment(source, output.clone()));
                let key = (capacity.mana, capacity.generic, next_life);
                let replace = next.get(&key).is_none_or(|retained| {
                    payment_assignment_rank(&candidate, 0, avoid)
                        < payment_assignment_rank(retained, 0, avoid)
                });
                if replace {
                    next.insert(key, candidate);
                }
            }
            let key = (mana, generic, life_spent);
            let replace = next.get(&key).is_none_or(|retained| {
                payment_assignment_rank(&plan, 0, avoid)
                    < payment_assignment_rank(retained, 0, avoid)
            });
            if replace {
                next.insert(key, plan);
            }
        }
        states = next;
    }

    states
        .into_iter()
        .filter_map(|((mana, generic, life_spent), plan)| {
            let pool = PaymentCapacity { mana, generic };
            let life_mana = life_mana_needed_for_payment(
                pool,
                cost,
                x,
                life_mana_enabled,
                life_available.saturating_sub(life_spent),
            )?;
            Some((payment_assignment_rank(&plan, life_mana, avoid), plan))
        })
        .min_by(|(left, _), (right, _)| left.cmp(right))
        .map(|(_, plan)| plan)
}

type PaymentAssignmentRank = (
    u16,
    usize,
    usize,
    usize,
    usize,
    Vec<(usize, GameObjectId)>,
    usize,
    u16,
);

fn payment_assignment_rank(
    assignment: &[PlannedManaActivation],
    life_mana: u16,
    avoid: Option<GameObjectId>,
) -> PaymentAssignmentRank {
    let source_life = assignment
        .iter()
        .map(|payment| payment.life_payment)
        .fold(0_u16, u16::saturating_add);
    (
        source_life.saturating_add(life_mana),
        assignment
            .iter()
            .filter(|payment| Some(payment.source) == avoid)
            .count(),
        assignment
            .iter()
            .filter(|payment| payment.kind.cost_object().is_some())
            .count(),
        assignment.len(),
        assignment
            .iter()
            .filter(|payment| !payment.benefits_payment)
            .count(),
        assignment
            .iter()
            .map(|payment| (payment.order, payment.source))
            .collect(),
        assignment
            .iter()
            .map(|payment| payment.flexibility)
            .sum(),
        assignment
            .iter()
            .map(|payment| payment.production.total())
            .fold(0_u16, u16::saturating_add),
    )
}

fn life_mana_needed_for_payment(
    pool: PaymentCapacity,
    cost: ManaCost,
    x: u16,
    life_mana_enabled: bool,
    life_available: u16,
) -> Option<u16> {
    let maximum = if life_mana_enabled {
        life_available
    } else {
        0
    };
    (0..=maximum).find(|amount| {
        let mut capacity = pool;
        capacity.mana.add_color(ManaColor::Colorless, *amount);
        can_cover_payment(capacity, cost, x)
    })
}

struct PaymentAssignmentSearch<'a> {
    sources: &'a [FlexibleManaSource],
    cost: ManaCost,
    x: u16,
    life_mana_enabled: bool,
    avoid: Option<GameObjectId>,
    assignment: Vec<PlannedManaActivation>,
    best_assignment: Option<Vec<PlannedManaActivation>>,
    best_rank: Option<PaymentAssignmentRank>,
    consumed: Vec<GameObjectId>,
    contributors: Vec<GameObjectId>,
}

impl<'a> PaymentAssignmentSearch<'a> {
    fn new(
        sources: &'a [FlexibleManaSource],
        cost: ManaCost,
        x: u16,
        life_mana_enabled: bool,
        avoid: Option<GameObjectId>,
    ) -> Self {
        Self {
            sources,
            cost,
            x,
            life_mana_enabled,
            avoid,
            assignment: Vec::new(),
            best_assignment: None,
            best_rank: None,
            consumed: Vec::new(),
            contributors: Vec::new(),
        }
    }

    fn life_mana_needed(&self, pool: PaymentCapacity, life_available: u16) -> Option<u16> {
        life_mana_needed_for_payment(
            pool,
            self.cost,
            self.x,
            self.life_mana_enabled,
            life_available,
        )
    }

    fn consider_solution(&mut self, pool: PaymentCapacity, life_available: u16) -> bool {
        let Some(life_mana) = self.life_mana_needed(pool, life_available) else {
            return false;
        };
        let rank = payment_assignment_rank(&self.assignment, life_mana, self.avoid);
        if self.best_rank.as_ref().is_none_or(|best| rank < *best) {
            self.best_rank = Some(rank);
            self.best_assignment = Some(self.assignment.clone());
        }
        true
    }

    fn can_still_cover(&self, index: usize, payment_capacity: PaymentCapacity) -> bool {
        // A colour assignment cannot make up a total-mana shortfall. Prune
        // before branching over every output of every remaining source.
        let maximum_total = self.sources[index..]
            .iter()
            .filter_map(|source| {
                source
                    .outputs
                    .iter()
                    .map(ManaSourceOutput::payment_total)
                    .max()
            })
            .fold(payment_capacity.total(), u16::saturating_add);
        let required_total = colored_cost_total(self.cost)
            .saturating_add(self.cost.generic)
            .saturating_add(self.x.saturating_mul(self.cost.x_multiplier));
        maximum_total >= required_total
            && remaining_sources_can_cover_required_colors(
                self.sources,
                index,
                payment_capacity,
                self.cost,
            )
    }

    fn push_output(&mut self, source_index: usize, output: ManaSourceOutput) {
        let source = &self.sources[source_index];
        self.assignment.push(planned_payment(source, output));
    }

    fn assign_flexible(
        &mut self,
        index: usize,
        pool: PaymentCapacity,
        life_available: u16,
    ) -> bool {
        let mut found = self.consider_solution(pool, life_available);
        let payment_capacity =
            with_life_mana_capacity(pool, self.life_mana_enabled, life_available);
        if index == self.sources.len() || !self.can_still_cover(index, payment_capacity) {
            return found;
        }

        let output_count = self.sources[index].outputs.len();
        for output_index in 0..output_count {
            let output = self.sources[index].outputs[output_index].clone();
            let cost_object = output.kind.cost_object();
            if output.life_payment > life_available
                || cost_object.is_some_and(|object| self.consumed.contains(&object))
            {
                continue;
            }
            let mut next = pool;
            next.add_output(&output);
            let life_payment = output.life_payment;
            self.push_output(index, output);
            if let Some(object) = cost_object {
                self.consumed.push(object);
            }
            found |= self.assign_flexible(index + 1, next, life_available - life_payment);
            if cost_object.is_some() {
                self.consumed.pop();
            }
            self.assignment.pop();
        }
        found | self.assign_flexible(index + 1, pool, life_available)
    }
}

fn remaining_sources_can_cover_required_colors(
    sources: &[FlexibleManaSource],
    index: usize,
    pool: PaymentCapacity,
    cost: ManaCost,
) -> bool {
    let fixed_colors_fit = colored_mana()
        .into_iter()
        .chain(core::iter::once(ManaColor::Colorless))
        .all(|color| {
            let maximum = sources[index..]
                .iter()
                .filter_map(|source| {
                    source
                        .outputs
                        .iter()
                        .map(|output| output.payment_amount(color))
                        .max()
                })
                .fold(pool.amount(color), u16::saturating_add);
            maximum >= mana_cost_amount(cost, color)
        });
    if !fixed_colors_fit {
        return false;
    }

    // This deliberately overestimates flexible sources by counting the best
    // output of each source independently for every colour. It remains a safe
    // prune, while the global hybrid assignment still catches conflicts such
    // as one available white being claimed by both `{W/U}` and `{W/B}`.
    let mut optimistic = pool.mana;
    for color in colored_mana() {
        let additional = sources[index..]
            .iter()
            .filter_map(|source| {
                source
                    .outputs
                    .iter()
                    .map(|output| output.payment_amount(color))
                    .max()
            })
            .fold(0_u16, u16::saturating_add);
        optimistic.add_color(color, additional);
    }
    can_cover_hybrid_cost(optimistic, cost)
}

/// Finds one complete contribution-aware payment. Unlike ordinary mana
/// planning, this search may select no output, one output, or a combined
/// mana-plus-contribution output from each permanent. It also tracks costs so a
/// permanent sacrificed to a mana ability is never subsequently used, and
/// tracks the aggregate life paid by mana abilities in the payment.
impl PaymentAssignmentSearch<'_> {
    fn assign_contributions(
        &mut self,
        index: usize,
        pool: PaymentCapacity,
        life_available: u16,
    ) -> bool {
        let mut found = self.consider_solution(pool, life_available);
        let payment_capacity =
            with_life_mana_capacity(pool, self.life_mana_enabled, life_available);
        if index == self.sources.len() || !self.can_still_cover(index, payment_capacity) {
            return found;
        }

        let source_id = self.sources[index].source;
        let output_count = self.sources[index].outputs.len();
        for output_index in 0..output_count {
            let output = self.sources[index].outputs[output_index].clone();
            let uses_contribution = output.kind.uses_contribution();
            let cost_object = output.kind.cost_object();
            if output.life_payment > life_available
                || (uses_contribution && self.consumed.contains(&source_id))
                || cost_object.is_some_and(|object| {
                    self.consumed.contains(&object) || self.contributors.contains(&object)
                })
            {
                continue;
            }

            let mut next = pool;
            next.add_output(&output);
            let life_payment = output.life_payment;
            self.push_output(index, output);
            if uses_contribution {
                self.contributors.push(source_id);
            }
            if let Some(object) = cost_object {
                self.consumed.push(object);
            }
            found |= self.assign_contributions(index + 1, next, life_available - life_payment);
            if cost_object.is_some() {
                self.consumed.pop();
            }
            if uses_contribution {
                self.contributors.pop();
            }
            self.assignment.pop();
        }

        found | self.assign_contributions(index + 1, pool, life_available)
    }
}
