import { useState } from "react";
import type { MatchState } from "./game-types";

type DeckCard = MatchState["main"][number];

function groups(cards: DeckCard[]) {
  const counts = new Map<number, { card: DeckCard; count: number }>();
  for (const card of cards) {
    const entry = counts.get(card.id);
    if (entry) entry.count += 1;
    else counts.set(card.id, { card, count: 1 });
  }
  return [...counts.values()].sort((a, b) => a.card.name.localeCompare(b.card.name));
}

export function MatchResult({ match, message, error, next, newMatch }: {
  match: MatchState;
  message: string;
  error: string | null;
  next: (main: number[], sideboard: number[], humanFirst: boolean) => void;
  newMatch: () => void;
}) {
  const [main, setMain] = useState(match.main);
  const [sideboard, setSideboard] = useState(match.sideboard);
  const [humanFirst, setHumanFirst] = useState(match.humanChooses);
  const move = (card: DeckCard, fromMain: boolean) => {
    const source = [...(fromMain ? main : sideboard)];
    source.splice(source.findIndex((entry) => entry.id === card.id), 1);
    if (fromMain) { setMain(source); setSideboard([...sideboard, card]); }
    else { setSideboard(source); setMain([...main, card]); }
  };
  return <section className="match-result" role="dialog" aria-modal="true" aria-labelledby="match-result-title">
    <span>{match.finished ? "MATCH OVER" : `GAME ${match.game} COMPLETE`}</span>
    <h1 id="match-result-title">{match.finished ? (match.wins[0] === 2 ? "You win the match" : "Opponent wins the match") : message}</h1>
    {error && <p role="alert">{error}</p>}
    <p className="match-score">You {match.wins[0]} – {match.wins[1]} Opponent · First to two wins</p>
    {!match.finished && <>
      <h2>Sideboard for game {match.game + 1}</h2>
      <p>Move cards between your deck and sideboard. Your registered cards stay fixed. The bot keeps its deck unchanged.</p>
      <div className="sideboard-columns">
        {([true, false] as const).map((isMain) => <section key={String(isMain)}>
          <h3>{isMain ? "Main deck" : "Sideboard"} · {(isMain ? main : sideboard).length}</h3>
          <ul>{groups(isMain ? main : sideboard).map(({ card, count }) => <li key={card.id}>
            <span>{count} × {card.name}</span>
            <button onClick={() => move(card, isMain)} aria-label={`Move one ${card.name} to ${isMain ? "sideboard" : "main deck"}`}>{isMain ? "Out →" : "← In"}</button>
          </li>)}</ul>
        </section>)}
      </div>
      {match.humanChooses ? <label>Next game <select value={String(humanFirst)} onChange={(event) => setHumanFirst(event.target.value === "true")}>
        <option value="true">Play first</option><option value="false">Draw first</option>
      </select></label> : <p>The opponent chooses to play first.</p>}
      <div className="match-buttons">
        <button onClick={() => { setMain(match.main); setSideboard(match.sideboard); }}>Reset changes</button>
        <button className="result-primary" onClick={() => next(main.map((card) => card.id), sideboard.map((card) => card.id), humanFirst)}>Start game {match.game + 1}</button>
      </div>
    </>}
    <button onClick={newMatch}>New match</button>
  </section>;
}
