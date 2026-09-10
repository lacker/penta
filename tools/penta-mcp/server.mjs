import { pathToFileURL } from "node:url";
import { McpServer } from "@modelcontextprotocol/server";
import { serveStdio } from "@modelcontextprotocol/server/stdio";
import * as z from "zod/v4";
import { SessionClient } from "./client.mjs";

const uint = z.number().int().min(0).max(0xffffffff);
const connection = z.string();
const waitMs = z.number().int().min(0).max(25_000).default(25_000);
const choice = z.union([
  z.object({ index: uint }).strict(),
  z.object({ decision: uint, options: z.array(uint).max(4096) }).strict(),
  z.object({ action: z.record(z.string(), z.unknown()) }).strict(),
]);

export function createServer(base = process.env.PENTA_SERVER_URL ?? "http://localhost:3000", client = new SessionClient(base)) {
  const server = new McpServer({ name: "penta", version: "0.1.0" }, {
    instructions: "Play through exact engine choices. Attach only to your assigned seat. play submits and waits; next waits without moving. Responses contain a full observation or exact changes from baseRevision; request next(full=true) to resynchronize. The reconstruction checkpoint is separate. Large menus carry counts and inspect references; inspect pages or searches every option in engine order. All actions, including mana abilities, remain accessible. Batch exact action values, never old indices. No tool chooses moves or passes priority. On an uncertain play failure, retry before submitting a different play.",
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
    z.object({ room: z.string(), token: z.string() }), "attach", true);
  tool("next", "Wait for your next decision; full=true returns a complete playing observation.",
    z.object({ connection, waitMs, full: z.boolean().default(false) }), "next", true);
  tool("play", "Submit exact choices and wait. Batch actions stop at the first unfulfilled choice; inspect the accepted count.",
    z.object({ connection, revision: z.string(), choices: z.array(choice).min(1).max(64), requestId: z.string().optional(), waitMs }), "play");
  tool("retry", "Retry the last uncertain play with its original request ID, without playing twice.",
    z.object({ connection, waitMs }), "retry");
  tool("inspect", "Read exact details. Catalog lookup accepts definition IDs or a name query; no card ranking is applied.",
    z.object({ connection, section: z.enum(["observation", "checkpoint", "catalog", "legalActions", "decision", "match", "record"]),
      definitions: z.array(uint).optional(), query: z.string().optional(), actionType: z.string().optional(),
      offset: z.number().int().min(0).default(0), limit: z.number().int().min(1).max(100).default(100) }), "inspect", true);
  return server;
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  await serveStdio(() => createServer());
}
