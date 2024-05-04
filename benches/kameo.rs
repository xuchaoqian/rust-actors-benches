#[macro_use]
extern crate criterion;

use std::future::Future;

use criterion::{BatchSize, Criterion};
use kameo::{
    actor::ActorRef,
    error::BoxError,
    message::{Context, Message},
    Actor,
};

fn create_actors(c: &mut Criterion) {
    struct BenchActorMessage;
    struct BenchActor;

    impl Actor for BenchActor {
        fn on_start(
            &mut self,
            _actor_ref: ActorRef<Self>,
        ) -> impl Future<Output = Result<(), BoxError>> + Send {
            async {
                // actor_ref.send(BenchActorMessage).await.unwrap();
                Ok(())
            }
        }
    }

    impl Message<BenchActorMessage> for BenchActor {
        type Reply = ();

        async fn handle(
            &mut self,
            _msg: BenchActorMessage,
            _ctx: Context<'_, Self, Self::Reply>,
        ) -> Self::Reply {
            // ctx.actor_ref().kill();
            ()
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
                        let handler = kameo::actor::spawn_unsync(BenchActor);
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
                        let handler = kameo::actor::spawn_unsync(BenchActor);
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

    struct BenchActorMessage {
        n: u64,
    }

    struct MessagingActor {
        state: u64,
    }

    impl Actor for MessagingActor {
        fn on_start(
            &mut self,
            _actor_ref: ActorRef<Self>,
        ) -> impl Future<Output = Result<(), BoxError>> + Send {
            async { Ok(()) }
        }
    }

    impl Message<BenchActorMessage> for MessagingActor {
        type Reply = u64;

        async fn handle(
            &mut self,
            msg: BenchActorMessage,
            ctx: Context<'_, Self, Self::Reply>,
        ) -> Self::Reply {
            self.state += msg.n;
            if self.state >= NUM_MSGS {
                ctx.actor_ref().kill();
            }
            self.state
        }
    }

    let id = format!(
        "Waiting on {NUM_MSGS} messages to be processed [ by single-threaded + send_async ]"
    );
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    c.bench_function(&id, move |b| {
        b.iter_batched(
            || {
                runtime.block_on(
                    async move { kameo::actor::spawn_unsync(MessagingActor { state: 0 }) },
                )
            },
            |actor_ref| {
                runtime.block_on(async move {
                    for _ in 0..NUM_MSGS {
                        let _ = actor_ref.send_async(BenchActorMessage { n: 1 });
                    }
                    actor_ref
                })
            },
            BatchSize::PerIteration,
        );
    });

    let id =
        format!("Waiting on {NUM_MSGS} messages to be processed [ by single-threaded + send ]");
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap();
    c.bench_function(&id, move |b| {
        b.iter_batched(
            || {
                runtime.block_on(
                    async move { kameo::actor::spawn_unsync(MessagingActor { state: 0 }) },
                )
            },
            |actor_ref| {
                runtime.block_on(async move {
                    for _ in 0..NUM_MSGS {
                        let _ = actor_ref.send(BenchActorMessage { n: 1 }).await;
                    }
                    actor_ref
                })
            },
            BatchSize::PerIteration,
        );
    });

    let id = format!(
        "Waiting on {NUM_MSGS} messages to be processed [ by multi-threaded + send_async ]"
    );
    let runtime = tokio::runtime::Builder::new_multi_thread().build().unwrap();
    c.bench_function(&id, move |b| {
        b.iter_batched(
            || {
                runtime.block_on(
                    async move { kameo::actor::spawn_unsync(MessagingActor { state: 0 }) },
                )
            },
            |actor_ref| {
                runtime.block_on(async move {
                    for _ in 0..NUM_MSGS {
                        let _ = actor_ref.send_async(BenchActorMessage { n: 1 });
                    }
                    actor_ref
                })
            },
            BatchSize::PerIteration,
        );
    });

    let id = format!("Waiting on {NUM_MSGS} messages to be processed [ by multi-threaded + send ]");
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_time()
        .build()
        .unwrap();
    c.bench_function(&id, move |b| {
        b.iter_batched(
            || {
                runtime.block_on(
                    async move { kameo::actor::spawn_unsync(MessagingActor { state: 0 }) },
                )
            },
            |actor_ref| {
                runtime.block_on(async move {
                    for _ in 0..NUM_MSGS {
                        let _ = actor_ref.send(BenchActorMessage { n: 1 }).await;
                    }
                    actor_ref
                })
            },
            BatchSize::PerIteration,
        );
    });
}

criterion_group!(kameo, create_actors, process_messages);
criterion_main!(kameo);
