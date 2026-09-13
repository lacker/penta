import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

const registry = JSON.parse(
  await readFile(new URL("../app/cedh-decks.json", import.meta.url), "utf8"),
);

test("cEDH setup registry exposes the top 16 event lists", () => {
  const entries = Object.entries(registry);
  assert.equal(entries.length, 16);
  assert.ok(
    entries.every(([name, description]) =>
      name.startsWith("Nacional de cEDH 100K @ WolfCon 2026 — ")
      && description.startsWith("Exact tournament decklist for "),
    ),
  );
  assert.equal(
    registry["Nacional de cEDH 100K @ WolfCon 2026 — 1st place, Gustavo Arrambide"],
    "Exact tournament decklist for Gustavo Arrambide, 1st place at Nacional de cEDH 100K @ WolfCon 2026.",
  );
});
