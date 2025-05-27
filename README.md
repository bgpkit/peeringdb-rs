# `peeringdb-rs`: unofficial utility crate for accessing PeeringDB data

*This readme is generated from the library's doc comments using [cargo-readme](https://github.com/livioribeiro/cargo-readme). Please refer to the Rust docs website for the full documentation*

[![Crates.io](https://img.shields.io/crates/v/peeringdb-rs)](https://crates.io/crates/peeringdb-rs)
[![Docs.rs](https://docs.rs/peeringdb-rs/badge.svg)](https://docs.rs/peeringdb-rs)
[![License](https://img.shields.io/crates/l/peeringdb-rs)](https://raw.githubusercontent.com/bgpkit/peeringdb-rs/main/LICENSE)

## Accessing PeeringDB data with API calls

`peeringdb_rs` is a utility crate to help users accessing important data
provided by [PeeringDB](https://www.peeringdb.com/) with their [public API](https://www.peeringdb.com/apidocs).

This is an _**unofficial**_ crate and use it with caution.

### Supported Data Types

- [`net`][api_net]: [PeeringdbIx]
- [`org`][api_org]: [PeeringdbOrg]
- [`ix`][api_ix]: [PeeringdbIx]
- [`netixlan`][api_netixlan]: [PeeringdbNetixlan]

Use the public function `load_peeringdb_XXXX()` to get a Vec of the corresponding data objects above.
For example, [load_peeringdb_net] will return a Vec of [PeeringdbNet] if success.

[api_net]: https://www.peeringdb.com/apidocs/#tag/api/operation/list%20net
[api_org]: https://www.peeringdb.com/apidocs/#tag/api/operation/list%20org
[api_ix]: https://www.peeringdb.com/apidocs/#tag/api/operation/list%20ix
[api_netixlan]: https://www.peeringdb.com/apidocs/#tag/api/operation/list%20netixlan

### PeeringDB API key required

It is strongly recommended to obtain a [PeeringDB API key][api_key_blog] and set the `PEERINGDB_API_KEY` environment variable.
Without it, the API call will likely to fail due to rate limiting.

[api_key_blog]: https://docs.peeringdb.com/blog/api_keys/

## License

See [PeeringDB Acceptable Use Policy](https://www.peeringdb.com/aup) more usage license information.

The usage of this library is under MIT license.
