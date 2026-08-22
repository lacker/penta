/**
 * The bot registry: one Durable Object holding every bot that has ever
 * registered, and which of them are online right now.
 *
 * Presence is a lease, not a connection. A bot says it is online by
 * heartbeating, and stops being online by not heartbeating -- a crashed bot
 * disappears on its own instead of lingering as something a human can click
 * and wait on forever. The heartbeat's reply carries the bot's invitations,
 * so a whole bot is a loop over one HTTP call: heartbeat, and if a room came
 * back, go play it. No socket required anywhere in the flow.
 *
 * A registered bot is playable by anyone -- humans through the web client,
 * and eventually a tournament scheduler -- and plays one game at a time. An
 * invitation it never picks up expires, so a bot that dies mid-game frees
 * itself for the next challenger.
 */

import {
  MAX_BOTS,
  PRESENCE_MS,
  isOnline,
  liveInvites,
  publicBot,
  worthKeeping,
} from "./bot-presence.mjs";
import {
  incompatibility,
  incompatibilityBody,
  parseBotCompatibility,
  parseServerCompatibility,
  publicServerCompatibility,
} from "./bot-compatibility.mjs";
import { engine } from "./engine";

interface BotCompatibility {
  protocolVersion: number;
  capabilities: string[];
  requiredCapabilities: string[];
  requiredSimulationFingerprint?: string;
}

interface ServerCompatibility extends BotCompatibility {
  requiredCapabilities: string[];
  simulationFingerprint: string;
  legacyUndeclaredProtocolVersion: number;
}

let compatibilityReady: Promise<ServerCompatibility> | null = null;
function serverCompatibility(): Promise<ServerCompatibility> {
  compatibilityReady ??= engine().then(({ HostedGame }) =>
    parseServerCompatibility(JSON.parse(HostedGame.botCompatibilityJson())),
  );
  return compatibilityReady;
}

function incompatibleResponse(
  server: ServerCompatibility,
  bot: BotCompatibility,
  exposeBot = false,
): Response {
  return Response.json(incompatibilityBody(server, bot, exposeBot), { status: 409 });
}

interface DurableStorage {
  get<T>(key: string): Promise<T | undefined>;
  put<T>(key: string, value: T): Promise<void>;
  delete(key: string): Promise<boolean>;
  list<T>(options: { prefix: string }): Promise<Map<string, T>>;
  setAlarm(time: number): Promise<void>;
}
interface DurableState {
  storage: DurableStorage;
}

/**
 * Only the game rooms: enough to make a dropped bot lose its game, verify a
 * challenger owns the room it is pointing a bot at, and tell a room whether
 * the bot claiming its seat opted into open decklists.
 */
interface RegistryEnv {
  GAME_ROOMS: {
    idFromName(name: string): unknown;
    get(id: unknown): { fetch(request: Request): Promise<Response> };
  };
}

/** A game a bot has been asked to play, and has not finished or dropped. */
interface Invite {
  room: string;
  /** Who asked: a human clicking, or a scheduler pairing a round. */
  reason: "challenge" | "event";
  at: number;
  /**
   * The room's bot-seat token, handed on to the bot. Presenting it is how a
   * challenger proves it started the room it is pointing this bot at --
   * without which anyone could park every listed bot in a room of their own
   * and keep them all busy.
   */
  token: string;
}

interface RegisteredBot {
  id: string;
  /** Held by the bot; proves a heartbeat is really from it. */
  token: string;
  name: string;
  /** What it plays when a challenger does not choose for it. */
  deck: string;
  /**
   * Opts into open decklists: this bot is willing to have its own deck named
   * to an opponent who has also opted in. Off by default, so an existing bot
   * that never sets this sees no behavior change at all. Absent only on
   * registrations stored before this field existed, same as `compatibility`.
   * See `game-room.ts` for where the mutual opt-in actually takes effect.
   */
  discloseDeck?: boolean;
  registeredAt: string;
  /** When it last heartbeated. Presence is this, and nothing else. */
  lastSeen: number;
  invites: Invite[];
  /** Absent only on registrations stored before compatibility negotiation. */
  compatibility?: BotCompatibility;
}

/** What a bot looks like from outside: no token, presence resolved. */
interface PublicBot {
  id: string;
  name: string;
  deck: string;
  discloseDeck: boolean;
  online: boolean;
  busy: boolean;
}

