import { randomUUID } from "node:crypto";
import { page, present } from "./views.mjs";

class HttpError extends Error {
  constructor(status, message) { super(`Penta HTTP ${status}: ${message}`); this.status = status; }
}

export class SessionClient {
  #base;
  #fetch;
  #connections = new Map();

  constructor(base, fetcher = fetch) {
    const url = new URL(base);
    if (!["http:", "https:"].includes(url.protocol) || url.username || url.password) throw new Error("invalid Penta server URL");
    this.#base = url.origin;
    this.#fetch = fetcher;
  }

  async #request(room, route, token, body, signal) {
    const path = room === null ? route : `/_game/${encodeURIComponent(room)}/${route}`;
    const response = await this.#fetch(`${this.#base}${path}`, {
      method: body === undefined ? "GET" : "POST",
      headers: { "content-type": "application/json", ...(token ? { "x-penta-token": token } : {}) },
      body: body === undefined ? undefined : JSON.stringify(body),
      signal: signal ? AbortSignal.any([signal, AbortSignal.timeout(35_000)]) : AbortSignal.timeout(35_000),
    });
    const text = await response.text();
    let value;
    try { value = JSON.parse(text); } catch { throw new Error(`Penta returned HTTP ${response.status} without JSON`); }
    if (!response.ok) throw new HttpError(response.status, value.error ?? response.statusText);
    return value;
  }

  async options() { return this.#request(null, "/_engine/options"); }

  async create({ format, p1Deck, p2Deck, matchMode, humanSeat }) {
    const room = randomUUID();
    const humanFirst = humanSeat !== "p2";
    const opened = await this.#request(room, "start", null, {
      format, humanDeck: humanFirst ? p1Deck : p2Deck, botDeck: humanFirst ? p2Deck : p1Deck,
      humanFirst, seed: 0, botPolicy: "external", matchMode, sessionApi: true,
    });
    const seats = {
      [humanFirst ? "p1" : "p2"]: { room, token: opened.humanToken },
      [humanFirst ? "p2" : "p1"]: { room, token: opened.botToken },
    };
    let humanUrl;
    if (humanSeat) {
      const url = new URL(this.#base);
      url.searchParams.set("hosted", room);
      url.searchParams.set("format", format);
      url.searchParams.set("deck", humanFirst ? p1Deck : p2Deck);
      url.searchParams.set("first", String(humanFirst));
      url.searchParams.set("matchMode", matchMode);
      url.searchParams.set("hostedBot", "External");
      url.hash = new URLSearchParams({ seatToken: opened.humanToken }).toString();
      humanUrl = url.href;
    }
    return { room, seats, ...(humanUrl ? { humanUrl } : {}) };
  }

  async attach({ room, token }) {
    const view = await this.#request(room, "session", token);
    const connection = randomUUID().slice(0, 8);
    this.#connections.set(connection, { room, token, view });
    return { connection, ...this.#display(connection, view, true) };
  }

  #get(connection) {
    const session = this.#connections.get(connection);
    if (!session) throw new Error("unknown connection; attach with the seat's saved room and token");
    return session;
  }

  #display(connection, view, full) {
    const session = this.#get(connection);
    session.view = view;
    const displayed = present(view, session.previous, full);
    session.previous = displayed.previous;
    return displayed.result;
  }

  async next({ connection, full = false, waitMs = 25_000 }, signal) {
    const { room, token } = this.#get(connection);
    const view = await this.#request(room, `session?wait=${waitMs}`, token, undefined, signal);
    return this.#display(connection, view, full);
  }

  async play({ connection, revision, choices, requestId, waitMs = 25_000 }, signal) {
    const session = this.#get(connection);
    // Keep the exact request through uncertain network failures. A retry uses
    // the same ID and payload; it cannot accidentally apply a second move.
    const body = { revision, choices, requestId: requestId ?? randomUUID() };
    if (session.pending && JSON.stringify(session.pending) !== JSON.stringify(body)) {
      throw new Error("an earlier play has an uncertain outcome; use retry first");
    }
    session.pending = body;
    let view;
    try {
      view = await this.#request(session.room, `play?wait=${waitMs}`, session.token, body, signal);
    } catch (error) {
      // Validation/authorization refusals have a definite outcome. Transport
      // and server failures can occur after commit and must retain the receipt.
      if (error instanceof HttpError && error.status >= 400 && error.status < 500) session.pending = undefined;
      throw error;
    }
    session.pending = undefined;
    return this.#display(connection, view, false);
  }

  async retry({ connection, waitMs = 25_000 }, signal) {
    const session = this.#get(connection);
    if (!session.pending) throw new Error("no uncertain play is pending");
    return this.play({ connection, ...session.pending, waitMs }, signal);
  }

  async inspect({ connection, section, definitions, query, actionType, offset = 0, limit = 100 }) {
    const session = this.#get(connection);
    if (section === "catalog") {
      if (!definitions?.length && !query?.trim()) throw new Error("catalog inspection requires definition IDs or a name query");
      session.catalog ??= await this.#request(session.room, "catalog", session.token);
      const cards = session.catalog.cards.filter(card =>
        definitions?.includes(card.definition) || (query && card.name.toLowerCase().includes(query.toLowerCase())));
      const { items, ...cursor } = page(cards, offset, limit);
      return { cards: items, ...cursor };
    }
    if (section === "record") return this.#request(session.room, "record", session.token);
    const view = await this.#request(session.room, "session", session.token);
    if (!view.observation) return view;
    if (section === "observation") return this.#display(connection, view, true);
    if (section === "legalActions" || (section === "decision" && view.observation.decision)) {
      const decision = view.observation.decision;
      let items = section === "legalActions" ? view.observation.legalActions : decision.options;
      if (actionType) items = items.filter(item => item.type === actionType);
      if (query) items = items.filter(item => JSON.stringify(item).toLowerCase().includes(query.toLowerCase()));
      return { revision: view.revision,
        ...(section === "decision" ? { decision: { ...decision, options: undefined } } : {}), ...page(items, offset, limit) };
    }
    return { revision: view.revision, [section]: view.observation[section] ?? null };
  }
}
