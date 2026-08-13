/**
 * One hosted game, owned by one Durable Object.
 *
 * The engine and the presentation layer live here: `WebGame` -- the same
 * Rust the browser runs for a local game -- produces the `state_json()` the
 * React app already renders, so the client cannot tell which side of the
 * wire the engine is on. A `Game` holds the opponent's hand and the library
 * order, which is why no game object ever leaves this room; seats get views.
 *
 * Two kinds of socket connect. A `human` socket sends the same commands the
 * local UI issues and receives state pushes. A `bot` socket is prompted with
 * the opponent seat's protocol observation whenever that seat holds the
 * decision, and answers with an action index -- the contract remote bots
 * already speak over the bindings.
 *
 * A bot that would rather not hold a socket open can poll instead: `opponent`
 * reports whether the external seat holds the decision and hands back the
 * same observation, and `command` accepts its `botAct`.
 *
 * Beats are delivered exactly once. The wasm accumulates the opponent's
 * beats until the next human command clears them, so a room that pushed
 * state twice in one window would replay the same beats twice. The room
 * remembers how many it has delivered and sends only the tail; the counter
 * resets when a human command resets the queue.
 *
 * Nothing stores engine state. A game at rest is its configuration plus
 * every command applied -- human and bot alike -- and is rebuilt by replay,
 * refused loudly across a replay-format or exact-simulation change.
 */

import { type EngineModule, engine } from "./engine";
import { replayCompatibilityError } from "./replay-compatibility.mjs";

type WebGame = InstanceType<EngineModule["WebGame"]>;

interface GameConfig {
  humanDeck: string;
  botDeck: string;
  botPolicy: string;
  humanFirst: boolean;
  seed: number;
  format?: string;
}

/** Everything either seat can do, in a form that can be written down. */
type Command =
  | { t: "act"; index: number }
  | { t: "choose"; decision: number; options: number[] }
  | { t: "attackAll" }
  | { t: "cancelAttackers" }
  | { t: "blocks"; assignments: string }
  | { t: "undoMana" }
  | { t: "phaseStop"; phase: string; enabled: boolean }
  | { t: "autopass"; enabled: boolean }
  | { t: "botAct"; index: number }
  | { t: "loseOnTime"; seat: "human" | "bot"; reason: string };

/** The bot seat's one verb; everything else belongs to the human. */
const BOT_COMMANDS = new Set(["botAct"]);

interface StoredGame {
  config: GameConfig;
  commands: Command[];
  /** Optional only for rooms stored before exact replay metadata existed. */
  replayVersion?: number;
  simulationFingerprint?: string;
  /** Package and bot-wire metadata retained for diagnostics, not replay gates. */
  engineVersion: string;
  protocolVersion: number;
  /**
   * Who may act as each seat. A room id is short and shared in a URL, so it
   * identifies a room without authorising anything; these do the authorising.
   * The human's is minted for whoever started the room, and the bot's is
   * handed to a bot through the invitation that names this room.
   */
  humanToken: string;
  botToken: string;
}

interface DurableStorage {
  get<T>(key: string): Promise<T | undefined>;
  put<T>(key: string, value: T): Promise<void>;
  delete(key: string): Promise<boolean>;
  setAlarm(time: number): Promise<void>;
  deleteAlarm(): Promise<void>;
}
interface DurableState {
  storage: DurableStorage;
  blockConcurrencyWhile<T>(callback: () => Promise<T>): Promise<T>;
}

/** The worker runtime's socket pair; typed by hand like the rest of Env. */
interface RoomSocket {
  accept(): void;
  send(message: string): void;
  close(code?: number, reason?: string): void;
  addEventListener(type: "message", handler: (event: { data: unknown }) => void): void;
  addEventListener(type: "close", handler: () => void): void;
}
declare const WebSocketPair: new () => { 0: RoomSocket; 1: RoomSocket };

/** Either the live game, a reason it cannot be rebuilt, or neither. */
interface LoadOutcome {
  game?: WebGame;
  refused?: string;
}

import { FINISHED_ROOM_MS, moveBudgetMs } from "./bot-presence.mjs";

const STORED = "hosted-game";

/** The header a seat presents its token in. */
const TOKEN_HEADER = "x-penta-token";

