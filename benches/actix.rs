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

        fn started(&mut self, _ctx: &mut Self::Context) {
            // ctx.notify(BenchActorMessage);
        }
    }

    impl Handler<BenchActorMessage> for BenchActor {
        type Result = ();

        fn handle(&mut self, _msg: BenchActorMessage, _ctx: &mut Context<Self>) -> Self::Result {
            // ctx.stop();
            ()
        }
    }

    let small = 100;
    let large = 10000;

    let id = format!("Creation of {small} actors");
    let system = System::new();
    let arbiter = Arbiter::new();
    let arbiter_handle = arbiter.handle();
    c.bench_function(&id, |b| {
        b.iter_batched(
            || {},
            |()| {
                let arbiter_handle = arbiter_handle.clone();
                system.block_on(async move {
                    let mut addrs = vec![];
                    for _ in 0..small {
                        let addr = BenchActor::start_in_arbiter(&arbiter_handle, |_ctx| BenchActor);
                        addrs.push(addr);
                    }
                })
            },
            BatchSize::PerIteration,
        );
    });
    System::current().stop();
    let _ = system.run();

    let id = format!("Creation of {large} actors");
    let system = System::new();
    let arbiter = Arbiter::new();
    let arbiter_handle = arbiter.handle();
    c.bench_function(&id, |b| {
        b.iter_batched(
            || {},
            |()| {
                let arbiter_handle = arbiter_handle.clone();
                system.block_on(async move {
                    let mut addrs = vec![];
                    for _ in 0..large {
                        let addr = BenchActor::start_in_arbiter(&arbiter_handle, |_ctx| BenchActor);
                        addrs.push(addr);
                    }
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
    struct BenchActorMessage {
        n: u64,
    }

    struct MessagingActor {
        state: u64,
    }

    impl Actor for MessagingActor {
        type Context = Context<Self>;

        fn started(&mut self, _ctx: &mut Self::Context) {}
    }

    impl Handler<BenchActorMessage> for MessagingActor {
        type Result = u64;

        fn handle(&mut self, msg: BenchActorMessage, ctx: &mut Context<Self>) -> Self::Result {
            self.state += msg.n;
            if self.state >= NUM_MSGS {
                ctx.stop();
            }
            self.state
        }
    }

    let id =
        format!("Waiting on {NUM_MSGS} messages to be processed [ by single-threaded + do_send ]");
    let system = System::new();
    c.bench_function(&id, |b| {
        b.iter_batched(
            || {},
            |()| {
                system.block_on(async move {
                    let addr = MessagingActor { state: 0 }.start();
                    for _ in 0..NUM_MSGS {
                        let _ = addr.do_send(BenchActorMessage { n: 1 });
                    }
                    addr
                })
            },
            BatchSize::PerIteration,
        );
    });
    System::current().stop();
    let _ = system.run();

    let id =
        format!("Waiting on {NUM_MSGS} messages to be processed [ by single-threaded + send ]");
    let system = System::new();
    c.bench_function(&id, |b| {
        b.iter_batched(
            || {},
            |()| {
                system.block_on(async move {
                    let addr = MessagingActor { state: 0 }.start();
                    for _ in 0..NUM_MSGS {
                        let _ = addr.send(BenchActorMessage { n: 1 }).await.unwrap();
                    }
                    addr
                })
            },
            BatchSize::PerIteration,
        );
    });
    System::current().stop();
    let _ = system.run();

    let id =
        format!("Waiting on {NUM_MSGS} messages to be processed [ by multi-threaded + do_send ]");
    let system = System::new();
    let arbiter = Arbiter::new();
    c.bench_function(&id, |b| {
        b.iter_batched(
            || {
                MessagingActor::start_in_arbiter(&arbiter.handle(), |_ctx| MessagingActor {
                    state: 0,
                })
            },
            |addr| {
                system.block_on(async move {
                    for _ in 0..NUM_MSGS {
                        let _ = addr.do_send(BenchActorMessage { n: 1 });
                    }
                    addr
                });
            },
            BatchSize::PerIteration,
        );
    });
    System::current().stop();
    let _ = system.run();

    let id = format!("Waiting on {NUM_MSGS} messages to be processed [ by multi-threaded + send ]");
    let system = System::new();
    let arbiter = Arbiter::new();
    c.bench_function(&id, |b| {
        b.iter_batched(
            || {
                MessagingActor::start_in_arbiter(&arbiter.handle(), |_ctx| MessagingActor {
                    state: 0,
                })
            },
            |addr| {
                system.block_on(async move {
                    for _ in 0..NUM_MSGS {
                        let _ = addr.send(BenchActorMessage { n: 1 }).await.unwrap();
                    }
                    addr
                });
            },
            BatchSize::PerIteration,
        );
    });
    System::current().stop();
    let _ = system.run();
}

criterion_group!(actix, create_actors, process_messages);
criterion_main!(actix);
