import React from 'react';

import { GameAnalysisView } from './GameAnalysisView/GameAnalysisView';
import { GameState, createGameState, Tile, D6 } from './GameState';
import { GameView } from './GameView/GameView';
import { GameAnalysisService } from './services/GameAnalysisService/GameAnalysisService';

export type AppProps = {
  appDependencies: AppDependencies,
}
export type AppDependencies = {
  gameAnalysisService: GameAnalysisService,
}
export function App(props: AppProps) {
  const [gameState, setGameState] = React.useState<GameState>(createGameState());

  const toggleTileOpen = React.useCallback((tile: Tile) => {
      let indexOfTile = tile.value - 1;
      let updatedTiles = [...gameState.tiles];
      updatedTiles[indexOfTile] = {
          ...updatedTiles[indexOfTile],
          isOpen: !updatedTiles[indexOfTile].isOpen,
      };

      setGameState({
          ...gameState,
          tiles: updatedTiles,
      })
  }, [gameState])

  const setDie1 = React.useCallback((pips: D6) => {
      setGameState({
          ...gameState,
          d1: pips,
      })
  }, [gameState]);

  const setDie2 = React.useCallback((pips: D6) => {
      setGameState({
          ...gameState,
          d2: pips,
      })
  }, [gameState]);

  return (
    <div className="App">
      <GameView gameState={gameState}
        toggleTileOpen={toggleTileOpen}
        setDie1={setDie1}
        setDie2={setDie2}
      />
      <GameAnalysisView gameState={gameState}
        gameAnalysisService={props.appDependencies.gameAnalysisService}
      />
    </div>
  );
}
