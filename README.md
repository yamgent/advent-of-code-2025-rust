# advent-of-code-xxxx-rust

1. Clone template
2. Replace all `xxxx` with current year (do not accidentally replace content inside `actual_inputs` folder!)
3. Update the `src/bin/empty/main.rs` `include_str!()` macro to use the correct year.
4. Add `INPUT_REPO_TOKEN` secret with access to `actual_inputs` folder so that CI can access the inputs
    * Just "Contents - Read-only" access for the input repo is sufficient

## Useful commands

Start a new day:

```sh
cd src/bin/
cp -r empty ./dayXX
```

Watch changes with `cargo-watch`:

```sh
cargo watch -x 'test --bin dayXX'
```

Test everything, even for cases that are ignored:

```sh
cargo test -- --include-ignored
```
