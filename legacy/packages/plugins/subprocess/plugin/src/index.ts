import {
  Client,
  Module,
  manifest,
  Args_exec,
  Subprocess_BufferEncodingEnum,
  Subprocess_BufferEncodingString,
} from "./wrap";

import { exec } from "child_process";
import util from "util";
import { PluginFactory } from "@polywrap/core-js";

const cpExec = util.promisify(exec);


type NoConfig = Record<string, never>;

function toBufferEncodingString(beEnum: Subprocess_BufferEncodingEnum | Subprocess_BufferEncodingString): Subprocess_BufferEncodingString {
  switch (beEnum) {
    case Subprocess_BufferEncodingEnum.ascii:
      return "ascii";
    case Subprocess_BufferEncodingEnum.base64:
      return "base64";
    case Subprocess_BufferEncodingEnum.base64url:
      return "base64url";
    case Subprocess_BufferEncodingEnum.binary:
      return "binary";
    case Subprocess_BufferEncodingEnum.hex:
      return "hex";
    case Subprocess_BufferEncodingEnum.latin1:
      return "latin1";
    case Subprocess_BufferEncodingEnum.ucs2:
      return "ucs2";
    case Subprocess_BufferEncodingEnum.utf16le:
      return "utf16le";
    case Subprocess_BufferEncodingEnum.utf8:
      return "utf8";
    default:
      return beEnum;
  }
}

export class SubprocessPlugin extends Module<NoConfig> {
  constructor(config: NoConfig) {
    super(config);
  }

  async exec(args: Args_exec, client: Client) {
    return await cpExec(args.command, {
      cwd: args.options?.cwd ? args.options.cwd : (this.env.cwd as string),
      env: args.options?.env ? Object.fromEntries(args.options.env.entries()) : undefined,
      encoding: args.options?.encoding ? toBufferEncodingString(args.options.encoding) : "utf8",
      shell: args.options?.shell ? args.options.shell : undefined
    })
  }

}
export const subprocessPlugin: PluginFactory<NoConfig> = () => {
  return {
    factory: () => new SubprocessPlugin({}),
    manifest,
  };
};

export const plugin = subprocessPlugin;
