#!/usr/bin/env node
import * as dotenv from "dotenv"
import { getPolywrapClient } from "./getPolywrapClient";
import { runApp } from "./runApp";
import { initAppData } from "./initAppData";

dotenv.config();

const appDataRootPath = process.env.APPDATA || (process.platform == 'darwin' ? process.env.HOME + '/Library/Preferences' : process.env.HOME + "/.local/share");
export const appDataPath = `${appDataRootPath}/pwr`;
export const polywrapAppDataPath = `${appDataRootPath}/polywrap`;

export const paths = {
  cache: {
    wrappers: {
      ipfs: `${polywrapAppDataPath}/cache/wrappers/ipfs`,
    }
  }
};

(async () => {
  initAppData();

  const args = process.argv.slice(2);
  const uri = args[0];
  const appArgs = args.slice(1);

  await runApp(
    parseUri(uri), 
    appArgs,
    getPolywrapClient()
  );
})();

function parseUri(uri: string): string {
  if(uri.endsWith(".eth") && !uri.startsWith("wrap://ens/") && !uri.startsWith("ens/")) {
    return `wrap://ens/${uri}`;
  } else if (uri.startsWith("Qm")) {
    return `wrap://ipfs/${uri}`;
  } else if (uri.startsWith("ipfs://")) {
    return `wrap://ipfs/${uri.slice("ipfs://".length, uri.length)}`;
  } else if (uri.startsWith(".") || uri.startsWith("/")) {
    return `wrap://file/${uri}`;
  } else if (!uri.includes("/")) {
    return `wrap://pwr/${uri}`;
  } else {
    return uri;
  }
}
