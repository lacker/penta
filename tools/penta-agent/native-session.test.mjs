import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { Readable, Writable } from "node:stream";
import test from "node:test";
import { MAX_SESSION_LINE_BYTES, runNativeSession } from "./native-session.mjs";

const connection = { server: "http://127.0.0.1:41234", secret: "private-seat-secret", presentation: "exact" };
async function session(chunks, command) {
  let text = "";
  const output = new Writable({ write(chunk, _encoding, done) { text += chunk; done(); } });
  await runNativeSession(Readable.from(chunks), output, command);
  return { text, replies: text.trim().split("\n").filter(Boolean).map(line => JSON.parse(line)) };
}
const initialization = `${JSON.stringify(connection)}\n`;

test("session initializes without network access, dispatches serially, and drains final line on EOF", async () => {
  const calls = [];
  let active = false;
  const result = await session([initialization, '{"method":"observe"}\n{"method":"wait","waitMs":1}'], async (method, args) => {
    assert.equal(active, false);
    active = true;
    await new Promise(resolve => setImmediate(resolve));
    calls.push({ method, args });
    active = false;
    return { method };
  });
  assert.deepEqual(result.replies, [{ status: "initialized" }, { method: "observe" }, { method: "wait" }]);
  assert.deepEqual(calls, [{ method: "observe", args: connection }, { method: "wait", args: { ...connection, waitMs: 1 } }]);
  assert.ok(!result.text.includes(connection.secret));
});

test("session keeps running after parse, validation and command errors without printing secrets", async () => {
  const result = await session([initialization,
    `bad JSON ${connection.secret}\n{}\n{"method":"wait","secret":"replacement"}\n{"method":"choose"}\n{"method":"observe"}\n`,
  ], async method => {
    if (method === "choose") throw new Error(`failed for ${connection.secret}`);
    return { status: "ready", nested: [connection.secret] };
  });
  assert.deepEqual(result.replies, [
    { status: "initialized" }, { error: "invalid JSON line" },
    { error: "method must be a nonempty string" }, { error: "session requests must omit server and secret" },
    { error: "failed for [redacted]" }, { status: "ready", nested: ["[redacted]"] },
  ]);
  assert.ok(!result.text.includes(connection.secret));
});

test("session rejects invalid startup once and empty EOF exits without requests", async () => {
  const command = () => assert.fail("must not dispatch");
  assert.deepEqual((await session([], command)).replies, []);
  assert.deepEqual((await session(['{"server":"bad","secret":"sensitive"}\n{}\n'], command)).replies,
    [{ error: "server must be an HTTP origin without credentials" }]);
});

test("oversized lines are bounded, discarded through newline, and do not kill initialized sessions", async () => {
  const result = await session([initialization, "x".repeat(MAX_SESSION_LINE_BYTES), "x\n", '{"method":"observe"}\n'],
    async () => ({ status: "ready" }));
  assert.deepEqual(result.replies, [{ status: "initialized" },
    { error: `JSON line exceeds ${MAX_SESSION_LINE_BYTES} bytes` }, { status: "ready" }]);
});

test("sessionStats measures dispatches, errors and actual output bytes without host requests", async () => {
  const calls = [];
  const result = await session([initialization, '{"method":"observe"}\n{"method":"choose"}\nnot json\n{"method":"sessionStats"}\n'],
    async method => {
      calls.push(method);
      if (method === "choose") throw new Error("missing requestId");
      return { status: "ready", text: "é" };
    });
  assert.deepEqual(calls, ["observe", "choose"]);
  const stats = result.replies.at(-1);
  assert.deepEqual(stats.callsByMethod, { observe: 1, choose: 1 });
  assert.equal(stats.errors, 2);
  assert.equal(stats.outputBytes, Buffer.byteLength(result.text.split("\n").slice(0, -2).join("\n") + "\n"));
  assert.ok(Number.isInteger(stats.elapsedMs) && stats.elapsedMs >= 0);
});

test("full is local, resets printed knowledge across waiting, and actual compressed bytes count in stats", async () => {
  const printed = { rulesText: "A sufficiently long printed rule repeated across observations." };
  const value = { presentation: "pilot-v2", status: "ready", position: {
    rules: { items: [{ definition: "d", available: true, printed }], total: 1, reference: null },
  } };
  const calls = [];
  const result = await session([initialization,
    '{"method":"observe"}\n{"method":"observe","full":false}\n{"method":"wait","full":true}\n{"method":"observe"}\n{"method":"sessionStats"}\n',
  ], async (method, args) => {
    calls.push({ method, args });
    return method === "wait" ? { status: "waiting", position: null } : value;
  });
  assert.deepEqual(result.replies[1], value);
  assert.deepEqual(result.replies[2].position.rules.items[0], { definition: "d", printed: null, printedPreviouslySent: true });
  assert.deepEqual(result.replies[4], value);
  assert.ok(calls.every(call => !Object.hasOwn(call.args, "full")));
  const lines = result.text.trimEnd().split("\n");
  assert.equal(result.replies[5].outputBytes, Buffer.byteLength(lines.slice(0, -1).join("\n") + "\n"));
});

test("invalid full is rejected locally and the next request still runs", async () => {
  const calls = [];
  const result = await session([initialization, '{"method":"observe","full":"yes"}\n{"method":"observe"}\n'],
    async method => { calls.push(method); return { status: "ready" }; });
  assert.deepEqual(calls, ["observe"]);
  assert.equal(result.replies[1].error, "full must be a boolean");
  assert.equal(result.replies[2].status, "ready");
});

