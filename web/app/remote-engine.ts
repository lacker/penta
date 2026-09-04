/**
 * The hosted counterpart of the local wasm game.
 *
 * The React app's whole contract with an engine is a handful of synchronous
 * command methods and one `state_json()` read, and this class keeps that
 * shape over a WebSocket: commands are fired at the room, `state_json()`
 * returns the last snapshot the room pushed, and each push asks the app to
 * re-read. There is deliberately no game object on this side of the wire --
 * a `Game` holds the opponent's hand, and this client only ever holds views.
 *
 * A command therefore does not change `state_json()` synchronously. The
 * app's refresh de-duplicates on the raw snapshot string, so the stale
 * re-read right after a command is a no-op and the real update lands when
 * the push arrives.
 */

import type { CardArtPreference } from "./card-art-mode";

export type RemoteConfig = {
  gameId: string;
  format: string;
  artPreference: CardArtPreference;
  humanDeck: string;
  botDeck: string;
  botPolicy: string;
  humanFirst: boolean;
  seed: number;
  /** Called after every pushed snapshot; the app re-reads `state_json`. */
  onUpdate: () => void;
  /** Room-reported failures, which a synchronous try/catch cannot see. */
  onError: (message: string) => void;
};

type RoomMessage =
  | { t: "state"; state: unknown }
  | { t: "error"; message: string };

export class RemoteEngineGame {
  #socket: WebSocket;
  #state: string;
  #humanToken = "";
  /** Handed to a bot when challenging it; it authorises the opponent seat. */
  botToken = "";
  #config: RemoteConfig;

  private constructor(socket: WebSocket, state: string, config: RemoteConfig) {
    this.#socket = socket;
    this.#state = state;
    this.#config = config;
    socket.addEventListener("message", (event) => {
      let message: RoomMessage;
      try {
        message = JSON.parse(String(event.data)) as RoomMessage;
      } catch {
        return;
      }
      if (message.t === "state") {
        this.#state = JSON.stringify(message.state);
        config.onUpdate();
      } else if (message.t === "error") {
        config.onError(message.message);
      }
    });
    socket.addEventListener("close", (event) => {
      if (!event.wasClean) {
        config.onError("lost the connection to the hosted game");
      }
    });
  }

  /** Starts (or restarts) the room's game, then joins it as the human seat. */
  static async connect(config: RemoteConfig): Promise<RemoteEngineGame> {
    const base = `/_game/${encodeURIComponent(config.gameId)}`;
    const response = await fetch(`${base}/start`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({
        format: config.format,
        artPreference: config.artPreference,
        humanDeck: config.humanDeck,
        botDeck: config.botDeck,
        botPolicy: config.botPolicy,
        humanFirst: config.humanFirst,
        seed: config.seed,
      }),
    });
    const body = await response.text();
    if (!response.ok) {
      throw new Error(`the room refused to start: ${body}`);
    }
    // The one time the room sends its tokens. The human's stays in this tab
    // and authorises this seat; the bot's is what a challenge hands on.
    const opened = JSON.parse(body) as {
      state: unknown;
      humanToken: string;
      botToken: string;
    };
    const state = JSON.stringify(opened.state);
    const protocol = window.location.protocol === "https:" ? "wss:" : "ws:";
    const socket = new WebSocket(
      `${protocol}//${window.location.host}${base}/ws?role=human&token=${encodeURIComponent(opened.humanToken)}`,
    );
    await new Promise<void>((resolve, reject) => {
      socket.addEventListener("open", () => resolve(), { once: true });
      socket.addEventListener("error", () => reject(new Error("could not join the room")), {
        once: true,
      });
    });
    const game = new RemoteEngineGame(socket, state, config);
    game.#humanToken = opened.humanToken;
    game.botToken = opened.botToken;
    return game;
  }

  #send(command: object): void {
    if (this.#socket.readyState !== WebSocket.OPEN) {
      this.#config.onError("the hosted game is not connected");
      return;
    }
    this.#socket.send(JSON.stringify(command));
  }

  act(index: number): void {
    this.#send({ t: "act", index });
  }

  choose_decision(decision: number, optionsJson: string): void {
    this.#send({ t: "choose", decision, options: JSON.parse(optionsJson) as number[] });
  }

  attack_all(): void {
    this.#send({ t: "attackAll" });
  }

  cancel_attackers(): void {
    this.#send({ t: "cancelAttackers" });
  }

  finalize_blocks(assignmentsJson: string): void {
    this.#send({ t: "blocks", assignments: assignmentsJson });
  }

  undo_mana(): void {
    this.#send({ t: "undoMana" });
  }

  set_phase_stop(phase: string, enabled: boolean): void {
    this.#send({ t: "phaseStop", phase, enabled });
  }

  set_autopass(enabled: boolean): void {
    this.#send({ t: "autopass", enabled });
  }

  state_json(): string {
    return this.#state;
  }

  /** Headers that prove this tab holds the human seat. */
  humanHeaders(): Record<string, string> {
    return { "x-penta-token": this.#humanToken };
  }

  free(): void {
    this.#socket.close(1000, "the tab moved on");
  }
}
