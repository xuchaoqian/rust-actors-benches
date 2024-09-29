#[macro_use]
extern crate criterion;

use std::future::Future;

use criterion::{BatchSize, Criterion};
use kameo::{
    actor::ActorRef,
    error::BoxError,
    mailbox::unbounded::UnboundedMailbox,
    message::{Context, Message},
    request::{MessageSend, MessageSendSync},
    Actor,
};

fn create_actors(c: &mut Criterion) {
    struct BenchActorMessage;
    struct BenchActor;

    impl Actor for BenchActor {
        type Mailbox = UnboundedMailbox<Self>;
    }

    impl Message<BenchActorMessage> for BenchActor {
        type Reply = ();

        async fn handle(
            &mut self,
            _msg: BenchActorMessage,
            _ctx: Context<'_, Self, Self::Reply>,
        ) -> Self::Reply {
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
                    let mut actor_refs = vec![];
                    for _ in 0..small {
                        let actor_ref = kameo::actor::spawn(BenchActor);
                        actor_refs.push(actor_ref);
                    }
                    actor_refs
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
                    let mut actor_refs = vec![];
                    for _ in 0..large {
                        let actor_ref = kameo::actor::spawn(BenchActor);
                        actor_refs.push(actor_ref);
                    }
                    actor_refs
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
        type Mailbox = UnboundedMailbox<Self>;

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

    let id =
        format!("Waiting on {NUM_MSGS} messages to be processed [ by single-threaded + tell ]");
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    c.bench_function(&id, move |b| {
        b.iter_batched(
            || runtime.block_on(async move { kameo::actor::spawn(MessagingActor { state: 0 }) }),
            |actor_ref| {
                for _ in 0..NUM_MSGS {
                    let _ = actor_ref.tell(BenchActorMessage { n: 1 }).send_sync();
                }
                actor_ref
            },
            BatchSize::PerIteration,
        );
    });

    let id = format!("Waiting on {NUM_MSGS} messages to be processed [ by single-threaded + ask ]");
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap();
    c.bench_function(&id, move |b| {
        b.iter_batched(
            || runtime.block_on(async move { kameo::actor::spawn(MessagingActor { state: 0 }) }),
            |actor_ref| {
                runtime.block_on(async move {
                    for _ in 0..NUM_MSGS {
                        let _ = actor_ref.ask(BenchActorMessage { n: 1 }).send().await;
                    }
                    actor_ref
                })
            },
            BatchSize::PerIteration,
        );
    });

    let id = format!("Waiting on {NUM_MSGS} messages to be processed [ by multi-threaded + tell ]");
    let runtime = tokio::runtime::Builder::new_multi_thread().build().unwrap();
    c.bench_function(&id, move |b| {
        b.iter_batched(
            || runtime.block_on(async move { kameo::actor::spawn(MessagingActor { state: 0 }) }),
            |actor_ref| {
                for _ in 0..NUM_MSGS {
                    let _ = actor_ref.tell(BenchActorMessage { n: 1 }).send_sync();
                }
                actor_ref
            },
            BatchSize::PerIteration,
        );
    });

    let id = format!("Waiting on {NUM_MSGS} messages to be processed [ by multi-threaded + ask ]");
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_time()
        .build()
        .unwrap();
    c.bench_function(&id, move |b| {
        b.iter_batched(
            || runtime.block_on(async move { kameo::actor::spawn(MessagingActor { state: 0 }) }),
            |actor_ref| {
                runtime.block_on(async move {
                    for _ in 0..NUM_MSGS {
                        let _ = actor_ref.ask(BenchActorMessage { n: 1 }).send().await;
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
