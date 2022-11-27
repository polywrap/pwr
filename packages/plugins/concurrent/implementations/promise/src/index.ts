import { Client, InvokeResult, PluginFactory } from "@polywrap/core-js";
// import { msgpackEncode } from "@polywrap/msgpack-js";
import {
  Args_result,
  Args_schedule,
  // Args_status,
  Int,
  // Interface_ReturnWhenEnum,
  Interface_Task,
  Interface_TaskResult,
  Interface_TaskStatus,
  Interface_TaskStatusEnum,
  manifest,
  Module,
} from "./wrap";

function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export interface ConcurrentPromisePluginConfig extends Record<string, unknown> {
  cache?: Map<string, string>;
}

export class ConcurrentPromisePlugin extends Module<ConcurrentPromisePluginConfig> {
  private _totalTasks = 0;
  private _tasks: Map<number, Promise<InvokeResult>> = new Map();
  private _status: Map<number, Interface_TaskStatus> = new Map();

  constructor(config: ConcurrentPromisePluginConfig) {
    super(config);
  }

  // public async result(
  //   input: Args_result,
  //   client: Client
  // ): Promise<Array<Interface_TaskResult>> {
  //   switch (input.returnWhen) {
  //     case Interface_ReturnWhenEnum.FIRST_COMPLETED: {
  //       const result = await Promise.race(
  //         input.taskIds.map((id) => this.resolveTask(id))
  //       );
  //       return [result];
  //     }
  //     case Interface_ReturnWhenEnum.ALL_COMPLETED: {
  //       const results = await Promise.all(
  //         input.taskIds.map((id) => this.resolveTask(id))
  //       );
  //       return results;
  //     }
  //     default: {
  //       throw new Error("Not Implemented");
  //     }
  //   }
  // }

  public async result(
    input: Args_result,
    client: Client
  ): Promise<Interface_TaskResult | null> {
    console.log(this._status.get(input.taskId));
    if (
      this._status.get(input.taskId) === Interface_TaskStatusEnum.COMPLETED ||
      this._status.get(input.taskId) === Interface_TaskStatusEnum.FAILED ||
      this._status.get(input.taskId) === Interface_TaskStatusEnum.CANCELLED
    ) {
      return await this.resolveTask(input.taskId);
    }
    await sleep(input.timeout);
    if (
      this._status.get(input.taskId) === Interface_TaskStatusEnum.COMPLETED ||
      this._status.get(input.taskId) === Interface_TaskStatusEnum.FAILED ||
      this._status.get(input.taskId) === Interface_TaskStatusEnum.CANCELLED
    ) {
      return await this.resolveTask(input.taskId);
    }
    return null;
  }

  // public async status(
  //   input: Args_status,
  //   client: Client
  // ): Promise<Array<Interface_TaskStatus>> {
  //   return input.taskIds.map((id) => this._status[id]);
  // }

  public schedule(input: Args_schedule, client: Client): Int {
    return this.scheduleTask(input.task, client);
    // return input.tasks.map((task) => {
    //   const taskId = this.scheduleTask(
    //     {
    //       ...task,
    //     },
    //     client
    //   );
    //   return taskId;
    // });
  }

  private scheduleTask(task: Interface_Task, client: Client): number {
    const taskId = this._totalTasks;
    console.log("*****************");
    this._tasks.set(
      taskId,
      client
        .invoke({
          uri: task.uri,
          method: task.method,
          args: JSON.parse(task.args),
        })
        .then((res) => {
          this._status.set(
            taskId,
            res.ok
              ? Interface_TaskStatusEnum.COMPLETED
              : Interface_TaskStatusEnum.FAILED
          );
          return res;
        }).catch((err) => {
          this._status.set(taskId, Interface_TaskStatusEnum.FAILED);
          throw err;
        })
    );
    this._status.set(taskId, Interface_TaskStatusEnum.RUNNING);
    this._totalTasks += 1;
    return taskId;
  }

  private resolveTask(taskId: number): Promise<Interface_TaskResult> {
    const task = this._tasks.get(taskId);
    if (!task) throw new Error;
    return task
      .then((result: InvokeResult) => {
        if (!result.ok) {
          return {
            taskId,
            result: JSON.stringify(undefined),
            error: result.error?.message,
            status: Interface_TaskStatusEnum.FAILED,
          } as Interface_TaskResult;
        }
        return {
          taskId: taskId,
          result: JSON.stringify(result.value),
          error: undefined,
          status: Interface_TaskStatusEnum.COMPLETED,
        } as Interface_TaskResult;
      })
      .catch((err) => {
        return {
          taskId: taskId,
          result: JSON.stringify(undefined),
          error: err.message as string,
          status: Interface_TaskStatusEnum.FAILED,
        } as Interface_TaskResult;
      });
  }
}

export const concurrentPromisePlugin: PluginFactory<
  ConcurrentPromisePluginConfig
> = (config: ConcurrentPromisePluginConfig) => {
  return {
    factory: () => new ConcurrentPromisePlugin(config),
    manifest: manifest,
  };
};

export const plugin = concurrentPromisePlugin;