const PREFIX = "bot:";
/** A display field, not an identifier; long names are simply cut. */
const MAX_NAME = 40;

function requestObject(value: unknown): Record<string, unknown> {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new Error("request body must be an object");
  }
  return value as Record<string, unknown>;
}

function stringArray(value: unknown, field: string): string[] {
  if (value === undefined) return [];
  if (!Array.isArray(value) || value.some((item) => typeof item !== "string")) {
    throw new Error(`${field} must be an array of strings`);
  }
  return value;
}

function mintToken(): string {
  const bytes = crypto.getRandomValues(new Uint8Array(24));
  return [...bytes].map((byte) => byte.toString(16).padStart(2, "0")).join("");
}

function identifier(): string {
  return `${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 8)}`;
}

export class BotRegistry {
  readonly #state: DurableState;
  readonly #env: RegistryEnv;

  constructor(state: DurableState, env: RegistryEnv) {
    this.#state = state;
    this.#env = env;
  }

  async fetch(request: Request): Promise<Response> {
    const url = new URL(request.url);
    const parts = url.pathname.split("/").filter(Boolean);
    // Shapes: /_bots, /_bots/register, /_bots/<id>/heartbeat,
    // /_bots/<id>/challenge.
    const [, first, second] = parts;
    try {
      if (!first) return await this.#list();
      if (first === "register" && request.method === "POST") {
        return await this.#register(requestObject(await request.json()));
      }
      if (second === "heartbeat" && request.method === "POST") {
        return await this.#heartbeat(
          first,
          requestObject(await request.json().catch(() => ({}))),
        );
      }
      if (second === "challenge" && request.method === "POST") {
        return await this.#challenge(first, requestObject(await request.json()));
      }
      return Response.json({ error: `unknown route ${url.pathname}` }, { status: 404 });
    } catch (cause) {
      return Response.json({ error: String(cause) }, { status: 400 });
    }
  }

  async #register(body: Record<string, unknown>): Promise<Response> {
    const name = (typeof body.name === "string" ? body.name : "")
      .trim()
      .slice(0, MAX_NAME);
    if (!name) return Response.json({ error: "a bot needs a name" }, { status: 400 });
    const server = await serverCompatibility();
    const compatibility = parseBotCompatibility(body.compatibility, server);
    if (!incompatibility(server, compatibility).compatible) {
      return incompatibleResponse(server, compatibility, true);
    }
    // Sweeping first means the cap counts bots anyone could actually play,
    // not every name ever registered.
    const remaining = await this.#evictStale();
    if (remaining >= MAX_BOTS) {
      return Response.json(
        { error: "the registry is full; try again later" },
        { status: 503 },
      );
    }
    const bot: RegisteredBot = {
      id: identifier(),
      token: mintToken(),
      name,
      deck: (typeof body.deck === "string" ? body.deck : "").trim() || "Sligh",
      discloseDeck: body.discloseDeck === true,
      registeredAt: new Date().toISOString(),
      // Registering is not being online: the first heartbeat is.
      lastSeen: 0,
      invites: [],
      compatibility,
    };
    await this.#state.storage.put(PREFIX + bot.id, bot);
    return Response.json({
      id: bot.id,
      token: bot.token,
      deck: bot.deck,
      discloseDeck: bot.discloseDeck,
      compatibility: publicServerCompatibility(server),
    });
  }

  /**
   * Renews presence and hands back the bot's outstanding invitations. A bot
   * reports the rooms it has finished with `done`, which is also how it frees
   * itself for the next challenger.
   */
  async #heartbeat(
    id: string,
    body: Record<string, unknown>,
  ): Promise<Response> {
    const existing = await this.#state.storage.get<RegisteredBot>(PREFIX + id);
    if (!existing) return Response.json({ error: `no bot ${id}` }, { status: 404 });
    if (body.token !== existing.token) {
      return Response.json({ error: "wrong token" }, { status: 403 });
    }
    const server = await serverCompatibility();
    // The first lazy WASM load can yield. Re-read after it so a concurrent
    // challenge or heartbeat cannot be overwritten from a stale snapshot.
    const bot = await this.#state.storage.get<RegisteredBot>(PREFIX + id);
    if (!bot) return Response.json({ error: `no bot ${id}` }, { status: 404 });
    if (body.token !== bot.token) {
      return Response.json({ error: "wrong token" }, { status: 403 });
    }
    const compatibility = parseBotCompatibility(
      Object.hasOwn(body, "compatibility") ? body.compatibility : bot.compatibility,
      server,
    );
    if (!incompatibility(server, compatibility).compatible) {
      return incompatibleResponse(server, compatibility, true);
    }
    const now = Date.now();
    const done = new Set(stringArray(body.done, "done"));
    bot.lastSeen = now;
    bot.invites = liveInvites(bot.invites, now).filter(
      (invite) => !done.has(invite.room),
    );
    bot.compatibility = compatibility;
    // Optional, and left as whatever it already was when this heartbeat does
    // not mention it -- a bot that only sometimes echoes it back should not
    // be toggled off by omission.
    if (Object.hasOwn(body, "discloseDeck")) {
      bot.discloseDeck = body.discloseDeck === true;
    }
    await this.#state.storage.put(PREFIX + id, bot);
    return Response.json({
      invites: bot.invites,
      deck: bot.deck,
      discloseDeck: bot.discloseDeck,
      compatibility: publicServerCompatibility(server),
    });
  }

  /** Asks an online, idle bot to play a room that has already been started. */
  async #challenge(
    id: string,
    body: Record<string, unknown>,
  ): Promise<Response> {
    const room = (typeof body.room === "string" ? body.room : "").trim();
    if (!room) return Response.json({ error: "a challenge needs a room" }, { status: 400 });
    const token = (typeof body.token === "string" ? body.token : "").trim();
    if (!token) {
      return Response.json(
        { error: "a challenge needs the room's bot token" },
        { status: 400 },
      );
    }
    const reason = body.reason ?? "challenge";
    if (reason !== "challenge" && reason !== "event") {
      return Response.json({ error: "unknown challenge reason" }, { status: 400 });
    }
    const existing = await this.#state.storage.get<RegisteredBot>(PREFIX + id);
    if (!existing) return Response.json({ error: `no bot ${id}` }, { status: 404 });
    // The room is the only thing that knows its own token, so ask it. A
    // challenger who cannot produce it did not start the room, and pointing
    // bots at rooms you do not own is how you would keep every bot busy.
    if (!(await this.#roomAccepts(room, token))) {
      return Response.json(
        { error: "that is not this room's bot token" },
        { status: 403 },
      );
    }
    const now = Date.now();
    const server = await serverCompatibility();
    // Verifying the room calls another Durable Object and can yield. Reload
    // so a heartbeat declaration or another challenge that landed meanwhile
    // cannot be overwritten from the stale pre-verification snapshot.
    const bot = await this.#state.storage.get<RegisteredBot>(PREFIX + id);
    if (!bot) return Response.json({ error: `no bot ${id}` }, { status: 404 });
    const compatibility = parseBotCompatibility(bot.compatibility, server);
    if (!incompatibility(server, compatibility).compatible) {
      return incompatibleResponse(server, compatibility);
    }
    bot.invites = liveInvites(bot.invites, now);
    if (!isOnline(bot.lastSeen, now)) {
      return Response.json({ error: `${bot.name} is offline` }, { status: 409 });
    }
    if (bot.invites.length > 0) {
      return Response.json({ error: `${bot.name} is already playing` }, { status: 409 });
    }
    bot.invites.push({ room, reason, at: now, token });
    await this.#state.storage.put(PREFIX + id, bot);
    // Someone is now waiting on this bot, so start watching whether it is
    // still here. Nothing else in the registry runs on its own.
    await this.#state.storage.setAlarm(now + PRESENCE_MS);
    // Tell the room whether the bot that just claimed the seat opted into
    // open decklists -- the registry is the only trustworthy source for that,
    // since it is this bot's own declaration and not something a challenger
    // could assert on its behalf. Best-effort: a room that cannot be told
    // simply plays the redacted observation it always has.
    await this.#declareDisclosure(room, bot.discloseDeck === true);
    return Response.json({ room, deck: bot.deck, discloseDeck: bot.discloseDeck === true });
  }

  /**
   * Records, on the room itself, whether the bot now filling the seat opted
   * into open decklists. Called only once a challenge has actually succeeded,
   * so a bot that never gets to play never leaves a stray declaration behind.
   */
  async #declareDisclosure(room: string, discloseDeck: boolean): Promise<void> {
    try {
      const stub = this.#env.GAME_ROOMS.get(this.#env.GAME_ROOMS.idFromName(room));
      await stub.fetch(
        new Request(`https://room/_game/${room}/disclose-bot-deck`, {
          method: "POST",
          headers: { "content-type": "application/json" },
          body: JSON.stringify({ discloseDeck }),
        }),
      );
    } catch {
      // The room may be finished or gone; there is no game left to disclose
      // anything about either way.
    }
  }

  /**
   * Checks every bot that owes somebody a game. A bot that has stopped
   * heartbeating has abandoned its opponent, so its room is told to end the
   * game against it -- a human should not sit waiting out a move clock for a
   * process that is gone.
   *
   * The move clock in the room is the backstop for a bot that is still alive
   * but wedged; this is the faster, more specific answer for one that is not.
   */
  async alarm(): Promise<void> {
    const stored = await this.#state.storage.list<RegisteredBot>({ prefix: PREFIX });
    const now = Date.now();
    let watching = false;
    for (const bot of stored.values()) {
      const invites = liveInvites(bot.invites, now);
      if (invites.length === 0) {
        if (invites.length !== bot.invites.length) {
          bot.invites = invites;
          await this.#state.storage.put(PREFIX + bot.id, bot);
        }
        continue;
      }
      if (isOnline(bot.lastSeen, now)) {
        watching = true;
        continue;
      }
      for (const invite of invites) {
        await this.#loseOnTime(invite.room, `${bot.name} stopped answering`);
      }
      bot.invites = [];
      await this.#state.storage.put(PREFIX + bot.id, bot);
    }
    await this.#evictStale();
    if (watching) await this.#state.storage.setAlarm(now + PRESENCE_MS);
  }

  /** Whether `room` agrees that `token` is its bot seat's. */
  async #roomAccepts(room: string, token: string): Promise<boolean> {
    try {
      const stub = this.#env.GAME_ROOMS.get(this.#env.GAME_ROOMS.idFromName(room));
      const reply = await stub.fetch(
        new Request(`https://room/_game/${room}/verify-bot-token`, {
          method: "POST",
          headers: { "content-type": "application/json" },
          body: JSON.stringify({ token }),
        }),
      );
      if (!reply.ok) return false;
      const { ok } = (await reply.json()) as { ok?: boolean };
      return ok === true;
    } catch {
      return false;
    }
  }

  /** Tells a room its bot is gone. A room that has already finished says so. */
  async #loseOnTime(room: string, reason: string): Promise<void> {
    try {
      const stub = this.#env.GAME_ROOMS.get(this.#env.GAME_ROOMS.idFromName(room));
      await stub.fetch(
        new Request(`https://room/_game/${room}/lose-on-time`, {
          method: "POST",
          headers: { "content-type": "application/json" },
          body: JSON.stringify({ seat: "bot", reason }),
        }),
      );
    } catch {
      // The room may be finished or gone; the invite is dropped either way.
    }
  }

  /**
   * Deletes registrations nobody has used inside the retention window,
   * returning how many are left. A registration is a name and a token; there
   * is no reason to keep one for a bot that stopped coming back.
   */
  async #evictStale(): Promise<number> {
    const stored = await this.#state.storage.list<RegisteredBot>({ prefix: PREFIX });
    const now = Date.now();
    let kept = 0;
    for (const bot of stored.values()) {
      if (worthKeeping(bot, now)) {
        kept += 1;
        continue;
      }
      await this.#state.storage.delete(PREFIX + bot.id);
    }
    return kept;
  }

  async #list(): Promise<Response> {
    const stored = await this.#state.storage.list<RegisteredBot>({ prefix: PREFIX });
    const now = Date.now();
    const server = await serverCompatibility();
    const bots: PublicBot[] = [];
    for (const bot of stored.values()) {
      // An offline bot is not worth listing: nobody can play it, and its
      // registration is a private detail of whoever runs it.
      if (!isOnline(bot.lastSeen, now)) continue;
      const compatibility = parseBotCompatibility(bot.compatibility, server);
      if (!incompatibility(server, compatibility).compatible) continue;
      bots.push(publicBot(bot, now));
    }
    bots.sort((left, right) => left.name.localeCompare(right.name));
    return Response.json({ compatibility: publicServerCompatibility(server), bots });
  }
}
