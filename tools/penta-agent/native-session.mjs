import { createNativeClient, validateNativeConnection } from "./native-client.mjs";
import { sessionInteraction } from "./session-interaction.mjs";
import { SessionRules } from "./session-rules.mjs";

export const MAX_SESSION_LINE_BYTES = 1024 * 1024;

/** Bounded input buffering; dispatch and output finish before reading more input. */
async function* lines(input) {
  let pending = Buffer.alloc(0);
  let oversized = false;
  for await (const chunk of input) {
    const bytes = Buffer.isBuffer(chunk) ? chunk : Buffer.from(chunk);
    let start = 0;
    while (start < bytes.length) {
      const newline = bytes.indexOf(10, start);
      const end = newline < 0 ? bytes.length : newline;
      const part = bytes.subarray(start, end);
      if (!oversized && pending.length + part.length <= MAX_SESSION_LINE_BYTES) {
        pending = Buffer.concat([pending, part]);
      } else {
        oversized = true;
        pending = Buffer.alloc(0);
      }
      if (newline < 0) break;
      yield oversized ? null : pending.toString("utf8");
      pending = Buffer.alloc(0);
      oversized = false;
      start = newline + 1;
    }
  }
  if (oversized || pending.length) yield oversized ? null : pending.toString("utf8");
}

function redact(value, secret) {
  if (typeof value === "string") return secret ? value.split(secret).join("[redacted]") : value;
  if (Array.isArray(value)) return value.map(item => redact(item, secret));
  if (value && typeof value === "object") {
    return Object.fromEntries(Object.entries(value).map(([key, item]) => [redact(key, secret), redact(item, secret)]));
  }
  return value;
}

/** First line initializes credentials; subsequent lines are explicit commands. */
export async function runNativeSession(input, output, command = createNativeClient()) {
  let connection;
  let first = true;
  let secret;
  let startedAt;
  let outputBytes = 0;
  let errors = 0;
  const callsByMethod = {};
  const commandElapsedMs = {};
  const interactionStats = { waitRenewals: 0 };
  const inputGapMs = { count: 0, total: 0, max: 0 };
  let readyAt;
  let eventCursor = 0;
  const seatViewMethods = new Set(["observe", "wait", "choose", "play", "interact", "next", "concede", "yield", "cancelYield", "receipt"]);

  const printedRules = new SessionRules();
  const countedMethods = new Set(["seats", "status", "export", "close", "observe", "wait", "choose", "play", "interact", "next", "concede", "yield", "cancelYield", "receipt", "transcript", "note", "catalog", "discover", "inspect_ref", "schema"]);
  const reply = value => new Promise((resolve, reject) => {
    const line = `${JSON.stringify(redact(value, secret))}\n`;
    output.write(line, error => {
      if (error) reject(error);
      else { outputBytes += Buffer.byteLength(line); resolve(); }
    });
  });
  for await (const line of lines(input)) {
    const receivedAt = performance.now();
    try {
      if (line === null) throw new Error(`JSON line exceeds ${MAX_SESSION_LINE_BYTES} bytes`);
      let request;
      try { request = JSON.parse(line); } catch { throw new Error("invalid JSON line"); }
      if (!request || typeof request !== "object" || Array.isArray(request)) throw new Error("request must be a JSON object");
      if (first) {
        secret = typeof request.secret === "string" ? request.secret : undefined;
        if (Object.keys(request).some(key => !["server", "secret", "presentation"].includes(key))) throw new Error("initialization accepts only server, secret, and presentation");
        validateNativeConnection(request);
        connection = { ...request };
        first = false;
        startedAt = performance.now();
        await reply({ status: "initialized" });
        continue;
      }
      if (Object.hasOwn(request, "server") || Object.hasOwn(request, "secret")) throw new Error("session requests must omit server and secret");
      const { method, full, ...args } = request;
      if (typeof method !== "string" || !method) throw new Error("method must be a nonempty string");
      if (full !== undefined && typeof full !== "boolean") throw new Error("full must be a boolean");
      // Reset before dispatch even if it returns waiting/no position or fails.
      // The next inline rules are then self-contained after context loss.
      if (full === true) { printedRules.reset(); eventCursor = 0; }
      if (method === "sessionStats") {
        if (Object.keys(args).length) throw new Error("sessionStats accepts no arguments");
        await reply({ callsByMethod: { ...callsByMethod }, commandElapsedMs: { ...commandElapsedMs }, wire: command.stats ?? null, ...interactionStats, inputGapMs: { ...inputGapMs }, errors, outputBytes, elapsedMs: Math.floor(performance.now() - startedAt) });
        continue;
      }
      const counter = countedMethods.has(method) ? method : "unknown";
      callsByMethod[counter] = (callsByMethod[counter] ?? 0) + 1;
      if (readyAt !== undefined) {
        const gap = receivedAt - readyAt;
        inputGapMs.count++;
        inputGapMs.total += gap;
        inputGapMs.max = Math.max(inputGapMs.max, gap);
        readyAt = undefined;
      }
      const delivery = ["pilot-v2", "pilot-v3"].includes(args.presentation ?? connection.presentation ?? "pilot-v3") && seatViewMethods.has(method);
      if (delivery) args.afterEvent = eventCursor;
      const dispatchAt = performance.now();
      let result;
      try {
        result = ["next", "interact"].includes(method)
          ? await sessionInteraction(method, args, connection, command, interactionStats)
          : await command(method, { ...connection, ...args });
      } finally {
        commandElapsedMs[counter] = (commandElapsedMs[counter] ?? 0) + performance.now() - dispatchAt;
      }
      const prepared = printedRules.prepare(result);
      await reply(prepared.packet);
      prepared.commit();
      const delivered = result.host?.eventDelivery ?? result.events;
      if (delivery && Number.isSafeInteger(delivered?.next)) eventCursor = delivered.next;
      if (result.status === "ready") readyAt = performance.now();
    } catch (error) {
      errors++;
      await reply({ error: error.message });
      // Invalid startup cannot establish a usable session.
      if (first) return;
    }
  }
}
