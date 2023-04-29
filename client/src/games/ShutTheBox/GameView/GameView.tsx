import styles from "@/styles/GameView.module.css";
import { DiceView } from "../DiceView/DiceView";
import { D6, GameState, Tile } from "../GameState";
import { TilesView } from "../TilesView/TilesView";

export type GameViewProps = {
  gameState: GameState;
  toggleTileOpen: (tile: Tile) => void;
  setDie1: (die: D6) => void;
  setDie2: (die: D6) => void;
};

export function GameView(props: GameViewProps): JSX.Element {
  return (
    <div className={`${styles.game_view}`}>
      <div className={`${styles.board}`}>
        <TilesView
          gameState={props.gameState}
          toggleTileOpen={props.toggleTileOpen}
        />
        <div className={`${styles.spacer}`}></div>
        <DiceView
          gameState={props.gameState}
          setDie1={props.setDie1}
          setDie2={props.setDie2}
        />
      </div>
    </div>
  );
}
