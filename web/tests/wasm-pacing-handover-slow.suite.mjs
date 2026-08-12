import test from "node:test";

import { ownPlaySettlesBeforeTurnHandover } from "./wasm-pacing-slow-scenarios.mjs";

test(
  "[slow] your own play is on the board before the turn it ended is announced",
  ownPlaySettlesBeforeTurnHandover,
);
