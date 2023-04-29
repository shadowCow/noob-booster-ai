import React from 'react';


import {GameState, Tile} from '../GameState';

export type TilesViewProps = {
    gameState: GameState,
    toggleTileOpen: (tile: Tile) => void,
}

export function TilesView(props: TilesViewProps): JSX.Element {

    return <div className="tiles">
        {props.gameState.tiles.map(tile =>
            <TileView tile={tile}
                key={tile.value}
                toggleTileOpen={props.toggleTileOpen}
            />
        )}
    </div>
}

type TileViewProps = {
    tile: Tile,
    toggleTileOpen: (tile: Tile) => void,
}

function TileView(props: TileViewProps): JSX.Element {
    const className = props.tile.isOpen
        ? "tile tile_open"
        : "tile tile_closed";

    const tileValueContent = props.tile.isOpen
        ? props.tile.value
        : null;

    return <div className={className}
        onClick={() => props.toggleTileOpen(props.tile)}
    >
        <span className="tile_value">{tileValueContent}</span>
    </div>
}