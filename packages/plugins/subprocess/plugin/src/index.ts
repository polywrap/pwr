import {
  Client,
  Module,
  manifest,
  Args_exec,
  Subprocess_BufferEncodingEnum,
  Subprocess_BufferEncodingString,
  Args_spawn,
  Subprocess_IOType,
  Subprocess_IOTypeEnum,
  Logger_Module,
  Logger_LogLevelEnum,
} from "./wrap";

import { exec, IOType, spawn } from "child_process";
import util from "util";
import { PluginFactory } from "@polywrap/core-js";

const cpExec = util.promisify(exec);

type NoConfig = Record<string, never>;

function toBufferEncodingString(
  beEnum: Subprocess_BufferEncodingEnum | Subprocess_BufferEncodingString
): Subprocess_BufferEncodingString {
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

function parseStdioOption(stdio: Subprocess_IOType): IOType {
  switch (stdio) {
    case Subprocess_IOTypeEnum.ignore:
      return "ignore";
    case Subprocess_IOTypeEnum.inherit:
      return "inherit";
    case Subprocess_IOTypeEnum.pipe:
      return "pipe";
    case Subprocess_IOTypeEnum.overlapped:
      return "overlapped";
    default:
      return stdio;
  }
}

export class SubprocessPlugin extends Module<NoConfig> {
  constructor(config: NoConfig) {
    super(config);
  }

  async exec(args: Args_exec, client: Client) {
    return await cpExec(args.command, {
      cwd: args.options?.cwd ? args.options.cwd : (this.env.cwd as string),
      env: args.options?.env
        ? Object.fromEntries(args.options.env.entries())
        : undefined,
      encoding: args.options?.encoding
        ? toBufferEncodingString(args.options.encoding)
        : "utf8",
      shell: args.options?.shell ? args.options.shell : undefined,
    });
  }

  async spawn(args: Args_spawn, client: Client) {
    let child = spawn(args.command, {
      cwd: args.options?.cwd ? args.options.cwd : (this.env.cwd as string),
      env: args.options?.env
        ? Object.fromEntries(args.options.env.entries())
        : undefined,
      shell: args.options?.shell ? args.options.shell : undefined,
      stdio: args.options?.stdio
        ? parseStdioOption(args.options.stdio)
        : undefined,
    });
    const logName = this.env.processId
      ? `subprocess.${this.env.processId}`
      : "subprocess";

    if (child.stdout) {
      for await (const data of child.stdout) {
        Logger_Module.log(
          {
            message: `${logName}: INFO - ${data}`,
            level: Logger_LogLevelEnum.INFO,
          },
          client
        );
      }
    }

    if (child.stderr) {
      for await (const error of child.stderr) {
        Logger_Module.log(
          {
            message: `${logName}: ERROR - ${error}`,
            level: Logger_LogLevelEnum.ERROR,
          },
          client
        );
      }
    }

    return true;
  }
}
export const subprocessPlugin: PluginFactory<NoConfig> = () => {
  return {
    factory: () => new SubprocessPlugin({}),
    manifest,
  };
};

export const plugin = subprocessPlugin;
