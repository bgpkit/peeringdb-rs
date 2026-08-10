# Changelog

All notable changes to this project will be documented in this file.

## v0.2.0 -- 2026-08-10

### Highlights

* add `load_peeringdb_XXXX_filtered()` variants that accept PeeringDB API query
  parameters for server-side filtering, so callers can fetch a subset instead of
  the whole table (#3)
* fix `load_peeringdb_netixlan` fetching `/api/org` instead of `/api/netixlan`;
  org records were silently deserializing into `PeeringdbNetixlan` because every
  field except `id` is `Option` (#1)
* omit the `Authorization` header when `PEERINGDB_API_KEY` is unset or empty,
  fixing anonymous (keyless) requests that previously hit HTTP 400 (#2)

### Contributors

* [@hellerve](https://github.com/hellerve) authored all three changes (#1, #2, #3)

## v0.1.3 -- 2025-10-29

* update to `oneio` v0.20.0 with better control over rustls cryto providers

## v0.1.2 -- 2025-09-09

### Highlights

* add `User-Agent` header to requests to avoid getting "403 Forbidden" responses from PeeringDB
* updated to `oneio` v0.19.0`

## v0.1.1 -- 2025-05-31

### Highlights

* use `rustls` for TLS connections
    * updated to `oneio` v0.18.1

## v0.1.0 -- 2025-05-27

Add four functions to load corresponding data entries from PeeringDB:

* `load_peeringdb_ix()`
* `load_peeringdb_net()`
* `load_peeringdb_netixlan()`
* `load_peeringdb_org()`

Set `PEERINGDB_API_KEY` environment variable to ensure higher rate limits.
