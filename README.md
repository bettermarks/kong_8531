# docker compose setup to reproduce https://github.com/Kong/kong/issues/8531

> This branch contains a simplified setup

> !! Here we attempted a green blue deployment 

## requirements
* [hey](https://github.com/rakyll/hey) for load testing

## setup

We create a go plugin, using the Go PDK, that returns 200 when it receives a request.
- [Go](https://github.com/Kong/go-pdk)

Behind it is a teapot service that returns 418, so we know if the plugin
was bypassed.

We use Kong version `3.1.0`.


### kong

> Small prep: `mkdir -p tmp/blue tmp/green` .
> These are used for the separate green and blue sockets

```bash
docker compose up --build kong > kong.log
```

Wait until kong and the plugin are healthy: look for something like
```
kong_8531-kong-1        | 2022/12/12 17:32:59 [info] 1131#0: *29 [goplugin:1142] 2022/12/12 17:32:59 Listening on socket: /usr/local/kong/goplugin.socket, context: ngx.timer
```

### loadtest

```bash
{ date; hey -c 8 -z 120s http://localhost/go; date; } > metrics.log &
```

### kong reload

```bash
sleep 1; docker compose exec kong kong-reload.sh
```

### wait for the loadtest to end
```bash
wait
```


## assumptions

Kong serves under reasonable load


## observations


On my machine it was not reliable whether the problem would happen every time.
But at least 1 in 3.


[metrics](metrics.log):

```
Mon 12 Dec 2022 18:59:00 CET

Summary:
  Total:	79.2675 secs
  Slowest:	15.1998 secs
  Fastest:	0.0011 secs
  Average:	0.2436 secs
  Requests/sec:	23.2756
  
  Total data:	1050 bytes
  Size/request:	0 bytes

Response time histogram:
  0.001 [1]	|
  1.521 [1832]	|■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  3.041 [3]	|
  4.561 [2]	|
  6.081 [0]	|
  7.600 [0]	|
  9.120 [0]	|
  10.640 [0]	|
  12.160 [1]	|
  13.680 [0]	|
  15.200 [2]	|


Latency distribution:
  10% in 0.0957 secs
  25% in 0.1003 secs
  50% in 0.1978 secs
  75% in 0.2977 secs
  90% in 0.4007 secs
  95% in 0.5983 secs
  99% in 1.1022 secs

Details (average, fastest, slowest):
  DNS+dialup:	0.0000 secs, 0.0011 secs, 15.1998 secs
  DNS-lookup:	0.0000 secs, 0.0000 secs, 0.0031 secs
  req write:	0.0000 secs, 0.0000 secs, 0.0020 secs
  resp wait:	0.2433 secs, 0.0010 secs, 15.1982 secs
  resp read:	0.0001 secs, 0.0000 secs, 0.0020 secs

Status code distribution:
  [200]	1814 responses
  [418]	2 responses
  [500]	25 responses

Error distribution:
  [4]	Get "http://localhost/go": context deadline exceeded (Client.Timeout exceeded while awaiting headers)

Mon 12 Dec 2022 19:00:19 CET
```

[error log](kong.log) contains further details.


### Analysis

* is not graceful - there are closed connections
* plugin is skipped/omitted, but teapot service reached (status 418)
* reaches a state, where a certain rate of all consecutive requests is corrupt.

