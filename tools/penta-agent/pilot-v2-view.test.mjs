import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";
import { PilotV2View } from "./pilot-v2-view.mjs";
import { pilotV2Schema, validatePilotV2 } from "./pilot-v2-contract.mjs";
import { validateContract } from "./pilot-contract.mjs";
import { pilotV2Declarations } from "./generate-pilot-v2-types.mjs";

const observation = { protocolVersion: 33, seat: "p1", activeSeat: "p2", prioritySeat: "p1",
  life: [20, 19], hand: [], battlefield: [], stack: [], graveyards: [[], []], legalActions: [], updates: [], decision: null };
const envelope = obs => ({ apiVersion: 1, status: "ready", revision: "r1", observation: { ...observation, ...obs } });
const render = (obs, catalog) => new PilotV2View(envelope(obs), catalog);

test("v2 summaries retain current state and resolve unaltered canonical details", () => {
  const permanent = { objectId: 4, name: "Creature", definition: "d", controller: "p2", tapped: false,
    power: 2, toughness: 3, damage: 1, canAttack: false, enteredThisTurn: true, flying: true,
    counters: [{ name: "+1/+1", count: 1 }], attachments: [8], token: false, chosenColor: null,
    characteristics: { kind: "printed", definition: "d", partId: 0 }, hasIndividualState: true,
    future: { flag: false }, art: { artist: "A" } };
  const obs = { battlefield: [permanent], hand: [{ objectId: 1, name: "Hidden to opponent", definition: "h" }],
    exiles: [[], [{ objectId: 7, name: "Known" }]], future: { zero: 0 } };
  const view = render(obs), position = view.packet.position;
  const summary = position.zones.battlefield.items[0];
  assert.equal(summary.tapped, false);
  assert.equal(summary.canAttack, false);
  assert.equal(summary.power, 2);
  assert.deepEqual(summary.current, { flying: true, attachments: [8],
    characteristics: permanent.characteristics, hasIndividualState: true, future: { flag: false } });
  assert.deepEqual(view.inspect(summary.reference).items, [permanent]);
  assert.deepEqual(view.inspect(position.reference).items, [envelope(obs).observation]);
  assert.deepEqual(position.zones.exiles.items[0], []);
  assert.equal(position.zones.exiles.items[1][0].name, "Known");
  assert.deepEqual(position.facts.future, { zero: 0 });
  assert.throws(() => render({}).inspect(summary.reference), /unknown or expired/);
});

test("v2 never loses choices, decision options or update history when paging", () => {
  for (const count of [0, 1, 101]) {
    const actions = Array.from({ length: count }, (_, index) => ({ index, type: "PassPriority", future: null }));
    const options = Array.from({ length: count }, (_, id) => ({ id, future: [false] }));
    const view = render({ legalActions: actions, decision: { id: 2, minimum: 0, maximum: count, options }, updates: [{ future: 3 }] });
    const position = view.packet.position;
    for (const [collection, expected] of [[position.choices, actions], [position.decision.options, options]]) {
      assert.deepEqual(Object.keys(collection), ["items", "total", "reference"]);
      const rows = [...collection.items];
      if (collection.reference) {
        let offset = collection.items.length;
        do { const page = view.inspect(collection.reference, { offset }); rows.push(...page.items); offset = page.nextOffset; } while (offset !== null);
      }
      assert.deepEqual(rows.map(item => item.action ?? item), expected);
    }
    assert.deepEqual(position.updates.items, [{ future: 3 }]);
  }
});

