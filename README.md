# actix-ractor-benches

## cargo bench --bench actix

- Creation of 100 actors time: [105.65 µs **106.47** µs 107.48 µs]
- Creation of 10000 actors time: [10.060 ms **10.409 ms** 10.777 ms]
- Waiting on 100000 messages to be processed [ by **do_send()** ] time: [12.251 ms **12.355 ms** 12.468 ms]
- Waiting on 100000 messages to be processed [ by **send()** ] time: [229.02 ms **230.16 ms** 231.40 ms]

## cargo bench --bench ractor

- Creation of 100 actors time: [785.76 µs **805.60 µs** 827.46 µs]
- Creation of 10000 actors time: [80.931 ms **82.482 ms** 84.284 ms]
- Waiting on 100000 messages to be processed [ by **new_current_thread + cast()** ] time: [42.816 ms **43.083 ms** 43.379 ms]
- Waiting on 100000 messages to be processed [ by **new_current_thread + call()** ] time: [133.98 ms **134.62 ms** 135.30 ms]
- Waiting on 100000 messages to be processed [ by **new_multi_thread + cast()** ] time: [40.024 ms **40.313 ms** 40.635 ms]
- Waiting on 100000 messages to be processed [ by **new_multi_thread + call()** ] time: [1.3607 s **1.3681 s** 1.3759 s]
