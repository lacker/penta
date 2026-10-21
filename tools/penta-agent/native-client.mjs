import { DecisionView } from "./decision-view.mjs";
import { PilotView } from "./pilot-view.mjs";
import { pilotSchema, validateContract } from "./pilot-contract.mjs";
import { PilotV2View } from "./pilot-v2-view.mjs";
import { pilotV2Schema } from "./pilot-v2-contract.mjs";
import { PilotV3View } from "./pilot-v3-view.mjs";
import { pilotV3Schema } from "./pilot-v3-contract.mjs";

const methods = new Set(["seats", "status", "export", "close", "observe", "wait", "choose", "play", "interact", "concede", "yield", "cancelYield", "receipt", "transcript", "note", "catalog", "discover"]);
const mutations = new Set(["choose", "play", "interact", "concede", "yield", "cancelYield"]);

export function validateNativeConnection({ server, secret, presentation = "pilot-v3" }) {
  let url;
  try { url = new URL(server); } catch { throw new Error("server must be an HTTP origin without credentials"); }
  if (!["http:", "https:"].includes(url.protocol) || url.username || url.password || url.search || url.hash || url.pathname !== "/") {
    throw new Error("server must be an HTTP origin without credentials");
  }
  if (typeof secret !== "string" || !secret) throw new Error("secret must be a nonempty string");
  if (!["pilot-v3", "pilot-v2", "pilot-v1", "decision-v1", "exact"].includes(presentation)) throw new Error("unsupported presentation");
  return url;
}

/** One serial session owns its public catalog cache and transport counters. */
export function createNativeClient(fetcher = fetch) {
  const context = { catalog: null, stats: { callsByMethod: {}, elapsedMsByMethod: {}, requestBytes: 0, responseJsonBytes: 0, elapsedMs: 0 } };
  let origin;
  const command = async (method, input, options = {}) => {
    if (method !== "schema") {
      const nextOrigin = validateNativeConnection(input).origin;
      if (origin !== nextOrigin) context.catalog = null;
      origin = nextOrigin;
    }
    return nativeCommand(method, input, fetcher, Object.assign(context, { signal: options.signal }));
  };
  command.stats = context.stats;
  return command;
}

