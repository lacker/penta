import assert from "node:assert/strict";
import test from "node:test";

import {
  actionMatchesTargetedOrigin,
  abilityOriginKey,
  groupTargetedActionsByOrigin,
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
