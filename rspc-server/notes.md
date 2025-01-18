Notes:

1. Installed diesel:

   ```sh
   cargo install diesel_cli --no-default-features --features sqlite
   diesel setup

   # Also added to Cargo.toml:
   diesel = { version: "2.2.0", features = ['chrono', 'sqlite']}
   dotenvy
   ```

2. I wanted to add a script runner as well (Just vs cargo-make)

   ```sh
   # Just
   just <command>
   # File: Justfile
   # - Works for more than just rust projects
   # brew install just

   cargo make <command>
   # File: Makefile.toml (I like the syntax more.)
   # - Only works for Rust projects I guess.
   # cargo install --force cargo-make
   ```
