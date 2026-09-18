import { mkdirSync, readFileSync, renameSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import path from "node:path";
import { createHash } from "node:crypto";
import { getWorktreeDevPort } from "../../web/worktree-port.js";

export const root = fileURLToPath(new URL("../../", import.meta.url));
export const directory = path.resolve(process.env.PENTA_AGENT_DIR ?? path.join(root, ".penta-agent"));
export const agentPort = Number(process.env.PENTA_AGENT_PORT ?? (49152 + createHash("sha256").update(directory).digest().readUInt16BE(0) % 16384));
if (!Number.isInteger(agentPort) || agentPort < 1 || agentPort > 65535) throw new Error("PENTA_AGENT_PORT must be an integer from 1 to 65535");
export const file = name => path.join(directory, name);
export const serverUrl = () => {
  const url = new URL(process.env.PENTA_SERVER_URL ?? `http://localhost:${getWorktreeDevPort()}`);
  if (!["http:", "https:"].includes(url.protocol) || url.username || url.password) throw new Error("invalid Penta server URL");
  return url.origin;
};
export function read(name, fallback) {
  try { return JSON.parse(readFileSync(file(name), "utf8")); }
  catch (error) { if (error.code === "ENOENT") return fallback; throw error; }
}
export function write(name, value) {
  mkdirSync(directory, { recursive: true, mode: 0o700 });
  const temporary = `${file(name)}.${process.pid}.tmp`;
  writeFileSync(temporary, JSON.stringify(value), { mode: 0o600 });
  renameSync(temporary, file(name));
}
export function alive(pid) {
  if (!Number.isInteger(pid) || pid < 1) return false;
  try { process.kill(pid, 0); return true; }
  catch (error) { return error.code !== "ESRCH"; }
}
export async function call(address, command, args) {
  const response = await fetch(`http://127.0.0.1:${address.port}`, {
    method: "POST", headers: { authorization: `Bearer ${address.token}`, "content-type": "application/json" },
    body: JSON.stringify({ command, args }), signal: AbortSignal.timeout(45_000),
  });
  const result = await response.json();
  if (!response.ok) throw new Error(result.error);
  return result;
}
