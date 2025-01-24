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

- Creation of 10000 actors time: **5.0315 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + try_send** ] time: **9.0102 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + try_send** ] time: **28.867 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + send** ] time: **212.71 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + send** ] time: **923.45 ms**

## cargo bench --bench [ractor](https://github.com/slawlor/ractor)

- Creation of 10000 actors time: **68.510 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + cast** ] time: **36.834 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + cast** ] time: **34.510 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + call** ] time: **121.69 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + call** ] time: **1.1713 s**

## cargo bench --bench [kameo](https://github.com/tqwewe/kameo)

- Creation of 10000 actors time: **47.906 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + tell** ] time: **13.954 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + tell** ] time: **27.482 ms**
- Waiting on 100000 messages to be processed [ by **single-threaded + ask** ] time: **85.921 ms**
- Waiting on 100000 messages to be processed [ by **multi-threaded + ask** ] time: **1.1574 s**

## Comparison

![rust actors benches](./rust-actors-benches/rust-actors-benches.001.png)
![rust actors benches](./rust-actors-benches/rust-actors-benches.002.png)
![rust actors benches](./rust-actors-benches/rust-actors-benches.003.png)
![rust actors benches](./rust-actors-benches/rust-actors-benches.004.png)
![rust actors benches](./rust-actors-benches/rust-actors-benches.005.png)
