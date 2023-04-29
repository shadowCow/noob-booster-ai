import styles from "@/styles/Tiles.module.css";
import { GameState, Tile } from "../GameState";

export type TilesViewProps = {
  gameState: GameState;
  toggleTileOpen: (tile: Tile) => void;
};

export function TilesView(props: TilesViewProps): JSX.Element {
  return (
    <div className={`${styles.tiles}`}>
      {props.gameState.tiles.map((tile) => (
        <TileView
          tile={tile}
          key={tile.value}
          toggleTileOpen={props.toggleTileOpen}
        />
      ))}
    </div>
  );
}

type TileViewProps = {
  tile: Tile;
  toggleTileOpen: (tile: Tile) => void;
};

function TileView(props: TileViewProps): JSX.Element {
  const openOrClosedClass = props.tile.isOpen
    ? styles.tile_open
    : styles.tile_closed;

  const tileValueContent = props.tile.isOpen ? props.tile.value : null;

  return (
    <div
      className={`${styles.tile} ${openOrClosedClass}`}
      onClick={() => props.toggleTileOpen(props.tile)}
    >
      <span className={`${styles.tile_value}`}>{tileValueContent}</span>
    </div>
  );
}
