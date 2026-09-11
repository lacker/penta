"use client";

import { CardArt } from "./CardArt";
import type { CardArtMode } from "./card-art-mode";
import type { Card, CommanderHistory, Owner } from "./game-types";

type Props = {
  cards: Card[];
  commanders: CommanderHistory[];
  owner: Owner;
  cardArtMode: CardArtMode;
  actionCount?: (id: number) => number;
  selectedCard?: number | null;
  onSelect?: (id: number) => void;
};

const historyFor = (card: Card, commanders: CommanderHistory[]) =>
  commanders.find((commander) => commander.object === card.id);

function CommanderStats({ commander }: { commander: CommanderHistory }) {
  return (
    <small className="commander-stats">
      Tax +{commander.commandZoneCasts * 2} · casts {commander.commandZoneCasts}
      <br />
      Damage: you {commander.combatDamage.human} · opponent {commander.combatDamage.opponent}
    </small>
  );
}

/** Public designated-card state and the player-owned cards currently castable
 * from the command zone. Keeping this separate from the battlefield makes the
 * ordinary game board unchanged when no commanders were selected. */
export function CommandZone({
  cards,
  commanders,
  owner,
  cardArtMode,
  actionCount = () => 0,
  selectedCard = null,
  onSelect,
}: Props) {
  if (cards.length === 0 && commanders.length === 0) return null;
  const inZone = new Set(cards.map((card) => card.id));
  const label = owner === "human" ? "Your command zone" : "Opponent command zone";

  return (
    <section className={`command-zone command-zone-${owner}`} aria-label={label}>
      <div className="command-zone-heading">
        <strong>{label}</strong>
        <span>Public commanders</span>
      </div>
      <div className="command-zone-content">
        {cards.map((card) => {
          const commander = historyFor(card, commanders);
          const actionable = owner === "human" && actionCount(card.id) > 0;
          const body = (
            <>
              <CardArt
                mode={cardArtMode}
                cardKind={card.kind}
                scryfallId={card.art?.scryfallId ?? ""}
              />
              <span className="command-zone-card-name">{card.name}</span>
              {commander && <CommanderStats commander={commander} />}
            </>
          );
          return actionable || owner === "human" ? (
            <button
              className={`command-zone-card ${card.id === selectedCard ? "is-selected" : ""}`}
              key={card.id}
              type="button"
              disabled={!actionable}
              onClick={() => onSelect?.(card.id)}
            >
              {body}
            </button>
          ) : (
            <div className="command-zone-card" key={card.id}>{body}</div>
          );
        })}
        {commanders.filter((commander) => commander.object == null || !inZone.has(commander.object)).map((commander) => (
          <div className="commander-history" key={`${commander.owner}-${commander.definition}`}>
            <strong>{commander.name}</strong>
            <span>Away from the command zone</span>
            <CommanderStats commander={commander} />
          </div>
        ))}
      </div>
    </section>
  );
}
