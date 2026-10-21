#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { nativeCommand } from "./native-client.mjs";
import { runNativeSession } from "./native-session.mjs";

const help = `Usage: node tools/penta-agent/native.mjs METHOD [JSON | @FILE | -]
Each input includes {server, secret, ...args}. Prefer stdin for credentials.
Persistent route: node tools/penta-agent/native.mjs session
First stdin line: {server, secret, presentation?}; reply: {status:"initialized"}.
Then one {method, ...args} JSON request per line, with one JSON response per line.
Requests run serially, lines are limited to 1 MiB, and EOF exits cleanly.
Individual request errors keep the session alive; invalid initialization exits.
Credentials stay in memory. No automatic choices, retries, or request IDs.
Local sessionStats reports command timings, wire counts/JSON bytes, renewals, input gaps, errors and output bytes.
Input gaps measure response-ready to next input, not model time.
Stats exclude their own call count and measure bytes before their response.
Session rules print once per unchanged summary; printedPreviouslySent marks reuse.
Send {method:"observe",full:true} after context loss to reprint rules.
Native play choices accept {proposal:{action:{...},payment?}}, {action:{...}}, {decision,options}, or one {index}.
Use choose for tickets; native play does not accept ticket batches or bare actions.
Seat methods: observe, discover, wait, choose, play, interact, concede, yield, cancelYield, receipt, transcript, note, catalog.
Session next waits for ready/complete/paused; interact submits once then waits internally.
Both accept local maxWaitMs (default/max 60000); timeout returns waiting with sessionWait.
interact accepts requestId, revision, choices (typed proposals/actions/decisions), optional yield, waitMs.
Empty interact choices require yield. Session interact defaults waitMs to 25000.
Organizer methods: seats, status, export, close.
Adapter method: inspect_ref {reference, offset?, limit?}.
choose/play/interact/concede/yield/cancelYield require a caller-retained requestId. Retry the identical
method and arguments after uncertain outcomes. No credentials or requests are saved.
Views default to pilot-v3; pilot-v2/pilot-v1/decision-v1 menus require explicit discover. exact remains available.
schema prints the pilot-v3 JSON Schema without credentials; presentation: "pilot-v1" selects v1.
See docs/native-matches.md for startup, reconnect and lifecycle limits.`;

try {
  const [method, input, ...extra] = process.argv.slice(2);
  if (!method || method === "--help" || method === "help") console.log(help);
  else if (method === "session") {
    if (input !== undefined || extra.length) throw new Error("session reads JSON lines from stdin; no command-line arguments");
    await runNativeSession(process.stdin, process.stdout);
  }
  else {
    if (extra.length) throw new Error("pass one JSON object, @FILE, or - for stdin");
    const text = input === "-" ? readFileSync(0, "utf8") : input?.startsWith("@") ? readFileSync(input.slice(1), "utf8") : input ?? "{}";
    console.log(JSON.stringify(await nativeCommand(method, JSON.parse(text))));
  }
} catch (error) {
  console.error(JSON.stringify({ error: error.message }));
  process.exitCode = 1;
}