/**
 * One refusal for every seat mismatch, so none of them leak which seat was
 * wrong. A fresh response each call: a `Response` body is consumed on use, so
 * a shared instance would work once and then fail.
 */
function forbidden(): Response {
  return Response.json({ error: "that seat is not yours" }, { status: 403 });
}

/** Names the bearer of a seat. Long enough not to be guessed. */
function mintToken(): string {
  const bytes = crypto.getRandomValues(new Uint8Array(24));
  return [...bytes].map((byte) => byte.toString(16).padStart(2, "0")).join("");
}

/** Constant-time-ish comparison; these are short and compared rarely. */
function tokenMatches(expected: string | undefined, given: string | null): boolean {
  if (!expected || !given || expected.length !== given.length) return false;
  let difference = 0;
  for (let index = 0; index < expected.length; index += 1) {
    difference |= expected.charCodeAt(index) ^ given.charCodeAt(index);
  }
  return difference === 0;
}
const CLOCK = "move-clock";

/** Who the room is waiting on, and when their patience runs out. */
interface MoveClock {
  seat: "human" | "bot";
  deadline: number;
}

function apply(game: WebGame, command: Command): void {
  switch (command.t) {
    case "act":
      game.act(command.index);
      return;
    case "choose":
      game.choose_decision(command.decision, JSON.stringify(command.options));
      return;
    case "attackAll":
      game.attack_all();
      return;
    case "cancelAttackers":
      game.cancel_attackers();
      return;
    case "blocks":
      game.finalize_blocks(command.assignments);
      return;
    case "undoMana":
      game.undo_mana();
      return;
    case "phaseStop":
      game.set_phase_stop(command.phase, command.enabled);
      return;
    case "autopass":
      game.set_autopass(command.enabled);
      return;
    case "botAct":
      game.opponentAct(command.index);
      return;
    case "loseOnTime":
      game.loseOnTime(command.seat);
  }
}

export class GameRoom {
  readonly #state: DurableState;
  #game: WebGame | null = null;
  #stored: StoredGame | null = null;
  /** Sockets watching the human seat; all of them get state pushes. */
  readonly #humans = new Set<RoomSocket>();
  /** The external driver for the opponent seat, one at a time. */
  #bot: RoomSocket | null = null;
  /** How many of the wasm's accumulated beats have already been sent. */
  #deliveredBeats = 0;
  /** Who the room is waiting on, mirrored from storage. */
  #clock: MoveClock | null = null;

  constructor(state: DurableState) {
    this.#state = state;
  }

