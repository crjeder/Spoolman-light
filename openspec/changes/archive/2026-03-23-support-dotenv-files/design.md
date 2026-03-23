## Approach

Use the [`dotenvy`](https://crates.io/crates/dotenvy) crate — the actively maintained successor to `dotenv`. It reads a `.env` file from the current directory (or any parent) and loads key=value pairs into `std::env` without overwriting existing environment variables.

`dotenvy::dotenv().ok()` is the canonical one-liner: `.ok()` discards the `Result`, so a missing `.env` is silently ignored. This is the correct behaviour for a server that may run in Docker or CI with no `.env` present.

## Change Details

### `crates/spoolman-server/Cargo.toml`

Add under `[dependencies]`:

```toml
dotenvy = "0.15"
```

### `crates/spoolman-server/src/main.rs`

Insert as the very first line of `main()`:

```rust
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();          // load .env if present — must come first
    let cfg = config::Config::from_env();
    // …rest unchanged…
}
```

Placement before `Config::from_env()` is mandatory: environment variables must be in `std::env` before they are read.

### `.gitignore`

Verify `.env` is listed. If not, add it. A `.env.example` file with commented-out keys can optionally be committed as documentation; this is out of scope for the task but noted for future reference.

## Alternatives Considered

- **`dotenv` (original crate)** — unmaintained since 2020; `dotenvy` is the drop-in successor.
- **Shell scripts / `direnv`** — external tooling; not portable across editors and CI.
- **Config file (TOML/JSON)** — heavier than needed; `.env` is the convention for twelve-factor apps.

## Testing

No automated tests needed. Manual verification:

1. Create a `.env` in the repo root with `SPOOLMAN_PORT=9000`.
2. Run `cargo run -p spoolman-server` — server must bind on port 9000.
3. Delete `.env` — server must still start (defaults apply).
4. Set `SPOOLMAN_PORT=8001` in the shell *and* `SPOOLMAN_PORT=9000` in `.env` — shell value must win (dotenvy never overwrites existing vars).
