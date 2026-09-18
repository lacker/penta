// Transport-independent command surface. The skill supplies the workflow;
// all gameplay decisions and exact choices still come from the caller.
export const methods = {
  options: "options", start_match: "create", attach: "attach", next: "next",
  play: "play", choose: "choose", retry: "retry", inspect: "inspect", inspect_ref: "inspectReference",
};
export function validate(command, args) {
  if (!Object.hasOwn(methods, command) && !["up", "status", "stop"].includes(command)) throw new Error(`unknown command: ${command}`);
  if (!args || typeof args !== "object" || Array.isArray(args)) throw new Error("arguments must be a JSON object");
  if (args.waitMs !== undefined && (!Number.isInteger(args.waitMs) || args.waitMs < 0 || args.waitMs > 25_000)) throw new Error("waitMs must be an integer from 0 to 25000");
  const required = {
    attach: ["room", "token"], start_match: ["format", "p1Deck", "p2Deck", "requestId"],
    next: ["connection"], play: ["connection", "revision"], retry: ["connection"],
    choose: ["ticket"], inspect: ["connection", "section"], inspect_ref: ["reference"],
  };
  for (const key of required[command] ?? []) {
    if (typeof args[key] !== "string" || !args[key].length) throw new Error(`${key} must be a nonempty string`);
  }
  if (command === "start_match") {
    args.matchMode ??= "first-to-two-wins";
    if (!["one-conclusion", "first-to-two-wins"].includes(args.matchMode)) throw new Error("invalid matchMode");
    if (args.humanSeat !== undefined && !["p1", "p2"].includes(args.humanSeat)) throw new Error("invalid humanSeat");
  }
  if (command === "attach") args.presentation ??= "decision-v1";
  if (command === "play" && (!Array.isArray(args.choices) || !args.choices.length || args.choices.length > 64)) throw new Error("choices must contain 1 to 64 explicit choices");
  if (args.options !== undefined && (!Array.isArray(args.options) || !args.options.every(id => Number.isInteger(id) && id >= 0 && id <= 0xffffffff))) throw new Error("options must contain unsigned integer IDs");
  if (args.definitions !== undefined && (!Array.isArray(args.definitions) || !args.definitions.every(id => typeof id === "string"))) throw new Error("definitions must contain string IDs");
  for (const key of ["offset", "limit"]) {
    if (args[key] !== undefined && (!Number.isInteger(args[key]) || args[key] < (key === "limit" ? 1 : 0) || (key === "limit" && args[key] > 100))) throw new Error(`invalid ${key}`);
  }
}
