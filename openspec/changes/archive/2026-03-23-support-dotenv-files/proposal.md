## Why

Running the server locally requires setting several environment variables (`SPOOLMAN_DATA_FILE`, `SPOOLMAN_HOST`, `SPOOLMAN_PORT`, `SPOOLMAN_CORS_ORIGIN`, etc.) every time. There is no standard way to persist these for a local dev session. Every developer either exports them manually in their shell or wraps the binary in a script. A `.env` file is the industry-standard solution for this and is already listed in `TODO.md`.

## What Changes

- **`crates/spoolman-server/Cargo.toml`** — add `dotenvy = "0.15"` dependency.
- **`crates/spoolman-server/src/main.rs`** — call `dotenvy::dotenv().ok()` as the very first statement in `main()`, before `Config::from_env()`, so variables from `.env` populate `std::env` before they are read.
- **`.gitignore`** — ensure `.env` is listed (keep secrets out of version control).
- **`CHANGELOG.md`** — add entry under `[Unreleased] → Added`.
- **`TODO.md`** — remove the `.env` support item (now implemented).

## Capabilities

### New Capabilities

- `dotenv-loading`: On startup the server silently tries to load a `.env` file from the current working directory. If the file does not exist the server starts normally without error, so production environments are unaffected.

## Impact

- Single-line change to `main.rs`; no logic changes.
- `.ok()` on the result means missing `.env` is never an error — zero risk to Docker/production deployments where no `.env` file is present.
- No API, storage, or frontend changes.
