import { createServer } from "node:http";
import { randomBytes } from "node:crypto";
import { isDeepStrictEqual } from "node:util";
import { mkdirSync } from "node:fs";
import { SessionClient } from "./client.mjs";
import { Backend } from "./backend.mjs";
import { methods, validate } from "./commands.mjs";
import { directory, read, write, serverUrl, agentPort } from "./runtime.mjs";

mkdirSync(directory, { recursive: true, mode: 0o700 });
const base = serverUrl();
let client, backend, persist, sessions, creations;
const token = randomBytes(32).toString("hex");
let active = 0;
const host = createServer(async (request, response) => {
  response.setHeader("content-type", "application/json");
  if (request.method !== "POST" || request.headers.authorization !== `Bearer ${token}`) {
    response.writeHead(403).end(JSON.stringify({ error: "unauthorized local agent request" })); return;
  }
  active++;
  try {
    let text = "";
    for await (const chunk of request) {
      text += chunk;
      if (text.length > 1_000_000) throw new Error("command exceeds 1 MB");
    }
    const { command, args = {} } = JSON.parse(text);
    validate(command, args);
    let result;
    if (command === "status") result = { status: "running", server: base, pid: process.pid, connections: sessions.map(([connection]) => connection) };
    else if (command === "stop") {
      if (active > 1) throw new Error("a command is still running; wait before stopping");
      result = { status: "stopped", note: "Game server remains running; connections and pending moves are saved" };
      response.on("finish", () => { host.close(); host.closeAllConnections(); });
    } else if (command === "up") result = await backend.up(args.waitMs);
    else if (command === "start_match") {
      const { requestId, ...config } = args;
      const previous = creations.get(requestId);
      if (previous) {
        if (!isDeepStrictEqual(previous.config, config)) throw new Error("start_match requestId already used with different arguments");
        if (!previous.result) throw new Error("match creation outcome is uncertain; credentials were not saved. Do not automatically create a replacement game");
        result = previous.result;
      } else {
        const state = await backend.up(0);
        if (state.status !== "ready") throw new Error(`server is starting; run up before start_match; log: ${state.log}`);
        if (creations.has(requestId)) throw new Error("match creation is already in progress; repeat the same request after it finishes");
        creations.set(requestId, { config }); persist();
        result = await client.create(config);
        creations.set(requestId, { config, result }); persist();
      }
    } else result = await client[methods[command]](args);
    response.end(JSON.stringify(result));
  } catch (error) {
    response.writeHead(400).end(JSON.stringify({ error: error.message }));
  } finally { active--; }
});
// The listening socket is the ownership lock: the OS releases it on a crash.
// Bind before reading recovery data so competing CLI starts cannot both own it.
host.on("error", error => {
  console.error(error.message);
  process.exitCode = 1;
});
host.listen(agentPort, "127.0.0.1", () => {
  const saved = read("sessions.json", { base, sessions: [], creations: [] });
  if (saved.base !== base) throw new Error("saved sessions belong to another server; use a different PENTA_AGENT_DIR");
  creations = new Map(saved.creations);
  sessions = saved.sessions;
  persist = () => write("sessions.json", { base, sessions, creations: [...creations] });
  client = new SessionClient(base, fetch, { sessions, save: value => { sessions = value; persist(); } });
  backend = new Backend(base);
  write("daemon.json", { pid: process.pid, port: host.address().port, token, base });
});
for (const signal of ["SIGINT", "SIGTERM"]) process.on(signal, () => { host.close(); host.closeAllConnections(); });
