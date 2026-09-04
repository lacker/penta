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
 * Nothing stores authoritative engine state. A game at rest is its
 * configuration plus every command applied -- human and bot alike -- and is
 * rebuilt by replay, refused loudly across a replay-format or
 * exact-simulation change. The last human-visible projection is cached only
 * so a reconnect cannot observe a private opponent decision during replay.
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
  artPreference?: "debut" | "format-matching";
  /**
   * The human seat's own opt-in to open decklists: it is willing to have
   * `humanDeck` named to a bot opponent who has also opted in. Off by
   * default. Not passed to `WebGame` -- it plays no part in the deal, only
   * in what `#withOpponentDeck` later adds to the bot's observation.
   */
  humanDiscloseDeck?: boolean;
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
  /**
   * The bot seat's own opt-in to open decklists, learned from the registry
   * once the bot that actually claimed the seat is known -- unlike
   * `config.humanDiscloseDeck`, this cannot be declared at `start`, since
   * whichever bot the room's starter goes on to invite is not yet decided.
   * See `disclose-bot-deck` below.
   */
  botDiscloseDeck?: boolean;
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

/** A human snapshot from a boundary at which that seat may actually act. */
interface HumanStateCache {
  /** Number of replay commands already reflected in `state`. */
  commandCount: number;
  state: Record<string, unknown> & {
    opponentActions?: unknown[];
    result?: unknown;
  };
}

import { FINISHED_ROOM_MS, moveBudgetMs, waitBudgetMs } from "./bot-presence.mjs";

