/**
 * The bug ledger: one Durable Object holding every report.
 *
 * A report is a description plus the game's replay -- the config and command
 * journal that deterministically rebuild the board the reporter was looking
 * at. That makes a bug reproducible by machine: an agent lists the open
 * reports, rebuilds each game natively, fixes what it finds, and marks the
 * report resolved with a note. `docs/bug-reports.md` is that workflow.
 */

interface DurableStorage {
  get<T>(key: string): Promise<T | undefined>;
  put<T>(key: string, value: T): Promise<void>;
  list<T>(options: { prefix: string }): Promise<Map<string, T>>;
}
interface DurableState {
  storage: DurableStorage;
}

interface BugReport {
  id: string;
  reportedAt: string;
  status: "open" | "resolved";
  description: string;
  /** `WebGame::replay_json` output: config, commands, format, and simulation identity. */
  replay: unknown;
  /** Where it happened: page URL, hosted room if any. */
  context?: Record<string, unknown>;
  resolution?: string;
  resolvedAt?: string;
}

const PREFIX = "bug:";

export class BugTracker {
  readonly #state: DurableState;

  constructor(state: DurableState) {
    this.#state = state;
  }

  async fetch(request: Request): Promise<Response> {
    const url = new URL(request.url);
    const parts = url.pathname.split("/").filter(Boolean);
    // Shapes: /_bugs/report, /_bugs/list, /_bugs/<id>, /_bugs/<id>/resolve.
    const [, first, second] = parts;
    try {
      if (first === "report" && request.method === "POST") {
        return await this.#report((await request.json()) as Partial<BugReport>);
      }
      if (first === "list") {
        return await this.#list();
      }
      if (first && second === "resolve" && request.method === "POST") {
        return await this.#resolve(first, (await request.json()) as { resolution?: string });
      }
      if (first && !second) {
        const bug = await this.#state.storage.get<BugReport>(PREFIX + first);
        if (!bug) return Response.json({ error: `no bug ${first}` }, { status: 404 });
        return Response.json(bug);
      }
      return Response.json({ error: "unknown bug route" }, { status: 404 });
    } catch (cause) {
      return Response.json({ error: String(cause) }, { status: 400 });
    }
  }

  async #report(body: Partial<BugReport>): Promise<Response> {
    const description = (body.description ?? "").trim();
    if (!description) {
      return Response.json({ error: "a bug report needs a description" }, { status: 400 });
    }
    // Time-ordered ids so a plain listing reads chronologically.
    const id = `${Date.now().toString(36)}-${crypto
      .getRandomValues(new Uint32Array(1))[0]
      .toString(36)}`;
    const bug: BugReport = {
      id,
      reportedAt: new Date().toISOString(),
      status: "open",
      description,
      replay: body.replay ?? null,
      context: body.context,
    };
    await this.#state.storage.put(PREFIX + id, bug);
    return Response.json({ id });
  }

  async #list(): Promise<Response> {
    const stored = await this.#state.storage.list<BugReport>({ prefix: PREFIX });
    const bugs = [...stored.values()]
      .sort((a, b) => a.reportedAt.localeCompare(b.reportedAt))
      .map((bug) => ({
        id: bug.id,
        reportedAt: bug.reportedAt,
        status: bug.status,
        description: bug.description,
        commands: Array.isArray((bug.replay as { commands?: unknown[] } | null)?.commands)
          ? (bug.replay as { commands: unknown[] }).commands.length
          : 0,
        resolution: bug.resolution,
      }));
    return Response.json({ bugs });
  }

  async #resolve(id: string, body: { resolution?: string }): Promise<Response> {
    const bug = await this.#state.storage.get<BugReport>(PREFIX + id);
    if (!bug) return Response.json({ error: `no bug ${id}` }, { status: 404 });
    bug.status = "resolved";
    bug.resolution = body.resolution ?? "";
    bug.resolvedAt = new Date().toISOString();
    await this.#state.storage.put(PREFIX + id, bug);
    return Response.json({ id, status: bug.status });
  }
}
