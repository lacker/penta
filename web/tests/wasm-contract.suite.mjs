import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

import { initializeWasm, WebGame } from "./wasm-test-support.mjs";
import { SessionClient } from "../../tools/penta-mcp/client.mjs";
import "./wasm-session-roundtrips.mjs";

test("session API controls either seat and advances only forced continuations in the browser match", async () => {
  await initializeWasm();
  const game = new WebGame("Sligh", "The Deck", "External", false, 42);
  game.enableSessionApi();
  try {
    const first = JSON.parse(game.sessionObserveJson("bot"));
    assert.equal(first.seat, "p1");
    assert.equal(JSON.parse(game.sessionObserveJson("human")).seat, "p2");
    assert.throws(() => game.sessionAct("human", 0), /does not hold/);
    game.sessionAct("bot", first.legalActions.find(action => action.type === "KeepHand").index);
    assert.equal(game.sessionDecisionRole(), "human");
    const human = JSON.parse(game.sessionObserveJson("human"));
    const keep = human.legalActions.find(action => action.type === "KeepHand");
    game.sessionAct("human", keep.index);
    const role = game.sessionDecisionRole();
    const before = game.sessionObserveJson(role);
    assert.equal(JSON.parse(before).forcedAction, null);
    assert.ok(Array.isArray(JSON.parse(before).updates));
    game.set_autopass(true);
    assert.equal(game.sessionObserveJson(role), before, "UI preferences never move an externally controlled seat");
    assert.equal(JSON.parse(game.state_json()).autopassEnabled, false);
    assert.equal(JSON.parse(game.state_json()).passLabel, null, "the browser delegates no optional passing choices");
    const replay = WebGame.fromReplayJson(game.replayJson());
    try { assert.equal(replay.sessionObserveJson(role), before); } finally { replay.free(); }
    assert.ok(JSON.parse(game.sessionCatalogJson()).cards.length > 0);
    assert.ok(JSON.parse(WebGame.sessionOptionsJson()).formats.find(format => format.id === "old-school-93-94").decks.includes("Sligh"));
  } finally { game.free(); }
});

