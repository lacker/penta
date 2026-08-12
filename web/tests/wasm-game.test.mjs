// Compatibility entry point for tools that run the complete WASM suite.
// Targeted development commands execute the domain suite files directly.
import "./wasm-casting.suite.mjs";
import "./wasm-combat.suite.mjs";
import "./wasm-combat-slow.suite.mjs";
import "./wasm-contract.suite.mjs";
import "./wasm-pacing.suite.mjs";
import "./wasm-pacing-actions-slow.suite.mjs";
import "./wasm-pacing-handover-slow.suite.mjs";
import "./wasm-pacing-replay-slow.suite.mjs";
import "./wasm-state.suite.mjs";
