import { Inter } from "next/font/google";
import { createShutTheBoxDepedencies } from "../games/ShutTheBox";
import { App } from "../games/ShutTheBox/App";

const inter = Inter({ subsets: ["latin"] });

const appDependencies = createShutTheBoxDepedencies();

export default function Home() {
  return (
    <>
      <App appDependencies={appDependencies} />
    </>
  );
}
