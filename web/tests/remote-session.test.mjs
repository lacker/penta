import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { afterEach, test } from "node:test";
import ts from "typescript";

const originals = { window: globalThis.window, fetch: globalThis.fetch, WebSocket: globalThis.WebSocket };
afterEach(() => Object.assign(globalThis, originals));

async function fixture(fragment = "") {
  const requests = [];
  const sockets = [];
  const saved = new Map();
  let url = new URL(`http://localhost/?hosted=room${fragment}`);
  globalThis.window = {
    get location() { return url; },
    sessionStorage: { getItem: key => saved.get(key) ?? null, setItem: (key, value) => saved.set(key, value) },
    history: { replaceState: (_state, _title, value) => { url = new URL(value); } },
  };
  globalThis.fetch = async (path, init) => {
    requests.push({ path, init });
    return Response.json(path.endsWith("/state") ? { sessionRevision: "rev", turn: 8 } : {
      humanToken: "new-human", botToken: "new-bot", state: { sessionRevision: "rev", turn: 0 },
    });
  };
  class Socket {
    static OPEN = 1;
    readyState = 1;
    sent = [];
    constructor(address) { this.url = address; sockets.push(this); }
    addEventListener(type, callback) { if (type === "open") queueMicrotask(callback); }
    send(value) { this.sent.push(JSON.parse(value)); }
    close() {}
  }
  globalThis.WebSocket = Socket;
  const source = await readFile(new URL("../app/remote-engine.ts", import.meta.url), "utf8");
  const js = ts.transpileModule(source, { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext } }).outputText;
  const { RemoteEngineGame } = await import(`data:text/javascript;base64,${Buffer.from(js).toString("base64")}`);
  const config = { gameId: "room", format: "old-school-93-94", humanDeck: "The Deck", botDeck: "Sligh",
    botPolicy: "external", humanFirst: true, seed: 1, onUpdate() {}, onError(error) { throw new Error(error); } };
  return { RemoteEngineGame, config, requests, sockets, saved };
}

test("session API browser invitation attaches without restarting and survives refresh", async () => {
  const { RemoteEngineGame, config, requests, sockets } = await fixture("#seatToken=invited-human");
  const first = await RemoteEngineGame.connect(config);
  assert.equal(requests[0].path, "/_game/room/state");
  assert.equal(requests[0].init.headers["x-penta-token"], "invited-human");
  assert.equal(window.location.hash, "");
  assert.equal(first.botToken, "");
  first.act(3);
  assert.deepEqual(sockets[0].sent[0], { t: "act", index: 3, revision: "rev" });
  first.free();
  const second = await RemoteEngineGame.connect(config);
  assert.equal(requests[1].path, "/_game/room/state");
  assert.equal(JSON.parse(second.state_json()).turn, 8);
  assert.equal(requests.some(request => request.path.endsWith("/start")), false);
});

test("session API browser-created rooms save credentials and refresh without redealing", async () => {
  const { RemoteEngineGame, config, requests } = await fixture();
  const first = await RemoteEngineGame.connect(config);
  assert.equal(first.botToken, "new-bot");
  assert.equal(requests[0].path, "/_game/room/start");
  first.free();
  await RemoteEngineGame.connect(config);
  assert.equal(requests[1].path, "/_game/room/state");
});
