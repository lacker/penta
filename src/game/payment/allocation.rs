//! Allocation capacity retains restrictions on individual cost symbols. Aggregate colors
//! alone cannot decide whether a unit that cannot pay generic mana may be spent.
use crate::card::{FlexibleManaSymbol, ManaColor, ManaCost};
use crate::game::{Mana, ManaPool};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(in crate::game) struct PaymentPool {
    pub(in crate::game) mana: ManaPool,
    pub(in crate::game) non_generic: ManaPool,
    pub(in crate::game) any_color: bool,
    pub(in crate::game) any_type: bool,
    pub(in crate::game) direct: ManaPool,
}
impl From<ManaPool> for PaymentPool {
    fn from(mana: ManaPool) -> Self {
        Self {
            mana,
            ..Self::default()
        }
    }
}
impl PaymentPool {
    pub(in crate::game) const fn total(self) -> u16 {
        self.mana.total().saturating_add(self.direct.total())
    }
    pub(in crate::game) const fn amount(self, color: ManaColor) -> u16 {
        self.mana
            .amount(color)
            .saturating_add(self.direct.amount(color))
    }
    pub(in crate::game) fn add(&mut self, other: impl Into<Self>) {
        let other = other.into();
        self.mana.add(other.mana);
        self.non_generic.add(other.non_generic);
        self.direct.add(other.direct);
        self.any_color |= other.any_color;
        self.any_type |= other.any_type;
    }
    pub(in crate::game) fn add_color(&mut self, color: ManaColor, amount: u16) {
        self.mana.add_color(color, amount);
    }
    pub(in crate::game) fn add_unit(&mut self, mana: Mana, non_generic: bool) {
        self.add_color(mana.color, 1);
        if non_generic {
            self.non_generic.add_color(mana.color, 1);
        }
    }
    pub(in crate::game) fn needs_symbol_allocation(self) -> bool {
        self.any_color || self.any_type || self.non_generic.total() > 0 || self.direct.total() > 0
    }
}

/// A circulation with lower bounds enforces both the cost and every explicitly selected
/// unit. Nodes are color/restriction classes, not individual units, so large X stays small.
#[derive(Clone)]
struct Flow {
    residual: Vec<Vec<i32>>,
}
impl Flow {
    fn new(nodes: usize) -> Self {
        Self {
            residual: vec![vec![0; nodes]; nodes],
        }
    }
    fn edge(&mut self, from: usize, to: usize, capacity: i32) {
        self.residual[from][to] += capacity;
    }
    fn bounded(&mut self, balance: &mut [i32], from: usize, to: usize, low: i32, high: i32) {
        self.edge(from, to, high - low);
        balance[from] -= low;
        balance[to] += low;
    }
    fn maximize(&mut self, source: usize, sink: usize) -> i32 {
        let mut total = 0;
        loop {
            let mut parent = vec![usize::MAX; self.residual.len()];
            parent[source] = source;
            let mut queue = std::collections::VecDeque::from([source]);
            while let Some(node) = queue.pop_front() {
                for (next, previous) in parent.iter_mut().enumerate() {
                    if *previous == usize::MAX && self.residual[node][next] > 0 {
                        *previous = node;
                        queue.push_back(next);
                    }
                }
            }
            if parent[sink] == usize::MAX {
                return total;
            }
            let mut amount = i32::MAX;
            let mut node = sink;
            while node != source {
                amount = amount.min(self.residual[parent[node]][node]);
                node = parent[node];
            }
            node = sink;
            while node != source {
                let previous = parent[node];
                self.residual[previous][node] -= amount;
                self.residual[node][previous] += amount;
                node = previous;
            }
            total += amount;
        }
    }
}
#[derive(Clone)]
struct Demand {
    colors: Vec<ManaColor>,
    amount: u16,
    generic: bool,
    any_color: bool,
    unit_cost: ManaCost,
}

pub(in crate::game) fn symbol_payment_remainder(
    pool: PaymentPool,
    required: PaymentPool,
    cost: ManaCost,
    x: u16,
) -> Option<PaymentPool> {
    symbol_payment(pool, required, cost, x).map(|(remaining, _)| remaining)
}

pub(in crate::game) fn contribution_remainder(
    pool: PaymentPool,
    cost: ManaCost,
    x: u16,
) -> Option<ManaCost> {
    symbol_payment(
        pool,
        PaymentPool {
            direct: pool.direct,
            ..PaymentPool::default()
        },
        cost,
        x,
    )
    .map(|(_, remaining)| remaining)
}