test("session API decision-v1 tickets share browser commands, sideboarding, seat views, and durable replay", async () => {
  await initializeWasm();
  const { HostedGame } = await import("../app/wasm/penta_wasm.js");
  const support = await import("./game-room-support.mjs");
  const HttpResponse = globalThis.Response;
  support.installRoomGlobals({ WebGame, HostedGame });
  try {
    const { GameRoom } = await support.loadGameRoom();
    const storage = new support.MemoryStorage();
    let room = new GameRoom(support.durableState(storage));
    const call = async (route, token, body) => {
      const response = await room.fetch(support.request(route, { token, body }));
      const value = await response.json();
      assert.equal(response.status, 200, JSON.stringify(value));
      return value;
    };
    const opened = await call("start", undefined, { format: "old-school-93-94", humanDeck: "Sligh", botDeck: "The Deck",
      botPolicy: "external", humanFirst: false, seed: 42, sessionApi: true, matchMode: "first-to-two-wins" });
    const client = new SessionClient("http://localhost", async (url, init) => {
      const response = await room.fetch(new Request(url, init));
      return HttpResponse.json(await response.json(), { status: response.status });
    });
    const { connection } = await client.attach({ room: "test", token: opened.botToken, presentation: "decision-v1" });
    const choose = async (action, options) => {
      const presented = await client.next({ connection, waitMs: 0 });
      let choices = presented.choices;
      if (choices.reference) {
        const all = [];
        let offset = 0;
        while (offset !== null) {
          const page = await client.inspectReference({ reference: choices.reference, offset });
          all.push(...page.items); offset = page.nextOffset;
        }
        choices = all;
      } else if (!Array.isArray(choices)) choices = choices.rows.map(row => ({ ...choices.shared, ...row }));
      const ticket = choices.find(choice => choice.index === action.index).ticket;
      const result = await client.choose({ ticket, options, waitMs: 0 });
      assert.equal(result.receipt.accepted, 1);
      const commandCount = storage.values.get("hosted-game").commands.length;
      // A receipt survives eviction of the real hosted room/engine.
      room = new GameRoom(support.durableState(storage));
      await client.choose({ ticket, options, waitMs: 0 });
      assert.equal(storage.values.get("hosted-game").commands.length, commandCount);
    };
    let sideboarded = false;
    let complete = false;
    for (let step = 0; step < 100; step++) {
      let view = await call("session", opened.humanToken);
      let token = opened.humanToken;
      if (view.status === "waiting") { token = opened.botToken; view = await call("session", token); }
      assert.equal(view.observation.seat, token === opened.humanToken ? "p2" : "p1");
      if (view.status === "complete") { complete = true; break; }
      const decision = view.observation.decision;
      if (decision) {
        sideboarded ||= view.observation.match.stage === "sideboarding";
        const options = decision.options.slice(0, decision.minimum).map(option => option.id);
        if (token === opened.botToken) await choose(view.observation.legalActions.find(action => action.type === "ChooseDecision"), options);
        else await call("play", token, { revision: view.revision, requestId: `step-${step}`, choices: [{ decision: decision.id, options }] });
      } else {
        const keep = view.observation.legalActions.find(action => action.type === "KeepHand");
        if (token === opened.humanToken && !keep) {
          const browser = await call("state", token);
          const concede = browser.actions.find(action => /concede/i.test(action.label));
          assert.ok(concede);
          await call("command", token, { t: "act", index: concede.index, revision: browser.sessionRevision });
        } else {
          const action = keep ?? view.observation.legalActions.find(action => action.type === "PassPriority");
          assert.ok(action);
          if (token === opened.botToken) await choose(action);
          else await call("play", token, { revision: view.revision, requestId: `step-${step}`, choices: [{ index: action.index }] });
        }
      }
      // Evict and rebuild while both seats submit match and in-game decisions.
      const before = await call("session", opened.humanToken);
      room = new GameRoom(support.durableState(storage));
      assert.deepEqual(await call("session", opened.humanToken), before);
    }
    assert.ok(sideboarded);
    assert.ok(complete);
    const record = await call("record", opened.botToken);
    const replay = WebGame.fromReplayJson(JSON.stringify(record));
    try {
      for (const [role, token] of [["human", opened.humanToken], ["bot", opened.botToken]]) {
        assert.deepEqual(JSON.parse(replay.sessionObserveJson(role)), (await call("session", token)).observation);
      }
    } finally { replay.free(); }
  } finally { support.restoreRoomGlobals(); }
});

test("naming choices expose a pending notice and then the selected answer", async () => {
  await initializeWasm();
  const game = new WebGame(
    "Anderson Omnidoor Thragfire", "Braun-Duin Naya Midrange",
    "External", false, 0, "isd-m14-standard",
  );
  try {
    for (let step = 0; step < 30; step++) {
      const view = JSON.parse(game.opponentObserveJson());
      const cavern = view.hand.find(card => card.name === "Cavern of Souls");
      const play = view.legalActions.find(action => action.type === "PlayLand" && action.card === cavern?.objectId);
      if (play) { game.opponentAct(play.index); break; }
      const botAction = view.legalActions.find(action => action.type === "KeepHand")
        ?? view.legalActions.find(action => action.type === "PassPriority");
      if (botAction) { game.opponentAct(botAction.index); continue; }
      const state = JSON.parse(game.state_json());
      const humanAction = state.actions.find(action => action.label === "Keep this hand")
        ?? state.actions.find(action => action.kind === "pass");
      assert.ok(humanAction, "the setup can advance to Cavern");
      game.act(humanAction.index);
    }
    const waiting = JSON.parse(game.state_json()).decision;
    assert.equal(waiting.visibility, "PublicNotice");
    assert.equal(waiting.optionsVisible, false);
    assert.deepEqual(waiting.options, []);
    assert.equal(waiting.prompt, "Choose a creature type");
    const chooser = JSON.parse(game.opponentObserveJson()).decision;
    const angel = chooser.options.find(option => option.label === "Angel");
    assert.ok(angel, "the choosing bot receives its full menu");
    game.opponentChooseDecision(chooser.id, JSON.stringify([angel.id]));
    const after = JSON.parse(game.state_json());
    assert.equal(after.decision, null);
    assert.ok(after.battlefield.some(card => card.chosenCreatureType === "Angel"));
    assert.ok(after.opponentActions.some(action => action.label === "Angel"));
  } finally { game.free(); }
});

