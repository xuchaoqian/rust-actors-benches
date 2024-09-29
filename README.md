# rust-actors-benches

## Preconditions

- CPU: 3.1 GHz Quad-Core Intel Core i7
- Memory: 16 GB 2133 MHz LPDDR3
- MacOS: 13.6.5
- Rust: 1.81
- Actix: 0.13.5
- Ractor: 0.11.1
- Kameo: 0.11.0

## cargo bench --bench [actix](https://github.com/actix/actix)

- Creation of 100 actors time: **44.392 µs**
- Creation of 10000 actors time: **8.9535 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + do_send** ] time: **14.457 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + do_send** ] time: **29.415 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + send** ] time: **218.10 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + send** ] time: **940.27 ms**

## cargo bench --bench [ractor](https://github.com/slawlor/ractor)

- Creation of 100 actors time: **699.34 µs**
- Creation of 10000 actors time: **70.927 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + cast** ] time: **32.729 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + cast** ] time: **31.609 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + call** ] time: **115.89 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + call** ] time: **1.1704 s**

## cargo bench --bench [kameo](https://github.com/tqwewe/kameo)

- Creation of 100 actors time: **153.87 µs**
- Creation of 10000 actors time: **18.790 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + tell** ] time: **13.747 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + tell** ] time: **24.168 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + ask** ] time: **78.395 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + ask** ] time: **1.1699 s**

## Comparison

![rust actors benches](./rust-actors-benches.png)