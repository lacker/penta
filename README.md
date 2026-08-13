# Penta

[![CI](https://github.com/lacker/penta/actions/workflows/ci.yml/badge.svg)](https://github.com/lacker/penta/actions/workflows/ci.yml)

Penta is a deterministic, headless simulator for two-player constructed Magic:
The Gathering, built for writing AI bots against. The Rust engine is available
directly, through Python and C bindings, and in a local browser client compiled
to WebAssembly.

Penta currently ships two explicit format profiles:

- **Eternal Central Old School 93/94**, including mana burn and fifteen
  built-in archetypes.
- **ISD–RTR Standard**, using the final pre-Theros legality snapshot and the
  eight SCG Open Atlanta Top 8 decks.

The project is growing incrementally toward a full Magic engine. It is playable
end to end, but it does not yet implement the complete Comprehensive Rules or
every printed effect in the supported card pool. Unsupported behavior remains
visible through implementation coverage rather than resolving as a silent
approximation. See [formats and current scope](docs/formats.md) for details.

## Try it

Check the local toolchain and run a deterministic match between the built-in
policies:

```sh
make doctor
cargo run --release --bin penta-match -- \
  --p1 random --p2 handcrafted --deck1 Sligh --deck2 "The Deck" \
  --games 10 --seed 1
```

To run the browser client:

```sh
cd web
pnpm install
pnpm run dev
```

Linked worktrees receive stable independent development ports. The server
prints the active URL, and `pnpm run dev:url` reports it without starting a new
process.

### Deployment

The client is deployed to <https://penta.lacker.workers.dev>. It plays games
in the browser and serves the hosted-game and bot-registry routes, so a bot
anywhere can put itself online there and be played: see the
[bot guide](docs/bots.md). Each seat of a hosted room is held by a token
minted when the room starts, and the routes that create things are held to
ten a minute per address.

`/_engine/self-check` plays a whole game inside the Worker and is separately
gated behind `ENGINE_SELF_CHECK`, which the deployment does not set.

## Write a bot

Bots choose an index from the engine's hidden-information-safe legal-action
list. Python, C, C++, and Rust integrations share the same deterministic game
and protocol semantics. Start with the [bot guide](docs/bots.md).

Query the breaking protocol epoch, conservative simulation fingerprint, and
package engine version through the selected binding. Pin the simulation
fingerprint alongside trained weights; compatibility history and migration
notes are in the [changelog](CHANGELOG.md).

## Documentation

- [Documentation index](docs/README.md)
- [Design doctrine](docs/design-doctrine.md)
- [Engine architecture](docs/engine.md)
- [Implementing cards](docs/implementing-cards.md)
- [Engine interfaces](docs/interfaces.md)
- [Formats and current scope](docs/formats.md)
- [Development guide](docs/development.md)
- [Performance guide](docs/performance.md)
- [Web client guide](web/README.md)

## Validation

The root `Makefile` is the canonical task catalog. `make help` lists focused
targets, `make check-fast` is the broad development checkpoint, and
`make check` runs the complete engine and web gate. See the
[development guide](docs/development.md) for the contributor workflow.

## License

Penta is licensed under the [Apache License 2.0](LICENSE).
