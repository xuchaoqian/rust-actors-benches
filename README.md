# rust-actors-benches

## Preconditions

- CPU: 3.1 GHz Quad-Core Intel Core i7
- Memory: 16 GB 2133 MHz LPDDR3
- MacOS: 13.6.5
- Rust: 1.78
- Actix: 0.13.3
- Ractor: 0.9.7
- Kameo: 0.8.0

## cargo bench --bench [actix](https://github.com/actix/actix)

- Creation of 100 actors time: **55.924 µs**
- Creation of 10000 actors time: **15.639 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + do_send** ] time: **16.563 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + do_send** ] time: **29.702 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + send** ] time: **244.69 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + send** ] time: **963.50 ms**

## cargo bench --bench [ractor](https://github.com/slawlor/ractor)

- Creation of 100 actors time: **1.6219 ms**
- Creation of 10000 actors time: **164.34 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + cast** ] time: **39.842 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + cast** ] time: **38.489 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + call** ] time: **126.76 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + call** ] time: **1.2307 s**

## cargo bench --bench [kameo](https://github.com/tqwewe/kameo)

- Creation of 100 actors time: **155.83 µs**
- Creation of 10000 actors time: **17.216 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + send_async** ] time: **13.898 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + send_async** ] time: **22.205 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + send** ] time: **80.683 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + send** ] time: **1.1576 s**

## Comparison

![rust actors benches](./rust-actors-benches.png)