test("The Deck exposes colored costs and control rules to the browser", async () => {
  await initializeWasm();

  const game = new WebGame("The Deck", "The Deck", "Handcrafted", true, 3);
  const opening = JSON.parse(game.state_json());
  const swords = opening.human.hand.find(
    (card) => card.name === "Swords to Plowshares",
  );
  const serra = opening.human.hand.find((card) => card.name === "Serra Angel");
  assert.ok(swords);
  assert.equal(swords.manaCost.white, 1);
  assert.match(swords.rulesText, /exile target creature/i);
  assert.ok(serra);
  assert.equal(serra.manaCost.white, 2);
  assert.equal(serra.power, 4);
  assert.equal(serra.toughness, 4);

  game.free();
});

test("card status exposes only complete or unsupported definitions", async () => {
  await initializeWasm();

  const game = new WebGame(
    "Briksza Naya Midrange",
    "Greer G/R Aggro",
    "Handcrafted",
    true,
    2,
    "isd-m14-standard",
  );
  const opening = JSON.parse(game.state_json());
  const pilgrim = opening.human.hand.find(
    (card) => card.name === "Avacyn's Pilgrim",
  );
  const bonfire = opening.human.hand.find(
    (card) => card.name === "Bonfire of the Damned",
  );

  assert.ok(pilgrim);
  assert.equal(
    pilgrim.implementationStatus,
    "complete",
    "its creature body and printed mana ability are fully modeled despite the legacy play gate",
  );
  assert.ok(bonfire);
  assert.equal(bonfire.implementationStatus, "complete");
  assert.ok(
    opening.human.hand.every((card) => card.metadataOnly === undefined),
    "the browser bridge no longer projects the internal gate as coverage",
  );

  game.free();
});
test("mandatory library searches are serialized as generic private choices", async () => {
  await initializeWasm();

  const game = new WebGame("The Deck", "Goblins", "Random", true, 214);
  let state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Keep this hand").index);
  state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Cast Black Lotus").index);
  state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Cast Demonic Tutor").index);
  state = JSON.parse(game.state_json());

  assert.equal(state.decision.kind, "Choice");
  assert.equal(state.decision.orderSemantics, undefined);
  assert.equal(state.decision.visibility, "Private");
  // Demonic Tutor asks for an unrestricted card, so it must find one while the
  // library is nonempty. Qualified hidden-zone searches may still use zero.
  assert.equal(state.decision.minimum, 1);
  assert.equal(state.decision.maximum, 1);
  assert.equal(
    state.decision.cancellable,
    false,
    "the search resolves rather than backing out of the spell",
  );
  assert.ok(state.decision.options.length > 40);
  const choice = state.decision.options[0];
  game.choose_decision(state.decision.id, JSON.stringify([choice.id]));
  assert.equal(JSON.parse(game.state_json()).decision, null);

  game.free();
});

test("concurrent triggers expose resolution ordering and frozen stack ability metadata", async () => {
  await initializeWasm();

  const game = new WebGame("Artifacts", "Goblins", "Random", true, 183);
  let state = JSON.parse(game.state_json());
  const act = (label) => {
    const action = state.actions.find((candidate) => candidate.label.includes(label));
    assert.ok(action, `${label} is available`);
    game.act(action.index);
    state = JSON.parse(game.state_json());
  };

  act("Keep this hand");
  act("Cast Black Lotus");
  act("Cast Mox Sapphire");
  act("Black Lotus for Red");
  act("Mox Sapphire for Blue");
  act("Cast Ankh of Mishra");
  act("Cast Ankh of Mishra");
  act("Play Mountain");

  assert.equal(state.decision.kind, "TriggerOrder");
  assert.equal(state.decision.orderSemantics, "resolution");
  assert.equal(state.decision.minimum, 2);
  assert.equal(state.decision.maximum, 2);
  assert.equal(state.decision.options.length, 2);
  assert.equal(state.decision.options[0].triggerId, state.decision.options[0].id);
  assert.match(state.decision.options[0].abilityText, /Whenever a land enters/);

  const desiredResolutionOrder = state.decision.options.map((option) => option.id).reverse();
  const firstSource = state.decision.options[1].cardId;
  game.set_autopass(false);
  game.choose_decision(state.decision.id, JSON.stringify(desiredResolutionOrder));
  state = JSON.parse(game.state_json());

  assert.equal(state.stack.length, 2);
  assert.equal(state.stack[0].kind, "TriggeredAbility");
  assert.equal(state.stack[0].sourceId, firstSource);
  assert.equal(state.stack[0].abilityId, 0);
  assert.match(state.stack[0].abilityText, /Whenever a land enters/);
  assert.ok(
    state.stack.every((object) => object.kind === "TriggeredAbility"),
    "both objects remain visibly distinct triggered abilities",
  );

  game.free();
});

