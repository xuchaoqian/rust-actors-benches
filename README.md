# actix-ractor-benches

## cargo bench --bench actix

- Creation of 100 actors time: **106.47**
- Creation of 10000 actors time: **10.409 ms**
- Waiting on 100000 messages to be processed [ by **do_send()** ] time: **12.355 ms**
- Waiting on 100000 messages to be processed [ by **send()** ] time: **230.16 ms**

## cargo bench --bench ractor

- Creation of 100 actors time: **805.60 µs**
- Creation of 10000 actors time: **82.482 ms**
- Waiting on 100000 messages to be processed [ by **new_current_thread + cast()** ] time: **43.083 ms**
- Waiting on 100000 messages to be processed [ by **new_current_thread + call()** ] time: **134.62 ms**
- Waiting on 100000 messages to be processed [ by **new_multi_thread + cast()** ] time: **40.313 ms**
- Waiting on 100000 messages to be processed [ by **new_multi_thread + call()** ] time: **1.3681 s**

## Conclusion

- In terms of creating actors, Actix is 5~10 times faster than Ractor.
- In terms of casting msgs, Actix is 3~5 times faster than Ractor.
- In terms of call msgs, Actix is 1.5~2 times slower than Ractor in single-thread context.
- In terms of call msgs, Actix is 5~10 times faster than Ractor in multi-thread context.
