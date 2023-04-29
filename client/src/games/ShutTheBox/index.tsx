import { ShutTheBoxDependencies } from "./App";
import { createGameAnalysisServiceImpl } from "./services/GameAnalysisService/GameAnalysisService.impl";

export function createShutTheBoxDepedencies(): ShutTheBoxDependencies {
  return {
    gameAnalysisService: createGameAnalysisServiceImpl(),
  };
}
