pub use crate::wrap::imported::*;

use polywrap_wasm_rs::JSON;
use serde::de::DeserializeOwned;

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};

pub struct State<T> {
    id: i32,
    timeout: u32,
    result: Option<Result<T, String>>,
    waker: Option<Waker>,
}

pub struct InvokeFuture<T: DeserializeOwned> {
    state: Box<State<T>>,
}

impl<T: DeserializeOwned> Future for InvokeFuture<T> {
    type Output = Result<T, String>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.state.result.is_some() {
            return Poll::Ready(self.state.result.take().unwrap());
        }

        match ConcurrentModule::result(&ArgsResult {
            task_id: self.state.id.clone(),
            timeout: self.state.timeout.clone(),
        }) {
            Ok(Some(task_res)) => match JSON::from_value::<T>(task_res.result) {
                Ok(v) => Poll::Ready(Ok(v)),
                Err(e) => Poll::Ready(Err(e.to_string())),
            },
            Ok(None) => {
                cx.waker().clone().wake(); // Reschedule in the queue to run in the future
                Poll::Pending
            },
            Err(err) => Poll::Ready(Err(err)),
        }
    }
}

impl<T: DeserializeOwned> InvokeFuture<T> {
    pub fn new(task: ConcurrentTask, timeout: u32) -> Self {
        let result = ConcurrentModule::schedule(&ArgsSchedule { task });

        if result.is_err() {
            let state: State<T> = State {
                id: -1,
                timeout,
                result: Some(Err(result.unwrap_err())),
                waker: None,
            };
            return InvokeFuture {
                state: Box::new(state),
            };
        }

        let id = result.unwrap();
        let state: State<T> = State {
            id,
            timeout,
            result: None,
            waker: None,
        };
        return InvokeFuture {
            state: Box::new(state),
        };
    }
}


// Executor task1 (3), task2 (2)

// ready_queue -> [task1, task2]

// executor -> task1.poll(waker) 
// waker.wake -> ready_queue.push(task1)

// ready_queue -> [task2, task1]

// InvokeFuture::new(task: ).await;