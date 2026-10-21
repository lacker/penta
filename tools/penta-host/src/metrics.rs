//! Organizer-only lifetime command aggregates, independent of the diagnostic ring.

use serde::Serialize;

use crate::Host;

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CommandMetric {
    count: u64,
    total_micros: u128,
    max_micros: u128,
}

impl Host {
    pub(crate) fn record_command_duration(&mut self, method: &str, micros: u128) {
        // Never retain arbitrary method strings, which may contain private input.
        let method = match method {
            "seats" | "status" | "export" | "close" | "observe" | "wait" | "choose" | "play"
            | "interact" | "concede" | "yield" | "cancelYield" | "receipt" | "transcript"
            | "note" | "catalog" | "discover" => method,
            _ => "unknown",
        };
        let metric = self.command_metrics.entry(method.to_owned()).or_default();
        metric.count = metric.count.saturating_add(1);
        metric.total_micros = metric.total_micros.saturating_add(micros);
        metric.max_micros = metric.max_micros.max(micros);
    }
}
