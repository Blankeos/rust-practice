# 🦀 Rust Practice

Learning Rust because I wanna get into low-level programming and do some cool stuff with it.

This technically follows the Rust Book coincidentally.

Projects on this repo:

1. [x] Hello World - Rust basics. (Like `node index.js`) - `rustc main.rs && ./main`
2. [x] Hello Cargo - Rust basics + Package Manager + Build Tool. (Like `npm run dev`)
3. [x] Guessing Game - Random number generate, then take user input until random number is guessed.
4. [x] Salary - Console app that estimates salary based on hourly, daily, monthly, and yearly rates.
5. [ ] Tried Passwords CLI - Recreation of my [Go implementation](https://github.com/Blankeos/tried-passwords-cli).
6. [ ] HTTP Server - Simple HTTP server to understand networking and concurrency in Rust.
7. [ ] Actix Server - API Server with a framework called Actix (Like Go's Fiber wrapped around HTTP standard libs).
8. [x] Rspc Server - API Server but typesafe for TypeScript (like tRPC), expect it's in Rust! Very sick.
9. [ ] TCP Socket Chat - A console-based chat application using sockets to understand network communication.
10. [ ] Random Quotes - A simple CLI application that fetches a random quote from the [API Ninjas Quotes](https://api-ninjas.com/api/quotes).
11. [ ] Tic Tac Toe - A simple Tic Tac Toe game with persistence.
12. [ ] WebSocket Chat - A console-based chat application using WebSockets to understand network communication on browser.
13. [ ] Static Site Generator - Converts markdown to HTML.
14. [ ] Tauri - Electron-like webview for Rust.
15. [x] GPUI_Hello - A simple `gpui` program (Zed's gui library) for practice.
16. [x] GPUI_Input - A simple input field with `gpui` for practice.
17. [ ] Todo GUI Application - A simple GUI application to manage tasks.

### Notes

- Install Rust using [rustup](https://doc.rust-lang.org/book/ch01-01-installation.html#installing-rustup-on-linux-or-macos) - The installation and updating process is as convenient as Bun.
- CLI's you need to know: `rustup` is for installing and updating Rust. While `rustc` is the compiler. `cargo` is the package manager + build tool.
  - Coming from Bun, that's essentially: `bun`, `bun build`, and `bun install`/`bun run`
- `cargo new <project-name>` to create a new project in a file called project-name.
- `cargo new --lib` to create a project with lib.rs crate.
- `cargo init` to create a project inside current folder.
- `cargo build` to build the project or `cargo run` to build and run in one command. Or `cargo build --release` for a release build in `target/release` instead of `target/debug`.
- `cargo run -q` to run the program without the noise (doesn't remove the WARN though).
- `cargo check` to see if it compiles or not.
- `cargo update` updates the dependencies (like `pnpm update -i` I think).
- `cargo doc --open` opens the docs of all your dependencies in the browser.
- `cargo add <dep>` add a cargo dep to the project. Or you can also add a snippet into `[dependencies]` of `Cargo.toml` yourself.
- `cargo install --path .` Installs the binary to your path (It's not like bun install)
- If you want something that "installs deps" like bun install, just run any: `cargo check`, `cargo build`, `cargo run`, or `cargo clippy --all-targets`
- `cargo install --list` to see what you installed in your path.
- `cargo clippy --all-targets` - should be your primary feedback loop, can even do this on save.

#### Terminologies

- Crate - module/package. Must have at least 1 crate. I think it's just either a `.rs` file or a "project".
- Binary crate - inside `src/bin/*.rs` - can have 0 or multiple.
- Library crate - inside `src/lib.rs` - can only have 1.
- `mod` - that's a module in code. By default it's private. Add `pub` to make it public. `fn` also follows the same rule.
- `struct` and `impl` - Basically OOP in Rust, but better. Follows the same privacy rules as `mod`
- `enum` - you already know. Follows the same privacy rules as `mod` as well.

#### Faster Rust feedback loop tips:

- Use `[profile.dev] opt-level = 1` in `Cargo.toml` for faster compile times before making release builds.
- More:

```toml
# I got this from Bevy's docs.

# Enable a small amount of optimization in the dev profile.
[profile.dev]
opt-level = 1

# Enable a large amount of optimization in the dev profile for dependencies.
[profile.dev.package.'*']
opt-level = 3
```

- Primary feedback loop instead of compile: `cargo clippy --all-targets`. Can do this on save.

### Libraries

Libs that are kinda essential to know

- tokio - async
- serde - for serializing/deserializing. It's very generic so treat it like a library.
- serde_json - specifically for JSON serialization/deserialization (not confused anymore ?)
- clap - for making CLIs
- bevy - making games
- wgpu - for programming the GPU (used by Firefox's web gpu)
- axum - for web apis and web framework stuff
- embassy - embedded systems
- rayon - for parallelism (contrasts with tokio for async)
- nom - for parsing, so you can avoid regex.
- cargo lambda - make rust bins for aws lambda easy
- wasm-bindgen - Rust program that can run in web (wasm). (I used this, very nice). + wasm-pack
- napi-rs - node.js add-ons in rust.
- polars - df library in rust (doesn't work in wasm). I use rowboat.

### Resources

- [Rust Book](https://doc.rust-lang.org/book/ch01-01-installation.html) - Teaches Getting Started to Advanced Rust.
- [Rust Book but Video Playlist by "Let's Get Rusty"](https://www.youtube.com/watch?v=OX9HJsJUDxA&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=2)
- [Rustlings](https://github.com/rust-lang/rustlings) - Small exercises that can complement the Rust Book.
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html) - A collection of runnable examples that illustrate various Rust concepts and standard libraries.
