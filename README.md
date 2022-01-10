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
this project is licensed under the [gnu gpl3](https://gnu.org/licenses/gpl-3.0.en.html).
