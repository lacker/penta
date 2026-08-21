// Assigning flexible payment-source outputs.
//
// Included textually into `mana_planning.rs`, so the imports here are the
// parent module's. A permanent's outputs are alternatives: in particular,
// tapping a mana creature for mana and tapping it for convoke cannot both be
// chosen from the same source.

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

struct ManaPlanSelection {
    available: Vec<PlannedManaActivation>,
    selected: Vec<PlannedManaActivation>,
    pool: PaymentCapacity,
    life_spent: u16,
    avoid: Option<GameObjectId>,
}

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

    const fn channel_capacity(&self, channel_enabled: bool, life_available: u16) -> u16 {
        if channel_enabled {
            life_available
                .saturating_sub(self.life_spent)
                .saturating_sub(1)
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
        channel_enabled: bool,
        life_available: u16,
    ) -> Option<()> {
        // Convoke can never pay a true colorless symbol. Channel can, but its
        // synthesized mana shares the remaining life budget with every
        // selected PayLife mana ability.
        while self
            .pool
            .amount(ManaColor::Colorless)
            .saturating_add(self.channel_capacity(channel_enabled, life_available))
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
        channel_enabled: bool,
        life_available: u16,
    ) -> Option<()> {
        while self
            .pool
            .total()
            .saturating_add(self.channel_capacity(channel_enabled, life_available))
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
        let uses_convoke = self.payment_uses_convoke(request.purpose);
        let channel_enabled = self.channel_active[request.player.index()];
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
        let sources = self.flexible_mana_sources(request, cost, uses_convoke, barred);
        let mut search = PaymentAssignmentSearch::new(&sources, cost, x, channel_enabled);
        let found = if uses_convoke {
            search.assign_convoke(0, starting_pool, request.life_available)
        } else {
            search.assign_flexible(0, starting_pool, request.life_available)
        };
        found.then_some(search.assignment)
    }

    fn flexible_mana_sources(
        &self,
        request: ManaPlanningRequest<'_>,
        cost: ManaCost,
        uses_convoke: bool,
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
            if uses_convoke {
                let convoke_outputs = self.convoke_outputs(permanent);
                let combined =
                    Self::mana_and_convoke_outputs(&activations, &mana_outputs, &convoke_outputs);
                // Prefer a harmless Convoke tap; combined outputs come last
                // because they are only needed when this source contributes twice.
                outputs = convoke_outputs;
                outputs.extend(mana_outputs.iter().copied());
                outputs.extend(combined);
            }
            if !outputs.is_empty() {
                sources.push(FlexibleManaSource {
                    source: permanent.card.id,
                    outputs,
                    order,
                });
            }
            if uses_convoke {
                self.append_repeatable_convoke_mana_sources(
                    &mut sources,
                    permanent,
                    &activations,
                    &mana_outputs,
                    order,
                );
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
            Self::mana_activation_preserves_tap_cost_payer(permanent, activation, payer)
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
                AbilityCostDef::SacrificeSource
                    | AbilityCostDef::ExileSource
                    | AbilityCostDef::ReturnSourceToHand
            )
        });
        // An activation that itself costs mana is left to the player. The
        // plan adds each source's production to a running pool, and one that
        // also spends from that pool would be counted as free.
        let costs_mana = activation
            .costs
            .iter()
            .any(|cost| matches!(cost, AbilityCostDef::Mana(_)));
        let consumes_reserved = Self::activation_consumes_reserved(activation, request.reserved);
        Self::mana_for_activation(activation)
            .first()
            .is_some_and(|mana| self.mana_can_pay_for(*mana, request.purpose))
            && preserves_tap_cost_payer
            && preserves_required_source
            && !costs_mana
            && !consumes_reserved
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
                        AbilityCostDef::SacrificeSource
                            | AbilityCostDef::ExileSource
                            | AbilityCostDef::ReturnSourceToHand
                    )
                }))
            // A multi-object sacrifice has no concrete object IDs in the mana
            // activation. Conservatively leave it to manual activation when
            // the spell has reserved an object for another cost.
            || (!reserved.is_empty()
                && activation
                    .costs
                    .iter()
                    .any(|cost| matches!(cost, AbilityCostDef::SacrificePermanents { .. })))
    }
}

fn can_cover_payment(capacity: PaymentCapacity, cost: ManaCost, x: u16) -> bool {
    if capacity.generic == 0 {
        return can_pay(capacity.mana, cost, x);
    }
    let pool = capacity.mana;
    pool.white >= cost.white
        && pool.blue >= cost.blue
        && pool.black >= cost.black
        && pool.red >= cost.red
        && pool.green >= cost.green
        && pool.colorless >= cost.colorless
        && can_cover_hybrid_cost(pool, cost)
        && capacity.total()
            >= colored_cost_total(cost)
                .saturating_add(cost.generic)
                .saturating_add(x.saturating_mul(cost.x_multiplier))
}

fn with_channel_capacity(
    mut capacity: PaymentCapacity,
    channel_enabled: bool,
    life_available: u16,
) -> PaymentCapacity {
    if channel_enabled {
        capacity
            .mana
            .add_color(ManaColor::Colorless, life_available.saturating_sub(1));
    }
    capacity
}

struct PaymentAssignmentSearch<'a> {
    sources: &'a [FlexibleManaSource],
    cost: ManaCost,
    x: u16,
    channel_enabled: bool,
    assignment: Vec<PlannedManaActivation>,
    consumed: Vec<GameObjectId>,
    convokers: Vec<GameObjectId>,
}

