/** Offline presentation size comparison; never executes a move or invokes a model. */
import { readFile } from "node:fs/promises";
import { gunzipSync } from "node:zlib";
import { present } from "./views.mjs";
import { DecisionView } from "./decision-view.mjs";

const path = process.argv[2];
if (!path) throw new Error("usage: node tools/penta-mcp/measure-trace.mjs trace.jsonl[.gz] [--decision] [--catalog catalog.json]");
const decisions = process.argv.includes("--decision");
const catalogIndex = process.argv.indexOf("--catalog");
const catalog = catalogIndex < 0 ? undefined : JSON.parse(await readFile(process.argv[catalogIndex + 1], "utf8"));
const bytes = await readFile(path);
const lines = (path.endsWith(".gz") ? gunzipSync(bytes) : bytes).toString().trim().split("\n");
const previous = new Map();
const rules = new Map();
const totals = { observations: 0, rawCharacters: 0, fullPlayingCharacters: 0, compactPlayingCharacters: 0 };
if (decisions) totals.decisionPlayingCharacters = 0;
for (const line of lines) {
  // One seat observation per row, or a referee row with observations by seat.
  const row = JSON.parse(line);
  if (row.auto) continue;
  const observation = row.observation ?? row.observations?.[row.seat];
  if (!observation) throw new Error("each explicit row needs seat, revision, and observation");
  const view = { apiVersion: 1, status: "ready", role: row.seat, revision: String(row.revision), observation };
  const compact = present(view, previous.get(row.seat));
  previous.set(row.seat, compact.previous);
  totals.observations++;
  totals.rawCharacters += JSON.stringify(observation).length;
  totals.fullPlayingCharacters += JSON.stringify(present(view).result).length;
  totals.compactPlayingCharacters += JSON.stringify(compact.result).length;
  if (decisions) {
    if (!rules.has(row.seat)) rules.set(row.seat, new Set());
    const { observation: _observation, ...envelope } = view;
    totals.decisionPlayingCharacters += JSON.stringify({ ...envelope,
      ...new DecisionView(view, catalog, rules.get(row.seat)).packet }).length;
  }
}
console.log(JSON.stringify({ ...totals,
  ...(decisions ? { catalogIncluded: !!catalog } : {}),
  note: "JSON characters, not model tokens; excludes inspections, tool schemas, and reasoning. auto rows are excluded." }, null, 2));
