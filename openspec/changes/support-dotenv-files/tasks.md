## 1. Add dotenvy dependency

- [ ] 1.1 In `crates/spoolman-server/Cargo.toml`, add `dotenvy = "0.15"` under `[dependencies]`

## 2. Load .env at startup

- [ ] 2.1 In `crates/spoolman-server/src/main.rs`, add `dotenvy::dotenv().ok();` as the first statement inside `main()`, before `config::Config::from_env()`

## 3. Verify .gitignore

- [ ] 3.1 Check `.gitignore` at the repo root; add `.env` if not already listed (keep `.env.example` out of scope)

## 4. Update CHANGELOG and TODO

- [ ] 4.1 Add entry to `CHANGELOG.md` under `[Unreleased] → Added`: "`.env` file support via `dotenvy`: the server silently loads a `.env` file from the working directory on startup, before reading environment variables. Missing file is not an error."
- [ ] 4.2 Remove the `.env` support item from `TODO.md`
