import { pathToFileURL } from "node:url";
import { McpServer } from "@modelcontextprotocol/server";
import { serveStdio } from "@modelcontextprotocol/server/stdio";
import * as z from "zod/v4";
import { SessionClient } from "./client.mjs";
import { getWorktreeDevPort } from "../../web/worktree-port.js";

const uint = z.number().int().min(0).max(0xffffffff);
const connection = z.string();
const waitMs = z.number().int().min(0).max(25_000).default(25_000);
const choice = z.union([
  z.object({ index: uint }).strict(),
  z.object({ decision: uint, options: z.array(uint).max(4096) }).strict(),
  z.object({ action: z.record(z.string(), z.unknown()) }).strict(),
  z.object({ ticket: z.string(), options: z.array(uint).max(4096).optional() }).strict(),
]);

export function createServer(base = process.env.PENTA_SERVER_URL ?? `http://localhost:${getWorktreeDevPort()}`, client = new SessionClient(base)) {
  const server = new McpServer({ name: "penta", version: "0.1.0" }, {
    instructions: "Attach only to your assigned seat with presentation=decision-v1. Read the current position, updates, and choices. Submit already-decided groups together with play(choices=[{ticket},...]); choose(ticket) handles one action or a whole selection. Both return the next view; do not call next again when ready. Decision tickets require explicit option IDs in options, preserving order. Read oversized sections with inspect_ref(reference). Use next when waiting or next(full=true) to refresh. Resolve uncertain submissions with retry before another move. All real decisions remain yours; only engine-forced continuations advance automatically. Each shared/rows table is self-contained: shared fields apply to every row. Printed card text is reference material; observed characteristics may differ. No explanation is required with a move. Exact play/inspect remain available; default attach presentation=exact returns full JSON or changes from baseRevision. Never reuse old indices or parse descriptive labels into commands.",
  });
  const tool = (name, description, schema, method, readOnly = false) => server.registerTool(name, {
    description, inputSchema: schema,
    annotations: { readOnlyHint: readOnly, destructiveHint: false, openWorldHint: false },
  }, async (args, ctx) => {
    try {
      const value = await client[method](args, ctx.mcpReq.signal);
      return { content: [{ type: "text", text: JSON.stringify(value) }] };
    } catch (error) {
      return { isError: true, content: [{ type: "text", text: error.message }] };
    }
  });
  tool("options", "List the server's registered formats and built-in deck names.", z.object({}), "options", true);
  tool("start_match", "Create a match; give each player only its own seat credential. humanSeat adds a browser link.", z.object({
    format: z.string(), p1Deck: z.string(), p2Deck: z.string(),
    matchMode: z.enum(["one-conclusion", "first-to-two-wins"]).default("first-to-two-wins"),
    humanSeat: z.enum(["p1", "p2"]).optional(),
  }), "create");
  tool("attach", "Connect to an existing match using only the assigned seat's credential.",
    z.object({ room: z.string(), token: z.string(), presentation: z.enum(["exact", "decision-v1"]).default("exact") }), "attach", true);
  tool("next", "Wait for your next decision; full=true returns a complete playing observation.",
    z.object({ connection, waitMs, full: z.boolean().default(false) }), "next", true);
  tool("play", "Submit already-decided choices together and wait, including attacker/blocker groups. Batch choices can use current-view tickets, exact actions, or decisions. Stop at new information you need. Inspect receipt.accepted; retry uncertain results.",
    z.object({ connection, revision: z.string(), choices: z.array(choice).min(1).max(64), requestId: z.string().optional(), waitMs }), "play");
  tool("retry", "Retry the last uncertain play with its original request ID, without playing twice.",
    z.object({ connection, waitMs }), "retry");
  tool("choose", "Submit a decision-v1 ticket and wait. Decision selections require explicit option IDs; repeat an identical ticket to recover its retained receipt.",
    z.object({ ticket: z.string(), options: z.array(uint).max(4096).optional(), waitMs }).strict(), "choose");
  tool("inspect_ref", "Read a frozen decision-v1 reference in engine order. Expired references require next(full=true).",
    z.object({ reference: z.string(), offset: z.number().int().min(0).default(0), limit: z.number().int().min(1).max(100).default(100),
      query: z.string().optional(), actionType: z.string().optional() }), "inspectReference", true);
  tool("inspect", "Read exact details. Catalog lookup accepts definition IDs or a name query; no card ranking is applied.",
    z.object({ connection, section: z.enum(["observation", "checkpoint", "catalog", "legalActions", "decision", "updates", "match", "record"]),
      definitions: z.array(z.string()).optional(), query: z.string().optional(), actionType: z.string().optional(),
      offset: z.number().int().min(0).default(0), limit: z.number().int().min(1).max(100).default(100) }), "inspect", true);
  return server;
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  await serveStdio(() => createServer());
}
