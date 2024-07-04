# 🦀 Rust Practice

Learning Rust because I wanna get into low-level programming and do some cool stuff with it.

This technically follows the Rust Book coincidentally.

Projects on this repo:

1. [x] Hello World - Rust basics. (Like `node index.js`)
2. [x] Hello Cargo - Rust basics + Package Manager + Build Tool. (Like `npm run dev`)
3. [ ] Guessing Game - Random number generate, then take user input until random number is guessed.
4. [ ] Salary - Console app that estimates salary based on hourly, daily, monthly, and yearly rates.
5. [ ] Tried Passwords CLI - Recreation of my [Go implementation](https://github.com/Blankeos/tried-passwords-cli).
6. [ ] HTTP Server - Simple HTTP server to understand networking and concurrency in Rust.
7. [ ] TCP Socket Chat - A console-based chat application using sockets to understand network communication.
8. [ ] Random Quotes - A simple CLI application that fetches a random quote from the [API Ninjas Quotes](https://api-ninjas.com/api/quotes).
9. [ ] Tic Tac Toe - A simple Tic Tac Toe game with persistence.
10. [ ] WebSocket Chat - A console-based chat application using WebSockets to understand network communication on browser.
11. [ ] Static Site Generator - Converts markdown to HTML.
12. [ ] Tauri - Electron-like webview for Rust.\*\*\*\*
13. [ ] Todo GUI Application - A simple GUI application to manage tasks.

### Notes

- Install Rust using [rustup](https://doc.rust-lang.org/book/ch01-01-installation.html#installing-rustup-on-linux-or-macos) - The installation and updating process is as convenient as Bun.
- CLI's you need to know: `rustup` is for installing and updating Rust. While `rustc` is the compiler. `cargo` is the package manager + build tool.
  - Coming from Bun, that's essentially: `bun`, `bun build`, and `bun install`/`bun run`
- Cargo is awesome because you can `cargo new` to create a project.
- `cargo build` to build the project or `cargo run` to build and run in one command. Or `cargo build --release` for a release build in `target/release` instead of `target/debug`.
- `cargo check` to see if it compiles or not.

### Resources

- [Rust Book](https://doc.rust-lang.org/book/ch01-01-installation.html) - Teaches Getting Started to Advanced Rust.
- [Rustlings](https://github.com/rust-lang/rustlings) - Small exercises that can complement the Rust Book.
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html) - A collection of runnable examples that illustrate various Rust concepts and standard libraries.
