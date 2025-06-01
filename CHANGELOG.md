# Changelog

All notable changes to this project will be documented in this file.

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
