import test from "node:test";

import {
  opponentActionSnapshotsExcludeNextDraw,
  ownSpellResolutionIsNotOpponentBeat,
} from "./wasm-pacing-slow-scenarios.mjs";

test(
  "[slow] opponent-action snapshots never contain your next draw",
  opponentActionSnapshotsExcludeNextDraw,
);

test(
  "[slow] your own spell resolving is not a beat you have to sit through",
  ownSpellResolutionIsNotOpponentBeat,
);
