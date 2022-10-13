# 2022 TD Hackathon: Rusty Web App!

This project aims to explore the current state of full-stack web development using the [Rust programming language](https://www.rust-lang.org/).

## Getting Started

1. Ensure Rust is on your system by installing [`rustup`](https://rustup.rs/).
2. Install the `trunk` tool with `cargo`, the Rust package manager: `cargo install trunk`.
3. Because Yew uses WebAssembly, we need to install the WebAssembly build target for the Rust compiler: `rustup target add wasm32-unknown-unknown`.
4. In one terminal, navigate to `/backend` and run `cargo run`.
5. In another terminal, navigate to `/frontend` and run `trunk serve`. This will recompile the frontend when the source has been modified.
6. Navigate to `http://localhost:3000` and behold the glory of a Rust-powered web app!
