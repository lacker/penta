import assert from "node:assert/strict";
import test from "node:test";
import { DecisionView } from "./decision-view.mjs";
import { factorRows } from "./decision-format.mjs";

const definition = "00000000-0000-0000-0000-000000000001";
const catalog = { cards: [{ definition, name: "Printed name", rulesText: "Flying", parts: [{ id: 0, name: "Printed name", rulesText: "Flying" }] },
  { definition: "hidden", name: "Secret opponent card", rulesText: "Never inline" }] };
const observation = {
  protocolVersion: 32, seat: "p1", activeSeat: "p2", prioritySeat: "p1", turn: 3, activeTurn: 2, step: "Main1",
  life: [0, 20], manaPools: [{ red: 1 }, {}], hand: [{ objectId: 9, name: "Same name", definition }],
  battlefield: [{ objectId: 10, name: "Copied name", definition, power: 3, toughness: 2, tapped: false,
    counters: [{ name: "+1/+1", count: 1 }], art: { scryfallId: definition, artist: "Artist" } },
    { objectId: 11, name: "Same name", faceDown: true, tapped: true, power: 2, toughness: 2 }],
  stack: [{ objectId: 20, name: "Same name", targets: [{ permanent: 10 }], abilityOrigin: { kind: "card", definition, partId: 0, ability: 2 } }],
  graveyards: [[], []], exiles: [[], []], decision: null,
  publicReveals: [{ objectId: 10, name: "Historical name", definition }],
  updates: [{ type: "PrivateInspection", cards: [{ name: "Previously seen", definition }] }, { type: "Notice", value: 0 }, { type: "Notice", value: 0 }],
  future: { art: { semantic: true }, absentDistinctFromNull: null },
  legalActions: [
    { index: 0, type: "CastSpell", card: 9, modes: [1, 2], targets: [{ permanent: 10 }], x: 0, payment: { sacrifices: [] } },
    { index: 1, type: "CastSpell", card: 9, modes: [2, 1], targets: [{ permanent: 11 }], x: 1, payment: { sacrifices: [10] } },
    { index: 2, type: "ActivateManaAbility", source: 10, color: "Red", future: false },
    { index: 3, type: "FutureAction", novel: { values: [null, 0, false, []] } },
  ],
  checkpoint: { mustStayOutOfOrdinaryView: "checkpoint sentinel", definition: "hidden" },
};
const ready = (value = observation, revision = "a") => ({ apiVersion: 1, status: "ready", revision, observation: value });
function rows(value) { return Array.isArray(value) ? value : value.rows.map(row => ({ ...value.shared, ...row })); }

test("decision view preserves exact choices, exceptional state, histories and unknown fields", () => {
  const view = new DecisionView(ready(), catalog);
  const packet = view.packet;
  const choices = rows(packet.choices);
  assert.deepEqual(choices.map(({ ticket, label, ...action }) => action), observation.legalActions);
  assert.equal(new Set(choices.map(choice => choice.ticket)).size, choices.length);
  assert.deepEqual(choices.map(choice => view.choice(choice.ticket)), observation.legalActions.map(action => ({ index: action.index })));
  assert.match(choices[2].label, /Copied name/);
  assert.doesNotMatch(choices[2].label, /Historical/);
  const battlefield = rows(packet.position.battlefield).map(({ label, ...object }) => object);
  const expected = structuredClone(observation.battlefield); delete expected[0].art;
  assert.deepEqual(battlefield, expected);
  assert.deepEqual(packet.updates, observation.updates);
  assert.deepEqual(packet.facts.future, observation.future);
  assert.deepEqual(packet.facts.publicReveals, observation.publicReveals);
  assert.deepEqual(packet.position.stack[0].abilityOrigin, observation.stack[0].abilityOrigin);
  assert.doesNotMatch(JSON.stringify(packet), /checkpoint sentinel|Secret opponent card|hidden/);
  assert.deepEqual(view.inspect(packet.provenance.reference).items[0].checkpoint, observation.checkpoint);
  assert.equal(packet.rules[0].definition, definition);
  assert.match(packet.rules[0].printed.provenance, /Printed reference/);
  assert.equal(packet.rules[0].printed.rulesText, "Flying");
});

