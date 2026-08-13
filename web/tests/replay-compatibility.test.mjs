import assert from "node:assert/strict";
import test from "node:test";

import { replayCompatibilityError } from "../worker/replay-compatibility.mjs";

const current = { replayVersion: 3, simulationFingerprint: "sha256-current" };

test("persisted games require their replay format and simulation identity", () => {
  assert.equal(replayCompatibilityError(current, current), null);
  assert.match(
    replayCompatibilityError({ ...current, replayVersion: 2 }, current),
    /replay 2/,
  );
  assert.match(
    replayCompatibilityError(
      { ...current, simulationFingerprint: "sha256-old" },
      current,
    ),
    /sha256-old/,
  );
});

test("package and bot-wire metadata do not gate a persisted replay", () => {
  assert.equal(
    replayCompatibilityError(
      { ...current, engineVersion: "old", protocolVersion: 1 },
      current,
    ),
    null,
  );
});

test("legacy records without exact replay metadata fail clearly", () => {
  assert.match(
    replayCompatibilityError({}, current),
    /before exact replay compatibility metadata/,
  );
});
