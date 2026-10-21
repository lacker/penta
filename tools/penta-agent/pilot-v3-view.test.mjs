import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";
import { PilotV3View } from "./pilot-v3-view.mjs";
import { pilotV3Schema } from "./pilot-v3-contract.mjs";
import { validateContract } from "./pilot-contract.mjs";
import { pilotV3Declarations } from "./generate-pilot-v3-types.mjs";
import { nativeCommand } from "./native-client.mjs";

const observation = { protocolVersion: 33, actionInterface: "proposal-v1", seat: "p1", hand: [], battlefield: [],
  stack: [], updates: [], decision: null, actionContext: { kind: "priority", ordinaryPriority: true }, actionAbilities: [] };
const envelope = { apiVersion: 2, status: "ready", revision: "r1", observation };
const credentials = { server: "http://127.0.0.1:41234", secret: "seat-only" };

test("proposal state retains all visible objects, options and rules without a menu or paging", () => {
  const cards = Array.from({ length: 120 }, (_, objectId) => ({ objectId, definition: "d", name: "Card", tapped: false,
    characteristics: { kind: "printed", definition: "d", partId: 1 }, future: { flag: false } }));
  const printed = { definition: "d", name: "Card", rulesText: "A".repeat(13000), parts: [{ id: 1, name: "Card", playOptions: [{ id: 2 }] }] };
  const options = cards.map(card => ({ id: card.objectId, card }));
  const obs = { ...observation, hand: cards, decision: { id: 9, minimum: 0, maximum: 120, options } };
  const view = new PilotV3View({ ...envelope, observation: obs }, { cards: [printed] });
  const position = view.packet.position;
  assert.equal(position.actionInterface, "proposal-v1");
  assert.ok(!Object.hasOwn(position, "choices"));
  assert.deepEqual(position.zones.hand.items, cards);
  assert.deepEqual(position.decision.options.items, options);
  assert.deepEqual(position.rules.items[0].printed, printed);
  assert.deepEqual(position.actionContext, obs.actionContext);
  assert.deepEqual(position.actionAbilities, obs.actionAbilities);
  assert.ok(!Object.hasOwn(position.facts, "actionContext"));
  assert.ok(!Object.hasOwn(position.facts, "actionAbilities"));
  assert.deepEqual(view.inspect(position.reference).items, [obs]);
});

test("sideboarding includes printed rules for the seat's entire main deck and sideboard", () => {
  const cards = ["main", "side", "hidden"].map(definition => ({ definition,
    name: definition, rulesText: `Rules for ${definition}`, parts: [] }));
  const obs = { ...observation, actionContext: { kind: "decision", ordinaryPriority: false },
    match: { mode: "first-to-two-wins", stage: "sideboarding", wins: [1, 0],
      main: ["main", "main"], sideboard: ["side"] },
    checkpoint: { definition: "hidden" },
    decision: { id: 7, minimum: 2, maximum: 3, options: [
      { id: 0, label: "main", card: null, zone: "Library" },
      { id: 1, label: "main", card: null, zone: "Library" },
      { id: 2, label: "side", card: null, zone: "OutsideGame" },
    ] } };
  const view = new PilotV3View({ ...envelope, observation: obs }, { cards });
  assert.deepEqual(view.packet.position.rules.items.map(rule => rule.definition), ["main", "side"]);
  assert.deepEqual(view.packet.position.rules.items.map(rule => rule.printed), cards.slice(0, 2));
  assert.deepEqual(view.packet.position.situation.match, obs.match);
  assert.deepEqual(view.packet.position.decision.options.items.map(row =>
    ({ ...view.packet.position.decision.options.defaults, ...row })), obs.decision.options);
  const missing = new PilotV3View({ ...envelope, observation: obs }, { cards: [] });
  assert.ok(missing.packet.position.rules.items.every(rule => !rule.available));
});

