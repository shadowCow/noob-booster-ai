import styles from "@/styles/splendor.module.css";
import {
  Card,
  CardTier,
  GemColor,
  GemCounts,
  LocationTile,
  initSplendorState,
  transition,
} from "./splendor_rules";
import { assertNever } from "../../utils/adt";
import { Deck } from "../card_utils/card_utils";
import { useReducer } from "react";

export function Splendor() {
  const [state, dispatch] = useReducer(transition, initSplendorState());
  return (
    <div className={styles.game}>
      <SharedBoardView
        tier1Deck={state.cardStacks[0]}
        tier2Deck={state.cardStacks[1]}
        tier3Deck={state.cardStacks[2]}
        tier1UpCards={state.board[0]}
        tier2UpCards={state.board[1]}
        tier3UpCards={state.board[2]}
      />
    </div>
  );
}

function SharedBoardView(props: {
  tier1Deck: Deck<Card>;
  tier2Deck: Deck<Card>;
  tier3Deck: Deck<Card>;
  tier1UpCards: Array<Card | undefined>;
  tier2UpCards: Array<Card | undefined>;
  tier3UpCards: Array<Card | undefined>;
}) {
  return (
    <div className={styles.shared_board}>
      <TierView tier={3} deck={props.tier3Deck} upCards={props.tier3UpCards} />
      <TierView tier={2} deck={props.tier2Deck} upCards={props.tier2UpCards} />
      <TierView tier={1} deck={props.tier1Deck} upCards={props.tier1UpCards} />
    </div>
  );
}

function GemBankView(props: { gems: GemCounts }) {}

function LocationTilesView(props: { tiles: Array<LocationTile> }) {}

function LocationTileView(props: { tile: LocationTile }) {
  return <div className={styles.location_tile}></div>;
}

function TierView(props: {
  tier: CardTier;
  deck: Array<Card>;
  upCards: Array<Card | undefined>;
}) {
  return (
    <div className={styles.tier}>
      <DeckView tier={props.tier} size={props.deck.length} />
      <div className={styles.tier_cards}>
        {props.upCards.map((card) =>
          card === undefined ? (
            <EmptyCardView />
          ) : (
            <CardView key={card.name} card={card} />
          )
        )}
      </div>
    </div>
  );
}

function DeckView(props: { tier: CardTier; size: number }) {
  const content = props.size === 0 ? <div>Empty</div> : <p>{props.tier}</p>;

  return <div className={styles.deck}>{content}</div>;
}

function CardView(props: { card: Card }) {
  const colorStyle = getColorStyle(props.card.color);

  return (
    <div className={styles.card}>
      <div className={`${styles.card_header} ${styles.colorStyle}`}>
        <p className={styles.card_name}>{props.card.name}</p>
        <div className={styles.card_avenger_symbols}>
          {getAvengerSymbols(props.card.avengerCount).map((a) => (
            <p>A</p>
          ))}
        </div>
        <p className={styles.card_points}>{props.card.points}</p>
      </div>
      <div className={styles.card_body}>
        <div className={styles.card_cost}></div>
        <div className={styles.card_graphic}></div>
      </div>
    </div>
  );
}

function EmptyCardView() {
  return <div className={styles.empty_card}></div>;
}

function GemCostView(props: { color: GemColor; cost: number }) {}

function GemView(props: { color: GemColor }) {}

function PlayerView() {}

function getAvengerSymbols(count: number) {
  const symbols: Array<string> = [];
  for (let i = 0; i < count; i++) {
    symbols.push("A");
  }

  return symbols;
}

function getColorStyle(gemColor: GemColor): string {
  switch (gemColor) {
    case "purple":
      return styles.purple;
    case "red":
      return styles.red;
    case "orange":
      return styles.orange;
    case "blue":
      return styles.blue;
    case "yellow":
      return styles.yellow;
    default:
      assertNever(gemColor);
  }
}
