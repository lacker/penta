/**
 * Runs `GameRoom` under `node --test`.
 *
 * The room is a Durable Object: it expects Workers globals, loads the engine
 * as WASM, and reaches other objects through bindings. None of that exists
 * here, so the source is transpiled with its worker-only imports swapped for
 * injected globals. Loading the real source is the point -- a hand-copied
 * room would pass its tests while the deployed one failed.
 *
 * Each suite brings its own `WebGame` stub, because what a room does is only
 * observable through what it asks the engine. Everything else is here, once:
 * four suites previously carried their own copy of this loader, and each copy
 * named the room's imports literally, so adding one import to `game-room.ts`
 * broke three test files that had nothing to do with the change. The
 * replacements below match whatever the room actually imports.
 */

import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";

import ts from "typescript";

import * as botPresence from "../worker/bot-presence.mjs";

export class MemoryStorage {
  values = new Map();
  alarm = null;

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

  async setAlarm(time) {
    this.alarm = time;
  }

  async deleteAlarm() {
    this.alarm = null;
  }
}

export class TestResponse {
  constructor(body = null, init = {}) {
    this.status = init.status ?? 200;
    this.webSocket = init.webSocket;
    this.body = body === null ? "" : String(body);
  }

  static json(value, init = {}) {
    return new TestResponse(JSON.stringify(value), init);
  }

  async json() {
    return JSON.parse(this.body);
  }
}

export class RoomSocket {
  sent = [];
  handlers = new Map();

  accept() {}

  send(message) {
    this.sent.push(message);
  }

  close() {}

  addEventListener(type, handler) {
    this.handlers.set(type, handler);
  }
}

/** Every pair the room has opened, in order, for a suite that drives sockets. */
export const socketPairs = [];

class TestWebSocketPair {
  constructor() {
    const pair = { 0: new RoomSocket(), 1: new RoomSocket() };
    socketPairs.push(pair);
    return pair;
  }
}

export function durableState(storage = new MemoryStorage()) {
  return { storage, blockConcurrencyWhile: (callback) => callback() };
}

export function request(route, { token, body } = {}) {
  return new Request(`https://room.test/${route}`, {
    method: body === undefined ? "GET" : "POST",
    headers: token ? { "x-penta-token": token } : undefined,
    body: body === undefined ? undefined : JSON.stringify(body),
  });
}

/** The version numbers a room checks before it will rebuild a stored game. */
export class TestHostedGame {
  static replayVersion() {
    return 1;
  }

  static simulationFingerprint() {
    return "test-fingerprint";
  }

  static engineVersion() {
    return "test-engine";
  }

  static protocolVersion() {
    return 1;
  }
}

const originalResponse = globalThis.Response;
const originalWebSocketPair = globalThis.WebSocketPair;

/**
 * Installs the globals the transpiled room closes over.
 *
 * `presence` overrides only the timing rules a suite needs to control; every
 * other rule is the real one, so a test cannot quietly diverge from what the
 * deployment does by forgetting to supply a value.
 *
 * @param {{ WebGame: unknown, HostedGame?: unknown,
 *           presence?: Record<string, unknown>,
 *           replayCompatibilityError?: () => string | null }} options
 */
export function installRoomGlobals({
  WebGame,
  HostedGame = TestHostedGame,
  presence = {},
  replayCompatibilityError = () => null,
}) {
  // Most room tests model only the engine methods relevant to their scenario.
  // Give those small fakes the current game-construction entry point while
  // preserving a suite's explicit implementation when it wants to inspect
  // the art preference itself.
  WebGame.withArtPreference ??= (...args) => new WebGame(...args);
  globalThis.Response = TestResponse;
  globalThis.WebSocketPair = TestWebSocketPair;
  globalThis.__gameRoomEngine = async () => ({ WebGame, HostedGame });
  globalThis.__replayCompatibilityError = replayCompatibilityError;
  globalThis.__botPresence = { ...botPresence, ...presence };
}

export function restoreRoomGlobals() {
  globalThis.Response = originalResponse;
  globalThis.WebSocketPair = originalWebSocketPair;
  socketPairs.length = 0;
  delete globalThis.__gameRoomEngine;
  delete globalThis.__replayCompatibilityError;
  delete globalThis.__botPresence;
}

export async function loadGameRoom() {
  let source = await readFile(new URL("../worker/game-room.ts", import.meta.url), "utf8");
  const replacements = [
    [
      /^import \{ type EngineModule, engine \} from "\.\/engine";$/m,
      "const engine = globalThis.__gameRoomEngine;",
    ],
    [
      /^import \{ replayCompatibilityError \} from "\.\/replay-compatibility\.mjs";$/m,
      "const replayCompatibilityError = globalThis.__replayCompatibilityError;",
    ],
    // Whatever the room imports from the presence rules, by name, so that
    // adding one does not break every suite that never mentions it.
    [
      /^import \{([^}]+)\} from "\.\/bot-presence\.mjs";$/m,
      (_match, names) => `const {${names}} = globalThis.__botPresence;`,
    ],
  ];
  for (const [pattern, replacement] of replacements) {
    assert.match(source, pattern, `test loader expected a match for ${pattern}`);
    source = source.replace(pattern, replacement);
  }
  const javascript = ts.transpileModule(source, {
    compilerOptions: { module: ts.ModuleKind.ESNext, target: ts.ScriptTarget.ES2022 },
  }).outputText;
  const resolved = javascript.replace(/from "(\.\/[^"\n]+\.mjs)"/g, (_match, path) => `from "${new URL(`../worker/${path.slice(2)}`, import.meta.url).href}"`);
  const encoded = Buffer.from(resolved).toString("base64");
  return import(`data:text/javascript;base64,${encoded}`);
}