test("factoring roundtrips default distinctions, missing fields and ordered rows", () => {
  const values = [undefined, null, false, 0, [], {}, ""];
  const input = values.map(value => ({ common: "long shared value".repeat(20), ...(value === undefined ? {} : { value }) }));
  assert.deepEqual(rows(factorRows(input)), input);
  assert.ok(JSON.stringify(factorRows(input)).length < JSON.stringify(input).length);
});

test("decision tickets preserve the full selection space and require exact ordered option IDs", () => {
  const decision = { id: 12, minimum: 0, maximum: 2, order: "libraryTopFirst", cancelable: true,
    options: [{ id: 4, label: "Same" }, { id: 2, label: "Same" }] };
  const view = new DecisionView(ready({ ...observation, decision,
    legalActions: [{ index: 0, type: "ChooseDecision", decision: 12 }, { index: 1, type: "CancelDecision", decision: 12 }] }), catalog);
  const [choose, cancel] = rows(view.packet.choices);
  for (const options of [[], [4], [2], [4, 2], [2, 4]]) assert.deepEqual(view.choice(choose.ticket, options), { decision: 12, options });
  for (const options of [undefined, [0], [4, 4], [4, 2, 4]]) assert.throws(() => view.choice(choose.ticket, options), /options array|invalid option/);
  assert.deepEqual(view.choice(cancel.ticket), { index: 1 });
  assert.throws(() => view.choice(cancel.ticket, []), /do not accept/);
});

test("frozen references page every choice and disclosure; card text can be refreshed", () => {
  const actions = Array.from({ length: 220 }, (_, index) => ({ index, type: "FutureAction", payload: `Option ${index}` }));
  const updates = actions.map(({ index }) => ({ type: "Disclosure", name: `Card ${index}` }));
  const seen = new Set();
  const view = new DecisionView(ready({ ...observation, legalActions: actions, updates }), catalog, seen);
  assert.equal(view.packet.choices.count, 220);
  const all = [];
  let offset = 0;
  while (offset !== null) { const page = view.inspect(view.packet.choices.reference, { offset }); all.push(...page.items); offset = page.nextOffset; }
  assert.deepEqual(all.map(({ ticket, label, ...action }) => action), actions);
  assert.equal(view.inspect(view.packet.updates.reference, { offset: 219 }).items[0].name, "Card 219");
  assert.deepEqual(view.choice(all[219].ticket), { index: 219 });
  const cached = new DecisionView(ready(), catalog, seen);
  assert.equal(cached.packet.rules[0].printed, undefined);
  assert.equal(cached.inspect(cached.packet.rules[0].reference).items[0].rulesText, "Flying");
  assert.ok(new DecisionView(ready(), catalog, seen, true).packet.rules[0].printed);
  assert.throws(() => cached.inspect(view.packet.choices.reference), /expired reference/);
  view.active = false;
  assert.throws(() => view.choice(all[219].ticket), /stale ticket/);
});

test("decision view checks compatibility and does not join mismatched catalog text", () => {
  assert.throws(() => new DecisionView({ ...ready(), apiVersion: 2 }, catalog), /requires session API/);
  assert.throws(() => new DecisionView(ready({ ...observation, protocolVersion: 33 }), catalog), /bot protocol 32/);
  const view = new DecisionView(ready({ ...observation, simulationFingerprint: "current" }),
    { ...catalog, simulationFingerprint: "other" });
  assert.deepEqual(view.packet.rules, [{ definition, available: false }]);
});

test("future presentation-key collisions remain exact without replacing actionable tickets", () => {
  const action = { index: 42, type: "FutureAction", ticket: "engine field", label: "engine label", exact: false };
  const object = { objectId: 12, name: "Token", label: "engine object label" };
  const view = new DecisionView(ready({ ...observation, legalActions: [action], hand: [object] }), catalog);
  assert.deepEqual(view.packet.choices[0].exact, action);
  assert.deepEqual(view.choice(view.packet.choices[0].ticket), { index: 42 });
  assert.deepEqual(view.packet.position.hand[0], object);
});
