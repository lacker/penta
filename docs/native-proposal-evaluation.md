# Native action proposal evaluation

Manual evaluations on 2026-10-20 and 2026-10-21, on one Apple Silicon host, using release
builds. This evaluates the state/proposal path described in
[native matches](native-matches.md). Raw exports, CPU captures, request logs,
probe sources, binaries, and timing JSON are retained locally under ignored
`target/profiles/direct-agent-actions/`; private game exports are not committed.

## Controlled enumeration workload

The baseline source tree matched upstream main
`139fbedccbb618110c6b3c81cfec7dd2f870891e`. The synthetic fixture uses seed 1,
turn 3 precombat main, twelve opposing Llanowar Elves, four Jeskai Revelations
in hand, and fourteen mana sources: four Great Hall of the Biblioplex, two
Starting Town, and eight cycling basic lands. This is a stress position, not
a claim about the reported remote game's actual board.

The old observation generated 1,456 cast alternatives. The proposal path
returns visible state and checks the selected action. Both builds produced
identical visible state after excluding `legalActions`, `checkpoint`, and the
simulation fingerprint. No game choice was replaced with a policy decision.

Three unprofiled measurements after one warmup, with identical fixture and
machine and no live evaluation game or compiler running:

| Operation | Baseline median | Proposal median |
| --- | ---: | ---: |
| Observe the position | 5,475.681 ms | 0.516 ms |
| Submit PassPriority | 5,471.700 ms | 0.338 ms |

Hyperfine measured the entire probe, including catalog/setup, observation
serialization, and pass: 11.386 s ± 0.158 s versus 0.275 s ± 0.021 s,
about 41.4× faster for that workload. This is not a whole-game speedup claim.
The earlier CPU sample attributed the expensive path to cast-target expansion
and repeated mana assignment.

## Model games

For the first two runs, the organizer launched one host at a time. Each fresh-context pilot received
only its own seat capability and used the persistent memory-only CLI session.
Host processing times exclude HTTP waiting and authority-lock queueing.
Elapsed pilot time includes thinking, tool dispatch, and opponent waits.

| Run | Pilots | Result | Host interactions | Maximum processing | Total processing |
| --- | --- | --- | ---: | ---: | ---: |
| WOE-HOB, first iteration | Astra Max Control vs 6.1 Sol Medium Bant Airbending | Control won turn 21 by decking Bant | 120 | 136.532 ms | 2.463 s |
| Old School, revised interface | Astra Medium vs Astra Medium, Sligh mirror | P1 won turn 9 with Ball Lightning trample | 27 | 34.295 ms | 0.268 s |

The WOE-HOB run lasted about 17 minutes from host launch to completion. Pilots
reported 58 and 67 session calls, five proposal rejections in total, and one
explicit discovery. All recovered. Restricted mana, chosen funding order,
five-color converge, flashback, targets, combat, and additional costs were used.
The 581-command export replayed with identical final observations using its
matching evaluation binary.

The revised Sligh run took 3m10s from launch; pilot sessions lasted 2m49s and
2m38s. Both finished with `pilot-v3`, made no discovery calls, and reported
504,487 output bytes combined. One redundant finish-attack action was rejected
after its two-attack accepted prefix; the pilot recovered using the receipt.
Two inputs failed local validation: an omitted empty cast-choice object and an
unsupported yield condition. The 220-command export replayed with identical
final observations. Different decks, seeds, and model settings mean these games
are not a controlled comparison of total wall time or token cost.

Pilot feedback led to these changes before the second run:

- Expose runtime alternative-cost IDs, including granted flashback, with each
  visible object's ability metadata.
- Let a mana combination determine its internal representative color.
- Name cast payment resources `costObjects`, including cards exiled for evidence.
- Fix absent cast-offer metadata being serialized as null at completion.
- Suppress unchanged, successfully delivered ability metadata in persistent
  sessions, with `full:true` recovery and full delivery after changes/reappearance.

The first iteration produced 6,349,425 model-facing output bytes across both
pilots, larger than desirable. Reprocessing its recorded response revisions
with the metadata cache reduced ability metadata from 1,350,312 to 202,584 bytes
for Control and from 1,623,483 to 220,194 bytes for Bant, reductions of 85.0% and
86.4%. This isolates metadata delivery on the same positions; it is not a
measurement of model tokens, billing, or total game-output reduction.

Final small follow-ups allow omitted empty cast choices and omit artwork and
printing lists from inline card rules, retaining their exact references. These
were validated with focused tests after the Sligh run.

## Concurrent complex-deck games

Two release hosts ran concurrently on 2026-10-21 at source revision
`58bf0f3e8df188beddc3e5fcd386638d2fc476c6`. Both seats used Astra Medium.
The orchestration limit allowed two pilot agents: each fresh-context agent
controlled the same seat in both matches, receiving only its own capabilities
and observations. Thus this exercises concurrent hosts and overlapping model
play, but not four independent model contexts. No compiler ran during play.

