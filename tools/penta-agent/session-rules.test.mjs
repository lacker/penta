import assert from "node:assert/strict";
import test from "node:test";
import { SessionRules } from "./session-rules.mjs";
import { validateContract } from "./pilot-contract.mjs";
import { pilotV3Schema } from "./pilot-v3-contract.mjs";

const rule = (definition = "d", text = "Draw a card.") =>
  ({ definition, available: true, name: "Card", printed: { rulesText: text }, reference: `r:${definition}` });
const packet = items => ({ presentation: "pilot-v2", status: "ready", host: { receipt: { updates: [1] } },
  position: { rules: { items, total: items.length, reference: null }, choices: { items: [2] }, updates: [3] } });
function deliver(cache, value) {
  const prepared = cache.prepare(value);
  prepared.commit();
  return prepared.packet;
}

test("only repeated semantic printed summaries are omitted, without changing other state or inputs", () => {
  const cache = new SessionRules();
  const original = packet([rule()]);
  const before = structuredClone(original);
  assert.deepEqual(deliver(cache, original), before);
  const repeated = deliver(cache, original);
  assert.deepEqual(repeated.position.rules.items, [{ definition: "d", printed: null, printedPreviouslySent: true, reference: "r:d" }]);
  assert.deepEqual({ ...repeated.position, rules: undefined }, { ...original.position, rules: undefined });
  assert.deepEqual(repeated.host, original.host);
  assert.deepEqual(original, before);
  assert.deepEqual(deliver(cache, packet([rule("d", "Draw two cards."), rule("new")])), packet([rule("d", "Draw two cards."), rule("new")]));
  assert.deepEqual(deliver(cache, original), original, "changing back also reprints");
});

test("object order is ignored, but array order and changed values are meaningful", () => {
  const cache = new SessionRules();
  const first = { ...rule(), printed: { rulesText: "a", manaCost: { blue: 1, generic: 2 }, parts: [1, 2] } };
  deliver(cache, packet([first]));
  const reordered = { ...first, printed: { parts: [1, 2], manaCost: { generic: 2, blue: 1 }, rulesText: "a" } };
  assert.equal(deliver(cache, packet([reordered])).position.rules.items[0].printedPreviouslySent, true);
  const changed = { ...first, printed: { ...first.printed, parts: [2, 1] } };
  assert.deepEqual(deliver(cache, packet([changed])).position.rules.items[0], changed);
});

test("unavailable, paged and inspected rules never imply delivery of omitted printed contents", () => {
  const cache = new SessionRules();
  const paged = packet([]);
  paged.position.rules = { items: [], total: 1, reference: "rules:page" };
  deliver(cache, paged);
  deliver(cache, { items: [rule()], reference: "rules:page" });
  assert.deepEqual(deliver(cache, packet([rule()])), packet([rule()]));
  const unavailable = { ...rule(), available: false, printed: null };
  assert.deepEqual(deliver(cache, packet([unavailable])), packet([unavailable]));
  assert.deepEqual(deliver(cache, packet([rule()])), packet([rule()]));
});

test("uncommitted output, reset, independent sessions, exact and pilot-v1 never suppress initial details", () => {
  const cache = new SessionRules();
  const value = packet([rule()]);
  cache.prepare(value); // Represents a failed output write.
  assert.deepEqual(deliver(cache, value), value);
  cache.reset();
  assert.deepEqual(deliver(cache, value), value);
  assert.deepEqual(deliver(new SessionRules(), value), value);
  for (const presentation of ["exact", "pilot-v1"]) {
    const legacy = { ...value, presentation };
    assert.deepEqual(deliver(cache, legacy), legacy);
    assert.deepEqual(deliver(cache, legacy), legacy);
  }
});


test("cached rows retain current references and reprint changed metadata", () => {
  const cache = new SessionRules();
  deliver(cache, packet([rule()]));
  const refreshed = { ...rule(), reference: "new-reference" };
  assert.deepEqual(deliver(cache, packet([refreshed])).position.rules.items[0], {
    definition: "d", printed: null, printedPreviouslySent: true, reference: "new-reference",
  });
  const renamed = { ...refreshed, name: "Changed card name" };
  assert.deepEqual(deliver(cache, packet([renamed])).position.rules.items[0], renamed);
});

