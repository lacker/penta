import assert from "node:assert/strict";
import test from "node:test";

import {
  abilityOriginKey,
  actionHasTargets,
  buildAbilityActionGroups,
} from "../app/ability-actions.mjs";

const action = (index, abilityId, targets = {}) => ({
  index,
  label: "Activate Jace, Memory Adept",
  kind: "primary",
  cardId: 41,
  ability: { kind: "printed", definition: 700, partId: 0, abilityId },
  abilitySummary: [
    "+1: Draw a card. Target player mills a card.",
    "0: Target player mills ten cards.",
    "−7: Any number of target players each draw twenty cards.",
  ][abilityId],
  targetCount: 0,
  targetCardIds: [],
  targetPlayers: [],
  targetStackIds: [],
  targetSelections: [],
  sacrificeCardIds: [],
  bottomCardIds: [],
  ...targets,
});

test("Jace optional targets stay grouped with the exact -7 ability", () => {
  const zeroTargets = action(20, 2);
  const oneTarget = action(21, 2, {
    targetPlayer: "human",
    targetCount: 1,
    targetPlayers: ["human"],
  });
  const twoTargets = action(22, 2, {
    targetPlayer: "human",
    targetCount: 2,
    targetPlayers: ["human", "opponent"],
  });
  const actions = [
    action(10, 0, { targetPlayer: "opponent", targetCount: 1 }),
    action(11, 1, { targetPlayer: "opponent", targetCount: 1 }),
    zeroTargets,
    oneTarget,
    twoTargets,
  ];

  const groups = buildAbilityActionGroups(actions);
  assert.equal(groups.length, 3);
  const ultimate = groups.find((group) => group.key.endsWith(":2"));
  assert.ok(ultimate);
  assert.deepEqual(ultimate.targetless, [zeroTargets]);
  assert.deepEqual(ultimate.targeted, [oneTarget, twoTargets]);
  assert.equal(actionHasTargets(zeroTargets), false);
  assert.equal(actionHasTargets(twoTargets), true);
});

test("ability origin keys preserve definition, part, and grant provenance", () => {
  const printed = { kind: "printed", definition: 10, partId: 2, abilityId: 0 };
  assert.notEqual(
    abilityOriginKey(printed),
    abilityOriginKey({ ...printed, definition: 11 }),
  );
  assert.notEqual(
    abilityOriginKey(printed),
    abilityOriginKey({ ...printed, partId: 3 }),
  );
  assert.notEqual(
    abilityOriginKey({
      kind: "granted",
      source: 8,
      sourceDefinition: 10,
      sourcePartId: 0,
      sourceAbilityId: 1,
      grantId: 0,
    }),
    abilityOriginKey({
      kind: "granted",
      source: 8,
      sourceDefinition: 10,
      sourcePartId: 0,
      sourceAbilityId: 1,
      grantId: 1,
    }),
  );
});

test("inline token origin keys include the current source and token grant provenance", () => {
  const token = { kind: "token", partId: 0, abilityId: 1 };
  assert.notEqual(abilityOriginKey(token, 41), abilityOriginKey(token, 42));
  assert.notEqual(
    abilityOriginKey(token, 41),
    abilityOriginKey({ ...token, abilityId: 2 }, 41),
  );

  const tokenGranted = {
    kind: "tokenGranted",
    source: 40,
    sourcePartId: 0,
    sourceAbilityId: 1,
    grantId: 3,
  };
  assert.notEqual(
    abilityOriginKey(tokenGranted, 41),
    abilityOriginKey({ ...tokenGranted, grantId: 4 }, 41),
  );
  const tokenGrantedKey = abilityOriginKey(tokenGranted, 41);
  assert.ok(tokenGrantedKey);
  assert.ok(!tokenGrantedKey.includes("undefined"));

  const first = { ...action(30, 0), cardId: 41, ability: token };
  const sameSource = {
    ...action(31, 0, { targetPlayer: "opponent", targetCount: 1 }),
    cardId: 41,
    ability: token,
  };
  const otherSource = { ...action(32, 0), cardId: 42, ability: token };
  const groups = buildAbilityActionGroups([first, sameSource, otherSource]);
  assert.equal(groups.length, 2);
  assert.deepEqual(groups[0].actions, [first, sameSource]);
  assert.deepEqual(groups[1].actions, [otherSource]);
});

test("inline emblem origin keys include the current source and grant provenance", () => {
  const emblem = { kind: "emblem", abilityId: 1 };
  assert.notEqual(abilityOriginKey(emblem, 51), abilityOriginKey(emblem, 52));
  assert.notEqual(
    abilityOriginKey(emblem, 51),
    abilityOriginKey({ ...emblem, abilityId: 2 }, 51),
  );

  const emblemGranted = {
    kind: "emblemGranted",
    source: 50,
    sourceAbilityId: 1,
    grantId: 3,
  };
  assert.notEqual(
    abilityOriginKey(emblemGranted, 51),
    abilityOriginKey({ ...emblemGranted, grantId: 4 }, 51),
  );
  const emblemGrantedKey = abilityOriginKey(emblemGranted, 51);
  assert.ok(emblemGrantedKey);
  assert.ok(!emblemGrantedKey.includes("undefined"));
});

test("face-down origin keys include the current source and grant provenance", () => {
  const faceDown = { kind: "faceDown", abilityId: 1 };
  assert.notEqual(
    abilityOriginKey(faceDown, 61),
    abilityOriginKey(faceDown, 62),
  );
  assert.notEqual(
    abilityOriginKey(faceDown, 61),
    abilityOriginKey({ ...faceDown, abilityId: 2 }, 61),
  );

  const granted = {
    kind: "faceDownGranted",
    source: 60,
    sourceAbilityId: 1,
    grantId: 3,
  };
  assert.notEqual(
    abilityOriginKey(granted, 61),
    abilityOriginKey({ ...granted, grantId: 4 }, 61),
  );
  assert.ok(!abilityOriginKey(granted, 61).includes("undefined"));
});
