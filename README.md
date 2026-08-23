# Penta

[![CI](https://github.com/lacker/penta/actions/workflows/ci.yml/badge.svg)](https://github.com/lacker/penta/actions/workflows/ci.yml)

Penta is a deterministic, headless simulator for two-player Magic:
The Gathering, built for writing AI bots against. The Rust engine is available
directly, through Python and C bindings, and in a local browser client compiled
to WebAssembly.

Penta currently ships seven explicit format profiles:

- **Old School 93/94**, using Eternal Central rules including mana burn and fifteen
  built-in archetypes.
- **Premodern**, spanning Fourth Edition through Scourge with its own ban list.
- **Standard: ISD-M14**, using the final pre-Theros legality snapshot and ten
  built-in decks: the eight SCG Open Atlanta Top 8 lists plus two January 2013
  SCG matchup decks.
- **Standard: ISD-M14**, the same window under its first-and-last-set label.
- **Standard: SOM-M13**, combining Scars of Mirrodin and Innistrad blocks.
- **Cube: Vintage** and **Cube: The Pauper Cube**, each defined by a dated fixed
  card list rather than a set window.

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
targets; local validation normally follows the changed behavior, while PR CI
runs the complete repository gates. `make check-fast`, `make check`, and
`make ci` remain available for intentional aggregate validation. See the
[development guide](docs/development.md) for the contributor workflow.

## License

Penta is licensed under the [Apache License 2.0](LICENSE).