test("v2 labels identify self-targets and distinguish abilities without UUIDs or default clutter", () => {
  const legalActions = [0, 1].map(index => ({ index, type: "ActivateAbility", source: 4,
    ability: { kind: "printed", definition: "full-uuid-descriptor", abilityId: index },
    targets: [{ type: "permanent", objectId: 4 }], x: 0, modeIds: [], manaPayment: null }));
  const choices = render({ battlefield: [{ objectId: 4, name: "Strip Mine" }], legalActions }).packet.position.choices.items;
  assert.match(choices[0].label, /target #4 Strip Mine/);
  assert.notEqual(choices[0].label, choices[1].label);
  assert.doesNotMatch(choices[0].label, /full-uuid|manaPayment|X=0|modeIds/);
  assert.deepEqual(choices.map(choice => choice.action), legalActions);
});

test("v2 names yield timing explicitly and leaves input untouched", () => {
  const state = { requestId: "y", status: "active", until: [{ type: "opponentActs" }], maxPasses: 64, passes: 6, stopReason: null, conditionIndex: null };
  const host = { apiVersion: 1, status: "waiting", yield: { ...state, passes: 8 }, receipt: { requestId: "y", accepted: true, yield: state } };
  const before = structuredClone(host), packet = new PilotV2View(host).packet;
  assert.equal(packet.host.currentYield.passes, 8);
  assert.equal(packet.host.receipt.yieldAtAcceptance.passes, 6);
  assert.equal(packet.host.yield, undefined);
  assert.equal(packet.host.receipt.yield, undefined);
  assert.deepEqual(host, before);
});

test("v2 rules show printed text with complete catalog on demand and stable references", () => {
  const card = { definition: "d", name: "Card", rulesText: "Draw a card.", manaCost: { blue: 1 }, future: { large: true } };
  const obs = { hand: [{ objectId: 2, definition: "d", name: "Card" }] };
  const full = render(obs, { cards: [card] }), missing = render(obs, { cards: [] });
  const rule = full.packet.position.rules.items[0];
  assert.deepEqual(rule.printed, { rulesText: "Draw a card.", manaCost: { blue: 1 } });
  assert.deepEqual(full.inspect(rule.reference).items, [card]);
  assert.equal(missing.packet.position.rules.items[0].available, false);
  assert.equal(full.packet.position.zones.hand.items[0].reference, missing.packet.position.zones.hand.items[0].reference);
});

test("v2 schema resolves locally and declarations are generated from that document", () => {
  const walk = value => {
    if (!value || typeof value !== "object") return;
    if (value.$ref) assert.ok(pilotV2Schema.$defs[value.$ref.split("/").at(-1)]);
    Object.values(value).forEach(walk);
  };
  walk(pilotV2Schema);
  assert.equal(readFileSync(new URL("./pilot-v2.d.ts", import.meta.url), "utf8"), pilotV2Declarations());
  const packet = render({ battlefield: [{ objectId: 1, tapped: true }] }).packet;
  packet.position.zones.battlefield.items[0].tapped = "yes";
  assert.throws(() => validatePilotV2(packet), /battlefield\.items\[0\]\.tapped: expected boolean/);
});

test("v2 printed rules avoid single-part compatibility duplicates and retain multiple parts", () => {
  const front = { id: 0, name: "Card", rulesText: "Draw a card.", manaCost: { blue: 1 }, typeLine: "Sorcery" };
  const back = { id: 1, name: "Other", rulesText: "Gain 2 life.", typeLine: "Sorcery" };
  const card = { definition: "d", name: "Card", ...front, structure: { kind: "single", mainPartId: 0 }, parts: [front] };
  const obs = { hand: [{ objectId: 2, definition: "d", name: "Card" }] };
  assert.deepEqual(render(obs, { cards: [card] }).packet.position.rules.items[0].printed,
    { rulesText: front.rulesText, manaCost: front.manaCost, typeLine: front.typeLine });
  const split = { ...card, structure: { kind: "split", leftPartId: 0, rightPartId: 1 }, parts: [front, back] };
  assert.deepEqual(render(obs, { cards: [split] }).packet.position.rules.items[0].printed.parts, [front, back]);
});

test("v2 ability identities survive reordered and disappearing actions", () => {
  const actions = [
    { index: 0, type: "ActivateAbility", source: 4, ability: { kind: "printed", definition: "d", partId: 0, abilityId: 1 }, targets: [{ type: "permanent", objectId: 4 }] },
    { index: 1, type: "ActivateAbility", source: 4, ability: { kind: "printed", definition: "d", partId: 0, abilityId: 2 }, targets: [] },
    { index: 2, type: "ActivateAbility", source: 4, ability: { kind: "granted", source: 7, sourceAbilityId: 3, grantId: 5 }, targets: [] },
  ];
  const labels = legalActions => render({ battlefield: [{ objectId: 4, name: "Source" }], legalActions }).packet.position.choices.items;
  const original = labels(actions);
  assert.deepEqual(labels([...actions].reverse()).map(choice => choice.label), original.map(choice => choice.label).reverse());
  for (const [index, action] of actions.entries()) assert.equal(labels([action])[0].label, original[index].label);
  assert.match(original[0].label, /target #4 Source/);
  assert.match(original[0].label, /printed .* 0\.1/);
  const reordered = { ...actions[2], ability: { grantId: 5, sourceAbilityId: 3, source: 7, kind: "granted" } };
  assert.equal(labels([reordered])[0].label, original[2].label);
  assert.deepEqual(original.map(choice => choice.action), actions);
});

test("v2 short references preserve catalog stability and reject other revisions", () => {
  const revision = "long-native-revision-".repeat(20);
  const item = { objectId: 4, name: "Card", definition: "d", future: { exact: [false, null] } };
  const host = { ...envelope({ hand: [item] }), revision };
  const full = new PilotV2View(host, { cards: [{ definition: "d", name: "Card" }] });
  const missing = new PilotV2View(host, { cards: [] });
  const next = new PilotV2View({ ...host, revision: `${revision}next` });
  const reference = full.packet.position.zones.hand.items[0].reference;
  assert.match(reference, /^p2r:[A-Za-z0-9_-]{43}$/);
  assert.equal(reference.length, 47);
  assert.ok(reference.length < revision.length / 4);
  assert.equal(reference, missing.packet.position.zones.hand.items[0].reference);
  assert.deepEqual(full.inspect(reference).items, [item]);
  assert.deepEqual(missing.inspect(reference).items, [item]);
  assert.notEqual(reference, next.packet.position.zones.hand.items[0].reference);
  assert.throws(() => next.inspect(reference), /unknown or expired/);
  assert.notEqual(reference, full.packet.position.reference);
  assert.throws(() => full.inspect(`${revision}:r:pilot-v1:hand.0`), /unknown or expired/);
});

test("v2 schema permits explicit previously-delivered printed summaries", () => {
  const view = render({ hand: [{ objectId: 4, name: "Card", definition: "d" }] },
    { cards: [{ definition: "d", name: "Card", rulesText: "Draw a card." }] });
  const rule = view.packet.position.rules.items[0];
  rule.printed = null;
  rule.printedPreviouslySent = true;
  validatePilotV2(view.packet);
  assert.equal(view.inspect(rule.reference).items[0].rulesText, "Draw a card.");
  rule.printedPreviouslySent = "yes";
  assert.throws(() => validatePilotV2(view.packet), /printedPreviouslySent: expected boolean/);
});


test("v2 native play choices follow strict host wrapper shapes", () => {
  const check = value => validateContract(value, "NativePlayChoice", pilotV2Schema);
  for (const value of [{ index: 0 }, { action: { type: "PassPriority", index: 1, future: null } }, { decision: 1, options: [0, 2] }]) assert.deepEqual(check(value), value);
  for (const value of [{ index: -1 }, { ticket: "x" }, { index: 0, action: { type: "PassPriority" } }, { action: {} }, { action: { type: 1 } }, { decision: 1, options: [-1] }, { decision: 1, options: Array(4097).fill(0) }, { decision: 1, options: [], future: false }]) assert.throws(() => check(value), /NativePlayChoice\./);
});

test("v2 interaction schema requires explicit choices or a bounded yield", () => {
  const check = value => validateContract(value, "NativeInteractionRequest", pilotV2Schema);
  const base = { requestId: "move-1", revision: "r1", choices: [{ action: { type: "PlayLand", card: 4 } }] };
  assert.deepEqual(check(base), base);
  const policy = { until: [{ type: "nextStep", step: "End" }, { type: "opponentActs" }], maxPasses: 64 };
  check({ ...base, yield: policy });
  check({ ...base, choices: [], yield: policy });
  for (const change of [
    { choices: [] }, { choices: [{ index: 0 }] },
    { yield: { until: [] } }, { yield: { ...policy, maxPasses: 257 } },
    { yield: { until: [{ type: "guessWhatIWant" }] } }, { maxWaitMs: 60000 },
  ]) assert.throws(() => check({ ...base, ...change }), /NativeInteractionRequest\./);
});

test("v2 concrete decision labels identify selected option IDs without changing actions", () => {
  const legalActions = [
    { index: 0, type: "ChooseDecision", decision: 2, options: [7] },
    { index: 1, type: "ChooseDecision", decision: 2, options: [9] },
    { index: 2, type: "ChooseDecision", decision: 2, options: [] },
  ];
  const choices = render({ legalActions, decision: { id: 2, minimum: 0, maximum: 1, options: [
    { id: 7, label: "Mountain" }, { id: 9, label: "Chaos Orb" },
  ] } }).packet.position.choices.items;
  assert.match(choices[0].label, /select \[7\] Mountain/);
  assert.match(choices[1].label, /select \[9\] Chaos Orb/);
  assert.match(choices[2].label, /select no options/);
  assert.deepEqual(choices.map(choice => choice.action), legalActions);
});

test("v2 large menus retain an inline prefix and expose every action family", () => {
  const legalActions = [
    ...Array.from({ length: 40 }, (_, index) => ({ index, type: "CastSpell", card: 4, x: index, detail: "x".repeat(400) })),
    { index: 40, type: "PlayLand", card: 5 },
  ];
  const view = render({ legalActions, hand: [{ objectId: 4, name: "Fireball" }, { objectId: 5, name: "Mountain" }] });
  const { choices, choiceGroups } = view.packet.position;
  assert.ok(choices.items.length > 0 && choices.items.length < choices.total);
  assert.deepEqual(choiceGroups.map(group => [group.firstIndex, group.count]), [[0, 40], [40, 1]]);
  assert.match(choiceGroups[1].example, /Mountain/);
  assert.equal(view.inspect(choices.reference, { offset: 40, limit: 1 }).items[0].action.type, "PlayLand");
  validatePilotV2(view.packet);
});