export async function nativeCommand(method, input, fetcher = fetch, context = {}) {
  if (method === "schema") {
    const presentation = input?.presentation ?? "pilot-v3";
    if (!["pilot-v1", "pilot-v2", "pilot-v3"].includes(presentation)) throw new Error("schema presentation must be pilot-v1, pilot-v2, or pilot-v3");
    return structuredClone(presentation === "pilot-v1" ? pilotSchema : presentation === "pilot-v2" ? pilotV2Schema : pilotV3Schema);
  }
  if (!input || typeof input !== "object" || Array.isArray(input)) throw new Error("arguments must be a JSON object");
  const { server, secret, presentation = "pilot-v3", ...args } = input;
  const pilot = ["pilot-v1", "pilot-v2", "pilot-v3"].includes(presentation);
  const modern = ["pilot-v2", "pilot-v3"].includes(presentation);
  const contract = pilotV3Schema;
  const url = validateNativeConnection({ server, secret, presentation });
  if (!methods.has(method) && method !== "inspect_ref") throw new Error(`unknown native command: ${method}`);
  if (args.waitMs !== undefined && (!Number.isInteger(args.waitMs) || args.waitMs < 0 || args.waitMs > 25_000)) throw new Error("waitMs must be an integer from 0 to 25000");
  if ((mutations.has(method) || method === "receipt") && (typeof args.requestId !== "string" || !args.requestId || Buffer.byteLength(args.requestId, "utf8") > 128)) throw new Error("requestId must contain 1 to 128 UTF-8 bytes; retain it for retries");
  const { afterEvent, ...contractArgs } = args;
  if (afterEvent !== undefined && (!Number.isSafeInteger(afterEvent) || afterEvent < 0)) throw new Error("afterEvent must be a nonnegative safe integer");
  if (method === "yield") validateContract(contractArgs, "YieldRequest");
  if (method === "cancelYield") validateContract(contractArgs, "CancelYieldRequest");
  if (method === "interact") {
    if (Array.isArray(args.choices)) args.choices = args.choices.map(choice => {
      if (choice?.action?.type !== "ChooseDecision") return choice;
      if (Object.keys(choice).some(key => key !== "action") ||
          Object.keys(choice.action).some(key => !["index", "type", "decision", "options"].includes(key))) {
        throw new Error("ChooseDecision accepts only an action wrapper with index, type, decision, and options; unsupported fields cannot be discarded");
      }
      const { decision, options } = choice.action;
      return { decision, options };
    });
    validateContract({ ...contractArgs, choices: args.choices }, "NativeInteractionRequest", contract);
  }
  if (method === "play" || method === "interact") {
    if (!Array.isArray(args.choices) || (!args.choices.length && !(method === "interact" && args.yield)) || args.choices.length > 64) throw new Error("choices must contain 1 to 64 entries");
    args.choices = args.choices.map((choice, index) => {
      try { validateContract(choice, "NativePlayChoice", contract); }
      catch (error) {
        throw new Error(`choices[${index}] must be {proposal: {...}}, {action: {...}}, {decision: ID, options: [IDs]}, or a single {index: N}. ${error.message}. Wrap a displayed action in {action: ...}; use choose for a ticket.`);
      }
      if (args.choices.length > 1 && Object.hasOwn(choice, "index")) throw new Error(`choices[${index}]: batches require {action: ...} or {decision, options}; indices can change between actions`);
      if (!Object.hasOwn(choice, "action")) return choice;
      // Observation indices are presentation metadata, not canonical action fields.
      const { index: _index, ...action } = choice.action;
      return { ...choice, action };
    });
  }
  for (const key of ["offset", "limit"]) {
    if (args[key] !== undefined && (!Number.isInteger(args[key]) || args[key] < (key === "limit" ? 1 : 0) || (key === "limit" && args[key] > 100))) throw new Error(`invalid ${key}`);
  }
  for (const key of ["query", "actionType"]) {
    if (args[key] !== undefined && typeof args[key] !== "string") throw new Error(`${key} must be a string`);
  }
  const request = async (operation, body) => {
    const bodyText = JSON.stringify({ method: operation, args: body });
    const started = performance.now();
    const stats = context.stats;
    if (stats) {
      stats.callsByMethod[operation] = (stats.callsByMethod[operation] ?? 0) + 1;
      stats.requestBytes += Buffer.byteLength(bodyText);
    }
    let response;
    let result;
    try {
      const timeout = AbortSignal.timeout(35_000);
      response = await fetcher(url.href, {
        method: "POST", headers: { authorization: `Bearer ${secret}`, "content-type": "application/json" },
        body: bodyText, signal: context.signal ? AbortSignal.any([timeout, context.signal]) : timeout,
      });
      try { result = await response.json(); } catch { throw new Error(`Penta HTTP ${response.status} without JSON; retry uncertain mutations with the identical request`); }
      if (stats) stats.responseJsonBytes += Buffer.byteLength(JSON.stringify(result));
    } catch (error) {
      if (mutations.has(operation)) throw new Error(`${error.message}; outcome uncertain for requestId ${body.requestId}; recover its receipt or retry the identical request`);
      throw error;
    } finally {
      if (stats) {
        const elapsed = performance.now() - started;
        stats.elapsedMs += elapsed;
        stats.elapsedMsByMethod[operation] = (stats.elapsedMsByMethod[operation] ?? 0) + elapsed;
      }
    }
    if (!response.ok) throw new Error(`Penta HTTP ${response.status}: ${result.error ?? response.statusText}`);
    return result;
  };
  const build = async view => {
    const events = modern ? view.events : null;
    if (events) {
      const { events: _events, ...rest } = view;
      view = rest;
    }
    let catalog = null;
    if (view.observation) {
      try { catalog = context.catalog ?? await request("catalog", {}); context.catalog = catalog; }
      catch (error) {
        if (!pilot) throw error;
        view = { ...view, presentationWarning: "Public catalog unavailable; exact choices and observation fields retained. Printed rules may be unavailable." };
      }
    }
    const rendered = presentation === "pilot-v3" ? new PilotV3View(view, catalog)
      : presentation === "pilot-v2" ? new PilotV2View(view, catalog)
      : presentation === "pilot-v1" ? new PilotView(view, catalog)
      : new DecisionView(view, catalog, new Set(), true, { id: view.revision });
    if (events) {
      rendered.packet.host.eventDelivery = { after: events.after, next: events.next,
        ...(!rendered.packet.position ? { items: events.items } : {}) };
      // Cursor acknowledgement covers every item, so event deliveries cannot page.
      if (rendered.packet.position) rendered.packet.position.updates = { items: events.items, total: events.items.length, reference: null };
    }
    return rendered;
  };
  if (method === "inspect_ref") {
    const { reference, ...paging } = args;
    if (typeof reference !== "string" || !reference) throw new Error("reference must be a nonempty string");
    const view = await request(presentation === "pilot-v3" || presentation === "exact" ? "observe" : "discover", {});
    const plausible = presentation === "pilot-v3"
      ? /^p3r:[A-Za-z0-9_-]{43}$/.test(reference)
      : presentation === "pilot-v2"
      ? /^p2r:[A-Za-z0-9_-]{43}$/.test(reference)
      : reference.startsWith(`${view.revision}:r`);
    if (!view.observation || !plausible) throw new Error("stale reference; observe again");
    const current = await build(view);
    if (modern && !current.references.has(reference)) throw new Error("stale reference; observe again");
    return current.inspect(reference, paging);
  }
  const result = await request(method, args);
  if (presentation === "exact" || (!result.observation && (!pilot || ![1, 2].includes(result.apiVersion) || !["ready", "waiting", "complete", "paused"].includes(result.status)))) return result;
  try {
    const view = await build(result);
    if (pilot) return view.packet;
    const { observation: _observation, ...envelope } = result;
    return { ...envelope, ...view.packet };
  } catch {
    // The mutation may already be committed. Presentation is never its outcome.
    return { ...result, presentation: "exact", presentationWarning: "Compact presentation unavailable; exact observation retained." };
  }
}
