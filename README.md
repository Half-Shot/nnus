# nnus

No Nonsense URL Shortnener written in Rust.

```sh
cargo install --git https://github.com/Half-Shot/nnus.git#1.0.0
mkdir ~/.nnus/
cd ~/.nnus/
nnus
```

## Installation

You can install the binaries found inside [Releases](https://github.com/Half-Shot/nnus/releases).

**OR**

You can install using `cargo`

(You will need [Rust](https://www.rust-lang.org/). You can install it using [RustUp](https://rustup.rs/))

``cargo install --git https://github.com:Half-Shot/nnus.git#1.0.0``

## Features

- You can be redirected to a new URL by following a short URL (`GET`)
- You can check where a URL will redirect you before going to it (`HEAD`)

- Configuration in SQLite3
- Lightweight process (`<50KB`)

## Configuration

You must add links to the `links` table in the generated SQLite3 database.

```sh
sqlite3 ~/.nnus/db.db3 "INSERT INTO links VALUES ('nnus', 'https://github.com/Half-Shot/nnus')"
```

A restart of the `nnus` process is not required.

You can also query links using

```sh
sqlite3 ~/.nnus/db.db3 "SELECT * FROM links"
```

You should sent the bind address with the `NNUS_BIND` environment variable, by default this is `127.0.0.1:6767`. It is expected that you host this service behind a 
reverse proxy like [NGINX](https://www.nginx.com/).

You can set the database storage location with the `NNUS_DB` environment variable. By default this is `~/db.db3`.

## Contact

You can contact me on Matrix via [`@Half-Shot:half-shot.uk`](https://matrix.to/#/@Half-Shot:half-shot.uk).