import { Action, GameState, pipsFor } from "../../GameState";
import { GameAnalysisService } from "./GameAnalysisService";

export function createGameAnalysisServiceImpl(): GameAnalysisService {
  const service: GameAnalysisService = {
    findBestAction: async (gameState) => {
      const gameStateRequestBody = createGameStateRequestBody(gameState);

      return fetch(`/shut-the-box/find-best-action`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(gameStateRequestBody),
      })
        .then((r) => r.json())
        .then((json) => deserBestActionResponse(json))
        .catch((err) => {
          console.error(err);
          return undefined;
        });
    },
  };

  return service;
}

function deserBestActionResponse(rawJson: any): Action | undefined {
  console.log("best action response", rawJson);
  if ("action" in rawJson) {
    const action = rawJson["action"];

    if (action === null) {
      return undefined;
    } else if (isAction(action)) {
      return action;
    } else {
      throw new Error(`invalid action returned ${action}`);
    }
  } else {
    throw new Error(`invalid BestActionResponse ${rawJson}`);
  }
}

function isAction(maybeAction: any): maybeAction is Action {
  return (
    Array.isArray(maybeAction) &&
    maybeAction.every((t) => typeof t === "number")
  );
}

type GameStateRequestBody = {
  dice_value: number;
  tiles_open: Array<boolean>;
};

function createGameStateRequestBody(
  gameState: GameState
): GameStateRequestBody {
  return {
    dice_value: pipsFor(gameState.d1) + pipsFor(gameState.d2),
    tiles_open: gameState.tiles.map((t) => t.isOpen),
  };
}
