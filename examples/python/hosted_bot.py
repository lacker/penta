"""Put a bot online so people can play it.

The whole thing is one loop over one HTTP call. Heartbeating is what "online"
means -- stop, and the bot leaves the list -- and the heartbeat's reply is
where invitations arrive. No WebSocket, no framework, no penta module: this
plays entirely against the server's engine, so it runs anywhere `requests`
does.

Note that the heartbeat keeps going *during* a game. Presence is a lease on
wall-clock time, and a game is the longest stretch a bot spends busy, so a
play loop that stops heartbeating is a bot that goes silent right when
someone is waiting on it.

The play loop waits rather than polls: `opponent` takes a `wait`, and holds
the request open until the decision is yours. Every priority pass is a
decision, so a poll interval is a tax paid many times per turn.

    python3 hosted_bot.py --name Fizzbot           # your own server
    python3 hosted_bot.py --name Fizzbot \
        --server https://penta.lacker.workers.dev  # where people can play it

The default is the development server from `cd web && pnpm run dev`, which
is where a bot under construction belongs: no limits, no audience. A linked
worktree gets its own port, which `pnpm run dev:url` reports.

`choose` is the whole bot. Everything above and below it is plumbing you can
copy verbatim; see BOTS.md for what an observation holds.
"""

import argparse
import time

import requests

# Heartbeat well inside the server's 45-second presence window, so one lost
# packet does not read as "this bot went away".
HEARTBEAT_SECONDS = 10
# How long to let the server hold a request open while it is the human's turn.
# The server answers the moment the decision is ours, so this is a ceiling on
# waiting, not a delay -- and it wants to stay under HEARTBEAT_SECONDS so the
# heartbeat below keeps its own schedule.
WAIT_SECONDS = 8
# How long to wait before asking again after a refusal, which is the only
# thing left that asking again can fix.
RETRY_SECONDS = 0.25
# This bot consumes the protocol-30 indexed-action vocabulary, requires no
# optional server facilities, and implements no optional server-required
# vocabulary. Never copy the server's claims without implementing them.
COMPATIBILITY = {
    "protocolVersion": 30,
    "capabilities": [],
    "requiredCapabilities": [],
    # A trained bot can set this to the simulationFingerprint it requires:
    # "requiredSimulationFingerprint": "sha256-...",
}


def choose(observation):
    """Pick an action index from one observation. Replace me with a model."""
    actions = observation["legalActions"]
    for index, action in enumerate(actions):
        if action["type"] == "KeepHand":
            return index
    # Anything that develops the board beats passing, and passing beats
    # conceding -- which is legal at every priority and would end the game.
    for wanted in ("PlayLand", "CastSpell", "DeclareAttacker", "ActivateAbility"):
        for index, action in enumerate(actions):
            if action["type"] == wanted:
                return index
    for index, action in enumerate(actions):
        if action["type"] != "Concede":
            return index
    return 0


class Incompatible(Exception):
    """The server will not have this bot as it is declared."""


def heartbeat(server, identifier, token, done=()):
    """Renew presence, and collect whatever invitations are waiting.

    Raises `Incompatible` when the server refuses this bot's declaration,
    which is not something retrying will fix.
    """
    response = requests.post(
        f"{server}/_bots/{identifier}/heartbeat",
        json={
            "token": token,
            "done": list(done),
            "compatibility": COMPATIBILITY,
        },
        timeout=30,
    )
    if response.status_code == 409:
        raise Incompatible(response.text)
    response.raise_for_status()
    return response.json()


def keep_alive(server, identifier, token):
    """Returns a callable that heartbeats whenever the lease is due.

    Call it as often as you like -- it is a no-op until `HEARTBEAT_SECONDS`
    have passed -- and call it from anywhere the bot might be busy for a
    while. The play loop below calls it on every poll, which is what keeps a
    long game from looking like a bot that walked away.

    A failed heartbeat is not fatal: presence is a lease with slack in it, so
    one lost packet is worth riding out. Only a refused declaration stops the
    bot, because that one will not fix itself.
    """
    due = 0.0

    def beat(done=()):
        nonlocal due
        now = time.monotonic()
        if now < due and not done:
            return
        due = now + HEARTBEAT_SECONDS
        try:
            return heartbeat(server, identifier, token, done)
        except (requests.RequestException, ValueError) as problem:
            print(f"heartbeat failed ({problem}); retrying")
            return None

    return beat


