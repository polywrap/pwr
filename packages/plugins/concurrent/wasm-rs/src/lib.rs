pub mod executor;
pub mod invoke_future;
pub mod wrap;

use JSON::json;
pub use executor::*;
pub use invoke_future::*;
pub use polywrap_wasm_rs::JSON;
use wrap::*;

pub fn run(_: ArgsRun) -> bool {
    let (executor, spawner) = new_executor_and_spawner();

    // Spawn a task to print before and after waiting on a timer.
    spawner.spawn(async {
        println!("spawn!");
        // Wait for our timer future to complete after two seconds.
        let result = InvokeFuture::<String>::new(
            ConcurrentTask {
                uri: "ens/hello.eth".to_string(),
                method: "hello".to_string(),
                args: json!({"greet": "Hello World!"}),
            },
            1000,
        )
        .await;
        println!("result!");
        println!("{}", &result.unwrap());
        println!("done!");
    });

    // Drop the spawner so that our executor knows it is finished and won't
    // receive more incoming tasks to run.
    drop(spawner);

    // Run the executor until the task queue is empty.
    // This will print "howdy!", pause, and then print "done!".
    executor.run();

    return true;
}
