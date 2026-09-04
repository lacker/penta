import initWasm, { WebGame as RustWebGame } from "./wasm/penta_wasm.js";
// wasm-bindgen's loader defaults to new URL(..., import.meta.url), which Vite 8
// resolves to a file: URL the browser refuses to load. Ask Vite for the asset
// URL instead.
import wasmUrl from "./wasm/penta_wasm_bg.wasm?url";
import type { GameState } from "./game-types";
import type { FormatId } from "./game-config";
import type { CardArtPreference } from "./card-art-mode";

/**
 * What the React app needs from an engine, local or hosted: the command
 * methods it fires and the one snapshot read. The wasm `WebGame` satisfies
 * this structurally; `RemoteEngineGame` satisfies it over a WebSocket.
 */
export interface EngineGame {
  act(index: number): void;
  choose_decision(decision: number, optionsJson: string): void;
  attack_all(): void;
  cancel_attackers(): void;
  finalize_blocks(assignmentsJson: string): void;
  undo_mana(): void;
  set_phase_stop(phase: string, enabled: boolean): void;
  set_autopass(enabled: boolean): void;
  state_json(): string;
  free(): void;
}

export type EngineConfig = {
  format: FormatId;
  artPreference: CardArtPreference;
  humanDeck: string;
  botDeck: string;
  policy: string;
  humanFirst: boolean;
  seed: number;
};

/** Loads the generated WASM module exactly once per browser session. */
export async function initializeEngine(): Promise<void> {
  await initWasm({ module_or_path: wasmUrl });
}

export function createEngineGame(config: EngineConfig): EngineGame {
  return new RustWebGame(
    config.humanDeck,
    config.botDeck,
    config.policy,
    config.humanFirst,
    config.seed,
    config.format,
    config.artPreference,
  );
}

/**
 * Publishes a console handle for putting cards onto the battlefield, so a
 * board state can be reached directly instead of played toward.
 *
 * The underlying entry point exists only in a WASM build compiled with the
 * `dev-cheats` feature, which the production build never enables, so this is a
 * no-op in a deployed client rather than something to be trusted or guarded
 * against there.
 *
 *   penta.put("human", "Thragtusk")
 */
export function publishDevHandle(
  currentGame: () => EngineGame | null,
  refresh: () => void,
): void {
  if (typeof window === "undefined") {
    return;
  }
  type Cheat = (seat: string, card: string) => void;
  const cheatOf = (game: EngineGame, name: string) =>
    (game as unknown as Record<string, Cheat | undefined>)[name];
  const game = currentGame();
  if (!game || typeof cheatOf(game, "dev_put_onto_battlefield") !== "function") {
    return;
  }
  // Resolve the game on each call: dealing a new one frees the old WASM
  // object, and a handle holding it would fault.
  const invoke = (name: string, seat: string, cardName: string, done: string) => {
    const live = currentGame();
    const cheat = live && cheatOf(live, name);
    if (!live || typeof cheat !== "function") {
      return "no game in play";
    }
    cheat.call(live, seat, cardName);
    refresh();
    return done;
  };
  (window as unknown as { penta: unknown }).penta = {
    put(seat: "human" | "bot", cardName: string) {
      return invoke(
        "dev_put_onto_battlefield",
        seat,
        cardName,
        `put ${cardName} onto ${seat}'s battlefield`,
      );
    },
    bury(seat: "human" | "bot", cardName: string) {
      return invoke(
        "dev_put_into_graveyard",
        seat,
        cardName,
        `put ${cardName} into ${seat}'s graveyard`,
      );
    },
  };
}

/** Decodes the single JSON boundary exposed by the Rust facade. */
export function readEngineState(game: EngineGame): GameState {
  return JSON.parse(game.state_json()) as GameState;
}
