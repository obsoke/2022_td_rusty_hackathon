# 2022 TD Hackathon: Rusty Web App!

This project aims to explore the current state of full-stack web development using the [Rust programming language](https://www.rust-lang.org/).

## Requirements

These tools should be installed on your system before continuing:

- `sqlite`

## Getting Started

1. Ensure Rust is on your system by installing [`rustup`](https://rustup.rs/).
2. Install the `trunk` tool with `cargo`, the Rust package manager: `cargo install trunk`.
3. Because Yew uses WebAssembly, we need to install the WebAssembly build target for the Rust compiler: `rustup target add wasm32-unknown-unknown`.
4. In one terminal, navigate to `/backend` and run `cargo run`.
5. In another terminal, navigate to `/frontend` and run `trunk serve`. This will recompile the frontend when the source has been modified.
6. Navigate to `http://localhost:3000` and behold the glory of a Rust-powered web app!

## The App

A flashcard viewer/manager!

To Do:
- [ ] Manager
  - [ ] Create a new category for flashcards
  - [ ] Add a new flashcard to the category
- [X] Viewer
  - [X] Start learning with flashcards by selecting a category
  - [X] Display the "question" portion of the flashcard
  - [X] Display the "answer" portion of the flashcard upon button click
  - [X] Click a button to view the next card
  
## Retro

There were a few challenges in writing this app fully in Rust.

### Sharing types

I wanted to put my types in a central place in order to share them between the frontend and backend. This worked on the backend for the most part, as the environment was simple. However, when trying to use this `types` crate on the frontend, I ran into issues compiling dependencies into WebAssembly.
