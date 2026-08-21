// Spending a pool against one cost.
//
// Separate from the planning above because it answers a different question:
// planning decides which sources to tap and whether a cast is affordable at
// all, while this decides which mana in the pool actually leaves it.
// Included textually into `mana_planning.rs`, so the imports here are the
// parent module's.

#[cfg(test)]
pub(super) fn pay_cost(pool: &mut ManaPool, cost: ManaCost, x: u16) {
    pay_cost_with_generic_strategy(
        pool,
        cost,
        x,
        // No rider to prefer, so each pair spends in its printed order.
        &|_| false,
        &[
            ManaColor::Colorless,
            ManaColor::Green,
            ManaColor::Black,
            ManaColor::Red,
            ManaColor::White,
            ManaColor::Blue,
        ],
        false,
    );
}

/// Spends a pool against one cost. `spread_generic_colors` pays the generic
/// portion across as many colours as it can instead of draining them in
/// order, which is what converge wants and nothing else does.
pub(super) fn pay_cost_with_generic_strategy(
    pool: &mut ManaPool,
    cost: ManaCost,
    x: u16,
    hybrid_preference: &impl Fn(ManaColor) -> bool,
    generic_order: &[ManaColor],
    spread_generic_colors: bool,
) {
    for color in colored_mana() {
        pool.remove_color(color, mana_cost_amount(cost, color));
    }
    pool.remove_color(ManaColor::Colorless, cost.colorless);
    if cost.hybrid_total() > 0 {
        let hybrid = maximum_hybrid_payment(*pool, cost, hybrid_preference);
        debug_assert_eq!(hybrid.total, hybrid_required_total(cost));
        for (pair, allocation) in HybridPair::ALL.into_iter().zip(hybrid.allocations) {
            let (first, second) = pair.colors();
            pool.remove_color(first, allocation[0]);
            pool.remove_color(second, allocation[1]);
        }
    }
    let generic = cost
        .generic
        .saturating_add(x.saturating_mul(cost.x_multiplier));
    if spread_generic_colors {
        pay_generic_spreading_colors(pool, generic, generic_order);
    } else {
        pay_generic_in_order(pool, generic, generic_order);
    }
}

pub(super) fn add_generic(mut cost: ManaCost, additional: u16) -> ManaCost {
    cost.generic = cost.generic.saturating_add(additional);
    cost
}

/// A cost reduction only ever removes generic mana, and never takes a cost
/// below its colored requirements (CR 601.2f).
pub(super) fn reduce_generic(mut cost: ManaCost, reduction: u16) -> ManaCost {
    cost.generic = cost.generic.saturating_sub(reduction);
    cost
}

pub(super) fn add_mana_cost(mut cost: ManaCost, additional: ManaCost) -> ManaCost {
    cost.generic = cost.generic.saturating_add(additional.generic);
    cost.white = cost.white.saturating_add(additional.white);
    cost.blue = cost.blue.saturating_add(additional.blue);
    cost.black = cost.black.saturating_add(additional.black);
    cost.red = cost.red.saturating_add(additional.red);
    cost.green = cost.green.saturating_add(additional.green);
    cost.colorless = cost.colorless.saturating_add(additional.colorless);
    for index in 0..HybridPair::COUNT {
        cost.hybrid[index] = cost.hybrid[index].saturating_add(additional.hybrid[index]);
    }
    cost.variable_x |= additional.variable_x;
    cost.x_multiplier = cost.x_multiplier.saturating_add(additional.x_multiplier);
    cost
}

/// Pays a generic requirement one mana at a time, cycling through the order
/// rather than draining each colour before touching the next. This is the
/// payment converge wants: with white, white, and blue in the pool, "{1}{W}"
/// spends one of each colour rather than both whites, which is what any
/// caster of a converge spell means by it.
pub(super) fn pay_generic_spreading_colors(pool: &mut ManaPool, amount: u16, order: &[ManaColor]) {
    let mut remaining = amount;
    while remaining > 0 {
        let mut spent_this_pass = false;
        for color in order {
            if remaining == 0 {
                break;
            }
            if pool.amount(*color) > 0 {
                pool.remove_color(*color, 1);
                remaining -= 1;
                spent_this_pass = true;
            }
        }
        if !spent_this_pass {
            break;
        }
    }
    debug_assert_eq!(remaining, 0);
}

