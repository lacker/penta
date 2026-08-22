import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test, { after } from "node:test";

import ts from "typescript";

import {
  MAX_BOTS,
  PRESENCE_MS,
  isOnline,
  liveInvites,
  publicBot,
  worthKeeping,
} from "../worker/bot-presence.mjs";
import {
  incompatibility,
  incompatibilityBody,
  parseBotCompatibility,
  parseServerCompatibility,
  publicServerCompatibility,
} from "../worker/bot-compatibility.mjs";

/**
 * Open decklists on the registry side: `discloseDeck` is stored on a bot's
 * registration, echoed back by register/heartbeat, and -- only once a
 * challenge actually succeeds -- forwarded to the room that bot just
 * claimed, via `disclose-bot-deck`. A bot that never opts in behaves exactly
 * as before, and the room is still told, but with `discloseDeck: false`,
 * which `game-room.ts` treats the same as never having been told at all.
 *
 * This harness mirrors the transpile-from-source approach in
 * `game-room-cache.test.mjs`: `BotRegistry` is loaded with its worker-only
 * imports swapped for injected globals, so it runs under `node --test`
 * without a Workers runtime.
 */

class MemoryStorage {
  values = new Map();
  alarms = [];

  async get(key) {
    const value = this.values.get(key);
    return value === undefined ? undefined : structuredClone(value);
  }

  async put(key, value) {
    this.values.set(key, structuredClone(value));
  }

  async delete(key) {
    return this.values.delete(key);
  }

  async list({ prefix }) {
    const result = new Map();
    for (const [key, value] of this.values) {
      if (key.startsWith(prefix)) result.set(key, structuredClone(value));
    }
    return result;
  }

  async setAlarm(time) {
    this.alarms.push(time);
  }
}

class TestResponse {
  constructor(body = null, init = {}) {
    this.status = init.status ?? 200;
    this.body = body === null ? "" : String(body);
  }

  static json(value, init = {}) {
    return new TestResponse(JSON.stringify(value), init);
  }

  async json() {
    return JSON.parse(this.body);
  }
}

const SERVER_COMPATIBILITY = {
  protocolVersion: 1,
  capabilities: [],
  requiredCapabilities: [],
  simulationFingerprint: "test-fingerprint",
  legacyUndeclaredProtocolVersion: 1,
};

class TestHostedGame {
  static botCompatibilityJson() {
    return JSON.stringify(SERVER_COMPATIBILITY);
  }
}

/** A fake `GAME_ROOMS` room that records every call it receives and accepts any bot token. */
function fakeRoom() {
  const calls = [];
  return {
    calls,
    async fetch(request) {
      const url = new URL(request.url);
      const route = url.pathname.split("/").pop();
      const body = request.method === "POST" ? await request.json() : undefined;
      calls.push({ route, body });
      return { ok: true, json: async () => ({ ok: true }) };
    },
  };
}

function fakeEnv(room) {
  return {
    GAME_ROOMS: {
      idFromName: (name) => name,
      get: () => room,
    },
  };
}

const originalResponse = globalThis.Response;
globalThis.Response = TestResponse;
globalThis.__botPresence = {
  MAX_BOTS,
  PRESENCE_MS,
  isOnline,
  liveInvites,
  publicBot,
  worthKeeping,
};
globalThis.__botCompatibility = {
  incompatibility,
  incompatibilityBody,
  parseBotCompatibility,
  parseServerCompatibility,
  publicServerCompatibility,
};
globalThis.__botRegistryEngine = async () => ({ HostedGame: TestHostedGame });

after(() => {
  globalThis.Response = originalResponse;
  delete globalThis.__botPresence;
  delete globalThis.__botCompatibility;
  delete globalThis.__botRegistryEngine;
});

async function loadBotRegistry() {
  let source = await readFile(
    new URL("../worker/bot-registry.ts", import.meta.url),
    "utf8",
  );
  const replacements = [
    [
      'import {\n  MAX_BOTS,\n  PRESENCE_MS,\n  isOnline,\n  liveInvites,\n  publicBot,\n  worthKeeping,\n} from "./bot-presence.mjs";',
      "const {\n  MAX_BOTS,\n  PRESENCE_MS,\n  isOnline,\n  liveInvites,\n  publicBot,\n  worthKeeping,\n} = globalThis.__botPresence;",
    ],
    [
      'import {\n  incompatibility,\n  incompatibilityBody,\n  parseBotCompatibility,\n  parseServerCompatibility,\n  publicServerCompatibility,\n} from "./bot-compatibility.mjs";',
      "const {\n  incompatibility,\n  incompatibilityBody,\n  parseBotCompatibility,\n  parseServerCompatibility,\n  publicServerCompatibility,\n} = globalThis.__botCompatibility;",
    ],
    [
      'import { engine } from "./engine";',
      "const engine = globalThis.__botRegistryEngine;",
    ],
  ];
  for (const [from, to] of replacements) {
    assert.ok(source.includes(from), `test loader expected ${from}`);
    source = source.replace(from, to);
  }
  const javascript = ts.transpileModule(source, {
    compilerOptions: {
      module: ts.ModuleKind.ESNext,
      target: ts.ScriptTarget.ES2022,
    },
  }).outputText;
  const encoded = Buffer.from(javascript).toString("base64");
  return import(`data:text/javascript;base64,${encoded}`);
}

