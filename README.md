# nhdl
nhdl - a command-line doujinshi downloader client built in rust!

## goals
- **fast and asynchronous** doujinshi downloader, supporting various doujinshi sites.
- **simple** to use and setup.
- **lightweight** and **cross-platform**.

## non-goals
- give access to paywalled content.
- support for every doujinshi site in existence.

## build
```
git clone https://github.com/j1nxie/nhdl.git
cd nhdl
cargo build --release
```

## releases
releases are available in the [releases](https://github.com/j1nxie/nhdl/releases) tab and sidebar!

## usage
run `nhdl` in a command-line interface for help on usage.

## roadmap
- [x] basic support for single NH links through stdin.
- [x] multithreaded downloading.
- [x] a text file / id parser for mass download.
- [x] toml-based configuration for:
    - [x] download path
    - [x] proxy support
- [ ] pretty-printing of download progress.

## license
licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## contribution

unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
