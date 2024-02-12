# rust-quickjs
rust-quickjs

# Benchmark
```
ankit.anand-mbp@BA-00026768 foo % wrk -t5 -c50 http://localhost:8080/
Running 10s test @ http://localhost:8080/
  5 threads and 50 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.87ms    1.71ms  49.01ms   94.18%
    Req/Sec     5.91k   491.39     8.32k    78.17%
  296389 requests in 10.10s, 1.07GB read
Requests/sec:  29338.74
Transfer/sec:    108.92MB
```