const abilityRecord = (source = 8, text = "Add red mana.") => ({ source, abilities: [{
  ability: { kind: "intrinsicBasicLand", landType: "mountain" }, kind: "mana", text,
  targetSlots: [], modes: null,
}] });
const proposalPacket = actionAbilities => ({ presentation: "pilot-v3", status: "ready",
  position: { rules: { items: [], total: 0, reference: null }, actionAbilities,
    zones: { hand: { items: [{ objectId: 8 }], total: 1, reference: null } },
    decision: { options: { items: [{ id: 2 }], total: 1, reference: null } },
  } });
const cachedAbility = source => ({ source, abilitiesPreviouslySent: true });

test("proposal sessions retain visible sources and suppress only identical delivered ability records", () => {
  const cache = new SessionRules();
  const original = proposalPacket([abilityRecord()]);
  const before = structuredClone(original);
  assert.deepEqual(deliver(cache, original), original);
  const repeated = deliver(cache, original);
  assert.deepEqual(repeated.position.actionAbilities, [cachedAbility(8)]);
  assert.deepEqual({ ...repeated.position, actionAbilities: undefined }, { ...original.position, actionAbilities: undefined });
  assert.deepEqual(original, before);
  const changed = abilityRecord(8, "Add two red mana.");
  const added = abilityRecord(9);
  assert.deepEqual(deliver(cache, proposalPacket([changed, added])).position.actionAbilities, [changed, added]);
  const withCost = { ...added, grantedCastCosts: [{ playOptionId: 0, alternativeCostId: 0,
    kind: "granted", text: "Cast for free.", manaCost: "{0}" }] };
  assert.deepEqual(deliver(cache, proposalPacket([changed, withCost])).position.actionAbilities, [cachedAbility(8), withCost]);
  assert.deepEqual(deliver(cache, proposalPacket([changed])).position.actionAbilities, [cachedAbility(8)]);
  assert.deepEqual(deliver(cache, proposalPacket([changed, withCost])).position.actionAbilities, [cachedAbility(8), withCost]);
  deliver(cache, proposalPacket([]));
  assert.deepEqual(deliver(cache, original), original);
});

test("proposal knowledge commits only with delivery and reset restores full context", () => {
  const cache = new SessionRules();
  const original = proposalPacket([abilityRecord()]);
  cache.prepare(original);
  assert.deepEqual(deliver(cache, original), original);
  cache.prepare(proposalPacket([])); // A lost disappearance must not prune knowledge.
  assert.deepEqual(deliver(cache, original).position.actionAbilities, [cachedAbility(8)]);
  const changed = proposalPacket([abilityRecord(8, "Changed ability.")]);
  cache.prepare(changed);
  assert.deepEqual(deliver(cache, changed), changed);
  cache.reset(); // The session uses this for observe/full:true after context loss.
  assert.deepEqual(deliver(cache, changed), changed);
  assert.deepEqual(deliver(new SessionRules(), changed), changed);
  const legacy = { ...changed, presentation: "pilot-v2" };
  assert.deepEqual(deliver(cache, legacy), legacy);
  assert.deepEqual(deliver(cache, legacy), legacy);
});

test("proposal ability metadata compares all fields semantically and works without printed rules", () => {
  const cache = new SessionRules();
  const original = proposalPacket([abilityRecord()]);
  delete original.position.rules;
  deliver(cache, original);
  const record = original.position.actionAbilities[0];
  const reordered = { abilities: record.abilities.map(({ text, ...rest }) => ({ ...rest, text })), source: record.source };
  assert.deepEqual(deliver(cache, proposalPacket([reordered])).position.actionAbilities, [cachedAbility(8)]);
  const two = { ...record, abilities: [...record.abilities, { ...record.abilities[0], text: "Second ability." }] };
  deliver(cache, proposalPacket([two]));
  const reversed = { ...two, abilities: [...two.abilities].reverse() };
  assert.deepEqual(deliver(cache, proposalPacket([reversed])).position.actionAbilities, [reversed]);
});

test("proposal ability contract distinguishes full records from exact cached markers", () => {
  for (const record of [abilityRecord(), cachedAbility(8)]) {
    validateContract(record, "ProposalObjectAbilities", pilotV3Schema);
  }
  for (const record of [{ source: 8 }, { source: 8, abilitiesPreviouslySent: false },
    { ...abilityRecord(), abilitiesPreviouslySent: true }, { abilitiesPreviouslySent: true }]) {
    assert.throws(() => validateContract(record, "ProposalObjectAbilities", pilotV3Schema));
  }
});