test("an unrestricted search cannot fail to find from a nonempty library", async () => {
  await initializeWasm();

  const game = new WebGame("The Deck", "Goblins", "Random", true, 214);
  let state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Keep this hand").index);
  state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Cast Black Lotus").index);
  state = JSON.parse(game.state_json());
  game.act(state.actions.find((action) => action.label === "Cast Demonic Tutor").index);
  state = JSON.parse(game.state_json());

  const handBeforeSearch = state.human.hand.length;
  const libraryBeforeSearch = state.human.library;

  assert.throws(
    () => game.choose_decision(state.decision.id, JSON.stringify([])),
    /is not legal/,
    "Demonic Tutor rejects an empty selection while a card is available",
  );
  state = JSON.parse(game.state_json());
  assert.equal(state.decision.minimum, 1, "the mandatory search remains pending");

  game.choose_decision(
    state.decision.id,
    JSON.stringify([state.decision.options[0].id]),
  );
  state = JSON.parse(game.state_json());

  assert.equal(state.decision, null, "the search resolved");
  assert.equal(state.human.hand.length, handBeforeSearch + 1, "one card was found");
  assert.equal(state.human.library, libraryBeforeSearch - 1, "one card left the library");

  game.free();
});

test("two mulligans identify each card in every bottom pair", async () => {
  await initializeWasm();

  const game = new WebGame("The Deck", "Sligh", "Handcrafted", true, 77);
  let state = JSON.parse(game.state_json());
  for (let mulligan = 0; mulligan < 2; mulligan += 1) {
    const takeMulligan = state.actions.find(
      (action) => action.label === "Take a mulligan",
    );
    assert.ok(takeMulligan);
    game.act(takeMulligan.index);
    state = JSON.parse(game.state_json());
    assert.equal(state.human.hand.length, 7);
  }

  const keep = state.actions.find((action) => action.label === "Keep this hand");
  assert.ok(keep);
  game.act(keep.index);
  state = JSON.parse(game.state_json());

  const bottomActions = state.actions.filter(
    (action) => action.bottomCardIds.length > 0,
  );
  assert.equal(bottomActions.length, 21);
  assert.ok(bottomActions.every((action) => action.bottomCardIds.length === 2));
  assert.deepEqual(
    [...new Set(bottomActions.flatMap((action) => action.bottomCardIds))].sort(
      (a, b) => a - b,
    ),
    state.human.hand.map((card) => card.id).sort((a, b) => a - b),
  );

  const chosen = bottomActions[0];
  game.act(chosen.index);
  state = JSON.parse(game.state_json());
  assert.equal(state.human.hand.length, 5);
  assert.ok(
    chosen.bottomCardIds.every(
      (cardId) => !state.human.hand.some((card) => card.id === cardId),
    ),
  );
  assert.equal(state.pregame, false);

  game.free();
});

test("opponent pregame choices do not block the game with animations", async () => {
  await initializeWasm();

  const game = new WebGame("The Deck", "Sligh", "Handcrafted", true, 0);
  const opening = JSON.parse(game.state_json());
  const keep = opening.actions.find((action) => action.label === "Keep this hand");
  game.act(keep.index);

  const afterKeep = JSON.parse(game.state_json());
  assert.ok(
    afterKeep.opponentActions.every(
      (action) =>
        action.label !== "Keep this hand" &&
        action.label !== "Take a mulligan" &&
        !action.label.startsWith("Bottom "),
    ),
    "keep, mulligan, and bottom choices stay out of the opponent animation queue",
  );

  game.free();
});

