# hpwnr

Command-line version of [Happwner](https://github.com/Omegaplexx/Happwner). A **Happ** and **v2RayTun** subscription-link decryptor/encryptor that can fetch, decrypt, encrypt, and convert proxy profiles. It is available as both a CLI tool and a Rust library, built on a from-scratch reimplementation of the Happ and v2RayTun crypto.

Developed by slavrom21 & Omegaplex


## Features

* Decrypt `happ://crypt*/` links. Supported formats: `crypt5`, `crypt4`, `crypt3`, `crypt2`, `crypt`
* Decrypt `v2raytun://crypt/` links. Supported keys: `crypt3`, `crypt4`, `key3`
* Encrypt plaintext back into any of those formats
* Resolve wrapper/redirect links: `happ://add/`, `v2raytun://import/`, `incy://add/`, `incy://import/`, and `happ://` or `v2raytun://` links embedded in `http(s)` URLs
* Fetch subscription profiles over HTTP (custom User-Agent / X-HWID) and automatically decrypt the AES-128-GCM body layer *(full build only, see [Variants](#variants))*
* Convert Xray configs into `vless`/`vmess`/`trojan`/`hy2`/`ss`/`tuic` links, or into a merged **sing-box** config


## Variants

hpwnr builds in two flavors, selected with a Cargo feature:

* **full** (default) — everything above, including HTTP fetch and AES-128-GCM response decryption. Pulls in a TLS stack (`ureq` + `ring`).
* **lite** (`--no-default-features`) — crypt/decrypt and convert only, with the whole network stack removed: no fetch, no AES response layer, no User-Agent/HWID handling. It has no C dependencies, which makes it roughly a third of the size (~0.8 MB vs ~2.1 MB) and portable to far more targets.

```bash
cargo build --release                        # full
cargo build --release --no-default-features  # lite
```

The `fetch` feature is what carries the network stack; `default = ["fetch"]`, so dropping default features gives the lite build.


## Usage

`hpwnr <link> <arguments>`

Use `h` or `help` to show the help page. The argument order and leading hyphens do not matter, so `h`, `-h`, `--help` all work.

See full README in original project for detailed usage, flags, and examples.