  async fetch(request: Request): Promise<Response> {
    const url = new URL(request.url);
    const route = url.pathname.split("/").pop();
    // A socket cannot set headers, so it carries its token in the query.
    const presented =
      request.headers.get(TOKEN_HEADER) ?? url.searchParams.get("token");
    try {
      if (route === "start") {
        return await this.#start((await request.json()) as GameConfig, presented);
      }
      const game = await this.#load();
      if (!game) return Response.json({ error: "no game here yet" }, { status: 404 });
      const seat = this.#seatFor(presented);
      if (route === "ws") {
        const role = url.searchParams.get("role") ?? "human";
        if (seat !== role) return forbidden();
        return this.#connect(game, role);
      }
      // The human's snapshot holds their hand, and the record holds the whole
      // game; neither belongs to whoever guessed the room id.
      if (route === "state") {
        return seat === "human" ? this.#snapshot(game) : forbidden();
      }
      if (route === "opponent") {
        return seat === "bot" ? this.#opponentView(game) : forbidden();
      }
      if (route === "verify-bot-token") {
        // Object-to-object only, like `lose-on-time`. The registry asks this
        // before it will point a bot at a room, so that a challenger has to
        // have started the room rather than merely named it.
        const body = (await request.json().catch(() => ({}))) as { token?: string };
        return Response.json({
          ok: tokenMatches(this.#stored?.botToken, body.token ?? null),
        });
      }
      if (route === "lose-on-time") {
        // Reachable only object-to-object: the public router refuses this
        // path, so a caller here is this room's alarm or the registry.
        const body = (await request.json().catch(() => ({}))) as {
          seat?: "human" | "bot";
          reason?: string;
        };
        if (body.seat !== "human" && body.seat !== "bot") {
          return Response.json({ error: "a timeout needs a seat" }, { status: 400 });
        }
        await this.#apply(game, {
          t: "loseOnTime",
          seat: body.seat,
          reason: body.reason ?? "ran out of time",
        });
        return this.#snapshot(game);
      }
      if (route === "command") {
        const command = (await request.json()) as Command;
        const speaker = BOT_COMMANDS.has(command.t) ? "bot" : "human";
        if (seat !== speaker) return forbidden();
        await this.#apply(game, command);
        return seat === "bot"
          ? Response.json({ ok: true })
          : this.#snapshot(game);
      }
      if (route === "record") {
        if (seat !== "human") return forbidden();
        const stored = this.#stored;
        if (!stored) return Response.json({ error: "no game" }, { status: 404 });
        if (stored.config.botPolicy.toLowerCase() !== "external") {
          return Response.json(stored);
        }
        // An external game's seed is both hands; the record hides it until
        // the game is decided, after which it is the replay's provenance.
        const finished = Boolean(
          (JSON.parse(game.state_json()) as { result?: unknown }).result,
        );
        return Response.json(
          finished
            ? stored
            : { ...stored, config: { ...stored.config, seed: null } },
        );
      }
      return Response.json({ error: `unknown route ${route}` }, { status: 404 });
    } catch (cause) {
      return Response.json({ error: String(cause) }, { status: 400 });
    }
  }

  /**
   * The external seat's view, for a bot that polls instead of holding a
   * socket. Same observation the bot socket is prompted with; the bot plays
   * it by posting `botAct` to `command`, so a whole remote bot needs only
   * these two ordinary requests.
   */
  #opponentView(game: WebGame): Response {
    const result = (JSON.parse(game.state_json()) as { result?: unknown }).result;
    if (!game.opponentIsDeciding()) {
      return Response.json({ deciding: false, result: result ?? null });
    }
    return Response.json({
      deciding: true,
      result: result ?? null,
      observation: JSON.parse(game.opponentObserveJson()),
    });
  }

  /** Which seat a presented token speaks for, if any. */
  #seatFor(presented: string | null): "human" | "bot" | null {
    const stored = this.#stored;
    if (!stored) return null;
    if (tokenMatches(stored.humanToken, presented)) return "human";
    if (tokenMatches(stored.botToken, presented)) return "bot";
    return null;
  }

  /** The full current snapshot, for polling callers; beats are not consumed. */
  #snapshot(game: WebGame): Response {
    return Response.json(this.#withClock(game, JSON.parse(game.state_json())));
  }

  /**
   * Adds the clock to a state payload. A deadline a player cannot see is a
   * trap, so the room reports it and lets the client show what it likes.
   */
  #withClock(game: WebGame, state: Record<string, unknown>): Record<string, unknown> {
    const clock = this.#clock;
    if (!clock || (JSON.parse(game.state_json()) as { result?: unknown }).result) {
      return state;
    }
    return { ...state, moveClock: { seat: clock.seat, deadline: clock.deadline } };
  }

  async #start(config: GameConfig, presented: string | null): Promise<Response> {
    // Restarting a room throws away whatever is in it, so only the seat that
    // started it may. A fresh room has nobody to ask.
    const existing =
      this.#stored ?? (await this.#state.storage.get<StoredGame>(STORED));
    if (existing && !tokenMatches(existing.humanToken, presented)) {
      return forbidden();
    }
    const { WebGame, HostedGame } = await engine();
    if (config.botPolicy.toLowerCase() === "external") {
      // Against a real opponent the seed is the deal itself: whoever picks it
      // can precompute both hands. The room rolls its own and never sends it.
      config = { ...config, seed: crypto.getRandomValues(new Uint32Array(1))[0] };
    }
    // Built before it is stored, so a bad deck is rejected rather than
    // written down as a room that can never open.
    const game = new WebGame(
      config.humanDeck,
      config.botDeck,
      config.botPolicy,
      config.humanFirst,
      config.seed,
      config.format,
    );
    const stored: StoredGame = {
      config,
      commands: [],
      replayVersion: HostedGame.replayVersion(),
      simulationFingerprint: HostedGame.simulationFingerprint(),
      engineVersion: HostedGame.engineVersion(),
      protocolVersion: HostedGame.protocolVersion(),
      // A restart re-mints, so a token handed out for the previous game does
      // not carry into this one.
      humanToken: mintToken(),
      botToken: mintToken(),
    };
    await this.#state.storage.put(STORED, stored);
    this.#game = game;
    this.#stored = stored;
    this.#deliveredBeats = 0;
    await this.#dispatch(game);
    // The only time either token is ever sent. The starter keeps the human's
    // and hands the bot's to whichever bot it invites.
    return Response.json({
      state: JSON.parse(game.state_json()),
      humanToken: stored.humanToken,
      botToken: stored.botToken,
    });
  }

  async #load(): Promise<WebGame | null> {
    if (this.#game) return this.#game;
    // A throw inside `blockConcurrencyWhile` resets the object and the caller
    // only sees an opaque 500, so a refusal comes back as a value.
    const outcome: LoadOutcome = await this.#state.blockConcurrencyWhile(async () => {
      if (this.#game) return { game: this.#game };
      const stored = await this.#state.storage.get<StoredGame>(STORED);
      if (!stored) return {};
      const { WebGame, HostedGame } = await engine();
      const replayVersion = HostedGame.replayVersion();
      const simulationFingerprint = HostedGame.simulationFingerprint();
      const refused = replayCompatibilityError(stored, {
        replayVersion,
        simulationFingerprint,
      });
      if (refused) return { refused };
      const game = new WebGame(
        stored.config.humanDeck,
        stored.config.botDeck,
        stored.config.botPolicy,
        stored.config.humanFirst,
        stored.config.seed,
        stored.config.format,
      );
      for (const [position, command] of stored.commands.entries()) {
        try {
          apply(game, command);
        } catch (cause) {
          return {
            refused:
              `command ${position} of ${stored.commands.length} ` +
              `(${command.t}) no longer applies: ${String(cause)}`,
          };
        }
      }
      this.#game = game;
      this.#stored = stored;
      // A rebuilt room delivers the live queue afresh to whoever connects.
      this.#deliveredBeats = 0;
      return { game };
    });
    if (outcome.refused) throw new Error(outcome.refused);
    return outcome.game ?? null;
  }

  async #apply(game: WebGame, command: Command): Promise<void> {
    const stored = this.#stored;
    if (!stored) throw new Error("no game");
    apply(game, command);
    if (!BOT_COMMANDS.has(command.t)) {
      // A human command resets the wasm's beat queue, so delivery starts over.
      this.#deliveredBeats = 0;
    }
    stored.commands.push(command);
    await this.#state.storage.put(STORED, stored);
    await this.#dispatch(game);
  }

  #connect(game: WebGame, role: string): Response {
    if (role !== "human" && role !== "bot") {
      return Response.json({ error: `unknown role ${role}` }, { status: 400 });
    }
    const pair = new WebSocketPair();
    const server = pair[1];
    server.accept();
    if (role === "bot") {
      this.#bot?.close(1000, "replaced by a newer driver");
      this.#bot = server;
      server.addEventListener("message", (event) => {
        void this.#onBotMessage(game, event.data);
      });
      server.addEventListener("close", () => {
        if (this.#bot === server) this.#bot = null;
      });
      if (game.opponentIsDeciding()) this.#promptBot(game);
    } else {
      this.#humans.add(server);
      server.addEventListener("message", (event) => {
        void this.#onHumanMessage(game, event.data);
      });
      server.addEventListener("close", () => {
        this.#humans.delete(server);
      });
      server.send(this.#deliverable(game));
    }
    return new Response(null, { status: 101, webSocket: pair[0] } as ResponseInit);
  }

  async #onHumanMessage(game: WebGame, data: unknown): Promise<void> {
    let command: Command;
    try {
      command = JSON.parse(String(data)) as Command;
      if (BOT_COMMANDS.has(command.t)) throw new Error("that verb belongs to the bot socket");
    } catch (cause) {
      this.#toHumans(JSON.stringify({ t: "error", message: String(cause) }));
      return;
    }
    try {
      await this.#apply(game, command);
    } catch (cause) {
      this.#toHumans(JSON.stringify({ t: "error", message: String(cause) }));
      // The command did not take; the board they see is still right.
    }
  }

  async #onBotMessage(game: WebGame, data: unknown): Promise<void> {
    try {
      const message = JSON.parse(String(data)) as { t: string; index: number };
      if (message.t !== "act") throw new Error("the bot socket speaks only act");
      await this.#apply(game, { t: "botAct", index: message.index });
    } catch (cause) {
      this.#bot?.send(JSON.stringify({ t: "error", message: String(cause) }));
    }
  }

  /**
   * After any change: either the opponent seat holds the decision and its
   * driver is prompted, or the human's view is current and gets pushed.
   */
  async #dispatch(game: WebGame): Promise<void> {
    await this.#armClock(game);
    if (game.opponentIsDeciding()) {
      this.#promptBot(game);
      // Humans are not pushed here. Their snapshot would describe their own
      // seat as holding a decision it does not hold, and an actionable panel
      // that rejects every click is worse than a still one.
      return;
    }
    this.#toHumans(this.#deliverable(game));
    const result = (JSON.parse(game.state_json()) as { result?: unknown }).result;
    if (result && this.#bot) {
      this.#bot.send(JSON.stringify({ t: "result", result }));
    }
  }

  /**
   * Restarts the clock for whoever must move next. Every applied command
   * resets it, so the budget is per move rather than per game, and an alarm
   * is what enforces it: nobody has to be connected for a timeout to land.
   */
  async #armClock(game: WebGame): Promise<void> {
    const finished = Boolean(
      (JSON.parse(game.state_json()) as { result?: unknown }).result,
    );
    if (finished) {
      this.#clock = null;
      await this.#state.storage.delete(CLOCK);
      // A decided game is worth keeping for a while -- the players may still
      // be looking at it, and a bug report may still want its replay -- but
      // not forever. The alarm comes back to release it.
      await this.#state.storage.setAlarm(Date.now() + FINISHED_ROOM_MS);
      return;
    }
    const seat: MoveClock["seat"] = game.opponentIsDeciding() ? "bot" : "human";
    const clock: MoveClock = { seat, deadline: Date.now() + moveBudgetMs(seat) };
    this.#clock = clock;
    await this.#state.storage.put(CLOCK, clock);
    await this.#state.storage.setAlarm(clock.deadline);
  }

  /**
   * The clock ran out, so the seat that stopped answering loses -- recorded
   * as an ordinary command, which keeps the room's replay complete, and
   * reported as a timeout rather than a concession, because the player did
   * not choose it.
   */
  async alarm(): Promise<void> {
    const game = await this.#load();
    if (!game) return;
    if ((JSON.parse(game.state_json()) as { result?: unknown }).result) {
      // The retention alarm: the game ended, its keeping time is up.
      await this.#state.storage.delete(STORED);
      await this.#state.storage.delete(CLOCK);
      this.#game = null;
      this.#stored = null;
      return;
    }
    const clock = this.#clock ?? (await this.#state.storage.get<MoveClock>(CLOCK));
    if (!clock) return;
    const remaining = clock.deadline - Date.now();
    if (remaining > 0) {
      // Something rearmed the clock while this alarm was in flight.
      await this.#state.storage.setAlarm(clock.deadline);
      return;
    }
    const seat: MoveClock["seat"] = game.opponentIsDeciding() ? "bot" : "human";
    if (seat !== clock.seat) {
      // They moved after all; the next dispatch owns the clock now.
      await this.#armClock(game);
      return;
    }
    await this.#apply(game, {
      t: "loseOnTime",
      seat: clock.seat,
      reason: "ran out of time",
    });
  }

  #promptBot(game: WebGame): void {
    this.#bot?.send(
      JSON.stringify({
        t: "observe",
        observation: JSON.parse(game.opponentObserveJson()),
      }),
    );
  }

  /** The state message with only the beats nobody has been sent yet. */
  #deliverable(game: WebGame): string {
    const state = JSON.parse(game.state_json()) as {
      opponentActions?: unknown[];
    };
    const beats = state.opponentActions ?? [];
    state.opponentActions = beats.slice(this.#deliveredBeats);
    this.#deliveredBeats = beats.length;
    return JSON.stringify({ t: "state", state: this.#withClock(game, state) });
  }

  #toHumans(payload: string): void {
    for (const socket of this.#humans) {
      socket.send(payload);
    }
  }
}