test("the Robots deck and its new card rules are packaged for the browser", async () => {
  await initializeWasm();

  const game = new WebGame(
    "Robots",
    "Robots",
    "Handcrafted",
    true,
    823380616,
  );
  const opening = JSON.parse(game.state_json());
  const juggernaut = opening.human.hand.find(
    (card) => card.name === "Juggernaut",
  );
  assert.ok(juggernaut, "the deterministic Robots hand includes Juggernaut");
  assert.equal(juggernaut.power, 5);
  assert.equal(juggernaut.toughness, 3);
  assert.match(juggernaut.rulesText, /attacks each combat if able/i);
  assert.match(juggernaut.rulesText, /can't be blocked by Walls/i);
  assert.equal(
    juggernaut.implementationStatus,
    "complete",
    "the body, the attack requirement, and the Wall restriction all execute",
  );

  game.free();
});

test("the packaged Rust engine plays through browser actions", async () => {
  await initializeWasm();

  const game = new WebGame("Goblins", "Sligh", "Handcrafted", true, 9394);
  const opening = JSON.parse(game.state_json());
  assert.equal(opening.human.hand.length, 7);
  assert.equal(opening.opponent.handSize, 7);
  assert.ok(
    opening.human.hand.every((card) => card.manaCost !== undefined),
    "cards expose their casting costs to the interface",
  );
  assert.ok(
    opening.human.hand.every(
      (card) =>
        typeof card.rulesText === "string" &&
        (card.kind.includes("land") || card.rulesText.length > 0),
    ),
    "cards expose rules text while ordinary lands may have an empty text box",
  );
  const openingCreature = opening.human.hand.find((card) =>
    card.kind.includes("creature"),
  );
  assert.ok(openingCreature, "the deterministic opening hand includes a creature");
  assert.equal(typeof openingCreature.power, "number");
  assert.equal(typeof openingCreature.toughness, "number");
  assert.ok(opening.actions.some((action) => action.label === "Keep this hand"));

  const keep = opening.actions.find((action) => action.label === "Keep this hand");
  game.act(keep.index);
  const afterKeep = JSON.parse(game.state_json());
  assert.equal(afterKeep.turn, 1);
  assert.equal(
    afterKeep.step,
    "Precombat Main",
    "the web facade passes through an uneventful opening upkeep",
  );
  assert.ok(Array.isArray(afterKeep.opponentActions));
  assert.ok(
    afterKeep.opponentActions.every((action) => action.label !== "Pass priority"),
    "routine opponent priority passes stay out of the animation queue",
  );
  assert.ok(
    afterKeep.opponentActions.every((action) => action.kind !== "mana"),
    "mana taps stay out of the standalone animation queue",
  );
  assert.ok(
    afterKeep.opponentActions.every(
      (action) =>
        action.state &&
        Array.isArray(action.state.battlefield) &&
        action.state.opponentActions.length === 0,
    ),
    "each opponent animation carries a non-recursive board snapshot",
  );
  assert.ok(
    afterKeep.actions.some(
      (action) => action.kind === "primary" || action.kind === "combat",
    ),
    "choice-free priority windows are passed automatically",
  );
  assert.ok(
    !afterKeep.actions.some((action) => action.label === "Keep this hand"),
  );
  assert.ok(
    afterKeep.events.every(
      (event) =>
        !event.includes("CardInstanceId") &&
        !event.includes("active_player") &&
        !event.includes("card #"),
    ),
    "the game log contains player-facing descriptions rather than engine diagnostics",
  );

  game.free();
});

test("every deck the picker offers is one the engine can build", async () => {
  await initializeWasm();

  const decksByFormat = {
    "old-school-93-94": [
      "Goblins", "Sligh", "Artifacts", "Robots", "The Deck", "Mono Black",
      "White Weenie", "Erhnamgeddon", "Counterburn", "Lions DIB",
      "Lion Dib Bolt", "BWR Aggro", "GR Aggro", "Troll Disk", "Jeskai Aggro",
    ],
    "isd-m14-standard": [
      "Briksza Naya Midrange", "Greer G/R Aggro", "Fyrberg B/G Midrange",
      "Smith Naya Midrange", "McDuffie U/W/R Flash", "Lorren U/W Flash",
      "Arch U/W Flash", "Kuenzinger Junk Reanimator",
      "Anderson Omnidoor Thragfire", "Braun-Duin Naya Midrange",
    ],
    premodern: [
      "Sligh",
      "RG Goblins",
      "GAT",
      "Replenish",
      "Landstill",
      "Stasis",
      "BW Control",
      "Angry Hermit",
    ],
  };

  for (const [format, names] of Object.entries(decksByFormat)) {
    for (const name of names) {
      const game = new WebGame(name, name, "Handcrafted", true, 1, format);
      const state = JSON.parse(game.state_json());
      assert.equal(state.format, format, `${name} uses the selected format`);
      assert.equal(state.human.hand.length, 7, `${name} deals an opening hand`);
      game.free();
    }
  }

  assert.throws(
    () => new WebGame("Goblins", "Goblins", "Handcrafted", true, 1, "isd-m14-standard"),
    /unknown deck for format/,
    "a deck from another format cannot leak into Standard",
  );
  assert.throws(
    () => new WebGame("Briksza Naya Midrange", "Briksza Naya Midrange", "Handcrafted", true, 1),
    /unknown deck for format/,
    "the compatibility default remains Old School",
  );
  assert.throws(
    () => new WebGame("Goblins", "Goblins", "Handcrafted", true, 1, "not-a-format"),
    /unknown format/,
  );
  assert.throws(
    () => new WebGame("Smith Naya Midrange", "Smith Naya Midrange", "Handcrafted", true, 1, "isd-dgm-standard"),
    /unknown format/,
    "the removed ISD-DGM profile is rejected",
  );
});

test("the Random setup choice is a placeholder, never a deck name", async () => {
  await initializeWasm();

  const source = await readFile(new URL("../app/game-config.ts", import.meta.url), "utf8");
  const sentinel = /export const randomDeck = "([^"]+)"/.exec(source)?.[1];
  assert.equal(sentinel, "Random");
  assert.equal(
    /export const defaultHumanDeck = (\w+)/.exec(source)?.[1],
    "randomDeck",
    "both seats default to Random",
  );
  assert.equal(/export const defaultBotDeck = (\w+)/.exec(source)?.[1], "randomDeck");

  assert.ok(
    !/^\s*Random:\s*"/m.test(source),
    "the sentinel is not one of the real decks, so it must be resolved before it reaches the engine",
  );
  assert.throws(
    () => new WebGame(sentinel, "Goblins", "Handcrafted", true, 1),
    /unknown deck/,
    "the engine rejects it, which is why the browser resolves it first",
  );

  const game = new WebGame("Goblins", "Goblins", "Handcrafted", true, 1);
  game.free();
});


