import { spawn } from "node:child_process";
import { closeSync, existsSync, openSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { setTimeout as delay } from "node:timers/promises";
import { file, root, read, write, alive } from "./runtime.mjs";

// The agent can wait in bounded calls while a first-run install/build continues.
export class Backend {
  constructor(base, { remote = Boolean(process.env.PENTA_SERVER_URL), launch } = {}) {
    this.base = base;
    this.remote = remote;
    this.launch = launch ?? (() => this.startLocal());
  }
  async ready() {
    try {
      const response = await fetch(`${this.base}/_engine/options`, { signal: AbortSignal.timeout(1000) });
      if (!response.ok) return false;
      const value = await response.json();
      return value.apiVersion === 1 && Array.isArray(value.formats);
    } catch { return false; }
  }
  startLocal() {
    // Kept separate from the session daemon so reconnecting it cannot kill games.
    const log = openSync(file("backend.log"), "a", 0o600);
    const child = spawn(process.execPath, [fileURLToPath(new URL("backend.mjs", import.meta.url)), "--run"], {
      cwd: root, env: process.env, detached: true, stdio: ["ignore", log, log],
    });
    closeSync(log);
    child.on("error", error => { this.failure = error.message; });
    child.unref();
    write("backend.json", { pid: child.pid, base: this.base });
  }
  async up(waitMs = 25_000) {
    if (await this.ready()) return { status: "ready", server: this.base };
    if (this.remote) throw new Error(`Penta server unavailable at ${this.base}; PENTA_SERVER_URL disables local startup`);
    if (!this.starting) {
      const previous = read("backend.json", {});
      if (previous.base !== this.base || !alive(previous.pid)) {
        this.starting = true;
        this.launch();
      }
    }
    const deadline = Date.now() + waitMs;
    do {
      if (await this.ready()) return { status: "ready", server: this.base };
      if (this.failure) throw new Error(this.failure);
      const running = read("backend.json", {});
      if (running.pid && !alive(running.pid)) {
        this.starting = false;
        throw new Error(`Penta startup failed; inspect ${file("backend.log")}, fix the cause, and run up again`);
      }
      if (Date.now() >= deadline) break;
      await delay(250);
    } while (true);
    return { status: "starting", server: this.base, log: file("backend.log"), next: "Run up again to await readiness" };
  }
}

if (process.argv[2] === "--run") {
  const run = args => new Promise((resolve, reject) => {
    const child = spawn("pnpm", ["--dir", path.join(root, "web"), ...args], { stdio: "inherit" });
    child.on("error", reject);
    child.on("exit", (code, signal) => code === 0 ? resolve() : reject(new Error(`pnpm ${args.join(" ")} exited ${signal ?? code}`)));
  });
  try {
    if (!existsSync(path.join(root, "web/node_modules/.bin/vinext"))) await run(["install", "--frozen-lockfile"]);
    await run(["run", "dev"]);
  } catch (error) { console.error(error.message); process.exitCode = 1; }
}