| WOE-HOB matchup | Result | Launch to completion | Host interactions | Maximum processing | Total processing |
| --- | --- | ---: | ---: | ---: | ---: |
| 4/5C Control vs Bant Airbending | Bant won turn 16, life −1–16 | 16m15s | 80 | 280.567 ms | 3.529 s |
| 4c Legends vs Izzet Spellementals | Izzet won turn 14, life −5–17 | 8m05s | 44 | 45.466 ms | 0.697 s |

The hosts overlapped for about eight minutes. Neither reported an engine failure
or multi-minute processing spike. Catalog initialization was the largest single
recorded command in each host, at 299.881 ms and 307.502 ms. Some HTTP requests
took 25 seconds because they intentionally waited for the other pilot; those
durations are separate from `commandMetrics`. A model context compaction also
paused the Bant pilot. The sandbox denied the organizer's process sampling, so
there is no CPU/RSS time series from which to quantify contention.

Control–Bant exercised airbend, restricted mana, Fetch Quest, repeated Web Watch
casts, transformation, trample, and She-Hulk retaliation. Legends–Izzet exercised
creature-type selection, counters, cantrips, removal, bounce, and tap effects.
Their 429- and 323-command exports replayed with identical final observations
using the saved evaluation binary. Private exports, request logs, manifests,
pilot statistics, and replay results are retained in `complex1-control-bant/`
and `complex2-legends-izzet/` under the local artifact directory above.

Pilots reported 141 and 76 client calls, and 2,228,271 and 1,097,700 model-facing
output bytes, respectively. These are serialized bytes, not model token counts
or billing. The Control pilot's counts combine two local client sessions: it
reconnected after a recovered timeout to exercise the updated client. Its final
six interactions had no errors. Each client also downloaded the large catalog
once; those wire bytes are not included in model-facing output. These games do
not establish a speedup over earlier runs with different decks and seeds.

Observed friction led to these follow-ups:

- An initial mutation's 25-second host long poll raced the client's 25-second
  wait budget. The move had committed, but the client reported uncertainty;
  retrieving the receipt recovered it. The initial request now uses the existing
  transport timeout, while the local budget bounds subsequent read-only waits.
  A regression test covers a receipt arriving after that budget expires.
- Invalid proposal shapes now report their failing field and expected tags
  through nested unions, instead of only saying “invalid union value.” Local
  validation still rejects the same invalid inputs before contacting the host.
- Cavern of Souls offered 325 creature types, and repeated empty metadata caused
  the tool output to truncate. `options.defaults` now factors that metadata
  without removing any choice. Reprocessing the recorded table reduced its
  JSON from 33,471 to 9,223 bytes (72.4%); expanding defaults reproduces every
  original row exactly. This measures that table, not the entire response.
- Airbent objects lacked their `{2}` permission cost in proposal metadata,
  prompting one explicit discovery. Visible objects now expose the applicable
  exile/graveyard `playPermission`, separately from alternative-cost IDs,
  without enumerating casts or payment plans.
- Pilots sometimes appended `FinishDeclaringAttackers` after the final attacker
  had already advanced combat automatically. Guidance now explains when the
  finish is needed and how a rejected suffix prevents yield installation;
  runtime batch semantics remain unchanged.

The detailed errors and timeout change were present in the Control pilot's
reconnected client. The option reduction was checked on the recorded Cavern
decision and with lossless reconstruction tests; no later live decision used
it. Permission metadata was added after play and validated with focused engine
tests. These changes do not have a new pair of full-game wall-time measurements.

## Native match lifecycle confirmation

A focused Sligh–The Deck check on 2026-10-21 used a release host with
`first-to-two-wins`, two persistent `session-rpc` clients, and finite CLI calls
using the default `pilot-v3` presentation. Concessions ended three games to
exercise a 2–1 match; this was a lifecycle check, not three model-played games.
Both seats swapped a main-deck land for Red Elemental Blast at each intermission,
and the next games retained those submitted decks. The previous loser owned the
play/draw decision, seat identities remained stable, and completion occurred
only at two wins. No legacy discovery was used. All 16 journal commands replayed
with identical final observations. Local evidence is under
`target/profiles/native-match-confirmation/`.

The first check found a presentation gap: match lists contain bare definition
IDs, so rules for sideboard cards not yet seen in play were omitted. The client
now includes printed references for the seat's entire main deck and sideboard.
The CLI check was repeated successfully after the fix. A focused host regression
also covers illegal sideboard rejection without state changes, idempotent
submission retries, private decisions, and stopping a yield at a game boundary.

## Scope and remaining limits

The reported remote multi-minute `commandMetrics.maxMicros` incidents have not
been reproduced from their exact states. These measurements establish a concrete
enumeration bottleneck and remove it from ordinary native observation, passing,
and cast proposal submission; they do not establish every remote stall's cause.

Optional `discover`, `nonManaAction` yields, explicit payment-editor discovery,
and some activated-ability checks still enumerate. `nonManaAction` also stops
conservatively for payment-editor entry, which can create extra interactions.
Phase/event yields avoid that discovery work. Cast refusal diagnostics identify
validation stages but do not yet isolate every invalid announcement field.
The host remains synchronous; this change adds no cancellation deadline or
background-job protocol for a genuinely expensive engine operation.
