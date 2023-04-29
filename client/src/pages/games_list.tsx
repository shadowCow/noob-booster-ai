import styles from "@/styles/games_list.module.css";
import { useState } from "react";
import { ShutTheBox } from "../games/ShutTheBox/App";
import { GameAnalysisService } from "../games/ShutTheBox/services/GameAnalysisService/GameAnalysisService";

type AvailableGames = "shut_the_box";
const games: Array<AvailableGames> = ["shut_the_box"];

function gameDisplayName(game: AvailableGames): string {
  switch (game) {
    case "shut_the_box":
      return "Shut the Box";
    default:
      return "Unknown Game";
  }
}

export function AllGamesView(props: {
  gameAnalysisService: GameAnalysisService;
}) {
  const [selectedGame, setSelectedGame] =
    useState<AvailableGames>("shut_the_box");
  return (
    <div className={styles.container}>
      <GamesList games={games} setSelectedGame={setSelectedGame} />
      <GameArea
        game={selectedGame}
        gameAnalysisService={props.gameAnalysisService}
      />
    </div>
  );
}

function GamesList(props: {
  games: Array<AvailableGames>;
  setSelectedGame: (game: AvailableGames) => void;
}) {
  return (
    <ul className={styles.list}>
      {props.games.map((game) => (
        <li key={game} onClick={() => props.setSelectedGame(game)}>
          <p>{gameDisplayName(game)}</p>
        </li>
      ))}
    </ul>
  );
}

function GameArea(props: {
  game: AvailableGames;
  gameAnalysisService: GameAnalysisService;
}) {
  switch (props.game) {
    case "shut_the_box":
      return (
        <ShutTheBox
          appDependencies={{ gameAnalysisService: props.gameAnalysisService }}
        />
      );
    default:
      return <div>No Game Selected</div>;
  }
}