test("inline printed rules omit presentation metadata while references preserve the exact catalog", () => {
  const printed = { definition: "d", name: "Mountain", rulesText: "R".repeat(13000),
    isBasicLand: true, manaCost: null, structure: { kind: "single", part: 0 },
    parts: [{ id: 0, name: "Mountain", types: ["land"], subtypes: ["mountain"],
      future: { art: "semantic", printings: ["retain nested unknown fields"] } }],
    playOptions: [{ id: 0, kind: "land", part: 0 }],
    future: { flag: false, absentDistinctFromNull: null },
    art: { scryfallId: "primary", artist: "Artist" },
    printings: Array.from({ length: 120 }, (_, variant) => ({ set: "test", variant,
      art: { scryfallId: `printing-${variant}`, artist: "Artist" } })) };
  const catalog = { cards: [printed] };
  const original = structuredClone(catalog);
  const view = new PilotV3View({ ...envelope, observation: { ...observation,
    hand: [{ objectId: 1, definition: "d", name: "Mountain" }] } }, catalog);
  const rule = view.packet.position.rules.items[0];
  const expected = structuredClone(printed);
  delete expected.printings;
  delete expected.art;
  assert.deepEqual(rule.printed, expected);
  assert.deepEqual(view.inspect(rule.reference).items, [original.cards[0]]);
  assert.deepEqual(catalog, original);
});

test("large decision tables factor only known empty metadata and reconstruct every option", () => {
  const options = Array.from({ length: 325 }, (_, id) => ({ id, label: `Creature type ${id}`,
    abilityText: null, card: null, members: [], triggerId: null, zone: "None", future: null }));
  options[1] = { ...options[1], card: { objectId: 2 }, members: [{ name: "Member" }],
    abilityText: "Choose this", triggerId: 7, zone: "Battlefield" };
  const obs = { ...observation, decision: { id: 9, minimum: 1, maximum: 1, options } };
  const before = structuredClone(obs);
  const view = new PilotV3View({ ...envelope, observation: obs }, { cards: [] });
  const compact = view.packet.position.decision.options;
  assert.equal(compact.total, 325);
  assert.deepEqual(compact.defaults, { abilityText: null, card: null, members: [], triggerId: null, zone: "None" });
  assert.deepEqual(compact.items[0], { id: 0, label: "Creature type 0", future: null });
  assert.deepEqual(compact.items.map(item => ({ ...compact.defaults, ...item })), options);
  assert.deepEqual(view.inspect(view.packet.position.reference).items, [before]);
  assert.deepEqual(obs, before);
  assert.ok(JSON.stringify(compact).length < JSON.stringify(options).length / 2);
  delete options[0].card;
  const absent = new PilotV3View({ ...envelope, observation: obs }, { cards: [] }).packet.position.decision.options;
  assert.ok(!Object.hasOwn(absent.defaults, "card"));
  assert.deepEqual(absent.items.map(item => ({ ...absent.defaults, ...item })), options);
});

test("native default sends one typed proposal unchanged and never discovers", async () => {
  const calls = [];
  const fetcher = async (_url, options) => {
    const body = JSON.parse(options.body); calls.push(body);
    return Response.json(body.method === "catalog" ? { cards: [] } : envelope);
  };
  const proposal = { action: { type: "CastSpell", card: 4, choices: { playOptionId: 0,
    targetSelections: [{ slotId: 0, targets: [{ type: "player", seat: "p2" }] }] } },
    payment: { funding: [{ action: { type: "ActivateManaAbility", source: 8,
      ability: { kind: "intrinsicBasicLand", landType: "mountain" }, color: "red" } }], manaUnits: [0] } };
  const result = await nativeCommand("interact", { ...credentials, revision: "r1", requestId: "m1", choices: [{ proposal }] }, fetcher);
  assert.equal(result.presentation, "pilot-v3");
  assert.deepEqual(calls.map(call => call.method), ["interact", "catalog"]);
  assert.deepEqual(calls[0].args.choices, [{ proposal }]);
  assert.ok(!Object.hasOwn(calls[0].args, "actionInterface"));
  const malformed = structuredClone(proposal); malformed.action.targets = [];
  await assert.rejects(nativeCommand("interact", { ...credentials, revision: "r1", requestId: "m2", choices: [{ proposal: malformed }] }, fetcher), /proposal\.action\.targets: unexpected field/);
  assert.equal(calls.length, 2);
});

