/** Verify native stdio tool discovery and backend reachability without a game. */
import { fileURLToPath } from "node:url";
import { Client } from "@modelcontextprotocol/client";
import { StdioClientTransport } from "@modelcontextprotocol/client/stdio";

const client = new Client({ name: "penta-probe", version: "1" });
try {
  await client.connect(new StdioClientTransport({ command: process.execPath,
    args: [fileURLToPath(new URL("server.mjs", import.meta.url))], env: process.env, stderr: "inherit" }));
  const { tools } = await client.listTools();
  const names = tools.map(tool => tool.name);
  for (const name of ["attach", "choose", "next", "inspect_ref", "retry"]) {
    if (!names.includes(name)) throw new Error(`missing MCP tool: ${name}`);
  }
  const result = await client.callTool({ name: "options", arguments: {} });
  if (result.isError) throw new Error(result.content[0].text);
  const options = JSON.parse(result.content[0].text);
  console.log(JSON.stringify({ transport: "stdio", tools: names, formats: options.formats.map(format => format.id),
    note: "Transport and backend verified. The pilot task must also discover these tools in its own host." }, null, 2));
} finally { await client.close(); }