const STORED = "hosted-game";
const HUMAN_STATE = "human-state";

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
      // The reason travels with the command, so a replay of this room ends
      // with the same account of the ending that the player was given.
      game.loseOnTime(command.seat, command.reason);
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
  /**
   * When the bot seat last showed up here: connected, polled, or moved.
   *
   * Deliberately memory-only. Its one reader asks whether the bot has been
   * here within a presence window, and a bot that has been here within a
   * presence window has kept this object loaded -- so an instance with no
   * value is an instance nothing has talked to, which is the same answer.
   * Persisting it would put a storage write on the poll loop, which is the
   * chattiest path in the room, to learn nothing new.
   */
  #lastBotSeen: number | null = null;
  /**
   * Bots parked on `opponent`, each waiting to be told the board moved.
   *
   * A parked request holds no storage operation, so the object goes on
   * serving everyone else while these wait -- which is the whole point:
   * the bot is asleep rather than asking again every quarter second.
   */
  readonly #parked = new Set<() => void>();
  /** Last state it was safe to render to the human seat. */
  #humanState: HumanStateCache | null = null;

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
      if (route === "bot-activity") {
        // Object-to-object only, like `lose-on-time`: the registry asks this
        // before it concludes that a bot has abandoned this room. Answered
        // before `#load()` on purpose -- it needs no game, and a registry
        // poll should not pay to replay one.
        return Response.json({ lastSeen: this.#lastBotSeen });
      }
      const game = await this.#load();
      if (!game) return Response.json({ error: "no game here yet" }, { status: 404 });
      const seat = this.#seatFor(presented);
      // Presenting the bot seat's token is the bot being here. Polling
      // counts as much as moving: a bot waiting out the human's turn is
      // alive, and the move clock -- not this -- is what answers a bot that
      // is running but wedged.
      if (seat === "bot") this.#lastBotSeen = Date.now();
      if (route === "ws") {
        const role = url.searchParams.get("role") ?? "human";
        if (seat !== role) return forbidden();
        return this.#connect(game, role);
      }
      // The human's snapshot holds their hand, and the record holds the whole
      // game; neither belongs to whoever guessed the room id.
      if (route === "state") {
        return seat === "human" ? this.#snapshot() : forbidden();
      }
      if (route === "opponent") {
        if (seat !== "bot") return forbidden();
        return await this.#opponentView(game, waitBudgetMs(url.searchParams.get("wait")));
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
      if (route === "disclose-bot-deck") {
        // Object-to-object only, like `verify-bot-token` and `lose-on-time`:
        // the registry calls this right after a challenge succeeds, once it
        // knows which bot actually claimed the seat and whether that bot
        // opted into open decklists. This alone discloses nothing -- it only
        // sets the flag `#withOpponentDeck` later reads, and only takes
        // effect once the human seat has opted in too.
        const body = (await request.json().catch(() => ({}))) as {
          discloseDeck?: boolean;
        };
        await this.#setBotDiscloseDeck(body.discloseDeck === true);
        return Response.json({ ok: true });
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
        return this.#snapshot();
      }
      if (route === "command") {
        const command = (await request.json()) as Command;
        const speaker = BOT_COMMANDS.has(command.t) ? "bot" : "human";
        if (seat !== speaker) return forbidden();
        await this.#apply(game, command);
        return seat === "bot"
          ? Response.json({ ok: true })
          : this.#snapshot();
      }
      if (route === "record") {
        if (seat !== "human") return forbidden();
        const stored = this.#stored;
        if (!stored) return Response.json({ error: "no game" }, { status: 404 });
        // A replay is useful to the human, but the bearer credentials that
        // authorize both seats never are. Keep the response allowlisted so a
        // future stored secret is private unless deliberately made public.
        const record = {
          config: stored.config,
          commands: stored.commands,
          replayVersion: stored.replayVersion,
          simulationFingerprint: stored.simulationFingerprint,
          engineVersion: stored.engineVersion,
          protocolVersion: stored.protocolVersion,
        };
        if (stored.config.botPolicy.toLowerCase() !== "external") {
          return Response.json(record);
        }
        // An external game's seed is both hands, and its command journal can
        // record a private choice that produced no public event. Neither is
        // a live-game observation: withhold both until the result makes the
        // complete replay ordinary post-game provenance.
        return Response.json(
          game.isFinished()
            ? record
            : {
                ...record,
                config: { ...stored.config, seed: null },
                commands: [],
              },
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
  async #opponentView(game: WebGame, waitMs: number): Promise<Response> {
    // A bot that asked to wait is held here until its turn arrives rather
    // than sent away to ask again. What it costs a poller is a whole poll
    // interval per decision, and a game asks the opponent seat for many more
    // decisions than a person would guess -- every priority pass is one.
    if (waitMs > 0) {
      const deadline = Date.now() + waitMs;
      while (!game.opponentIsDeciding() && !game.isFinished()) {
        const remaining = deadline - Date.now();
        if (remaining <= 0) break;
        if (!(await this.#park(remaining))) break;
      }
      // Parking is the bot being here, for as long as it is parked: it is
      // not going to say anything else while it waits.
      this.#lastBotSeen = Date.now();
    }
    // `resultJson` rather than the whole state: this runs on every poll a
    // bot makes, including the many that only ask whether it is their turn
    // yet, and the state it used to build for one member grows all game.
    const result = this.#result(game);
    if (!game.opponentIsDeciding()) {
      return Response.json({ deciding: false, result });
    }
    return Response.json({
      deciding: true,
      result,
      observation: this.#withOpponentDeck(JSON.parse(game.opponentObserveJson())),
    });
  }

  /** The finished game's result, or null while it is live. */
  #result(game: WebGame): unknown {
    const result = game.resultJson();
    return result === undefined ? null : JSON.parse(result);
  }

  /**
   * Open decklists: adds `opponentDeck`, naming the human seat's deck, to a
   * bot's observation -- but only once both seats have opted in for this
   * particular room. Absent that mutual opt-in this is the identity
   * function, so every existing bot's observation is byte-for-byte what
   * `protocol.rs` already produces. This lives here, not in the engine: it
   * is meta-game information a hosted room happens to know (who is playing
   * what), the same way an announced paper-Magic decklist is, not something
   * observable from the board `PlayerObservation` models.
   */
  #withOpponentDeck(observation: Record<string, unknown>): Record<string, unknown> {
    const stored = this.#stored;
    if (!stored?.config.humanDiscloseDeck || !stored.botDiscloseDeck) return observation;
    return { ...observation, opponentDeck: stored.config.humanDeck };
  }

  /** Records whether the bot that just claimed the seat opted into open decklists. */
  async #setBotDiscloseDeck(discloseDeck: boolean): Promise<void> {
    const stored = this.#stored;
    if (!stored) return;
    stored.botDiscloseDeck = discloseDeck;
    await this.#state.storage.put(STORED, stored);
  }

  /** Which seat a presented token speaks for, if any. */
  #seatFor(presented: string | null): "human" | "bot" | null {
    const stored = this.#stored;
    if (!stored) return null;
    if (tokenMatches(stored.humanToken, presented)) return "human";
    if (tokenMatches(stored.botToken, presented)) return "bot";
    return null;
  }

  /** The last human-safe snapshot, for polling callers; beats are not consumed. */
  #snapshot(): Response {
    if (!this.#humanState) {
      throw new Error("the human-visible game state is unavailable");
    }
    return Response.json(this.#humanState.state);
  }

  /**
   * Adds the clock to a state payload. A deadline a player cannot see is a
   * trap, so the room reports it and lets the client show what it likes.
   */
  #withClock(state: HumanStateCache["state"]): HumanStateCache["state"] {
    const clock = this.#clock;
    if (!clock || state.result) {
      return state;
    }
    return { ...state, moveClock: { seat: clock.seat, deadline: clock.deadline } };
  }

  /** Captures and persists a boundary the human seat is allowed to render. */
  async #rememberHumanState(game: WebGame): Promise<void> {
    const stored = this.#stored;
    if (!stored) throw new Error("no game");
    const state = this.#withClock(JSON.parse(game.state_json()) as HumanStateCache["state"]);
    const cached = { commandCount: stored.commands.length, state };
    if (cached.commandCount !== this.#humanState?.commandCount) {
      // The wasm's beat queue begins again at each human command. Keep the
      // delivery cursor attached to the cached boundary until a new safe
      // boundary replaces it, then begin delivery for that queue at zero.
      this.#deliveredBeats = 0;
    }
    this.#humanState = cached;
    if (stored.config.botPolicy.toLowerCase() === "external") {
      await this.#state.storage.put(HUMAN_STATE, cached);
    }
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
    const externalOpponent = config.botPolicy.toLowerCase() === "external";
    if (externalOpponent) {
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
      config.artPreference,
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
    // No command has run yet, so this initial pregame view predates any
    // card-dependent private window. It is the safe fallback if the external
    // seat owns the first mulligan decision.
    const humanState: HumanStateCache = {
      commandCount: 0,
      state: JSON.parse(game.state_json()) as HumanStateCache["state"],
    };
    await this.#state.storage.put(STORED, stored);
    if (externalOpponent) {
      await this.#state.storage.put(HUMAN_STATE, humanState);
    } else {
      await this.#state.storage.delete(HUMAN_STATE);
    }
    this.#game = game;
    this.#stored = stored;
    this.#humanState = humanState;
    this.#deliveredBeats = 0;
    await this.#dispatch(game);
    // The only time either token is ever sent. The starter keeps the human's
    // and hands the bot's to whichever bot it invites.
    return Response.json({
      state: this.#humanState?.state ?? humanState.state,
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
      const externalOpponent = stored.config.botPolicy.toLowerCase() === "external";
      const persistedHumanState = externalOpponent
        ? await this.#state.storage.get<HumanStateCache>(HUMAN_STATE)
        : undefined;
      this.#clock = (await this.#state.storage.get<MoveClock>(CLOCK)) ?? null;
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
        stored.config.artPreference,
      );
      // Replaying also recovers the last safe boundary for rooms created
      // before the cache existed, or after a storage write was interrupted.
      let replayedHumanState: HumanStateCache = {
        commandCount: 0,
        state: JSON.parse(game.state_json()) as HumanStateCache["state"],
      };
      for (const [position, command] of stored.commands.entries()) {
        try {
          apply(game, command);
          if (!game.opponentIsDeciding()) {
            replayedHumanState = {
              commandCount: position + 1,
              state: JSON.parse(game.state_json()) as HumanStateCache["state"],
            };
          }
        } catch (cause) {
          return {
            refused:
              `command ${position} of ${stored.commands.length} ` +
              `(${command.t}) no longer applies: ${String(cause)}`,
          };
        }
      }
      if (replayedHumanState.commandCount === stored.commands.length) {
        replayedHumanState.state = this.#withClock(replayedHumanState.state);
      }
      const humanState =
        persistedHumanState?.commandCount === replayedHumanState.commandCount
          ? persistedHumanState
          : replayedHumanState;
      this.#game = game;
      this.#stored = stored;
      this.#humanState = humanState;
      if (externalOpponent && humanState !== persistedHumanState) {
        await this.#state.storage.put(HUMAN_STATE, humanState);
      }
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
      server.send(this.#deliverable());
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
    // A socket bot presents its token once, at connect, so its later moves
    // arrive as frames rather than as requests `fetch` could notice.
    this.#lastBotSeen = Date.now();
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
    // Whatever this dispatch decides, a parked bot is owed a fresh look --
    // but only once the room is done, never mid-bookkeeping.
    try {
      await this.#armClock(game);
      if (game.opponentIsDeciding()) {
        this.#promptBot(game);
        // Humans are not pushed here. Their snapshot would describe their own
        // seat as holding a decision it does not hold, and an actionable panel
        // that rejects every click is worse than a still one.
        return;
      }
      await this.#rememberHumanState(game);
      this.#toHumans(this.#deliverable());
      const result = this.#humanState?.state.result;
      if (result && this.#bot) {
        this.#bot.send(JSON.stringify({ t: "result", result }));
      }
    } finally {
      this.#wake();
    }
  }

  /**
   * Restarts the clock for whoever must move next. Every applied command
   * resets it, so the budget is per move rather than per game, and an alarm
   * is what enforces it: nobody has to be connected for a timeout to land.
   */
  async #armClock(game: WebGame): Promise<void> {
    if (game.isFinished()) {
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
    if (game.isFinished()) {
      // The retention alarm: the game ended, its keeping time is up.
      await this.#state.storage.delete(STORED);
      await this.#state.storage.delete(HUMAN_STATE);
      await this.#state.storage.delete(CLOCK);
      this.#game = null;
      this.#stored = null;
      this.#humanState = null;
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
        observation: this.#withOpponentDeck(JSON.parse(game.opponentObserveJson())),
      }),
    );
  }

  /**
   * Waits for the next change to the board, or for `timeoutMs` to pass.
   * Resolves true when something actually happened, false on the timeout,
   * so a caller can tell "look again" from "your time is up".
   */
  #park(timeoutMs: number): Promise<boolean> {
    return new Promise((resolve) => {
      const release = (moved: boolean) => {
        clearTimeout(timer);
        this.#parked.delete(wake);
        resolve(moved);
      };
      const wake = () => release(true);
      const timer = setTimeout(() => release(false), timeoutMs);
      this.#parked.add(wake);
    });
  }

  /** Releases every parked bot; the board is not what it was. */
  #wake(): void {
    const waiting = [...this.#parked];
    this.#parked.clear();
    for (const release of waiting) release();
  }

  /** The state message with only the beats nobody has been sent yet. */
  #deliverable(): string {
    if (!this.#humanState) {
      throw new Error("the human-visible game state is unavailable");
    }
    const state = { ...this.#humanState.state };
    const beats = state.opponentActions ?? [];
    state.opponentActions = beats.slice(this.#deliveredBeats);
    this.#deliveredBeats = beats.length;
    return JSON.stringify({ t: "state", state });
  }

  #toHumans(payload: string): void {
    for (const socket of this.#humans) {
      socket.send(payload);
    }
  }
}
