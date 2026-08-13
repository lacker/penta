/** Cloudflare Worker entry point for the vinext-starter template. */
import { engine } from "./engine";

/** Plays a game to its end inside the Worker and reports what happened. */
async function selfCheck(): Promise<Response> {
  const { HostedGame } = await engine();
  const game = new HostedGame("old-school-93-94", "Sligh", "The Deck", "4242");
  for (let step = 0; step < 20000; step += 1) {
    const seat = game.decisionSeat();
    if (!seat) break;
    const observation = JSON.parse(game.observeJson(seat)) as {
      legalActions: { type: string }[];
    };
    if (observation.legalActions.length === 0) break;
    const index = observation.legalActions.findIndex(
      (action) => action.type !== "PassPriority" && action.type !== "Concede",
    );
    game.act(index === -1 ? 0 : index);
  }
  const history = JSON.parse(game.historyJson()) as number[];
  // The whole point of storing actions: rebuild and check it lands identically.
  const rebuilt = HostedGame.replay(
    "old-school-93-94",
    "Sligh",
    "The Deck",
    "4242",
    new Uint32Array(history),
  );
  return Response.json({
    engineVersion: HostedGame.engineVersion(),
    protocolVersion: HostedGame.protocolVersion(),
    simulationFingerprint: HostedGame.simulationFingerprint(),
    replayVersion: HostedGame.replayVersion(),
    actions: history.length,
    result: game.resultJson() ? JSON.parse(game.resultJson()!) : null,
    replayMatches: rebuilt.historyJson() === game.historyJson()
      && rebuilt.resultJson() === game.resultJson(),
  });
}
import { handleImageOptimization, DEFAULT_DEVICE_SIZES, DEFAULT_IMAGE_SIZES } from "vinext/server/image-optimization";
import handler from "vinext/server/app-router-entry";

interface AssetBinding {
  fetch(input: RequestInfo | URL, init?: RequestInit): Promise<Response>;
}

interface DurableObjectNamespace {
  idFromName(name: string): unknown;
  get(id: unknown): { fetch(request: Request): Promise<Response> };
}

/** Cloudflare's rate limiter: one call, one verdict, no counters of ours. */
interface RateLimiter {
  limit(options: { key: string }): Promise<{ success: boolean }>;
}

interface Env {
  ASSETS: AssetBinding;
  GAME_ROOMS: DurableObjectNamespace;
  BUGS: DurableObjectNamespace;
  BOTS: DurableObjectNamespace;
  /** Guards the routes that create things: rooms, registrations, invitations. */
  CREATE_LIMIT?: RateLimiter;
  /**
   * Set to `enabled` to serve the server-side game routes: hosted games and
   * the bot registry. Each seat of a room is held by a token, and the routes
   * that create things are rate limited, so this is safe to serve publicly.
   */
  HOSTED_GAMES?: string;
  /**
   * Set to `enabled` to serve `/_engine/self-check`, which plays a full game
   * inside the Worker. Useful when bringing up a deployment, unmetered, and
   * of no use to a player -- so it is separate and stays off.
   */
  ENGINE_SELF_CHECK?: string;
  // Reserved for a future Sites database binding; this project currently has none.
  DB: unknown;
  IMAGES: {
    input(stream: ReadableStream): {
      transform(options: Record<string, unknown>): {
        output(options: { format: string; quality: number }): Promise<{ response(): Response }>;
      };
    };
  };
}

interface ExecutionContext {
  waitUntil(promise: Promise<unknown>): void;
  passThroughOnException(): void;
}

// Image security config. SVG sources with .svg extension auto-skip the
// optimization endpoint on the client side (served directly, no proxy).
// To route SVGs through the optimizer (with security headers), set
// dangerouslyAllowSVG: true in next.config.js and uncomment below:
// const imageConfig: ImageConfig = { dangerouslyAllowSVG: true };

/** Room routes a client may reach. See the note where they are dispatched. */
const PUBLIC_ROOM_ROUTES = new Set([
  "start",
  "ws",
  "state",
  "opponent",
  "command",
  "record",
]);

/**
 * Routes that make new state, and so are worth a limit: a room, a
 * registration, an invitation. Reads and moves are not limited here -- a game
 * in progress is chatty by nature, and the move clock already bounds it.
 */
function createsState(pathname: string): boolean {
  return (
    pathname === "/_bots/register" ||
    pathname.endsWith("/challenge") ||
    /^\/_game\/[^/]+\/start$/.test(pathname)
  );
}

const worker = {
  async fetch(request: Request, env: Env, ctx: ExecutionContext): Promise<Response> {
    const url = new URL(request.url);

    const hostedGames = env.HOSTED_GAMES === "enabled";

    // Keyed by client IP. Cloudflare sets this header itself, so a caller
    // cannot spoof its way into a fresh bucket.
    if (hostedGames && env.CREATE_LIMIT && createsState(url.pathname)) {
      const key = request.headers.get("cf-connecting-ip") ?? "unknown";
      const { success } = await env.CREATE_LIMIT.limit({ key });
      if (!success) {
        return Response.json(
          { error: "too many requests; slow down" },
          { status: 429, headers: { "retry-after": "60" } },
        );
      }
    }

    // Proof that the engine runs server-side, not a product endpoint: it
    // plays a whole game per request. Its own flag, so serving games in
    // public does not also hand out a CPU-heavy endpoint nobody needs.
    if (url.pathname === "/_engine/self-check") {
      return env.ENGINE_SELF_CHECK === "enabled"
        ? selfCheck()
        : new Response("not found", { status: 404 });
    }

    // /_game/<id>/<route>. The id names the Durable Object, so every request
    // for one game reaches the same instance and the engine has a single
    // writer. Only these routes are reachable from outside: `lose-on-time`
    // deliberately is not, which is what makes an object-to-object call from
    // the room's own alarm or the bot registry distinguishable from a
    // stranger ending someone's game.
    const room = hostedGames ? url.pathname.match(/^\/_game\/([^/]+)\/([^/]+)$/) : null;
    if (room && PUBLIC_ROOM_ROUTES.has(room[2])) {
      const stub = env.GAME_ROOMS.get(env.GAME_ROOMS.idFromName(room[1]));
      return stub.fetch(request);
    }

    // The bot registry: one object for the whole deployment, holding who is
    // online. `/_bots` lists them; the rest is register, heartbeat, challenge.
    if (hostedGames && (url.pathname === "/_bots" || url.pathname.startsWith("/_bots/"))) {
      const stub = env.BOTS.get(env.BOTS.idFromName("bots"));
      return stub.fetch(request);
    }

    // The bug ledger: one object for the whole deployment.
    if (hostedGames && url.pathname.startsWith("/_bugs/")) {
      const stub = env.BUGS.get(env.BUGS.idFromName("bugs"));
      return stub.fetch(request);
    }

    if (url.pathname === "/_vinext/image") {
      const allowedWidths = [...DEFAULT_DEVICE_SIZES, ...DEFAULT_IMAGE_SIZES];
      return handleImageOptimization(request, {
        fetchAsset: (path) => env.ASSETS.fetch(new Request(new URL(path, request.url))),
        transformImage: async (body, { width, format, quality }) => {
          const result = await env.IMAGES.input(body).transform(width > 0 ? { width } : {}).output({ format, quality });
          return result.response();
        },
      }, allowedWidths);
    }

    return handler.fetch(request, env, ctx);
  },
};

export { GameRoom } from "./game-room";
export { BugTracker } from "./bug-tracker";
export { BotRegistry } from "./bot-registry";

export default worker;
