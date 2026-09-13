import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

const registry = JSON.parse(await readFile(new URL("../app/duel-commander-decks.json", import.meta.url), "utf8"));

test("Duel Commander exposes the eight published Bologna lists", () => {
  const names = Object.keys(registry);
  assert.equal(names.length, 8);
  assert.ok(names.every((name) => name.startsWith("CommandFest Italy 2026 — ")));
  assert.ok(names.some((name) => name.includes("Filippo Vicino")));
  assert.ok(names.some((name) => name.includes("Francesco Berruti")));
});
