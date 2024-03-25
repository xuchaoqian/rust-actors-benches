# actix-ractor-benches

## Preconditions

- CPU: 3.1 GHz Quad-Core Intel Core i7
- Memory: 16 GB 2133 MHz LPDDR3
- MacOS: 13.6.5
- Rust: 1.77
- Actix: 0.13.3
- Ractor: 0.9.7

## cargo bench --bench actix

- Creation of 100 actors time: **57.613 µs**
- Creation of 10000 actors time: **10.864 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + do_send** ] time: **14.491 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + do_send** ] time: **9.1211 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + send** ] time: **241.23 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + send** ] time: **987.28 ms**

## cargo bench --bench ractor

- Creation of 100 actors time: **772.38 µs**
- Creation of 10000 actors time: **81.368 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + cast** ] time: **40.488 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + cast** ] time: **38.376 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + call** ] time: **127.87 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + call** ] time: **1.2065 s**

## Conclusion

- In terms of creating actors, Actix is **5~15x faster** than Ractor.
- Actix's do_send is **3~5x faster** than Ractor's cast in single-threaded context.
- Actix's do_send is **3~5x faster** than Ractor's cast in multi-threaded context.
- Actix's send is **1.5~2x slower** than Ractor's call in single-threaded context.
- Actix's send is **1.2~1.5x faster** than Ractor's call in multi-threaded context.
