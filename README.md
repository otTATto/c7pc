# c7pc

c7pc is a desktop port of [Chal7777](https://github.com/otTATto/Chal7777) — a timing game where you try to stop the timer at exactly 7.777 seconds.

## Tech Stack

This is a desktop app built with [Tauri](https://tauri.app), a framework with a Rust-based backend and a Vue-based frontend.

| Layer           | Technology                              |
| --------------- | --------------------------------------- |
| Framework       | [Tauri 2](https://tauri.app)            |
| Frontend        | [Vue 3](https://vuejs.org) + TypeScript |
| Backend         | Rust                                    |
| Package manager | [Bun](https://bun.sh)                   |
| Build tool      | [Vite](https://vite.dev)                |

## How to Develop

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Bun](https://bun.sh)
- Platform-specific dependencies (see [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/))

### Steps

1. Install dependencies.
   ```sh
   bun install
   ```
2. Start up a dev server.
   ```sh
   bun run tauri dev
   ```

## How to Build

```sh
bun run tauri build
```

The installer is generated in `src-tauri/target/release/bundle/`.
