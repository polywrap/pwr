import fs from "fs";
import { polywrapAppDataPath } from "./main";

export const initAppData = () => {
  !fs.existsSync(polywrapAppDataPath) && fs.mkdirSync(polywrapAppDataPath);
  !fs.existsSync(`${polywrapAppDataPath}/cache`) && fs.mkdirSync(`${polywrapAppDataPath}/cache`);
  !fs.existsSync(`${polywrapAppDataPath}/cache/wrappers`) && fs.mkdirSync(`${polywrapAppDataPath}/cache/wrappers`);
  !fs.existsSync(`${polywrapAppDataPath}/cache/wrappers/ipfs`) && fs.mkdirSync(`${polywrapAppDataPath}/cache/wrappers/ipfs`);
};
