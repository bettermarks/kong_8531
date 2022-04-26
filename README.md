# docker compose setup to reproduce https://github.com/Kong/kong/issues/8531

## setup

We create a dummy plugin in each available PDK language:
- [Go](https://github.com/Kong/go-pdk)
- [Python](https://github.com/Kong/kong-python-pdk)
- [JavaScript](https://github.com/Kong/kong-js-pdk)

Rust PDK is incomplete:
- [Rust](https://github.com/jgramoll/kong-rust-pdk)

Each plugin injects a specific header, e.g. `x-goplugin` and forwards the request to a simple reflection service, which the loadtest tests against to check if the plugin was intercepting the request. 

We use Kong version `2.7.1`.


### kong

``` bash
docker compose up > kong.log
```

### loadtest

``` bash
(cd loadtest && cargo run --release -- --host http://localhost --no-reset-metrics --no-task-metrics -u 8 -t 20) > metrics.log
```

### kong reload

``` bash
docker compose exec kong kong reload
```


## assumptions

Kong serves under reasonable load - see [goose loadtest](https://goose.rs).


## observations

In every case we were able to DOS kong plugins by reload.


[metrics](metrics.log):

```
All 8 users hatched.

22:18:33 [WARN] "/js": error sending request for url (http://localhost/js): connection closed before message completed
22:18:33 [WARN] "/js": error sending request for url (http://localhost/js): connection closed before message completed
22:18:33 [WARN] "/go": error sending request for url (http://localhost/go): connection closed before message completed
22:18:33 [WARN] "/go": error sending request for url (http://localhost/go): connection closed before message completed
22:18:33 [WARN] "/go": error sending request for url (http://localhost/go): connection closed before message completed
22:18:54 [WARN] "/js": error sending request for url (http://localhost/js): operation timed out
22:18:54 [WARN] "/js": error sending request for url (http://localhost/js): operation timed out
22:18:55 [WARN] "/js": error sending request for url (http://localhost/js): operation timed out
22:18:55 [WARN] "/js": error sending request for url (http://localhost/js): operation timed out
22:18:55 [WARN] "/js": error sending request for url (http://localhost/js): operation timed out
22:18:55 [WARN] "/js": error sending request for url (http://localhost/js): operation timed out
22:18:56 [WARN] "/js": error sending request for url (http://localhost/js): operation timed out
22:18:56 [WARN] "/js": error sending request for url (http://localhost/js): operation timed out

 === PER REQUEST METRICS ===
 ------------------------------------------------------------------------------
 Name                     |        # reqs |        # fails |    req/s |  fail/s
 ------------------------------------------------------------------------------
 GET /go                  |        14,312 |     282 (2.0%) |   715.60 |   14.10
 GET /js                  |        14,320 |     618 (4.3%) |   716.00 |   30.90
 GET /python              |        14,320 |         2 (0%) |   716.00 |    0.10
 -------------------------+---------------+----------------+----------+--------
 Aggregated               |        42,952 |     902 (2.1%) |     2148 |   45.10
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |        Min |         Max |     Median
 ------------------------------------------------------------------------------
 GET /go                  |        0.93 |          1 |         202 |          1
 GET /js                  |       13.70 |          1 |      20,001 |          2
 GET /python              |        1.93 |          1 |       2,061 |          1
 -------------------------+-------------+------------+-------------+-----------
 Aggregated               |        5.52 |          1 |      20,001 |          1
 ------------------------------------------------------------------------------
 Slowest page load within specified percentile of requests (in ms):
 ------------------------------------------------------------------------------
 Name                     |    50% |    75% |    98% |    99% |  99.9% | 99.99%
 ------------------------------------------------------------------------------
 GET /go                  |      1 |      1 |      3 |      4 |     11 |    200
 GET /js                  |      2 |      3 |      6 |      8 |     54 | 20,000
 GET /python              |      1 |      2 |      4 |      5 |     19 |    430
 -------------------------+--------+--------+--------+--------+--------+-------
 Aggregated               |      1 |      2 |      5 |      6 |     25 | 20,000

 === ERRORS ===
 ------------------------------------------------------------------------------
 Count       | Error
 ------------------------------------------------------------------------------
 608           GET /js: plugin header missing
 279           GET /go: plugin header missing
 8             GET /js: error sending request /js: operation timed out
 3             GET /go: error sending request /go: connection closed before message completed
 2             GET /python: plugin header missing
 2             GET /js: error sending request /js: connection closed before message completed
 ------------------------------------------------------------------------------
 ------------------------------------------------------------------------------
 Users: 8
 Target host: http://localhost/
 Starting: 2022-04-27 00:18:19 - 2022-04-27 00:18:26 (duration: 00:00:07)
 Running:  2022-04-27 00:18:26 - 2022-04-27 00:18:46 (duration: 00:00:20)
 Stopping: 2022-04-27 00:18:46 - 2022-04-27 00:18:56 (duration: 00:00:10)

 goose v0.15.2
 ------------------------------------------------------------------------------

```

[error log](kong.log) contains further details.


### Python plugin

Python plugin is nearly stable and seems to "repair" its corruption.


### Go plugin

* is not graceful - there are closed connections
* plugin is skipped/omitted, but reflection service reached (missing header) 
* reaches a currupted state, where a certain rate of all consecutive requests are corrupt.


### JavaScript plugin

* is not graceful - closed connections and also timeouts
* plugin is skipped/omitted, but reflection service reached (missing header) 
* reaches a state, where **all** consecutive requests are corrupt.


### nginx worker 100% CPU

Sometimes a nginx worker turns crazy by consuming 100% CPU - `strace` shows:

```
epoll_pwait(50, [], 512, 0, NULL, 8)    = 0
epoll_pwait(50, [], 512, 0, NULL, 8)    = 0
epoll_pwait(50, [], 512, 0, NULL, 8)    = 0
epoll_pwait(50, [], 512, 0, NULL, 8)    = 0
epoll_pwait(50, [], 512, 0, NULL, 8)    = 0
epoll_pwait(50, [], 512, 0, NULL, 8)    = 0
epoll_pwait(50, [], 512, 0, NULL, 8)    = 0
epoll_pwait(50, [], 512, 0, NULL, 8)    = 0
epoll_pwait(50, [], 512, 0, NULL, 8)    = 0
epoll_pwait(50, [], 512, 0, NULL, 8)    = 0
```
