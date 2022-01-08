import { Action, GameState } from "../../GameState";

export type GameAnalysisService = {
    findBestAction: (gameState: GameState) => Promise<Action | undefined>;
}