function durableState() {
  return { storage: new MemoryStorage() };
}

function request(route, { body } = {}) {
  return new Request(`https://registry.test/_bots/${route}`, {
    method: body === undefined ? "GET" : "POST",
    headers: body === undefined ? undefined : { "content-type": "application/json" },
    body: body === undefined ? undefined : JSON.stringify(body),
  });
}

async function register(registry, overrides = {}) {
  const response = await registry.fetch(
    request("register", { body: { name: "Fizzbot", deck: "Sligh", ...overrides } }),
  );
  return response.json();
}

async function heartbeat(registry, id, overrides = {}) {
  const response = await registry.fetch(
    request(`${id}/heartbeat`, { body: { done: [], ...overrides } }),
  );
  return response.json();
}

test("a bot that never opts in registers and heartbeats exactly as before", async () => {
  const { BotRegistry } = await loadBotRegistry();
  const registry = new BotRegistry(durableState(), fakeEnv(fakeRoom()));
  const registered = await register(registry);
  assert.equal(registered.discloseDeck, false);
  const beat = await heartbeat(registry, registered.id, { token: registered.token });
  assert.equal(beat.discloseDeck, false);
});

test("a bot can opt in at registration, and it is echoed back", async () => {
  const { BotRegistry } = await loadBotRegistry();
  const registry = new BotRegistry(durableState(), fakeEnv(fakeRoom()));
  const registered = await register(registry, { discloseDeck: true });
  assert.equal(registered.discloseDeck, true);
});

test("a heartbeat that omits discloseDeck leaves the prior declaration alone", async () => {
  const { BotRegistry } = await loadBotRegistry();
  const registry = new BotRegistry(durableState(), fakeEnv(fakeRoom()));
  const registered = await register(registry, { discloseDeck: true });
  const beat = await heartbeat(registry, registered.id, { token: registered.token });
  assert.equal(beat.discloseDeck, true, "omitting the field is not the same as turning it off");
});

test("a heartbeat can turn the opt-in on or back off", async () => {
  const { BotRegistry } = await loadBotRegistry();
  const registry = new BotRegistry(durableState(), fakeEnv(fakeRoom()));
  const registered = await register(registry);
  const on = await heartbeat(registry, registered.id, {
    token: registered.token,
    discloseDeck: true,
  });
  assert.equal(on.discloseDeck, true);
  const off = await heartbeat(registry, registered.id, {
    token: registered.token,
    discloseDeck: false,
  });
  assert.equal(off.discloseDeck, false);
});

test("a successful challenge tells the room the claiming bot's opt-in", async () => {
  const { BotRegistry } = await loadBotRegistry();
  const room = fakeRoom();
  const registry = new BotRegistry(durableState(), fakeEnv(room));
  const registered = await register(registry, { discloseDeck: true });
  await heartbeat(registry, registered.id, { token: registered.token });
  const response = await registry.fetch(
    request(`${registered.id}/challenge`, {
      body: { room: "room-1", token: "bot-seat-token" },
    }),
  );
  const challenge = await response.json();
  assert.equal(challenge.discloseDeck, true);
  const disclosure = room.calls.find((call) => call.route === "disclose-bot-deck");
  assert.ok(disclosure, "the registry must tell the room about the opt-in");
  assert.deepEqual(disclosure.body, { discloseDeck: true });
});

test("a bot that never opted in still gets a disclose-bot-deck call, declaring false", async () => {
  const { BotRegistry } = await loadBotRegistry();
  const room = fakeRoom();
  const registry = new BotRegistry(durableState(), fakeEnv(room));
  const registered = await register(registry);
  await heartbeat(registry, registered.id, { token: registered.token });
  await registry.fetch(
    request(`${registered.id}/challenge`, {
      body: { room: "room-1", token: "bot-seat-token" },
    }),
  );
  const disclosure = room.calls.find((call) => call.route === "disclose-bot-deck");
  assert.ok(disclosure);
  assert.deepEqual(disclosure.body, { discloseDeck: false });
});
