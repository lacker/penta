import assert from "node:assert/strict";
import test from "node:test";

import {
  actionMatchesTargetedOrigin,
  abilityOriginKey,
  groupTargetedActionsByOrigin,
  targetActionsAreUnambiguous,
  targetedActionOriginKey,
} from "../app/targeted-action-groups.mjs";

const action = (index, ability, targetCardId, abilityLabel) => ({
  index,
  label: `Activate source → target ${targetCardId}`,
  abilityLabel,
  ability,
  cardId: 9,
  targetCardId,
  kind: "primary",
  bottomCardIds: [],
});

test("targeted actions group by the complete printed ability origin", () => {
  const firstOrigin = { kind: "printed", definition: 17, partId: 0, abilityId: 1 };
  const secondOrigin = { kind: "printed", definition: 17, partId: 0, abilityId: 2 };
  const actions = [
    action(0, firstOrigin, 40, "Remove a counter: deal 1 damage"),
    action(1, firstOrigin, 41, "Remove a counter: deal 1 damage"),
    action(2, secondOrigin, 40, "Tap: destroy target creature"),
  ];

  const groups = groupTargetedActionsByOrigin(actions);

  assert.equal(groups.length, 2);
  assert.deepEqual(groups[0].actions, actions.slice(0, 2));
  assert.deepEqual(groups[1].actions, actions.slice(2));
  assert.equal(groups[0].label, "Remove a counter: deal 1 damage");
  assert.equal(groups[1].label, "Tap: destroy target creature");
  assert.notEqual(groups[0].key, groups[1].key);
  assert.equal(actionMatchesTargetedOrigin(actions[0], 9, groups[0].key), true);
  assert.equal(actionMatchesTargetedOrigin(actions[2], 9, groups[0].key), false);
  assert.equal(
    actionMatchesTargetedOrigin({ ...actions[2], cardId: 10 }, 9, groups[0].key),
    true,
    "the source-scoped filter must not hide unrelated panel actions",
  );
});

test("granted origins include every identity field", () => {
  const base = {
    kind: "granted",
    source: 8,
    sourceDefinition: 21,
    sourcePartId: 0,
    sourceAbilityId: 1,
    grantId: 3,
  };
  const same = { ...base };
  const differentGrant = { ...base, grantId: 4 };

  assert.equal(abilityOriginKey(base), abilityOriginKey(same));
  assert.notEqual(abilityOriginKey(base), abilityOriginKey(differentGrant));
});

test("inline token origins remain source-scoped without fake definitions", () => {
  const token = { kind: "token", partId: 0, abilityId: 1 };
  const first = action(0, token, 40, "Token ability");
  const second = action(1, token, 41, "Token ability");
  const otherSource = { ...action(2, token, 42, "Token ability"), cardId: 10 };
  const groups = groupTargetedActionsByOrigin([first, second, otherSource]);

  assert.equal(groups.length, 2);
  assert.deepEqual(groups[0].actions, [first, second]);
  assert.deepEqual(groups[1].actions, [otherSource]);
  assert.match(groups[0].key, /^object:9:token:/);
  assert.match(groups[1].key, /^object:10:token:/);

  const tokenGranted = {
    kind: "tokenGranted",
    source: 8,
    sourcePartId: 0,
    sourceAbilityId: 1,
    grantId: 3,
  };
  assert.notEqual(
    abilityOriginKey(tokenGranted, 9),
    abilityOriginKey({ ...tokenGranted, grantId: 4 }, 9),
  );
});

test("inline emblem origins remain source-scoped without fake definitions", () => {
  const emblem = { kind: "emblem", abilityId: 1 };
  const first = action(0, emblem, 40, "Emblem ability");
  const second = action(1, emblem, 41, "Emblem ability");
  const otherSource = { ...action(2, emblem, 42, "Emblem ability"), cardId: 10 };
  const groups = groupTargetedActionsByOrigin([first, second, otherSource]);

  assert.equal(groups.length, 2);
  assert.deepEqual(groups[0].actions, [first, second]);
  assert.deepEqual(groups[1].actions, [otherSource]);
  assert.match(groups[0].key, /^object:9:emblem:/);
  assert.match(groups[1].key, /^object:10:emblem:/);

  const emblemGranted = {
    kind: "emblemGranted",
    source: 8,
    sourceAbilityId: 1,
    grantId: 3,
  };
  assert.notEqual(
    abilityOriginKey(emblemGranted, 9),
    abilityOriginKey({ ...emblemGranted, grantId: 4 }, 9),
  );
});

test("face-down origins remain source-scoped without fake definitions", () => {
  const faceDown = { kind: "faceDown", abilityId: 1 };
  const first = action(0, faceDown, 40, "Ward {2}");
  const second = action(1, faceDown, 41, "Ward {2}");
  const otherSource = {
    ...action(2, faceDown, 42, "Ward {2}"),
    cardId: 10,
  };
  const groups = groupTargetedActionsByOrigin([first, second, otherSource]);

  assert.equal(groups.length, 2);
  assert.deepEqual(groups[0].actions, [first, second]);
  assert.deepEqual(groups[1].actions, [otherSource]);
  assert.match(groups[0].key, /^object:9:face-down:/);
  assert.match(groups[1].key, /^object:10:face-down:/);

  const granted = {
    kind: "faceDownGranted",
    source: 8,
    sourceAbilityId: 1,
    grantId: 3,
  };
  assert.notEqual(
    abilityOriginKey(granted, 9),
    abilityOriginKey({ ...granted, grantId: 4 }, 9),
  );
});

test("actions without an ability origin retain the simple single group flow", () => {
  const actions = [
    action(0, null, 40, null),
    action(1, undefined, 41, undefined),
  ];
  const groups = groupTargetedActionsByOrigin(actions);

  assert.equal(groups.length, 1);
  assert.equal(groups[0].key, targetedActionOriginKey(actions[0]));
  assert.deepEqual(groups[0].actions, actions);
  assert.equal(groups[0].label, "Choose a target");
});

test("drag targeting rejects distinct choices that share one target", () => {
  const payMana = action(0, null, 40, null);
  const payLife = {
    ...action(1, null, 40, null),
    label: "Cast Gut Shot (pay 2 life for 1 R/P) → target 40",
  };
  const targetKeys = (candidate) => [`card:${candidate.targetCardId}`];

  assert.equal(
    targetActionsAreUnambiguous([payMana, payLife], targetKeys),
    false,
    "a drop must not silently select one mana-payment action",
  );
  assert.equal(
    targetActionsAreUnambiguous(
      [payMana, action(2, null, 41, null)],
      targetKeys,
    ),
    true,
    "one action per target remains safe to drag",
  );
});
