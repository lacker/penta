import assert from "node:assert/strict";
import test from "node:test";

import {
  battlefieldWithObservedUntap,
  cardChoiceLabel,
  cardChoiceStateKey,
  cardPileStateKey,
  duplicatePermanentMarkers,
} from "../app/card-visible-state.mjs";

test("a chosen card name is visible and separates otherwise identical piles", () => {
  const jace = { chosenCardName: "Jace, Memory Adept" };
  const domri = { chosenCardName: "Domri Rade" };

  assert.equal(cardChoiceLabel(jace), "Named card: Jace, Memory Adept");
  assert.notEqual(cardChoiceStateKey(jace), cardChoiceStateKey(domri));
});

test("chosen creature types remain visible and part of pile identity", () => {
  const human = { chosenCreatureType: "Human" };
  const angel = { chosenCreatureType: "Angel" };

  assert.equal(cardChoiceLabel(human), "Chosen type: Human");
  assert.notEqual(cardChoiceStateKey(human), cardChoiceStateKey(angel));
});

test("a pile never combines permanents with different public or individual state", () => {
  const ready = {
    id: 10,
    name: "Mana Vault",
    kind: "artifact",
    tapped: false,
    token: false,
  };

  assert.notEqual(cardPileStateKey(ready), cardPileStateKey({ ...ready, tapped: true }));
  assert.notEqual(cardPileStateKey(ready), cardPileStateKey({ ...ready, token: true }));
  assert.notEqual(
    cardPileStateKey({ ...ready, hasIndividualState: true }),
    cardPileStateKey({ ...ready, id: 11, hasIndividualState: true }),
  );
});

test("the turn banner previews the observed untap result instead of inventing one", () => {
  const held = [
    { id: 10, name: "Mana Vault", tapped: true },
    { id: 11, name: "Mana Vault", tapped: false },
    { id: 12, name: "Mox Sapphire", tapped: true },
  ];
  const incoming = [
    { ...held[0], tapped: true },
    { ...held[1], tapped: false },
    { ...held[2], tapped: false },
  ];

  assert.deepEqual(
    battlefieldWithObservedUntap(held, incoming).map((card) => card.tapped),
    [true, false, false],
  );
});

test("same-name permanents receive stable per-player markers", () => {
  const markers = duplicatePermanentMarkers([
    { id: 10, owner: "human", name: "Mana Vault" },
    { id: 11, owner: "human", name: "Mana Vault" },
    { id: 12, owner: "opponent", name: "Mana Vault" },
  ]);

  assert.equal(markers.get(10), "1");
  assert.equal(markers.get(11), "2");
  assert.equal(markers.has(12), false);
});
