#!/usr/bin/env node
import { spawn } from "node:child_process";
import { closeSync, mkdirSync, openSync, readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { setTimeout as delay } from "node:timers/promises";
import { alive, call, directory, file, read, root, serverUrl } from "./runtime.mjs";
import { validate } from "./commands.mjs";

const help = `Usage: node tools/penta-agent/cli.mjs COMMAND [JSON | @FILE | -]
Commands:
  up          Start/reuse the worktree game server; wait up to 25 seconds.
  status      Show local session process and saved connection handles.
  options     List formats and decks (run up first).
  start_match Create a match: requestId, format, p1Deck, p2Deck, humanSeat?, matchMode?.
              Repeat the same requestId and arguments to retrieve its saved result.
  attach      Join with room, token, presentation? (defaults to decision-v1).
  next        Wait/refresh: connection, full?, waitMs?.
  choose      Submit one ticket: ticket, options?, waitMs?.
  play        Submit a batch: connection, revision, choices, requestId?, waitMs?.
  retry       Recover the most recent play: connection, waitMs?.
  inspect_ref Read a frozen reference: reference, offset?, limit?.
  inspect     Read exact details: connection, section, definitions?, query?, offset?, limit?.
  stop        Stop the local session process; leave the game server running.
JSON goes on stdout; errors go on stderr. Use - or @FILE for credentials.
PENTA_SERVER_URL selects an existing server; it disables local game-server startup.
PENTA_AGENT_PORT overrides the deterministic local control port if it is occupied.
PENTA_AGENT_DIR isolates local sessions (default: this checkout's .penta-agent).
Node >=22.13 is required. No MCP registration or client dependencies are needed.`;

async function ensure() {
  mkdirSync(directory, { recursive: true, mode: 0o700 });
  const existing = read("daemon.json", {});
  if (alive(existing.pid)) {
    if (existing.base !== serverUrl()) throw new Error("session process uses another server; stop it with its original configuration or set PENTA_AGENT_DIR");
    try { await call(existing, "status", {}); return existing; }
    catch { /* An exited daemon's PID can have been reused; the port still arbitrates ownership. */ }
  }
  const log = openSync(file("daemon.log"), "a", 0o600);
  const child = spawn(process.execPath, [fileURLToPath(new URL("daemon.mjs", import.meta.url))], {
    cwd: root, env: { ...process.env, PENTA_AGENT_DIR: directory }, detached: true, stdio: ["ignore", log, log],
  });
  closeSync(log);
  child.on("error", error => console.error(error.message));
  child.unref();
  for (let attempt = 0; attempt < 100; attempt++) {
    await delay(50);
    const address = read("daemon.json", {});
    if (!alive(address.pid)) continue;
    try { await call(address, "status", {}); return address; } catch {}
  }
  throw new Error(`session process did not start; inspect ${file("daemon.log")}`);
}

try {
  const [command, input, ...extra] = process.argv.slice(2);
  if (!command || command === "--help" || command === "help") console.log(help);
  else {
    if (extra.length) throw new Error("pass command arguments as one JSON object, @FILE, or - for stdin");
    const json = input === "-" ? readFileSync(0, "utf8") : input?.startsWith("@") ? readFileSync(input.slice(1), "utf8") : input ?? "{}";
    const args = JSON.parse(json);
    validate(command, args);
    const address = await ensure();
    console.log(JSON.stringify(await call(address, command, args)));
  }
} catch (error) {
  console.error(JSON.stringify({ error: error.message }));
  process.exitCode = 1;
}