fn symbol_payment(
    pool: PaymentPool,
    required: PaymentPool,
    cost: ManaCost,
    x: u16,
) -> Option<(PaymentPool, ManaCost)> {
    let mut demands = Vec::new();
    for (index, color) in ManaColor::ALL.into_iter().enumerate() {
        let total = crate::game::mana_planning::mana_cost_amount(cost, color);
        let restricted = cost.restricted_generic[index];
        let amount = total.checked_sub(restricted)?;
        if restricted > 0 {
            demands.push(Demand {
                colors: vec![color],
                amount: restricted,
                generic: true,
                any_color: false,
                unit_cost: {
                    let mut unit = ManaCost::of_color(color, 1);
                    unit.restricted_generic[index] = 1;
                    unit
                },
            });
        }
        if amount > 0 {
            demands.push(Demand {
                amount,
                generic: false,
                colors: vec![color],
                any_color: color != ManaColor::Colorless,
                unit_cost: ManaCost::of_color(color, 1),
            });
        }
    }
    let generic = cost
        .generic
        .saturating_add(x.saturating_mul(cost.x_multiplier));
    flexible_allocations(pool, required, cost, 0, demands, generic)
}
fn flexible_allocations(
    pool: PaymentPool,
    required: PaymentPool,
    cost: ManaCost,
    index: usize,
    mut demands: Vec<Demand>,
    generic: u16,
) -> Option<(PaymentPool, ManaCost)> {
    let Some(symbol) = FlexibleManaSymbol::ALL.get(index).copied() else {
        if generic > 0 {
            demands.push(Demand {
                colors: ManaColor::ALL.to_vec(),
                amount: generic,
                generic: true,
                any_color: false,
                unit_cost: ManaCost::new(1, 0),
            });
        }
        return allocate(pool, required, &demands);
    };
    let count = cost.flexible_count(symbol);
    if count == 0 {
        return flexible_allocations(pool, required, cost, index + 1, demands, generic);
    }
    let colors = symbol.mana_options().to_vec();
    let maximum_generic = if symbol.generic_alternative().is_some() {
        count
    } else {
        0
    };
    for generic_symbols in 0..=maximum_generic {
        let mut branch = demands.clone();
        if count > generic_symbols {
            branch.push(Demand {
                colors: colors.clone(),
                amount: count - generic_symbols,
                generic: false,
                any_color: true,
                unit_cost: ManaCost::default().with_flexible_symbol(symbol, 1),
            });
        }
        if let Some(remaining) = flexible_allocations(
            pool,
            required,
            cost,
            index + 1,
            branch,
            generic.saturating_add(
                generic_symbols.saturating_mul(symbol.generic_alternative().unwrap_or(0)),
            ),
        ) {
            return Some(remaining);
        }
    }
    None
}
fn allocate(
    pool: PaymentPool,
    required: PaymentPool,
    demands: &[Demand],
) -> Option<(PaymentPool, ManaCost)> {
    let source = 0;
    let units = 1;
    let slots = 19;
    let sink = slots + demands.len();
    let super_source = sink + 1;
    let super_sink = sink + 2;
    let mut flow = Flow::new(super_sink + 1);
    let mut balance = vec![0; super_sink + 1];
    let mut available = [0; 18];
    let mut mandatory = [0; 18];
    for (index, color) in ManaColor::ALL.into_iter().enumerate() {
        let restricted = pool.non_generic.amount(color);
        let selected = required.non_generic.amount(color);
        available[index] = i32::from(pool.mana.amount(color).checked_sub(restricted)?);
        available[index + 6] = i32::from(restricted);
        available[index + 12] = i32::from(pool.direct.amount(color));
        mandatory[index] = i32::from(required.mana.amount(color).checked_sub(selected)?);
        mandatory[index + 6] = i32::from(selected);
        mandatory[index + 12] = i32::from(required.direct.amount(color));
    }
    for index in 0..18 {
        if mandatory[index] > available[index] {
            return None;
        }
        flow.bounded(
            &mut balance,
            source,
            units + index,
            mandatory[index],
            available[index],
        );
        let color = ManaColor::ALL[index % 6];
        for (slot, demand) in demands.iter().enumerate() {
            let direct = index >= 12;
            let color_fits = if direct {
                demand.generic || (color != ManaColor::Colorless && demand.colors.contains(&color))
            } else {
                demand.colors.contains(&color)
                    || (pool.any_color && demand.any_color)
                    || (pool.any_type && !demand.generic)
            };
            if color_fits && (index < 6 || direct || !demand.generic) {
                flow.edge(units + index, slots + slot, i32::from(u16::MAX));
            }
        }
    }
    let mut total = 0;
    for (slot, demand) in demands.iter().enumerate() {
        let amount = i32::from(demand.amount);
        total += amount;
        flow.bounded(&mut balance, slots + slot, sink, amount, amount);
    }
    flow.bounded(&mut balance, sink, source, total, total);
    let mut needed = 0;
    for (node, amount) in balance.into_iter().enumerate() {
        if amount > 0 {
            flow.edge(super_source, node, amount);
            needed += amount;
        } else if amount < 0 {
            flow.edge(node, super_sink, -amount);
        }
    }
    if flow.maximize(super_source, super_sink) != needed {
        return None;
    }
    let mut remaining = PaymentPool {
        any_color: pool.any_color,
        any_type: pool.any_type,
        ..PaymentPool::default()
    };
    for (index, color) in ManaColor::ALL.into_iter().enumerate() {
        let normal = u16::try_from(flow.residual[source][units + index]).ok()?;
        let restricted = u16::try_from(flow.residual[source][units + index + 6]).ok()?;
        remaining.add_color(color, normal.saturating_add(restricted));
        remaining.non_generic.add_color(color, restricted);
        remaining.direct.add_color(
            color,
            u16::try_from(flow.residual[source][units + index + 12]).ok()?,
        );
    }
    let mut residual_cost = ManaCost::default();
    for (slot, demand) in demands.iter().enumerate() {
        let direct_paid: i32 = (12..18)
            .map(|index| flow.residual[slots + slot][units + index])
            .sum();
        let remaining = demand
            .amount
            .checked_sub(u16::try_from(direct_paid).ok()?)?;
        for _ in 0..remaining {
            residual_cost = residual_cost.plus(demand.unit_cost);
        }
    }
    Some((remaining, residual_cost))
}