test("CLI session reads stdin and exits cleanly at EOF with JSON-only stdout", async () => {
  const child = spawn(process.execPath, [new URL("./native.mjs", import.meta.url).pathname, "session"], { stdio: "pipe" });
  let stdout = "";
  let stderr = "";
  child.stdout.on("data", chunk => { stdout += chunk; });
  child.stderr.on("data", chunk => { stderr += chunk; });
  const exited = new Promise((resolve, reject) => {
    child.on("error", reject);
    child.on("exit", code => resolve(code));
  });
  child.stdin.end(`${initialization}{"method":"choose"}\n{"method":"unknown"}\n`);
  assert.equal(await exited, 0);
  assert.equal(stderr, "");
  const replies = stdout.trim().split("\n").map(line => JSON.parse(line));
  assert.equal(replies[0].status, "initialized");
  assert.match(replies[1].error, /requestId/);
  assert.match(replies[2].error, /unknown native command/);
  assert.ok(!stdout.includes(connection.secret));
});


test("next renews transport waits without exposing routine waiting responses", async () => {
  const calls = [];
  const result = await session([initialization, '{"method":"next"}\n{"method":"sessionStats"}\n'], async (method, args) => {
    calls.push({ method, args });
    return { status: calls.length === 3 ? "ready" : "waiting" };
  });
  assert.deepEqual(calls.map(call => call.method), ["observe", "wait", "wait"]);
  assert.equal(result.replies[1].status, "ready");
  assert.equal(result.replies[2].waitRenewals, 2);
  assert.ok(calls.slice(1).every(call => call.args.waitMs <= 25_000));
});

test("interact submits once, retains original receipt across waits, and strips local budget", async () => {
  const receipt = { requestId: "m", accepted: 1, yieldAtAcceptance: { active: true } };
  const calls = [];
  const result = await session([initialization, '{"method":"interact","requestId":"m","revision":"r","choices":[],"maxWaitMs":60000}\n'], async (method, args) => {
    calls.push({ method, args });
    return method === "interact" ? { status: "waiting", host: { receipt } }
      : { status: "ready", host: { receipt: null } };
  });
  assert.deepEqual(calls.map(call => call.method), ["interact", "wait"]);
  assert.equal(calls[0].args.waitMs, 25_000);
  assert.equal(Object.hasOwn(calls[0].args, "maxWaitMs"), false);
  assert.deepEqual(result.replies[1].host.receipt, receipt);
});

test("initial mutation receipt survives expiry of the local wait budget", async () => {
  const { setTimeout: delay } = await import("node:timers/promises");
  const receipt = { requestId: "committed", accepted: 1 };
  const calls = [];
  const result = await session([initialization,
    '{"method":"interact","requestId":"committed","maxWaitMs":1}\n'],
  async (method, args, { signal } = {}) => {
    calls.push({ method, args });
    await delay(15, undefined, { signal });
    return { status: "waiting", host: { receipt } };
  });
  assert.deepEqual(calls.map(call => call.method), ["interact"]);
  assert.equal(calls[0].args.waitMs, 1);
  assert.deepEqual(result.replies[1].host.receipt, receipt);
  assert.equal(result.replies[1].sessionWait.reason, "timeout");
  assert.equal(result.replies[1].error, undefined);
});

test("bounded waits return explicit timeout and read failure preserves committed receipt", async () => {
  const receipt = { requestId: "m", accepted: 1 };
  const timed = await session([initialization, '{"method":"next","maxWaitMs":0}\n'], async () => ({ status: "waiting" }));
  assert.equal(timed.replies[1].sessionWait.reason, "timeout");
  const failed = await session([initialization, '{"method":"interact","requestId":"m"}\n'], async method => {
    if (method === "interact") return { status: "waiting", receipt };
    throw new Error("read failed");
  });
  assert.deepEqual(failed.replies[1].receipt, receipt);
  assert.equal(failed.replies[1].sessionWait.reason, "error");
});

test("v2 session acknowledges delivered event cursors and full resets replay", async () => {
  const calls = [];
  const startup = `${JSON.stringify({ ...connection, presentation: "pilot-v2" })}\n`;
  const result = await session([startup, '{"method":"observe"}\n{"method":"interact","maxWaitMs":60000}\n{"method":"observe","full":true}\n'], async (method, args) => {
    calls.push({ method, args });
    const waiting = method === "interact";
    return { status: waiting ? "waiting" : "ready", host: { eventDelivery: { after: args.afterEvent, next: calls.length } } };
  });
  assert.deepEqual(calls.map(call => [call.method, call.args.afterEvent]), [["observe", 0], ["interact", 1], ["wait", 1], ["observe", 0]]);
  assert.equal(result.replies[2].host.eventDelivery.next, 3);
});

test("failed output does not acknowledge events", async () => {
  const startup = `${JSON.stringify({ ...connection, presentation: "pilot-v2" })}\n`;
  let writes = 0;
  const output = { write(_line, callback) { callback(++writes === 2 ? new Error("output failed") : null); } };
  const cursors = [];
  await runNativeSession(Readable.from([startup, '{"method":"observe"}\n{"method":"observe"}\n']), output, async (_method, args) => {
    cursors.push(args.afterEvent);
    return { status: "ready", host: { eventDelivery: { after: args.afterEvent, next: 4 } } };
  });
  assert.deepEqual(cursors, [0, 0]);
});
