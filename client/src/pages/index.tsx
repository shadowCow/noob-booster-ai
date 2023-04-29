import { Inter } from "next/font/google";
import { createShutTheBoxDepedencies } from "../games/ShutTheBox";
import { ShutTheBox } from "../games/ShutTheBox/App";
import { ApiTest } from "./api_test";
import { AllGamesView } from "./games_list";

const inter = Inter({ subsets: ["latin"] });

const appDependencies = createShutTheBoxDepedencies();

export default function Home() {
  return (
    <>
      <AllGamesView gameAnalysisService={appDependencies.gameAnalysisService} />
    </>
  );
}
