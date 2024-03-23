#[macro_use]
extern crate criterion;

use actix::prelude::*;
use criterion::{BatchSize, Criterion};

fn create_actors(c: &mut Criterion) {
    #[derive(Message)]
    #[rtype(result = "()")]
    struct BenchActorMessage;

    struct BenchActor;

    impl Actor for BenchActor {
        type Context = Context<Self>;

        fn started(&mut self, ctx: &mut Self::Context) {
            ctx.notify(BenchActorMessage);
        }
    }

    impl Handler<BenchActorMessage> for BenchActor {
        type Result = ();

        fn handle(&mut self, _msg: BenchActorMessage, ctx: &mut Context<Self>) -> Self::Result {
            ctx.stop();
            ()
        }
    }

    let small = 100;
    let large = 10000;

    let id = format!("Creation of {small} actors");
    let system = System::new();
    c.bench_function(&id, |b| {
        b.iter_batched(
            || {},
            |()| {
                system.block_on(async move {
                    let mut handles = vec![];
                    for _ in 0..small {
                        let handle = BenchActor.start();
                        handles.push(handle);
                    }
                })
            },
            BatchSize::PerIteration,
        );
    });

    System::current().stop();
    let _ = system.run();

    let system = System::new();
    let id = format!("Creation of {large} actors");
    c.bench_function(&id, |b| {
        b.iter_batched(
            || {},
            |()| {
                system.block_on(async move {
                    let mut handles = vec![];
                    for _ in 0..large {
                        let handler = BenchActor.start();
                        handles.push(handler);
                    }
                    handles
                })
            },
            BatchSize::PerIteration,
        );
    });
    System::current().stop();
    let _ = system.run();
}

#[allow(clippy::async_yields_async)]
fn process_messages(c: &mut Criterion) {
    const NUM_MSGS: u64 = 100000;

    #[derive(Message)]
    #[rtype(result = "u64")]
    struct BenchActorMessage;

    struct MessagingActor {
        state: u64,
    }

    impl Actor for MessagingActor {
        type Context = Context<Self>;

        fn started(&mut self, _ctx: &mut Self::Context) {}
    }

    impl Handler<BenchActorMessage> for MessagingActor {
        type Result = u64;

        fn handle(&mut self, _msg: BenchActorMessage, ctx: &mut Context<Self>) -> Self::Result {
            self.state += 1;
            if self.state >= NUM_MSGS {
                ctx.stop();
            }
            self.state
        }
    }

    let system = System::new();
    let id = format!("Waiting on {NUM_MSGS} messages to be processed [ by send() ]");
    c.bench_function(&id, |b| {
        b.iter_batched(
            || {},
            |()| {
                system.block_on(async move {
                    let addr = MessagingActor { state: 0 }.start();
                    for _ in 0..NUM_MSGS {
                        let _ = addr.send(BenchActorMessage).await.unwrap();
                    }
                    addr
                })
            },
            BatchSize::PerIteration,
        );
    });

    let id = format!("Waiting on {NUM_MSGS} messages to be processed [ by do_send() ]");
    c.bench_function(&id, |b| {
        b.iter_batched(
            || {},
            |()| {
                system.block_on(async move {
                    let addr = MessagingActor { state: 0 }.start();
                    for _ in 0..NUM_MSGS {
                        let _ = addr.do_send(BenchActorMessage);
                    }
                    addr
                })
            },
            BatchSize::PerIteration,
        );
    });

    System::current().stop();
    let _ = system.run();
}

criterion_group!(actix, create_actors, process_messages);
criterion_main!(actix);
