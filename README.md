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
cargo build
cargo run
```
## releases
releases are available in the [releases](https://github.com/j1nxie/nhdl/releases) tab and sidebar!
## usage
run `nhdl` and input either a NH link or a valid url. after hitting enter, you should see the title, id and tags of given doujinshi if given id was valid.
download will immediately start, saved at `<nhdl directory>/<id>`.
## roadmap
- [x] basic support for single NH links through stdin.
- [x] multithreaded downloading.
- [ ] a text file / id parser for mass download.
- [ ] toml-based configuration for:
    - [ ] download path
    - [ ] thread count
    - [ ] proxy support
## license
this project is licensed under the [gnu gpl3](https://gnu.org/licenses/gpl-3.0.en.html).
## credits / dependencies
- [tokio](https://github.com/tokio-rs/tokio)
- [reqwest](https://github.com/seanmonstar/reqwest)
- [scraper](https://github.com/causal-agent/scraper)
- [textwrap](https://github.com/mgeisler/textwrap)
- [toml-rs](https://github.com/alexcrichton/toml-rs)
