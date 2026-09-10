/** Exact bot commands shared by both seat connections. No policy lives here. */
export class SessionError extends Error {
  constructor(message, status = 400) { super(message); this.status = status; }
}

export function sessionView(game, stored, role) {
  if (!stored.config.sessionApi && role !== "bot") {
    throw new SessionError("the human seat requires a sessionApi match", 409);
  }
  const done = game.isFinished();
  const deciding = stored.config.sessionApi
    ? game.sessionDecisionRole() === role : game.opponentIsDeciding();
  // Neither a revision counter nor an intermediate private decision escapes
  // while the other seat acts. An opaque token is exposed only at a safe stop.
  if (!done && !deciding) return { apiVersion: 1, status: "waiting", role };
  const observation = JSON.parse(stored.config.sessionApi
    ? game.sessionObserveJson(role) : game.opponentObserveJson());
  return { apiVersion: 1, status: done ? "complete" : "ready", role,
    revision: stored.revision, observation };
}

function uint(value) {
  return Number.isInteger(value) && value >= 0 && value <= 0xffffffff;
}

export function parsePlay(body) {
  if (!body || typeof body !== "object" || Array.isArray(body)) throw new SessionError("expected a play object");
  for (const field of ["revision", "requestId"]) {
    if (typeof body[field] !== "string" || !body[field].length || body[field].length > 128) {
      throw new SessionError(`${field} must be a nonempty string of at most 128 characters`);
    }
  }
  if (!Array.isArray(body.choices) || !body.choices.length || body.choices.length > 64) {
    throw new SessionError("choices must contain 1 to 64 explicit choices");
  }
  for (const choice of body.choices) {
    if (!choice || typeof choice !== "object" || Array.isArray(choice)) throw new SessionError("invalid choice");
    const keys = Object.keys(choice).sort().join(",");
    if (keys === "index" && uint(choice.index)) {
      if (body.choices.length !== 1) throw new SessionError("batch choices must use action values or decision IDs, not stale indices");
    } else if (keys === "decision,options" && uint(choice.decision)
        && Array.isArray(choice.options) && choice.options.length <= 4096 && choice.options.every(uint)) {
      // Option order is meaningful for some engine decisions; keep it intact.
    } else if (keys !== "action" || !choice.action || typeof choice.action !== "object"
        || Array.isArray(choice.action) || typeof choice.action.type !== "string" || "index" in choice.action) {
      throw new SessionError("choose an index, an exact action without index, or decision and option IDs");
    }
  }
  return { revision: body.revision, requestId: body.requestId, choices: body.choices };
}

/** Object key order is presentation; every field and every array position matters. */
export function canonical(value) {
  if (Array.isArray(value)) return `[${value.map(canonical).join(",")}]`;
  if (value && typeof value === "object") {
    return `{${Object.keys(value).sort().map(key => `${JSON.stringify(key)}:${canonical(value[key])}`).join(",")}}`;
  }
  return JSON.stringify(value);
}

export function choiceCommand(game, stored, role, choice) {
  const view = sessionView(game, stored, role);
  if (view.status !== "ready") throw new SessionError("the seat does not hold a decision", 409);
  if ("options" in choice) {
    if (view.observation.decision?.id !== choice.decision) throw new SessionError("stale decision", 409);
    return { t: role === "human" ? "choose" : "botChoose", ...choice };
  }
  let index = choice.index;
  if (choice.action) {
    const wanted = canonical(choice.action);
    const matches = view.observation.legalActions.filter(candidate => {
      const action = { ...candidate }; delete action.index;
      return canonical(action) === wanted;
    });
    if (matches.length !== 1) throw new SessionError("the exact action is not uniquely legal", 409);
    index = matches[0].index;
  }
  if (!view.observation.legalActions.some(action => action.index === index)) throw new SessionError("illegal action index");
  return stored.config.sessionApi ? { t: "sessionAct", role, index } : { t: "botAct", index };
}