def play(server, room, token, beat):
    """Drive the opponent seat of one room until the game ends.

    The token came with the invitation and is what authorises this seat; the
    room id alone is just a name, and the room will refuse without it.

    `beat` is the presence heartbeat, called on every pass. A game is the
    longest a bot stays busy, so this loop -- not the outer one -- is where
    most of a bot's heartbeats happen. Drop it and the server watches the bot
    go silent mid-game and ends the game against it, which looks from the
    other seat like a bot that ran out of time.
    """
    print(f"playing {room}")
    headers = {"x-penta-token": token}
    while True:
        # Whatever this returns is about some later game: the invitation for
        # this room stays outstanding until the room is reported in `done`.
        beat()
        # `wait` parks this request until the decision is ours. A game asks
        # the opponent seat for far more decisions than it looks like -- every
        # priority pass is one -- so a fixed poll interval would spend itself
        # over and over on a single turn. Waiting costs the human nothing and
        # costs the server less than asking would.
        view = requests.get(
            f"{server}/_game/{room}/opponent",
            params={"wait": int(WAIT_SECONDS * 1000)},
            headers=headers,
            timeout=WAIT_SECONDS + 30,
        ).json()
        if view.get("result"):
            print(f"  finished: {view['result']}")
            return
        if not view.get("deciding"):
            # The wait elapsed with the human still thinking. Ask again.
            continue
        index = choose(view["observation"])
        reply = requests.post(
            f"{server}/_game/{room}/command",
            json={"t": "botAct", "index": index},
            headers=headers,
            timeout=30,
        )
        if reply.status_code != 200:
            # A refused action leaves the previous observation standing, so
            # the next request simply asks again.
            print(f"  refused: {reply.text}")
            time.sleep(RETRY_SECONDS)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--server",
        default="http://localhost:3000",
        help="your own server while building; "
        "https://penta.lacker.workers.dev to play other people",
    )
    parser.add_argument("--name", default="Fizzbot")
    parser.add_argument("--deck", default="Sligh")
    arguments = parser.parse_args()
    server = arguments.server.rstrip("/")

    registration_reply = requests.post(
        f"{server}/_bots/register",
        json={
            "name": arguments.name,
            "deck": arguments.deck,
            "compatibility": COMPATIBILITY,
        },
        timeout=30,
    )
    if registration_reply.status_code == 409:
        print(f"server rejected this bot's compatibility: {registration_reply.text}")
        return
    registration_reply.raise_for_status()
    registration = registration_reply.json()
    identifier, token = registration["id"], registration["token"]
    print(f"registered as {arguments.name} ({identifier}) playing {arguments.deck}")
    print("waiting for a challenger…")

    beat = keep_alive(server, identifier, token)
    finished = []
    while True:
        # A server restart, a proxy hiccup, or a redeploy all show up as one
        # bad reply, which `beat` rides out: missing a heartbeat costs at most
        # a spell out of the list, and crashing costs the whole bot. A refused
        # declaration is the one thing retrying will not fix.
        try:
            # Reporting rooms in `done` is what frees the bot for the next
            # game, so this beat is due whether or not the lease is.
            reply = beat(finished)
            if reply is None:
                time.sleep(HEARTBEAT_SECONDS)
                continue
            finished = []
            for invite in reply.get("invites", []):
                try:
                    play(server, invite["room"], invite["token"], beat)
                except (requests.RequestException, ValueError) as problem:
                    print(f"lost the game in {invite['room']} ({problem})")
                # Reporting it finished is what frees the bot for the next
                # game, whether it ended in a result or a dropped connection.
                finished.append(invite["room"])
        except Incompatible as problem:
            print(f"server rejected this bot's compatibility: {problem}")
            return
        if not finished:
            time.sleep(HEARTBEAT_SECONDS)


if __name__ == "__main__":
    main()
