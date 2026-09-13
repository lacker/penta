//! Direct spell-cost contributions bind a resource to a particular cost symbol.
//! They never enter the mana pool and are paid after mana abilities finish.
use super::super::{
    Game, GameObjectId, ManaColor, ManaContributionKind, ManaCost, ManaPool, PlannedManaActivation,
    PlannedPaymentKind,
};
use super::{
    ManaPaymentObligation, funding::PaymentDraft, preview::PaymentFrame,
    resources::PaymentReservation,
};
use crate::card::FlexibleManaSymbol;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::game) enum PaymentSymbol {
    Generic,
    Color(ManaColor),
    Flexible(FlexibleManaSymbol),
}

impl PaymentSymbol {
    pub(in crate::game) fn cost(self) -> ManaCost {
        match self {
            Self::Generic => ManaCost::new(1, 0),
            Self::Color(color) => ManaCost::of_color(color, 1),
            Self::Flexible(symbol) => ManaCost::default().with_flexible_symbol(symbol, 1),
        }
    }

    fn subtract(self, cost: &mut ManaCost) -> Option<()> {
        match self {
            Self::Generic => cost.generic = cost.generic.checked_sub(1)?,
            Self::Color(color) => {
                let amount = match color {
                    ManaColor::White => &mut cost.white,
                    ManaColor::Blue => &mut cost.blue,
                    ManaColor::Black => &mut cost.black,
                    ManaColor::Red => &mut cost.red,
                    ManaColor::Green => &mut cost.green,
                    ManaColor::Colorless => &mut cost.colorless,
                };
                *amount = amount.checked_sub(1)?;
            }
            Self::Flexible(symbol) => *cost = cost.without_flexible(symbol, 1)?,
        }
        Some(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::game) struct BoundContribution {
    pub(in crate::game) source: GameObjectId,
    pub(in crate::game) kind: ManaContributionKind,
    pub(in crate::game) symbol: PaymentSymbol,
}

#[derive(Clone, Debug)]
pub(in crate::game) struct BoundCastContributions {
    pub(in crate::game) plan: Vec<PlannedManaActivation>,
    pub(in crate::game) remaining: ManaPaymentObligation,
}

impl Game {
    pub(in crate::game) fn contribution_candidates(
        &self,
        draft: &PaymentDraft,
    ) -> Option<Vec<BoundContribution>> {
        let (preview, frame) = self.preview_funding(draft)?;
        if !preview.pending_decisions.is_empty() {
            return Some(Vec::new());
        }
        Some(preview.contributions_for_frame(&frame, &draft.contributions))
    }

    fn contributions_for_frame(
        &self,
        frame: &PaymentFrame,
        chosen: &[BoundContribution],
    ) -> Vec<BoundContribution> {
        let kinds = self.payment_contributions(&frame.obligation.purpose);
        let mut candidates = Vec::new();
        for permanent in self
            .battlefield
            .iter()
            .filter(|p| p.controller == frame.obligation.player)
        {
            let source = permanent.card.id;
            if chosen.iter().any(|c| c.source == source)
                || frame
                    .reserved
                    .iter()
                    .any(|r| matches!(r, PaymentReservation::Untapped(id) if *id == source))
            {
                continue;
            }
            for output in self.permanent_contribution_outputs(permanent, kinds) {
                let PlannedPaymentKind::Contribution(kind) = output.kind else {
                    continue;
                };
                let mut symbols = Vec::new();
                if frame.obligation.cost.generic > 0 {
                    symbols.push(PaymentSymbol::Generic);
                }
                for color in ManaColor::ALL {
                    if output.colored_contribution.amount(color) == 0 {
                        continue;
                    }
                    if super::super::mana_planning::mana_cost_amount(frame.obligation.cost, color)
                        > 0
                    {
                        symbols.push(PaymentSymbol::Color(color));
                    }
                    for symbol in FlexibleManaSymbol::ALL {
                        if frame.obligation.cost.flexible_count(symbol) > 0
                            && symbol.mana_options().contains(&color)
                        {
                            symbols.push(PaymentSymbol::Flexible(symbol));
                        }
                    }
                }
                for symbol in symbols {
                    let contribution = BoundContribution {
                        source,
                        kind,
                        symbol,
                    };
                    if !candidates.contains(&contribution) {
                        candidates.push(contribution);
                    }
                }
            }
        }
        if kinds.delve && frame.obligation.cost.generic > 0 {
            for card in &self.players[frame.obligation.player.index()].graveyard {
                if chosen.iter().any(|c| c.source == card.id)
                    || frame
                        .reserved
                        .iter()
                        .any(|r| matches!(r, PaymentReservation::Object(id) if *id == card.id))
                {
                    continue;
                }
                candidates.push(BoundContribution {
                    source: card.id,
                    kind: ManaContributionKind::Delve,
                    symbol: PaymentSymbol::Generic,
                });
            }
        }
        candidates
    }

    pub(in crate::game) fn apply_contribution_bindings(
        &self,
        frame: &mut PaymentFrame,
        bindings: &[BoundContribution],
    ) -> Option<()> {
        let mut chosen = Vec::new();
        for contribution in bindings {
            if !self
                .contributions_for_frame(frame, &chosen)
                .contains(contribution)
            {
                return None;
            }
            contribution.symbol.subtract(&mut frame.obligation.cost)?;
            chosen.push(*contribution);
        }
        Some(())
    }

    pub(in crate::game) fn bind_cast_contributions(
        draft: &PaymentDraft,
        remaining: ManaPaymentObligation,
    ) -> BoundCastContributions {
        let plan = draft
            .contributions
            .iter()
            .enumerate()
            .map(|(order, contribution)| PlannedManaActivation {
                source: contribution.source,
                kind: PlannedPaymentKind::Contribution(contribution.kind),
                production: ManaPool::default(),
                // The bound residual bill owns symbol allocation. These compatibility
                // fields are irrelevant to its execution; only the resource is paid.
                colored_contribution: ManaPool::default(),
                generic_payment: 1,
                life_payment: 0,
                benefits_payment: false,
                flexibility: 1,
                order,
            })
            .collect();
        BoundCastContributions { plan, remaining }
    }
}
