import assert from "node:assert/strict";
import test from "node:test";

import { maxSeed, parseSeed, seedTextIsInvalid } from "../app/match-seed.mjs";

test("a whole number within 32 bits is a seed", () => {
  assert.equal(parseSeed("0"), 0);
  assert.equal(parseSeed("9394"), 9394);
  assert.equal(parseSeed(String(maxSeed)), maxSeed);
  // A player pasting a seed should not be tripped up by surrounding spaces.
  assert.equal(parseSeed("  42 "), 42);
});

test("anything the engine cannot deal is rejected", () => {
  assert.equal(parseSeed(null), null);
  assert.equal(parseSeed(undefined), null);
  assert.equal(parseSeed(""), null);
  assert.equal(parseSeed("   "), null);
  assert.equal(parseSeed("-1"), null);
  assert.equal(parseSeed("1.5"), null);
  assert.equal(parseSeed("1e3"), null);
  assert.equal(parseSeed("seed"), null);
  assert.equal(parseSeed(String(maxSeed + 1)), null);
});

test("only non-empty unparseable text is a mistake", () => {
  assert.equal(seedTextIsInvalid(""), false);
  assert.equal(seedTextIsInvalid("  "), false);
  assert.equal(seedTextIsInvalid("7"), false);
  assert.equal(seedTextIsInvalid("seven"), true);
  assert.equal(seedTextIsInvalid("99999999999"), true);
});
