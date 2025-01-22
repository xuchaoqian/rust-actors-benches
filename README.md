# rust-actors-benches

## Preconditions

- CPU: 3.1 GHz Quad-Core Intel Core i7
- Memory: 16 GB 2133 MHz LPDDR3
- MacOS: 13.6.5
- Rust: 1.84.0
- Actix: 0.13.5
- Ractor: 0.14.6
- Kameo: 0.14.0

## cargo bench --bench [actix](https://github.com/actix/actix)

- Creation of 100 actors time: **45.527 µs**
- Creation of 10000 actors time: **11.389 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + do_send** ] time: **14.091 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + do_send** ] time: **28.439 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + send** ] time: **224.48 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + send** ] time: **931.06 ms**

## cargo bench --bench [ractor](https://github.com/slawlor/ractor)

- Creation of 100 actors time: **850.30 µs**
- Creation of 10000 actors time: **77.209 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + cast** ] time: **37.467 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + cast** ] time: **34.060 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + call** ] time: **122.74 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + call** ] time: **1.1686 s**

## cargo bench --bench [kameo](https://github.com/tqwewe/kameo)

- Creation of 100 actors time: **543.61 µs**
- Creation of 10000 actors time: **53.045 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + tell** ] time: **14.525 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + tell** ] time: **28.440 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + ask** ] time: **87.762 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + ask** ] time: **1.1482 s**

## Comparison

![rust actors benches](./rust-actors-benches/rust-actors-benches.001.png)
![rust actors benches](./rust-actors-benches/rust-actors-benches.002.png)
![rust actors benches](./rust-actors-benches/rust-actors-benches.003.png)
![rust actors benches](./rust-actors-benches/rust-actors-benches.004.png)
![rust actors benches](./rust-actors-benches/rust-actors-benches.005.png)
