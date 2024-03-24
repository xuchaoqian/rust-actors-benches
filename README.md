# actix-ractor-benches

## Preconditions

- CPU: 3.1 GHz Quad-Core Intel Core i7
- Memory: 16 GB 2133 MHz LPDDR3
- MacOS: 13.6.5
- Rust: 1.75
- Actix: 0.13.3
- Ractor: 0.9.7

## cargo bench --bench actix

- Creation of 100 actors time: **44.789 µs**
- Creation of 10000 actors time: **12.808 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + do_send** ] time: **13.598 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + do_send** ] time: **28.191 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + send** ] time: **238.01 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + send** ] time: **954.64 ms**

## cargo bench --bench ractor

- Creation of 100 actors time: **805.60 µs**
- Creation of 10000 actors time: **82.482 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + cast** ] time: **43.083 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + cast** ] time: **40.313 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + call** ] time: **134.62 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + call** ] time: **1.3681 s**

## Conclusion

- In terms of creating actors, Actix is **5~20x faster** than Ractor.
- Actix's do_send is **3~5x faster** than Ractor's cast in single-threaded context.
- Actix's do_send is **1.3~1.5x faster** than Ractor's cast in multi-threaded context.
- Actix's send is **1.5~1x slower** than Ractor's call in single-threaded context.
- Actix's send is **1.3~1.5x faster** than Ractor's call in multi-threaded context.