impl<'a> PaymentAssignmentSearch<'a> {
    fn new(
        sources: &'a [FlexibleManaSource],
        cost: ManaCost,
        x: u16,
        channel_enabled: bool,
    ) -> Self {
        Self {
            sources,
            cost,
            x,
            channel_enabled,
            assignment: Vec::new(),
            consumed: Vec::new(),
            convokers: Vec::new(),
        }
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
                    .map(|output| output.payment_total())
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
        self.assignment.push(PlannedManaActivation {
            source: source.source,
            kind: output.kind,
            production: output.production,
            convoke_production: output.convoke_production,
            generic_payment: output.generic_payment,
            life_payment: output.life_payment,
            benefits_payment: output.benefits_payment,
            flexibility: source.outputs.len(),
            order: source.order,
        });
    }

    fn assign_flexible(
        &mut self,
        index: usize,
        pool: PaymentCapacity,
        life_available: u16,
    ) -> bool {
        let payment_capacity =
            with_channel_capacity(pool, self.channel_enabled, life_available);
        if can_cover_payment(payment_capacity, self.cost, self.x) {
            return true;
        }
        if !self.can_still_cover(index, payment_capacity) || index == self.sources.len() {
            return false;
        }

        let output_count = self.sources[index].outputs.len();
        for output_index in 0..output_count {
            let output = self.sources[index].outputs[output_index];
            let cost_object = output.kind.cost_object();
            if output.life_payment > life_available
                || cost_object.is_some_and(|object| self.consumed.contains(&object))
            {
                continue;
            }
            let mut next = pool;
            next.add_output(&output);
            self.push_output(index, output);
            if let Some(object) = cost_object {
                self.consumed.push(object);
            }
            if self.assign_flexible(index + 1, next, life_available - output.life_payment) {
                return true;
            }
            if cost_object.is_some() {
                self.consumed.pop();
            }
            self.assignment.pop();
        }
        self.assign_flexible(index + 1, pool, life_available)
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

/// Finds one complete Convoke-aware payment. Unlike ordinary mana planning,
/// this search may select no output, one output, or a combined mana-plus-
/// Convoke output from each permanent. It also tracks cross-source costs so a
/// permanent sacrificed to a mana ability is never subsequently used, and
/// tracks the aggregate life paid by mana abilities in the payment.
impl PaymentAssignmentSearch<'_> {
    fn assign_convoke(
        &mut self,
        index: usize,
        pool: PaymentCapacity,
        life_available: u16,
    ) -> bool {
        let payment_capacity =
            with_channel_capacity(pool, self.channel_enabled, life_available);
        if can_cover_payment(payment_capacity, self.cost, self.x) {
            return true;
        }
        if !self.can_still_cover(index, payment_capacity) || index == self.sources.len() {
            return false;
        }

        let source_id = self.sources[index].source;
        let output_count = self.sources[index].outputs.len();
        for output_index in 0..output_count {
            let output = self.sources[index].outputs[output_index];
            let uses_convoke = output.kind.uses_convoke();
            let cost_object = output.kind.cost_object();
            if output.life_payment > life_available
                || (uses_convoke && self.consumed.contains(&source_id))
                || cost_object.is_some_and(|object| {
                    self.consumed.contains(&object) || self.convokers.contains(&object)
                })
            {
                continue;
            }

            let mut next = pool;
            next.add_output(&output);
            self.push_output(index, output);
            if uses_convoke {
                self.convokers.push(source_id);
            }
            if let Some(object) = cost_object {
                self.consumed.push(object);
            }
            if self.assign_convoke(index + 1, next, life_available - output.life_payment) {
                return true;
            }
            if cost_object.is_some() {
                self.consumed.pop();
            }
            if uses_convoke {
                self.convokers.pop();
            }
            self.assignment.pop();
        }

        self.assign_convoke(index + 1, pool, life_available)
    }
}

/// Stabilizes the selected payment while respecting dependencies between
/// mana abilities. If one activation sacrifices another selected source, the
/// sacrificed source has to activate first. A cycle has no legal execution
/// order and therefore no plan.
fn order_mana_activations_before_consumption(
    mut activations: Vec<PlannedManaActivation>,
) -> Option<Vec<PlannedManaActivation>> {
    if activations.iter().any(|activation| {
        activation.kind.cost_object().is_some_and(|object| {
            activations
                .iter()
                .any(|candidate| candidate.source == object && candidate.kind.uses_convoke())
        })
    }) {
        return None;
    }

    let mut ordered = Vec::with_capacity(activations.len());
    while !activations.is_empty() {
        let next = (0..activations.len()).find(|activation_index| {
            let activation = &activations[*activation_index];
            activation.kind.cost_object().is_none_or(|object| {
                !activations
                    .iter()
                    .enumerate()
                    .any(|(candidate_index, candidate)| {
                        candidate_index != *activation_index && candidate.source == object
                    })
            })
        })?;
        ordered.push(activations.remove(next));
    }
    Some(ordered)
}

fn unique_payment_source_ids(plan: Vec<PlannedManaActivation>) -> Vec<GameObjectId> {
    let mut sources = Vec::new();
    for payment in plan {
        if !sources.contains(&payment.source) {
            sources.push(payment.source);
        }
    }
    sources
}
