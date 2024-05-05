// Copyright (c) Sean Lawlor
//
// This source code is licensed under both the MIT license found in the
// LICENSE-MIT file in the root directory of this source tree.

#[macro_use]
extern crate criterion;

use criterion::{BatchSize, Criterion};
use ractor::{call_t, Actor, ActorProcessingErr, ActorRef, RpcReplyPort};

fn create_actors(c: &mut Criterion) {
    struct BenchActorMessage;
    struct BenchActor;

    impl Actor for BenchActor {
        type Msg = BenchActorMessage;
        type State = ();
        type Arguments = ();

        async fn pre_start(
            &self,
            _myself: ActorRef<Self::Msg>,
            _: (),
        ) -> Result<Self::State, ActorProcessingErr> {
            // let _ = myself.cast(BenchActorMessage);
            Ok(())
        }

        async fn handle(
            &self,
            _myself: ActorRef<Self::Msg>,
            _message: Self::Msg,
            _state: &mut Self::State,
        ) -> Result<(), ActorProcessingErr> {
            // myself.stop(None);
            Ok(())
        }
    }

    let small = 100;
    let large = 10000;

    let id = format!("Creation of {small} actors");
    let runtime = tokio::runtime::Builder::new_multi_thread().build().unwrap();
    c.bench_function(&id, move |b| {
        b.iter_batched(
            || {},
            |()| {
                runtime.block_on(async move {
                    let mut handles = vec![];
                    for _ in 0..small {
                        let (_, handler) = Actor::spawn(None, BenchActor, ())
                            .await
                            .expect("Failed to create test agent");
                        handles.push(handler);
                    }
                    handles
                })
            },
            BatchSize::PerIteration,
        );
    });

    let id = format!("Creation of {large} actors");
    let runtime = tokio::runtime::Builder::new_multi_thread().build().unwrap();
    c.bench_function(&id, move |b| {
        b.iter_batched(
            || {},
            |()| {
                runtime.block_on(async move {
                    let mut handles = vec![];
                    for _ in 0..large {
                        let (_, handler) = Actor::spawn(None, BenchActor, ())
                            .await
                            .expect("Failed to create test agent");
                        handles.push(handler);
                    }
                    handles
                })
            },
            BatchSize::PerIteration,
        );
    });
}

#[allow(clippy::async_yields_async)]
fn process_messages(c: &mut Criterion) {
    const NUM_MSGS: u64 = 100000;

    enum BenchActorMessage {
        Cast(u64),
        Call(u64, RpcReplyPort<u64>),
    }

    struct MessagingActor;

    impl Actor for MessagingActor {
        type Msg = BenchActorMessage;
        type State = u64;
        type Arguments = ();

        async fn pre_start(
            &self,
            _myself: ActorRef<Self::Msg>,
            _: (),
        ) -> Result<Self::State, ActorProcessingErr> {
            Ok(0u64)
        }

        async fn handle(
            &self,
            myself: ActorRef<Self::Msg>,
            message: Self::Msg,
            state: &mut Self::State,
        ) -> Result<(), ActorProcessingErr> {
            match message {
                BenchActorMessage::Cast(n) => {
                    *state += n;
                }
                BenchActorMessage::Call(n, reply_port) => {
                    *state += n;
                    reply_port.send(*state).unwrap_or_else(|_err| {});
                }
            }
            if *state >= NUM_MSGS {
                myself.stop(None);
            }
            Ok(())
        }
    }

    let id =
        format!("Waiting on {NUM_MSGS} messages to be processed [ by single-threaded + cast ]");
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    c.bench_function(&id, move |b| {
        b.iter_batched(
            || {
                runtime.block_on(async move {
                    let (actor_ref, handle) = Actor::spawn(None, MessagingActor, ())
                        .await
                        .expect("Failed to create test actor");
                    (actor_ref, handle)
                })
            },
            |(actor_ref, handle)| {
                runtime.block_on(async move {
                    for _ in 0..NUM_MSGS {
                        let _ = actor_ref.cast(BenchActorMessage::Cast(1));
                    }
                    let _ = handle.await;
                })
            },
            BatchSize::PerIteration,
        );
    });

    let id =
        format!("Waiting on {NUM_MSGS} messages to be processed [ by single-threaded + call ]");
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap();
    c.bench_function(&id, move |b| {
        b.iter_batched(
            || {
                runtime.block_on(async move {
                    let (actor_ref, handle) = Actor::spawn(None, MessagingActor, ())
                        .await
                        .expect("Failed to create test actor");
                    (actor_ref, handle)
                })
            },
            |(actor_ref, handle)| {
                runtime.block_on(async move {
                    for _ in 0..NUM_MSGS {
                        let _ = call_t!(actor_ref, BenchActorMessage::Call, 10, 1);
                    }
                    let _ = handle.await;
                })
            },
            BatchSize::PerIteration,
        );
    });

    let id = format!("Waiting on {NUM_MSGS} messages to be processed [ by multi-threaded + cast ]");
    let runtime = tokio::runtime::Builder::new_multi_thread().build().unwrap();
    c.bench_function(&id, move |b| {
        b.iter_batched(
            || {
                runtime.block_on(async move {
                    let (actor_ref, handle) = Actor::spawn(None, MessagingActor, ())
                        .await
                        .expect("Failed to create test actor");
                    (actor_ref, handle)
                })
            },
            |(actor_ref, handle)| {
                runtime.block_on(async move {
                    for _ in 0..NUM_MSGS {
                        let _ = actor_ref.cast(BenchActorMessage::Cast(1));
                    }
                    let _ = handle.await;
                })
            },
            BatchSize::PerIteration,
        );
    });

    let id = format!("Waiting on {NUM_MSGS} messages to be processed [ by multi-threaded + call ]");
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_time()
        .build()
        .unwrap();
    c.bench_function(&id, move |b| {
        b.iter_batched(
            || {
                runtime.block_on(async move {
                    let (actor_ref, handle) = Actor::spawn(None, MessagingActor, ())
                        .await
                        .expect("Failed to create test actor");
                    (actor_ref, handle)
                })
            },
            |(actor_ref, handle)| {
                runtime.block_on(async move {
                    for _ in 0..NUM_MSGS {
                        let _ = call_t!(actor_ref, BenchActorMessage::Call, 10, 1);
                    }
                    let _ = handle.await;
                })
            },
            BatchSize::PerIteration,
        );
    });
}

criterion_group!(ractor, create_actors, process_messages);
criterion_main!(ractor);
