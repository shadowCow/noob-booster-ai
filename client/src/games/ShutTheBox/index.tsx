import { AppDependencies } from "./App";
import { createGameAnalysisServiceImpl } from "./services/GameAnalysisService/GameAnalysisService.impl";

export function createShutTheBoxDepedencies(): AppDependencies {
  return {
    gameAnalysisService: createGameAnalysisServiceImpl(),
  };
}