test("native proposal input errors preserve actionable field paths through unions", async () => {
  const cases = [
    [{ type: "Mulligan" }, /proposal\.action\.type: expected one of .*"TakeMulligan"/],
    [{ type: "CastSpell" }, /proposal\.action\.card: required/],
    [{ type: "CastSpell", card: 1, choices: { x: 65536 } }, /proposal\.action\.choices\.x: number out of bounds/],
    [{ type: "CastSpell", card: 1, choices: { targetSelections: [{ slotId: 0,
      targets: [{ type: "player", objectId: 2 }] }] } }, /targets\[0\]\.seat: required/],
    [{ type: "PlayLand", card: 1, typo: 0 }, /proposal\.action\.typo: unexpected field/],
  ];
  for (const method of ["interact", "play"]) {
    for (const [action, message] of cases) {
      await assert.rejects(nativeCommand(method, { ...credentials, revision: "r1", requestId: "invalid",
        choices: [{ proposal: { action } }] }, () => assert.fail("invalid input must not reach the host")), message);
    }
  }
  await assert.rejects(nativeCommand("interact", { ...credentials, revision: "r1", requestId: "invalid",
    choices: [], yield: { until: [{ type: "stackEmpty" }] } }, () => assert.fail("invalid yield must not reach the host")),
  /yield\.until\[0\]\.type: expected one of .*"opponentCastsSpell"/);
});

test("proposal schema rejects malformed targets and generated declarations stay current", () => {
  assert.throws(() => validateContract({ action: { type: "CastSpell", card: 1, choices: { playOptionId: 0,
    targetSelections: [{ slotId: 0, targets: [{ type: "player", objectId: 4 }] }] } } }, "NativeProposal", pilotV3Schema));
  assert.equal(readFileSync(new URL("./pilot-v3.d.ts", import.meta.url), "utf8"), pilotV3Declarations());
});

test("proposal mana combinations replace the redundant color field", () => {
  const action = { type: "ActivateManaAbility", source: 8,
    ability: { kind: "intrinsicBasicLand", landType: "forest" }, combination: { blue: 1, green: 1 } };
  validateContract(action, "ProposalManaAction", pilotV3Schema);
  validateContract({ ...action, color: "green" }, "ProposalManaAction", pilotV3Schema);
  assert.throws(() => validateContract({ ...action, source: -1 }, "ProposalManaAction", pilotV3Schema));
  assert.throws(() => validateContract({ ...action, combination: null }, "ProposalManaAction", pilotV3Schema));
  assert.throws(() => validateContract({ ...action, extra: 1 }, "ProposalManaAction", pilotV3Schema));
});


test("proposal context and effective ability metadata are typed and required", () => {
  const ability = { ability: { kind: "intrinsicBasicLand", landType: "mountain" }, kind: "mana", text: "Add red mana.",
    targetSlots: [{ slotId: 0, minimum: 1, maximum: 1, countFromRules: false, another: false, excludesSource: false, dividedTotal: null }], modes: null };
  const view = new PilotV3View({ ...envelope, observation: { ...observation,
    actionAbilities: [{ source: 8, abilities: [ability] }] } }, { cards: [] });
  assert.deepEqual(view.packet.position.actionAbilities, [{ source: 8, abilities: [ability] }]);
  const missing = structuredClone(envelope);
  delete missing.observation.actionContext;
  assert.throws(() => new PilotV3View(missing, { cards: [] }));
  const bad = structuredClone(ability);
  bad.kind = "guessed";
  assert.throws(() => validateContract(bad, "ProposalAbilityDescription", pilotV3Schema));
  for (const choices of [{ modeIds: [256] }, { x: 65536 }, { playOptionId: 256 },
    { targetSelections: [{ slotId: 256, targets: [] }] }]) {
    assert.throws(() => validateContract({ action: { type: "CastSpell", card: 1, choices } }, "NativeProposal", pilotV3Schema));
  }
});
