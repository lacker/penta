# Penta web client

This directory contains the local browser client for the Rust game engine. It
uses React, vinext, and a generated WebAssembly bridge; no account or database
is required to play a local game.

## Prerequisites

- Node.js `>=22.13.0`
- Rust and the `wasm32-unknown-unknown` target
- `wasm-bindgen` on `PATH`

## Quick start

```bash
pnpm install
pnpm run dev
```

The primary checkout uses `http://localhost:3000`; linked Git worktrees each
receive a stable, distinct port so their development servers can run together.
The server prints its URL at startup, and `pnpm run dev:url` reports it without
starting the server. A linked worktree keeps its assignment in the ignored
`web/.dev-port` file. Editor and agent previews that can only be configured
with one fixed port should set `PENTA_DEV_PORT`; both the server and
`dev:url` honour it, so the tool's configuration and the running server cannot
drift apart. The client defaults to The Deck versus Goblins, and all
game state stays in the browser. Development, production builds, and tests keep
the Git-ignored WASM bindings current automatically. Cargo checks incrementally,
and unchanged bindings skip `wasm-bindgen`.

## Checks

From the repository root, use the canonical `Makefile` task catalog and select
the smallest relevant suite while iterating:

```bash
make lint-web
make typecheck-web
make test-web-wasm-casting PATTERN='auto-pass'
make test-web-render
```

The fast WASM tests are organized into `contract`, `casting`, `combat`, `pacing`,
and `state` domains. Each has a discoverable target such as
`make test-web-wasm-contract`; `make test-web-wasm` runs all five. These targets
do not build the production application. Run the simulation-heavy sweeps with
`make test-web-wasm-slow`, or use `make test-web-full` for every web test.
`PATTERN` can narrow any domain or aggregate target by test name.

Use these focused web targets when the WASM adapter, a browser-consumed
contract, or web code changes. Native card and game-rule changes do not need a
local web suite merely because the engine also compiles to WASM; the complete
web gate runs in PR CI. `make check-fast` and `make check` remain available for
intentional aggregate validation rather than as automatic PR prerequisites.

From this directory, `pnpm run test:fast`, `pnpm run test:wasm`,
`pnpm run test:wasm:slow`, and `pnpm run test:render` provide targeted aliases.
The fast alias also discovers standalone Node tests that do not need a
production build.
`pnpm run typecheck` checks the application through the root orchestration.
`pnpm test` and `pnpm run test:all` retain the complete web gate. `pnpm lint`
and `pnpm build` remain available directly.

## Visual verification

For every change that can affect rendered output or user interaction:

1. Start or restart the development server from the current worktree. Confirm
   that the URL reported by `pnpm run dev:url` is served by that process; do not
   accept a fallback port or assume an older server picked up the change.
2. Open the rendered application in a browser and inspect it visually. A build,
   DOM snapshot, or HTTP response is not a visual check.
3. Check at least a 1280x720 laptop viewport for clipping, overlap, off-screen
   controls, unreadable content, and inaccessible horizontal overflow.
4. Exercise enough state to display the changed component. For game-table work,
   inspect cards in hand and on the battlefield when applicable.
5. After the final code change, take a fresh screenshot and inspect it before
   reporting completion.

Keep the verified server running unless the user asks otherwise.

## Deploying

The client deploys to Cloudflare Workers. `worker/index.ts` is the entry point
and `vite.config.ts` declares the Worker; the build writes a ready Wrangler
config to `dist/server/wrangler.json`.

```bash
pnpm run deploy
```

That rebuilds the WASM artifact and the client, then publishes the `penta`
Worker. It needs a Cloudflare account — `npx wrangler login` once, and
`npx wrangler whoami` to check which account is active. The game runs entirely
in the browser, so there are no D1, R2, or other storage bindings to provision,
and local development needs no Cloudflare credentials at all.

The public client is deployed at <https://penta.lacker.workers.dev>. Hosted
games and the bot registry are gated by `HOSTED_GAMES`; the engine self-check is
gated by `ENGINE_SELF_CHECK`. When asked about current deployment state, verify
it rather than answering from memory. One request distinguishes an enabled bot
registry from the disabled route:

```bash
curl -s -o /dev/null -w '%{http_code}\n' https://penta.lacker.workers.dev/_bots
```