pub(super) fn pay_generic_in_order(pool: &mut ManaPool, amount: u16, order: &[ManaColor]) {
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

pub(super) fn colored_mana() -> Vec<ManaColor> {
    vec![
        ManaColor::White,
        ManaColor::Blue,
        ManaColor::Black,
        ManaColor::Red,
        ManaColor::Green,
    ]
}

/// "Spend only black mana on X." The restriction does not change how much the
/// spell costs, only which mana may pay for it, so folding the X portion out
/// of the generic requirement and into the coloured one says exactly that in
/// the vocabulary every payment path already speaks.
pub(super) fn fold_restricted_x(cost: ManaCost, x: u16, color: ManaColor) -> (ManaCost, u16) {
    let amount = x.saturating_mul(cost.x_multiplier);
    let mut folded = cost;
    match color {
        ManaColor::White => folded.white = folded.white.saturating_add(amount),
        ManaColor::Blue => folded.blue = folded.blue.saturating_add(amount),
        ManaColor::Black => folded.black = folded.black.saturating_add(amount),
        ManaColor::Red => folded.red = folded.red.saturating_add(amount),
        ManaColor::Green => folded.green = folded.green.saturating_add(amount),
        // No printed card restricts X to colourless, and generic already
        // accepts it, so there is nothing to fold.
        ManaColor::Colorless => return (cost, x),
    }
    (folded, 0)
}

pub(super) const fn mana_cost_amount(cost: ManaCost, color: ManaColor) -> u16 {
    match color {
        ManaColor::White => cost.white,
        ManaColor::Blue => cost.blue,
        ManaColor::Black => cost.black,
        ManaColor::Red => cost.red,
        ManaColor::Green => cost.green,
        // `{C}` is a requirement like any coloured symbol: only colorless
        // mana pays it. Generic is the part any mana pays, and is separate.
        ManaColor::Colorless => cost.colorless,
    }
}

/// Every symbol that has to be paid with particular mana rather than with
/// whatever is around: the five colours, `{C}`, and the hybrids.
pub(super) const fn colored_cost_total(cost: ManaCost) -> u16 {
    cost.white
        + cost.blue
        + cost.black
        + cost.red
        + cost.green
        + cost.colorless
        + cost.hybrid_total()
}

pub(super) const fn mana_cost_value(cost: ManaCost) -> u16 {
    cost.generic.saturating_add(colored_cost_total(cost))
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) struct HybridPayment {
    /// How many symbols of each pair are assigned to its first and second
    /// printed colours.
    pub(super) allocations: [[u16; 2]; HybridPair::COUNT],
    pub(super) total: u32,
}

pub(super) fn hybrid_required_total(cost: ManaCost) -> u32 {
    cost.hybrid.into_iter().map(u32::from).sum()
}

fn hybrid_color_node(color: ManaColor) -> usize {
    const COLOR_START: usize = 1 + HybridPair::COUNT;
    COLOR_START
        + match color {
            ManaColor::White => 0,
            ManaColor::Blue => 1,
            ManaColor::Black => 2,
            ManaColor::Red => 3,
            ManaColor::Green => 4,
            ManaColor::Colorless => unreachable!("hybrid symbols have colored halves"),
        }
}

/// Finds a maximum, globally consistent assignment of hybrid symbols to the
/// available colours. Checking each pair independently is insufficient: one
/// white mana cannot simultaneously pay a `{W/U}` and a `{W/B}` symbol.
///
/// This is a tiny max-flow network (ten pair nodes and five colour nodes).
/// Pair-to-colour edges are visited in the caller's preferred order so mana
/// with a useful spend rider remains the default when several maximum flows
/// exist; residual edges still let a later pair reroute an earlier choice.
pub(super) fn maximum_hybrid_payment(
    available: ManaPool,
    cost: ManaCost,
    hybrid_preference: &impl Fn(ManaColor) -> bool,
) -> HybridPayment {
    const SOURCE: usize = 0;
    const PAIR_START: usize = 1;
    const COLOR_START: usize = PAIR_START + HybridPair::COUNT;
    const SINK: usize = COLOR_START + 5;
    const NODE_COUNT: usize = SINK + 1;

    if cost.hybrid_total() == 0 {
        return HybridPayment::default();
    }

    let mut residual = [[0_u32; NODE_COUNT]; NODE_COUNT];
    for pair in HybridPair::ALL {
        let pair_node = PAIR_START + pair.index();
        let required = u32::from(cost.hybrid[pair.index()]);
        residual[SOURCE][pair_node] = required;
        let (first, second) = pair.colors();
        residual[pair_node][hybrid_color_node(first)] = required;
        residual[pair_node][hybrid_color_node(second)] = required;
    }
    for color in colored_mana() {
        residual[hybrid_color_node(color)][SINK] = u32::from(available.amount(color));
    }

    let mut total = 0_u32;
    loop {
        let mut parent = [usize::MAX; NODE_COUNT];
        parent[SOURCE] = SOURCE;
        let mut queue = std::collections::VecDeque::from([SOURCE]);
        while let Some(node) = queue.pop_front() {
            let mut neighbors = core::array::from_fn::<_, NODE_COUNT, _>(|index| index);
            if (PAIR_START..COLOR_START).contains(&node) {
                neighbors.sort_by_key(|candidate| {
                    let color = match *candidate {
                        COLOR_START => Some(ManaColor::White),
                        value if value == COLOR_START + 1 => Some(ManaColor::Blue),
                        value if value == COLOR_START + 2 => Some(ManaColor::Black),
                        value if value == COLOR_START + 3 => Some(ManaColor::Red),
                        value if value == COLOR_START + 4 => Some(ManaColor::Green),
                        _ => None,
                    };
                    color.map_or((true, false), |color| {
                        (false, hybrid_preference(color))
                    })
                });
            }
            for next in neighbors {
                if parent[next] == usize::MAX && residual[node][next] > 0 {
                    parent[next] = node;
                    queue.push_back(next);
                }
            }
            if parent[SINK] != usize::MAX {
                break;
            }
        }
        if parent[SINK] == usize::MAX {
            break;
        }

        let mut amount = u32::MAX;
        let mut node = SINK;
        while node != SOURCE {
            let previous = parent[node];
            amount = amount.min(residual[previous][node]);
            node = previous;
        }
        node = SINK;
        while node != SOURCE {
            let previous = parent[node];
            residual[previous][node] -= amount;
            residual[node][previous] = residual[node][previous].saturating_add(amount);
            node = previous;
        }
        total = total.saturating_add(amount);
    }

    let mut allocations = [[0_u16; 2]; HybridPair::COUNT];
    for pair in HybridPair::ALL {
        let pair_node = PAIR_START + pair.index();
        let (first, second) = pair.colors();
        allocations[pair.index()] = [
            u16::try_from(residual[hybrid_color_node(first)][pair_node])
                .expect("hybrid flow is bounded by one u16 symbol count"),
            u16::try_from(residual[hybrid_color_node(second)][pair_node])
                .expect("hybrid flow is bounded by one u16 symbol count"),
        ];
    }
    HybridPayment { allocations, total }
}

/// Available colored capacity after this cost's fixed colored symbols have
/// been reserved. True colorless is deliberately absent: it never pays a
/// hybrid symbol.
pub(super) fn mana_available_for_hybrid(mut pool: ManaPool, cost: ManaCost) -> ManaPool {
    for color in colored_mana() {
        pool.remove_color(color, mana_cost_amount(cost, color));
    }
    pool.colorless = 0;
    pool
}

pub(super) fn can_cover_hybrid_cost(pool: ManaPool, cost: ManaCost) -> bool {
    if cost.hybrid_total() == 0 {
        return true;
    }
    maximum_hybrid_payment(mana_available_for_hybrid(pool, cost), cost, &|_| false).total
        == hybrid_required_total(cost)
}

/// Whether one colour can pay any hybrid symbol this cost carries.
pub(super) fn hybrid_pays_with(cost: ManaCost, color: ManaColor) -> bool {
    HybridPair::ALL
        .into_iter()
        .any(|pair| cost.hybrid[pair.index()] > 0 && pair.contains(color))
}