test("external concession is explicit, seat safe, and replayable", async () => {
  await initializeWasm();
  const game = new WebGame("The Deck", "Goblins", "External", true, 23);
  let replay;
  try {
    const before = JSON.parse(game.opponentObserveJson());
    assert.deepEqual(before.publicReveals, []);
    assert.ok(before.legalActions.every(action => action.type !== "Concede"));
    game.opponentConcede();
    const record = game.replayJson();
    assert.ok(JSON.parse(record).commands.some(command => command.t === "botConcede"));
    replay = WebGame.fromReplayJson(record);
    assert.deepEqual(JSON.parse(replay.opponentObserveJson()), JSON.parse(game.opponentObserveJson()));
  } finally { replay?.free(); game.free(); }
});

test("persistent card references use natural printing keys", async () => {
  await initializeWasm();
  const game = new WebGame("The Deck", "The Deck", "External", true, 3);
  const uuid = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/;
  try {
    const observation = JSON.parse(game.opponentObserveJson());
    assert.ok(observation.hand.length > 0);
    for (const card of observation.hand) assert.match(card.definition, uuid);
    game.enable_match();
    const state = JSON.parse(game.state_json());
    assert.ok(state.match.main.length > 0);
    for (const card of [...state.match.main, ...state.match.sideboard]) {
      assert.match(card.id, uuid);
      assert.equal(typeof card.name, "string");
    }
  } finally { game.free(); }
});
