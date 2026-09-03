import assert from "node:assert/strict";
import test from "node:test";

import {
  MAX_VISIBLE_DECISION_OPTIONS,
  searchableDecisionOptions,
} from "../app/decision-options.mjs";

const choices = Array.from({ length: 60 }, (_, id) => ({
  id,
  label: id === 41 ? "Pithing Needle" : `Card ${String(id).padStart(2, "0")}`,
  zone: "None",
}));

test("large decisions begin as an empty type-ahead instead of a scrolling catalog", () => {
  const view = searchableDecisionOptions(choices, "");

  assert.equal(view.searchable, true);
  assert.deepEqual(view.matches, []);
  assert.equal(view.matchCount, choices.length);
  assert.equal(view.truncated, true);
});

test("type-ahead finds a card directly and caps broad result sets", () => {
  const exact = searchableDecisionOptions(choices, "needle");
  assert.deepEqual(exact.matches.map((option) => option.label), ["Pithing Needle"]);

  const broad = searchableDecisionOptions(choices, "card");
  assert.equal(broad.matches.length, MAX_VISIBLE_DECISION_OPTIONS);
  assert.equal(broad.matchCount, 59);
  assert.equal(broad.truncated, true);
});

test("small decisions remain visible and selected large-choice options stay reachable", () => {
  const small = searchableDecisionOptions(choices.slice(0, 3), "");
  assert.equal(small.searchable, false);
  assert.equal(small.matches.length, 3);

  const selected = searchableDecisionOptions(choices, "needle", [2]);
  assert.deepEqual(selected.matches.map((option) => option.id), [2, 41]);
});
