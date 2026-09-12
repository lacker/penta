import { randomBytes } from "node:crypto";
import { page } from "./views.mjs";
import { actionLabel, objectNames } from "./decision-labels.mjs";
import { factorRows, namedObject, printedReference, visibleDefinitions, withoutArt } from "./decision-format.mjs";

const SITUATION = ["seat", "activeSeat", "prioritySeat", "turn", "activeTurn", "phase", "step", "pregame", "match", "regularCombatDamagePending", "result"];
const PLAYERS = ["life", "manaPools", "poison", "energy", "playerCounters", "monarch", "opponentHandSize", "librarySizes", "faceDownExileSizes"];
const PROVENANCE = ["protocolVersion", "protocolCapabilities", "engineVersion", "simulationFingerprint", "format"];

export class DecisionView {
  id = randomBytes(8).toString("base64url");
  tickets = new Map();
  references = new Map();
  active = true;

  constructor(view, catalog, seenRules = new Set(), full = false) {
    if (view.apiVersion !== 1 || (view.observation.protocolVersion !== undefined && view.observation.protocolVersion !== 32)) {
      throw new Error("decision-v1 requires session API 1 and bot protocol 32; use the exact interface or update the adapter");
    }
    this.revision = view.revision;
    this.source = structuredClone(view.observation);
    const observation = this.source;
    const names = objectNames(observation);
    const remaining = { ...observation };
    const take = keys => Object.fromEntries(keys.filter(key => Object.hasOwn(remaining, key)).map(key => {
      const value = remaining[key]; delete remaining[key]; return [key, value];
    }));
    const provenance = take(PROVENANCE);
    if (Object.hasOwn(remaining, "checkpoint")) { provenance.checkpoint = remaining.checkpoint; delete remaining.checkpoint; }
    const packet = { presentation: "decision-v1", view: this.id,
      notation: "Each shared/rows table is self-contained: shared fields apply to every row. All IDs and exact choice fields are retained. Player arrays use [p1,p2]. Stack order is bottom to top.",
      situation: take(SITUATION), players: take(PLAYERS), position: {} };
    for (const field of ["hand", "battlefield", "stack", "graveyards", "exiles", "emblems", "companions", "cardCounters"]) {
      if (!Object.hasOwn(remaining, field)) continue;
      const original = remaining[field]; delete remaining[field];
      const rows = Array.isArray(original) ? original.map(value => Array.isArray(value) ? factorRows(value.map(namedObject)) : namedObject(value)) : original;
      packet.position[field] = this.section(field, rows);
    }
    if (Object.hasOwn(remaining, "updates")) {
      packet.updates = this.section("updates", withoutArt(remaining.updates)); delete remaining.updates;
    }
    const actions = remaining.legalActions ?? [];
    const choices = actions.map(action => {
      const ticket = `${this.id}:a${action.index}`;
      const value = structuredClone(action);
      if (action.type === "ChooseDecision") {
        const decision = observation.decision;
        if (!decision || decision.id !== action.decision) throw new Error("decision marker has no matching selection schema");
        this.tickets.set(ticket, { decision: structuredClone(decision) });
      } else this.tickets.set(ticket, { index: action.index, action: value });
      const label = actionLabel(action, names);
      // Preserve future engine fields that collide with presentation keys.
      return Object.hasOwn(value, "ticket") || Object.hasOwn(value, "label")
        ? { ticket, label, exact: value } : { ticket, label, ...value };
    });
    packet.choices = this.section("choices", choices);
    delete remaining.legalActions;
    if (Object.hasOwn(remaining, "decision")) {
      packet.decision = structuredClone(remaining.decision);
      if (packet.decision?.options) packet.decision.options = this.section("decision options", packet.decision.options.map(namedObject));
      delete remaining.decision;
    }
    // Optional new engine fields, match state and historical knowledge survive
    // without needing the adapter to know their meaning.
    packet.facts = this.section("other observed facts", withoutArt(remaining));
    packet.provenance = { reference: this.reference("provenance and checkpoint", provenance) };
    let inlineSize = 0;
    const compatibleCatalog = !catalog || ["protocolVersion", "simulationFingerprint"].every(key =>
      catalog[key] === undefined || observation[key] === undefined || catalog[key] === observation[key]);
    const byId = new Map((compatibleCatalog ? catalog?.cards ?? [] : []).map(card => [card.definition, card]));
    const rules = visibleDefinitions(observation).map(definition => {
      const card = byId.get(definition);
      if (!card) return { definition, available: false };
      const body = printedReference(card);
      const text = JSON.stringify(body);
      const key = `${definition}:${text}`;
      const entry = { definition, name: card.name, reference: this.reference("printed card reference", card) };
      if ((full || !seenRules.has(key)) && inlineSize + text.length <= 12_000) {
        entry.printed = body; inlineSize += text.length; seenRules.add(key);
      }
      return entry;
    });
    packet.rules = this.section("card references", rules);
    this.packet = packet;
  }

  reference(kind, value) {
    const ref = `${this.id}:r${this.references.size}`;
    this.references.set(ref, { kind, value: structuredClone(value) });
    return ref;
  }

  section(kind, value) {
    const compact = Array.isArray(value) ? factorRows(value) : value;
    if ((Array.isArray(value) && value.length > 100) || JSON.stringify(compact).length > 12_000) {
      return { count: Array.isArray(value) ? value.length : 1, reference: this.reference(kind, value) };
    }
    return compact;
  }

  inspect(reference, { offset = 0, limit = 100, query, actionType } = {}) {
    const found = this.references.get(reference);
    if (!found) throw new Error("unknown or expired reference; request next(full=true)");
    let items = Array.isArray(found.value) ? found.value : [found.value];
    if (actionType) items = items.filter(item => item?.type === actionType);
    if (query) items = items.filter(item => JSON.stringify(item).toLowerCase().includes(query.toLowerCase()));
    return { view: this.id, reference, kind: found.kind, ...page(items, offset, limit) };
  }

  choice(ticket, options) {
    if (!this.active) throw new Error("stale ticket; request next(full=true)");
    const entry = this.tickets.get(ticket);
    if (!entry) throw new Error("unknown or expired ticket; request next(full=true)");
    if (!entry.decision) {
      if (options !== undefined) throw new Error("concrete action tickets do not accept options");
      return { index: entry.index };
    }
    const decision = entry.decision;
    if (!Array.isArray(options)) throw new Error("decision tickets require an explicit options array");
    const allowed = new Set(decision.options.map(option => option.id));
    if (options.length < decision.minimum || options.length > decision.maximum || new Set(options).size !== options.length || options.some(id => !allowed.has(id))) {
      throw new Error("invalid option IDs or selection count; preserve the displayed decision bounds and order");
    }
    return { decision: decision.id, options: [...options] };
  }
}
